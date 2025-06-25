/// Nonce generation for cryptographic operations with collision resistance
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashSet;
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use super::secure_random::SecureRandom;

/// Nonce generation strategy
#[derive(Debug, Clone, PartialEq)]
pub enum NonceStrategy {
    Random,         // Pure random nonces
    Counter,        // Sequential counter
    Timestamp,      // Timestamp-based
    Hybrid,         // Combination of timestamp + random + counter
    Uuid,           // UUID-based nonces
    Custom,         // Custom generation logic
}

/// Nonce format
#[derive(Debug, Clone, PartialEq)]
pub enum NonceFormat {
    Binary,         // Raw binary data
    Hex,           // Hexadecimal string
    Base64,        // Base64 encoded
    Base64Url,     // Base64URL encoded
    Decimal,       // Decimal string (for numeric nonces)
}

/// Nonce configuration
#[derive(Debug, Clone)]
pub struct NonceConfig {
    pub strategy: NonceStrategy,
    pub format: NonceFormat,
    pub size_bytes: usize,
    pub check_duplicates: bool,
    pub max_history: usize,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl Default for NonceConfig {
    fn default() -> Self {
        Self {
            strategy: NonceStrategy::Hybrid,
            format: NonceFormat::Base64Url,
            size_bytes: 16,
            check_duplicates: true,
            max_history: 10000,
            prefix: None,
            suffix: None,
        }
    }
}

/// Nonce generator with collision detection
pub struct NonceGenerator {
    config: NonceConfig,
    secure_rng: SecureRandom,
    counter: AtomicU64,
    history: Arc<Mutex<HashSet<Vec<u8>>>>,
    start_time: SystemTime,
}

impl NonceGenerator {
    /// Create new nonce generator with default configuration
    pub fn new() -> AdvancedCryptoResult<Self> {
        Self::with_config(NonceConfig::default())
    }
    
    /// Create nonce generator with custom configuration
    pub fn with_config(config: NonceConfig) -> AdvancedCryptoResult<Self> {
        Ok(Self {
            config,
            secure_rng: SecureRandom::new()?,
            counter: AtomicU64::new(0),
            history: Arc::new(Mutex::new(HashSet::new())),
            start_time: SystemTime::now(),
        })
    }
    
    /// Generate a nonce according to the configured strategy
    pub fn generate(&self) -> AdvancedCryptoResult<String> {
        let nonce_bytes = match self.config.strategy {
            NonceStrategy::Random => self.generate_random()?,
            NonceStrategy::Counter => self.generate_counter()?,
            NonceStrategy::Timestamp => self.generate_timestamp()?,
            NonceStrategy::Hybrid => self.generate_hybrid()?,
            NonceStrategy::Uuid => self.generate_uuid()?,
            NonceStrategy::Custom => self.generate_custom()?,
        };
        
        // Check for duplicates if enabled
        if self.config.check_duplicates {
            let mut history = self.history.lock().unwrap();
            
            if history.contains(&nonce_bytes) {
                // Collision detected, try again with additional randomness
                drop(history);
                return self.generate_with_collision_resolution();
            }
            
            // Add to history
            history.insert(nonce_bytes.clone());
            
            // Limit history size
            if history.len() > self.config.max_history {
                // Remove oldest entries (this is a simplification)
                let excess = history.len() - self.config.max_history;
                let to_remove: Vec<_> = history.iter().take(excess).cloned().collect();
                for item in to_remove {
                    history.remove(&item);
                }
            }
        }
        
        // Format the nonce
        let formatted = self.format_nonce(&nonce_bytes)?;
        
        // Add prefix and suffix if configured
        let mut result = String::new();
        if let Some(ref prefix) = self.config.prefix {
            result.push_str(prefix);
        }
        result.push_str(&formatted);
        if let Some(ref suffix) = self.config.suffix {
            result.push_str(suffix);
        }
        
        Ok(result)
    }
    
    /// Generate pure random nonce
    fn generate_random(&self) -> AdvancedCryptoResult<Vec<u8>> {
        self.secure_rng.bytes(self.config.size_bytes)
    }
    
