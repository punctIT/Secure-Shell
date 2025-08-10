use crate::command_system::common::{Command, Format, get_format, is_executable};
pub struct Execute {
    command: Command,
    current_dir: std::path::PathBuf,
}

impl Execute {
    pub fn new(cmd: Command, path: std::path::PathBuf) -> Self {
        Execute {
            command: cmd,
            current_dir: path,
        }
    }
    pub async fn get_executable_output(&self, exe_path: std::path::PathBuf) -> (String, bool) {
        let cmd_clone = self.command.cmd.clone();

        tokio::task::spawn_blocking(move || {
            let args: Vec<String> = match cmd_clone.len() {
                1 => Vec::new(),
                _ => cmd_clone[1..].to_vec(),
            };
            if let Ok(output) = std::process::Command::new(exe_path).args(args).output() {
                let stdout_str = String::from_utf8_lossy(&output.stdout);
                let stderr_str = String::from_utf8_lossy(&output.stderr);

                let (data, status) = if stdout_str.is_empty() {
                    (
                        format!(
                            "{}{}{}",
                            get_format(Format::Normal),
                            stderr_str,
                            get_format(Format::Split)
                        ),
                        false,
                    )
                } else {
                    (
                        format!(
                            "{}{}{}",
                            get_format(Format::Normal),
                            stdout_str,
                            get_format(Format::Split)
                        ),
                        true,
                    )
                };
                (data, status)
            } else {
                (
                    format!(
                        "{}{}{}",
                        get_format(Format::Error),
                        "Unexpected error",
                        get_format(Format::Split)
                    ),
                    false,
                )
            }
        })
        .await
        .unwrap_or_else(|_| ("".to_string(), false))
    }
    pub async fn get_output(&self) -> (String, bool) {
        let new_path = self.current_dir.join(&self.command.cmd[0]);
        if new_path.exists() && is_executable(new_path.clone()).unwrap_or(false) {
            self.get_executable_output(new_path).await
        } else if new_path.is_dir() {
            let error = format!(" {}: Is a directory", self.command.cmd[0]);
            (
                format!(
                    "{}{}{}",
                    get_format(Format::Error),
                    error,
                    get_format(Format::Split)
                ),
                false,
            )
        } else if new_path.is_file() {
            let error = format!(
                " {}: cannot execute binary file: Exec format error",
                self.command.cmd[0]
            );
            (
                format!(
                    "{}{}{}",
                    get_format(Format::Error),
                    error,
                    get_format(Format::Split)
                ),
                false,
            )
        } else {
            let error = format!(" {}: No such file or directory", self.command.cmd[0]);
            (
                format!(
                    "{}{}{}",
                    get_format(Format::Error),
                    error,
                    get_format(Format::Split)
                ),
                false,
            )
        }
    }
}
