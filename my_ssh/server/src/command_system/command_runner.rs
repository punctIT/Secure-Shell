use std::path::PathBuf;
use crate::command_system::common::Command;
pub struct RunCommand{
    path:std::path::PathBuf,
    command:Command,
    input:Option<String>
}
enum Commands {
    ChangeDirectory,
    Unknown(String),
}

impl Commands {
    fn from_str(cmd: &str) -> Self {
        match cmd {
            "cd"|"next"=>Commands::ChangeDirectory,
            other => Commands::Unknown(other.to_string()),
        }
    }
}


impl RunCommand{
    pub fn new(current_path:PathBuf,command:&Command,input:Option<String>)->Self{
        Self { 
            path:current_path,
            command:command.clone(),
            input:input
        }
    }
    pub fn test(&mut self)->(PathBuf,Option<String>){
        let mut output:String= String::new();
        let command = Commands::from_str(&self.command.cmd[0]);
        match command {
            Commands::ChangeDirectory=>{
                let new_path=self.path.join(&self.command.cmd[1]);
                if new_path.exists(){
                    self.path=new_path;
                }
            }
            Commands::Unknown(cmd)=>{
                output=format!("Error , Command {} not found",cmd).to_string();
            }
        }
        
        if output.is_empty(){
            return (self.path.clone(),None);
        }
        else {
            return (self.path.clone(),Some(output));
        }
        
    }
}