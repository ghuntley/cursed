//! Sprintf module stub for vibez

use crate::error::CursedError;

// Stub implementations - TODO: implement properly
pub fn validate_format_string(_format: &str) -> Result<bool, CursedError> {
    Ok(true)
}

pub fn count_format_specifiers(_format: &str) -> usize {
    0
}
