use std::collections::BTreeMap;
use std::io::{self, Write};

fn get_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("failed to flush stdout");

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("failed to read line");

    buf.trim().to_string()
}

enum Query {
    Get { key: String },
    Set { key: String, value: String },
}

fn parse_input(input: String) -> Option<Query> {
    let split: Vec<&str> = input.split_ascii_whitespace().filter(|e| !e.is_empty()).collect();

    if split.is_empty() {
        None
    } else if split[0].to_ascii_uppercase() == "GET" {
        if split.len() != 2 {
            println!("incorrect number of values in GET query");
            return None;
        }
        
        Some(Query::Get { key: split[1].to_string() })
    } else if split[0].to_ascii_uppercase() == "SET" {
        if split.len() != 3 {
            eprintln!("incorrect number of values in SET query");
            return None;
        }
        
        Some(Query::Set {
            key: split[1].to_string(),
            value: split[2].to_string()
        })
    } else {
        eprintln!("invalid query command: {}", split[0]);
        None
    }
}

fn main() {
    let mut db = BTreeMap::new();
    
    loop {
        let input = get_input("> ");

        if input == "exit" {
            return;
        }

        let parse_result = parse_input(input);

        if parse_result.is_none() {
            continue;
        }

        let query = parse_result.unwrap();

        match query {
            Query::Get { key } => {
                if db.contains_key(&key) {
                    println!("{}: {}", key, db.get(&key).unwrap());
                } else {
                    eprintln!("no value found at key {key}");
                }
            }
            Query::Set { key, value } => {
                db.insert(key.clone(), value.clone());
                println!("value {value} inserted at {key}");
            }
        }
    }
}
