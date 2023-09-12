# RCCP - Rust Calculator for Cool People

RCCP is a simple command-line calculator written in Rust as an alternative to more conventional GUI calculators. It supports basic arithmetic calculation, and respects the order of operation (BIDMAS/BODMAS/PEDMAS).

## Features

- Basic arithmetic operations: addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`).
- Order of operations: operations are performed in the correct order according to BODMAS/BIDMAS/PEDMAS rules.
- Parentheses: parentheses can be used to change the order of operations.
- Simple Command Line Interface

## Usage

To use RCCP, simply run the program, then enter an expression at the prompt:

```
> 2 + 3 * 4
14
```
As we can see in this example, RCCP correctly performs the multiplication before the addition, resulting in the `14` instead of `20`.

## License

This project is licensed under the MIT License.

## Todo
 - [ ] Add support for decimals in input and outputs
 - [ ] Add support for trigonometric functions 
 - [ ] Add a way to gracefully exit