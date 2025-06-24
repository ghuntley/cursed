use crate::error::Error;
/// Context support for signal_boost - re-export from exec_vibez
/// This provides a unified context system across both modules

pub use crate::stdlib::exec_vibez::context::{
    VibeContext, CancelFunc, ContextError,
    Background, TODO, WithTimeout, WithDeadline, WithCancel, WithValue
};
