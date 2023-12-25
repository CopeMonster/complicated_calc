use crate::token::{TokenType, Tokens};

pub fn calculate(tokens: Tokens) -> Result<f64, &'static str> {
    let mut stack: Vec<f64> = vec![];

    for token in &tokens.tokens {
        match token.token_type {
            TokenType::Operand(num) => {
                stack.push(num);
            },
            TokenType::Operator(op) => {
                if stack.len() < 2 {
                    return Err("not enough operands for operator");
                }

                let num1 = match stack.pop() {
                    Some(n) => n,
                    None => return Err("failed to get number 1"),
                };

                let num2 = match stack.pop() {
                    Some(n) => n,
                    None => return Err("failed to get number 2"),
                }; 

                let result = execute_operation(num2, num1, op);
            
                stack.push(result);
            },
        }
    }

    if stack.len() != 1 {
        Err("invalid expression")
    } else {
        stack.pop().ok_or("failed to pop from stack")
    }
}

pub fn execute_operation(num1: f64, num2: f64, operator: char) -> f64 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("unknown operator"),
    }
}