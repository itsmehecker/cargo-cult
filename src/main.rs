use clap::{Arg, Command};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Read, Write};
use std::process::Command as ProcessCommand;
use dirs_next::home_dir;
use std::io::stdin;
use std::path::PathBuf;


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

fn get_history_path() -> io::Result<PathBuf> {
    let home = home_dir().expect("Could not find home directory");
    let config_path = if cfg!(target_os = "windows") {
        PathBuf::from(r"%appdata%\Microsoft\Windows\PowerShell\PSReadLine\ConsoleHost_history.txt")
    } else {
        home.join(".history_config")
    };

    if config_path.exists() {
        let mut file = File::open(&config_path)?;
        let mut choice = String::new();
        file.read_to_string(&mut choice)?;
        let choice = choice.trim();
        if choice == "bash" {
            return Ok(home.join(".bash_history"));
        } else {
            return Ok(home.join(".zsh_history"));
        }
    } else {
        let historybash = home.join(".bash_history");
        if historybash.exists() {
            println!("Found .bash_history file. Do you want to use bash history instead of zsh history? (y/n)");
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let input = input.trim();
            let mut file = OpenOptions::new().write(true).create(true).open(&config_path)?;
            if input.eq_ignore_ascii_case("y") {
                writeln!(file, "bash")?;
                return Ok(home.join(".bash_history"));
            } else if input.eq_ignore_ascii_case("n") {
                writeln!(file, "zsh")?;
                return Ok(home.join(".zsh_history"));
            } else {
                println!("Invalid input. Using zsh history.");
                writeln!(file, "zsh")?;
                return Ok(home.join(".zsh_history"));
            }
        } else {
            return Ok(home.join(".zsh_history"));
        }
    }
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

    let history_path = get_history_path()?;
    let file = File::open(history_path)?;

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let mut unique_lines = HashSet::new();
    let unique_lines: Vec<String> = lines.into_iter()
        .filter(|line| unique_lines.insert(line.clone()))
        .collect();

    let results: Vec<String> = unique_lines.iter()
        .filter(|line| line.contains(&snippet))
        .cloned()
        .collect();

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