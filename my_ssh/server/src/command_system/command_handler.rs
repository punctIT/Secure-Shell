use crate::command_system::command_runner::RunCommand;
use crate::command_system::common::{Command, Format, get_format};
use crate::command_system::operation_handler::OperationHandler;
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
        let mut final_result: Option<String> = None;
        let mut jump_cmd = false;

        let mut last_succes = true;

        let mut current_dir = self.current_dir.clone();
        let mut cmd_succes = true;
        let mut result: Option<String> = None;

        let mut cmds_output = String::from("");
        let mut input: Option<String> = None;
        for (i, cmd) in self.cmds.clone().iter().enumerate() {
            let op = cmd.op.clone().unwrap_or("".to_string());
            if op == r"<" {
                input = None;
            }

            if !jump_cmd {
                let mut runner = RunCommand::new(
                    self.current_dir.clone(),
                    self.root.clone(),
                    cmd,
                    input.clone(),
                );
                (current_dir, result, cmd_succes) = runner.test();
                self.current_dir = current_dir; //cd 
            }
            let sliced_cmds: Vec<Command> = self.cmds[i..].to_vec();
            let operation = OperationHandler::new(
                result.clone().unwrap_or("".to_string()),
                cmds_output,
                &sliced_cmds,
                self.current_dir.clone(),
                last_succes & cmd_succes,
            );
            let (current_output, output, jump, op_succes) = operation.get_output();
            jump_cmd=jump;
            cmds_output = current_output;
            last_succes = op_succes & cmd_succes;
            
            if !output.is_empty() {
                final_result = Some(final_result.unwrap_or("".to_string()) + output.as_str());
            }
            dbg!(&cmds_output);
            dbg!(&final_result);
        }
        final_result
    }

    pub fn get_output(&mut self) -> (String, PathBuf) {
        //dbg!(&self.cmds);
        let output = self.run_commands().unwrap_or("12".to_string());

        let current_dir = match self.current_dir.strip_prefix(&self.root) {
            Ok(path) => path,
            Err(_) => {
                self.current_dir = self.root.clone();
                std::path::Path::new("")
            }
        };
        let reply = format!("{}\r\n:{}\r\n", output, current_dir.display());
        //dbg!(&reply);
        (reply, self.current_dir.clone())
    }

    fn get_commands(client_input: String) -> Vec<Command> {
        let mut cmds: Vec<Command> = Vec::new();
        let splited_input: Vec<&str> = client_input.split_whitespace().collect();
        let iter_splited_input = splited_input.iter();
        let op = ["&&", "|", "||", "<", ">", ";"];

        let mut current_cmd: Vec<String> = Vec::new();
        for c in iter_splited_input {
            if op.contains(c) {
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
