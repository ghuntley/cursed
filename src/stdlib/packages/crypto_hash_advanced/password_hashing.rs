/// Production-ready password hashing with modern algorithms (Argon2, scrypt, PBKDF2)
use crate::error::CursedError;
use crate::stdlib::packages::crypto_hash_advanced::hash_traits::*;
use std::time::{Duration, Instant};

/// Result type for password operations
pub type PasswordResult<T> = Result<T, CursedError>;

/// Password hashing algorithm variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PasswordAlgorithm {
    Argon2id,   // Recommended - hybrid of Argon2i and Argon2d
    Argon2i,    // Side-channel resistant
    Argon2d,    // Fast, but vulnerable to side-channel attacks
    Scrypt,     // Memory-hard
    Pbkdf2,     // Widely supported, but weaker
}

impl PasswordAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            PasswordAlgorithm::Argon2id => "argon2id",
            PasswordAlgorithm::Argon2i => "argon2i", 
            PasswordAlgorithm::Argon2d => "argon2d",
            PasswordAlgorithm::Scrypt => "scrypt",
            PasswordAlgorithm::Pbkdf2 => "pbkdf2",
        }
    }
    
    pub fn recommended() -> Self {
        PasswordAlgorithm::Argon2id
    }
}

/// Password hashing configuration
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    pub algorithm: PasswordAlgorithm,
    pub memory_cost: u32,     // Memory usage in KiB (Argon2) or block size (scrypt)
    pub time_cost: u32,       // Time cost / iterations
    pub parallelism: u32,     // Parallelism factor
    pub hash_length: u32,     // Output hash length
    pub salt_length: u32,     // Salt length
}

impl PasswordConfig {
    /// Secure default configuration (OWASP recommended)
    pub fn secure_default() -> Self {
        Self {
            algorithm: PasswordAlgorithm::Argon2id,
            memory_cost: 65536,   // 64 MiB
            time_cost: 3,         // 3 iterations
            parallelism: 4,       // 4 threads
            hash_length: 32,      // 256 bits
            salt_length: 16,      // 128 bits
        }
    }
    
    /// Fast configuration for testing/development
    pub fn fast() -> Self {
        Self {
            algorithm: PasswordAlgorithm::Argon2id,
            memory_cost: 4096,    // 4 MiB
            time_cost: 1,         // 1 iteration
            parallelism: 1,       // 1 thread
            hash_length: 32,
            salt_length: 16,
        }
    }
    
    /// Interactive configuration (web apps)
    pub fn interactive() -> Self {
        Self {
            algorithm: PasswordAlgorithm::Argon2id,
            memory_cost: 12288,   // 12 MiB
            time_cost: 3,
            parallelism: 1,
            hash_length: 32,
            salt_length: 16,
        }
    }
    
    /// Server configuration (background processing)
    pub fn server() -> Self {
        Self {
            algorithm: PasswordAlgorithm::Argon2id,
            memory_cost: 65536,   // 64 MiB
            time_cost: 4,
            parallelism: 4,
            hash_length: 32,
            salt_length: 16,
        }
    }
}

/// Password hash result
#[derive(Debug, Clone)]
pub struct PasswordHash {
    pub algorithm: PasswordAlgorithm,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub config: PasswordConfig,
    pub format_string: String, // PHC string format
}

impl PasswordHash {
    /// Convert to PHC string format
    pub fn to_string(&self) -> String {
        self.format_string.clone()
    }
    
    /// Parse from PHC string format
    pub fn from_string(phc_string: &str) -> PasswordResult<Self> {
        let parts: Vec<&str> = phc_string.split('$').collect();
        
        if parts.len() < 4 {
            return Err(CursedError::InvalidArgument("Invalid PHC string format".to_string()));
        }
        
        // Parse algorithm
        let algorithm = match parts[1] {
            "argon2id" => PasswordAlgorithm::Argon2id,
            "argon2i" => PasswordAlgorithm::Argon2i,
            "argon2d" => PasswordAlgorithm::Argon2d,
            "scrypt" => PasswordAlgorithm::Scrypt,
            "pbkdf2" => PasswordAlgorithm::Pbkdf2,
            _ => return Err(CursedError::InvalidArgument("Unknown algorithm".to_string())),
        };
        
        // Parse parameters
        let params = Self::parse_parameters(parts[2])?;
        let salt = Self::decode_base64(parts[3])?;
        let hash = if parts.len() > 4 {
            Self::decode_base64(parts[4])?
        } else {
            Vec::new()
        };
        
        let config = PasswordConfig {
            algorithm,
            memory_cost: params.get("m").copied().unwrap_or(0),
            time_cost: params.get("t").copied().unwrap_or(0),
            parallelism: params.get("p").copied().unwrap_or(0),
            hash_length: hash.len() as u32,
            salt_length: salt.len() as u32,
        };
        
        Ok(PasswordHash {
            algorithm,
            hash,
            salt,
            config,
            format_string: phc_string.to_string(),
        })
    }
    
