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
    ErrorFormat,
    ListDirFormat,
    ColorFormat(&'static str),
    SplitFormat,
}
pub fn get_format(format: Format) -> &'static str {
    match format {
        Format::ErrorFormat => "?&E",
        Format::ListDirFormat => "?&L",
        Format::ColorFormat("CYAN") => "^!",
        Format::SplitFormat => "\n\n",
        _ => "",
    }
}
