use super::Operation;

/// Represents the addition operation
pub struct Add;

impl Operation for Add {
    /// Executes the addition operation
    ///
    /// # Arguments
    /// * `a` - First operand
    /// * `b` - Second operand
    ///
    /// # Returns
    /// * `Ok(f64)` - The sum of a and b
    ///
    /// # Example
    /// ```
    /// use calculator_rs::operations::{Operation, arithmetic::Add};
    ///
    /// let add = Add;
    /// assert_eq!(add.execute(2.0, 3.0), Ok(5.0));
    /// ```
    fn execute(&self, a: f64, b: f64) -> Result<f64, String> {
        Ok(a + b)
    }
}

/// Represents the subtraction operation
pub struct Subtract;

impl Operation for Subtract {
    /// Executes the subtraction operation
    ///
    /// # Arguments
    /// * `a` - First operand
    /// * `b` - Second operand
    ///
    /// # Returns
    /// * `Ok(f64)` - The difference between a and b
    ///
    /// # Example
    /// ```
    /// use calculator_rs::operations::{Operation, arithmetic::Subtract};
    ///
    /// let sub = Subtract;
    /// assert_eq!(sub.execute(5.0, 3.0), Ok(2.0));
    /// ```
    fn execute(&self, a: f64, b: f64) -> Result<f64, String> {
        Ok(a - b)
    }
}

/// Represents the multiplication operation
pub struct Multiply;

impl Operation for Multiply {
    /// Executes the multiplication operation
    ///
    /// # Arguments
    /// * `a` - First operand
    /// * `b` - Second operand
    ///
    /// # Returns
    /// * `Ok(f64)` - The product of a and b
    ///
    /// # Example
    /// ```
    /// use calculator_rs::operations::{Operation, arithmetic::Multiply};
    ///
    /// let mul = Multiply;
    /// assert_eq!(mul.execute(2.0, 3.0), Ok(6.0));
    /// ```
    fn execute(&self, a: f64, b: f64) -> Result<f64, String> {
        Ok(a * b)
    }
}

/// Represents the division operation
pub struct Divide;

impl Operation for Divide {
    /// Executes the division operation
    ///
    /// # Arguments
    /// * `a` - First operand (dividend)
    /// * `b` - Second operand (divisor)
    ///
    /// # Returns
    /// * `Ok(f64)` - The quotient of a divided by b
    /// * `Err(String)` - Error message if division by zero is attempted
    ///
    /// # Example
    /// ```
    /// use calculator_rs::operations::{Operation, arithmetic::Divide};
    ///
    /// let div = Divide;
    /// assert_eq!(div.execute(6.0, 2.0), Ok(3.0));
    /// assert!(div.execute(5.0, 0.0).is_err());
    /// ```
    fn execute(&self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let add = Add;
        assert_eq!(add.execute(2.0, 3.0), Ok(5.0));
        assert_eq!(add.execute(-2.0, 3.0), Ok(1.0));
        assert_eq!(add.execute(0.0, 0.0), Ok(0.0));
    }

    #[test]
    fn test_subtract() {
        let sub = Subtract;
        assert_eq!(sub.execute(5.0, 3.0), Ok(2.0));
        assert_eq!(sub.execute(3.0, 5.0), Ok(-2.0));
        assert_eq!(sub.execute(0.0, 0.0), Ok(0.0));
    }

    #[test]
    fn test_multiply() {
        let mul = Multiply;
        assert_eq!(mul.execute(2.0, 3.0), Ok(6.0));
        assert_eq!(mul.execute(-2.0, 3.0), Ok(-6.0));
        assert_eq!(mul.execute(0.0, 5.0), Ok(0.0));
    }

    #[test]
    fn test_divide() {
        let div = Divide;
        assert_eq!(div.execute(6.0, 2.0), Ok(3.0));
        assert_eq!(div.execute(5.0, 2.0), Ok(2.5));
        assert_eq!(div.execute(0.0, 5.0), Ok(0.0));
        assert!(div.execute(5.0, 0.0).is_err());
    }
}
