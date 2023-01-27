#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut eval_stack: Vec<i32> = Vec::new();
    for op in inputs.iter() {
        match op {
            CalculatorInput::Value(val) => eval_stack.push(*val),
            op => {
                let (l, r) = (eval_stack.pop()?, eval_stack.pop()?);
                match op {
                    CalculatorInput::Add => eval_stack.push(l + r),
                    CalculatorInput::Subtract => eval_stack.push(r - l),
                    CalculatorInput::Multiply => eval_stack.push(l * r),
                    CalculatorInput::Divide => eval_stack.push(r / l),
                    _ => unreachable!(),
                }
            }
        }
    }

    (eval_stack.len() == 1).then(|| eval_stack.pop()).flatten()
}
