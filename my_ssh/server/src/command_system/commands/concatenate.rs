use crate::command_system::common::{Command, Format, get_format};

pub struct Cat {
    command: Command,
    current_dir: std::path::PathBuf,
}

impl Cat {
    pub fn new(cmd: Command, path: std::path::PathBuf) -> Self {
        Cat {
            command: cmd,
            current_dir: path,
        }
    }
    pub fn get_output(&self) -> (String, bool) {
        let mut output = String::new();
        if self.command.cmd.len() != 1 {
            for file in &self.command.cmd[1..] {
                let new_path = self.current_dir.join(file);
                if new_path.exists() && new_path.is_file() {
                    let text = std::fs::read_to_string(&new_path).unwrap_or("".to_string());
                    //dbg!(&text);
                    if output.is_empty() {
                        output = text;
                    } else {
                        output = format!("{}\n{}", output, text);
                    }
                }
                if !new_path.exists() {
                    if output.is_empty() {
                        output = format!("cat: {}: No such file or directory", file);
                    } else {
                        output = format!("{}\ncat: {}: No such file or directory", output, file);
                    }
                }
                if new_path.is_dir() {
                    if output.is_empty() {
                        output = format!("cat: {}: No such file or directory", file);
                    } else {
                        output = format!("{}\ncat: {}: No such file or directory", output, file);
                    }
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
            true,
        )
    }
}
