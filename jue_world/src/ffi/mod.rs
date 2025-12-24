/// FFI module for Jue-World V2.0
///
/// This module provides FFI functionality.

pub use crate::ffi_system::ffi_call_generator::FfiCallGenerator;

/// Create a standard FFI registry
pub fn create_standard_ffi_registry() -> crate::ffi_system::global_ffi_registry::FfiRegistry {
    let mut registry = crate::ffi_system::global_ffi_registry::FfiRegistry::new();
    // Add standard functions here if needed
    registry
}