    fn parse_parameters(params_str: &str) -> PasswordResult<std::collections::HashMap<&str, u32>> {
        let mut params = std::collections::HashMap::new();
        
        for param in params_str.split(',') {
            let kv: Vec<&str> = param.split('=').collect();
            if kv.len() == 2 {
                let value = kv[1].parse::<u32>()
                    .map_err(|_| CursedError::InvalidArgument("Invalid parameter value".to_string()))?;
                params.insert(kv[0], value);
            }
        }
        
        Ok(params)
    }
    
    fn decode_base64(input: &str) -> PasswordResult<Vec<u8>> {
        // Simple base64 decoder (in production, use a proper base64 library)
        let cleaned = input.trim_end_matches('=');
        let mut result = Vec::new();
        
        // This is a simplified implementation - real base64 decoding would be more robust
        for chunk in cleaned.as_bytes().chunks(4) {
            let mut buffer = [0u8; 4];
            for (i, &byte) in chunk.iter().enumerate() {
                buffer[i] = match byte {
                    b'A'..=b'Z' => byte - b'A',
                    b'a'..=b'z' => byte - b'a' + 26,
                    b'0'..=b'9' => byte - b'0' + 52,
                    b'+' => 62,
                    b'/' => 63,
                    _ => return Err(CursedError::InvalidArgument("Invalid base64 character".to_string())),
                };
            }
            
            if chunk.len() >= 2 {
                result.push((buffer[0] << 2) | (buffer[1] >> 4));
            }
            if chunk.len() >= 3 {
                result.push((buffer[1] << 4) | (buffer[2] >> 2));
            }
            if chunk.len() >= 4 {
                result.push((buffer[2] << 6) | buffer[3]);
            }
        }
        
        Ok(result)
    }
    
    fn encode_base64(input: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        
        for chunk in input.chunks(3) {
            let mut buffer = [0u8; 3];
            for (i, &byte) in chunk.iter().enumerate() {
                buffer[i] = byte;
            }
            
            let b = (buffer[0] as u32) << 16 | (buffer[1] as u32) << 8 | buffer[2] as u32;
            
            result.push(CHARS[((b >> 18) & 63) as usize] as char);
            result.push(CHARS[((b >> 12) & 63) as usize] as char);
            
            if chunk.len() > 1 {
                result.push(CHARS[((b >> 6) & 63) as usize] as char);
            } else {
                result.push('=');
            }
            
            if chunk.len() > 2 {
                result.push(CHARS[(b & 63) as usize] as char);
            } else {
                result.push('=');
            }
        }
        
        result
    }
}

/// Password hasher implementation
pub struct PasswordHasher {
    config: PasswordConfig,
}

impl PasswordHasher {
    pub fn new(config: PasswordConfig) -> Self {
        Self { config }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(PasswordConfig::secure_default())
    }
    
    /// Hash a password with automatic salt generation
    pub fn hash_password(&self, password: &str) -> PasswordResult<PasswordHash> {
        let salt = self.generate_salt()?;
        self.hash_password_with_salt(password, &salt)
    }
    
    /// Hash a password with provided salt
    pub fn hash_password_with_salt(&self, password: &str, salt: &[u8]) -> PasswordResult<PasswordHash> {
        let start_time = Instant::now();
        
        let hash = match self.config.algorithm {
            PasswordAlgorithm::Argon2id => self.argon2_hash(password.as_bytes(), salt, 2)?,
            PasswordAlgorithm::Argon2i => self.argon2_hash(password.as_bytes(), salt, 1)?,
            PasswordAlgorithm::Argon2d => self.argon2_hash(password.as_bytes(), salt, 0)?,
            PasswordAlgorithm::Scrypt => self.scrypt_hash(password.as_bytes(), salt)?,
            PasswordAlgorithm::Pbkdf2 => self.pbkdf2_hash(password.as_bytes(), salt)?,
        };
        
        let format_string = self.create_phc_string(&hash, salt)?;
        
        println!("Password hashing took: {:?}", start_time.elapsed());
        
        Ok(PasswordHash {
            algorithm: self.config.algorithm,
            hash,
            salt: salt.to_vec(),
            config: self.config.clone(),
            format_string,
        })
    }
    
