[![Build binary release](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml)

[![Tests](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml)

[![Clippy](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml)


# All possible combinations of letters in word - TEST.

**Developer:** Jaya Khan 

## Abstract
The program takes a 4-letter word "TEST" and initializes an empty set to store the combinations. It then uses four nested loops to generate all possible combinations of the letters in the word.

For each combination, the program creates a vector containing the letters in the correct order and inserts it into the set. Finally, the program iterates over the set and prints each combination.

The output of the program will be a list of all possible combinations of the letters in the word "TEST", such as "TEST", "TETS", "TSET", "TSTE", "TTSE", "TTES", etc. Note that there are 24 possible combinations in total (4 factorial).

## Requirements
The code was tested on:
- rust edition = 2021


## Project Structure
                                                                               
    reverse-text
    ├── Cargo.toml
    ├── README.md   
    ├── src
        ├── main.rs
    ├── .github
        ├── workflows
            ├── lint.yml
            ├── release.yml
            ├── rustfmt.yml
            ├── tests.yml
    ├── Makefile


## Commands to install Rust
1. sudo apt install curl
2. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
3. source $HOME/.cargo/env

    ### Check installation:
    rustc --version

    ### Command to uninstall rust
    rustup self uninstall

## Command to build project (CI/CD): 
`make all`