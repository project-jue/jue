/// Test for division by zero error handling
use physics_layer::primitives::{div_f64, div_i32, ArithmeticError};

#[test]
fn test_division_by_zero() {
    assert_eq!(div_i32(10, 0), Err(ArithmeticError::DivisionByZero));
    assert_eq!(div_f64(5.0, 0.0), Err(ArithmeticError::DivisionByZero));
}
