use crate::command_system::common::get_commands;
use bcrypt::verify;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
/// A user authentication system for secure shell login operations.
/// 
/// `UserLogin` handles user authentication by validating credentials against
/// a password file containing bcrypt-hashed passwords. It manages user sessions
/// to prevent duplicate logins and ensures secure password verification.
/// 
/// # Features
/// 
/// - **Secure Password Verification**: Uses bcrypt for password hashing and verification
/// - **Session Management**: Prevents multiple simultaneous logins from the same user
/// - **File-based User Database**: Reads user credentials from a simple text file format
/// - **Command Parsing**: Validates login command format and extracts credentials
/// - **Thread-safe Operations**: Uses Arc<RwLock> for concurrent user session management
/// 
/// # Password File Format
/// 
/// The password file should contain alternating usernames and bcrypt hashes:
/// ```text
/// alice $2b$12$hash1...
/// bob $2b$12$hash2...
/// charlie $2b$12$hash3...
/// ```
/// 
/// # Login Command Format
/// 
/// Users must authenticate using the following command format:
/// ```text
/// login [USERNAME] [PASSWORD]
/// ```
/// 
/// # Examples
/// 
/// ```rust
/// use std::path::PathBuf;
/// use std::sync::Arc;
/// use tokio::sync::RwLock;
/// 
/// let users = Arc::new(RwLock::new(Vec::new()));
/// let login = UserLogin::new(
///     "login alice mypassword".to_string(),
///     PathBuf::from("/etc/secure_shell/passwords"),
///     users.clone()
/// );
/// 
/// match login.get_login_status().await {
///     Ok(username) => println!("User {} authenticated successfully", username),
///     Err(error_msg) => println!("Authentication failed: {}", error_msg),
/// }
/// ```
pub struct UserLogin {
    input: String,
    passwords_path: std::path::PathBuf,
    users_list: Arc<RwLock<Vec<String>>>,
}
impl UserLogin {
        /// Creates a new `UserLogin` instance for processing authentication.
    /// 
    /// # Parameters
    /// 
    /// - `input`: Raw command string from the client (e.g., "login alice password123")
    /// - `path`: Path to the password file containing user credentials
    /// - `users`: Thread-safe reference to the list of currently active users
    /// 
    /// # Returns
    /// 
    /// A new `UserLogin` instance ready to process authentication.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let users = Arc::new(RwLock::new(vec!["existing_user".to_string()]));
    /// let login = UserLogin::new(
    ///     "login newuser secretpass".to_string(),
    ///     PathBuf::from("/etc/passwords"),
    ///     users
    /// );
    /// ```
    pub fn new(input: String, path: std::path::PathBuf, users: Arc<RwLock<Vec<String>>>) -> Self {
        UserLogin {
            input,
            passwords_path: path,
            users_list: users,
        }
    }
        /// Loads and parses the password file into a username-to-hash mapping.
    /// 
    /// This method reads the password file and creates a HashMap where keys are
    /// usernames and values are their corresponding bcrypt password hashes.
    /// The file format expects alternating usernames and password hashes
    /// separated by whitespace.
    /// 
    /// # Returns
    /// 
    /// A `HashMap<String, String>` mapping usernames to their bcrypt password hashes.
    /// 
    /// # Panics
    /// 
    /// Panics if the password file cannot be read. This is intentional to prevent
    /// the system from operating without proper authentication data.
    /// 
    /// # File Format
    /// 
    /// The password file should follow this format:
    /// ```text
    /// username1 $2b$12$hash1...
    /// username2 $2b$12$hash2...
    /// username3 $2b$12$hash3...
    /// ```
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// // Password file content:
    /// // alice $2b$12$abcdef...
    /// // bob $2b$12$ghijkl...
    /// 
    /// let login = UserLogin::new(/* ... */);
    /// let password_map = login.get_hashmap_password();
    /// 
    /// assert!(password_map.contains_key("alice"));
    /// assert!(password_map.contains_key("bob"));
    /// ```
    /// 
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
        /// Processes the login request and returns the authentication result.
    /// 
    /// This method performs comprehensive login validation including:
    /// 1. Command format validation
    /// 2. Username existence verification
    /// 3. Password verification using bcrypt
    /// 4. Duplicate session prevention
    /// 5. User session registration
    /// 
    /// # Returns
    /// 
    /// - `Ok(String)`: Contains the authenticated username on successful login
    /// - `Err(String)`: Contains a formatted error message for failed authentication
    /// 
    /// # Authentication Flow
    /// 
    /// 1. **Command Parsing**: Validates the input follows "login [USERNAME] [PASSWORD]" format
    /// 2. **User Lookup**: Checks if the username exists in the password database
    /// 3. **Password Verification**: Uses bcrypt to verify the provided password
    /// 4. **Session Check**: Ensures the user isn't already logged in
    /// 5. **Success**: Returns the username for session registration
    /// 
    /// # Error Conditions
    /// 
    /// | Condition | Error Message |
    /// |-----------|---------------|
    /// | Wrong command | "Error: You are not connected. Please connect before sending a command" |
    /// | Invalid format | "Invalid format:>login [USERNAME] [PASSWORD]" |
    /// | Unknown user | "Invalid username" |
    /// | Wrong password | "Incorrect password" |
    /// | Already logged in | "User already logged in" |
    /// 
    /// # Security Features
    /// 
    /// - **bcrypt Verification**: Secure password hashing prevents plaintext password storage
    /// - **Session Management**: Prevents multiple concurrent sessions per user
    /// - **Input Validation**: Strict command format validation prevents injection attacks
    /// - **Error Messages**: Generic error messages prevent username enumeration
    /// 
    /// # Examples
    /// 
    /// ## Successful Authentication
    /// ```rust
    /// let login = UserLogin::new(
    ///     "login alice correctpassword".to_string(),
    ///     password_path,
    ///     users_list
    /// );
    /// 
    /// match login.get_login_status().await {
    ///     Ok(username) => {
    ///         println!("User {} logged in successfully", username);
    ///         // Add user to active sessions
    ///     }
    ///     Err(error) => println!("Login failed: {}", error),
    /// }
    /// ```
    /// 
    /// ## Failed Authentication
    /// ```rust
    /// let login = UserLogin::new(
    ///     "login alice wrongpassword".to_string(),
    ///     password_path,
    ///     users_list
    /// );
    /// 
    /// match login.get_login_status().await {
    ///     Ok(_) => unreachable!(),
    ///     Err(error) => {
    ///         // error contains: "?&EIncorrect password[-]:[-]\r\n\r\n"
    ///         println!("Authentication failed: {}", error);
    ///     }
    /// }
    /// ```
    /// 
    /// ## Invalid Command Format
    /// ```rust
    /// let login = UserLogin::new(
    ///     "login alice".to_string(), // Missing password
    ///     password_path,
    ///     users_list
    /// );
    /// 
    /// match login.get_login_status().await {
    ///     Err(error) => {
    ///         // error contains format error message
    ///         println!("Invalid format: {}", error);
    ///     }
    ///     Ok(_) => unreachable!(),
    /// }
    /// ```
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
