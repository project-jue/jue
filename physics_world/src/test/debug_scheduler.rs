use super::*;
use crate::types::OpCode;

#[test]
fn debug_simple_yield() {
    let mut scheduler = PhysicsScheduler::new();

    // Create a simple actor that just yields
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![OpCode::Yield], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Execute the actor
    let result = scheduler.tick();
    println!("Debug: Result = {:?}", result);

    // Check what the VM state looks like after execution
    let actor = &scheduler.actors[0];
    println!(
        "Debug: VM IP = {}, Stack = {:?}",
        actor.vm.ip, actor.vm.stack
    );
    println!("Debug: VM Instructions = {:?}", actor.vm.instructions);
}
