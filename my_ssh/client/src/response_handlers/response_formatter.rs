use colored::*;
use std::io::Write;
pub struct ShowResponse {
    response: String,
}

fn get_color(color: &str) -> colored::Color {
    match color {
        "^!" => Color::Blue,
        "^#" => Color::Green,
        _ => Color::White,
    }
}

impl ShowResponse {
    pub fn new(response: String) -> Self {
        ShowResponse { response }
    }
    fn show_list_style(&self, word: Vec<&str>) {
        fn print_space(len: usize) {
            for _ in 0..len {
                print!(" ");
            }
        }
        let mut count = 0;
        let mut max_len = 0usize;
        for e in &word {
            max_len = std::cmp::max(max_len, e.chars().count());
        }
        for e in word {
            if e.starts_with('^') {
                let color_code = &e[0..2];
                print!("{}", e[2..].color(get_color(color_code)));
                print_space(max_len + 1 - e[2..].len());
            } else {
                print!("{}", e);
                print_space(max_len + 1 - e.len());
            }
            count += 1;
            if count % 3 == 0 {
                count = 0;
                println!()
            }
            std::io::stdout().flush().unwrap();
        }
        println!()
    }
    fn show_grep_style(&self, word: Vec<&str>) {
        for w in word {
            if !w.is_empty() {
                let mut red_status = false;
                let chars: Vec<_> = w.chars().collect();
                let mut i = 0;
                while i < chars.len() {
                    let c = chars[i];
                    if i + 1 < chars.len() && c == '^' && chars[i + 1] == '@' {
                        red_status = true;
                        i += 2;
                        continue;
                    }
                    if i + 1 < chars.len() && c == '~' && chars[i + 1] == '~' {
                        red_status = false;
                        i += 2;
                        continue;
                    }
                    if red_status {
                        print!("{}", c.to_string().bright_red());
                    } else {
                        print!("{}", c);
                    }
                    i += 1;
                }
            }
        }
        println!();
    }
    fn split_response(&self) {
        let props: Vec<&str> = self
            .response
            .split("?&")
            .filter(|f| !f.is_empty())
            .collect();
        for w in props {
            let word: Vec<&str> = w[1..].split("\n\n").filter(|f| !f.is_empty()).collect();
            //dbg!(&word);
            match w.chars().next() {
                Some('E') => {
                    println!("{}", word[0]);
                }
                Some('L') => {
                    self.show_list_style(word);
                }
                Some('C') => {
                    self.show_grep_style(word);
                }
                Some('N') => {
                    for w in word {
                        if !w.is_empty() {
                            print!("{} ", w);
                        }
                    }
                    println!();
                }
                Some(_) => (),
                None => {
                    println!("String gol");
                }
            }
        }
    }
    pub fn show(&self) {
        self.split_response();
    }
}
