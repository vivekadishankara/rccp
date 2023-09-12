/*                                   
 \                                        /
 / RCCP - Rust Calculator for Cool People \
 \          Author: d3vboi                /
 /                                        \
 */ 

use std::io;
use std::io::Write;


struct Operators {}
impl Operators {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }
    fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
    fn divide(&self, a: i32, b: i32) -> i32 {
        a / b
    }
}

fn is_integer(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

// Parses operator precedence
fn parse_operator(operator: &str) -> i32 {
    match operator {
        "+" => 1,
        "-" => 1,
        "*" => 2,
        "/" => 2,
        _ => 0
    }
}

fn eval_input(input: &str) -> Vec<String> {
    let mut split_input: Vec<char> = input.chars().collect();
    let mut output: Vec<String> = Vec::new();
    let mut current_number = String::new();
    split_input.remove(split_input.len() - 1);
    for c in split_input {
        if is_integer(&c.to_string()) {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                output.push(current_number.clone());
                current_number.clear();
            }
            output.push(c.to_string());
        }
    }

    if !current_number.is_empty() {
        output.push(current_number);
    }
    // Inline for loop to remove spaces
    output = output.into_iter().filter(|x| x != " ").collect();

    output
}

// Converts input into Reverse Polish Notation using the Shunting Yard Algorithm
fn shunting_yard(input: Vec<String>) -> Vec<String> {
    let mut output_queue: Vec<String> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();

    for token in input {
        if is_integer(&token) {
            output_queue.push(token);
        } else if token == "(" {
            operator_stack.push(token);
        } else if token == ")" {
            while *operator_stack.last().unwrap() != "(" {
                output_queue.push(operator_stack.pop().unwrap());
            }
            operator_stack.pop(); // Pop the left parenthesis
        } else {
            while !operator_stack.is_empty() && parse_operator(&operator_stack[operator_stack.len() - 1]) >= parse_operator(&token) {
                output_queue.push(operator_stack.pop().unwrap());
            }
            operator_stack.push(token);
        }
    }

    while !operator_stack.is_empty() {
        output_queue.push(operator_stack.pop().unwrap());
    }

    output_queue
}

// Evaluates Reverse Polish Notation
fn eval_rpn(input: Vec<String>, ops: &Operators) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in input {
        if is_integer(&token) {
            stack.push(token.parse().unwrap());
        } else {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            let result = match token.as_str() {
                "+" => ops.add(a, b),
                "-" => ops.subtract(a, b),
                "*" => ops.multiply(a, b),
                "/" => ops.divide(a, b),
                _ => panic!("Unknown operator: {}", token),
            };
            stack.push(result);
        }
    }

    stack[0]
}

fn main() {
    let ops = Operators {};
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let output = eval_input(&input);
        let rpn = shunting_yard(output);
        let result = eval_rpn(rpn, &ops);
        println!("{}", result);
    }
}