use crate::command_system::common::get_commands;
use bcrypt::verify;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
pub struct UserLogin {
    input: String,
    passwords_path: std::path::PathBuf,
    users_list: Arc<RwLock<Vec<String>>>,
}
impl UserLogin {
    pub fn new(input: String, path: std::path::PathBuf, users: Arc<RwLock<Vec<String>>>) -> Self {
        UserLogin {
            input,
            passwords_path: path,
            users_list: users,
        }
    }
    fn get_hashmap_password(&self) -> HashMap<String, String> {
        let mut hashmap = HashMap::new();
        let users_password = std::fs::read_to_string(&self.passwords_path)
            .unwrap_or_else(|_| panic!("Error , password file"));
        let split_text: Vec<String> = users_password
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let mut i = 0;
        while i < split_text.len() {
            if i + 1 < split_text.len() {
                hashmap.insert(split_text[i].clone(), split_text[i + 1].clone());
            }
            i += 2;
        }
        hashmap
    }
    pub async fn get_login_status(&self) -> Result<String, String> {
        let cmd = get_commands(self.input.clone());
        let pass_map = self.get_hashmap_password();
        if cmd[0].cmd[0] == "login" {
            if cmd[0].cmd.len() == 3 {
                let username = cmd[0].cmd[1].clone();
                if let Some(stored_hash) = pass_map.get(&username) {
                    let password = cmd[0].cmd[2].clone();
                    if verify(password, stored_hash).unwrap_or(false) {
                        let users_lock = self.users_list.read().await;
                        let user_exists = users_lock
                            .iter()
                            .any(|user_entry| user_entry.contains(&username));

                        if user_exists {
                            Err(format!(
                                "{}[-]:{}[-]\r\n\r\n",
                                "?&EUser already logged in", ""
                            ))
                        } else {
                            Ok(username)
                        }
                    } else {
                        Err(format!("{}[-]:{}[-]\r\n\r\n", "?&EIncorrect password", ""))
                    }
                } else {
                    Err(format!("{}[-]:{}[-]\r\n\r\n", "?&EInvalid username", ""))
                }
            } else {
                Err(format!(
                    "{}[-]:{}[-]\r\n\r\n",
                    "?&EInvalid format:>login [USERNAME] [PASSWORD] ", ""
                ))
            }
        } else {
            Err(format!(
                "{}[-]:{}[-]\r\n\r\n",
                "?&EError: You are not connected. Please connect before sending a command", ""
            ))
        }
    }
}
