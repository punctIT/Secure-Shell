use crate::command_system::commands::change_directory::ChangeDIR;
use crate::command_system::commands::concatenate::Cat;
use crate::command_system::commands::echo::Echo;
use crate::command_system::commands::executable_files::Execute;
use crate::command_system::commands::global_regular_expresion_print::Grep;
use crate::command_system::commands::make_director::MakeDir;
use crate::command_system::commands::move_class::MoveFileAndDir;
use crate::command_system::commands::remove_director::RmDir;
use crate::command_system::commands::remove_file::RemoveFile;
use crate::command_system::commands::users::ListUsers;
use crate::command_system::commands::word_count::WordCount;
use crate::command_system::common::Format;
use crate::command_system::{
    commands::list_files::ListFiles,
    common::{Command, get_format},
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
/// A command runner that executes individual shell commands within a secure environment.
/// 
/// `RunCommand` serves as a dispatcher that routes commands to their appropriate handlers
/// and manages the execution context including current directory, root directory constraints,
/// input piping, and user session management.
/// 
/// The runner supports a wide range of Unix-like commands including file operations,
/// directory management, text processing, and system utilities, all executed within
/// a sandboxed environment for security.
/// 
/// # Supported Commands
/// 
/// - **File Operations**: `cat`, `mv`, `rm`, `ls`
/// - **Directory Operations**: `cd`, `pwd`, `mkdir`, `rmdir`
/// - **Text Processing**: `echo`, `grep`, `wc` (word count)
/// - **System Utilities**: `who`, `users`
/// - **Executable Files**: `./filename` (execute local programs)
/// 
/// # Examples
/// 
/// ```rust
/// use std::path::PathBuf;
/// use std::sync::Arc;
/// use tokio::sync::RwLock;
/// 
/// let users = Arc::new(RwLock::new(vec!["alice".to_string()]));
/// let command = Command { cmd: vec!["ls".to_string(), "-la".to_string()], op: None };
/// 
/// let mut runner = RunCommand::new(
///     PathBuf::from("/home/user"),
///     PathBuf::from("/home"),
///     &command,
///     None,
///     users
/// );
/// 
/// let (new_path, output, success) = runner.test().await;
/// ```
pub struct RunCommand {
    path: std::path::PathBuf,
    command: Command,
    input: Option<String>,
    root: std::path::PathBuf,
    users_list: Arc<RwLock<Vec<String>>>,
}
enum Commands {
    ChangeDirectory,
    PrintWorkingDirectory,
    ListFiles,
    Echo,
    Cat,
    Grep,
    MoveFileAndDir,
    MakeDir,
    RemoveDir,
    WordCount,
    Remove,
    Users,
    Unknown(String),
}

impl Commands {
    /// Converts a command string to the corresponding `Commands` enum variant.
    /// 
    /// This method performs case-sensitive matching of command names to their
    /// corresponding enum variants. It supports command aliases (e.g., both
    /// "cd" and "next" map to `ChangeDirectory`).
    /// 
    /// # Parameters
    /// 
    /// - `cmd`: The command string to parse (e.g., "ls", "cd", "grep")
    /// 
    /// # Returns
    /// 
    /// The corresponding `Commands` enum variant, or `Commands::Unknown` if
    /// the command is not recognized.
    /// 
    /// # Command Mappings
    /// 
    /// | Command String | Enum Variant | Description |
    /// |----------------|--------------|-------------|
    /// | `cd`, `next` | `ChangeDirectory` | Change directory |
    /// | `pwd` | `PrintWorkingDirectory` | Print working directory |
    /// | `ls` | `ListFiles` | List files |
    /// | `echo` | `Echo` | Echo text |
    /// | `cat` | `Cat` | Concatenate files |
    /// | `grep` | `Grep` | Search patterns |
    /// | `mv` | `MoveFileAndDir` | Move/rename files |
    /// | `mkdir` | `MakeDir` | Create directories |
    /// | `rm` | `Remove` | Remove files |
    /// | `rmdir` | `RemoveDir` | Remove directories |
    /// | `wc` | `WordCount` | Word count |
    /// | `who`, `users` | `Users` | List users |
    /// | *other* | `Unknown(String)` | Unrecognized command |
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let cmd = Commands::from_str("ls");
    /// match cmd {
    ///     Commands::ListFiles => println!("List files command"),
    ///     Commands::Unknown(s) => println!("Unknown command: {}", s),
    ///     _ => println!("Other command"),
    /// }
    /// ```
    fn from_str(cmd: &str) -> Self {
        match cmd {
            "cd" | "next" => Commands::ChangeDirectory,
            "pwd" => Commands::PrintWorkingDirectory,
            "ls" => Commands::ListFiles,
            "echo" => Commands::Echo,
            "wc" => Commands::WordCount,
            "cat" => Commands::Cat,
            "grep" => Commands::Grep,
            "mv" => Commands::MoveFileAndDir,
            "mkdir" => Commands::MakeDir,
            "rm" => Commands::Remove,
            "rmdir" => Commands::RemoveDir,
            "who" | "users" => Commands::Users,
            other => Commands::Unknown(other.to_string()),
        }
    }
}

impl RunCommand {
     /// Creates a new `RunCommand` instance for executing a specific command.
    /// 
    /// # Parameters
    /// 
    /// - `current_path`: Current working directory where the command will be executed
    /// - `root`: Root directory for security sandboxing (commands cannot access parent directories)
    /// - `command`: The command structure containing the command name and arguments
    /// - `input`: Optional input string (typically from pipe operations)
    /// - `users`: Thread-safe reference to the list of active users
    /// 
    /// # Returns
    /// 
    /// A new `RunCommand` instance ready for execution.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let command = Command {
    ///     cmd: vec!["cat".to_string(), "file.txt".to_string()],
    ///     op: None
    /// };
    /// 
    /// let runner = RunCommand::new(
    ///     PathBuf::from("/home/user"),
    ///     PathBuf::from("/home"),
    ///     &command,
    ///     Some("input data".to_string()),
    ///     Arc::new(RwLock::new(vec!["user1".to_string()]))
    /// );
    /// ```
    pub fn new(
        current_path: PathBuf,
        root: PathBuf,
        command: &Command,
        input: Option<String>,
        users: Arc<RwLock<Vec<String>>>,
    ) -> Self {
        Self {
            path: current_path,
            root,
            command: command.clone(),
            input,
            users_list: users,
        }
    }
    /// Executes the command and returns the result with updated state.
    /// 
    /// This method serves as the main execution dispatcher. It:
    /// 1. Parses the command name to determine the command type
    /// 2. Routes the command to the appropriate handler
    /// 3. Manages special cases for input handling (pipes)
    /// 4. Updates the current directory state for directory-changing commands
    /// 5. Handles executable file execution for `./filename` patterns
    /// 
    /// # Returns
    /// 
    /// A tuple containing:
    /// - `PathBuf`: Updated current working directory after command execution
    /// - `Option<String>`: Command output (None if no output or suppressed)
    /// - `bool`: Success status (true if command executed successfully)
    /// 
    /// # Command-Specific Behaviors
    /// 
    /// ## Directory Commands
    /// - **`cd`**: Changes current directory and updates internal path state
    /// - **`pwd`**: Returns relative path from root directory
    /// 
    /// ## File Operations
    /// - **`ls`**: Lists directory contents (output suppressed if input provided)
    /// - **`cat`**: Displays file contents
    /// - **`mv`**: Moves/renames files and directories
    /// - **`rm`**: Removes files
    /// - **`mkdir`**: Creates directories
    /// - **`rmdir`**: Removes empty directories
    /// 
    /// ## Text Processing
    /// - **`echo`**: Outputs text (suppressed if input provided)
    /// - **`grep`**: Searches for patterns in input or files
    /// - **`wc`**: Counts words, lines, and characters
    /// 
    /// ## System Commands
    /// - **`who`/`users`**: Lists currently active users
    /// 
    /// ## Executable Files
    /// - **`./filename`**: Executes local executable files
    /// 
    /// ## Unknown Commands
    /// - Returns error message for unrecognized commands
    /// 
    /// # Input Handling
    /// 
    /// For certain commands (`ls`, `echo`), if input is provided (typically from pipes),
    /// the output is suppressed to prevent interfering with pipe operations.
    /// 
    /// # Error Handling
    /// 
    /// - Commands that fail return `false` for success status
    /// - Unknown commands return formatted error messages
    /// - Individual command handlers manage their own error conditions
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let mut runner = RunCommand::new(/* ... */);
    /// let (new_path, output, success) = runner.test().await;
    /// 
    /// if success {
    ///     println!("Command output: {}", output.unwrap_or_default());
    ///     println!("New directory: {:?}", new_path);
    /// } else {
    ///     eprintln!("Command failed");
    /// }
    /// ```
    pub async fn test(&mut self) -> (PathBuf, Option<String>, bool) {
        let command = Commands::from_str(&self.command.cmd[0]);
        let (output, succes) = match command {
            Commands::ChangeDirectory => {
                let cd = ChangeDIR::new(self.command.clone(), self.path.clone(), self.root.clone());
                let (path, new_output, new_succes) = cd.get_new_path_or_output();
                self.path = path;

                (Some(new_output), new_succes)
            }
            Commands::ListFiles => {
                let list = ListFiles::new(self.path.clone(), self.command.clone());
                let (new_output, new_succes) = list.get_output();
                if self.input.is_some() {
                    (Some("".to_string()), new_succes)
                } else {
                    (Some(new_output), new_succes)
                }
            }
            Commands::Cat => {
                let cat = Cat::new(self.command.clone(), self.path.clone());
                let (new_output, new_succes) = cat.get_output();
                (Some(new_output), new_succes)
            }
            Commands::WordCount => {
                let wc =
                    WordCount::new(self.command.clone(), self.input.clone(), self.path.clone());
                let (new_output, new_succes) = wc.get_output();
                (Some(new_output), new_succes)
            }
            Commands::Grep => {
                let grep = Grep::new(self.command.clone(), self.input.clone(), self.path.clone());
                let (new_output, new_succes) = grep.get_output();
                (Some(new_output), new_succes)
            }
            Commands::Users => {
                let who = ListUsers::new(self.command.clone(), self.users_list.clone());
                let (new_output, new_succes) = who.get_output().await;
                (Some(new_output), new_succes)
            }
            Commands::PrintWorkingDirectory => {
                let out = self
                    .path
                    .strip_prefix(&self.root)
                    .unwrap_or(std::path::Path::new("/"))
                    .to_string_lossy()
                    .to_string();
                (
                    Some(format!(
                        "{}home:/{}{}",
                        get_format(Format::Normal),
                        out,
                        get_format(Format::Split)
                    )),
                    true,
                )
            }
            Commands::MakeDir => {
                let mkdir = MakeDir::new(self.command.clone(), self.path.clone());
                let (new_output, new_succes) = mkdir.get_output();
                (Some(new_output), new_succes)
            }
            Commands::RemoveDir => {
                let rmdir = RmDir::new(self.command.clone(), self.path.clone());
                let (new_output, new_succes) = rmdir.get_output();
                (Some(new_output), new_succes)
            }
            Commands::Remove => {
                let rm = RemoveFile::new(self.command.clone(), self.path.clone());
                let (new_output, new_succes) = rm.get_output();
                (Some(new_output), new_succes)
            }
            Commands::MoveFileAndDir => {
                let mv = MoveFileAndDir::new(self.command.clone(), self.path.clone());
                let (new_output, new_succes) = mv.get_output();
                (Some(new_output), new_succes)
            }
            Commands::Echo => {
                let echo = Echo::new(self.command.clone());
                let (new_output, new_succes) = echo.get_output();
                if self.input.is_some() {
                    (Some("".to_string()), new_succes)
                } else {
                    (Some(new_output), new_succes)
                }
            }
            Commands::Unknown(cmd) => {
                if cmd.starts_with("./") {
                    let exe = Execute::new(self.command.clone(), self.path.clone());
                    let (new_output, new_succes) = exe.get_output().await;
                    (Some(new_output), new_succes)
                } else {
                    let new_succes = false;
                    let new_output = Some(format!(
                        "{}Error , Command {} not found {}",
                        get_format(Format::Error),
                        cmd,
                        get_format(Format::Split)
                    ));
                    (new_output, new_succes)
                }
            }
        };

        (self.path.clone(), output, succes)
    }
}
