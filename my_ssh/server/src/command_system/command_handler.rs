use crate::command_system::command_runner::RunCommand;
use crate::command_system::common::Command;
use crate::command_system::common::get_commands;
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
            cmds: get_commands(client_input),
            root: std::fs::canonicalize(&path).unwrap_or(path),
            current_dir: std::fs::canonicalize(&current_dir).unwrap_or(current_dir),
        }
    }
    async fn run_commands(&mut self) -> Option<String> {
        let mut final_result: Option<String> = None;
        let mut jump_cmd = false;

        let mut last_succes = true;

        let mut current_dir = self.current_dir.clone();
        let _ = current_dir;
        let mut cmd_succes = true;
        let mut result: Option<String> = None;

        let mut cmds_output = String::from("");
        let mut input: Option<String> = None;
        for (i, cmd) in self.cmds.clone().iter().enumerate() {
            let op = cmd.op.clone().unwrap_or("".to_string());
            if op == "<" {
                if self.cmds[i].cmd.is_empty() {
                    return None;
                }
                input = Some(
                    std::fs::read_to_string(self.current_dir.join(&self.cmds[i + 1].cmd[0]))
                        .unwrap_or("".to_string()),
                );
            }
            if op == ">" && self.cmds[i].cmd.is_empty() {
                return None;
            }
            if !jump_cmd {
                let mut runner = RunCommand::new(
                    self.current_dir.clone(),
                    self.root.clone(),
                    cmd,
                    input.clone(),
                );
                (current_dir, result, cmd_succes) = runner.test().await;
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
            let (mut current_output, output, jump, op_succes) = operation.get_output();
            jump_cmd = jump;
            if op == "|" {
                input = result.clone();
                current_output = String::from("");
            }
            if op == ">" && jump_cmd {
                result = None;
            }
            if op == "||" && !jump_cmd {
                final_result = None;
                current_output = String::from("");
            }

            cmds_output = current_output;
            last_succes = op_succes & cmd_succes;
            //dbg!(&input);
            if !output.is_empty() {
                final_result = Some(final_result.unwrap_or("".to_string()) + output.as_str());
            }
            //dbg!(&cmds_output);
            //dbg!(&final_result);
        }
        final_result
    }

    pub async fn get_output(&mut self) -> (String, PathBuf) {
        //dbg!(&self.cmds);
        let output = self.run_commands().await.unwrap_or("".to_string());

        let current_dir = match self.current_dir.strip_prefix(&self.root) {
            Ok(path) => path,
            Err(_) => {
                self.current_dir = self.root.clone();
                std::path::Path::new("")
            }
        };
        let reply = format!("{}[-]:{}[-]\r\n\r\n", output, current_dir.display());
        //dbg!(&reply);
        (reply, self.current_dir.clone())
    }
}
