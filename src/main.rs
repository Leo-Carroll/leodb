use std::collections::BTreeMap;
use std::io::{self, Write};

fn get_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("failed to flush stdout");

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("failed to read line");

    buf.trim().to_string()
}

fn main() {
    loop {
        let input = get_input("> ");

        if input == "exit" {
            return;
        }
        
        println!("{input}");
    }
}
