/// Capability types for Physics World
use serde::{Deserialize, Serialize};
use std::fmt;

/// Capability enum for the capability system
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum Capability {
    // Meta-capabilities
    MetaSelfModify, // Can modify own non-core code
    MetaGrant,      // Can grant capabilities to others (dangerous)

    // Macro & Compile-time capabilities
    MacroHygienic, // Can expand hygienic macros
    MacroUnsafe,   // Can generate arbitrary syntax
    ComptimeEval,  // Can execute code at compile-time

    // I/O & External World
    IoReadSensor,    // Read from virtual sensors
    IoWriteActuator, // Write to virtual actuators
    IoNetwork,       // Network access
    IoPersist,       // Write to persistent storage

    // System
    SysCreateActor,    // Can spawn new actors
    SysTerminateActor, // Can terminate actors (including self)
    SysClock,          // Access non-deterministic time

    // Resource privileges
    ResourceExtraMemory(u64), // Additional memory quota
    ResourceExtraTime(u64),   // Additional time quota
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Capability::MetaSelfModify => write!(f, "MetaSelfModify"),
            Capability::MetaGrant => write!(f, "MetaGrant"),
            Capability::MacroHygienic => write!(f, "MacroHygienic"),
            Capability::MacroUnsafe => write!(f, "MacroUnsafe"),
            Capability::ComptimeEval => write!(f, "ComptimeEval"),
            Capability::IoReadSensor => write!(f, "IoReadSensor"),
            Capability::IoWriteActuator => write!(f, "IoWriteActuator"),
            Capability::IoNetwork => write!(f, "IoNetwork"),
            Capability::IoPersist => write!(f, "IoPersist"),
            Capability::SysCreateActor => write!(f, "SysCreateActor"),
            Capability::SysTerminateActor => write!(f, "SysTerminateActor"),
            Capability::SysClock => write!(f, "SysClock"),
            Capability::ResourceExtraMemory(bytes) => write!(f, "ResourceExtraMemory({})", bytes),
            Capability::ResourceExtraTime(ms) => write!(f, "ResourceExtraTime({})", ms),
        }
    }
}
