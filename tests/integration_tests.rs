use calculator_rs::calculator::Calculator;
use calculator_rs::operations::advanced::*;
use calculator_rs::operations::arithmetic::*;

#[test]
fn test_calculator_operations() {
    let calc = Calculator::new();

    // Test addition
    assert_eq!(calc.calculate(Add, 2.0, 3.0), Ok(5.0));

    // Test subtraction
    assert_eq!(calc.calculate(Subtract, 5.0, 3.0), Ok(2.0));

    // Test multiplication
    assert_eq!(calc.calculate(Multiply, 4.0, 3.0), Ok(12.0));

    // Test division
    assert_eq!(calc.calculate(Divide, 10.0, 2.0), Ok(5.0));
    assert!(calc.calculate(Divide, 10.0, 0.0).is_err());

    // Test power
    assert_eq!(calc.calculate(Power, 2.0, 3.0), Ok(8.0));
}

#[test]
fn test_calculator_edge_cases() {
    let calc = Calculator::new();

    // Test operations with zero
    assert_eq!(calc.calculate(Add, 0.0, 0.0), Ok(0.0));
    assert_eq!(calc.calculate(Multiply, 5.0, 0.0), Ok(0.0));
    assert_eq!(calc.calculate(Power, 0.0, 5.0), Ok(0.0));

    // Test operations with negative numbers
    assert_eq!(calc.calculate(Add, -2.0, 3.0), Ok(1.0));
    assert_eq!(calc.calculate(Multiply, -2.0, -3.0), Ok(6.0));

    // Test power with fractional exponent
    assert_eq!(calc.calculate(Power, 4.0, 0.5), Ok(2.0));
}
