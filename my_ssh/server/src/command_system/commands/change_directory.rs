use crate::command_system::common::{Command, Format, get_format};

pub struct ChangeDIR {
    command: Command,
    current_dir: std::path::PathBuf,
    root:std::path::PathBuf,
}
impl ChangeDIR {
    pub fn new(command: Command, path: std::path::PathBuf,root:std::path::PathBuf) -> Self {
        ChangeDIR {
            command,
            current_dir: path,
            root,
        }
    }
    pub fn get_new_path_or_output(&self) -> (std::path::PathBuf, String) {
        let mut output=String::from("");
        let mut new_path=self.current_dir.clone();
        if self.command.cmd.len()==1{
           new_path=self.root.clone();
        }
        else if self.command.cmd.len()==2{
            let path_file = self.current_dir.join(&self.command.cmd[1]);
            if path_file.exists() && path_file.is_dir() {
                new_path=path_file;
            }
        }
        else {
            output=format!("{}{}{}",
                get_format(Format::ErrorFormat),
                "cd: too many arguments",
                get_format(Format::SplitFormat),
            )
        }
        (new_path,output)
       
    }
}
