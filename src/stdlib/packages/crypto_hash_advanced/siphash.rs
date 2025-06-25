/// Production-ready SipHash implementation - cryptographically strong keyed hash function
use crate::error::CursedError;
// use crate::stdlib::packages::crypto_hash_advanced::hash_traits::*;
use std::io::Read;

/// SipHash-2-4 implementation (2 compression rounds, 4 finalization rounds)
#[derive(Debug, Clone)]
pub struct SipHash {
    k0: u64,
    k1: u64,
    v0: u64,
    v1: u64,
    v2: u64,
    v3: u64,
    ntail: usize,
    tail: u64,
    length: usize,
    c_rounds: usize,
    d_rounds: usize,
}

impl SipHash {
    /// Create new SipHash with default parameters (2-4)
    pub fn new(key: &[u8; 16]) -> Self {
        Self::new_with_rounds(key, 2, 4)
    }
    
    /// Create SipHash with custom round counts
    pub fn new_with_rounds(key: &[u8; 16], c_rounds: usize, d_rounds: usize) -> Self {
        let k0 = u64::from_le_bytes([key[0], key[1], key[2], key[3], key[4], key[5], key[6], key[7]]);
        let k1 = u64::from_le_bytes([key[8], key[9], key[10], key[11], key[12], key[13], key[14], key[15]]);
        
        let mut hasher = Self {
            k0,
            k1,
            v0: k0 ^ 0x736f6d6570736575,
            v1: k1 ^ 0x646f72616e646f6d,
            v2: k0 ^ 0x6c7967656e657261,
            v3: k1 ^ 0x7465646279746573,
            ntail: 0,
            tail: 0,
            length: 0,
            c_rounds,
            d_rounds,
        };
        
        hasher.reset();
        hasher
    }
    
    /// SipHash-1-3 variant (faster, still secure for hash tables)
    pub fn new_13(key: &[u8; 16]) -> Self {
        Self::new_with_rounds(key, 1, 3)
    }
    
    fn sipround(v: &mut [u64; 4]) {
        v[0] = v[0].wrapping_add(v[1]);
        v[1] = v[1].rotate_left(13);
        v[1] ^= v[0];
        v[0] = v[0].rotate_left(32);
        
        v[2] = v[2].wrapping_add(v[3]);
        v[3] = v[3].rotate_left(16);
        v[3] ^= v[2];
        
        v[0] = v[0].wrapping_add(v[3]);
        v[3] = v[3].rotate_left(21);
        v[3] ^= v[0];
        
        v[2] = v[2].wrapping_add(v[1]);
        v[1] = v[1].rotate_left(17);
        v[1] ^= v[2];
        v[2] = v[2].rotate_left(32);
    }
    
    fn compress(&mut self, m: u64) {
        self.v3 ^= m;
        
        let mut v = [self.v0, self.v1, self.v2, self.v3];
        for _ in 0..self.c_rounds {
            Self::sipround(&mut v);
        }
        self.v0 = v[0];
        self.v1 = v[1];
        self.v2 = v[2];
        self.v3 = v[3];
        
        self.v0 ^= m;
    }
}

impl Hasher for SipHash {
    fn algorithm(&self) -> &'static str {
        match (self.c_rounds, self.d_rounds) {
            (1, 3) => "SipHash-1-3",
            (2, 4) => "SipHash-2-4",
            _ => "SipHash-custom",
        }
    }
    
    fn digest_size(&self) -> usize {
        8
    }
    
    fn update(&mut self, data: &[u8]) {
        self.length += data.len();
        let mut input = data;
        
        // Process any remaining tail bytes first
        if self.ntail > 0 {
            let needed = 8 - self.ntail;
            if input.len() >= needed {
                // Complete the current word
                for i in 0..needed {
                    self.tail |= (input[i] as u64) << ((self.ntail + i) * 8);
                }
                self.compress(self.tail);
                self.ntail = 0;
                self.tail = 0;
                input = &input[needed..];
            } else {
                // Still not enough for a complete word
                for (i, &byte) in input.iter().enumerate() {
                    self.tail |= (byte as u64) << ((self.ntail + i) * 8);
                }
                self.ntail += input.len();
                return;
            }
        }
        
        // Process 8-byte chunks
        while input.len() >= 8 {
            let m = u64::from_le_bytes([
                input[0], input[1], input[2], input[3],
                input[4], input[5], input[6], input[7],
            ]);
            self.compress(m);
            input = &input[8..];
        }
        
        // Handle remaining bytes
        if !input.is_empty() {
            self.tail = 0;
            for (i, &byte) in input.iter().enumerate() {
                self.tail |= (byte as u64) << (i * 8);
            }
            self.ntail = input.len();
        }
    }
    
    fn finalize(mut self) -> Vec<u8> {
        // Add length to the last byte position
        let b = (self.length as u64) << 56;
        
        // Include any remaining tail bytes
        if self.ntail > 0 {
            self.tail |= b;
            self.compress(self.tail);
        } else {
            self.compress(b);
        }
        
        // Finalization
        self.v2 ^= 0xff;
        
        let mut v = [self.v0, self.v1, self.v2, self.v3];
        for _ in 0..self.d_rounds {
            Self::sipround(&mut v);
        }
        
        let result = v[0] ^ v[1] ^ v[2] ^ v[3];
        result.to_le_bytes().to_vec()
    }
    
    fn reset(&mut self) {
        self.v0 = self.k0 ^ 0x736f6d6570736575;
        self.v1 = self.k1 ^ 0x646f72616e646f6d;
        self.v2 = self.k0 ^ 0x6c7967656e657261;
        self.v3 = self.k1 ^ 0x7465646279746573;
        self.ntail = 0;
        self.tail = 0;
        self.length = 0;
    }
}