    /// Generate counter-based nonce
    fn generate_counter(&self) -> AdvancedCryptoResult<Vec<u8>> {
        let counter = self.counter.fetch_add(1, Ordering::SeqCst);
        
        if self.config.size_bytes >= 8 {
            // Use full counter value
            let mut nonce = counter.to_be_bytes().to_vec();
            
            // Pad with random bytes if needed
            if self.config.size_bytes > 8 {
                let random_bytes = self.secure_rng.bytes(self.config.size_bytes - 8)?;
                nonce.extend(random_bytes);
            }
            
            nonce.truncate(self.config.size_bytes);
            Ok(nonce)
        } else {
            // Use truncated counter
            let bytes = counter.to_be_bytes();
            Ok(bytes[8 - self.config.size_bytes..].to_vec())
        }
    }
    
    /// Generate timestamp-based nonce
    fn generate_timestamp(&self) -> AdvancedCryptoResult<Vec<u8>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        if self.config.size_bytes >= 8 {
            let mut nonce = timestamp.to_be_bytes().to_vec();
            
            // Add random bytes for remaining space
            if self.config.size_bytes > 8 {
                let random_bytes = self.secure_rng.bytes(self.config.size_bytes - 8)?;
                nonce.extend(random_bytes);
            }
            
            nonce.truncate(self.config.size_bytes);
            Ok(nonce)
        } else {
            // Use truncated timestamp
            let bytes = timestamp.to_be_bytes();
            Ok(bytes[8 - self.config.size_bytes..].to_vec())
        }
    }
    
    /// Generate hybrid nonce (timestamp + counter + random)
    fn generate_hybrid(&self) -> AdvancedCryptoResult<Vec<u8>> {
        let mut nonce = Vec::with_capacity(self.config.size_bytes);
        
        // Calculate space allocation
        let timestamp_bytes = (self.config.size_bytes / 3).max(1);
        let counter_bytes = (self.config.size_bytes / 3).max(1);
        let random_bytes = self.config.size_bytes - timestamp_bytes - counter_bytes;
        
        // Timestamp component
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        let ts_bytes = timestamp.to_be_bytes();
        let ts_start = 8 - timestamp_bytes.min(8);
        nonce.extend_from_slice(&ts_bytes[ts_start..]);
        
        // Counter component
        let counter = self.counter.fetch_add(1, Ordering::SeqCst);
        let counter_bytes_data = counter.to_be_bytes();
        let counter_start = 8 - counter_bytes.min(8);
        nonce.extend_from_slice(&counter_bytes_data[counter_start..]);
        
        // Random component
        if random_bytes > 0 {
            let random_data = self.secure_rng.bytes(random_bytes)?;
            nonce.extend(random_data);
        }
        
        // Ensure exact size
        nonce.truncate(self.config.size_bytes);
        while nonce.len() < self.config.size_bytes {
            nonce.push(0);
        }
        
        Ok(nonce)
    }
    
    /// Generate UUID-based nonce
    fn generate_uuid(&self) -> AdvancedCryptoResult<Vec<u8>> {
        // Generate UUID v4 bytes
        let mut uuid_bytes = [0u8; 16];
        self.secure_rng.fill_bytes(&mut uuid_bytes)?;
        
        // Set version (4) and variant bits
        uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x40; // Version 4
        uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80; // Variant 10
        
        if self.config.size_bytes <= 16 {
            Ok(uuid_bytes[..self.config.size_bytes].to_vec())
        } else {
            // Extend with random bytes
            let mut nonce = uuid_bytes.to_vec();
            let additional = self.secure_rng.bytes(self.config.size_bytes - 16)?;
            nonce.extend(additional);
            Ok(nonce)
        }
    }
    
    /// Generate custom nonce (extendable for specific needs)
    fn generate_custom(&self) -> AdvancedCryptoResult<Vec<u8>> {
        // Default implementation is hybrid strategy
        self.generate_hybrid()
    }
    
