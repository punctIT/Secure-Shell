pub struct Configure {}
impl Configure {
    pub fn new() -> Self {
       Configure {}
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
    pub fn set_ip_path(&self) -> String {
        let mut input = String::new();
        println!("Enter server IP adress");
        std::io::stdin().read_line(&mut input).expect("Read Error");
        input.trim().to_string()
    }
}
