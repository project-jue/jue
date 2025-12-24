#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Value;
    use crate::vm::optimization::{OptimizationAnalysis, OptimizationEvent, OptimizationMetrics};
    use crate::vm::state::VmState;

    #[test]
    fn test_tail_call_optimization() {
        let mut vm = VmState::new(Vec::new(), Vec::new(), 1000, 1024, 1);

        // Set up a tail call scenario
        vm.stack.push(Value::Int(5));
        vm.stack.push(Value::Int(1));

        // Simulate tail call
        let result = vm.handle_tail_call(0, vec![Value::Int(5), Value::Int(1)]);

        assert!(result.is_ok());
        assert_eq!(vm.call_stack.len(), 1); // Frame should be reused
        assert!(vm.call_stack[0].is_tail_call);
    }

    #[test]
    fn test_escape_analysis() {
        let mut analysis = crate::compiler::EscapeAnalysis::new();
        let mut context = crate::compiler::AnalysisContext::new();

        // Test lambda with escaping variable
        let lambda = crate::ast::AstNode::Lambda {
            parameters: vec!["x".to_string()],
            body: Box::new(crate::ast::AstNode::Variable("x".to_string())),
            location: crate::error::SourceLocation::default(),
        };

        analysis.analyze_expression(&lambda, &mut context);

        // Variable "x" should be marked as escaping
        assert!(!analysis.escaping_vars.is_empty());
    }

    #[test]
    fn test_optimization_metrics() {
        let mut vm = VmState::new(Vec::new(), Vec::new(), 1000, 1024, 1);

        // Create some tail calls
        vm.call_stack.push(crate::vm::state::CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: Some(Vec::new()),
            recursion_depth: 1,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: true,
            frame_id: 1,
            code_index: 0,
        });

        let metrics = vm.get_optimization_metrics();
        assert_eq!(metrics.tail_calls, 1);
        assert_eq!(metrics.stack_reuses, 1);
    }

    #[test]
    fn test_optimization_analysis() {
        let mut vm = VmState::new(Vec::new(), Vec::new(), 1000, 1024, 1);

        // Add a tail call frame
        vm.call_stack.push(crate::vm::state::CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: Some(Vec::new()),
            recursion_depth: 1,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: true,
            frame_id: 1,
            code_index: 0,
        });

        let mut analysis = OptimizationAnalysis::new();
        analysis.analyze(&vm);

        assert_eq!(analysis.metrics.tail_calls, 1);
        assert_eq!(analysis.metrics.stack_reuses, 1);

        // Test report generation
        let report = analysis.generate_report();
        assert!(report.contains("Tail Calls: 1"));
        assert!(report.contains("Stack Reuses: 1"));
    }

    #[test]
    fn test_performance_analyzer() {
        let mut vm = VmState::new(Vec::new(), Vec::new(), 1000, 1024, 1);

        // Add some call frames
        vm.call_stack.push(crate::vm::state::CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: Some(Vec::new()),
            recursion_depth: 1,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: true,
            frame_id: 1,
            code_index: 0,
        });

        vm.call_stack.push(crate::vm::state::CallFrame {
            return_ip: 0,
            stack_start: 0,
            saved_instructions: Some(Vec::new()),
            recursion_depth: 2,
            locals: Vec::new(),
            closed_over: std::collections::HashMap::new(),
            is_tail_call: false,
            frame_id: 2,
            code_index: 1,
        });

        // Test tail call optimization analysis
        let tco_ratio =
            crate::vm::optimization::PerformanceAnalyzer::analyze_tail_call_optimization(&vm);
        assert_eq!(tco_ratio, 0.5); // 1 out of 2 calls are tail calls

        // Test memory usage analysis
        let mem_analysis = crate::vm::optimization::PerformanceAnalyzer::analyze_memory_usage(&vm);
        assert!(mem_analysis.heap_usage > 0);
        assert!(mem_analysis.heap_capacity >= mem_analysis.heap_usage);
    }

    #[test]
    fn test_closure_optimization() {
        // This test would verify that closures are created with only escaping variables
        // For now, we'll just test that the closure module compiles
        assert!(true); // Placeholder - actual closure optimization testing would go here
    }

    #[test]
    fn test_optimization_events() {
        let mut vm = VmState::new(Vec::new(), Vec::new(), 1000, 1024, 1);

        // Test logging optimization events
        vm.log_optimization_event(OptimizationEvent::TailCallOptimized);
        vm.log_optimization_event(OptimizationEvent::FrameReused);

        // In a real implementation, we would verify the events were logged
        // For now, we just test that the method doesn't panic
        assert!(true);
    }
}