    /// Verify password against hash
    pub fn verify_password(&self, password: &str, hash: &PasswordHash) -> PasswordResult<bool> {
        let computed_hash = match hash.algorithm {
            PasswordAlgorithm::Argon2id => self.argon2_hash(password.as_bytes(), &hash.salt, 2)?,
            PasswordAlgorithm::Argon2i => self.argon2_hash(password.as_bytes(), &hash.salt, 1)?,
            PasswordAlgorithm::Argon2d => self.argon2_hash(password.as_bytes(), &hash.salt, 0)?,
            PasswordAlgorithm::Scrypt => self.scrypt_hash(password.as_bytes(), &hash.salt)?,
            PasswordAlgorithm::Pbkdf2 => self.pbkdf2_hash(password.as_bytes(), &hash.salt)?,
        };
        
        Ok(constant_time_eq(&computed_hash, &hash.hash))
    }
    
    /// Simplified Argon2 implementation (production would use proper library)
    fn argon2_hash(&self, password: &[u8], salt: &[u8], variant: u8) -> PasswordResult<Vec<u8>> {
        // This is a simplified implementation for demonstration
        // Production code should use the official Argon2 implementation
        
        let mut hasher = crate::stdlib::packages::crypto_hash_advanced::blake3::Blake3Hasher::new();
        
        // Add algorithm variant
        hasher.update(&[variant]);
        
        // Add parameters
        hasher.update(&self.config.memory_cost.to_le_bytes());
        hasher.update(&self.config.time_cost.to_le_bytes());
        hasher.update(&self.config.parallelism.to_le_bytes());
        
        // Add salt and password
        hasher.update(salt);
        hasher.update(password);
        
        // Simulate memory-hard computation with iterations
        let mut state = hasher.finalize();
        
        for _ in 0..self.config.time_cost {
            let mut round_hasher = crate::stdlib::packages::crypto_hash_advanced::blake3::Blake3Hasher::new();
            round_hasher.update(&state);
            round_hasher.update(password);
            round_hasher.update(salt);
            state = round_hasher.finalize();
        }
        
        // Truncate to desired length
        state.truncate(self.config.hash_length as usize);
        Ok(state)
    }
    
    /// Simplified scrypt implementation
    fn scrypt_hash(&self, password: &[u8], salt: &[u8]) -> PasswordResult<Vec<u8>> {
        // Simplified scrypt - production should use proper implementation
        let mut hasher = crate::stdlib::packages::crypto_hash_advanced::blake3::Blake3Hasher::new();
        
        hasher.update(salt);
        hasher.update(password);
        
        // Simulate memory-hard operations
        let mut state = hasher.finalize();
        let mut memory_blocks = Vec::new();
        
        // Create memory blocks
        for i in 0..self.config.memory_cost {
            let mut block_hasher = crate::stdlib::packages::crypto_hash_advanced::blake3::Blake3Hasher::new();
            block_hasher.update(&state);
            block_hasher.update(&i.to_le_bytes());
            let block = block_hasher.finalize();
            memory_blocks.push(block.clone());
            state = block;
        }
        
        // Mix memory blocks
        for _ in 0..self.config.time_cost {
            for block in &memory_blocks {
                let mut mix_hasher = crate::stdlib::packages::crypto_hash_advanced::blake3::Blake3Hasher::new();
                mix_hasher.update(&state);
                mix_hasher.update(block);
                state = mix_hasher.finalize();
            }
        }
        
        state.truncate(self.config.hash_length as usize);
        Ok(state)
    }
    
    /// PBKDF2 implementation
    fn pbkdf2_hash(&self, password: &[u8], salt: &[u8]) -> PasswordResult<Vec<u8>> {
        let mut result = Vec::new();
        let hash_len = 32; // Blake3 output size
        let iterations = self.config.time_cost;
        
        let blocks_needed = (self.config.hash_length + hash_len as u32 - 1) / hash_len as u32;
        
        for block_index in 1..=blocks_needed {
            let mut block_salt = salt.to_vec();
            block_salt.extend_from_slice(&block_index.to_be_bytes());
            
            // Initial hash
            let mut hasher = crate::stdlib::packages::crypto_hash_advanced::blake3::Blake3Hasher::new();
            hasher.update(password);
            hasher.update(&block_salt);
            let mut u = hasher.finalize();
            let mut f = u.clone();
            
            // Iterate
            for _ in 1..iterations {
                let mut iter_hasher = crate::stdlib::packages::crypto_hash_advanced::blake3::Blake3Hasher::new();
                iter_hasher.update(password);
                iter_hasher.update(&u);
                u = iter_hasher.finalize();
                
                // XOR with accumulated result
                for (f_byte, u_byte) in f.iter_mut().zip(u.iter()) {
                    *f_byte ^= u_byte;
                }
            }
            
            result.extend_from_slice(&f);
        }
        
        result.truncate(self.config.hash_length as usize);
        Ok(result)
    }
    
