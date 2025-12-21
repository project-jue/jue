use super::error::PhysicsError;
/// Resource management for the Physics World scheduler
use super::execution::{
    ActorResourceQuota, PhysicsScheduler, ResourceAllocationResult, ResourceMonitoringStats,
    ResourceUsageSnapshot,
};

impl PhysicsScheduler {
    /// Updates resource usage statistics and tracks global resource consumption.
    pub fn update_resource_usage(&mut self) {
        self.global_step_count += 1;

        // Calculate total memory usage across all actors
        let mut total_memory = 0;
        let mut active_actors = 0;
        let mut waiting_actors = 0;
        let mut total_fragmentation = 0.0;
        let mut fragmentation_count = 0;

        for actor in &self.actors {
            let memory_usage = actor.vm.memory.next_free() as usize;
            total_memory += memory_usage;
            active_actors += 1;

            if actor.is_waiting {
                waiting_actors += 1;
            }

            // Calculate fragmentation for this actor's memory
            let fragmentation = actor.vm.memory.fragmentation_ratio();
            total_fragmentation += fragmentation;
            fragmentation_count += 1;
        }

        self.total_memory_usage = total_memory;

        // Calculate average fragmentation
        let average_fragmentation = if fragmentation_count > 0 {
            total_fragmentation / fragmentation_count as f32
        } else {
            0.0
        };

        // Take a snapshot periodically (every 100 steps)
        if self.global_step_count % 100 == 0 {
            self.resource_usage_history.push(ResourceUsageSnapshot {
                timestamp: self.global_step_count,
                global_step_count: self.global_step_count,
                total_memory_usage: self.total_memory_usage,
                total_cpu_time: self.global_step_count,
                active_actors,
                memory_fragmentation: average_fragmentation,
            });

            // Limit history size to prevent unbounded growth
            if self.resource_usage_history.len() > 1000 {
                self.resource_usage_history.remove(0);
            }
        }
    }

    /// Gets current resource monitoring statistics.
    pub fn get_resource_stats(&self) -> ResourceMonitoringStats {
        let memory_usage_percent = if self.memory_limit > 0 {
            (self.total_memory_usage as f32 / self.memory_limit as f32) * 100.0
        } else {
            0.0
        };

        let cpu_usage_percent = if self.cpu_time_limit > 0 {
            (self.global_step_count as f32 / self.cpu_time_limit as f32) * 100.0
        } else {
            0.0
        };

        // Calculate average fragmentation
        let average_fragmentation = if !self.actors.is_empty() {
            let mut total_fragmentation = 0.0;
            for actor in &self.actors {
                total_fragmentation += actor.vm.memory.fragmentation_ratio();
            }
            total_fragmentation / self.actors.len() as f32
        } else {
            0.0
        };

        let active_actors = self.actors.iter().filter(|a| !a.is_waiting).count() as u32;
        let waiting_actors = self.actors.iter().filter(|a| a.is_waiting).count() as u32;

        ResourceMonitoringStats {
            memory_usage: self.total_memory_usage,
            memory_limit: self.memory_limit,
            memory_usage_percent,
            cpu_time_used: self.global_step_count,
            cpu_time_limit: self.cpu_time_limit,
            cpu_usage_percent,
            fragmentation_ratio: average_fragmentation,
            active_actors,
            waiting_actors,
        }
    }

    /// Sets global resource limits.
    pub fn set_resource_limits(&mut self, memory_limit: usize, cpu_time_limit: u64) {
        self.memory_limit = memory_limit;
        self.cpu_time_limit = cpu_time_limit;
        self.resource_quota_system.global_memory_limit = memory_limit;
        self.resource_quota_system.global_cpu_limit = cpu_time_limit;
    }

    /// Checks if global resource limits are exceeded.
    pub fn check_resource_limits(&self) -> bool {
        (self.memory_limit > 0 && self.total_memory_usage > self.memory_limit)
            || (self.cpu_time_limit > 0 && self.global_step_count > self.cpu_time_limit)
    }

