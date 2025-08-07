use crate::command_system::common::Command;
use std::io::Write;

enum Operation {
    Pipe,
    InputRedirection,
    OutputRedirection,
    AndLogic,
    OrLogic,
    CommandSeparator,
}

impl Operation {
    fn from_str(cmd: &str) -> Self {
        match cmd {
            "|" => Operation::Pipe,
            "<" => Operation::InputRedirection,
            ">" => Operation::OutputRedirection,
            "&&" => Operation::AndLogic,
            "||" => Operation::OrLogic,
            ";" | "" => Operation::CommandSeparator,
            _ => Operation::CommandSeparator,
        }
    }
}

pub struct OperationHandler {
    output: String,
    last_output: String,
    commands: Vec<Command>,
    current_dir: std::path::PathBuf,
    last_succes: bool,
}
impl OperationHandler {
    pub fn new(
        output: String,
        last_output: String,
        command: &[Command],
        path: std::path::PathBuf,
        last_succes: bool,
    ) -> Self {
        OperationHandler {
            output,
            last_output,
            commands: command.to_owned(),
            current_dir: path,
            last_succes,
        }
    }
    fn write_in_file(
        &self,
        content: &String,
        name: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.current_dir.join(name);
        if !file_path.parent().unwrap().exists() {
            std::fs::create_dir_all(file_path.parent().unwrap())?;
        }
        let mut file = std::fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
    pub fn get_output(&self) -> (String, String, bool, bool) {
        let op_str = match &self.commands[0].op {
            Some(i) => i.as_str(),
            None => "",
        };
        //dbg!(&op_str);
        let op = Operation::from_str(op_str);
        match op {
            Operation::CommandSeparator => {
                let output = self.last_output.clone() + " " + self.output.as_str();
                ("".to_string(), output, false, true)
            }
            Operation::OutputRedirection => {
                if self
                    .write_in_file(&self.output, &self.commands[1].cmd[0])
                    .is_ok()
                {
                    return ("".to_string(), "".to_string(), true, true);
                }
                ("Error".to_string(), "".to_string(), true, false)
            }
            Operation::AndLogic => {
                let mut jump = false;
                let mut output = self.output.clone();
                let mut succes = true;
                if !self.last_succes {
                    jump = true;
                    output = "".to_string();
                    succes = false;
                }
                (
                    self.last_output.clone() + " " + output.as_str(),
                    "".to_string(),
                    jump,
                    succes,
                )
            }
            Operation::InputRedirection => ("".to_string(), "".to_string(), true, true),
            Operation::OrLogic => {
                let mut jump = false;
                let mut output = self.output.clone();
                let mut succes = true;
                if self.last_succes {
                    jump = true;
                    output = "".to_string();
                    succes = true;
                }
                (
                    self.last_output.clone() + " " + output.as_str(),
                    "".to_string(),
                    jump,
                    succes,
                )
            }
            Operation::Pipe => (
                self.last_output.clone() + " " + self.output.as_str(),
                "".to_string(),
                false,
                true,
            ),
        }
    }
}
