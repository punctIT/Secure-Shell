use std::path::PathBuf;

#[derive(Clone)]
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
