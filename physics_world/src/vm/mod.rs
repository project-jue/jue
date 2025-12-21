pub mod closure_fix;
pub mod debug;
pub mod error;
pub mod gc;
pub mod opcodes;
pub mod performance;
pub mod state;

pub use debug::{DebugEvent, DebugEventType, Debugger, Watchpoint, WatchpointTrigger};
pub use error::{ErrorContext, RecoveryAction, VmError};
pub use gc::{GarbageCollector, GcPtr, GcRoot, GcStats, HeapObject};
pub use performance::{
    PerformanceAnalysis, PerformanceMetrics, PerformanceMonitor, PerformanceSample,
};
pub use state::{InstructionResult, VmState};
