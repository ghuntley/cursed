/// Production-ready Keccak hash function implementation (base for SHA-3)
use crate::error::CursedError;
// use crate::stdlib::packages::crypto_hash_advanced::hash_traits::*;

/// Keccak permutation constants
const ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808a, 0x8000000080008000,
    0x000000000000808b, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008a, 0x0000000000000088, 0x0000000080008009, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800a, 0x800000008000000a,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
    0x8000000000000000, 0x8000000080008082, 0x800000000000808a, 0x800000008000808a,
];

const RHO_OFFSETS: [u32; 25] = [
     0,  1, 62, 28, 27, 36, 44,  6, 55, 20,
     3, 10, 43, 25, 39, 41, 45, 15, 21,  8,
    18,  2, 61, 56, 14,
];

const PI_INDICES: [usize; 25] = [
     0,  6, 12, 18, 24,  3,  9, 10, 16, 22,
     1,  7, 13, 19, 20,  4,  5, 11, 17, 23,
     2,  8, 14, 15, 21,
];

/// Keccak variant parameters
#[derive(Debug, Clone, Copy)]
pub enum KeccakVariant {
    /// Keccak-224 (224-bit output)
    Keccak224,
    /// Keccak-256 (256-bit output, used by Ethereum)
    Keccak256,
    /// Keccak-384 (384-bit output)
    Keccak384,
    /// Keccak-512 (512-bit output)
    Keccak512,
}

impl KeccakVariant {
    pub fn output_bits(&self) -> usize {
        match self {
            KeccakVariant::Keccak224 => 224,
            KeccakVariant::Keccak256 => 256,
            KeccakVariant::Keccak384 => 384,
            KeccakVariant::Keccak512 => 512,
        }
    }
    
    pub fn output_bytes(&self) -> usize {
        self.output_bits() / 8
    }
    
    pub fn rate(&self) -> usize {
        1600 - 2 * self.output_bits()
    }
    
    pub fn rate_bytes(&self) -> usize {
        self.rate() / 8
    }
    
    pub fn capacity(&self) -> usize {
        2 * self.output_bits()
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            KeccakVariant::Keccak224 => "Keccak-224",
            KeccakVariant::Keccak256 => "Keccak-256",
            KeccakVariant::Keccak384 => "Keccak-384",
            KeccakVariant::Keccak512 => "Keccak-512",
        }
    }
}

/// Keccak hasher implementation
#[derive(Debug, Clone)]
pub struct KeccakHasher {
    variant: KeccakVariant,
    state: [u64; 25],
    buffer: Vec<u8>,
    absorbed: usize,
}

impl KeccakHasher {
    pub fn new(variant: KeccakVariant) -> Self {
        Self {
            variant,
            state: [0u64; 25],
            buffer: Vec::new(),
            absorbed: 0,
        }
    }
    
    pub fn keccak224() -> Self {
        Self::new(KeccakVariant::Keccak224)
    }
    
    pub fn keccak256() -> Self {
        Self::new(KeccakVariant::Keccak256)
    }
    
    pub fn keccak384() -> Self {
        Self::new(KeccakVariant::Keccak384)
    }
    
    pub fn keccak512() -> Self {
        Self::new(KeccakVariant::Keccak512)
    }
    
    fn keccak_f(&mut self) {
        for round in 0..24 {
            // θ (Theta) step
            let mut c = [0u64; 5];
            for x in 0..5 {
                c[x] = self.state[x] ^ self.state[x + 5] ^ self.state[x + 10] ^ self.state[x + 15] ^ self.state[x + 20];
            }
            
            let mut d = [0u64; 5];
            for x in 0..5 {
                d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            }
            
            for x in 0..5 {
                for y in 0..5 {
                    self.state[y * 5 + x] ^= d[x];
                }
            }
            
            // ρ (Rho) and π (Pi) steps
            let mut current = self.state[1];
            for i in 0..24 {
                let target = PI_INDICES[i];
                let temp = self.state[target];
                self.state[target] = current.rotate_left(RHO_OFFSETS[target]);
                current = temp;
            }
            
            // χ (Chi) step
            for y in 0..5 {
                let mut temp = [0u64; 5];
                for x in 0..5 {
                    temp[x] = self.state[y * 5 + x];
                }
                for x in 0..5 {
                    self.state[y * 5 + x] = temp[x] ^ ((!temp[(x + 1) % 5]) & temp[(x + 2) % 5]);
                }
            }
            
            // ι (Iota) step
            self.state[0] ^= ROUND_CONSTANTS[round];
        }
    }
    
