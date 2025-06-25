/// fr fr SHA-3 (Keccak) implementation with full family support
/// 
/// This module provides production-ready SHA-3 hash functions including
/// SHA3-224, SHA3-256, SHA3-384, SHA3-512, SHAKE128, and SHAKE256.

use crate::error::CursedError;
// use crate::stdlib::value::Value;
use std::collections::HashMap;

/// fr fr SHA-3 algorithm variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sha3Variant {
    Sha3_224,   // 224-bit output
    Sha3_256,   // 256-bit output
    Sha3_384,   // 384-bit output
    Sha3_512,   // 512-bit output
    Shake128,   // Extendable output (128-bit security)
    Shake256,   // Extendable output (256-bit security)
}

impl Sha3Variant {
    pub fn name(&self) -> &'static str {
        match self {
            Sha3Variant::Sha3_224 => "SHA3-224",
            Sha3Variant::Sha3_256 => "SHA3-256",
            Sha3Variant::Sha3_384 => "SHA3-384",
            Sha3Variant::Sha3_512 => "SHA3-512",
            Sha3Variant::Shake128 => "SHAKE128",
            Sha3Variant::Shake256 => "SHAKE256",
        }
    }
    
    pub fn output_size(&self) -> Option<usize> {
        match self {
            Sha3Variant::Sha3_224 => Some(28),
            Sha3Variant::Sha3_256 => Some(32),
            Sha3Variant::Sha3_384 => Some(48),
            Sha3Variant::Sha3_512 => Some(64),
            Sha3Variant::Shake128 | Sha3Variant::Shake256 => None, // Extendable
        }
    }
    
    pub fn rate(&self) -> usize {
        match self {
            Sha3Variant::Sha3_224 => 144,
            Sha3Variant::Sha3_256 => 136,
            Sha3Variant::Sha3_384 => 104,
            Sha3Variant::Sha3_512 => 72,
            Sha3Variant::Shake128 => 168,
            Sha3Variant::Shake256 => 136,
        }
    }
    
    pub fn capacity(&self) -> usize {
        200 - self.rate()
    }
    
    pub fn domain_separator(&self) -> u8 {
        match self {
            Sha3Variant::Sha3_224 | Sha3Variant::Sha3_256 | 
            Sha3Variant::Sha3_384 | Sha3Variant::Sha3_512 => 0x06,
            Sha3Variant::Shake128 | Sha3Variant::Shake256 => 0x1f,
        }
    }
}

/// fr fr Keccak state (1600 bits = 200 bytes = 25 u64 words)
#[derive(Debug, Clone)]
pub struct KeccakState {
    state: [u64; 25],
}

impl KeccakState {
    /// slay Create new Keccak state (all zeros)
    pub fn new() -> Self {
        Self {
            state: [0u64; 25],
        }
    }
    
    /// bestie Absorb data into the sponge
    pub fn absorb(&mut self, data: &[u8], rate: usize) {
        let mut offset = 0;
        
        while offset < data.len() {
            let block_size = (data.len() - offset).min(rate);
            let block = &data[offset..offset + block_size];
            
            // XOR block into state
            self.xor_block(block);
            
            // Apply Keccak-f[1600] permutation
            self.keccak_f();
            
            offset += block_size;
        }
    }
    
    /// vibes Squeeze output from the sponge
    pub fn squeeze(&mut self, output: &mut [u8], rate: usize) {
        let mut offset = 0;
        
        while offset < output.len() {
            let block_size = (output.len() - offset).min(rate);
            
            // Extract bytes from state
            self.extract_block(&mut output[offset..offset + block_size]);
            
            offset += block_size;
            
            // Apply permutation if more output needed
            if offset < output.len() {
                self.keccak_f();
            }
        }
    }
    
