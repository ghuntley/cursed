/// Random byte generation with various encoding and utility functions
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use super::secure_random::SecureRandom;

/// Random byte generator with encoding and utility functions
pub struct RandomBytes {
    secure_rng: SecureRandom,
}

impl RandomBytes {
    /// Create new random byte generator
    pub fn new() -> AdvancedCryptoResult<Self> {
        Ok(Self {
            secure_rng: SecureRandom::new()?,
        })
    }
    
    /// Generate random bytes
    pub fn generate(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        self.secure_rng.bytes(size)
    }
    
    /// Generate random bytes as hexadecimal string
    pub fn hex(&self, size: usize) -> AdvancedCryptoResult<String> {
        let bytes = self.generate(size)?;
        Ok(bytes.iter().map(|b| format!("{:02x}", b)).collect())
    }
    
    /// Generate random bytes as uppercase hexadecimal string
    pub fn hex_upper(&self, size: usize) -> AdvancedCryptoResult<String> {
        let bytes = self.generate(size)?;
        Ok(bytes.iter().map(|b| format!("{:02X}", b)).collect())
    }
    
    /// Generate random bytes as base64 string
    pub fn base64(&self, size: usize) -> AdvancedCryptoResult<String> {
        let bytes = self.generate(size)?;
        Ok(self.base64_encode(&bytes))
    }
    
    /// Generate random bytes as base64url string (URL-safe base64)
    pub fn base64url(&self, size: usize) -> AdvancedCryptoResult<String> {
        let bytes = self.generate(size)?;
        Ok(self.base64url_encode(&bytes))
    }
    
    /// Generate random bytes as base32 string
    pub fn base32(&self, size: usize) -> AdvancedCryptoResult<String> {
        let bytes = self.generate(size)?;
        Ok(self.base32_encode(&bytes))
    }
    
    /// Generate random bytes as binary string (0s and 1s)
    pub fn binary(&self, size: usize) -> AdvancedCryptoResult<String> {
        let bytes = self.generate(size)?;
        Ok(bytes.iter()
            .map(|b| format!("{:08b}", b))
            .collect::<Vec<_>>()
            .join(""))
    }
    
    /// Generate random bytes with specific entropy per byte
    pub fn high_entropy(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        // Generate extra bytes and filter for high entropy
        let mut result = Vec::new();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = size * 10;
        
        while result.len() < size && attempts < MAX_ATTEMPTS {
            let candidate_bytes = self.generate(size * 2)?;
            
            for byte in candidate_bytes {
                if result.len() >= size {
                    break;
                }
                
                // Simple entropy check: avoid bytes with too many repeated bits
                if self.has_good_entropy(byte) {
                    result.push(byte);
                }
            }
            
            attempts += 1;
        }
        
        // Fill remaining with regular random bytes if needed
        while result.len() < size {
            result.extend(self.generate(size - result.len())?);
        }
        
        result.truncate(size);
        Ok(result)
    }
    
    /// Check if byte has good entropy (simple heuristic)
    fn has_good_entropy(&self, byte: u8) -> bool {
        // Count bits
        let ones = byte.count_ones();
        let zeros = 8 - ones;
        
        // Good entropy if not too many 0s or 1s
        ones >= 2 && zeros >= 2
    }
    
    /// Generate random bytes ensuring no repeating patterns
    pub fn no_patterns(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        if size == 0 {
            return Ok(Vec::new());
        }
        
        let mut result = Vec::with_capacity(size);
        let mut last_byte = None;
        let mut repeat_count = 0;
        
        while result.len() < size {
            let byte = self.secure_rng.u8()?;
            
            if let Some(last) = last_byte {
                if byte == last {
                    repeat_count += 1;
                    if repeat_count >= 3 {
                        // Skip this byte to avoid too many repeats
                        continue;
                    }
                } else {
                    repeat_count = 0;
                }
            }
            
            result.push(byte);
            last_byte = Some(byte);
        }
        
        Ok(result)
    }
    
    /// Generate random bytes with balanced bit distribution
    pub fn balanced_bits(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        let mut result = Vec::new();
        
        while result.len() < size {
            let byte = self.secure_rng.u8()?;
            
            // Check if byte has reasonably balanced bits (2-6 ones)
            let ones = byte.count_ones();
            if ones >= 2 && ones <= 6 {
                result.push(byte);
            }
        }
        
        Ok(result)
    }
    
    /// Generate cryptographic salt
    pub fn salt(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        // For cryptographic salt, we want high-quality random bytes
        self.high_entropy(size)
    }
    
    /// Generate random initialization vector (IV)
    pub fn iv(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        // IV should be unpredictable and unique
        self.generate(size)
    }
    
    /// Generate random nonce
    pub fn nonce(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        // Nonce should be unique (time-based component helps)
        let mut nonce = self.generate(size)?;
        
        if size >= 8 {
            // Add timestamp to ensure uniqueness
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64;
            
            let timestamp_bytes = timestamp.to_le_bytes();
            for (i, &byte) in timestamp_bytes.iter().enumerate().take(size) {
                nonce[i] ^= byte; // XOR with existing random data
            }
        }
        
        Ok(nonce)
    }
    
    /// Generate random key material
    pub fn key_material(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        // Key material needs highest quality randomness
        self.high_entropy(size)
    }
    
    /// Generate random padding
    pub fn padding(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        // Padding can be regular random bytes
        self.generate(size)
    }
    
    /// Generate random bytes and fill existing buffer
    pub fn fill(&self, buffer: &mut [u8]) -> AdvancedCryptoResult<()> {
        let bytes = self.generate(buffer.len())?;
        buffer.copy_from_slice(&bytes);
        Ok(())
    }
    
