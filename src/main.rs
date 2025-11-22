use std::io::{stdin, stdout, Write, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn main() {
    let lang = "EN_ENGLISH";
    println!("Hello and welcome, user");
    println!("Type a letter (A-Z) to list words, or quit (exit)\n");

    let path = Path::new("tests/dictionary/dic.txt");
    let mut words = Vec::new();

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(word) = line{
                words.push(word);
            } 
        }
    }
    else {
        println!("Warning: Dictionary file not found at {:?}", path);
    }

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim_end_matches(&['\r', '\n'][..]).to_lowercase();

        if input == "exit" 
            {break;}

        else if input == "lang" 
            {println!("{}", lang);}

        else if input.len() == 1 && input.chars().all(|c| c.is_ascii_alphabetic())
        {
            let letter = input.to_lowercase().chars().next().unwrap();
            let mut found = false;

            for word in &words {
                if word.to_lowercase().starts_with(letter) {
                    println!("- {}", word);
                    found = true;
                }

            }

            if !found {
                println!("No words found starting with '{}'", letter);
            }
        }
        else
        {
            println!("Please type a letter (A-Z) or quit (exit)...\n");
        }
    }
}
