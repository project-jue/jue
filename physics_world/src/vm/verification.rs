/// Verification infrastructure for Physics World VM
/// Provides runtime verification of VM state invariants
use crate::vm::error::{VerificationError, VmError};
use crate::vm::state::CallFrame;
use std::collections::HashMap;

/// Maximum verifiable recursion depth
pub const MAX_VERIFIABLE_DEPTH: u32 = 1000;

/// Verifiable trait for VM components
pub trait Verifiable {
    fn verify_invariants(&self) -> Result<(), VerificationError>;
    fn generate_proof_context(&self) -> ProofContext;
}

/// Verification error types
#[derive(Debug, Clone)]
pub enum VerificationError {
    StackConsistency {
        frame_id: u64,
        detail: String,
    },
    RecursionDepth {
        frame_id: u64,
        depth: u32,
        limit: u32,
    },
    TailCallConsistency {
        frame_id: u64,
        detail: String,
    },
    MemoryConsistency {
        detail: String,
    },
    InvalidState {
        detail: String,
    },
}

/// Proof context for verification
#[derive(Debug, Clone)]
pub struct ProofContext {
    pub frame_id: u64,
    pub stack_start: usize,
    pub locals_count: usize,
    pub closed_over_count: usize,
    pub recursion_depth: u32,
    pub is_tail_call: bool,
}

impl Verifiable for CallFrame {
    fn verify_invariants(&self) -> Result<(), VerificationError> {
        // Check stack consistency
        if self.stack_start > self.locals.len() {
            return Err(VerificationError::StackConsistency {
                frame_id: self.frame_id,
                detail: "stack_start exceeds locals length".to_string(),
            });
        }

        // Check recursion depth
        if self.recursion_depth > MAX_VERIFIABLE_DEPTH {
            return Err(VerificationError::RecursionDepth {
                frame_id: self.frame_id,
                depth: self.recursion_depth,
                limit: MAX_VERIFIABLE_DEPTH,
            });
        }

        // Check tail call consistency
        if self.is_tail_call && self.recursion_depth == 0 {
            return Err(VerificationError::TailCallConsistency {
                frame_id: self.frame_id,
                detail: "tail call flag set on root frame".to_string(),
            });
        }

        Ok(())
    }

    fn generate_proof_context(&self) -> ProofContext {
        ProofContext {
            frame_id: self.frame_id,
            stack_start: self.stack_start,
            locals_count: self.locals.len(),
            closed_over_count: self.closed_over.len(),
            recursion_depth: self.recursion_depth,
            is_tail_call: self.is_tail_call,
        }
    }
}

/// VM state verification functions
pub fn verify_vm_state(vm: &crate::vm::state::VmState) -> Result<(), VerificationError> {
    // Verify call stack consistency
    for (i, frame) in vm.call_stack.iter().enumerate() {
        frame.verify_invariants()?;

        // Check that frame IDs are unique and sequential
        if i > 0 && frame.frame_id <= vm.call_stack[i - 1].frame_id {
            return Err(VerificationError::InvalidState {
                detail: format!(
                    "Non-sequential frame IDs: {} <= {}",
                    frame.frame_id,
                    vm.call_stack[i - 1].frame_id
                ),
            });
        }
    }

    // Verify stack bounds
    if vm.stack.len() > vm.memory.capacity() {
        return Err(VerificationError::MemoryConsistency {
            detail: format!(
                "Stack size {} exceeds memory capacity {}",
                vm.stack.len(),
                vm.memory.capacity()
            ),
        });
    }

    Ok(())
}

/// Generate comprehensive verification report
pub fn generate_verification_report(vm: &crate::vm::state::VmState) -> VerificationReport {
    let mut report = VerificationReport {
        frame_verifications: Vec::new(),
        global_invariants: Vec::new(),
        warnings: Vec::new(),
        is_valid: true,
    };

    // Verify each call frame
    for frame in &vm.call_stack {
        let context = frame.generate_proof_context();
        match frame.verify_invariants() {
            Ok(_) => {
                report.frame_verifications.push(FrameVerification {
                    frame_id: frame.frame_id,
                    is_valid: true,
                    context,
                    errors: Vec::new(),
                });
            }
            Err(err) => {
                report.is_valid = false;
                report.frame_verifications.push(FrameVerification {
                    frame_id: frame.frame_id,
                    is_valid: false,
                    context,
                    errors: vec![err],
                });
            }
        }
    }

    // Check global invariants
    if vm.call_stack.len() > MAX_VERIFIABLE_DEPTH as usize {
        report.is_valid = false;
        report.global_invariants.push(GlobalInvariant {
            name: "call_stack_depth".to_string(),
            is_valid: false,
            detail: format!(
                "Call stack depth {} exceeds limit {}",
                vm.call_stack.len(),
                MAX_VERIFIABLE_DEPTH
            ),
        });
    } else {
        report.global_invariants.push(GlobalInvariant {
            name: "call_stack_depth".to_string(),
            is_valid: true,
            detail: format!("Call stack depth {} within limits", vm.call_stack.len()),
        });
    }

    // Add warnings for potential issues
    if vm.call_stack.len() > (MAX_VERIFIABLE_DEPTH / 2) as usize {
        report.warnings.push(VerificationWarning {
            severity: WarningSeverity::Medium,
            message: format!(
                "High call stack depth: {} (approaching limit {})",
                vm.call_stack.len(),
                MAX_VERIFIABLE_DEPTH
            ),
            suggestion: "Consider optimizing recursion or increasing stack limits".to_string(),
        });
    }

    report
}

/// Verification report structure
#[derive(Debug, Clone)]
pub struct VerificationReport {
    pub frame_verifications: Vec<FrameVerification>,
    pub global_invariants: Vec<GlobalInvariant>,
    pub warnings: Vec<VerificationWarning>,
    pub is_valid: bool,
}

/// Frame verification result
#[derive(Debug, Clone)]
pub struct FrameVerification {
    pub frame_id: u64,
    pub is_valid: bool,
    pub context: ProofContext,
    pub errors: Vec<VerificationError>,
}

/// Global invariant check result
#[derive(Debug, Clone)]
pub struct GlobalInvariant {
    pub name: String,
    pub is_valid: bool,
    pub detail: String,
}

/// Verification warning
#[derive(Debug, Clone)]
pub struct VerificationWarning {
    pub severity: WarningSeverity,
    pub message: String,
    pub suggestion: String,
}

/// Warning severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarningSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Convert verification errors to VM errors for compatibility
impl From<VerificationError> for VmError {
    fn from(err: VerificationError) -> Self {
        match err {
            VerificationError::StackConsistency { frame_id, detail } => VmError::HeapCorruption {
                context: Default::default(),
                message: format!("Stack consistency error in frame {}: {}", frame_id, detail),
            },
            VerificationError::RecursionDepth {
                frame_id,
                depth,
                limit,
            } => VmError::RecursionLimitExceeded {
                context: Default::default(),
                limit,
                current_depth: depth,
            },
            VerificationError::TailCallConsistency { frame_id, detail } => {
                VmError::HeapCorruption {
                    context: Default::default(),
                    message: format!(
                        "Tail call consistency error in frame {}: {}",
                        frame_id, detail
                    ),
                }
            }
            VerificationError::MemoryConsistency { detail } => VmError::MemoryLimitExceeded {
                context: Default::default(),
                limit: 0,
                requested: 0,
            },
            VerificationError::InvalidState { detail } => VmError::HeapCorruption {
                context: Default::default(),
                message: format!("Invalid VM state: {}", detail),
            },
        }
    }
}
