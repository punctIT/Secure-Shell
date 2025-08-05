use crate::command_system::commands::change_directory::ChangeDIR;
use crate::command_system::common::Format;
use crate::command_system::{
    commands::list_files::ListFiles,
    common::{Command, get_format},
};
use std::path::PathBuf;
pub struct RunCommand {
    path: std::path::PathBuf,
    command: Command,
    input: Option<String>,
    root:std::path::PathBuf,
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
    pub fn new(current_path: PathBuf, root:PathBuf,command: &Command, input: Option<String>) -> Self {
        Self {
            path: current_path,
            root,
            command: command.clone(),
            input,
        }
    }
    pub fn test(&mut self) -> (PathBuf, Option<String>) {
        let mut output: Option<String> = None;
        let command = Commands::from_str(&self.command.cmd[0]);
        let _ = self.input;
        match command {
            Commands::ChangeDirectory => {
                let cd= ChangeDIR::new(self.command.clone(), self.path.clone(), self.root.clone());
                let (path,new_output)=cd.get_new_path_or_output();
                self.path=path;
                output=Some(new_output);
            }
            Commands::ListFiles => {
                let list = ListFiles::new(self.path.clone(), self.command.clone());
                output = Some(list.get_output());
            }
            Commands::PrintWorkingDirectory => {
                
            }
            Commands::Unknown(cmd) => {
                output = Some(format!(
                    "{}Error , Command {} not found {}",
                    get_format(Format::ErrorFormat),
                    cmd.to_string(),
                    get_format(Format::SplitFormat)
                ));
            }
        }

        return (self.path.clone(), output);
    }
}
