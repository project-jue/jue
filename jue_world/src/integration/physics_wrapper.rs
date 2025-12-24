pub mod calls;

/// Physics-World integration for Jue-World V2.0 - Modular version
pub mod physics {
    pub use analysis::*;
    pub use calls::*;
    pub use capabilities::*;
    pub use compiler::*;
    pub use control_flow::*;
    pub use ffi::*;
    pub use lambdas::*;
    pub use lets::*;
    pub use literals::*;
    pub use symbols::*;
    pub use variables::*;
}

/// Re-export the main compilation function for backward compatibility
pub use physics::compile_to_physics_world;
