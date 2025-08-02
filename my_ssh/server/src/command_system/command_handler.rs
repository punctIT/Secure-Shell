use crate::command_system::command_runner::RunCommand;
use crate::command_system::common::Command;
use std::path::PathBuf;

pub struct CommandHandler {
    cmds: Vec<Command>,
    root: PathBuf,
    current_dir: PathBuf,
}
impl CommandHandler {
    pub fn new(client_input: String, path: PathBuf, current_dir: PathBuf) -> Self {
        CommandHandler {
            cmds: Self::get_commands(client_input),
            root: std::fs::canonicalize(&path).unwrap_or(path),
            current_dir: std::fs::canonicalize(&current_dir).unwrap_or(current_dir),
        }
    }
    fn run_commands(&mut self) -> Option<String> {
        let mut result: Option<String> = None;
        for cmd in &self.cmds {
            let mut runner = RunCommand::new(self.current_dir.clone(), cmd, None);

            (self.current_dir, result) = runner.test();
        }
        result
    }
    pub fn get_output(&mut self) -> (String, PathBuf) {
        let output = self.run_commands().unwrap_or_else(|| "".to_string());

        let current_dir = match self.current_dir.strip_prefix(&self.root) {
            Ok(path) => path,
            Err(_) => {
                self.current_dir = self.root.clone();
                std::path::Path::new("")
            }
        };
        let reply = format!("{}\r\nServer:{}\r\n", output.trim(), current_dir.display());
        return (reply, self.current_dir.clone());
    }

    fn get_commands(client_input: String) -> Vec<Command> {
        let mut cmds: Vec<Command> = Vec::new();
        let splited_input: Vec<&str> = client_input.split_whitespace().collect();
        let mut iter_splited_input = splited_input.iter();
        let op = ["&&", "|", "||", "<", ">", ";"];

        let mut current_cmd: Vec<String> = Vec::new();

        while let Some(c) = iter_splited_input.next() {
            if op.contains(&c.as_ref()) {
                cmds.push(Command {
                    cmd: current_cmd.clone(),
                    op: Some(c.to_string()),
                });
                current_cmd.clear();
            } else {
                current_cmd.push(c.to_string());
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
}
