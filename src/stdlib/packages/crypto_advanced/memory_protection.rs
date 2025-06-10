/// fr fr Memory protection stub
use super::errors::*;

#[derive(Debug, Clone)]
pub struct SecureMemory(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct ZeroOnDrop(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct ProtectedBytes(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct MemoryBarrier;

pub fn clear_sensitive_data(data: &mut [u8]) {
    data.fill(0);
}

pub fn memory_lock(data: &[u8]) -> Result<(), String> {
    Ok(())
}

pub fn memory_unlock(data: &[u8]) -> Result<(), String> {
    Ok(())
}