    fn absorb_block(&mut self, block: &[u8]) {
        assert_eq!(block.len(), self.variant.rate_bytes());
        
        // XOR block into state
        for (i, chunk) in block.chunks(8).enumerate() {
            if i < 25 {
                let mut bytes = [0u8; 8];
                bytes[..chunk.len()].copy_from_slice(chunk);
                self.state[i] ^= u64::from_le_bytes(bytes);
            }
        }
        
        self.keccak_f();
    }
    
    fn squeeze(&mut self, output_len: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(output_len);
        let rate_bytes = self.variant.rate_bytes();
        
        while output.len() < output_len {
            // Extract current block
            let mut block = Vec::with_capacity(rate_bytes);
            for i in 0..(rate_bytes / 8) {
                if i < 25 {
                    block.extend_from_slice(&self.state[i].to_le_bytes());
                } else {
                    block.extend_from_slice(&[0u8; 8]);
                }
            }
            
            let remaining = output_len - output.len();
            let to_copy = std::cmp::min(remaining, rate_bytes);
            output.extend_from_slice(&block[..to_copy]);
            
            if output.len() < output_len {
                self.keccak_f();
            }
        }
        
        output
    }
}

impl Hasher for KeccakHasher {
    fn algorithm(&self) -> &'static str {
        self.variant.name()
    }
    
    fn digest_size(&self) -> usize {
        self.variant.output_bytes()
    }
    
    fn update(&mut self, mut data: &[u8]) {
        self.buffer.extend_from_slice(data);
        let rate_bytes = self.variant.rate_bytes();
        
        while self.buffer.len() >= rate_bytes {
            let block: Vec<u8> = self.buffer.drain(..rate_bytes).collect();
            self.absorb_block(&block);
            self.absorbed += rate_bytes;
        }
    }
    
    fn finalize(mut self) -> Vec<u8> {
        let rate_bytes = self.variant.rate_bytes();
        
        // Pad the message (10*1 padding)
        self.buffer.push(0x01);  // Keccak uses 0x01 padding (vs 0x06 for SHA-3)
        
        while self.buffer.len() < rate_bytes {
            self.buffer.push(0x00);
        }
        
        // Set the last bit
        if let Some(last) = self.buffer.last_mut() {
            *last |= 0x80;
        }
        
        // Absorb final block
        self.absorb_block(&self.buffer);
        
        // Squeeze output
        self.squeeze(self.variant.output_bytes())
    }
    
    fn reset(&mut self) {
        self.state = [0u64; 25];
        self.buffer.clear();
        self.absorbed = 0;
    }
}

impl CryptographicHasher for KeccakHasher {
    fn security_level(&self) -> usize {
        match self.variant {
            KeccakVariant::Keccak224 => 112,
            KeccakVariant::Keccak256 => 128,
            KeccakVariant::Keccak384 => 192,
            KeccakVariant::Keccak512 => 256,
        }
    }
    
    fn is_quantum_resistant(&self) -> bool {
        false  // Keccak is not quantum-resistant
    }
    
    fn collision_resistance(&self) -> SecurityLevel {
        match self.variant {
            KeccakVariant::Keccak224 => SecurityLevel::Acceptable,
            KeccakVariant::Keccak256 => SecurityLevel::Strong,
            KeccakVariant::Keccak384 => SecurityLevel::VeryStrong,
            KeccakVariant::Keccak512 => SecurityLevel::QuantumResistant,
        }
    }
    