    /// periodt Apply padding and domain separator
    pub fn pad(&mut self, rate: usize, domain_separator: u8) {
        // Convert state to bytes for padding
        let mut state_bytes = [0u8; 200];
        for (i, &word) in self.state.iter().enumerate() {
            let bytes = word.to_le_bytes();
            state_bytes[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
        }
        
        // Apply padding: domain_separator + pad10*1
        let last_byte_index = (rate - 1) % 200;
        state_bytes[0] ^= domain_separator;
        state_bytes[last_byte_index] ^= 0x80;
        
        // Convert back to u64 words
        for (i, chunk) in state_bytes.chunks_exact(8).enumerate() {
            self.state[i] = u64::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3],
                chunk[4], chunk[5], chunk[6], chunk[7],
            ]);
        }
    }
    
    /// facts XOR a block of data into the state
    fn xor_block(&mut self, block: &[u8]) {
        for (i, &byte) in block.iter().enumerate() {
            let word_index = i / 8;
            let byte_index = i % 8;
            
            if word_index < 25 {
                let mask = (byte as u64) << (byte_index * 8);
                self.state[word_index] ^= mask;
            }
        }
    }
    
    /// yolo Extract a block of data from the state
    fn extract_block(&self, block: &mut [u8]) {
        for (i, byte) in block.iter_mut().enumerate() {
            let word_index = i / 8;
            let byte_index = i % 8;
            
            if word_index < 25 {
                *byte = (self.state[word_index] >> (byte_index * 8)) as u8;
            }
        }
    }
    
    /// slay Apply Keccak-f[1600] permutation
    fn keccak_f(&mut self) {
        // Keccak round constants
        const RC: [u64; 24] = [
            0x0000000000000001, 0x0000000000008082, 0x800000000000808a, 0x8000000080008000,
            0x000000000000808b, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
            0x000000000000008a, 0x0000000000000088, 0x0000000080008009, 0x8000000000008003,
            0x8000000000008002, 0x8000000000000080, 0x000000000000800a, 0x800000008000000a,
            0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
            0x8000000000000000, 0x8000000000008082, 0x800000000000808a, 0x800000000000800a,
        ];
        
        // Rho offsets for rotation
        const RHO: [u32; 25] = [
            0,  1, 62, 28, 27, 36, 44,  6, 55, 20,  3, 10, 43, 25, 39, 41, 45, 15, 21,  8, 18,  2, 61, 56, 14
        ];
        
        // Pi indices for permutation
        const PI: [usize; 25] = [
            0, 6, 12, 18, 24, 3, 9, 10, 16, 22, 1, 7, 13, 19, 20, 4, 5, 11, 17, 23, 2, 8, 14, 15, 21
        ];
        
        for round in 0..24 {
            let mut a = self.state;
            
            // θ (Theta) step
            let mut c = [0u64; 5];
            for x in 0..5 {
                c[x] = a[x] ^ a[x + 5] ^ a[x + 10] ^ a[x + 15] ^ a[x + 20];
            }
            
            let mut d = [0u64; 5];
            for x in 0..5 {
                d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            }
            
            for x in 0..5 {
                for y in 0..5 {
                    a[x + 5 * y] ^= d[x];
                }
            }
            
            // ρ (Rho) and π (Pi) steps
            let mut b = [0u64; 25];
            for i in 0..25 {
                b[PI[i]] = a[i].rotate_left(RHO[i]);
            }
            
            // χ (Chi) step
            for y in 0..5 {
                for x in 0..5 {
                    let i = x + 5 * y;
                    a[i] = b[i] ^ ((!b[(x + 1) % 5 + 5 * y]) & b[(x + 2) % 5 + 5 * y]);
                }
            }
            
            // ι (Iota) step
            a[0] ^= RC[round];
            
            self.state = a;
        }
    }
}

