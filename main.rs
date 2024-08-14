enum Expression {
    Number(i32),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
}
fn evaluate_expression(expr: &Expression) -> i32 {
    match expr {
        Expression::Number(n) => *n,
        Expression::Addition(left, right) => evaluate_expression(left) + evaluate_expression(right),
        Expression::Subtraction(left, right) => evaluate_expression(left) - evaluate_expression(right),
        Expression::Multiplication(left, right) => evaluate_expression(left) * evaluate_expression(right),
         Expression::Division(left, right) => evaluate_expression(left) / evaluate_expression(right),
    }
}

fn interpret_expression(input: &str) -> Option<i32> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let expression = parse_expression(&tokens)?;
    Some(evaluate_expression(&expression))
}


fn parse_expression(tokens: &[&str]) -> Option<Expression> {
    if tokens.is_empty() {
        return None;
    }
    
    let first_token = tokens[0];
    
    if let Ok(number) = first_token.parse::<i32>() {
        return Some(Expression::Number(number));
    }
    
    match first_token {
        "+" => {
            let left = parse_expression(&tokens[1..])?;
            let right = parse_expression(&tokens[2..])?;
            Some(Expression::Addition(Box::new(left), Box::new(right)))
        },
        "-" => {
            let left = parse_expression(&tokens[1..])?;
            let right = parse_expression(&tokens[2..])?;
            Some(Expression::Subtraction(Box::new(left), Box::new(right)))
        },
        "*" => {
            let left = parse_expression(&tokens[1..])?;
            let right = parse_expression(&tokens[2..])?;
            Some(Expression::Multiplication(Box::new(left), Box::new(right)))
        },
        "/" => {
            let left = parse_expression(&tokens[1..])?;
            let right = parse_expression(&tokens[2..])?;
            Some(Expression::Division(Box::new(left), Box::new(right)))
        },
        _ => None,
    }
}


fn main() {
    let expression1 = "5 + 3 * 2";
    let result1 = interpret_expression(expression1);
    println!("{} = {:?}", expression1, result1);
    
    let expression2 = "10 - 4 / 2";
    let result2 = interpret_expression(expression2);
    println!("{} = {:?}", expression2, result2);
}

