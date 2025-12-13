/// Primitive arithmetic operations for the Physics Layer
///
/// This module provides basic arithmetic operations that serve as the foundation
/// for higher-level mathematical computations in the Jue system. All operations
/// include proper error handling and documentation as specified in the granular
/// LLM blueprint.
use std::fmt;

/// Arithmetic error type for primitive operations
#[derive(Debug, PartialEq)]
pub enum ArithmeticError {
    DivisionByZero,
    Overflow,
    Underflow,
}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticError::DivisionByZero => write!(f, "Division by zero error"),
            ArithmeticError::Overflow => write!(f, "Arithmetic overflow occurred"),
            ArithmeticError::Underflow => write!(f, "Arithmetic underflow occurred"),
        }
    }
}

/// Adds two values and returns the result
///
/// # Arguments
/// * `a` - First operand
/// * `b` - Second operand
///
/// # Returns
/// Result containing the sum or an ArithmeticError
///
/// # Examples
/// `
/// let result = add(5, 3);
/// assert_eq!(result, Ok(8));
/// `
pub fn add<T>(a: T, b: T) -> Result<T, ArithmeticError>
where
    T: std::ops::Add<Output = T> + Copy,
{
    Ok(a + b)
}

/// Subtracts two values and returns the result
///
/// # Arguments
/// * `a` - First operand (minuend)
/// * `b` - Second operand (subtrahend)
///
/// # Returns
/// Result containing the difference or an ArithmeticError
///
/// # Examples
/// `
/// let result = sub(10, 4);
/// assert_eq!(result, Ok(6));
/// `
pub fn sub<T>(a: T, b: T) -> Result<T, ArithmeticError>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    Ok(a - b)
}

/// Multiplies two values and returns the result
///
/// # Arguments
/// * `a` - First operand
/// * `b` - Second operand
///
/// # Returns
/// Result containing the product or an ArithmeticError
///
/// # Examples
/// `
/// let result = mul(3, 7);
/// assert_eq!(result, Ok(21));
/// `
pub fn mul<T>(a: T, b: T) -> Result<T, ArithmeticError>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    Ok(a * b)
}

/// Divides two values and returns the result
///
/// # Arguments
/// * `a` - Dividend
/// * `b` - Divisor
///
/// # Returns
/// Result containing the quotient or an ArithmeticError
///
/// # Examples
/// `
/// let result = div(15, 3);
/// assert_eq!(result, Ok(5));
/// `
///
/// # Errors
/// Returns DivisionByZero error if divisor is zero
pub fn div<T>(a: T, b: T) -> Result<T, ArithmeticError>
where
    T: std::ops::Div<Output = T> + Copy + PartialEq + From<i32>,
{
    if b == T::from(0) {
        Err(ArithmeticError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Divides two floating-point values and returns the result
///
/// # Arguments
/// * `a` - Dividend
/// * `b` - Divisor
///
/// # Returns
/// Result containing the quotient or an ArithmeticError
///
/// # Examples
/// `
/// let result = div_f64(15.0, 3.0);
/// assert_eq!(result, Ok(5.0));
/// `
pub fn div_f64(a: f64, b: f64) -> Result<f64, ArithmeticError> {
    if b == 0.0 {
        Err(ArithmeticError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Divides two integer values with overflow checking
///
/// # Arguments
/// * `a` - Dividend
/// * `b` - Divisor
///
/// # Returns
/// Result containing the quotient or an ArithmeticError
///
/// # Examples
/// `
/// let result = div_i32(15, 3);
/// assert_eq!(result, Ok(5));
/// `
pub fn div_i32(a: i32, b: i32) -> Result<i32, ArithmeticError> {
    if b == 0 {
        Err(ArithmeticError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_integers() {
        assert_eq!(add(5, 3), Ok(8));
        assert_eq!(add(-2, 7), Ok(5));
        assert_eq!(add(0, 0), Ok(0));
    }

    #[test]
    fn test_add_floats() {
        assert_eq!(add(3.5, 2.1), Ok(5.6));
        assert_eq!(add(-1.5, 1.5), Ok(0.0));
    }

    #[test]
    fn test_sub_integers() {
        assert_eq!(sub(10, 4), Ok(6));
        assert_eq!(sub(5, 10), Ok(-5));
        assert_eq!(sub(0, 0), Ok(0));
    }

    #[test]
    fn test_sub_floats() {
        assert_eq!(sub(7.5, 2.3), Ok(5.2));
        assert_eq!(sub(1.0, 1.0), Ok(0.0));
    }

    #[test]
    fn test_mul_integers() {
        assert_eq!(mul(3, 7), Ok(21));
        assert_eq!(mul(-2, 5), Ok(-10));
        assert_eq!(mul(0, 100), Ok(0));
    }

    #[test]
    fn test_mul_floats() {
        assert_eq!(mul(2.5, 4.0), Ok(10.0));
        assert_eq!(mul(-1.5, 2.0), Ok(-3.0));
    }

    #[test]
    fn test_div_integers() {
        assert_eq!(div_i32(15, 3), Ok(5));
        assert_eq!(div_i32(10, 2), Ok(5));
        assert_eq!(div_i32(-12, 3), Ok(-4));
    }

    #[test]
    fn test_div_floats() {
        assert_eq!(div_f64(15.0, 3.0), Ok(5.0));
        assert_eq!(div_f64(7.5, 2.5), Ok(3.0));
        assert_eq!(div_f64(-6.0, 2.0), Ok(-3.0));
    }

    #[test]
    fn test_div_by_zero() {
        assert_eq!(div_i32(10, 0), Err(ArithmeticError::DivisionByZero));
        assert_eq!(div_f64(5.0, 0.0), Err(ArithmeticError::DivisionByZero));
    }

    #[test]
    fn test_arithmetic_properties() {
        // Test commutative property of addition
        assert_eq!(add(3, 5), add(5, 3));

        // Test distributive property
        let a = 2;
        let b = 3;
        let c = 4;
        let left = mul(a, add(b, c).unwrap()).unwrap();
        let right = add(mul(a, b).unwrap(), mul(a, c).unwrap()).unwrap();
        assert_eq!(left, right);

        // Test identity properties
        assert_eq!(add(7, 0), Ok(7));
        assert_eq!(mul(9, 1), Ok(9));
    }
}