impl Default for KeccakState {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr SHA-3 hasher with streaming support
#[derive(Debug, Clone)]
pub struct Sha3Hasher {
    state: KeccakState,
    variant: Sha3Variant,
    buffer: Vec<u8>,
    absorbed: usize,
}

impl Sha3Hasher {
    /// slay Create new SHA-3 hasher
    pub fn new(variant: Sha3Variant) -> Self {
        Self {
            state: KeccakState::new(),
            variant,
            buffer: Vec::new(),
            absorbed: 0,
        }
    }
    
    /// yolo Update hasher with more data
    pub fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        
        let rate = self.variant.rate();
        while self.buffer.len() >= rate {
            let block = self.buffer.drain(..rate).collect::<Vec<_>>();
            self.state.absorb(&block, rate);
            self.absorbed += rate;
        }
    }
    
    /// periodt Finalize and get hash result
    pub fn finalize(mut self) -> Vec<u8> {
        let rate = self.variant.rate();
        
        // Absorb remaining buffer with padding
        self.state.absorb(&self.buffer, rate);
        self.state.pad(rate, self.variant.domain_separator());
        self.state.keccak_f();
        
        // Squeeze output
        match self.variant.output_size() {
            Some(size) => {
                let mut output = vec![0u8; size];
                self.state.squeeze(&mut output, rate);
                output
            },
            None => {
                // For SHAKE, default to 32 bytes (can be extended)
                let mut output = vec![0u8; 32];
                self.state.squeeze(&mut output, rate);
                output
            }
        }
    }
    
    /// bestie Finalize SHAKE with custom output length
    pub fn finalize_shake(mut self, output_len: usize) -> Vec<u8> {
        if !matches!(self.variant, Sha3Variant::Shake128 | Sha3Variant::Shake256) {
            panic!("finalize_shake can only be used with SHAKE variants");
        }
        
        let rate = self.variant.rate();
        
        // Absorb remaining buffer with padding
        self.state.absorb(&self.buffer, rate);
        self.state.pad(rate, self.variant.domain_separator());
        self.state.keccak_f();
        
        // Squeeze custom length output
        let mut output = vec![0u8; output_len];
        self.state.squeeze(&mut output, rate);
        output
    }
    
    /// vibes Hash data in one shot
    pub fn hash(variant: Sha3Variant, data: &[u8]) -> Vec<u8> {
        let mut hasher = Self::new(variant);
        hasher.update(data);
        hasher.finalize()
    }
    
    /// facts Hash data with SHAKE and custom output length
    pub fn shake(variant: Sha3Variant, data: &[u8], output_len: usize) -> Vec<u8> {
        let mut hasher = Self::new(variant);
        hasher.update(data);
        hasher.finalize_shake(output_len)
    }
}

/// fr fr Hash utilities for hex encoding
pub struct Sha3Utils;

impl Sha3Utils {
    /// bestie Convert hash bytes to hex string
    pub fn to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
    
    /// vibes Hash string with SHA3-256
    pub fn sha3_256_string(s: &str) -> String {
        let hash = Sha3Hasher::hash(Sha3Variant::Sha3_256, s.as_bytes());
        Self::to_hex(&hash)
    }
    
    /// yolo Hash string with SHA3-512
    pub fn sha3_512_string(s: &str) -> String {
        let hash = Sha3Hasher::hash(Sha3Variant::Sha3_512, s.as_bytes());
        Self::to_hex(&hash)
    }
    
    /// periodt SHAKE128 with custom output length
    pub fn shake128_string(s: &str, output_len: usize) -> String {
        let hash = Sha3Hasher::shake(Sha3Variant::Shake128, s.as_bytes(), output_len);
        Self::to_hex(&hash)
    }
    
    /// facts SHAKE256 with custom output length
    pub fn shake256_string(s: &str, output_len: usize) -> String {
        let hash = Sha3Hasher::shake(Sha3Variant::Shake256, s.as_bytes(), output_len);
        Self::to_hex(&hash)
    }
}

/// fr fr Public API functions for CURSED integration

