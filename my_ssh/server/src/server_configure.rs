pub struct Configure {}
impl Configure {
    pub fn new() -> Self {
        Configure {}
    }
    /// Prompts the user to enter a certificate key file path and validates it.
    ///
    /// This method continuously prompts the user until a valid file path is provided.
    /// The path must exist and must be a file (not a directory).
    ///
    /// # Returns
    ///
    /// A `String` containing the validated certificate key file path.
    ///
    /// # Panics
    ///
    /// Panics if there's an error reading from stdin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = Configure::new();
    /// let cert_key_path = config.set_cert_key_path();
    /// println!("Certificate key path: {}", cert_key_path);
    /// ```
    pub fn set_cert_key_path(&self) -> String {
        let mut input = String::new();
        loop {
            println!("Enter cert_key path");
            std::io::stdin().read_line(&mut input).expect("Read Error");
            let path = std::path::PathBuf::from(&input.trim());
            if path.exists() && path.is_file() {
                break;
            } else {
                println!("Invalid path: Enter a valid cert key path");
            }
        }
        input.trim().to_string()
    }
    /// Prompts the user to enter a certificate file path and validates it.
    ///
    /// This method continuously prompts the user until a valid file path is provided.
    /// The path must exist and must be a file (not a directory).
    ///
    /// # Returns
    ///
    /// A `String` containing the validated certificate file path.
    ///
    /// # Panics
    ///
    /// Panics if there's an error reading from stdin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = Configure::new();
    /// let cert_path = config.set_cert_path();
    /// println!("Certificate path: {}", cert_path);
    /// ```
    pub fn set_cert_path(&self) -> String {
        let mut input = String::new();
        loop {
            println!("Enter cert path");
            std::io::stdin().read_line(&mut input).expect("Read Error");
            let path = std::path::PathBuf::from(&input.trim());
            if path.exists() && path.is_file() {
                break;
            } else {
                println!("Invalid path: Enter a valid cert path");
            }
        }
        input.trim().to_string()
    }
    /// Prompts the user to enter a working directory path and validates it.
    ///
    /// This method continuously prompts the user until a valid directory path is provided.
    /// The path must exist and must be a directory (not a file).
    ///
    /// # Returns
    ///
    /// A `String` containing the validated working directory path.
    ///
    /// # Panics
    ///
    /// Panics if there's an error reading from stdin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = Configure::new();
    /// let working_dir = config.set_working_directory();
    /// println!("Working directory: {}", working_dir);
    /// ```
    pub fn set_working_directory(&self) -> String {
        let mut input = String::new();
        loop {
            println!("Enter working directory path");
            std::io::stdin().read_line(&mut input).expect("Read Error");
            let path = std::path::PathBuf::from(&input.trim());
            if path.exists() && path.is_dir() {
                break;
            } else {
                println!("Invalid path: Enter a valid director");
            }
        }
        input.trim().to_string()
    }
    /// Prompts the user to enter a password file path and validates it.
    ///
    /// This method continuously prompts the user until a valid file path is provided.
    /// The path must exist and must be a file (not a directory).
    ///
    /// # Returns
    ///
    /// A `String` containing the validated password file path.
    ///
    /// # Panics
    ///
    /// Panics if there's an error reading from stdin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let config = Configure::new();
    /// let password_file = config.set_password_file();
    /// println!("Password file: {}", password_file);
    /// ```
    pub fn set_password_file(&self) -> String {
        let mut input = String::new();
        loop {
            println!("Enter password file path");
            std::io::stdin().read_line(&mut input).expect("Read Error");
            let path = std::path::PathBuf::from(&input.trim());
            if path.exists() && path.is_file() {
                break;
            } else {
                println!("Invalid path: Enter a valid file path");
            }
        }
        input.trim().to_string()
    }
}
