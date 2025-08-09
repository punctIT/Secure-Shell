use crate::command_system::commands::change_directory::ChangeDIR;
use crate::command_system::commands::concatenate::Cat;
use crate::command_system::commands::echo::Echo;
use crate::command_system::commands::global_regular_expresion_print::Grep;
use crate::command_system::commands::word_count::WordCount;
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
    root: std::path::PathBuf,
}
enum Commands {
    ChangeDirectory,
    PrintWorkingDirectory,
    ListFiles,
    Echo,
    Cat,
    Grep,
    WordCount,
    Unknown(String),
}

impl Commands {
    fn from_str(cmd: &str) -> Self {
        match cmd {
            "cd" | "next" => Commands::ChangeDirectory,
            "pwd" => Commands::PrintWorkingDirectory,
            "ls" => Commands::ListFiles,
            "echo" => Commands::Echo,
            "wc" => Commands::WordCount,
            "cat" => Commands::Cat,
            "grep" => Commands::Grep,
            other => Commands::Unknown(other.to_string()),
        }
    }
}

impl RunCommand {
    pub fn new(
        current_path: PathBuf,
        root: PathBuf,
        command: &Command,
        input: Option<String>,
    ) -> Self {
        Self {
            path: current_path,
            root,
            command: command.clone(),
            input,
        }
    }
    pub fn test(&mut self) -> (PathBuf, Option<String>, bool) {
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
                let new_succes = false;
                let new_output = Some(format!(
                    "{}Error , Command {} not found {}",
                    get_format(Format::Error),
                    cmd,
                    get_format(Format::Split)
                ));
                (new_output, new_succes)
            }
        };

        (self.path.clone(), output, succes)
    }
}
