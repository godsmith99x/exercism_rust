#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use CalculatorInput::{Add, Divide, Multiply, Subtract, Value};

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            Value(num) => stack.push(*num),
            _ if stack.len() < 2 => return None,
            _ => operation(&mut stack, &input),
        }
    }

    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}

fn operation(stack: &mut Vec<i32>, input: &CalculatorInput) {
    let num2 = stack.pop().unwrap();
    let num1 = stack.pop().unwrap();

    stack.push(match input {
        Add => num1 + num2,
        Subtract => num1 - num2,
        Multiply => num1 * num2,
        Divide => num1 / num2,
        _ => unreachable!(),
    });
}
