/// Forward Secrecy Implementation
use crate::error::CursedError;
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
// use crate::stdlib::packages::crypto_random::SecureRandom;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Forward secrecy configuration
#[derive(Debug, Clone)]
pub struct ForwardSecrecyConfig {
/// Forward secrecy manager
#[derive(Debug)]
pub struct ForwardSecrecyManager {
impl ForwardSecrecyManager {
    pub fn new() -> AdvancedCryptoResult<Self> {
        let config = ForwardSecrecyConfig {

        Ok(Self {
        })
    pub fn generate_ephemeral_key(&mut self, session_id: &str) -> AdvancedCryptoResult<Vec<u8>> {
        let key = self.secure_random.generate_bytes(32)?;
        self.ephemeral_keys.insert(session_id.to_string(), key.clone());
        self.key_history.insert(session_id.to_string(), SystemTime::now());
        Ok(key)
    pub fn rotate_keys(&mut self) -> AdvancedCryptoResult<usize> {
        let now = SystemTime::now();
        let mut rotated = 0;

        let sessions_to_rotate: Vec<String> = self.key_history.iter()
            .filter(|(_, &created_at)| {
                now.duration_since(created_at).unwrap_or(Duration::from_secs(0)) > self.config.key_rotation_interval
            })
            .map(|(session_id, _)| session_id.clone())
            .collect();

        for session_id in sessions_to_rotate {
            let new_key = self.secure_random.generate_bytes(32)?;
            
            // Securely delete old key
            if let Some(old_key) = self.ephemeral_keys.get_mut(&session_id) {
                old_key.fill(0);
            self.ephemeral_keys.insert(session_id.clone(), new_key);
            self.key_history.insert(session_id, now);
            rotated += 1;
        Ok(rotated)
    pub fn ensure_forward_secrecy(&self, session_id: &str) -> AdvancedCryptoResult<bool> {
        if let Some(created_at) = self.key_history.get(session_id) {
            let elapsed = SystemTime::now().duration_since(*created_at).unwrap_or(Duration::from_secs(0));
            Ok(elapsed < self.config.key_rotation_interval)
        } else {
            Ok(false)
        }
    }
impl Default for ForwardSecrecyManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default ForwardSecrecyManager")
    }
}
