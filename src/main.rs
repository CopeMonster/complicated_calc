use text_colorizer::Colorize;

use crate::{token::Tokens, calculator::calculate};

mod token;
mod utils;
mod calculator;

fn main() {
    loop {
        let mut expression = String::new();

        match std::io::stdin().read_line(&mut expression) {
            Ok(_) => {},
            Err(e) => {
                println!("{} failed to read line: {}", "Error:".red().bold(), e);
                continue;
            }
        }

        let expression = expression.trim();

        let tokens = match Tokens::parse(&expression) {
            Ok(t) => t,
            Err(e) => {
                println!("{} failed to parse expression: {}", "Error:".red().bold(), e);
                continue;
            },
        };
    
        match calculate(tokens) {
            Ok(result) => println!("Result of [{}]: {}", expression.bright_blue(), result.to_string().green().bold()),
            Err(err) => println!("{} failed to calculate expression: {}", "Error:".red().bold(), err),
        }
    }
}