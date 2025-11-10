use std::io::{self, Read};
use std::fs;

fn main() {

    let mut file_path = String::new();

    if let Err(e) = io::stdin().read_line(&mut file_path) {
        eprintln!("input error: {}", e);
        return;
    }

    let file_path = file_path.trim();

    match fs::File::open(file_path) {
        Ok(mut file) => {
            let mut test = Vec::new();
            match file.read_to_end(&mut test) {
                Ok(_) => println!("success"),
                Err(_) => println!("failure")
            }
        }
        Err(_) => println!("failure")
    }
}
