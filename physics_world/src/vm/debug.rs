use crate::vm::state::{VmDebugSnapshot, VmState};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

/// Enhanced debugging support
#[derive(Clone, Serialize, Deserialize)]
pub struct Debugger {
    pub breakpoints: HashSet<usize>,
    pub watchpoints: HashMap<String, Watchpoint>,
    pub step_mode: bool,
    pub call_stack_depth: usize,
    pub debug_log: Vec<DebugEvent>,
    pub vm_state_history: Vec<VmStateSnapshot>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            breakpoints: HashSet::new(),
            watchpoints: HashMap::new(),
            step_mode: false,
            call_stack_depth: 0,
            debug_log: Vec::new(),
            vm_state_history: Vec::new(),
        }
    }

    pub fn add_breakpoint(&mut self, address: usize) {
        self.breakpoints.insert(address);
    }

    pub fn remove_breakpoint(&mut self, address: usize) {
        self.breakpoints.remove(&address);
    }

    pub fn add_watchpoint(&mut self, name: &str, expression: &str) {
        self.watchpoints.insert(
            name.to_string(),
            Watchpoint {
                expression: expression.to_string(),
                last_value: None,
            },
        );
    }

    pub fn check_breakpoints(&self, vm: &VmState) -> bool {
        self.breakpoints.contains(&vm.ip)
    }

    pub fn check_watchpoints(&mut self, vm: &VmState) -> Vec<WatchpointTrigger> {
        let mut triggers = Vec::new();

        // Clone watchpoints to avoid borrow checker issues
        let watchpoints = self.watchpoints.clone();
        for (name, mut watchpoint) in watchpoints {
            if let Some(current_value) = self.evaluate_watchpoint(&watchpoint.expression, vm) {
                if let Some(last_value) = &watchpoint.last_value {
                    if current_value != *last_value {
                        triggers.push(WatchpointTrigger {
                            name: name.clone(),
                            old_value: last_value.clone(),
                            new_value: current_value.clone(),
                        });
                    }
                }
                // Update the original watchpoint
                if let Some(original) = self.watchpoints.get_mut(&name) {
                    original.last_value = Some(current_value);
                }
            }
        }

        triggers
    }

    pub fn check_watchpoints_snapshot(
        &mut self,
        vm_snapshot: &VmDebugSnapshot,
    ) -> Vec<WatchpointTrigger> {
        let mut triggers = Vec::new();

        // Clone watchpoints to avoid borrow checker issues
        let watchpoints = self.watchpoints.clone();
        for (name, mut watchpoint) in watchpoints {
            if let Some(current_value) =
                self.evaluate_watchpoint_snapshot(&watchpoint.expression, vm_snapshot)
            {
                if let Some(last_value) = &watchpoint.last_value {
                    if current_value != *last_value {
                        triggers.push(WatchpointTrigger {
                            name: name.clone(),
                            old_value: last_value.clone(),
                            new_value: current_value.clone(),
                        });
                    }
                }
                // Update the original watchpoint
                if let Some(original) = self.watchpoints.get_mut(&name) {
                    original.last_value = Some(current_value);
                }
            }
        }

        triggers
    }

    pub fn log_debug_event(&mut self, event: DebugEvent) {
        self.debug_log.push(event);
    }

    pub fn capture_vm_state(&mut self, vm: &VmState) {
        self.vm_state_history.push(VmStateSnapshot::from_vm(vm));
    }

    pub fn capture_vm_state_snapshot(&mut self, vm_snapshot: &VmDebugSnapshot) {
        let snapshot = VmStateSnapshot {
            ip: vm_snapshot.instruction_pointer,
            stack_depth: vm_snapshot.stack.len(),
            call_stack_depth: vm_snapshot.call_stack.len(),
            memory_usage: vm_snapshot.memory_usage,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };
        self.vm_state_history.push(snapshot);
    }

    pub fn get_debug_info(&self) -> DebugInfo {
        DebugInfo {
            breakpoints: self.breakpoints.clone(),
            watchpoints: self.watchpoints.clone(),
            events: self.debug_log.clone(),
        }
    }

    /// Simple watchpoint evaluation - in a real implementation this would parse expressions
    fn evaluate_watchpoint(&self, expression: &str, vm: &VmState) -> Option<String> {
        // Simple implementation - just return the expression as the value
        // In a real debugger, this would evaluate the expression in the VM context
        Some(expression.to_string())
    }

    /// Simple watchpoint evaluation using snapshot data
    fn evaluate_watchpoint_snapshot(
        &self,
        expression: &str,
        vm_snapshot: &VmDebugSnapshot,
    ) -> Option<String> {
        // Simple implementation - just return the expression as the value
        // In a real debugger, this would evaluate the expression in the VM context
        Some(expression.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugEvent {
    pub timestamp: u64,
    pub event_type: DebugEventType,
    pub data: DebugEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugEventType {
    BreakpointHit,
    WatchpointTriggered,
    StepCompleted,
    FunctionEntry,
    FunctionExit,
    ExceptionThrown,
    ExceptionCaught,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugEventData {
    Breakpoint(usize),
    Watchpoint(WatchpointTrigger),
    Function(String),
    Exception(String),
    // Other data types...
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watchpoint {
    pub expression: String,
    pub last_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchpointTrigger {
    pub name: String,
    pub old_value: String,
    pub new_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfo {
    pub breakpoints: HashSet<usize>,
    pub watchpoints: HashMap<String, Watchpoint>,
    pub events: Vec<DebugEvent>,
}

/// Snapshot of VM state for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmStateSnapshot {
    pub ip: usize,
    pub stack_depth: usize,
    pub call_stack_depth: usize,
    pub memory_usage: usize,
    pub timestamp: u64,
}

impl VmStateSnapshot {
    pub fn from_vm(vm: &VmState) -> Self {
        Self {
            ip: vm.ip,
            stack_depth: vm.stack.len(),
            call_stack_depth: vm.call_stack.len(),
            memory_usage: vm.memory.next_free() as usize,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
}