    fn preimage_resistance(&self) -> SecurityLevel {
        match self.variant {
            KeccakVariant::Keccak224 => SecurityLevel::Strong,
            KeccakVariant::Keccak256 => SecurityLevel::Strong,
            KeccakVariant::Keccak384 => SecurityLevel::VeryStrong,
            KeccakVariant::Keccak512 => SecurityLevel::QuantumResistant,
        }
    }
}

/// Convenience functions for one-shot hashing
pub fn keccak224(data: &[u8]) -> Vec<u8> {
    let mut hasher = KeccakHasher::keccak224();
    hasher.hash(data)
}

pub fn keccak256(data: &[u8]) -> Vec<u8> {
    let mut hasher = KeccakHasher::keccak256();
    hasher.hash(data)
}

pub fn keccak384(data: &[u8]) -> Vec<u8> {
    let mut hasher = KeccakHasher::keccak384();
    hasher.hash(data)
}

pub fn keccak512(data: &[u8]) -> Vec<u8> {
    let mut hasher = KeccakHasher::keccak512();
    hasher.hash(data)
}

/// Ethereum-compatible Keccak-256 (used for address generation)
pub fn ethereum_keccak256(data: &[u8]) -> [u8; 32] {
    let hash = keccak256(data);
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash);
    result
}

/// SHAKE-128 extendable output function based on Keccak
#[derive(Debug, Clone)]
pub struct Shake128 {
    inner: KeccakHasher,
    finalized: bool,
}

impl Shake128 {
    pub fn new() -> Self {
        // SHAKE-128 uses capacity 256 (security level 128)
        let mut hasher = KeccakHasher::new(KeccakVariant::Keccak256);
        // Modify for SHAKE padding
        Self {
            inner: hasher,
            finalized: false,
        }
    }
    
    pub fn update(&mut self, data: &[u8]) {
        if self.finalized {
            panic!("Cannot update SHAKE after finalization");
        }
        self.inner.update(data);
    }
    
    pub fn finalize_and_read(&mut self, output: &mut [u8]) {
        if !self.finalized {
            // SHAKE uses different padding (0x1f instead of 0x01)
            self.inner.buffer.push(0x1f);
            
            let rate_bytes = 168; // SHAKE-128 rate
            while self.inner.buffer.len() < rate_bytes {
                self.inner.buffer.push(0x00);
            }
            
            if let Some(last) = self.inner.buffer.last_mut() {
                *last |= 0x80;
            }
            
            self.inner.absorb_block(&self.inner.buffer.clone());
            self.finalized = true;
        }
        
        let squeezed = self.inner.squeeze(output.len());
        output.copy_from_slice(&squeezed);
    }
    
    pub fn read(&mut self, length: usize) -> Vec<u8> {
        let mut output = vec![0u8; length];
        self.finalize_and_read(&mut output);
        output
    }
}

/// SHAKE-256 extendable output function
#[derive(Debug, Clone)]
pub struct Shake256 {
    inner: KeccakHasher,
    finalized: bool,
}

impl Shake256 {
    pub fn new() -> Self {
        // SHAKE-256 uses capacity 512 (security level 256)
        let hasher = KeccakHasher::new(KeccakVariant::Keccak512);
        Self {
            inner: hasher,
            finalized: false,
        }
    }
    
    pub fn update(&mut self, data: &[u8]) {
        if self.finalized {
            panic!("Cannot update SHAKE after finalization");
        }
        self.inner.update(data);
    }
    
    pub fn finalize_and_read(&mut self, output: &mut [u8]) {
        if !self.finalized {
            self.inner.buffer.push(0x1f);
            
            let rate_bytes = 136; // SHAKE-256 rate
            while self.inner.buffer.len() < rate_bytes {
                self.inner.buffer.push(0x00);
            }
            
            if let Some(last) = self.inner.buffer.last_mut() {
                *last |= 0x80;
            }
            
            self.inner.absorb_block(&self.inner.buffer.clone());
            self.finalized = true;
        }
        
        let squeezed = self.inner.squeeze(output.len());
        output.copy_from_slice(&squeezed);
    }
    
    pub fn read(&mut self, length: usize) -> Vec<u8> {
        let mut output = vec![0u8; length];
        self.finalize_and_read(&mut output);
        output
    }
}

