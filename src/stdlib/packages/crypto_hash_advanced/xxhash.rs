/// Production-ready xxHash implementation - fast non-cryptographic hash function
use crate::error::CursedError;
// use crate::stdlib::packages::crypto_hash_advanced::hash_traits::*;
use std::io::Read;

// xxHash constants
const PRIME32_1: u32 = 0x9E3379B1;
const PRIME32_2: u32 = 0x85EBCA77;
const PRIME32_3: u32 = 0xC2B2AE3D;
const PRIME32_4: u32 = 0x27D4EB2F;
const PRIME32_5: u32 = 0x165667B1;

const PRIME64_1: u64 = 0x9E3779B185EBCA87;
const PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
const PRIME64_3: u64 = 0x165667B19E3779F9;
const PRIME64_4: u64 = 0x85EBCA77C2B2AE63;
const PRIME64_5: u64 = 0x27D4EB2F165667C5;

/// xxHash32 hasher implementation
#[derive(Debug, Clone)]
pub struct XxHash32 {
impl XxHash32 {
    pub fn new() -> Self {
        Self::with_seed(0)
    pub fn with_seed(seed: u32) -> Self {
        let mut hasher = Self {
        hasher.reset();
        hasher
    fn xxh32_round(acc: u32, input: u32) -> u32 {
        acc.wrapping_add(input.wrapping_mul(PRIME32_2))
            .rotate_left(13)
            .wrapping_mul(PRIME32_1)
    fn xxh32_finalize(mut h32: u32, len: u64) -> u32 {
        h32 = h32.wrapping_add(len as u32);
        
        h32 ^= h32 >> 15;
        h32 = h32.wrapping_mul(PRIME32_2);
        h32 ^= h32 >> 13;
        h32 = h32.wrapping_mul(PRIME32_3);
        h32 ^= h32 >> 16;
        
        h32
    }
}

impl Hasher for XxHash32 {
    fn algorithm(&self) -> &'static str {
        "xxHash32"
    fn digest_size(&self) -> usize {
        4
    fn update(&mut self, mut data: &[u8]) {
        self.total_len += data.len() as u64;
        
        // Handle leftover memory
        if self.memsize > 0 {
            let fill = 16 - self.memsize;
            if data.len() >= fill {
                self.memory[self.memsize..self.memsize + fill].copy_from_slice(&data[..fill]);
                
                let input1 = u32::from_le_bytes([self.memory[0], self.memory[1], self.memory[2], self.memory[3]]);
                let input2 = u32::from_le_bytes([self.memory[4], self.memory[5], self.memory[6], self.memory[7]]);
                let input3 = u32::from_le_bytes([self.memory[8], self.memory[9], self.memory[10], self.memory[11]]);
                let input4 = u32::from_le_bytes([self.memory[12], self.memory[13], self.memory[14], self.memory[15]]);
                
                self.v1 = Self::xxh32_round(self.v1, input1);
                self.v2 = Self::xxh32_round(self.v2, input2);
                self.v3 = Self::xxh32_round(self.v3, input3);
                self.v4 = Self::xxh32_round(self.v4, input4);
                
                data = &data[fill..];
                self.memsize = 0;
            } else {
                self.memory[self.memsize..self.memsize + data.len()].copy_from_slice(data);
                self.memsize += data.len();
                return;
            }
        }
        
        // Process 16-byte chunks
        while data.len() >= 16 {
            let input1 = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            let input2 = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
            let input3 = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
            let input4 = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
            
            self.v1 = Self::xxh32_round(self.v1, input1);
            self.v2 = Self::xxh32_round(self.v2, input2);
            self.v3 = Self::xxh32_round(self.v3, input3);
            self.v4 = Self::xxh32_round(self.v4, input4);
            
            data = &data[16..];
        // Store remaining bytes
        if !data.is_empty() {
            self.memory[..data.len()].copy_from_slice(data);
            self.memsize = data.len();
        }
    }
    
    fn finalize(mut self) -> Vec<u8> {
        let h32 = if self.total_len >= 16 {
            let mut h32 = self.v1.rotate_left(1)
                .wrapping_add(self.v2.rotate_left(7))
                .wrapping_add(self.v3.rotate_left(12))
                .wrapping_add(self.v4.rotate_left(18));
            
            // Process remaining memory
            let mut remaining = &self.memory[..self.memsize];
            while remaining.len() >= 4 {
                let input = u32::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3]]);
                h32 = h32.wrapping_add(input.wrapping_mul(PRIME32_3))
                    .rotate_left(17)
                    .wrapping_mul(PRIME32_4);
                remaining = &remaining[4..];
            for &byte in remaining {
                h32 = h32.wrapping_add((byte as u32).wrapping_mul(PRIME32_5))
                    .rotate_left(11)
                    .wrapping_mul(PRIME32_1);
            h32
        } else {
            let mut h32 = self.seed.wrapping_add(PRIME32_5);
            
            for &byte in &self.memory[..self.memsize] {
                h32 = h32.wrapping_add((byte as u32).wrapping_mul(PRIME32_5))
                    .rotate_left(11)
                    .wrapping_mul(PRIME32_1);
            h32
        
        Self::xxh32_finalize(h32, self.total_len).to_le_bytes().to_vec()
    fn reset(&mut self) {
        self.v1 = self.seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        self.v2 = self.seed.wrapping_add(PRIME32_2);
        self.v3 = self.seed;
        self.v4 = self.seed.wrapping_sub(PRIME32_1);
        self.total_len = 0;
        self.memory = [0; 16];
        self.memsize = 0;
    }
}

/// xxHash64 hasher implementation - preferred for 64-bit systems
#[derive(Debug, Clone)]
pub struct XxHash64 {
impl XxHash64 {
    pub fn new() -> Self {
        Self::with_seed(0)
    pub fn with_seed(seed: u64) -> Self {
        let mut hasher = Self {
        hasher.reset();
        hasher
    fn xxh64_round(acc: u64, input: u64) -> u64 {
        acc.wrapping_add(input.wrapping_mul(PRIME64_2))
            .rotate_left(31)
            .wrapping_mul(PRIME64_1)
    fn xxh64_merge_round(acc: u64, val: u64) -> u64 {
        let val = Self::xxh64_round(0, val);
        (acc ^ val).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4)
    fn xxh64_finalize(mut h64: u64, len: u64) -> u64 {
        h64 = h64.wrapping_add(len);
        
        h64 ^= h64 >> 33;
        h64 = h64.wrapping_mul(PRIME64_2);
        h64 ^= h64 >> 29;
        h64 = h64.wrapping_mul(PRIME64_3);
        h64 ^= h64 >> 32;
        
        h64
    }
}

impl Hasher for XxHash64 {
    fn algorithm(&self) -> &'static str {
        "xxHash64"
    fn digest_size(&self) -> usize {
        8
    fn update(&mut self, mut data: &[u8]) {
        self.total_len += data.len() as u64;
        
        // Handle leftover memory
        if self.memsize > 0 {
            let fill = 32 - self.memsize;
            if data.len() >= fill {
                self.memory[self.memsize..self.memsize + fill].copy_from_slice(&data[..fill]);
                
                let input1 = u64::from_le_bytes([
                ]);
                let input2 = u64::from_le_bytes([
                ]);
                let input3 = u64::from_le_bytes([
                ]);
                let input4 = u64::from_le_bytes([
                ]);
                
                self.v1 = Self::xxh64_round(self.v1, input1);
                self.v2 = Self::xxh64_round(self.v2, input2);
                self.v3 = Self::xxh64_round(self.v3, input3);
                self.v4 = Self::xxh64_round(self.v4, input4);
                
                data = &data[fill..];
                self.memsize = 0;
            } else {
                self.memory[self.memsize..self.memsize + data.len()].copy_from_slice(data);
                self.memsize += data.len();
                return;
            }
        }
        
        // Process 32-byte chunks
        while data.len() >= 32 {
            let input1 = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
            let input2 = u64::from_le_bytes([data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]]);
            let input3 = u64::from_le_bytes([data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23]]);
            let input4 = u64::from_le_bytes([data[24], data[25], data[26], data[27], data[28], data[29], data[30], data[31]]);
            
            self.v1 = Self::xxh64_round(self.v1, input1);
            self.v2 = Self::xxh64_round(self.v2, input2);
            self.v3 = Self::xxh64_round(self.v3, input3);
            self.v4 = Self::xxh64_round(self.v4, input4);
            
            data = &data[32..];
        // Store remaining bytes
        if !data.is_empty() {
            self.memory[..data.len()].copy_from_slice(data);
            self.memsize = data.len();
        }
    }
    
    fn finalize(mut self) -> Vec<u8> {
        let h64 = if self.total_len >= 32 {
            let mut h64 = self.v1.rotate_left(1)
                .wrapping_add(self.v2.rotate_left(7))
                .wrapping_add(self.v3.rotate_left(12))
                .wrapping_add(self.v4.rotate_left(18));
            
            h64 = Self::xxh64_merge_round(h64, self.v1);
            h64 = Self::xxh64_merge_round(h64, self.v2);
            h64 = Self::xxh64_merge_round(h64, self.v3);
            h64 = Self::xxh64_merge_round(h64, self.v4);
            
            // Process remaining memory
            let mut remaining = &self.memory[..self.memsize];
            while remaining.len() >= 8 {
                let input = u64::from_le_bytes([
                ]);
                h64 = (h64 ^ Self::xxh64_round(0, input))
                    .rotate_left(27)
                    .wrapping_mul(PRIME64_1)
                    .wrapping_add(PRIME64_4);
                remaining = &remaining[8..];
            while remaining.len() >= 4 {
                let input = u32::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3]]) as u64;
                h64 = (h64 ^ (input.wrapping_mul(PRIME64_1)))
                    .rotate_left(23)
                    .wrapping_mul(PRIME64_2)
                    .wrapping_add(PRIME64_3);
                remaining = &remaining[4..];
            for &byte in remaining {
                h64 = (h64 ^ ((byte as u64).wrapping_mul(PRIME64_5)))
                    .rotate_left(11)
                    .wrapping_mul(PRIME64_1);
            h64
        } else {
            let mut h64 = self.seed.wrapping_add(PRIME64_5);
            
            let mut remaining = &self.memory[..self.memsize];
            while remaining.len() >= 4 {
                let input = u32::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3]]) as u64;
                h64 = h64.wrapping_add(input.wrapping_mul(PRIME64_3))
                    .rotate_left(17)
                    .wrapping_mul(PRIME64_4);
                remaining = &remaining[4..];
            for &byte in remaining {
                h64 = h64.wrapping_add((byte as u64).wrapping_mul(PRIME64_5))
                    .rotate_left(11)
                    .wrapping_mul(PRIME64_1);
            h64
        
        Self::xxh64_finalize(h64, self.total_len).to_le_bytes().to_vec()
    fn reset(&mut self) {
        self.v1 = self.seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2);
        self.v2 = self.seed.wrapping_add(PRIME64_2);
        self.v3 = self.seed;
        self.v4 = self.seed.wrapping_sub(PRIME64_1);
        self.total_len = 0;
        self.memory = [0; 32];
        self.memsize = 0;
    }
}

/// Convenience functions for one-shot hashing
pub fn xxhash32(data: &[u8], seed: u32) -> u32 {
    let mut hasher = XxHash32::with_seed(seed);
    let hash_bytes = hasher.hash(data);
    u32::from_le_bytes([hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3]])
pub fn xxhash64(data: &[u8], seed: u64) -> u64 {
    let mut hasher = XxHash64::with_seed(seed);
    let hash_bytes = hasher.hash(data);
    u64::from_le_bytes([
    ])
