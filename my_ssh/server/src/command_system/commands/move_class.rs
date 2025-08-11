use crate::command_system::common::{Command, Format, get_format};

pub struct MoveFileAndDir {
    command: Command,
    current_dir: std::path::PathBuf,
}

impl MoveFileAndDir {
    pub fn new(cmd: Command, path: std::path::PathBuf) -> Self {
        MoveFileAndDir {
            command: cmd,
            current_dir: path,
        }
    }
    fn try_to_move(&self) -> Result<(String, bool), Box<dyn std::error::Error>> {
        let mut output = String::new();
        let mut status = true;
        let path = self.current_dir.join(self.command.cmd.last().unwrap());
        let parent = path.parent().unwrap_or(&path);
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
        for cmd in &self.command.cmd[1..self.command.cmd.len() - 1] {
            let mut output_cmd = String::new();
            let old_path = self.current_dir.join(cmd);
            if old_path.exists() {
                if let Err(e) = std::fs::rename(old_path, &path) {
                    output_cmd = format!("Unexpected Error {}", e);
                    status = false;
                }
            } else {
                output_cmd = format!("mv: cannot stat '{}': No such file or directory ", cmd);
                status = false;
            }
            if output.is_empty() {
                output = output_cmd;
            } else if !output_cmd.is_empty() {
                output = format!("{}\n{}", output, output_cmd);
            }
        }
        if output.is_empty() {
            return Ok((String::from(""), status));
        }
        Ok((
            format!(
                "{}{}{}",
                get_format(Format::Normal),
                output,
                get_format(Format::Split)
            ),
            status,
        ))
    }
    pub fn get_output(&self) -> (String, bool) {
        if self.command.cmd.len() <= 2 {
            (
                format!(
                    "{}{}{}",
                    get_format(Format::Error),
                    String::from("mv: missing destination file"),
                    get_format(Format::Split)
                ),
                true,
            )
        } else {
            match self.try_to_move() {
                Ok(output) => output,
                Err(e) => (
                    format!(
                        "{}{}{}",
                        get_format(Format::Error),
                        e,
                        get_format(Format::Split)
                    ),
                    true,
                ),
            }
        }
    }
}
