use std::path::PathBuf;

use crate::command_system::common::{Command, get_files};

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
    pub fn get_output(&self) -> Option<String> {
        let mut output: Option<String> = None;
        if self.command.cmd.len() == 1 {
            let mut output_string: String = String::new();
            let files = get_files(&self.current_dir).unwrap();
            for file in files {
                output_string += format!("{:?}\n", file).as_str();
            }
            output = Some(output_string)
        } 
        else {
            dbg!(&self.command.cmd);
            let mut output_string: String = String::new();
            let paths: Vec<PathBuf> = self.command.cmd[1..]
                .iter()
                .map(|f| self.current_dir.join(f))
                .collect();
            for path in &paths {
                if !path.exists() || path.is_file() {
                    output_string += format!(
                        "ls: cannot access '{}': No such file or directory\n",
                        path.strip_prefix(&self.current_dir)
                            .unwrap_or(&path)
                            .to_string_lossy()
                            .to_string()
                    )
                    .as_str();
                }
            }
            for path in paths {
                if  path.is_dir(){
                    let files = get_files(&path).unwrap();
                     for file in files {
                        output_string += format!("{:?}\n", file).as_str();
                    }
                }
            }
            output = Some(output_string)
        }

        output
    }
}