impl KeyedHasher for SipHash {
    fn set_key(&mut self, key: &[u8]) -> HashResult<()> {
        if key.len() != 16 {
            return Err(CursedError::InvalidArgument(
                format!("SipHash requires exactly 16 bytes key, got {}", key.len())
            ));
        }
        
        let key_array: [u8; 16] = key.try_into()
            .map_err(|_| CursedError::InvalidArgument("Invalid key length".to_string()))?;
        
        self.k0 = u64::from_le_bytes([key[0], key[1], key[2], key[3], key[4], key[5], key[6], key[7]]);
        self.k1 = u64::from_le_bytes([key[8], key[9], key[10], key[11], key[12], key[13], key[14], key[15]]);
        
        self.reset();
        Ok(())
    }
    
    fn key_length(&self) -> usize {
        16
    }
}

/// Convenience function for SipHash-2-4
pub fn siphash_24(data: &[u8], key: &[u8; 16]) -> u64 {
    let mut hasher = SipHash::new(key);
    let result = hasher.hash(data);
    u64::from_le_bytes([
        result[0], result[1], result[2], result[3],
        result[4], result[5], result[6], result[7],
    ])
}

/// Convenience function for SipHash-1-3 (faster variant)
pub fn siphash_13(data: &[u8], key: &[u8; 16]) -> u64 {
    let mut hasher = SipHash::new_13(key);
    let result = hasher.hash(data);
    u64::from_le_bytes([
        result[0], result[1], result[2], result[3],
        result[4], result[5], result[6], result[7],
    ])
}

/// SipHash key generator using system entropy
pub fn generate_siphash_key() -> HashResult<[u8; 16]> {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    
    // Use RandomState to get system entropy
    let mut key = [0u8; 16];
    let hasher1 = RandomState::new().build_hasher();
    let hasher2 = RandomState::new().build_hasher();
    
    // Get two u64 values and combine them
    let val1 = hasher1.finish();
    let val2 = hasher2.finish();
    
    key[..8].copy_from_slice(&val1.to_le_bytes());
    key[8..].copy_from_slice(&val2.to_le_bytes());
    
    Ok(key)
}

/// SipHash variant optimized for hash tables
#[derive(Debug, Clone)]
pub struct SipHashHashMap {
    inner: SipHash,
}

impl SipHashHashMap {
    pub fn new() -> HashResult<Self> {
        let key = generate_siphash_key()?;
        Ok(Self {
            inner: SipHash::new_13(&key), // Use faster 1-3 variant for hash tables
        })
    }
    
    pub fn with_key(key: &[u8; 16]) -> Self {
        Self {
            inner: SipHash::new_13(key),
        }
    }
    
    /// Hash a single u64 value efficiently
    pub fn hash_u64(&mut self, value: u64) -> u64 {
        let bytes = value.to_le_bytes();
        let result = self.inner.hash(&bytes);
        u64::from_le_bytes([
            result[0], result[1], result[2], result[3],
            result[4], result[5], result[6], result[7],
        ])
    }
    
    /// Hash a string efficiently
    pub fn hash_str(&mut self, value: &str) -> u64 {
        let result = self.inner.hash(value.as_bytes());
        u64::from_le_bytes([
            result[0], result[1], result[2], result[3],
            result[4], result[5], result[6], result[7],
        ])
    }
}

impl Hasher for SipHashHashMap {
    fn algorithm(&self) -> &'static str {
        "SipHash-HashMap"
    }
    
    fn digest_size(&self) -> usize {
        self.inner.digest_size()
    }
    
    fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }
    
    fn finalize(self) -> Vec<u8> {
        self.inner.finalize()
    }
    
    fn reset(&mut self) {
        self.inner.reset();
    }
}

