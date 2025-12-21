pub mod arithmetic;
pub mod basic;
/// Modular opcode handlers for the Physics World VM
/// Each opcode gets its own file for better organization and LLM-friendly editing
pub mod call;
pub mod capability;
pub mod closure;
pub mod comparison;
pub mod jump;
pub mod make_closure;
pub mod ret;
pub mod stack_ops;
