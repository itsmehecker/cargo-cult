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

## Build

To install the project, clone the repository and navigate to the project directory:

```bash
git clone https://github.com/itsmehecker/cargo-cult.git
cd cargo-cult
cargo build
```

## Usage
```bash
zfind <snippet>
```

or

```bash
cargo run <snippet>
```
## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

