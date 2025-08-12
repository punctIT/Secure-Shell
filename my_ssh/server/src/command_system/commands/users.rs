use crate::command_system::common::{Command, Format, get_format};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ListUsers {
    command: Command,
    users_list: Arc<RwLock<Vec<String>>>,
}

impl ListUsers {
    pub fn new(cmd: Command, users: Arc<RwLock<Vec<String>>>) -> Self {
        ListUsers {
            command: cmd,
            users_list: users,
        }
    }
    pub async fn get_output(&self) -> (String, bool) {
        let mut output = String::new();
        let mut status = false;
        if self.command.cmd.len() == 1 {
            let users_lock = self.users_list.read().await;
            status = true;
            for user in users_lock.iter() {
                if output.is_empty() {
                    output = user.clone();
                } else {
                    output = format!("{}\n{}", output, user);
                }
            }
            output = format!(
                "{}{}{}",
                get_format(Format::Normal),
                output,
                get_format(Format::Split)
            );
        }

        (output, status)
    }
}
