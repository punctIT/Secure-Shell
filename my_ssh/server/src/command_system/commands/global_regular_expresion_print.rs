use crate::command_system::common::{Command, Format, get_format, get_unformated_text};

pub struct Grep {
    command: Command,
    current_dir: std::path::PathBuf,
    input: Option<String>,
}

impl Grep {
    pub fn new(cmd: Command, input: Option<String>, path: std::path::PathBuf) -> Self {
        Grep {
            command: cmd,
            current_dir: path,
            input,
        }
    }
    fn get_colored_output(&self, input: String, pattern: &String) -> String {
        let mut output: String = String::new();
        for line in input.lines() {
            let mut line_output = String::new();
            let mut line_has_pattern = false;

            for part in line.split(' ').filter(|s| !s.is_empty()) {
                if part.contains(pattern) {
                    line_has_pattern = true;
                    let mut remaining_part = part;
                    let mut current_pos = 0;

                    while let Some(poz) = remaining_part.find(pattern) {
                        if let Some(before_slice) = remaining_part.get(0..poz) {
                            if line_output.is_empty() && current_pos == 0 {
                                line_output = before_slice.to_string();
                            } else {
                                line_output = format!("{}{}", line_output, before_slice);
                            }
                        }
                        line_output =
                            format!("{}{}", line_output, get_format(Format::Color("LIGHT_RED")));
                        line_output = format!("{}{}", line_output, pattern);
                        line_output =
                            format!("{}{}", line_output, get_format(Format::Color("stop")));
                        let next_start = poz + pattern.len();
                        current_pos += next_start;

                        if next_start >= remaining_part.len() {
                            break;
                        }

                        remaining_part = &remaining_part[next_start..];
                    }
                    if !remaining_part.is_empty() && !remaining_part.contains(pattern) {
                        line_output = format!("{}{}", line_output, remaining_part);
                    }
                }
            }

            if line_has_pattern {
                if !output.is_empty() {
                    output.push('\n');
                }
                output.push_str(&line_output);
            }
        }

        format!(
            "{}{}{}",
            get_format(Format::NormalColored),
            output,
            get_format(Format::Split)
        )
    }
    pub fn get_output(&self) -> (String, bool) {
        let mut output = String::new();
        let mut succes = false;
        if (self.command.cmd.len() < 3 && self.input.is_none())
            || (self.command.cmd.len() == 1 && self.input.is_some())
        {
            output = format!(
                "{}{}{}",
                get_format(Format::Error),
                "Usage: grep PATTERNS [FILE]",
                get_format(Format::Split)
            );
        }
        if self.command.cmd.len() >= 3 {
            let pattern = self.command.cmd[1].clone();
            for cmd in &self.command.cmd[2..] {
                let new_path = self.current_dir.join(cmd);
                if new_path.exists() && new_path.is_file() {
                    let text = std::fs::read_to_string(&new_path).unwrap_or("".to_string());
                    output = self.get_colored_output(text, &pattern);
                }
                if !new_path.exists() {
                    output = format!(
                        "{}{}{}",
                        get_format(Format::Error),
                        "grep: No such file or directory",
                        get_format(Format::Split)
                    );
                }
                if new_path.is_dir() {
                    output = format!(
                        "{}{}{}",
                        get_format(Format::Error),
                        "grep: Is a directory",
                        get_format(Format::Split)
                    );
                }
            }
            succes = true;
        } else if self.command.cmd.len() == 2 && self.input.is_some() {
            let pattern = self.command.cmd[1].clone();
            output = self.get_colored_output(
                get_unformated_text(self.input.clone().unwrap().as_str()),
                &pattern,
            )
        }
        (output, succes)
    }
}
