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
    fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }

    fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }

    fn multiply(&self, a: f64, b: f64) -> f64 {
        a * b
    }

    fn divide(&self, a: f64, b: f64) -> f64 {
        a / b
    }

    fn power(&self, a: f64, b: f64) -> f64 {
        a.powf(b)
    }

    fn factorial(&self, n: f64) -> f64 {
        if n <= 1.0 {
            1.0
        } else {
            n * self.factorial(n - 1.0)
        }
    }
    // TODO: Implement trig functions
    fn _sin(&self, angle: f64) -> f64 {
        angle.sin()
    }

    fn _cos(&self, angle: f64) -> f64 {
        angle.cos()
    }

    fn _tan(&self, angle: f64) -> f64 {
        angle.tan()
    }
}

fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

fn eval_input(input: &str) -> Vec<String> {
    if input == "exit\n" {
        std::process::exit(0);
    }
    let mut split_input: Vec<char> = input.chars().collect();
    let mut output: Vec<String> = Vec::new();
    let mut current_number = String::new();

    split_input.remove(split_input.len() - 1); // Remove the newline
    for c in split_input {
        if is_number(&c.to_string()) || c == '.' {
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

// Convert input to Reverse Polish Notation using shunting yard algorithm
fn shunting_yard(input: Vec<String>) -> Vec<String> {
    let mut output_queue: Vec<String> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();
    let mut last_token = String::new();
    for token in input {
        if is_number(&token) {
            output_queue.push(token.clone());
        } else if token == "(" {
            if is_number(&last_token) {
                operator_stack.push("*".to_string());
            }
            operator_stack.push(token.clone());
        } else if token == ")" {
            while *operator_stack.last().unwrap() != "(" {
                output_queue.push(operator_stack.pop().unwrap());
            }
            operator_stack.pop(); // Pop the left parenthesis
        } else {
            while !operator_stack.is_empty()
                && parse_operator(&operator_stack[operator_stack.len() - 1])
                    >= parse_operator(&token)
            {
                output_queue.push(operator_stack.pop().unwrap());
            }
            operator_stack.push(token.clone());
        }
        last_token = token.clone(); // Clone the token value
    }

    while !operator_stack.is_empty() {
        output_queue.push(operator_stack.pop().unwrap());
    }

    output_queue
}

// Perform an operation on the stack
fn perform_operation<F>(stack: &mut Vec<f64>, operation: F)
where
    F: FnOnce(f64, f64) -> f64,
{
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let result = operation(a, b);
    stack.push(result);
}
// Perform a factorial on the stack
fn perform_factorial<F>(stack: &mut Vec<f64>, operation: F)
where
    F: FnOnce(f64) -> f64,
{
    let a = stack.pop().unwrap();
    let result = operation(a);
    stack.push(result);
}

// Evaluate the RPN expression
fn eval_rpn(input: Vec<String>, ops: &Operators) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in input {
        if is_number(&token) {
            stack.push(token.parse().unwrap());
            continue;
        }

        match token.as_str() {
            "+" => perform_operation(&mut stack, |a, b| ops.add(a, b)),
            "-" => perform_operation(&mut stack, |a, b| ops.subtract(a, b)),
            "*" => perform_operation(&mut stack, |a, b| ops.multiply(a, b)),
            "/" => perform_operation(&mut stack, |a, b| ops.divide(a, b)),
            "^" => perform_operation(&mut stack, |a, b| ops.power(a, b)),
            "!" => perform_factorial(&mut stack, |a| ops.factorial(a)),
            _ => panic!("Unknown operator: {}", token),
        };
    }

    stack[0]
}

// Parses operator precedence
fn parse_operator(operator: &str) -> i32 {
    match operator {
        "+" => 1,
        "-" => 1,
        "*" => 2,
        "/" => 2,
        "^" => 3,
        "!" => 3,
        _ => 0
    }
}

fn main() {
    let ops = Operators {};
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let output = eval_input(&input);
        let rpn = shunting_yard(output);
        let result = eval_rpn(rpn, &ops);
        println!("{}", result);
    }
}