pub struct Configure {}
impl Configure {
    pub fn new() -> Self {
        return Configure {};
    }
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
    pub fn set_working_direcotry(&self) -> String {
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
