//just a working copy with pbcopy instead of clip
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;

fn func(u: &str, lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter(|line| line.contains(u))
        .cloned()
        .collect()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <snippet>");
        std::process::exit(1);
    }
    let snippet = &args[1];

    let file = File::open(".zsh_history")?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let results = func(snippet, &lines);
    if !results.is_empty(){
        if results.len() == 1 {
            let command = &results[0];
            println!("{}", command);
            Command::new("pbcopy")
                .arg(command)
                .output()
                .expect("Failed to copy to clipboard");
            println!("Command copied to clipboard. Paste it into the terminal to execute.");
        } else {
            println!("Multiple matches found:");
            for (i, result) in results.iter().enumerate() {
                println!("{}: {}", i + 1, result.trim());
            }
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
            let choice: usize = choice.trim().parse::<usize>().expect("Invalid input") - 1;
            if (choice < results.len()) {
                let command = &results[choice];
                println!("{}", command);
                Command::new("pbcopy")
                    .arg(command)
                    .output()
                    .expect("Failed to copy to clipboard");
                println!("Command copied to clipboard. Paste it into the terminal to execute.");
            } else {
                println!("Invalid choice.");
            }
        }
    } else {
        println!("No matching command found.");
    }
    Ok(())
}