    /// Generate nonce with collision resolution
    fn generate_with_collision_resolution(&self) -> AdvancedCryptoResult<String> {
        const MAX_RETRIES: usize = 10;
        
        for retry in 0..MAX_RETRIES {
            // Add extra randomness to avoid collision
            let extra_random = self.secure_rng.bytes(4)?;
            
            let mut nonce_bytes = match self.config.strategy {
                NonceStrategy::Random => self.generate_random()?,
                NonceStrategy::Counter => self.generate_counter()?,
                NonceStrategy::Timestamp => self.generate_timestamp()?,
                NonceStrategy::Hybrid => self.generate_hybrid()?,
                NonceStrategy::Uuid => self.generate_uuid()?,
                NonceStrategy::Custom => self.generate_custom()?,
            };
            
            // XOR with extra randomness
            for (i, &random_byte) in extra_random.iter().enumerate() {
                if i < nonce_bytes.len() {
                    nonce_bytes[i] ^= random_byte;
                }
            }
            
            // Add retry counter
            let retry_bytes = (retry as u32).to_le_bytes();
            for (i, &retry_byte) in retry_bytes.iter().enumerate() {
                if i < nonce_bytes.len() {
                    nonce_bytes[i] ^= retry_byte;
                }
            }
            
            // Check for collision again
            {
                let history = self.history.lock().unwrap();
                if !history.contains(&nonce_bytes) {
                    drop(history);
                    
                    // Add to history
                    let mut history = self.history.lock().unwrap();
                    history.insert(nonce_bytes.clone());
                    
                    // Format and return
                    let formatted = self.format_nonce(&nonce_bytes)?;
                    let mut result = String::new();
                    if let Some(ref prefix) = self.config.prefix {
                        result.push_str(prefix);
                    }
                    result.push_str(&formatted);
                    if let Some(ref suffix) = self.config.suffix {
                        result.push_str(suffix);
                    }
                    
                    return Ok(result);
                }
            }
        }
        
        Err("Failed to generate unique nonce after maximum retries".into())
    }
    
    /// Format nonce bytes according to configured format
    fn format_nonce(&self, bytes: &[u8]) -> AdvancedCryptoResult<String> {
        match self.config.format {
            NonceFormat::Binary => {
                // Return as-is (but we need a string representation)
                Ok(bytes.iter().map(|b| format!("{:02x}", b)).collect())
            }
            NonceFormat::Hex => {
                Ok(bytes.iter().map(|b| format!("{:02x}", b)).collect())
            }
            NonceFormat::Base64 => {
                Ok(self.base64_encode(bytes))
            }
            NonceFormat::Base64Url => {
                Ok(self.base64url_encode(bytes))
            }
            NonceFormat::Decimal => {
                // Convert to decimal (for shorter byte arrays)
                if bytes.len() <= 8 {
                    let mut value = 0u64;
                    for &byte in bytes {
                        value = value * 256 + byte as u64;
                    }
                    Ok(value.to_string())
                } else {
                    // For longer arrays, concatenate bytes as decimal
                    Ok(bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(""))
                }
            }
        }
    }
    
    /// Base64 encoding
    fn base64_encode(&self, data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        
        for chunk in data.chunks(3) {
            let mut buf = [0u8; 3];
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i] = byte;
            }
            
            let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
            
