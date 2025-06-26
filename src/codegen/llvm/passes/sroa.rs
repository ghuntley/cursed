//! Scalar Replacement of Aggregates pass

use crate::error::{CursedError, Result};

/// SROA pass
pub struct SroaPass;

impl SroaPass {
    pub fn new() -> Self {
        Self
    }
}
