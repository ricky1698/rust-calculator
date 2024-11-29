use super::Operation;

/// Represents the power operation (exponentiation)
pub struct Power;

impl Operation for Power {
    /// Executes the power operation
    ///
    /// # Arguments
    /// * `a` - Base number
    /// * `b` - Exponent
    ///
    /// # Returns
    /// * `Ok(f64)` - The result of raising a to the power of b
    ///
    /// # Example
    /// ```
    /// use calculator_rs::operations::{Operation, advanced::Power};
    ///
    /// let power = Power;
    /// assert_eq!(power.execute(2.0, 3.0), Ok(8.0));
    /// assert_eq!(power.execute(2.0, 0.5), Ok(2.0_f64.sqrt()));
    /// ```
    fn execute(&self, a: f64, b: f64) -> Result<f64, String> {
        Ok(a.powf(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        let power = Power;
        assert_eq!(power.execute(2.0, 3.0), Ok(8.0));
        assert_eq!(power.execute(2.0, 0.0), Ok(1.0));
        assert_eq!(power.execute(0.0, 5.0), Ok(0.0));
        assert_eq!(power.execute(2.0, 0.5), Ok(2.0_f64.sqrt()));
    }
}