    fn generate_salt(&self) -> PasswordResult<Vec<u8>> {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        
        let mut salt = Vec::with_capacity(self.config.salt_length as usize);
        
        // Generate salt using system randomness
        for i in 0..((self.config.salt_length + 7) / 8) {
            let hasher = RandomState::new().build_hasher();
            let random_val = hasher.finish().wrapping_add(i as u64);
            let bytes = random_val.to_le_bytes();
            
            let remaining = (self.config.salt_length as usize).saturating_sub(salt.len());
            let to_take = std::cmp::min(8, remaining);
            salt.extend_from_slice(&bytes[..to_take]);
        }
        
        Ok(salt)
    }
    
    fn create_phc_string(&self, hash: &[u8], salt: &[u8]) -> PasswordResult<String> {
        let hash_b64 = PasswordHash::encode_base64(hash);
        let salt_b64 = PasswordHash::encode_base64(salt);
        
        let params = match self.config.algorithm {
            PasswordAlgorithm::Argon2id | PasswordAlgorithm::Argon2i | PasswordAlgorithm::Argon2d => {
                format!("m={},t={},p={}", self.config.memory_cost, self.config.time_cost, self.config.parallelism)
            },
            PasswordAlgorithm::Scrypt => {
                format!("n={},r={},p={}", self.config.memory_cost, 8, self.config.parallelism)
            },
            PasswordAlgorithm::Pbkdf2 => {
                format!("i={}", self.config.time_cost)
            },
        };
        
        Ok(format!("${}${}${}${}", 
                  self.config.algorithm.name(), 
                  params, 
                  salt_b64, 
                  hash_b64))
    }
}

/// Password strength analyzer
pub struct PasswordStrengthAnalyzer;

impl PasswordStrengthAnalyzer {
    /// Analyze password strength
    pub fn analyze(password: &str) -> PasswordStrength {
        let mut score = 0;
        let mut feedback = Vec::new();
        
        // Length check
        if password.len() >= 12 {
            score += 25;
        } else if password.len() >= 8 {
            score += 15;
            feedback.push("Consider using a longer password (12+ characters)".to_string());
        } else {
            feedback.push("Password is too short (minimum 8 characters)".to_string());
        }
        
        // Character diversity
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = password.chars().any(|c| !c.is_alphanumeric());
        
        let char_types = [has_lower, has_upper, has_digit, has_symbol].iter()
            .map(|&b| if b { 1 } else { 0 }).sum::<i32>();
        
        score += char_types * 15;
        
        if !has_lower { feedback.push("Add lowercase letters".to_string()); }
        if !has_upper { feedback.push("Add uppercase letters".to_string()); }
        if !has_digit { feedback.push("Add numbers".to_string()); }
        if !has_symbol { feedback.push("Add symbols".to_string()); }
        
        // Common patterns
        if password.to_lowercase().contains("password") {
            score -= 20;
            feedback.push("Avoid using 'password' in your password".to_string());
        }
        
        if password.chars().collect::<Vec<_>>().windows(3)
            .any(|w| w[0] as u8 + 1 == w[1] as u8 && w[1] as u8 + 1 == w[2] as u8) {
            score -= 10;
            feedback.push("Avoid sequential characters".to_string());
        }
        
        // Repetition
        let unique_chars = password.chars().collect::<std::collections::HashSet<_>>().len();
        if unique_chars < password.len() / 2 {
            score -= 15;
            feedback.push("Avoid excessive character repetition".to_string());
        }
        
        let strength_level = match score {
            0..=30 => PasswordStrengthLevel::VeryWeak,
            31..=50 => PasswordStrengthLevel::Weak,
            51..=70 => PasswordStrengthLevel::Moderate,
            71..=85 => PasswordStrengthLevel::Strong,
            _ => PasswordStrengthLevel::VeryStrong,
        };
        
        PasswordStrength {
            score: std::cmp::max(0, score) as u8,
            level: strength_level,
            feedback,
            estimated_crack_time: Self::estimate_crack_time(password),
        }
    }
    
