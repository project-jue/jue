use super::actor::Actor;
use super::error::PhysicsError;
/// Priority management for the Physics World scheduler
use super::execution::PhysicsScheduler;

impl PhysicsScheduler {
    /// Selects the next actor based on priority scheduling.
    pub fn select_next_actor_by_priority(&mut self) {
        if self.actors.is_empty() {
            self.current_actor_index = 0;
            return;
        }

        // Check if we need to prevent starvation
        if self.starvation_counter >= self.starvation_threshold {
            // Force run a low-priority actor to prevent starvation
            self.select_lowest_priority_actor();
            self.starvation_counter = 0;
            return;
        }

        // Find the highest priority actor that is ready to run
        let mut highest_priority = 0;
        let mut highest_priority_index = 0;
        let mut found_ready_actor = false;

        for (index, actor) in self.actors.iter().enumerate() {
            // Skip actors that are waiting for capabilities
            if actor.is_waiting {
                continue;
            }

            // Calculate effective priority (considering temporary boosts)
            let effective_priority = self.calculate_effective_priority(actor);

            if !found_ready_actor || effective_priority > highest_priority {
                highest_priority = effective_priority;
                highest_priority_index = index;
                found_ready_actor = true;
            }
        }

        if found_ready_actor {
            self.current_actor_index = highest_priority_index;
            // Increment starvation counter if this is a high-priority actor
            if highest_priority > 128 {
                // Consider priority > 128 as "high"
                self.starvation_counter += 1;
            }
        } else {
            // No ready actors, just advance normally
            self.advance_to_next_actor();
        }
    }

    /// Selects the lowest priority actor to prevent starvation.
    fn select_lowest_priority_actor(&mut self) {
        if self.actors.is_empty() {
            self.current_actor_index = 0;
            return;
        }

        let mut lowest_priority = u8::MAX;
        let mut lowest_priority_index = 0;
        let mut found_ready_actor = false;

        for (index, actor) in self.actors.iter().enumerate() {
            // Skip actors that are waiting for capabilities
            if actor.is_waiting {
                continue;
            }

            // Calculate effective priority (considering temporary boosts)
            let effective_priority = self.calculate_effective_priority(actor);

            if !found_ready_actor || effective_priority < lowest_priority {
                lowest_priority = effective_priority;
                lowest_priority_index = index;
                found_ready_actor = true;
            }
        }

        if found_ready_actor {
            self.current_actor_index = lowest_priority_index;
        } else {
            // No ready actors, just advance normally
            self.advance_to_next_actor();
        }
    }

    /// Calculates the effective priority of an actor, considering temporary boosts.
    fn calculate_effective_priority(&self, actor: &Actor) -> u8 {
        // Start with base priority
        let mut effective_priority = actor.priority;

        // Apply temporary priority boost if present
        if let Some(boost) = actor.priority_boost {
            // Cap the boosted priority at 255
            effective_priority = effective_priority.saturating_add(boost as u8);
            // effective_priority is already capped to 255 by saturating_add above
        }

        effective_priority
    }

    // V2 Priority Scheduling - Priority Management Methods

    /// Enables priority-based scheduling.
    pub fn enable_priority_scheduling(&mut self) {
        self.use_priority_scheduling = true;
    }

    /// Disables priority-based scheduling (uses round-robin).
    pub fn disable_priority_scheduling(&mut self) {
        self.use_priority_scheduling = false;
    }

    /// Sets the priority of an actor.
    pub fn set_priority(&mut self, actor_id: u32, priority: u8) -> Result<(), PhysicsError> {
        if let Some(actor) = self.actors.iter_mut().find(|a| a.id == actor_id) {
            actor.priority = priority;
            Ok(())
        } else {
            Err(PhysicsError::ActorNotFound(actor_id))
        }
    }

    /// Gets the priority of an actor.
    pub fn get_priority(&self, actor_id: u32) -> Result<u8, PhysicsError> {
        if let Some(actor) = self.actors.iter().find(|a| a.id == actor_id) {
            Ok(actor.priority)
        } else {
            Err(PhysicsError::ActorNotFound(actor_id))
        }
    }

    /// Adjusts the priority of an actor by a delta.
    pub fn adjust_priority(&mut self, actor_id: u32, delta: i8) -> Result<u8, PhysicsError> {
        if let Some(actor) = self.actors.iter_mut().find(|a| a.id == actor_id) {
            if delta > 0 {
                actor.priority = actor.priority.saturating_add(delta as u8);
            } else {
                actor.priority = actor.priority.saturating_sub((-delta) as u8);
            }
            Ok(actor.priority)
        } else {
            Err(PhysicsError::ActorNotFound(actor_id))
        }
    }

    /// Sets a temporary priority boost for an actor.
    pub fn set_priority_boost(&mut self, actor_id: u32, boost: u32) -> Result<(), PhysicsError> {
        if let Some(actor) = self.actors.iter_mut().find(|a| a.id == actor_id) {
            actor.priority_boost = Some(boost);
            Ok(())
        } else {
            Err(PhysicsError::ActorNotFound(actor_id))
        }
    }

    /// Clears the priority boost for an actor.
    pub fn clear_priority_boost(&mut self, actor_id: u32) -> Result<(), PhysicsError> {
        if let Some(actor) = self.actors.iter_mut().find(|a| a.id == actor_id) {
            actor.priority_boost = None;
            Ok(())
        } else {
            Err(PhysicsError::ActorNotFound(actor_id))
        }
    }

    /// Sets the starvation threshold for priority scheduling.
    pub fn set_starvation_threshold(&mut self, threshold: u64) {
        self.starvation_threshold = threshold;
    }

    /// Gets the current starvation threshold.
    pub fn get_starvation_threshold(&self) -> u64 {
        self.starvation_threshold
    }
}
