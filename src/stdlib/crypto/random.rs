use crate::error::CursedError;
use std::sync::OnceLock;
use sha2::{Sha256, Digest};
use tracing::{debug, info, warn, error, instrument};

/// Cryptographically secure random number generator
#[derive(Debug)]
pub struct SecureRandom {
impl SecureRandom {
    /// Create a new secure random generator
    #[instrument]
    pub fn new() -> crate::error::Result<()> {
        let mut generator = Self {
            max_requests_before_reseed: 1_000_000, // Reseed every million requests
        generator.reseed()?;
        info!("Created new secure random generator");
        Ok(generator)
    /// Generate random bytes
    #[instrument(skip(self))]
    pub fn generate_bytes(&mut self, count: usize) -> crate::error::Result<()> {
        if self.reseed_counter >= self.max_requests_before_reseed {
            self.reseed()?;
        let mut result = vec![0u8; count];
        self.state.fill_bytes(&mut result)?;
        self.reseed_counter += 1;
        
        debug!(bytes_generated = count, reseed_counter = self.reseed_counter, "Generated random bytes");
        Ok(result)
    /// Generate a random u32
    #[instrument(skip(self))]
    pub fn generate_u32(&mut self) -> crate::error::Result<()> {
        let bytes = self.generate_bytes(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    /// Generate a random u64
    #[instrument(skip(self))]
    pub fn generate_u64(&mut self) -> crate::error::Result<()> {
        let bytes = self.generate_bytes(8)?;
        Ok(u64::from_le_bytes([
        ]))
    /// Generate random number in range [0, max)
    #[instrument(skip(self))]
    pub fn generate_range(&mut self, max: u64) -> crate::error::Result<()> {
        if max == 0 {
            return Ok(0);
        // Use rejection sampling to avoid bias
        let mask = (1u64 << (64 - max.leading_zeros())) - 1;
        loop {
            let candidate = self.generate_u64()? & mask;
            if candidate < max {
                debug!(candidate, max, "Generated random in range");
                return Ok(candidate);
            }
        }
    /// Reseed the generator with fresh entropy
    #[instrument(skip(self))]
    fn reseed(&mut self) -> crate::error::Result<()> {
        let entropy = collect_entropy()?;
        self.state.reseed(&entropy)?;
        self.reseed_counter = 0;
        debug!("Reseeded random generator");
        Ok(())
    }
}

/// ChaCha20-based random state (simplified implementation)
#[derive(Debug)]
struct ChaCha20State {
impl ChaCha20State {
    fn new() -> crate::error::Result<()> {
        Ok(Self {
        })
    fn reseed(&mut self, entropy: &[u8]) -> crate::error::Result<()> {
        if entropy.len() < 32 {
            return Err(CursedError::new("random_error", "Insufficient entropy"));
        // Hash the entropy to create key material
        let mut hasher = Sha256::new();
        hasher.update(entropy);
        let hash = hasher.finalize();

        // Use hash for key and nonce
        for i in 0..8 {
            self.key[i] = u32::from_le_bytes([
                hash[i * 4 + 2], hash[i * 4 + 3]
            ]);
        self.counter = 0;
        self.nonce[0] = u32::from_le_bytes([hash[28], hash[29], hash[30], hash[31]]);
        self.nonce[1] = 0; // Will be updated from additional entropy
        self.nonce[2] = 0;

        Ok(())
    fn fill_bytes(&mut self, output: &mut [u8]) -> crate::error::Result<()> {
        let mut offset = 0;
        while offset < output.len() {
            let block = self.generate_block();
            let copy_len = std::cmp::min(64, output.len() - offset);
            
            for i in 0..copy_len {
                output[offset + i] = ((block[i / 4] >> ((i % 4) * 8)) & 0xff) as u8;
            offset += copy_len;
            self.counter += 1;
        }
        Ok(())
    fn generate_block(&self) -> [u32; 16] {
        // Simplified ChaCha20 block generation
        let mut state = [0u32; 16];
        
        // Constants
        state[0] = 0x61707865;
        state[1] = 0x3320646e;
        state[2] = 0x79622d32;
        state[3] = 0x6b206574;
        
        // Key
        state[4..12].copy_from_slice(&self.key);
        
        // Counter and nonce
        state[12] = self.counter as u32;
        state[13] = (self.counter >> 32) as u32;
        state[14] = self.nonce[0];
        state[15] = self.nonce[1];

        // ChaCha20 rounds (simplified - normally 20 rounds)
        for _ in 0..10 {
            self.quarter_round(&mut state, 0, 4, 8, 12);
            self.quarter_round(&mut state, 1, 5, 9, 13);
            self.quarter_round(&mut state, 2, 6, 10, 14);
            self.quarter_round(&mut state, 3, 7, 11, 15);
            
            self.quarter_round(&mut state, 0, 5, 10, 15);
            self.quarter_round(&mut state, 1, 6, 11, 12);
            self.quarter_round(&mut state, 2, 7, 8, 13);
            self.quarter_round(&mut state, 3, 4, 9, 14);
        state
    fn quarter_round(&self, state: &mut [u32], a: usize, b: usize, c: usize, d: usize) {
        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(16);
        
        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(12);
        
        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(8);
        
        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(7);
    }
}

/// UUID v4 generation utilities
pub struct UuidV4Generator {
impl UuidV4Generator {
    /// Create new UUID generator
    #[instrument]
    pub fn new() -> crate::error::Result<()> {
        info!("Creating UUID v4 generator");
        Ok(Self {
        })
    /// Generate a UUID v4
    #[instrument(skip(self))]
    pub fn generate(&mut self) -> crate::error::Result<()> {
        let mut bytes = self.rng.generate_bytes(16)?;
        
        // Set version to 4
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        
        // Set variant bits
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        
        let uuid = format!(
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        );
        
        debug!(uuid = %uuid, "Generated UUID v4");
        Ok(uuid)
    /// Generate multiple UUIDs at once
    #[instrument(skip(self))]
    pub fn generate_batch(&mut self, count: usize) -> crate::error::Result<()> {
        let mut uuids = Vec::with_capacity(count);
        for _ in 0..count {
            uuids.push(self.generate()?);
        }
        debug!(count, "Generated UUID batch");
        Ok(uuids)
    }
}

/// Salt generation utilities
pub struct SaltGenerator {
impl SaltGenerator {
    /// Create new salt generator
    #[instrument]
    pub fn new() -> crate::error::Result<()> {
        info!("Creating salt generator");
        Ok(Self {
        })
    /// Generate cryptographic salt
    #[instrument(skip(self))]
    pub fn generate_salt(&mut self, length: usize) -> crate::error::Result<()> {
        if length == 0 {
            return Err(CursedError::new("salt_error", "Salt length must be greater than 0"));
        }
        if length > 1024 {
            return Err(CursedError::new("salt_error", "Salt length too large"));
        let salt = self.rng.generate_bytes(length)?;
        debug!(length, "Generated cryptographic salt");
        Ok(salt)
    /// Generate salt as hex string
    #[instrument(skip(self))]
    pub fn generate_salt_hex(&mut self, byte_length: usize) -> crate::error::Result<()> {
        let salt = self.generate_salt(byte_length)?;
        let hex = salt.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        debug!(byte_length, hex_length = hex.len(), "Generated salt as hex");
        Ok(hex)
    /// Generate salt as base64 string
    #[instrument(skip(self))]
    pub fn generate_salt_base64(&mut self, byte_length: usize) -> crate::error::Result<()> {
        let salt = self.generate_salt(byte_length)?;
        let b64 = base64::engine::general_purpose::STANDARD.encode(&salt);
        debug!(byte_length, base64_length = b64.len(), "Generated salt as base64");
        Ok(b64)
    }
}

/// Nonce generation utilities
pub struct NonceGenerator {
impl NonceGenerator {
    /// Create new nonce generator
    #[instrument]
    pub fn new() -> crate::error::Result<()> {
        info!("Creating nonce generator");
        Ok(Self {
        })
    /// Generate cryptographic nonce
    #[instrument(skip(self))]
    pub fn generate_nonce(&mut self, length: usize) -> crate::error::Result<()> {
        if length == 0 {
            return Err(CursedError::new("nonce_error", "Nonce length must be greater than 0"));
        }
        if length > 256 {
            return Err(CursedError::new("nonce_error", "Nonce length too large"));
        let nonce = self.rng.generate_bytes(length)?;
        debug!(length, "Generated cryptographic nonce");
        Ok(nonce)
    /// Generate time-based nonce (includes timestamp)
    #[instrument(skip(self))]
    pub fn generate_time_nonce(&mut self, random_bytes: usize) -> crate::error::Result<()> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| CursedError::new("nonce_error", &format!("Time error: {}", e)))?
            .as_nanos() as u64;
        
        let random_part = self.generate_nonce(random_bytes)?;
        
        let mut nonce = Vec::with_capacity(8 + random_bytes);
        nonce.extend_from_slice(&timestamp.to_le_bytes());
        nonce.extend_from_slice(&random_part);
        
        debug!(timestamp, random_bytes, total_length = nonce.len(), "Generated time-based nonce");
        Ok(nonce)
    /// Generate nonce for specific purpose
    #[instrument(skip(self))]
    pub fn generate_purpose_nonce(&mut self, purpose: &str, length: usize) -> crate::error::Result<()> {
        let mut nonce = self.generate_nonce(length)?;
        
        // Mix in purpose string for domain separation
        let mut hasher = Sha256::new();
        hasher.update(purpose.as_bytes());
        hasher.update(&nonce);
        let hash = hasher.finalize();
        
        // XOR first bytes with hash
        let mix_len = std::cmp::min(nonce.len(), hash.len());
        for i in 0..mix_len {
            nonce[i] ^= hash[i];
        debug!(purpose, length, "Generated purpose-specific nonce");
        Ok(nonce)
    }
}

/// Global entropy collection for seeding
static ENTROPY_POOL: OnceLock<std::sync::Mutex<Vec<u8>>> = OnceLock::new();

/// Collect system entropy for seeding random generators
#[instrument]
fn collect_entropy() -> crate::error::Result<()> {
    let mut entropy = Vec::with_capacity(256);
    
    // Add timestamp
    if let Ok(duration) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        entropy.extend_from_slice(&duration.as_nanos().to_le_bytes());
    // Add thread ID (if available)
    entropy.extend_from_slice(&std::thread::current().id().as_u64().to_le_bytes());
    
    // Add process ID approximation (using hash of current exe path)
    if let Ok(exe) = std::env::current_exe() {
        let mut hasher = Sha256::new();
        hasher.update(exe.to_string_lossy().as_bytes());
        entropy.extend_from_slice(&hasher.finalize()[..8]);
    // Add memory address entropy
    let stack_addr = &entropy as *const _ as usize;
    entropy.extend_from_slice(&stack_addr.to_le_bytes());
    
    // Try to read from OS random source
    #[cfg(unix)]
    {
        if let Ok(mut file) = std::fs::File::open("/dev/urandom") {
            use std::io::Read;
            let mut os_random = [0u8; 32];
            if file.read_exact(&mut os_random).is_ok() {
                entropy.extend_from_slice(&os_random);
            }
        }
    #[cfg(windows)]
    {
        // On Windows, we'd use CryptGenRandom or similar
        // For now, use more system state
        entropy.extend_from_slice(&std::process::id().to_le_bytes());
    // Mix with global entropy pool
    if let Some(pool) = ENTROPY_POOL.get() {
        if let Ok(mut global_entropy) = pool.lock() {
            if !global_entropy.is_empty() {
                let mix_len = std::cmp::min(entropy.len(), global_entropy.len());
                for i in 0..mix_len {
                    entropy[i] ^= global_entropy[i];
                }
            }
            
            // Add current entropy to global pool
            global_entropy.extend_from_slice(&entropy);
            if global_entropy.len() > 1024 {
                global_entropy.drain(..512); // Keep pool size reasonable
            }
        }
    } else {
        // Initialize global pool
        let _ = ENTROPY_POOL.set(std::sync::Mutex::new(entropy.clone()));
    if entropy.len() < 32 {
        return Err(CursedError::new("entropy_error", "Insufficient entropy collected"));
    debug!(entropy_bytes = entropy.len(), "Collected system entropy");
    Ok(entropy)
/// Quality check for random data
pub fn test_randomness_quality(data: &[u8]) -> RandomnessQuality {
    if data.is_empty() {
        return RandomnessQuality {
    // Basic entropy estimation
    let mut byte_counts = [0u32; 256];
    for &byte in data {
        byte_counts[byte as usize] += 1;
    let data_len = data.len() as f64;
    let mut entropy = 0.0;
    for &count in &byte_counts {
        if count > 0 {
            let p = count as f64 / data_len;
            entropy -= p * p.log2();
        }
    }
    
    // Chi-squared test
    let expected = data_len / 256.0;
    let mut chi_squared = 0.0;
    for &count in &byte_counts {
        let diff = count as f64 - expected;
        chi_squared += (diff * diff) / expected;
    // Simple pattern detection
    let has_patterns = has_obvious_patterns(data);
    
    // Basic quality thresholds
    let passes_basic_tests = entropy > 7.0 && chi_squared < 300.0 && !has_patterns;
    
    RandomnessQuality {
    }
}

/// Check for obvious patterns in data
fn has_obvious_patterns(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    // Check for repeated bytes
    let first_byte = data[0];
    if data.iter().all(|&b| b == first_byte) {
        return true;
    // Check for simple sequences
    let mut ascending = 0;
    let mut descending = 0;
    for window in data.windows(2) {
        if window[1] == window[0].wrapping_add(1) {
            ascending += 1;
        } else if window[1] == window[0].wrapping_sub(1) {
            descending += 1;
        }
    }
    
    let sequence_ratio = (ascending + descending) as f64 / (data.len() - 1) as f64;
    sequence_ratio > 0.8
/// Randomness quality metrics
#[derive(Debug, Clone)]
pub struct RandomnessQuality {
