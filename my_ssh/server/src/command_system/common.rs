use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Command {
    pub cmd: Vec<String>,
    pub op: Option<String>,
}

pub fn get_files(path: &std::path::Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let dir = std::fs::read_dir(path)?;
    let mut files: Vec<PathBuf> = Vec::new();
    for entry_res in dir {
        let entry = entry_res?;
        let file_path = entry.path();
        let file_path = std::fs::canonicalize(&file_path)?;
        files.push(file_path.clone());
    }
    Ok(files)
}

pub enum Format {
    Error,
    ListDir,
    Color(&'static str),
    Split,
    Normal,
    NormalColored,
}
pub fn get_format(format: Format) -> &'static str {
    match format {
        Format::Error => "?&E",
        Format::ListDir => "?&L",
        Format::NormalColored => "?&C",
        Format::Normal => "?&N",
        Format::Color("BLUE") => "^!",
        Format::Color("LIGHT_RED") => "^@",
        Format::Color("stop") => "~~",
        Format::Split => "\n\n",
        _ => "",
    }
}
pub fn get_unformated_text(text: &str) -> String {
    let mut new_text = String::new();
    let props: Vec<&str> = text.split("?&").filter(|f| !f.is_empty()).collect();
    for w in props {
        let word: Vec<&str> = w[1..].split("\n\n").filter(|f| !f.is_empty()).collect();
        for e in word {
            if e.starts_with('^') {
                let tail: String = e.chars().skip(2).collect();
                if new_text.is_empty() {
                    new_text = tail;
                } else {
                    new_text = format!("{} {}", new_text, tail);
                }
            } else if new_text.is_empty() {
                new_text = e.to_string();
            } else {
                new_text = format!("{} {}", new_text, e);
            }
        }
    }
    new_text
}
