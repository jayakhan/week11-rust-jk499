[![Build binary release](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml)

[![Tests](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml)

[![Clippy](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml)


# Add Node to Front in Singly LinkedList

**Developer:** Jaya Khan 

## Abstract
This program adds a node to the front of the singly linked list LinkedList. It does this first by creating a new node with the data passed to the function. It then takes the current head of the list and updates next pointer of the new node to point it to the previous head. Lastly, it makes the head of the list point to the new node. 



## Requirements
The code was tested on:
- rust edition = 2021


## Project Structure
                                                                               
    add-front
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