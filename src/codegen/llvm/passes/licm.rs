//! Loop Invariant Code Motion pass

use crate::error::{CursedError, Result};

/// LICM pass
pub struct LicmPass;

impl LicmPass {
    pub fn new() -> Self {
        Self
    }
}
