/// Comprehensive test for arithmetic properties
use physics_layer::primitives::{add, div_f64, div_i32, mul, sub, ArithmeticError};

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

#[test]
fn test_addition_properties() {
    // Commutative property: a + b = b + a
    assert_eq!(add(5, 3), add(3, 5));
    assert_eq!(add(-2, 7), add(7, -2));
    assert_eq!(add(0, 10), add(10, 0));

    // Associative property: (a + b) + c = a + (b + c)
    let a = 2;
    let b = 3;
    let c = 4;
    let left = add(add(a, b).unwrap(), c).unwrap();
    let right = add(a, add(b, c).unwrap()).unwrap();
    assert_eq!(left, right);

    // Identity element: a + 0 = a
    assert_eq!(add(42, 0), Ok(42));
    assert_eq!(add(0, 42), Ok(42));
    assert_eq!(add(-17, 0), Ok(-17));

    // Inverse property: a + (-a) = 0
    assert_eq!(add(8, -8), Ok(0));
    assert_eq!(add(-15, 15), Ok(0));
}

#[test]
fn test_multiplication_properties() {
    // Commutative property: a * b = b * a
    assert_eq!(mul(3, 7), mul(7, 3));
    assert_eq!(mul(-2, 5), mul(5, -2));
    assert_eq!(mul(0, 10), mul(10, 0));

    // Associative property: (a * b) * c = a * (b * c)
    let a = 2;
    let b = 3;
    let c = 4;
    let left = mul(mul(a, b).unwrap(), c).unwrap();
    let right = mul(a, mul(b, c).unwrap()).unwrap();
    assert_eq!(left, right);

    // Identity element: a * 1 = a
    assert_eq!(mul(42, 1), Ok(42));
    assert_eq!(mul(1, 42), Ok(42));
    assert_eq!(mul(-17, 1), Ok(-17));

    // Zero property: a * 0 = 0
    assert_eq!(mul(8, 0), Ok(0));
    assert_eq!(mul(0, 8), Ok(0));
    assert_eq!(mul(-15, 0), Ok(0));

    // Distributive property: a * (b + c) = (a * b) + (a * c)
    let a = 3;
    let b = 4;
    let c = 5;
    let left = mul(a, add(b, c).unwrap()).unwrap();
    let right = add(mul(a, b).unwrap(), mul(a, c).unwrap()).unwrap();
    assert_eq!(left, right);
}

#[test]
fn test_subtraction_properties() {
    // Anti-commutative property: a - b = -(b - a)
    let result1 = sub(10, 4).unwrap();
    let expected1 = -sub(4, 10).unwrap();
    assert_eq!(result1, expected1);

    let result2 = sub(7, 3).unwrap();
    let expected2 = -sub(3, 7).unwrap();
    assert_eq!(result2, expected2);

    // Identity property: a - 0 = a
    assert_eq!(sub(42, 0), Ok(42));
    assert_eq!(sub(-17, 0), Ok(-17));

    // Inverse property: a - a = 0
    assert_eq!(sub(8, 8), Ok(0));
    assert_eq!(sub(-15, -15), Ok(0));
}

#[test]
fn test_division_properties() {
    // Division by 1 returns the original number
    assert_eq!(div_i32(42, 1), Ok(42));
    assert_eq!(div_i32(-17, 1), Ok(-17));
    assert_eq!(div_f64(3.14, 1.0), Ok(3.14));

    // Division of 0 by non-zero number
    assert_eq!(div_i32(0, 5), Ok(0));
    assert_eq!(div_f64(0.0, 2.0), Ok(0.0));

    // Division by zero should return error
    assert_eq!(div_i32(10, 0), Err(ArithmeticError::DivisionByZero));
    assert_eq!(div_f64(5.0, 0.0), Err(ArithmeticError::DivisionByZero));
    assert_eq!(div_i32(0, 0), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn test_arithmetic_with_different_types() {
    // Test with integers
    assert_eq!(add(5, 3), Ok(8));
    assert_eq!(sub(10, 4), Ok(6));
    assert_eq!(mul(3, 7), Ok(21));
    assert_eq!(div_i32(15, 3), Ok(5));

    // Test with floating point numbers
    assert_eq!(add(3.5, 2.1), Ok(5.6));
    assert_eq!(sub(7.5, 2.3), Ok(5.2));
    assert_eq!(mul(2.5, 4.0), Ok(10.0));
    assert_eq!(div_f64(15.0, 3.0), Ok(5.0));

    // Test with negative numbers
    assert_eq!(add(-5, -3), Ok(-8));
    assert_eq!(sub(-10, -4), Ok(-6));
    assert_eq!(mul(-3, -7), Ok(21));
    assert_eq!(div_i32(-15, -3), Ok(5));
}

#[test]
fn test_edge_cases() {
    // Large numbers
    assert_eq!(add(1000000, 1000000), Ok(2000000));
    assert_eq!(mul(1000, 1000), Ok(1000000));

    // Small numbers
    assert_eq!(add(1, 1), Ok(2));
    assert_eq!(sub(1, 1), Ok(0));
    assert_eq!(mul(1, 1), Ok(1));

    // Zero cases
    assert_eq!(add(0, 0), Ok(0));
    assert_eq!(sub(0, 0), Ok(0));
    assert_eq!(mul(0, 0), Ok(0));
}

#[test]
fn test_arithmetic_identities() {
    // Additive identity: a + 0 = a
    assert_eq!(add(42, 0), Ok(42));
    assert_eq!(add(0, 42), Ok(42));

    // Multiplicative identity: a * 1 = a
    assert_eq!(mul(42, 1), Ok(42));
    assert_eq!(mul(1, 42), Ok(42));

    // Additive inverse: a + (-a) = 0
    assert_eq!(add(42, -42), Ok(0));
    assert_eq!(add(-42, 42), Ok(0));

    // Multiplicative inverse (for non-zero): a * (1/a) = 1 (approximately for floats)
    let result = mul(2.0, div_f64(1.0, 2.0).unwrap()).unwrap();
    assert!(result - 1.0 < 0.0001);
}

#[test]
fn test_complex_arithmetic_expressions() {
    // Test: (a + b) * (c - d) = a*c - a*d + b*c - b*d
    let a = 2;
    let b = 3;
    let c = 4;
    let d = 1;

    let left = mul(add(a, b).unwrap(), sub(c, d).unwrap()).unwrap();
    let right = sub(
        add(mul(a, c).unwrap(), mul(b, c).unwrap()).unwrap(),
        add(mul(a, d).unwrap(), mul(b, d).unwrap()).unwrap(),
    )
    .unwrap();

    // This should be: (2+3)*(4-1) = 5*3 = 15
    // And: (2*4 + 3*4) - (2*1 + 3*1) = (8 + 12) - (2 + 3) = 20 - 5 = 15
    assert_eq!(left, 15);
    assert_eq!(right, 15);
    assert_eq!(left, right);
}
