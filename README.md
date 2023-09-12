# RCCP - Rust Calculator for Cool People

RCCP is a simple command-line calculator written in Rust as an alternative to more conventional GUI calculators. It supports basic arithmetic calculation, and respects the order of operation (BIDMAS/BODMAS/PEDMAS).

## Features

- Basic arithmetic operations: addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`).
- Order of operations: operations are performed in the correct order according to BODMAS/BIDMAS/PEDMAS rules.
- Parentheses: parentheses can be used to change the order of operations.
- Simple Command Line Interface

## Installation

To install RCCP, you can use the rust package manager: cargo.
```
$ curl https://sh.rustup.rs -sSf | sh
$ cargo install rccp
$ rccp
```

Or you can build it from source, using the github repository.
```
$ git clone https://github.com/d3vboi/rccp.git
$ cd rccp
$ cargo run
```
Keep in mind, if you use this method you will need to be in the RCCP directory to be able to run the program
## Usage

To use RCCP, simply run the program, then enter an expression at the prompt:

```
$ rccp
> 2 + 3 * 4
14
```
As we can see in this example, RCCP correctly performs the multiplication before the addition, resulting in the `14` instead of `20`.
You do not need to add spaces in between characters.
`> 2+3*4` is also a valid expression.

## License

This project is licensed under the MIT License.

## Todo
 - [ ] Add support for decimals in input and outputs
 - [ ] Add support for trigonometric functions
 - [ ] Add a way to gracefully exit