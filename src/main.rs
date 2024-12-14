use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::Command as ProcessCommand;

fn func(u: &str, lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter(|line| line.contains(u))
        .cloned()
        .collect()
}

fn copy_to_clipboard(command: &str) -> io::Result<()> {
    let mut child = if cfg!(target_os = "windows") {
        ProcessCommand::new("clip")
    } else if cfg!(target_os = "macos") {
        ProcessCommand::new("pbcopy")
    } else {
        let mut cmd = ProcessCommand::new("xclip");
        cmd.arg("-selection").arg("clipboard");
        cmd
    }
    .stdin(std::process::Stdio::piped())
    .spawn()?;

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(command.as_bytes())?;
    child.wait()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("zfind")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Finds and copies commands from .zsh_history")
        .arg(
            Arg::new("snippet")
                .help("The snippet to search for in the command history")
                .required(true)
                .index(1),
        )
        .get_matches();

    let snippet = matches.get_one::<String>("snippet").unwrap();

    let file = File::open(".zsh_history")?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let results = func(snippet, &lines);
    if results.is_empty() {
        println!("No matching command found.");
    } else if results.len() == 1 {
        let command = &results[0];
        println!("{}", command);
        copy_to_clipboard(command)?;
        println!("Command copied to clipboard. Paste it into the terminal to execute.");
    } else {
        println!("Multiple matches found:");
        for (i, result) in results.iter().enumerate() {
            println!("{}: {}", i + 1, result.trim());
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice: usize = input.trim().parse().unwrap_or(0);
        if choice > 0 && choice <= results.len() {
            let command = &results[choice - 1];
            println!("{}", command);
            copy_to_clipboard(command)?;
            println!("Command copied to clipboard. Paste it into the terminal to execute.");
        } else {
            println!("Invalid choice.");
        }
    }

    Ok(())
}
