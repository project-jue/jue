// Re-export the PhysicsWorldCompiler from physics_compiler.rs
pub use crate::physics_integration::physics_compiler::PhysicsWorldCompiler;

// Note: The following modules are declared but their implementation files don't exist:
// pub mod analysis;
// pub mod calls;
// pub mod capabilities;
// pub mod compiler;
// pub mod control_flow;
// pub mod ffi;
// pub mod lambdas;
// pub mod lets;
// pub mod literals;
// pub mod symbols;
// pub mod variables;
//
// For now, we'll use the physics_compiler module directly
