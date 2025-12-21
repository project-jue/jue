/// Host function types for Physics World
use serde::{Deserialize, Serialize};

/// Host function enum for FFI operations
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostFunction {
    ReadSensor = 0,
    WriteActuator = 1,
    GetWallClockNs = 2,
    SpawnActor = 3,
    TerminateActor = 4,
    NetworkSend = 5,
    NetworkReceive = 6,
    PersistWrite = 7,
    PersistRead = 8,
}
