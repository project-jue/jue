use crate::vm::state::{VmDebugSnapshot, VmState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance monitoring system
#[derive(Clone, Serialize, Deserialize)]
pub struct PerformanceMonitor {
    pub metrics: PerformanceMetrics,
    pub counters: HashMap<String, u64>,
    pub timers: HashMap<String, u64>,
    pub samples: Vec<PerformanceSample>,
    pub sample_interval_millis: u64,
    pub last_sample_time_millis: u64,
}

impl PerformanceMonitor {
    pub fn new(sample_interval_millis: u64) -> Self {
        Self {
            metrics: PerformanceMetrics::default(),
            counters: HashMap::new(),
            timers: HashMap::new(),
            samples: Vec::new(),
            sample_interval_millis,
            last_sample_time_millis: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    pub fn increment_counter(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }

    pub fn start_timer(&mut self, name: &str) {
        self.timers.insert(
            name.to_string(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        );
    }

    pub fn stop_timer(&mut self, name: &str) {
        if let Some(start_time_millis) = self.timers.remove(name) {
            let duration_millis = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
                - start_time_millis;
            self.metrics.total_time_millis += duration_millis;
            self.metrics.counter_increments += 1;
        }
    }

    pub fn take_sample(&mut self, vm: &VmState) {
        let now_millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        if now_millis - self.last_sample_time_millis >= self.sample_interval_millis {
            let sample = PerformanceSample {
                timestamp: now_millis,
                instructions_executed: vm.steps_remaining,
                heap_usage: vm.memory.next_free() as usize,
                call_stack_depth: vm.call_stack.len(),
                counters: self.counters.clone(),
            };

            self.samples.push(sample);
            self.last_sample_time_millis = now_millis;
        }
    }

    pub fn take_sample_snapshot(&mut self, vm_snapshot: &VmDebugSnapshot) {
        let now_millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        if now_millis - self.last_sample_time_millis >= self.sample_interval_millis {
            let sample = PerformanceSample {
                timestamp: now_millis,
                instructions_executed: vm_snapshot.steps_remaining,
                heap_usage: vm_snapshot.memory_usage,
                call_stack_depth: vm_snapshot.call_stack.len(),
                counters: self.counters.clone(),
            };

            self.samples.push(sample);
            self.last_sample_time_millis = now_millis;
        }
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.clone()
    }

    pub fn get_analysis(&self) -> PerformanceAnalysis {
        let mut analysis = PerformanceAnalysis::default();

        if !self.samples.is_empty() {
            let first_sample = &self.samples[0];
            let last_sample = self.samples.last().unwrap();

            let time_diff_millis = last_sample.timestamp - first_sample.timestamp;
            let time_diff_secs = time_diff_millis as f64 / 1000.0;

            analysis.instructions_per_second =
                (last_sample.instructions_executed - first_sample.instructions_executed) as f64
                    / time_diff_secs;

            analysis.heap_growth_rate = (last_sample.heap_usage - first_sample.heap_usage) as f64
                / self.samples.len() as f64;
        }

        analysis
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub instructions_executed: u64,
    pub heap_allocations: u64,
    pub gc_collections: u64,
    pub total_time_millis: u64,
    pub counter_increments: u64,
    pub timer_operations: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub instructions_per_second: f64,
    pub heap_growth_rate: f64,
    pub gc_efficiency: f64,
    pub counter_trends: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSample {
    pub timestamp: u64,
    pub instructions_executed: u64,
    pub heap_usage: usize,
    pub call_stack_depth: usize,
    pub counters: HashMap<String, u64>,
}
