use chrono::Utc;
use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};

pub fn load() -> io::Result<std::io::Lines<BufReader<std::fs::File>>> {
    let mut text: String = Utc::now().format("%Y-%m-%d").to_string();
    text.push_str(".md");

    println!("Attemping to create file: {}", &text);
    
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&text)?;

    let reader = BufReader::new(file);

    Ok(reader.lines())
}
