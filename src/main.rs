mod calculator;
mod operations;

use calculator::Calculator;
use operations::advanced::*;
use operations::arithmetic::*;
use std::io::{self, Write};

fn main() {
    let calc = Calculator::new();

    println!("Advanced Calculator v{}", env!("CARGO_PKG_VERSION"));
    println!("Select operation:\n1. Add\n2. Subtract\n3. Multiply\n4. Divide\n5. Power");

    print!("Enter choice (1-5): ");
    io::stdout().flush().unwrap();
    let choice: u32 = match io::stdin().lines().next().unwrap().unwrap().parse() {
        Ok(num) if (1..=5).contains(&num) => num,
        _ => {
            println!("Invalid choice. Please enter a number between 1 and 5.");
            return;
        }
    };

    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let a: f64 = match io::stdin().lines().next().unwrap().unwrap().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number format");
            return;
        }
    };

    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let b: f64 = match io::stdin().lines().next().unwrap().unwrap().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number format");
            return;
        }
    };

    let result = match choice {
        1 => calc.calculate(Add, a, b),
        2 => calc.calculate(Subtract, a, b),
        3 => calc.calculate(Multiply, a, b),
        4 => calc.calculate(Divide, a, b),
        5 => calc.calculate(Power, a, b),
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    match result {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
