/// Integration test for Core-World and Physics-World bridge functionality
/// Tests the connection between the formal kernel and the VM

#[cfg(test)]
mod tests {
    use core_world::core_expr::{app, lam, nat, var, CoreExpr};
    use core_world::core_kernel::{alpha_equiv, normalize};
    use integration::core_expr_from_value;
    use physics_world::types::{OpCode, Value};
    use physics_world::vm::state::VmState;

    #[test]
    fn test_core_physics_bridge() {
        // Test basic conversion between Core-World and Physics-World types

        // Create a simple CoreExpr: λx.x (identity function)
        let identity = lam(var(0));

        // Test that the CoreExpr can be created and has expected properties
        match identity {
            CoreExpr::Lam(ref body) => {
                assert!(matches!(**body, CoreExpr::Var(0)));
            }
            _ => panic!("Expected lambda expression"),
        }

        // Test normalization of the identity function
        let normalized = normalize(identity.clone());
        assert_eq!(identity, normalized); // Identity should already be in normal form

        // Test alpha equivalence
        let identity2 = lam(var(0));
        assert!(alpha_equiv(identity, identity2));

        // Test conversion from Physics World Value to CoreExpr
        let test_values = vec![
            (Value::Nil, CoreExpr::Nat(0)),
            (Value::Bool(true), CoreExpr::Nat(1)),
            (Value::Bool(false), CoreExpr::Nat(0)),
            (Value::Int(42), CoreExpr::Nat(42)),
            (Value::Symbol(5), CoreExpr::Var(5)),
            (Value::ActorId(123), CoreExpr::Nat(123)),
        ];

        for (value, expected) in test_values {
            let result = core_expr_from_value(&value);
            assert_eq!(result, expected, "Failed to convert value {:?}", value);
        }
    }

    #[test]
    fn test_complex_expression_reduction() {
        // Test reduction of a more complex expression: (λx.x) (λy.y)
        // This should reduce to (λy.y) (identity function applied to identity function)

        let identity = lam(var(0));
        let app_expr = app(identity.clone(), identity.clone());

        // Test normalization
        let normalized = normalize(app_expr.clone());
        // The result should be the identity function
        assert_eq!(normalized, identity);

        // Test that alpha equivalence works for complex expressions
        let identity_applied_to_identity = app(lam(var(0)), lam(var(0)));
        let normalized2 = normalize(identity_applied_to_identity);
        assert_eq!(normalized, normalized2);
    }

    #[test]
    fn test_vm_state_creation() {
        // Test that we can create a VmState with basic OpCodes
        let bytecode = vec![OpCode::Nil, OpCode::Dup, OpCode::Pop];

        let constants = vec![Value::Int(42), Value::Bool(true), Value::Nil];

        // Create VM state with reasonable limits
        let vm_state = VmState::new(bytecode, constants, 1000, 1024);

        // Basic sanity check - the VM state should be created successfully
        // Note: We're not testing actual execution here, just construction
        assert_eq!(vm_state.steps_remaining, 1000);
    }

    #[test]
    fn test_nat_and_pair_expressions() {
        // Test CoreExpr variants for Nat and Pair
        let nat_expr = nat(42);
        assert_eq!(nat_expr, CoreExpr::Nat(42));

        let pair_expr = app(lam(var(0)), nat(1)); // Simple pair-like structure
        let normalized = normalize(pair_expr);

        // The application should reduce to the natural number
        assert_eq!(normalized, nat(1));
    }
}
