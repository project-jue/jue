/// Integration bridge module
/// Provides conversion functions between Core-World and Physics-World types
use core_world::core_expr::CoreExpr;
use physics_world::types::Value;

/// Convert a Physics-World Value to a Core-World CoreExpr
/// This function provides the bridge between the two layers
pub fn core_expr_from_value(value: &Value) -> CoreExpr {
    match value {
        Value::Nil => CoreExpr::Nat(0), // Represent nil as zero for simplicity
        Value::Bool(true) => CoreExpr::Nat(1),
        Value::Bool(false) => CoreExpr::Nat(0),
        Value::Int(n) => CoreExpr::Nat(*n as u64),
        Value::Symbol(usize) => CoreExpr::Var(*usize), // Use symbol index as variable index
        Value::Pair(_) => {
            // For pairs, we'll create a placeholder representation
            // In a full implementation, this would need proper pair handling
            CoreExpr::Nat(42) // Placeholder for pair representation
        }
        Value::Closure(_) => {
            // For closures, we'll create a placeholder representation
            // In a full implementation, this would need proper closure handling
            CoreExpr::Nat(43) // Placeholder for closure representation
        }
        Value::ActorId(id) => CoreExpr::Nat(*id as u64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_world::core_expr::{app, lam, var};

    #[test]
    fn test_core_expr_from_value_conversions() {
        // Test basic value conversions
        assert_eq!(core_expr_from_value(&Value::Nil), CoreExpr::Nat(0));
        assert_eq!(core_expr_from_value(&Value::Bool(true)), CoreExpr::Nat(1));
        assert_eq!(core_expr_from_value(&Value::Bool(false)), CoreExpr::Nat(0));
        assert_eq!(core_expr_from_value(&Value::Int(42)), CoreExpr::Nat(42));
        assert_eq!(core_expr_from_value(&Value::Symbol(5)), CoreExpr::Var(5));
        assert_eq!(
            core_expr_from_value(&Value::ActorId(123)),
            CoreExpr::Nat(123)
        );
    }
}
