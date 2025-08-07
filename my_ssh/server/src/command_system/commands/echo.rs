use crate::command_system::common::{Command, Format, get_format};

pub struct Echo {
    command: Command,
}

impl Echo {
    pub fn new(cmd: Command) -> Self {
        Echo { command: cmd }
    }
    pub fn get_output(&self) -> (String, bool) {
        let mut output = String::new();
        for i in &self.command.cmd[1..] {
            if !i.is_empty() {
                output = format!("{} {}", output, i);
            }
        }
        output = format!(
            "{}{}{}",
            get_format(Format::Normal),
            output,
            get_format(Format::Split)
        );
        (output, true)
    }
}