    /// Generate random bytes with XOR mask applied
    pub fn masked(&self, size: usize, mask: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let mut bytes = self.generate(size)?;
        
        for (i, byte) in bytes.iter_mut().enumerate() {
            if !mask.is_empty() {
                *byte ^= mask[i % mask.len()];
            }
        }
        
        Ok(bytes)
    }
    
    /// Generate random bytes ensuring they don't match a blacklist
    pub fn filtered(&self, size: usize, blacklist: &[Vec<u8>]) -> AdvancedCryptoResult<Vec<u8>> {
        const MAX_ATTEMPTS: usize = 1000;
        
        for _ in 0..MAX_ATTEMPTS {
            let bytes = self.generate(size)?;
            
            // Check if bytes match any blacklisted pattern
            let mut matches_blacklist = false;
            for banned in blacklist {
                if banned.len() <= bytes.len() {
                    for window in bytes.windows(banned.len()) {
                        if window == banned.as_slice() {
                            matches_blacklist = true;
                            break;
                        }
                    }
                }
                if matches_blacklist {
                    break;
                }
            }
            
            if !matches_blacklist {
                return Ok(bytes);
            }
        }
        
        Err("Failed to generate bytes not matching blacklist after maximum attempts".into())
    }
    
    /// Base64 encoding implementation
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
    
    /// Base64URL encoding implementation (URL-safe base64)
    fn base64url_encode(&self, data: &[u8]) -> String {
        let base64 = self.base64_encode(data);
        base64.replace('+', "-")
              .replace('/', "_")
              .trim_end_matches('=')
              .to_string()
    }
    
    /// Base32 encoding implementation
    fn base32_encode(&self, data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut result = String::new();
        
        for chunk in data.chunks(5) {
            let mut buf = [0u8; 5];
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i] = byte;
            }
            
            let b = ((buf[0] as u64) << 32) | ((buf[1] as u64) << 24) | ((buf[2] as u64) << 16) | ((buf[3] as u64) << 8) | (buf[4] as u64);
            
            for i in 0..8 {
                if i * 5 < chunk.len() * 8 {
                    result.push(CHARS[((b >> (35 - i * 5)) & 31) as usize] as char);
                } else {
                    result.push('=');
                }
            }
        }
        
        result
    }
    
    /// Generate random bytes for specific cryptographic purposes
    pub fn for_encryption_key(&self, key_size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        self.key_material(key_size)
    }
    
    pub fn for_hmac_key(&self, key_size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        self.key_material(key_size)
    }
    
    pub fn for_signature_key(&self, key_size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        self.high_entropy(key_size)
    }
    
    pub fn for_session_id(&self, size: usize) -> AdvancedCryptoResult<String> {
        self.base64url(size)
    }
    
    pub fn for_csrf_token(&self, size: usize) -> AdvancedCryptoResult<String> {
        self.hex(size)
    }
    
    pub fn for_api_key(&self, size: usize) -> AdvancedCryptoResult<String> {
        self.base64url(size)
    }
    
    /// Statistics about generated bytes
    pub fn analyze_bytes(&self, data: &[u8]) -> ByteAnalysis {
        let mut freq = [0u32; 256];
        let mut ones = 0;
        let mut zeros = 0;
        
        for &byte in data {
            freq[byte as usize] += 1;
            ones += byte.count_ones();
            zeros += byte.count_zeros();
        }
        
        // Calculate entropy
        let len = data.len() as f64;
        let mut entropy = 0.0;
        for &count in &freq {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }
        
        // Find most common byte
        let most_common_count = freq.iter().max().unwrap_or(&0);
        let most_common_byte = freq.iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .map(|(byte, _)| byte as u8)
            .unwrap_or(0);
        
        ByteAnalysis {
            length: data.len(),
            entropy,
            ones_count: ones,
            zeros_count: zeros,
            bit_balance: ones as f64 / (ones + zeros) as f64,
            most_common_byte,
            most_common_count: *most_common_count,
            unique_bytes: freq.iter().filter(|&&count| count > 0).count(),
        }
    }
}

/// Analysis results for byte data
#[derive(Debug, Clone)]
pub struct ByteAnalysis {
    pub length: usize,
    pub entropy: f64,
    pub ones_count: u32,
    pub zeros_count: u32,
    pub bit_balance: f64,
    pub most_common_byte: u8,
    pub most_common_count: u32,
    pub unique_bytes: usize,
}

impl Default for RandomBytes {
    fn default() -> Self {
        Self::new().expect("Failed to create default RandomBytes")
    }
}

/// Global functions for convenient access to random byte generation
pub fn random_bytes(size: usize) -> AdvancedCryptoResult<Vec<u8>> {
    RandomBytes::new()?.generate(size)
}

pub fn random_hex(size: usize) -> AdvancedCryptoResult<String> {
    RandomBytes::new()?.hex(size)
}

pub fn random_base64(size: usize) -> AdvancedCryptoResult<String> {
    RandomBytes::new()?.base64(size)
}

pub fn random_base64url(size: usize) -> AdvancedCryptoResult<String> {
    RandomBytes::new()?.base64url(size)
}

pub fn random_salt(size: usize) -> AdvancedCryptoResult<Vec<u8>> {
    RandomBytes::new()?.salt(size)
}

pub fn random_iv(size: usize) -> AdvancedCryptoResult<Vec<u8>> {
    RandomBytes::new()?.iv(size)
}

pub fn random_nonce(size: usize) -> AdvancedCryptoResult<Vec<u8>> {
    RandomBytes::new()?.nonce(size)
}

pub fn random_key_material(size: usize) -> AdvancedCryptoResult<Vec<u8>> {
    RandomBytes::new()?.key_material(size)
}
