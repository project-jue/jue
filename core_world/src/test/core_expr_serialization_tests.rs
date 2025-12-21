use super::*;
use crate::core_expr::{app, lam, nat, pair, var};

#[test]
fn test_var_serialization() {
    let expr = var(42);
    let serialized = serialize_core_expr(&expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(expr, deserialized);
}

#[test]
fn test_lam_serialization() {
    let expr = lam(var(0));
    let serialized = serialize_core_expr(&expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(expr, deserialized);
}

#[test]
fn test_app_serialization() {
    let expr = app(lam(var(0)), var(1));
    let serialized = serialize_core_expr(&expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(expr, deserialized);
}

#[test]
fn test_nat_serialization() {
    let expr = nat(12345);
    let serialized = serialize_core_expr(&expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(expr, deserialized);
}

#[test]
fn test_pair_serialization() {
    let expr = pair(var(0), var(1));
    let serialized = serialize_core_expr(&expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(expr, deserialized);
}

#[test]
fn test_complex_serialization() {
    let expr = app(lam(pair(var(0), nat(5))), nat(10));
    let serialized = serialize_core_expr(&expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(expr, deserialized);
}

#[test]
fn test_serialization_roundtrip() {
    let expr = app(lam(var(0)), var(42));
    let serialized = serialize_core_expr(&expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(expr, deserialized);
}

#[test]
fn test_empty_input_error() {
    let result = deserialize_core_expr(&[]);
    assert!(matches!(result, Err(ParseError::EmptyInput)));
}

#[test]
fn test_incomplete_data_error() {
    // Incomplete Var - only tag, no data
    let incomplete_var = vec![0x01];
    let result = deserialize_core_expr(&incomplete_var);
    assert!(matches!(result, Err(ParseError::IncompleteData)));

    // Incomplete Nat - only tag, no data
    let incomplete_nat = vec![0x04];
    let result = deserialize_core_expr(&incomplete_nat);
    assert!(matches!(result, Err(ParseError::IncompleteData)));
}

#[test]
fn test_invalid_tag_error() {
    let invalid_tag = vec![0xFF];
    let result = deserialize_core_expr(&invalid_tag);
    assert!(matches!(result, Err(ParseError::InvalidTag(0xFF))));
}
