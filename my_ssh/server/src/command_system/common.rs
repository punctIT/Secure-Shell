use shell_words::split;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Command {
    pub cmd: Vec<String>,
    pub op: Option<String>,
}

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

pub enum Format {
    Error,
    ListDir,
    Color(&'static str),
    Split,
    Normal,
    NormalColored,
}
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