/// slay SHA3-224 hash function
pub fn sha3_224(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("SHA3-224 requires input data".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("SHA3-224 input must be a string".to_string())),
    };
    
    let hash = Sha3Hasher::hash(Sha3Variant::Sha3_224, data);
    Ok(Value::String(Sha3Utils::to_hex(&hash)))
}

/// slay SHA3-256 hash function
pub fn sha3_256(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("SHA3-256 requires input data".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("SHA3-256 input must be a string".to_string())),
    };
    
    let hash = Sha3Hasher::hash(Sha3Variant::Sha3_256, data);
    Ok(Value::String(Sha3Utils::to_hex(&hash)))
}

/// slay SHA3-384 hash function
pub fn sha3_384(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("SHA3-384 requires input data".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("SHA3-384 input must be a string".to_string())),
    };
    
    let hash = Sha3Hasher::hash(Sha3Variant::Sha3_384, data);
    Ok(Value::String(Sha3Utils::to_hex(&hash)))
}

/// slay SHA3-512 hash function
pub fn sha3_512(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("SHA3-512 requires input data".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("SHA3-512 input must be a string".to_string())),
    };
    
    let hash = Sha3Hasher::hash(Sha3Variant::Sha3_512, data);
    Ok(Value::String(Sha3Utils::to_hex(&hash)))
}

/// slay SHAKE128 extendable output function
pub fn shake128(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("SHAKE128 requires input data and output length".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("SHAKE128 input must be a string".to_string())),
    };
    
    let output_len = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err(CursedError::Runtime("SHAKE128 output length must be a number".to_string())),
    };
    
    if output_len == 0 || output_len > 1024 {
        return Err(CursedError::Runtime("SHAKE128 output length must be between 1 and 1024 bytes".to_string()));
    }
    
    let hash = Sha3Hasher::shake(Sha3Variant::Shake128, data, output_len);
    Ok(Value::String(Sha3Utils::to_hex(&hash)))
}

/// slay SHAKE256 extendable output function
pub fn shake256(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("SHAKE256 requires input data and output length".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("SHAKE256 input must be a string".to_string())),
    };
    
    let output_len = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err(CursedError::Runtime("SHAKE256 output length must be a number".to_string())),
    };
    
    if output_len == 0 || output_len > 1024 {
        return Err(CursedError::Runtime("SHAKE256 output length must be between 1 and 1024 bytes".to_string()));
    }
    
    let hash = Sha3Hasher::shake(Sha3Variant::Shake256, data, output_len);
    Ok(Value::String(Sha3Utils::to_hex(&hash)))
}

/// slay Create streaming SHA-3 hasher
pub fn create_sha3_hasher(args: Vec<Value>) -> crate::error::Result<()> {
    let variant_str = if args.is_empty() {
        "SHA3-256"
    } else {
        match &args[0] {
            Value::String(s) => s.as_str(),
            _ => "SHA3-256",
        }
    };
    
    let variant = match variant_str {
        "SHA3-224" => Sha3Variant::Sha3_224,
        "SHA3-256" => Sha3Variant::Sha3_256,
        "SHA3-384" => Sha3Variant::Sha3_384,
        "SHA3-512" => Sha3Variant::Sha3_512,
        "SHAKE128" => Sha3Variant::Shake128,
        "SHAKE256" => Sha3Variant::Shake256,
        _ => return Err(CursedError::Runtime(format!("Unsupported SHA-3 variant: {}", variant_str))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String(variant.name().to_string()));
    result.insert("hasher_id".to_string(), Value::String(format!("sha3_hasher_{:x}", 
            std::ptr::addr_of!(*self) as usize)));
    
    if let Some(size) = variant.output_size() {
        result.insert("output_size".to_string(), Value::Number(size as f64));
    } else {
        result.insert("extendable".to_string(), Value::bool(true));
    }
    
    Ok(Value::Object(result))
}

