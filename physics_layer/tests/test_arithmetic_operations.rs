/// Arithmetic Operations Tests
use physics_layer::primitives::{add, div_f64, div_i32, mul, sub, ArithmeticError};

#[test]
fn test_arithmetic_operations() {
    // Test addition
    assert_eq!(add(5, 3), Ok(8));
    assert_eq!(add(-2, 7), Ok(5));
    assert_eq!(add(0, 0), Ok(0));

    // Test subtraction
    assert_eq!(sub(10, 4), Ok(6));
    assert_eq!(sub(5, 10), Ok(-5));
    assert_eq!(sub(0, 0), Ok(0));

    // Test multiplication
    assert_eq!(mul(3, 7), Ok(21));
    assert_eq!(mul(-2, 5), Ok(-10));
    assert_eq!(mul(0, 100), Ok(0));

    // Test division
    assert_eq!(div_i32(15, 3), Ok(5));
    assert_eq!(div_f64(15.0, 3.0), Ok(5.0));
}

#[test]
fn test_division_by_zero() {
    assert_eq!(div_i32(10, 0), Err(ArithmeticError::DivisionByZero));
    assert_eq!(div_f64(5.0, 0.0), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn test_arithmetic_properties() {
    // Commutative property of addition
    assert_eq!(add(3, 5), add(5, 3));

    // Distributive property
    let a = 2;
    let b = 3;
    let c = 4;
    let left = mul(a, add(b, c).unwrap()).unwrap();
    let right = add(mul(a, b).unwrap(), mul(a, c).unwrap()).unwrap();
    assert_eq!(left, right);

    // Identity properties
    assert_eq!(add(7, 0), Ok(7));
    assert_eq!(mul(9, 1), Ok(9));
}
