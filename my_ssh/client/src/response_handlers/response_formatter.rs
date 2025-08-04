use colored::*;
use std::io::Write;
pub struct ShowResponse {
    response: String,
}

impl ShowResponse {
    pub fn new(response: String) -> Self {
        ShowResponse { response: response }
    }
    fn split_response(&self) {
        let props: Vec<&str> = self
            .response
            .split("?&")
            .filter(|f| !f.is_empty())
            .collect();

        for w in props {
            let word: Vec<&str> = w[1..].split("\n\n").collect();
            dbg!(&word);
            match w.chars().next() {
                Some('E') => {
                    println!("{}",word[0]);
                }
                Some(c) => {
                     for e in word {
                        if e.starts_with("^!") {
                            print!("{} ", e[2..].green());
                        } else {
                            print!("{} ", e);
                        }
                        std::io::stdout().flush().unwrap();
                    }
                }
                None => {
                    // stringul e gol
                    println!("String gol");
                }
            }

           
        }
    }
    pub fn show(&self) {
        self.split_response();
    }
}
