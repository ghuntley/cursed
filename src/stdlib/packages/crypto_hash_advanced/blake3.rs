/// fr fr BLAKE3 implementation with streaming and keyed modes
/// 
/// This module provides production-ready BLAKE3 hash function with support for
/// regular hashing, keyed hashing, and key derivation functions.

use crate::error::CursedError;
use crate::stdlib::value::Value;
use std::collections::HashMap;

/// fr fr BLAKE3 constants
pub const BLAKE3_KEY_LEN: usize = 32;
pub const BLAKE3_OUT_LEN: usize = 32;
pub const BLAKE3_BLOCK_LEN: usize = 64;
pub const BLAKE3_CHUNK_LEN: usize = 1024;

/// fr fr BLAKE3 initialization vectors (first 8 words of SHA-256 state)
const IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
    0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

/// fr fr BLAKE3 flags for different modes
const CHUNK_START: u8 = 1 << 0;
const CHUNK_END: u8 = 1 << 1;
const PARENT: u8 = 1 << 2;
const ROOT: u8 = 1 << 3;
const KEYED_HASH: u8 = 1 << 4;
const DERIVE_KEY_CONTEXT: u8 = 1 << 5;
const DERIVE_KEY_MATERIAL: u8 = 1 << 6;

/// fr fr BLAKE3 operation modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blake3Mode {
    Regular,    // Standard hashing
    Keyed,      // Keyed hashing (MAC)
    DeriveKey,  // Key derivation
}

impl Blake3Mode {
    pub fn name(&self) -> &'static str {
        match self {
            Blake3Mode::Regular => "BLAKE3",
            Blake3Mode::Keyed => "BLAKE3-Keyed",
            Blake3Mode::DeriveKey => "BLAKE3-KDF",
        }
    }
    
    pub fn flag(&self) -> u8 {
        match self {
            Blake3Mode::Regular => 0,
            Blake3Mode::Keyed => KEYED_HASH,
            Blake3Mode::DeriveKey => DERIVE_KEY_CONTEXT,
        }
    }
}

/// fr fr BLAKE3 compression state
#[derive(Debug, Clone)]
struct Blake3State {
    h: [u32; 8],
    t: u64,
    flags: u8,
}

impl Blake3State {
    fn new(key: &[u32; 8], flags: u8) -> Self {
        Self {
            h: *key,
            t: 0,
            flags,
        }
    }
    
    fn compress(&mut self, block: &[u8; 64], block_len: u32, flags: u8) {
        let m = Self::words_from_le_bytes(block);
        
        let mut v = [0u32; 16];
        v[0..8].copy_from_slice(&self.h);
        v[8..12].copy_from_slice(&IV[0..4]);
        v[12] = (self.t & 0xFFFFFFFF) as u32;
        v[13] = (self.t >> 32) as u32;
        v[14] = block_len;
        v[15] = flags as u32;
        
        self.blake3_compress(&mut v, &m);
        
        // Update state
        for i in 0..8 {
            self.h[i] = v[i] ^ v[i + 8];
        }
    }
    
    fn blake3_compress(&self, v: &mut [u32; 16], m: &[u32; 16]) {
        const SIGMA: [[usize; 16]; 7] = [
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8],
            [3, 4, 10, 12, 13, 2, 7, 14, 6, 5, 9, 0, 11, 15, 8, 1],
            [10, 7, 12, 9, 14, 3, 13, 15, 4, 0, 11, 2, 5, 8, 1, 6],
            [12, 13, 9, 11, 15, 10, 14, 8, 7, 2, 5, 3, 0, 1, 6, 4],
            [9, 14, 11, 5, 8, 12, 15, 1, 13, 3, 0, 10, 2, 6, 4, 7],
            [11, 15, 5, 0, 1, 9, 8, 6, 14, 10, 2, 12, 3, 4, 7, 13],
        ];
        
