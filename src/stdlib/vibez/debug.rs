//! Debug module stub for vibez

use crate::error::CursedError;

// Stub implementations - TODO: implement properly
pub fn set_debug_level(_level: u8) -> Result<(), CursedError> {
    Ok(())
}

pub fn get_debug_level() -> u8 {
    0
}

pub fn is_debug_enabled() -> bool {
    false
}

pub fn init_debug_system() {
    // Stub implementation
}
