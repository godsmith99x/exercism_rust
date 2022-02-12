#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(n) => stack.push(*n),
            CalculatorInput::Add => 
        }
    }
}
