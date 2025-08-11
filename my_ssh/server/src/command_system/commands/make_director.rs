use std::fs;

use crate::command_system::common::{Command, Format, get_format};

pub struct MakeDir {
    command: Command,
    current_dir: std::path::PathBuf,
}

impl MakeDir {
    pub fn new(cmd: Command, path: std::path::PathBuf) -> Self {
        MakeDir {
            command: cmd,
            current_dir: path,
        }
    }
    fn try_create_folders(&self) -> (String, bool) {
        let mut output: String = String::new();
        let mut succes = true;
        for cmd in &self.command.cmd[1..] {
            let mut line_output = String::new();
            let new_path = self.current_dir.join(cmd);
            if new_path.exists() {
                succes = false;
                line_output = format!("mkdir: cannot create directory `{}`: File exists", cmd);
            } else if let Err(e) = fs::create_dir_all(new_path) {
                    line_output = format!("Error{}:", e);
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
                    String::from("mkdir: missing operand"),
                    get_format(Format::Split)
                ),
                false,
            )
        } else {
            self.try_create_folders()
        }
    }
}
