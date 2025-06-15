use std::io::stdin;
fn main() {
    println!("Give first number");

    let first_number = get_equation_number();

    println!("Give operator");
    let operator = get_input();

    let operator = match operator {
        Ok(o) if o == "+" => "+",
        _ => return println!("Could not find operatir"),
    };

    println!("Give second number");
    let second_number = get_equation_number();

    let (first_number, second_number) = match (first_number, second_number) {
        (Ok(first), Ok(second)) => (first, second),
        (Err(e1), Err(e2)) => {
            println!("Error {}, {}", e1, e2);
            return;
        }
        (Ok(_), Err(e)) => {
            println!("Error {}", e);
            return;
        }
        (Err(e), Ok(_)) => {
            println!("Error {}", e);
            return;
        }
    };

    print!(
        "Equasion is {} {} {}",
        first_number, operator, second_number
    );

    let operator = operator.trim();

    if operator == "+" {
        let res = first_number + second_number;
        println!("Result is {}", res);
    }
}

fn get_input() -> Result<String, String> {
    let mut input = String::new();
    stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    Ok(input.trim().to_string())
}

fn get_equation_number() -> Result<i32, String> {
    let num = get_input()?;

    num.parse::<i32>()
        .map_err(|e| format!("'{}' Could not parse that number", e))
}
