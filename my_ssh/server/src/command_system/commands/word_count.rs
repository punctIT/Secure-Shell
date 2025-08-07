use crate::command_system::common::{Command, Format, get_format};

pub struct WordCount {
    command: Command,
    currect_dir: std::path::PathBuf,
    input: Option<String>,
}

impl WordCount {
    pub fn new(cmd: Command, input: Option<String>, path: std::path::PathBuf) -> Self {
        WordCount {
            command: cmd,
            currect_dir: path,
            input,
        }
    }
    fn get_input_count(&self) -> (String, bool) {
        let mut count = 0;
        let mut status = true;
        if let Some(i) = self.input.clone() {
            if i.is_empty() {
                status = false;
            }
            count = i.split_whitespace().count();
        } else {
            status = false;
        }
        (
            format!(
                "{}{}{}",
                get_format(Format::Normal),
                count,
                get_format(Format::Split)
            ),
            status,
        )
    }
    pub fn get_output(&self) -> (String, bool) {
        let mut status = false;
        if self.command.cmd.len() != 1 {
            let mut output: String = String::new();
            for cmd in &self.command.cmd[1..] {
                let file_path = self.currect_dir.join(cmd);
                if !file_path.exists() {
                    if !output.is_empty() {
                        output =
                            format!("{}\n{}  ({})", output, "wc: No such file or directory", cmd);
                    } else {
                        output = format!("{}  ({})", "wc: No such file or directory", cmd);
                    }
                } else if file_path.is_file() {
                    let content = std::fs::read_to_string(file_path).unwrap_or("".to_string());
                    let count = content.split_whitespace().count();
                    output = format!("{}\n{}  ({})", output, count, cmd);
                    status = true;
                } else if file_path.is_dir() {
                    if !output.is_empty() {
                        output = format!("{}\n{}  ({})", output, "wc: Is a directory", cmd);
                    } else {
                        output = format!("{}  ({})", "wc: Is a directory", cmd);
                    }
                }
            }
            (
                format!(
                    "{}{}{}",
                    get_format(Format::Normal),
                    output,
                    get_format(Format::Split)
                ),
                status,
            )
        } else {
            self.get_input_count()
        }
    }
}
