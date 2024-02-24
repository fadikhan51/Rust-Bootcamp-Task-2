use std::io;
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn calculate(&self) -> Result<f64, String> {
        match &self {
            Operation::Add(a, b) => Ok(a + b),
            Operation::Subtract(a, b) => Ok(a-b),
            Operation::Multiply(a, b) => Ok(a*b),
            Operation::Divide(a, b) => {
                if *b == 0.0 {
                    Err("Division by zero is not possible".to_string())
                } else {
                    Ok(a/b)
                }
            }
        }
    }
}

fn main() {
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_number: f64 = input.trim().parse().expect("Please enter a valid number");

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation: char = input.trim().chars().next().expect("Please enter a valid operation");

    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_number: f64 = input.trim().parse().expect("Please enter a valid number");

    let operation = match operation {
        '+' => Some(Operation::Add(first_number, second_number)),
        '-' => Some(Operation::Subtract(first_number, second_number)),
        '*' => Some(Operation::Multiply(first_number, second_number)),
        '/' => Some(Operation::Divide(first_number, second_number)),
        _ => None,
    };

    match operation {
        None => {
            println!("Sorry you entered something wrong. Please run the program again");
        }
        Some(value) => {
            match value.calculate() {
                Ok(val) => {
                    println!("The answer is {}",val)
                }
                Err(err) => {
                    println!("Error : {}", err);
                }
            }
        }
    }

}
