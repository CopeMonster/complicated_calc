use crate::utils::{is_operator, is_higher_or_equal_precedence};

pub enum TokenType {
    Operand(f64),
    Operator(char),
}

pub struct Token {
    pub token: String,
    pub token_type: TokenType,
}

pub struct Tokens {
    pub tokens: Vec<Token>,
}

impl TokenType {
    pub fn get_token_type(token: &str) ->Result<TokenType, &'static str> {
        if is_operator(token) {
            match token.chars().nth(0) {
                Some(op) => Ok(TokenType::Operator(op)),
                None => Err("failed to get operator"),
            }
        } else if let Ok(num) = token.parse::<f64>() {
            Ok(TokenType::Operand(num))
        }
        else {
            Err("failed to get operator type")
        }
    }
}

impl Tokens {
    pub fn parse(expression: &str) -> Result<Tokens, &'static str> {
        let mut output_queue: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();
    
        for s in expression.split_whitespace() {
            let token = Token { 
                token: s.to_string(),
                token_type: match TokenType::get_token_type(s) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                },
            };
    
            match token.token_type {
                TokenType::Operand(_) => {
                    output_queue.push(token);
                },
                TokenType::Operator(op) => {
                    while let Some(top_operator) = operator_stack.last() {
                        if let TokenType::Operator(top_op) = top_operator.token_type {
                            if is_higher_or_equal_precedence(top_op, op) {
                                match operator_stack.pop() {
                                    Some(operator) => output_queue.push(operator),
                                    None => return Err("failed to pop from stack"),
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    operator_stack.push(token);
                },
            }
        }
    
        while let Some(op) = operator_stack.pop() {
            output_queue.push(op);
        }
    
        Ok(Self { tokens: output_queue })
    }
}