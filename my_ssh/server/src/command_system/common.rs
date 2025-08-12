use shell_words::split;
use std::path::PathBuf;

#[derive(Clone, Debug)]

/// Represents a parsed shell command with its arguments and optional operator.
/// 
/// `Command` is the fundamental unit of command processing in the shell system.
/// It contains the command name, arguments, and any shell operator that follows
/// the command (such as pipes, redirections, or logical operators).
/// 
/// # Fields
/// 
/// - `cmd`: Vector containing the command name (index 0) and its arguments
/// - `op`: Optional shell operator that follows this command
/// 
/// # Examples
/// 
/// ```rust
/// // Simple command: "ls"
/// let cmd = Command {
///     cmd: vec!["ls".to_string()],
///     op: None,
/// };
/// // Command with redirection: "echo hello >"
/// let cmd_with_redirect = Command {
///     cmd: vec!["echo".to_string(), "hello".to_string()],
///     op: Some(">".to_string()),
/// };
/// ```
pub struct Command {
    pub cmd: Vec<String>,
    pub op: Option<String>,
}

/// Retrieves all entries (files and directories) from the specified directory path.
/// 
/// This function reads the contents of a directory and returns the canonical
/// (absolute) paths of all entries. It handles both files and subdirectories,
/// resolving symbolic links to their actual targets.
/// 
/// # Parameters
/// 
/// - `path`: Reference to the directory path to read
/// 
/// # Returns
/// 
/// - `Ok(Vec<PathBuf>)`: Vector of canonical paths for all directory entries
/// - `Err(std::io::Error)`: I/O error if the directory cannot be read or paths cannot be canonicalized
/// 
/// # Errors
/// 
/// This function can fail if:
/// - The specified path does not exist
/// - The path is not a directory
/// - Permission is denied to read the directory
/// - A file path cannot be canonicalized (e.g., broken symbolic link)
/// - I/O errors occur during directory traversal
/// 
/// # Examples
/// 
/// ```rust
/// use std::path::Path;
/// 
/// // Read files from current directory
/// match get_files(Path::new(".")) {
///     Ok(files) => {
///         println!("Found {} entries:", files.len());
///         for file in files {
///             println!("  {}", file.display());
///         }
///     }
///     Err(e) => eprintln!("Error reading directory: {}", e),
/// }
/// 
/// // Read files from specific directory
/// let dir_path = Path::new("/home/user/documents");
/// let files = get_files(dir_path)?;
/// ```
/// 
/// # Note
/// 
/// The returned paths are canonicalized, meaning they are absolute paths
/// with all symbolic links resolved and `.` and `..` components removed.
pub fn get_files(path: &std::path::Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let dir = std::fs::read_dir(path)?;
    let mut files: Vec<PathBuf> = Vec::new();
    for entry_res in dir {
        let entry = entry_res?;
        let file_path = entry.path();
        let file_path = std::fs::canonicalize(&file_path)?;
        files.push(file_path.clone());
    }
    Ok(files)
}
/// Parses a command line string into a vector of `Command` structures.
/// 
/// This function takes raw shell input and breaks it down into individual commands
/// with their associated operators. It handles shell word splitting (respecting
/// quotes and escapes) and identifies shell operators that separate commands.
/// 
/// # Parameters
/// 
/// - `client_input`: Raw command line string from the user
/// 
/// # Returns
/// 
/// A vector of `Command` structures representing the parsed command pipeline.
/// Returns an empty vector if parsing fails.
/// 
/// # Supported Operators
/// 
/// | Operator | Description | Example |
/// |----------|-------------|---------|
/// | `&&` | Logical AND (execute next if current succeeds) | `mkdir test && cd test` |
/// | `\|` | Pipe (pass output to next command) | `ps aux \| grep nginx` |
/// | `\|\|` | Logical OR (execute next if current fails) | `rm file.txt \|\| echo "failed"` |
/// | `<` | Input redirection | `sort < input.txt` |
/// | `>` | Output redirection | `ls > output.txt` |
/// | `;` | Sequential execution | `pwd ; ls ; date` |
/// 
/// # Parsing Behavior
/// 
/// - Uses `shell_words` crate for proper shell word splitting
/// - Respects quoted strings and escape sequences
/// - Splits commands at operator boundaries
/// - Associates each operator with the preceding command
/// - The final command in a sequence has `op: None`
/// 
/// ## Quoted Arguments
/// ```rust
/// let commands = get_commands(r#"echo "hello world" | grep hello"#.to_string());
/// // Result: [
/// //   Command { cmd: ["echo", "hello world"], op: Some("|") },
/// //   Command { cmd: ["grep", "hello"], op: None }
/// // ]
/// ```
/// 
/// # Error Handling
/// 
/// If the input string contains invalid shell syntax (unclosed quotes, etc.),
/// the function returns an empty vector rather than panicking.
pub fn get_commands(client_input: String) -> Vec<Command> {
    let mut cmds: Vec<Command> = Vec::new();
    let op = ["&&", "|", "||", "<", ">", ";"];

    let parsed = match split(client_input.trim()) {
        Ok(v) => v,
        Err(_) => return cmds,
    };

    let mut current_cmd: Vec<String> = Vec::new();
    for token in parsed {
        if op.contains(&token.as_str()) {
            cmds.push(Command {
                cmd: current_cmd.clone(),
                op: Some(token),
            });
            current_cmd.clear();
        } else {
            current_cmd.push(token);
        }
    }

    if !current_cmd.is_empty() {
        cmds.push(Command {
            cmd: current_cmd,
            op: None,
        });
    }

    cmds
}
/// Enumeration of text formatting types used in the shell output system.
/// 
/// `Format` defines various formatting options for shell output, including
/// error messages, directory listings, colors, and special markers used
/// by the shell's display system.
/// 
/// # Variants
/// 
/// - `Error`: Error message formatting
/// - `ListDir`: Directory listing formatting
/// - `Color(&'static str)`: Specific color formatting with color name
/// - `Split`: Line separator/split formatting
/// - `Normal`: Normal text formatting
/// - `NormalColored`: Normal colored text formatting
/// 
/// # Examples
/// 
/// ```rust
/// // Error formatting
/// let error_format = get_format(Format::Error);
/// 
/// // Color formatting
/// let blue_format = get_format(Format::Color("BLUE"));
/// let red_format = get_format(Format::Color("LIGHT_RED"));
/// 
/// // Normal formatting
/// let normal_format = get_format(Format::Normal);
/// ```
pub enum Format {
    Error,
    ListDir,
    Color(&'static str),
    Split,
    Normal,
    NormalColored,
}
/// Returns the formatting string for the specified format type.
/// 
/// This function maps `Format` enum variants to their corresponding
/// formatting control strings used by the shell's display system.
/// 
/// # Parameters
/// 
/// - `format`: The format type to get the control string for
/// 
/// # Returns
/// 
/// A static string slice containing the formatting control sequence.
/// 
/// # Format Mappings
/// 
/// | Format | Control String | Purpose |
/// |--------|---------------|---------|
/// | `Error` | `"?&E"` | Error message prefix |
/// | `ListDir` | `"?&L"` | Directory listing prefix |
/// | `NormalColored` | `"?&C"` | Colored text prefix |
/// | `Normal` | `"?&N"` | Normal text prefix |
/// | `Color("BLUE")` | `"^!"` | Blue color marker |
/// | `Color("LIGHT_RED")` | `"^@"` | Light red color marker |
/// | `Color("GREEN")` | `"^#"` | Green color marker |
/// | `Color("stop")` | `"~~"` | Color stop marker |
/// | `Split` | `"\n\n"` | Line separator |
/// | *other* | `""` | Empty string for unknown formats |
/// 
/// # Examples
/// 
/// ```rust
/// // Format an error message
/// let error_msg = format!("{}File not found{}", 
///     get_format(Format::Error),
///     get_format(Format::Split)
/// );
/// 
/// // Format colored text
/// let colored_text = format!("{}Important{}{}",
///     get_format(Format::Color("LIGHT_RED")),
///     get_format(Format::Color("stop")),
///     get_format(Format::Split)
/// );
/// 
/// // Format normal output
/// let normal_output = format!("{}Command completed{}",
///     get_format(Format::Normal),
///     get_format(Format::Split)
/// );
/// ```
pub fn get_format(format: Format) -> &'static str {
    match format {
        Format::Error => "?&E",
        Format::ListDir => "?&L",
        Format::NormalColored => "?&C",
        Format::Normal => "?&N",
        Format::Color("BLUE") => "^!",
        Format::Color("LIGHT_RED") => "^@",
        Format::Color("GREEN") => "^#",
        Format::Color("stop") => "~~",
        Format::Split => "\n\n",
        _ => "",
    }
}
/// Removes formatting control sequences from formatted text.
/// 
/// This function processes text that contains shell formatting control sequences
/// and returns a clean, unformatted version suitable for plain text output
/// 
/// # Parameters
/// 
/// - `text`: The formatted text string containing control sequences
/// 
/// # Returns
/// 
/// A clean `String` with all formatting control sequences removed.
/// 
/// # Processing Rules
/// 
/// 1. **Section Parsing**: Splits text on `?&` markers to identify formatted sections
/// 2. **Format Type Detection**: Identifies the format type from the first character after `?&`
/// 3. **Color Sequence Removal**: Removes color markers like `^@`, `~~` from colored text
/// 4. **Prefix Removal**: Strips formatting prefixes from regular sections
/// 5. **Text Reconstruction**: Rebuilds clean text with proper spacing
/// 
/// # Supported Control Sequences
/// 
/// - `?&E`, `?&L`, `?&N`, `?&C`: Format type prefixes
/// - `^@`, `^!`, `^#`: Color start markers
/// - `~~`: Color stop marker
/// - `\n\n`: Section separators
/// 
/// # Examples
/// 
/// ## Basic Formatting Removal
/// ```rust
/// let formatted = "?&NHello World\n\n";
/// let clean = get_unformated_text(formatted);
/// assert_eq!(clean, "Hello World");
/// ```
/// 
/// ## Color Sequence Removal
/// ```rust
/// let colored = "?&C^@Important~~Text\n\n";
/// let clean = get_unformated_text(colored);
/// assert_eq!(clean, "ImportantText");
/// ```
/// 
/// ## Complex Formatting
/// ```rust
/// let complex = "?&EError: ?&C^@File not found~~\n\n?&NOperation failed\n\n";
/// let clean = get_unformated_text(complex);
/// assert_eq!(clean, "Error:  File not found Operation failed");
/// ```
/// 
/// ## Multiple Sections
/// ```rust
/// let multi = "?&NStatus: ?&C^#OK~~\n\n?&NFiles: 42\n\n";
/// let clean = get_unformated_text(multi);
/// assert_eq!(clean, "Status:  OK Files: 42");
/// ```
pub fn get_unformated_text(text: &str) -> String {
    let mut new_text = String::new();
    let props: Vec<&str> = text.split("?&").filter(|f| !f.is_empty()).collect();
    for w in props {
        let word: Vec<&str> = w[1..].split("\n\n").filter(|f| !f.is_empty()).collect();
        match w.chars().next() {
            Some('C') => {
                for e in word {
                    let chars: Vec<_> = e.chars().collect();
                    let mut i = 0;
                    while i < chars.len() {
                        let c = chars[i];
                        if i + 1 < chars.len() && c == '^' && chars[i + 1] == '@' {
                            i += 2;
                            continue;
                        }
                        if i + 1 < chars.len() && c == '~' && chars[i + 1] == '~' {
                            i += 2;
                            continue;
                        }

                        new_text = format!("{}{}", new_text, c);

                        i += 1;
                    }
                }
            }
            Some(_) => {
                for e in word {
                    if e.starts_with('^') {
                        let tail: String = e.chars().skip(2).collect();
                        if new_text.is_empty() {
                            new_text = tail;
                        } else {
                            new_text = format!("{} {}", new_text, tail);
                        }
                    } else if new_text.is_empty() {
                        new_text = e.to_string();
                    } else {
                        new_text = format!("{} {}", new_text, e);
                    }
                }
            }
            None => (),
        }
    }
    new_text
}

#[cfg(unix)]
pub fn is_executable(path: std::path::PathBuf) -> std::io::Result<bool> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    let metadata = fs::metadata(path)?;
    Ok(metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(windows)]
pub fn is_executable(path: std::path::PathBuf) -> std::io::Result<bool> {
    if !path.exists() {
        return Ok(false);
    }
    Ok(matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase())
            .as_deref(),
        Some("exe") | Some("bat") | Some("cmd") | Some("com")
    ))
}
