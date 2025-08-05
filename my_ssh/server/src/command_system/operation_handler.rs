use crate::command_system::common::Command;

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
    command: Command,
}
impl OperationHandler {
    pub fn new(output: String, command: &Command) -> Self {
        OperationHandler {
            output: output,
            command: command.clone(),
        }
    }
    pub fn get_output(&self) -> String {
        let op_str = match &self.command.op {
            Some(i) => i.as_str(),
            None => "",
        };
        let op = Operation::from_str(op_str);
        let output = match op {
            Operation::CommandSeparator => self.output.clone(),
            _ => "".to_string(),
        };
        output
    }
}
