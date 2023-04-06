[![Build binary release](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml)

[![Tests](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml)

[![Clippy](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml)


# Remove Node from the front of a Singly LinkedList

**Developer:** Jaya Khan 

## Abstract
This program removes a node from the front of the singly linked list. It does this first by taking the current head of the list and then calls map method to update the head of the list to point to the next node.  

## Requirements
The code was tested on:
- rust edition = 2021


## Project Structure
                                                                               
    remove-front
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