    /// Allocates resources to an actor with quota checking.
    pub fn allocate_resources_to_actor(
        &mut self,
        actor_id: u32,
        memory_request: usize,
        cpu_request: u64,
    ) -> ResourceAllocationResult {
        // Check global limits first
        if self.memory_limit > 0 && self.total_memory_usage + memory_request > self.memory_limit {
            return ResourceAllocationResult::GlobalLimitExceeded;
        }

        if self.cpu_time_limit > 0 && self.global_step_count + cpu_request > self.cpu_time_limit {
            return ResourceAllocationResult::GlobalLimitExceeded;
        }

        // Get or create actor quota
        let quota = self
            .resource_quota_system
            .actor_quotas
            .entry(actor_id)
            .or_insert_with(|| ActorResourceQuota {
                actor_id,
                memory_quota: self.resource_quota_system.default_memory_quota,
                cpu_quota: self.resource_quota_system.default_cpu_quota,
                memory_used: 0,
                cpu_used: 0,
                last_updated: self.global_step_count,
            });

        // Check actor quota
        if quota.memory_used + memory_request > quota.memory_quota {
            return ResourceAllocationResult::QuotaExceeded;
        }

        if quota.cpu_used + cpu_request > quota.cpu_quota {
            return ResourceAllocationResult::QuotaExceeded;
        }

        // Allocate resources
        quota.memory_used += memory_request;
        quota.cpu_used += cpu_request;
        quota.last_updated = self.global_step_count;

        self.total_memory_usage += memory_request;

        ResourceAllocationResult::Success
    }

    /// Releases resources from an actor.
    pub fn release_resources_from_actor(
        &mut self,
        actor_id: u32,
        memory_release: usize,
        cpu_release: u64,
    ) {
        if let Some(quota) = self.resource_quota_system.actor_quotas.get_mut(&actor_id) {
            quota.memory_used = quota.memory_used.saturating_sub(memory_release);
            quota.cpu_used = quota.cpu_used.saturating_sub(cpu_release);
            quota.last_updated = self.global_step_count;

            self.total_memory_usage = self.total_memory_usage.saturating_sub(memory_release);
        }
    }

    /// Sets custom resource quotas for an actor.
    pub fn set_actor_resource_quota(
        &mut self,
        actor_id: u32,
        memory_quota: usize,
        cpu_quota: u64,
    ) -> Result<(), PhysicsError> {
        let quota = self
            .resource_quota_system
            .actor_quotas
            .entry(actor_id)
            .or_insert_with(|| ActorResourceQuota {
                actor_id,
                memory_quota: self.resource_quota_system.default_memory_quota,
                cpu_quota: self.resource_quota_system.default_cpu_quota,
                memory_used: 0,
                cpu_used: 0,
                last_updated: self.global_step_count,
            });

        quota.memory_quota = memory_quota;
        quota.cpu_quota = cpu_quota;

        Ok(())
    }

    /// Gets resource usage for a specific actor.
    pub fn get_actor_resource_usage(&self, actor_id: u32) -> Option<ActorResourceQuota> {
        self.resource_quota_system
            .actor_quotas
            .get(&actor_id)
            .cloned()
    }

    /// Gets the resource usage history.
    pub fn get_resource_usage_history(&self) -> &[ResourceUsageSnapshot] {
        &self.resource_usage_history
    }

    /// Clears resource usage history.
    pub fn clear_resource_usage_history(&mut self) {
        self.resource_usage_history.clear();
    }

    /// Triggers defragmentation for all actors if fragmentation exceeds threshold.
    pub fn auto_defragment_if_needed(&mut self, threshold: f32) {
        for actor in &mut self.actors {
            if actor.vm.memory.should_defragment()
                && actor.vm.memory.fragmentation_ratio() > threshold
            {
                let _ = actor.vm.memory.defragment(); // Ignore result for automatic defrag
            }
        }
    }
}
