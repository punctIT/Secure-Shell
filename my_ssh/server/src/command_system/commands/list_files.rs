use std::path::PathBuf;

use crate::command_system::common::{Command, Format, get_files, get_format};

pub struct ListFiles {
    current_dir: std::path::PathBuf,
    command: Command,
}

impl ListFiles {
    pub fn new(path: std::path::PathBuf, cmd: Command) -> Self {
        ListFiles {
            current_dir: path,
            command: cmd,
        }
    }
    pub fn get_output(&self) -> String {
        let mut output: String = String::new();
        if self.command.cmd.len() == 1 {
            let files = get_files(&self.current_dir).unwrap();
            for file in files {
                let f = file
                    .strip_prefix(&self.current_dir)
                    .unwrap()
                    .to_str()
                    .unwrap_or("default");
                if file.is_file() {
                    output += format!("{}{}", f, get_format(Format::SplitFormat)).as_str();
                } else {
                    output += format!(
                        "{}{}{}",
                        get_format(Format::ColorFormat("CYAN")),
                        f,
                        get_format(Format::SplitFormat)
                    )
                    .as_str();
                }
            }
            output = format!(
                "{}{}{}",
                get_format(Format::ListDirFormat),
                get_format(Format::SplitFormat),
                output
            );
        } else {
            //dbg!(&self.command.cmd);

            let paths: Vec<PathBuf> = self.command.cmd[1..]
                .iter()
                .map(|f| self.current_dir.join(f))
                .collect();
            for path in &paths {
                if !path.exists() || path.is_file() {
                    output += format!(
                        "{}ls: cannot access '{}': No such file or directory\n{}",
                        get_format(Format::ErrorFormat),
                        path.strip_prefix(&self.current_dir)
                            .unwrap_or(&path)
                            .to_string_lossy()
                            .to_string(),
                        get_format(Format::SplitFormat)
                    )
                    .as_str();
                }
            }
            for path in paths {
                let mut out = String::new();
                if path.is_dir() {
                    let files = get_files(&path).unwrap();
                    for file in files {
                        let f = file
                            .strip_prefix(&path)
                            .unwrap()
                            .to_str()
                            .unwrap_or("default");
                        if file.is_file() {
                            out += format!("{}{}", f, get_format(Format::SplitFormat)).as_str();
                        } else {
                            out += format!(
                                "{}{}{}",
                                get_format(Format::ColorFormat("CYAN")),
                                f,
                                get_format(Format::SplitFormat)
                            )
                            .as_str();
                        }
                    }
                    output = format!(
                        "{}{}{}{}",
                        output,
                        get_format(Format::ListDirFormat),
                        get_format(Format::SplitFormat),
                        out
                    );
                }
            }
        }

        output
    }
}
