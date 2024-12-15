>zfind

## Project Overview

The goal of this project is to provide a way for users to find their previously used commands fast and easily

PS I added clip and pbcopy alr in the code so if you wanna re use it for maybe .bash_history/equivalent for windows you can just changing the file name would be enough
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

