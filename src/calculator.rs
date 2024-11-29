use crate::operations::Operation;

/// A simple calculator that can perform operations on two floating-point numbers.
pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    /// Performs the given operation on two floating-point numbers.
    ///
    /// # Parameters
    /// - `op`: The operation to perform, which implements the `Operation` trait.
    /// - `a`: The first operand.
    /// - `b`: The second operand.
    ///
    /// # Returns
    /// A `Result` containing the result of the operation as `f64` if successful,
    /// or a `String` containing an error message if the operation fails.
    pub fn calculate<Op: Operation>(&self, op: Op, a: f64, b: f64) -> Result<f64, String> {
        op.execute(a, b)
    }
}