            result.push(CHARS[((b >> 18) & 63) as usize] as char);
            result.push(CHARS[((b >> 12) & 63) as usize] as char);
            result.push(if chunk.len() > 1 { CHARS[((b >> 6) & 63) as usize] as char } else { '=' });
            result.push(if chunk.len() > 2 { CHARS[(b & 63) as usize] as char } else { '=' });
        }
        
        result
    }
    
    /// Base64URL encoding
    fn base64url_encode(&self, data: &[u8]) -> String {
        let base64 = self.base64_encode(data);
        base64.replace('+', "-")
              .replace('/', "_")
              .trim_end_matches('=')
              .to_string()
    }
    
    /// Generate batch of nonces
    pub fn batch(&self, count: usize) -> AdvancedCryptoResult<Vec<String>> {
        let mut nonces = Vec::with_capacity(count);
        for _ in 0..count {
            nonces.push(self.generate()?);
        }
        Ok(nonces)
    }
    
    /// Generate nonce for specific cryptographic use
    pub fn for_encryption(&self) -> AdvancedCryptoResult<String> {
        // For encryption, we typically want larger, random nonces
        let mut temp_config = self.config.clone();
        temp_config.strategy = NonceStrategy::Random;
        temp_config.size_bytes = temp_config.size_bytes.max(12); // At least 96 bits
        
        let temp_generator = Self::with_config(temp_config)?;
        temp_generator.generate()
    }
    
    pub fn for_signature(&self) -> AdvancedCryptoResult<String> {
        // For signatures, timestamp-based nonces can be useful
        let mut temp_config = self.config.clone();
        temp_config.strategy = NonceStrategy::Hybrid;
        
        let temp_generator = Self::with_config(temp_config)?;
        temp_generator.generate()
    }
    
    pub fn for_challenge_response(&self) -> AdvancedCryptoResult<String> {
        // For challenge-response, pure random is preferred
        let mut temp_config = self.config.clone();
        temp_config.strategy = NonceStrategy::Random;
        temp_config.size_bytes = 32; // 256 bits
        
        let temp_generator = Self::with_config(temp_config)?;
        temp_generator.generate()
    }
    
    pub fn for_session(&self) -> AdvancedCryptoResult<String> {
        // For sessions, hybrid with UUID format works well
        let mut temp_config = self.config.clone();
        temp_config.strategy = NonceStrategy::Uuid;
        temp_config.format = NonceFormat::Base64Url;
        
        let temp_generator = Self::with_config(temp_config)?;
        temp_generator.generate()
    }
    
    /// Get statistics about nonce generation
    pub fn get_stats(&self) -> NonceStats {
        let history = self.history.lock().unwrap();
        let uptime = self.start_time.elapsed().unwrap_or_default();
        
        NonceStats {
            total_generated: self.counter.load(Ordering::SeqCst),
            unique_nonces: history.len(),
            uptime_seconds: uptime.as_secs(),
            strategy: self.config.strategy.clone(),
            format: self.config.format.clone(),
            size_bytes: self.config.size_bytes,
        }
    }
    
    /// Clear nonce history
    pub fn clear_history(&self) {
        let mut history = self.history.lock().unwrap();
        history.clear();
    }
    
    /// Check if nonce exists in history
    pub fn is_duplicate(&self, nonce: &str) -> AdvancedCryptoResult<bool> {
        // Convert nonce back to bytes for comparison
        let nonce_bytes = match self.config.format {
            NonceFormat::Hex => {
                if nonce.len() % 2 != 0 {
                    return Err("Invalid hex nonce length".into());
                }
                
                let mut bytes = Vec::new();
                for chunk in nonce.as_bytes().chunks(2) {
                    let hex_str = std::str::from_utf8(chunk)?;
                    let byte = u8::from_str_radix(hex_str, 16)
                        .map_err(|_| "Invalid hex character")?;
                    bytes.push(byte);
                }
                bytes
            }
            _ => {
                // For other formats, this is more complex
                // For now, just convert string to bytes
                nonce.as_bytes().to_vec()
            }
        };
        
        let history = self.history.lock().unwrap();
        Ok(history.contains(&nonce_bytes))
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: NonceConfig) {
        self.config = config;
    }
    
    /// Get current configuration
    pub fn get_config(&self) -> &NonceConfig {
        &self.config
    }
}

/// Nonce generation statistics
#[derive(Debug, Clone)]
pub struct NonceStats {
    pub total_generated: u64,
    pub unique_nonces: usize,
    pub uptime_seconds: u64,
    pub strategy: NonceStrategy,
    pub format: NonceFormat,
    pub size_bytes: usize,
}

impl Default for NonceGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default NonceGenerator")
    }
}

/// Global functions for convenient nonce generation
pub fn generate_nonce() -> AdvancedCryptoResult<String> {
    NonceGenerator::new()?.generate()
}

pub fn generate_nonce_with_strategy(strategy: NonceStrategy) -> AdvancedCryptoResult<String> {
    let config = NonceConfig {
        strategy,
        ..Default::default()
    };
    NonceGenerator::with_config(config)?.generate()
}

pub fn generate_random_nonce(size_bytes: usize) -> AdvancedCryptoResult<String> {
    let config = NonceConfig {
        strategy: NonceStrategy::Random,
        size_bytes,
        ..Default::default()
    };
    NonceGenerator::with_config(config)?.generate()
}

pub fn generate_timestamp_nonce() -> AdvancedCryptoResult<String> {
    let config = NonceConfig {
        strategy: NonceStrategy::Timestamp,
        ..Default::default()
    };
    NonceGenerator::with_config(config)?.generate()
}

pub fn generate_uuid_nonce() -> AdvancedCryptoResult<String> {
    let config = NonceConfig {
        strategy: NonceStrategy::Uuid,
        format: NonceFormat::Base64Url,
        ..Default::default()
    };
    NonceGenerator::with_config(config)?.generate()
}

pub fn generate_encryption_nonce() -> AdvancedCryptoResult<String> {
    NonceGenerator::new()?.for_encryption()
}

pub fn generate_session_nonce() -> AdvancedCryptoResult<String> {
    NonceGenerator::new()?.for_session()
}
