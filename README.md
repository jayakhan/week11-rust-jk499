[![Build binary release](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml)

[![Tests](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml)

[![Clippy](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml)


# Simple Calculator (+, -, *, /)

**Developer:** Jaya Khan 

## Abstract
This program defines a module called calculation which contains four functions for performing basic arithmetic operations: add, subtract, multiply, and divide.

The main function prompts the user to input two numbers and an operation. It then converts the inputs to f64 floating-point numbers using Rust's parse() method.

The program uses a match expression to determine which operation the user wants to perform. If the user inputs +, it calls calculation::add with the two numbers as arguments. If the user inputs -, it calls calculation::subtract, and so on.

The program prints the result of the calculation to the console.


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