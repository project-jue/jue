#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Value;
    use crate::vm::state::VmState;
    use crate::vm::verification::{verify_vm_state, Verifiable, MAX_VERIFIABLE_DEPTH};

    #[test]
    fn test_call_frame_verification() {
        let mut frame = CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: None,
            recursion_depth: 1,
            locals: vec![Value::Int(42)],
            closed_over: std::collections::HashMap::new(),
            is_tail_call: false,
            frame_id: 1,
            code_index: 0,
        };

        assert!(frame.verify_invariants().is_ok());

        frame.stack_start = 2;
        assert!(matches!(
            frame.verify_invariants(),
            Err(crate::vm::verification::VerificationError::StackConsistency { .. })
        ));
    }

    #[test]
    fn test_recursion_depth_verification() {
        let frame = CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: None,
            recursion_depth: MAX_VERIFIABLE_DEPTH + 1,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: false,
            frame_id: 1,
            code_index: 0,
        };

        assert!(matches!(
            frame.verify_invariants(),
            Err(crate::vm::verification::VerificationError::RecursionDepth { .. })
        ));
    }

    #[test]
    fn test_tail_call_consistency_verification() {
        let frame = CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: None,
            recursion_depth: 0,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: true,
            frame_id: 1,
            code_index: 0,
        };

        assert!(matches!(
            frame.verify_invariants(),
            Err(crate::vm::verification::VerificationError::TailCallConsistency { .. })
        ));
    }

    #[test]
    fn test_vm_state_verification() {
        let mut vm = VmState::new(vec![], vec![], 1000, 1024, 1, 100);

        // Empty VM should verify successfully
        assert!(verify_vm_state(&vm).is_ok());

        // Add a valid call frame
        vm.call_stack.push(CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: None,
            recursion_depth: 1,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: false,
            frame_id: 1,
            code_index: 0,
        });

        assert!(verify_vm_state(&vm).is_ok());
    }

    #[test]
    fn test_frame_id_sequence_verification() {
        let mut vm = VmState::new(vec![], vec![], 1000, 1024, 1, 100);

        // Add frames with non-sequential IDs
        vm.call_stack.push(CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: None,
            recursion_depth: 1,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: false,
            frame_id: 2,
            code_index: 0,
        });

        vm.call_stack.push(CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: None,
            recursion_depth: 1,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: false,
            frame_id: 1, // Non-sequential
            code_index: 1,
        });

        assert!(matches!(
            verify_vm_state(&vm),
            Err(crate::vm::verification::VerificationError::InvalidState { .. })
        ));
    }

    #[test]
    fn test_proof_context_generation() {
        let frame = CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: None,
            recursion_depth: 1,
            locals: vec![Value::Int(42), Value::Bool(true)],
            closed_over: {
                let mut map = std::collections::HashMap::new();
                map.insert(0, Value::Int(100));
                map
            },
            is_tail_call: false,
            frame_id: 1,
            code_index: 0,
        };

        let context = frame.generate_proof_context();
        assert_eq!(context.frame_id, 1);
        assert_eq!(context.stack_start, 0);
        assert_eq!(context.locals_count, 2);
        assert_eq!(context.closed_over_count, 1);
        assert_eq!(context.recursion_depth, 1);
        assert!(!context.is_tail_call);
    }

    #[test]
    fn test_verification_report_generation() {
        let mut vm = VmState::new(vec![], vec![], 1000, 1024, 1, 100);

        // Add a valid frame
        vm.call_stack.push(CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: None,
            recursion_depth: 1,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: false,
            frame_id: 1,
            code_index: 0,
        });

        let report = crate::vm::verification::generate_verification_report(&vm);
        assert!(report.is_valid);
        assert_eq!(report.frame_verifications.len(), 1);
        assert!(report.global_invariants.len() > 0);
    }
}