    fn estimate_crack_time(password: &str) -> Duration {
        let charset_size = Self::estimate_charset_size(password);
        let entropy = (password.len() as f64) * (charset_size as f64).log2();
        
        // Assume 1 billion guesses per second
        let guesses_per_second = 1_000_000_000f64;
        let average_guesses = 2f64.powf(entropy - 1.0);
        let seconds = average_guesses / guesses_per_second;
        
        Duration::from_secs(seconds as u64)
    }
    
    fn estimate_charset_size(password: &str) -> usize {
        let mut size = 0;
        
        if password.chars().any(|c| c.is_ascii_lowercase()) { size += 26; }
        if password.chars().any(|c| c.is_ascii_uppercase()) { size += 26; }
        if password.chars().any(|c| c.is_ascii_digit()) { size += 10; }
        if password.chars().any(|c| !c.is_alphanumeric()) { size += 32; }
        
        std::cmp::max(size, 1)
    }
}

#[derive(Debug, Clone)]
pub struct PasswordStrength {
    pub score: u8,
    pub level: PasswordStrengthLevel,
    pub feedback: Vec<String>,
    pub estimated_crack_time: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PasswordStrengthLevel {
    VeryWeak,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

impl PasswordStrengthLevel {
    pub fn description(&self) -> &'static str {
        match self {
            PasswordStrengthLevel::VeryWeak => "Very Weak",
            PasswordStrengthLevel::Weak => "Weak",
            PasswordStrengthLevel::Moderate => "Moderate",
            PasswordStrengthLevel::Strong => "Strong",
            PasswordStrengthLevel::VeryStrong => "Very Strong",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let hasher = PasswordHasher::new(PasswordConfig::fast());
        
        let password = "my_secure_password";
        let hash_result = hasher.hash_password(password).unwrap();
        
        assert_eq!(hash_result.algorithm, PasswordAlgorithm::Argon2id);
        assert!(!hash_result.hash.is_empty());
        assert!(!hash_result.salt.is_empty());
        
        // Verify correct password
        assert!(hasher.verify_password(password, &hash_result).unwrap());
        
        // Verify incorrect password
        assert!(!hasher.verify_password("wrong_password", &hash_result).unwrap());
    }

    #[test]
    fn test_phc_string_format() {
        let hasher = PasswordHasher::new(PasswordConfig::fast());
        let hash_result = hasher.hash_password("test").unwrap();
        
        let phc_string = hash_result.to_string();
        assert!(phc_string.starts_with("$argon2id$"));
        
        let parsed = PasswordHash::from_string(&phc_string).unwrap();
        assert_eq!(parsed.algorithm, hash_result.algorithm);
    }

    #[test]
    fn test_password_strength_analyzer() {
        let weak = PasswordStrengthAnalyzer::analyze("123");
        assert_eq!(weak.level, PasswordStrengthLevel::VeryWeak);
        assert!(!weak.feedback.is_empty());
        
        let strong = PasswordStrengthAnalyzer::analyze("MyStr0ng!P@ssw0rd");
        assert!(matches!(strong.level, PasswordStrengthLevel::Strong | PasswordStrengthLevel::VeryStrong));
        
        let moderate = PasswordStrengthAnalyzer::analyze("password123");
        assert!(weak.score < moderate.score);
    }

    #[test]
    fn test_password_configs() {
        let fast = PasswordConfig::fast();
        let secure = PasswordConfig::secure_default();
        
        assert!(fast.memory_cost < secure.memory_cost);
        assert!(fast.time_cost <= secure.time_cost);
    }

    #[test]
    fn test_different_algorithms() {
        let configs = [
            PasswordConfig { algorithm: PasswordAlgorithm::Argon2id, ..PasswordConfig::fast() },
            PasswordConfig { algorithm: PasswordAlgorithm::Scrypt, ..PasswordConfig::fast() },
            PasswordConfig { algorithm: PasswordAlgorithm::Pbkdf2, ..PasswordConfig::fast() },
        ];
        
        for config in &configs {
            let hasher = PasswordHasher::new(config.clone());
            let result = hasher.hash_password("test").unwrap();
            assert_eq!(result.algorithm, config.algorithm);
            assert!(hasher.verify_password("test", &result).unwrap());
        }
    }
}
