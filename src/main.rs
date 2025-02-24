/*                                   
 \                                        /
 / RCCP - Rust Calculator for Cool People \
 \          Author: d3vboi                /
 /                                        \
*/ 
use colored::Colorize;
use std::io;
use std::io::Write;

mod tests;

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
    fn sqrt(&self, n: f64) -> f64 {
        n.sqrt()
    }
    fn sin(&self, angle: f64) -> f64 {
        angle.sin()
    }
    fn cos(&self, angle: f64) -> f64 {
        angle.cos()
    }
    fn tan(&self, angle: f64) -> f64 {
        angle.tan()
    }
    fn log(&self, n: f64) -> f64 {
        n.log10()
    }
}

fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

#[derive(PartialEq, Clone)]
enum Tokens {
    Number(f64),
    Operations(char),
    Empty,
}

// Math stuff, 
fn eval_input(input: &str) -> Vec<Tokens> {
    let input = input.replace(" ", "").replace(",", ".").replace("sin", "s").replace("cos", "c").replace("tan", "t").replace("log", "l").replace("sqrt", "q");
    if input == "exit\n" {
        std::process::exit(0);
    }
    let mut split_input: Vec<char> = input.chars().collect();
    let mut output: Vec<Tokens> = Vec::new();
    let mut current_number = String::new();

    split_input.remove(split_input.len() - 1); // Remove the newline character
    for (i, c) in split_input.iter().enumerate() {
        if c.is_ascii_digit() || *c == '.'{
            current_number.push(*c);
        } else if *c == '-' {
            if i == 0 || !split_input[i - 1].is_ascii_digit() {
                current_number.push(*c);
            } else {
                if !current_number.is_empty() {
                    output.push(Tokens::Number(current_number.parse().unwrap()));
                    current_number.clear();
                }
                output.push(Tokens::Operations(*c));
            }
        } else {
            if !current_number.is_empty() {
                output.push(Tokens::Number(current_number.parse().unwrap()));
                current_number.clear();
            }
            output.push(Tokens::Operations(*c));
        }
    }

    if !current_number.is_empty() {
        output.push(Tokens::Number(current_number.parse().unwrap())); // push the current number to the output if it isn't empty.
    }

    output
}

// Convert input to Reverse Polish Notation using shunting yard algorithm
fn shunting_yard(input: Vec<Tokens>) -> Vec<Tokens> {
    let mut output_queue: Vec<Tokens> = Vec::new();
    let mut operator_stack: Vec<Tokens> = Vec::new();
    let mut last_token = Tokens::Empty;
    for token in input {
        match token {
            Tokens::Number(_) => output_queue.push(token.clone()),
            Tokens::Operations(character) => {
                if character == '(' {
                    if let Tokens::Number(_) = last_token  {
                        operator_stack.push(Tokens::Operations('*'));
                    }
                    operator_stack.push(token.clone());
                } else if character == ')' {
                    while let Tokens::Operations('(') = operator_stack.last().unwrap() {
                        output_queue.push(operator_stack.pop().unwrap());
                    }
                    operator_stack.pop(); // Remove the left parenthesis
                } else {
                    while !operator_stack.is_empty() 
                        && parse_operator(&operator_stack[operator_stack.len() - 1])
                            >= parse_operator(&token)
                    {
                        output_queue.push(operator_stack.pop().unwrap());
                    }
                }
            },
            Tokens::Empty => panic!("Unknown operator"),
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
// Perform a unary operation on the stack
fn perform_unary_operation<F>(stack: &mut Vec<f64>, operation: F)
where
    F: FnOnce(f64) -> f64,
{
    let a = stack.pop().unwrap();
    let result = operation(a);
    stack.push(result);
}

// Evaluate the RPN expression
fn eval_rpn(input: Vec<Tokens>, ops: &Operators) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in input {
        match token {
            Tokens::Number(number) => stack.push(number),
            Tokens::Operations(operator) => {
                match operator {
                    '+' => perform_operation(&mut stack, |a, b| ops.add(a, b)),
                    '-' => perform_operation(&mut stack, |a, b| ops.subtract(a, b)),
                    '*' => perform_operation(&mut stack, |a, b| ops.multiply(a, b)),
                    '/' => perform_operation(&mut stack, |a, b| ops.divide(a, b)),
                    '^' => perform_operation(&mut stack, |a, b| ops.power(a, b)),
                    '!' => perform_unary_operation(&mut stack, |a| ops.factorial(a)),
                    'q' => perform_unary_operation(&mut stack, |a| ops.sqrt(a)), // sqrt
                    's' => perform_unary_operation(&mut stack, |a| ops.sin(a)), // sin
                    'c' => perform_unary_operation(&mut stack, |a| ops.cos(a)), // cos
                    't' => perform_unary_operation(&mut stack, |a| ops.tan(a)), // tan
                    'l' => perform_unary_operation(&mut stack, |a| ops.log(a)), // log
                    _ => if operator == '(' { panic!("Unknown operator: (, perhaps you forgot a closing bracket")} else {panic!("Unknown operator: {}", operator)},
                }
            },
            Tokens::Empty => panic!("Empty Token")
        }
    }
    stack[0]
}

// Parses operator precedence
fn parse_operator(token: &Tokens) -> i32 {
    if let Tokens::Operations(operator) = token {
        match operator {
            '+' => 1,
            '-' => 1,
            '*' => 2,
            '/' => 2,
            '^' => 3,
            '!' => 3,
            'q' => 3, // sqrt
            's' => 3, // sin
            'c' => 3, // cos
            't' => 3, // tan
            'l' => 3, // log
            _ => 0
        }
    } else {
        0
    }
    
}

fn main() {
    println!("{}{}{}{}{}{}{}{}{}
              {}{}", "Welcome to the ", "R".bold(), "ust ", "C".bold(), "alculator for ", "C".bold(), "ool ","P".bold(),"eople.", "Created by ", "d3vboi".purple().bold());
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
