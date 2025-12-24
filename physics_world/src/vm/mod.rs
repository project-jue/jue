pub mod call_state;
pub mod closure_fix;
pub mod debug;
pub mod error;
pub mod execution;
pub mod gc;
pub mod gc_integration;
pub mod opcodes;
pub mod performance;
pub mod state;

pub use call_state::{CallFrame, CallStack};
pub use debug::{DebugEvent, DebugEventType, Debugger, Watchpoint, WatchpointTrigger};
pub use error::{ErrorContext, RecoveryAction, VmError};
pub use execution::ExecutionEngine;
pub use gc::{GarbageCollector, GcPtr, GcRoot, GcStats, HeapObject};
pub use gc_integration::{GcIntegration, MemoryAnalysis};
pub use performance::{
    PerformanceAnalysis, PerformanceMetrics, PerformanceMonitor, PerformanceSample,
};
pub use state::{InstructionResult, VmState, VmDebugger};
