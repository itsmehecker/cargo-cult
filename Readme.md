>zfind

## Project Overview

The goal of this project is to provide a way for users to find their previously used commands fast and easily

PS I added clip and pbcopy alr in the code so if you wanna re use it for maybe .bash_history/equivalent for windows you can just changing the file name would be enough

## Usage for linux/macOS
it just requires you to have zsh or bash installed with the history file present 

## Usage for Windows
windows' cmd doesn't have native support for terminal history 
so you can use doskey which is a little too much for people to have it setup so I went with setting it up for powershell 
yeah thats right no other support for cmd.exe(im kidding you can still just change the .zsh_history to your persistant history file if you set it up with doskey)

as for powershell-
 ```
%appdata%\Microsoft\Windows\PowerShell\PSReadLine\ConsoleHost_history.txt
 ```
would be the path where the history is saved in

## Features

- detects the .zshfile
- displays multiple options
- automatically copies it to your clipboard for access
- fast as its made in rust
- uses dirs-next
- finds the bash_history file and creates a .history_config for with your prefered choice
- do keep in mind you can use any terminal with this as long as the commands don't have a timestamp (EXTENDED_HISTORY) on your .rc file like it has on zsh (turned off by default)
- if you would like to switch editors you could try editing the .history_config with nano/vim and change it
- bash for using bash and zsh for using zsh.
- removed duplicates

## To do's
- adding support for other terminals (prob not coz I don't wanna add extra lines of code for just nothing)
- ignoring timestamps so it works with devices with (EXTENDED_HISTORY/APPEND HISTORY TURNED ON)
## Build

To build the project, clone the repository and navigate to the project directory:

```bash
git clone https://github.com/itsmehecker/cargo-cult.git
cd cargo-cult
cargo build
```

To install from cargo 
```bash
cargo install zfind
```

## Usage
for ppl who install it with cargo
```bash
zfind <snippet>
```

or
for ppl who build it (haven't added to path)
```bash
cargo run <snippet>
```
## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

