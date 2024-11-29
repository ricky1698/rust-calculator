use cucumber::{given, then, when, World};

use calculator_rs::calculator::Calculator;
use calculator_rs::operations::arithmetic::*;

// Define the World struct
#[derive(Debug, Default, World)]
#[world(init = Self::new)]
pub struct CalcWorld {
    num1: f64,
    num2: f64,
    result: f64,
}

impl CalcWorld {
    fn new() -> Self {
        Self::default()
    }
}

// Define the test steps

/// This step initializes the world with two numbers
#[given(expr = "I have the numbers {float} and {float}")]
async fn have_numbers(world: &mut CalcWorld, num1: f64, num2: f64) {
    world.num1 = num1;
    world.num2 = num2;
}

/// This step adds the two numbers together
#[when("I add them together")]
async fn add_numbers(world: &mut CalcWorld) {
    let calc = Calculator::new();
    let result = calc.calculate(Add, world.num1, world.num2);

    match result {
        Ok(result) => world.result = result,
        Err(_) => panic!("Error occurred while adding numbers"),
    }
}

/// This step checks if the result is equal to the expected value
#[then(expr = "the result should be {float}")]
async fn check_result(world: &mut CalcWorld, expected: f64) {
    assert_eq!(world.result, expected);
}

#[tokio::main]
async fn main() {
    CalcWorld::cucumber().run("tests/features").await;
}
