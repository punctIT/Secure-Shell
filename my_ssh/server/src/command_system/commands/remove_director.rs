use std::fs;

use crate::command_system::common::{Command, Format, get_format};

pub struct RmDir {
    command: Command,
    current_dir: std::path::PathBuf,
}

impl RmDir {
    pub fn new(cmd: Command, path: std::path::PathBuf) -> Self {
        RmDir {
            command: cmd,
            current_dir: path,
        }
    }
    fn try_remove_folders(&self) -> (String, bool) {
        let mut output: String = String::new();
        let mut succes = true;
        for cmd in &self.command.cmd[1..] {
            let mut line_output = String::new();
            let new_path = self.current_dir.join(cmd);
            if !new_path.exists() {
                succes = false;
                line_output = format!(
                    "rmdir: failed to remove '{}': No such file or directory",
                    cmd
                );
            } else if new_path.is_file() {
                succes = false;
                line_output = format!("r rmdir: failed to remove '{}': Not a directory", cmd);
            } else if let Err(e) = fs::remove_dir(new_path) {
                line_output = format!("Error:{}", e);
                succes = false;
            }
            if !line_output.is_empty() {
                if output.is_empty() {
                    output = line_output;
                } else {
                    output = format!("{}\n{}", output, line_output);
                }
            }
        }
        if output.is_empty() {
            (String::from(""), succes)
        } else {
            (
                format!(
                    "{}{}{}",
                    get_format(Format::Normal),
                    output,
                    get_format(Format::Split)
                ),
                succes,
            )
        }
    }
    pub fn get_output(&self) -> (String, bool) {
        if self.command.cmd.len() == 1 {
            (
                format!(
                    "{}{}{}",
                    get_format(Format::Normal),
                    String::from("rmdir: missing operand"),
                    get_format(Format::Split)
                ),
                false,
            )
        } else {
            self.try_remove_folders()
        }
    }
}