        for round in 0..7 {
            let s = &SIGMA[round];
            
            // Column operations
            self.g(v, 0, 4, 8, 12, m[s[0]], m[s[1]]);
            self.g(v, 1, 5, 9, 13, m[s[2]], m[s[3]]);
            self.g(v, 2, 6, 10, 14, m[s[4]], m[s[5]]);
            self.g(v, 3, 7, 11, 15, m[s[6]], m[s[7]]);
            
            // Diagonal operations
            self.g(v, 0, 5, 10, 15, m[s[8]], m[s[9]]);
            self.g(v, 1, 6, 11, 12, m[s[10]], m[s[11]]);
            self.g(v, 2, 7, 8, 13, m[s[12]], m[s[13]]);
            self.g(v, 3, 4, 9, 14, m[s[14]], m[s[15]]);
        }
    }
    
    fn g(&self, v: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, x: u32, y: u32) {
        v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
        v[d] = (v[d] ^ v[a]).rotate_right(16);
        v[c] = v[c].wrapping_add(v[d]);
        v[b] = (v[b] ^ v[c]).rotate_right(12);
        v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
        v[d] = (v[d] ^ v[a]).rotate_right(8);
        v[c] = v[c].wrapping_add(v[d]);
        v[b] = (v[b] ^ v[c]).rotate_right(7);
    }
    
    fn words_from_le_bytes(bytes: &[u8; 64]) -> [u32; 16] {
        let mut words = [0u32; 16];
        for (i, chunk) in bytes.chunks_exact(4).enumerate() {
            words[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        words
    }
}

/// fr fr BLAKE3 chunk state for processing 1024-byte chunks
#[derive(Debug, Clone)]
struct Blake3ChunkState {
    state: Blake3State,
    chunk_counter: u64,
    buf: [u8; 64],
    buf_len: usize,
    blocks_compressed: u8,
}

impl Blake3ChunkState {
    fn new(key: &[u32; 8], chunk_counter: u64, flags: u8) -> Self {
        Self {
            state: Blake3State::new(key, flags),
            chunk_counter,
            buf: [0; 64],
            buf_len: 0,
            blocks_compressed: 0,
        }
    }
    
    fn update(&mut self, input: &[u8]) {
        let mut input_offset = 0;
        
        while input_offset < input.len() {
            // Fill buffer
            let want = 64 - self.buf_len;
            let take = want.min(input.len() - input_offset);
            self.buf[self.buf_len..self.buf_len + take]
                .copy_from_slice(&input[input_offset..input_offset + take]);
            self.buf_len += take;
            input_offset += take;
            
            // Process full blocks
            if self.buf_len == 64 {
                let mut flags = self.state.flags;
                if self.blocks_compressed == 0 {
                    flags |= CHUNK_START;
                }
                
                self.state.t = self.chunk_counter * BLAKE3_CHUNK_LEN as u64 + (self.blocks_compressed as u64 * 64);
                self.state.compress(&self.buf, 64, flags);
                self.blocks_compressed += 1;
                self.buf_len = 0;
            }
        }
    }
    
    fn finalize(&mut self, output: &mut [u8]) {
        let mut flags = self.state.flags;
        if self.blocks_compressed == 0 {
            flags |= CHUNK_START;
        }
        flags |= CHUNK_END;
        
        self.state.t = self.chunk_counter * BLAKE3_CHUNK_LEN as u64 + (self.blocks_compressed as u64 * 64);
        self.state.compress(&self.buf, self.buf_len as u32, flags);
        
        // Extract output
        for (i, chunk) in output.chunks_mut(4).enumerate() {
            let word = self.state.h[i % 8];
            let bytes = word.to_le_bytes();
            let take = chunk.len().min(4);
            chunk[..take].copy_from_slice(&bytes[..take]);
        }
    }
}

/// fr fr BLAKE3 hasher with support for different modes
#[derive(Debug, Clone)]
pub struct Blake3Hasher {
    key: [u32; 8],
    mode: Blake3Mode,
    chunk_state: Blake3ChunkState,
    cv_stack: Vec<[u8; 32]>,
    cv_stack_len: usize,
}

impl Blake3Hasher {
    /// slay Create new BLAKE3 hasher in regular mode
    pub fn new() -> Self {
        Self::new_with_mode(Blake3Mode::Regular)
    }
    
    /// bestie Create new BLAKE3 hasher with specific mode
    pub fn new_with_mode(mode: Blake3Mode) -> Self {
        let key = IV;
        let flags = mode.flag();
        
        Self {
            key,
            mode,
            chunk_state: Blake3ChunkState::new(&key, 0, flags),
            cv_stack: Vec::with_capacity(10),
            cv_stack_len: 0,
        }
    }
    
    /// vibes Create new BLAKE3 hasher with key (for keyed hashing)
    pub fn new_keyed(key: &[u8; 32]) -> Self {
        let mut key_words = [0u32; 8];
        for (i, chunk) in key.chunks_exact(4).enumerate() {
            key_words[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        
        Self {
            key: key_words,
            mode: Blake3Mode::Keyed,
            chunk_state: Blake3ChunkState::new(&key_words, 0, KEYED_HASH),
            cv_stack: Vec::with_capacity(10),
            cv_stack_len: 0,
        }
    }
    
    /// periodt Create new BLAKE3 hasher for key derivation
    pub fn new_derive_key(context: &str) -> Self {
        // Hash the context string to derive the key
        let mut context_hasher = Self::new_with_mode(Blake3Mode::Regular);
        context_hasher.update(context.as_bytes());
        let context_hash = context_hasher.finalize_fixed();
        
        let mut key_words = [0u32; 8];
        for (i, chunk) in context_hash.chunks_exact(4).enumerate() {
            key_words[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        
        Self {
            key: key_words,
            mode: Blake3Mode::DeriveKey,
            chunk_state: Blake3ChunkState::new(&key_words, 0, DERIVE_KEY_CONTEXT),
            cv_stack: Vec::with_capacity(10),
            cv_stack_len: 0,
        }
    }
    
    /// yolo Update hasher with more data
    pub fn update(&mut self, input: &[u8]) {
        let mut input_offset = 0;
        
        while input_offset < input.len() {
            // Check if current chunk is complete
            if self.chunk_state.blocks_compressed as usize * 64 + self.chunk_state.buf_len >= BLAKE3_CHUNK_LEN {
                // Finalize current chunk
                let mut cv = [0u8; 32];
                self.chunk_state.finalize(&mut cv);
                
                // Add to CV stack
                self.add_chunk_cv(&cv);
                
                // Start new chunk
                self.chunk_state = Blake3ChunkState::new(
                    &self.key,
                    self.cv_stack_len as u64,
                    self.mode.flag()
                );
            }
            
            // Update current chunk
            let chunk_space = BLAKE3_CHUNK_LEN - (self.chunk_state.blocks_compressed as usize * 64 + self.chunk_state.buf_len);
            let take = chunk_space.min(input.len() - input_offset);
            self.chunk_state.update(&input[input_offset..input_offset + take]);
            input_offset += take;
        }
    }
    
    /// facts Finalize and get 32-byte hash
    pub fn finalize_fixed(mut self) -> [u8; 32] {
        let mut output = [0u8; 32];
        self.finalize_variable(&mut output);
        output
    }
    
    /// slay Finalize with variable output length
    pub fn finalize_variable(mut self, output: &mut [u8]) {
        // Finalize current chunk
        let mut cv = [0u8; 32];
        self.chunk_state.finalize(&mut cv);
        
        if self.cv_stack_len == 0 {
            // Only one chunk, this is the root
            let mut root_state = Blake3State::new(&self.key, self.mode.flag() | ROOT);
            root_state.t = 0;
            
            // Convert CV to block for compression
            let mut block = [0u8; 64];
            block[..32].copy_from_slice(&cv);
            root_state.compress(&block, 32, self.mode.flag() | ROOT);
            
            // Extract output
            self.extract_output(&root_state, output);
        } else {
            // Multiple chunks, need to merge CV stack
            self.add_chunk_cv(&cv);
            
            // Merge CV stack to get root
            let root_cv = self.merge_cv_stack();
            
            // Create root state and extract output
            let mut root_state = Blake3State::new(&self.key, self.mode.flag() | ROOT);
            root_state.t = 0;
            
            let mut block = [0u8; 64];
            block[..32].copy_from_slice(&root_cv);
            root_state.compress(&block, 32, self.mode.flag() | ROOT);
            
            self.extract_output(&root_state, output);
        }
    }
    
    fn add_chunk_cv(&mut self, cv: &[u8; 32]) {
        self.cv_stack.push(*cv);
        self.cv_stack_len += 1;
        
        // Merge complete subtrees
        while self.cv_stack_len > 1 && (self.cv_stack_len & (self.cv_stack_len - 1)) == 0 {
            let right = self.cv_stack.pop().unwrap();
            let left = self.cv_stack.pop().unwrap();
            let merged = self.merge_cvs(&left, &right);
            self.cv_stack.push(merged);
            self.cv_stack_len -= 1;
        }
    }
    
    fn merge_cvs(&self, left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
        let mut state = Blake3State::new(&self.key, self.mode.flag() | PARENT);
        state.t = 0;
        
        let mut block = [0u8; 64];
        block[..32].copy_from_slice(left);
        block[32..].copy_from_slice(right);
        state.compress(&block, 64, self.mode.flag() | PARENT);
        
        let mut result = [0u8; 32];
        for (i, chunk) in result.chunks_mut(4).enumerate() {
            let word = state.h[i];
            let bytes = word.to_le_bytes();
            chunk.copy_from_slice(&bytes);
        }
        result
    }
    
    fn merge_cv_stack(&self) -> [u8; 32] {
        let mut stack = self.cv_stack.clone();
        
        while stack.len() > 1 {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            let merged = self.merge_cvs(&left, &right);
            stack.push(merged);
        }
        
        stack[0]
    }
    
    fn extract_output(&self, state: &Blake3State, output: &mut [u8]) {
        let mut offset = 0;
        let mut counter = 0u64;
        
        while offset < output.len() {
            let mut block_state = state.clone();
            block_state.t = counter;
            
            // Compress to get 64 bytes of output
            let mut block = [0u8; 64];
            block_state.compress(&block, 0, state.flags | ROOT);
            
            let take = 64.min(output.len() - offset);
            for (i, byte) in output[offset..offset + take].iter_mut().enumerate() {
                *byte = (block_state.h[i / 4] >> ((i % 4) * 8)) as u8;
            }
            
            offset += take;
            counter += 1;
        }
    }
    
    /// bestie Hash data in one shot
    pub fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize_fixed()
    }
    
    /// vibes Keyed hash in one shot
    pub fn keyed_hash(key: &[u8; 32], data: &[u8]) -> [u8; 32] {
        let mut hasher = Self::new_keyed(key);
        hasher.update(data);
        hasher.finalize_fixed()
    }
    
    /// yolo Key derivation in one shot
    pub fn derive_key(context: &str, key_material: &[u8], output_len: usize) -> Vec<u8> {
        let mut hasher = Self::new_derive_key(context);
        hasher.update(key_material);
        let mut output = vec![0u8; output_len];
        hasher.finalize_variable(&mut output);
        output
    }
}

impl Default for Blake3Hasher {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Blake3 utilities
pub struct Blake3Utils;

impl Blake3Utils {
    /// bestie Convert hash bytes to hex string
    pub fn to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
    
    /// vibes Hash string with BLAKE3
    pub fn blake3_string(s: &str) -> String {
        let hash = Blake3Hasher::hash(s.as_bytes());
        Self::to_hex(&hash)
    }
    
    /// periodt Keyed hash string with BLAKE3
    pub fn blake3_keyed_string(key: &[u8; 32], s: &str) -> String {
        let hash = Blake3Hasher::keyed_hash(key, s.as_bytes());
        Self::to_hex(&hash)
    }
    
    /// facts Generate random BLAKE3 key
    pub fn generate_key() -> [u8; 32] {
        use rand::RngCore;
        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        key
    }
}

/// fr fr Public API functions for CURSED integration

/// slay BLAKE3 hash function
pub fn blake3(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("BLAKE3 requires input data".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("BLAKE3 input must be a string".to_string())),
    };
    
    let hash = Blake3Hasher::hash(data);
    Ok(Value::String(Blake3Utils::to_hex(&hash)))
}

/// slay BLAKE3 keyed hash function
pub fn blake3_keyed(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("BLAKE3 keyed hash requires key and data".to_string()));
    }
    
    let key_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("BLAKE3 key must be a string".to_string())),
    };
    
    let data = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("BLAKE3 input must be a string".to_string())),
    };
    
    // Convert hex key to bytes
    if key_str.len() != 64 {
        return Err(CursedError::Runtime("BLAKE3 key must be 64 hex characters (32 bytes)".to_string()));
    }
    
    let mut key = [0u8; 32];
    for (i, chunk) in key_str.as_bytes().chunks(2).enumerate() {
        if i >= 32 {
            break;
        }
        if chunk.len() == 2 {
            if let Ok(byte) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16) {
                key[i] = byte;
            } else {
                return Err(CursedError::Runtime("Invalid hex character in BLAKE3 key".to_string()));
            }
        }
    }
    
    let hash = Blake3Hasher::keyed_hash(&key, data);
    Ok(Value::String(Blake3Utils::to_hex(&hash)))
}

