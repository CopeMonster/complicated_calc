pub fn is_operator(token: &str) -> bool {
    let operators = vec!["+", "-", "*", "/"];

    operators.contains(&token)
}

pub fn is_higher_or_equal_precedence(op1: char, op2: char) -> bool {
    let precedence1 = get_precedence(op1);
    let precedence2 = get_precedence(op2);

    precedence1 >= precedence2
}

pub fn get_precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}