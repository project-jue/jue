/// Host function types for Physics World
use serde::{Deserialize, Serialize};

/// Host function enum for FFI operations
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum HostFunction {
    // System operations
    ReadSensor = 0,
    WriteActuator = 1,
    GetWallClockNs = 2,
    SpawnActor = 3,
    TerminateActor = 4,
    NetworkSend = 5,
    NetworkReceive = 6,
    PersistWrite = 7,
    PersistRead = 8,

    // Arithmetic operations
    IntAdd = 9,
    IntSub = 10,
    IntMul = 11,
    IntDiv = 12,
    IntMod = 13,

    FloatAdd = 14,
    FloatSub = 15,
    FloatMul = 16,
    FloatDiv = 17,

    // Type conversions
    IntToFloat = 18,
    FloatToInt = 19,

    // Comparison operations
    IntEq = 20,
    IntLt = 21,
    IntGt = 22,
    FloatEq = 23,
    FloatLt = 24,
    FloatGt = 25,
}