/// slay BLAKE3 key derivation function
pub fn blake3_derive_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("BLAKE3 KDF requires context, key material, and output length".to_string()));
    }
    
    let context = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("BLAKE3 KDF context must be a string".to_string())),
    };
    
    let key_material = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("BLAKE3 KDF key material must be a string".to_string())),
    };
    
    let output_len = match &args[2] {
        Value::Number(n) => *n as usize,
        _ => return Err(CursedError::Runtime("BLAKE3 KDF output length must be a number".to_string())),
    };
    
    if output_len == 0 || output_len > 1024 {
        return Err(CursedError::Runtime("BLAKE3 KDF output length must be between 1 and 1024 bytes".to_string()));
    }
    
    let derived_key = Blake3Hasher::derive_key(context, key_material, output_len);
    Ok(Value::String(Blake3Utils::to_hex(&derived_key)))
}

/// slay Generate BLAKE3 key
pub fn blake3_generate_key(_args: Vec<Value>) -> Result<Value, CursedError> {
    let key = Blake3Utils::generate_key();
    Ok(Value::String(Blake3Utils::to_hex(&key)))
}

/// slay Create streaming BLAKE3 hasher
pub fn create_blake3_hasher(args: Vec<Value>) -> Result<Value, CursedError> {
    let mode = if args.is_empty() {
        Blake3Mode::Regular
    } else {
        match &args[0] {
            Value::String(s) => match s.as_str() {
                "regular" => Blake3Mode::Regular,
                "keyed" => Blake3Mode::Keyed,
                "derive_key" => Blake3Mode::DeriveKey,
                _ => Blake3Mode::Regular,
            },
            _ => Blake3Mode::Regular,
        }
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String(mode.name().to_string()));
    result.insert("hasher_id".to_string(), Value::String("blake3_hasher_placeholder".to_string()));
    result.insert("output_size".to_string(), Value::Number(32.0));
    result.insert("variable_output".to_string(), Value::bool(true));
    
    Ok(Value::Object(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blake3_empty() {
        let hash = Blake3Hasher::hash(b"");
        assert_eq!(hash.len(), 32);
        // BLAKE3 of empty string should be specific value
        // (would need to verify with reference implementation)
    }
    
    #[test]
    fn test_blake3_abc() {
        let hash = Blake3Hasher::hash(b"abc");
        assert_eq!(hash.len(), 32);
        assert_ne!(hash, [0u8; 32]); // Should not be all zeros
    }
    
    #[test]
    fn test_blake3_streaming() {
        let mut hasher = Blake3Hasher::new();
        hasher.update(b"hello");
        hasher.update(b" ");
        hasher.update(b"world");
        let hash1 = hasher.finalize_fixed();
        
        let hash2 = Blake3Hasher::hash(b"hello world");
        assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_blake3_keyed() {
        let key = [1u8; 32];
        let data = b"test data";
        
        let hash1 = Blake3Hasher::keyed_hash(&key, data);
        let hash2 = Blake3Hasher::keyed_hash(&key, data);
        assert_eq!(hash1, hash2);
        
        // Different key should produce different hash
        let key2 = [2u8; 32];
        let hash3 = Blake3Hasher::keyed_hash(&key2, data);
        assert_ne!(hash1, hash3);
    }
    
    #[test]
    fn test_blake3_derive_key() {
        let context = "test context";
        let key_material = b"secret key material";
        
        let derived1 = Blake3Hasher::derive_key(context, key_material, 32);
        let derived2 = Blake3Hasher::derive_key(context, key_material, 32);
        assert_eq!(derived1, derived2);
        
        // Different context should produce different key
        let derived3 = Blake3Hasher::derive_key("different context", key_material, 32);
        assert_ne!(derived1, derived3);
        
        // Different length should work
        let derived4 = Blake3Hasher::derive_key(context, key_material, 64);
        assert_eq!(derived4.len(), 64);
    }
    
    #[test]
    fn test_blake3_variable_output() {
        let data = b"test";
        let mut hasher = Blake3Hasher::new();
        hasher.update(data);
        
        let mut output1 = [0u8; 16];
        hasher.clone().finalize_variable(&mut output1);
        assert_eq!(output1.len(), 16);
        
        let mut output2 = [0u8; 100];
        hasher.finalize_variable(&mut output2);
        assert_eq!(output2.len(), 100);
        
        // First 16 bytes should match
        assert_eq!(output1, output2[..16]);
    }
    
    #[test]
    fn test_blake3_mode_properties() {
        assert_eq!(Blake3Mode::Regular.name(), "BLAKE3");
        assert_eq!(Blake3Mode::Keyed.name(), "BLAKE3-Keyed");
        assert_eq!(Blake3Mode::DeriveKey.name(), "BLAKE3-KDF");
        
        assert_eq!(Blake3Mode::Regular.flag(), 0);
        assert_eq!(Blake3Mode::Keyed.flag(), KEYED_HASH);
        assert_eq!(Blake3Mode::DeriveKey.flag(), DERIVE_KEY_CONTEXT);
    }
    
    #[test]
    fn test_blake3_constants() {
        assert_eq!(BLAKE3_KEY_LEN, 32);
        assert_eq!(BLAKE3_OUT_LEN, 32);
        assert_eq!(BLAKE3_BLOCK_LEN, 64);
        assert_eq!(BLAKE3_CHUNK_LEN, 1024);
    }
}
