use crate::error::{Result, CursedError};


pub struct MessageDigestManager {
    pub digest_mode: DigestMode,
}

#[derive(Debug, Clone)]
pub enum DigestMode {
    Sha256,
    Sha512,
    Blake3,
}

impl MessageDigestManager {
    pub fn new(mode: DigestMode) -> Self {
        Self { digest_mode: mode }
    }
}
