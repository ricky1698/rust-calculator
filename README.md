# rust-calculator

A simple calculator implementation in Rust that demonstrates clean architecture, modular design, and comprehensive testing strategies.

## Project Structure

```
src/
├── operations/
│   ├── arithmetic.rs           # Basic arithmetic operations (Add, Subtract, Multiply, Divide)
│   └── advanced.rs             # Advanced operations (Power)
├── tests/
│   ├── features/
│   │   └── addition.feature    # Cucumber feature files for BDD
│   ├── integration_tests.rs    # Integration tests
│   └── e2e_tests.rs            # End-to-End tests
├── calculator.rs               # Calculator struct and implementation
├── operations.rs                # Operation trait and implementation
├── lib.rs                      # Library entry point
└── main.rs                     # Main entry point

```

## Features

- Basic arithmetic operations: addition, subtraction, multiplication, division
- Advanced operations: power/exponentiation
- Error handling for edge cases (e.g., division by zero)
- Type-safe implementation using Rust's Result type

## Testing Strategy

### Unit Tests
Each operation module contains unit tests that verify the behavior of individual operations:
- Basic arithmetic operations testing in `src/operations/arithmetic.rs`
- Advanced operations testing in `src/operations/advanced.rs`

### Integration Tests
Located in `tests/integration_tests.rs`, these tests verify:
- Calculator operations working together
- Edge cases handling
- Combined functionality testing

### End-to-End Tests
Located in `tests/e2e_tests.rs`, using Cucumber for behavior-driven development (BDD):
- Feature-based testing
- Natural language test specifications
- Scenario-based testing

## Usage

```rust
use calculator_rs::calculator::Calculator;
use calculator_rs::operations::arithmetic::Add;

let calc = Calculator::new();
let result = calc.calculate(Add, 2.0, 3.0); // Returns Ok(5.0)
```

## Development

To run tests:
```bash
# Run unit and integration tests
cargo test

# Run E2E tests
cargo test --test e2e_tests
```