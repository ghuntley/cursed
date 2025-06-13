/// Forward Secrecy Implementation
use crate::error::CursedError;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::packages::crypto_random::SecureRandom;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Forward secrecy configuration
#[derive(Debug, Clone)]
pub struct ForwardSecrecyConfig {
    pub key_rotation_interval: Duration,
    pub max_messages_per_key: u64,
    pub secure_deletion: bool,
    pub key_derivation_function: String,
}

/// Forward secrecy manager
#[derive(Debug)]
pub struct ForwardSecrecyManager {
    ephemeral_keys: HashMap<String, Vec<u8>>,
    key_history: HashMap<String, SystemTime>,
    secure_random: SecureRandom,
    config: ForwardSecrecyConfig,
}

impl ForwardSecrecyManager {
    pub fn new() -> AdvancedCryptoResult<Self> {
        let config = ForwardSecrecyConfig {
            key_rotation_interval: Duration::from_secs(3600),
            max_messages_per_key: 1000,
            secure_deletion: true,
            key_derivation_function: "HKDF-SHA256".to_string(),
        };

        Ok(Self {
            ephemeral_keys: HashMap::new(),
            key_history: HashMap::new(),
            secure_random: SecureRandom::new()?,
            config,
        })
    }

    pub fn generate_ephemeral_key(&mut self, session_id: &str) -> AdvancedCryptoResult<Vec<u8>> {
        let key = self.secure_random.generate_bytes(32)?;
        self.ephemeral_keys.insert(session_id.to_string(), key.clone());
        self.key_history.insert(session_id.to_string(), SystemTime::now());
        Ok(key)
    }

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
            }
            
            self.ephemeral_keys.insert(session_id.clone(), new_key);
            self.key_history.insert(session_id, now);
            rotated += 1;
        }

        Ok(rotated)
    }

    pub fn ensure_forward_secrecy(&self, session_id: &str) -> AdvancedCryptoResult<bool> {
        if let Some(created_at) = self.key_history.get(session_id) {
            let elapsed = SystemTime::now().duration_since(*created_at).unwrap_or(Duration::from_secs(0));
            Ok(elapsed < self.config.key_rotation_interval)
        } else {
            Ok(false)
        }
    }
}

impl Default for ForwardSecrecyManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default ForwardSecrecyManager")
    }
}
