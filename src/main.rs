use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::Command as ProcessCommand;
use dirs_next::home_dir;
use std::io::stdin;

fn func(snippet: &str, lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter(|line| line.contains(snippet))
        .cloned()
        .collect()
}

fn copy_to_clipboard(command: &str) -> io::Result<()> {
    let mut child = if cfg!(target_os = "windows") {
        ProcessCommand::new("clip").stdin(std::process::Stdio::piped()).spawn()?
    } else if cfg!(target_os = "macos") {
        ProcessCommand::new("pbcopy").stdin(std::process::Stdio::piped()).spawn()?
    } else {
        ProcessCommand::new("xclip").arg("-selection").arg("clipboard").stdin(std::process::Stdio::piped()).spawn()?
    };

    child.stdin.as_mut().unwrap().write_all(command.as_bytes())?;
    child.wait()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("zsh_history_search")
        .author("itsmehecker")
        .about("Finds and copies commands from .zsh_history")
        .arg(
            Arg::new("snippet")
                .help("The snippet to search for in the command history")
                .required(true)
                .num_args(1..)
                .index(1),
        )
        .get_matches();

    let snippet: String = matches.get_many::<String>("snippet")
        .unwrap()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    let home = home_dir().expect("Could not find home directory");
    let history_path = home.join(".zsh_history");
    let file = File::open(history_path)?;

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let results = func(&snippet, &lines);
    if results.is_empty() {
        println!("No matching command found.");
    } else {
        let mut start = 0;
        let page_size = 15;

        loop {
            let end = (start + page_size).min(results.len());
            for (index, result) in results[start..end].iter().enumerate() {
                println!("{}: {}", start + index + 1, result);
            }

            if end == results.len() {
                println!("Enter the number of the command you want to copy:");
            } else {
                println!("Enter 'n' for next page, or the number of the command you want to copy:");
            }

            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == "n" && end < results.len() {
                start += page_size;
            } else if let Ok(index) = input.parse::<usize>() {
                if index > 0 && index <= results.len() {
                    let command = &results[index - 1];
                    println!("{}", command);
                    copy_to_clipboard(command)?;
                    println!("Command copied to clipboard. Paste it into the terminal to execute.");
                    break;
                } else {
                    println!("Invalid selection.");
                }
            } else {
                println!("Invalid input.");
            }

            if end == results.len() {
                break;
            }
        }
    }

    Ok(())
}