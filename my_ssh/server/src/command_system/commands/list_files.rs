use std::path::PathBuf;

use crate::command_system::common::{Command, Format, get_files, get_format, is_executable};

pub struct ListFiles {
    current_dir: std::path::PathBuf,
    command: Command,
}

impl ListFiles {
    pub fn new(path: std::path::PathBuf, cmd: Command) -> Self {
        ListFiles {
            current_dir: path.canonicalize().unwrap_or(path),
            command: cmd,
        }
    }
    fn get_files_current_dir(&self) -> (String, bool) {
        let mut output: String = String::new();
        let files = get_files(&self.current_dir).unwrap();
        for file in files {
            let f = file
                .strip_prefix(&self.current_dir)
                .unwrap()
                .to_str()
                .unwrap_or("default");
            if file.is_file() {
                let status = is_executable(file.clone()).unwrap_or(false);
                if status {
                    output += format!(
                        "{}{}{}",
                        get_format(Format::Color("GREEN")),
                        f,
                        get_format(Format::Split)
                    )
                    .as_str();
                } else {
                    output += format!("{}{}", f, get_format(Format::Split)).as_str();
                }
            } else {
                output += format!(
                    "{}{}{}",
                    get_format(Format::Color("BLUE")),
                    f,
                    get_format(Format::Split)
                )
                .as_str();
            }
        }
        output = format!(
            "{}{}{}",
            get_format(Format::ListDir),
            get_format(Format::Split),
            output
        );
        (output, true)
    }
    fn get_files_in_dir_name(&self) -> (String, bool) {
        let mut output: String = String::new();
        let mut succes = false;
        let paths: Vec<PathBuf> = self.command.cmd[1..]
            .iter()
            .map(|f| self.current_dir.join(f))
            .filter(|f| f.starts_with(&self.current_dir))
            .collect();
        for path in &paths {
            if !path.exists() || path.is_file() {
                output += format!(
                    "{}ls: cannot access '{}': No such file or directory\n{}",
                    get_format(Format::Error),
                    path.strip_prefix(&self.current_dir)
                        .unwrap_or(path)
                        .to_string_lossy(),
                    get_format(Format::Split)
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
                        .strip_prefix(path.canonicalize().unwrap())
                        .unwrap_or(std::path::Path::new(""))
                        .to_str()
                        .unwrap_or("default");
                    if file.is_file() {
                        out += format!("{}{}", f, get_format(Format::Split)).as_str();
                    } else {
                        out += format!(
                            "{}{}{}",
                            get_format(Format::Color("BLUE")),
                            f,
                            get_format(Format::Split)
                        )
                        .as_str();
                    }
                    succes = true;
                }
                output = format!(
                    "{}{}{}{}",
                    output,
                    get_format(Format::ListDir),
                    get_format(Format::Split),
                    out
                );
            }
        }
        (output, succes)
    }
    pub fn get_output(&self) -> (String, bool) {
        if self.command.cmd.len() == 1 {
            self.get_files_current_dir()
        } else {
            //dbg!(&self.command.cmd);
            self.get_files_in_dir_name()
        }
    }
}
