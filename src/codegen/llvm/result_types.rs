// Minimal resulttypes module - disabled for minimal build
use crate::error::{Error, Result};

// Minimal placeholder implementations
pub struct ResulttypesDisabled {}

impl ResulttypesDisabled {
    pub fn new() -> Result<Self> {
        Err(Error::NotImplemented(
            "resulttypes is disabled in minimal build".to_string()
        ))
    }
}
