use std::{fmt::Display, io::stdin, str::FromStr};

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            other => Err(format!("Unsupported operator '{other}'")),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sym = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        };
        write!(f, "{sym}")
    }
}

fn read_line(prompt: &str) -> Result<String, String> {
    println!("{prompt}");

    let mut input = String::new();
    stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    Ok(input.trim().to_string())
}

fn get_number(prompt: &str) -> Result<i32, String> {
    let line = read_line(prompt).map_err(|e: String| e.to_string())?;

    line.parse::<i32>()
        .map_err(|e| format!("Not a valid number {}", e))
}

fn read_operator(prompt: &str) -> Result<Operator, String> {
    let line = read_line(prompt).map_err(|e: String| e.to_string())?;
    line.parse()
}

fn calculate(ln: i32, op: Operator, rn: i32) -> Result<i32, String> {
    match op {
        Operator::Add => Ok(ln + rn),
        Operator::Sub => Ok(ln - rn),
        Operator::Mul => Ok(ln * rn),
        Operator::Div => Ok(ln / rn),
    }
}

fn main() -> Result<(), String> {
    let ln = get_number("First number")?;
    let op = read_operator("Operator")?;
    let rn = get_number("Second number")?;

    let result = calculate(ln, op, rn)?;
    println!("Equasion result is {}", result);
    Ok(())
}
