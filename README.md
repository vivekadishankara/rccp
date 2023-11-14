# RCCP - Rust Calculator for Cool People
## Overview
RCCP is a simple command-line calculator written in Rust as an alternative to more conventional GUI calculators. It supports basic arithmetic calculation, and respects the order of operation (BIDMAS/BODMAS/PEMDAS).

## Features

- Basic arithmetic operations: addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`).
- Exponents: use the `^` symbol to perform exponentiation (e.g., `2^3` equals 8).
- Factorial: use the `!` symbol to calculate the factorial of a number (e.g., `5!` equals 120).
- Sqare root: use `sqrt` to calculete the sqare root of a number (e.g., `sqrt(9)`)
- Trigonometry: use `sin`, `cos`, and `tan` in your equations (e.g., `sin(16)`)
- Logorithm: use `log` to calculate the logorithmic of a number (e.g., `log(100)`)
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
$ curl https://sh.rustup.rs -sSf | sh
$ cargo run -q
```
Keep in mind, if you use this method you will need to be in the RCCP directory to be able to run the cargo command.

## Usage

To use RCCP, simply run the program, then enter an expression at the prompt:

```
$ rccp
> 2 + 3 * 4
14
> exit
```
As we can see in this example, RCCP correctly performs the multiplication before the addition, resulting in the *14* instead of *20*.
You do not need to add spaces in between characters.
`> 2+3*4` is also a valid expression.

## License

This project is licensed under the MIT License.

## Todo
 - [ ] Support for arrow keys (e.g., expression history)
 - [ ] Error handling
 - [ ] Variable support + constant support (e.g., pi and e)
 - [ ] Minor bug fixes