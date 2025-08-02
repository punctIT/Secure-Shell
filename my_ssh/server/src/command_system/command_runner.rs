use crate::command_system::{commands::list_files::ListFiles, common::Command};
use std::path::PathBuf;
pub struct RunCommand {
    path: std::path::PathBuf,
    command: Command,
    input: Option<String>,
}
enum Commands {
    ChangeDirectory,
    PrintWorkingDirectory,
    ListFiles,
    Unknown(String),
}

impl Commands {
    fn from_str(cmd: &str) -> Self {
        match cmd {
            "cd" | "next" => Commands::ChangeDirectory,
            "pwd" => Commands::PrintWorkingDirectory,
            "ls" => Commands::ListFiles,
            other => Commands::Unknown(other.to_string()),
        }
    }
}

impl RunCommand {
    pub fn new(current_path: PathBuf, command: &Command, input: Option<String>) -> Self {
        Self {
            path: current_path,
            command: command.clone(),
            input: input,
        }
    }
    pub fn test(&mut self) -> (PathBuf, Option<String>) {
        let mut output: Option<String> = None;
        let command = Commands::from_str(&self.command.cmd[0]);
        match command {
            Commands::ChangeDirectory => {
                let new_path = self.path.join(&self.command.cmd[1]);
                if new_path.exists() && new_path.is_dir() {
                    self.path = std::fs::canonicalize(&new_path).unwrap_or(new_path);
                }
            }
            Commands::ListFiles => {
                let list = ListFiles::new(self.path.clone(), self.command.clone());
                output = list.get_output();
            }
            Commands::PrintWorkingDirectory => {}
            Commands::Unknown(cmd) => {
                output = Some(format!("Error , Command {} not found", cmd).to_string());
            }
        }

        return (self.path.clone(), output);
    }
}
