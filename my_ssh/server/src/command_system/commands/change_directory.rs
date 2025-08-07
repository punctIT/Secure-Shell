use crate::command_system::common::{Command, Format, get_format};

pub struct ChangeDIR {
    command: Command,
    current_dir: std::path::PathBuf,
    root: std::path::PathBuf,
}
impl ChangeDIR {
    pub fn new(command: Command, path: std::path::PathBuf, root: std::path::PathBuf) -> Self {
        ChangeDIR {
            command,
            current_dir: path,
            root,
        }
    }
    pub fn get_new_path_or_output(&self) -> (std::path::PathBuf, String, bool) {
        let mut succes = true;
        let mut output = String::from("");
        let mut new_path = self.current_dir.clone();
        if self.command.cmd.len() == 1 {
            new_path = self.root.clone();
        } else if self.command.cmd.len() == 2 {
            let path_file = self.current_dir.join(&self.command.cmd[1]);
            if path_file.exists() && path_file.is_dir() {
                new_path = path_file.clone();
            }
            if !path_file.exists() {
                output = format!(
                    "{}{}{}",
                    get_format(Format::Error),
                    "cd: No such file or directory",
                    get_format(Format::Split),
                );
            }
            if path_file.is_file() {
                output = format!(
                    "{}{}{}",
                    get_format(Format::Error),
                    "cd: Not a directory",
                    get_format(Format::Split),
                );
            }
        } else {
            output = format!(
                "{}{}{}",
                get_format(Format::Error),
                "cd: too many arguments",
                get_format(Format::Split),
            );
            succes = false;
        }
        (new_path, output, succes)
    }
}
