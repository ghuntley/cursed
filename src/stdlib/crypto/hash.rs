/// fr fr Basic hash functions for CURSED - secure hashing periodt
/// 
/// This module provides fundamental cryptographic hash functions including
/// SHA-256, SHA-512, and MD5 for legacy compatibility. Basic but solid bestie!

use std::fmt;

/// fr fr Common hash trait for all hash algorithms
pub trait HashFunction {
    type Output;
    
    /// slay Initialize new hasher instance
    fn new() -> Self;
    
    /// yolo Update hash with more data
    fn update(&mut self, data: &[u8]);
    
    /// periodt Finalize and get hash result
    fn finalize(self) -> Self::Output;
    
    /// bestie Hash data in one shot
    fn hash(data: &[u8]) -> Self::Output where Self: Sized {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }
    
    /// facts Get algorithm name
    fn algorithm_name(&self) -> &'static str;
    
    /// vibes Get output size in bytes
    fn output_size(&self) -> usize;
}

/// fr fr SHA-256 hash implementation - solid security periodt
#[derive(Debug, Clone)]
pub struct Sha256 {
    state: [u32; 8],
    buffer: Vec<u8>,
    length: u64,
}

impl Sha256 {
    /// SHA-256 initial hash values (first 32 bits of fractional parts of square roots of first 8 primes)
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    
    /// SHA-256 constants (first 32 bits of fractional parts of cube roots of first 64 primes)
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    
    /// bestie Process a 64-byte block
    fn process_block(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];
        
        // Prepare message schedule
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }
        
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        
        // Initialize working variables
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
        
        // Main loop
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(Self::K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        
        // Add working variables to hash value
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

impl HashFunction for Sha256 {
    type Output = [u8; 32];
    
    fn new() -> Self {
        Self {
            state: Self::H0,
            buffer: Vec::new(),
            length: 0,
        }
    }
    
    fn update(&mut self, data: &[u8]) {
        self.length += data.len() as u64;
        self.buffer.extend_from_slice(data);
        
        // Process complete 64-byte blocks
        while self.buffer.len() >= 64 {
            let mut block = [0u8; 64];
            block.copy_from_slice(&self.buffer[..64]);
            self.process_block(&block);
            self.buffer.drain(..64);
        }
    }
    
    fn finalize(mut self) -> Self::Output {
        let bit_length = self.length * 8;
        
        // Add padding
        self.buffer.push(0x80);
        
        // Pad to 56 bytes (448 bits) mod 64
        while self.buffer.len() % 64 != 56 {
            self.buffer.push(0x00);
        }
        
        // Append length as 64-bit big-endian
        self.buffer.extend_from_slice(&bit_length.to_be_bytes());
        
        // Process final block
        if self.buffer.len() == 64 {
            let mut block = [0u8; 64];
            block.copy_from_slice(&self.buffer);
            self.process_block(&block);
        }
        
        // Convert hash state to bytes
        let mut result = [0u8; 32];
        for (i, &word) in self.state.iter().enumerate() {
            result[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
        }
        
        result
    }
    
    fn algorithm_name(&self) -> &'static str {
        "SHA-256"
    }
    
    fn output_size(&self) -> usize {
        32
    }
}

/// fr fr SHA-512 hash implementation - extra strong security periodt
#[derive(Debug, Clone)]
pub struct Sha512 {
    state: [u64; 8],
    buffer: Vec<u8>,
    length: u128,
}

impl Sha512 {
    /// SHA-512 initial hash values (first 64 bits of fractional parts of square roots of first 8 primes)
    const H0: [u64; 8] = [
        0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
        0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
    ];
    
    /// SHA-512 constants (first 64 bits of fractional parts of cube roots of first 80 primes)
    const K: [u64; 80] = [
        0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
        0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
        0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
        0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
        0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
        0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
        0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
        0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
        0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
        0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
        0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
        0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
        0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
        0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
    ];
    
    /// bestie Process a 128-byte block
    fn process_block(&mut self, block: &[u8; 128]) {
        let mut w = [0u64; 80];
        
        // Prepare message schedule
        for i in 0..16 {
            w[i] = u64::from_be_bytes([
                block[i * 8], block[i * 8 + 1], block[i * 8 + 2], block[i * 8 + 3],
                block[i * 8 + 4], block[i * 8 + 5], block[i * 8 + 6], block[i * 8 + 7],
            ]);
        }
        
        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        
        // Initialize working variables
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
        
        // Main loop
        for i in 0..80 {
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(Self::K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        
        // Add working variables to hash value
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

impl HashFunction for Sha512 {
    type Output = [u8; 64];
    
    fn new() -> Self {
        Self {
            state: Self::H0,
            buffer: Vec::new(),
            length: 0,
        }
    }
    
    fn update(&mut self, data: &[u8]) {
        self.length += data.len() as u128;
        self.buffer.extend_from_slice(data);
        
        // Process complete 128-byte blocks
        while self.buffer.len() >= 128 {
            let mut block = [0u8; 128];
            block.copy_from_slice(&self.buffer[..128]);
            self.process_block(&block);
            self.buffer.drain(..128);
        }
    }
    
    fn finalize(mut self) -> Self::Output {
        let bit_length = self.length * 8;
        
        // Add padding
        self.buffer.push(0x80);
        
        // Pad to 112 bytes (896 bits) mod 128
        while self.buffer.len() % 128 != 112 {
            self.buffer.push(0x00);
        }
        
        // Append length as 128-bit big-endian
        self.buffer.extend_from_slice(&bit_length.to_be_bytes());
        
        // Process final block
        if self.buffer.len() == 128 {
            let mut block = [0u8; 128];
            block.copy_from_slice(&self.buffer);
            self.process_block(&block);
        }
        
        // Convert hash state to bytes
        let mut result = [0u8; 64];
        for (i, &word) in self.state.iter().enumerate() {
            result[i * 8..(i + 1) * 8].copy_from_slice(&word.to_be_bytes());
        }
        
        result
    }
    
    fn algorithm_name(&self) -> &'static str {
        "SHA-512"
    }
    
    fn output_size(&self) -> usize {
        64
    }
}

/// fr fr MD5 hash implementation - legacy compatibility only periodt
/// 
/// WARNING: MD5 is cryptographically broken! Only use for legacy compatibility.
/// For real security, use SHA-256 or SHA-512 bestie!
#[derive(Debug, Clone)]
pub struct Md5 {
    state: [u32; 4],
    buffer: Vec<u8>,
    length: u64,
}

impl Md5 {
    /// MD5 initial state
    const H0: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
    
    /// MD5 constants
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];
    
    /// bestie Process a 64-byte block
    fn process_block(&mut self, block: &[u8; 64]) {
        let mut x = [0u32; 16];
        for i in 0..16 {
            x[i] = u32::from_le_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }
        
        let [mut a, mut b, mut c, mut d] = self.state;
        
        // Round 1
        for i in 0..16 {
            let f = (b & c) | (!b & d);
            let g = i;
            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                a.wrapping_add(f)
                    .wrapping_add(x[g])
                    .wrapping_add(Self::K[i])
                    .rotate_left([7, 12, 17, 22][i % 4])
            );
            a = temp;
        }
        
        // Round 2
        for i in 16..32 {
            let f = (d & b) | (!d & c);
            let g = (5 * i + 1) % 16;
            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                a.wrapping_add(f)
                    .wrapping_add(x[g])
                    .wrapping_add(Self::K[i])
                    .rotate_left([5, 9, 14, 20][i % 4])
            );
            a = temp;
        }
        
        // Round 3
        for i in 32..48 {
            let f = b ^ c ^ d;
            let g = (3 * i + 5) % 16;
            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                a.wrapping_add(f)
                    .wrapping_add(x[g])
                    .wrapping_add(Self::K[i])
                    .rotate_left([4, 11, 16, 23][i % 4])
            );
            a = temp;
        }
        
        // Round 4
        for i in 48..64 {
            let f = c ^ (b | !d);
            let g = (7 * i) % 16;
            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                a.wrapping_add(f)
                    .wrapping_add(x[g])
                    .wrapping_add(Self::K[i])
                    .rotate_left([6, 10, 15, 21][i % 4])
            );
            a = temp;
        }
        
        // Add this chunk's hash to result
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }
}

impl HashFunction for Md5 {
    type Output = [u8; 16];
    
    fn new() -> Self {
        Self {
            state: Self::H0,
            buffer: Vec::new(),
            length: 0,
        }
    }
    
    fn update(&mut self, data: &[u8]) {
        self.length += data.len() as u64;
        self.buffer.extend_from_slice(data);
        
        // Process complete 64-byte blocks
        while self.buffer.len() >= 64 {
            let mut block = [0u8; 64];
            block.copy_from_slice(&self.buffer[..64]);
            self.process_block(&block);
            self.buffer.drain(..64);
        }
    }
    
    fn finalize(mut self) -> Self::Output {
        let bit_length = self.length * 8;
        
        // Add padding
        self.buffer.push(0x80);
        
        // Pad to 56 bytes mod 64
        while self.buffer.len() % 64 != 56 {
            self.buffer.push(0x00);
        }
        
        // Append length as 64-bit little-endian
        self.buffer.extend_from_slice(&bit_length.to_le_bytes());
        
        // Process final block
        if self.buffer.len() == 64 {
            let mut block = [0u8; 64];
            block.copy_from_slice(&self.buffer);
            self.process_block(&block);
        }
        
        // Convert hash state to bytes (little-endian for MD5)
        let mut result = [0u8; 16];
        for (i, &word) in self.state.iter().enumerate() {
            result[i * 4..(i + 1) * 4].copy_from_slice(&word.to_le_bytes());
        }
        
        result
    }
    
    fn algorithm_name(&self) -> &'static str {
        "MD5"
    }
    
    fn output_size(&self) -> usize {
        16
    }
}

/// fr fr Hash utilities and helper functions
pub struct HashUtils;

impl HashUtils {
    /// bestie Convert hash bytes to hex string
    pub fn to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
    
    /// vibes Hash string with SHA-256 
    pub fn sha256_string(s: &str) -> String {
        let hash = Sha256::hash(s.as_bytes());
        Self::to_hex(&hash)
    }
    
    /// yolo Hash string with SHA-512
    pub fn sha512_string(s: &str) -> String {
        let hash = Sha512::hash(s.as_bytes());
        Self::to_hex(&hash)
    }
    
    /// sus Hash string with MD5 (legacy only!)
    pub fn md5_string(s: &str) -> String {
        let hash = Md5::hash(s.as_bytes());
        Self::to_hex(&hash)
    }
    
    /// facts Compare two hashes in constant time (prevents timing attacks)
    pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }
}

/// fr fr Supported hash algorithms enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Sha256,
    Sha512, 
    Md5,
}

impl HashAlgorithm {
    /// slay Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::Sha256 => "SHA-256",
            HashAlgorithm::Sha512 => "SHA-512",
            HashAlgorithm::Md5 => "MD5",
        }
    }
    
    /// periodt Get output size in bytes
    pub fn output_size(&self) -> usize {
        match self {
            HashAlgorithm::Sha256 => 32,
            HashAlgorithm::Sha512 => 64,
            HashAlgorithm::Md5 => 16,
        }
    }
    
    /// bestie Check if algorithm is cryptographically secure
    pub fn is_secure(&self) -> bool {
        match self {
            HashAlgorithm::Sha256 | HashAlgorithm::Sha512 => true,
            HashAlgorithm::Md5 => false, // MD5 is broken periodt
        }
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// fr fr Hash result wrapper
#[derive(Debug, Clone)]
pub struct HashResult {
    pub algorithm: HashAlgorithm,
    pub digest: Vec<u8>,
}

impl HashResult {
    /// bestie Create new hash result
    pub fn new(algorithm: HashAlgorithm, digest: Vec<u8>) -> Self {
        Self { algorithm, digest }
    }
    
    /// vibes Get hex representation
    pub fn to_hex(&self) -> String {
        HashUtils::to_hex(&self.digest)
    }
    
    /// facts Get digest length
    pub fn len(&self) -> usize {
        self.digest.len()
    }
}

impl fmt::Display for HashResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.algorithm, self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sha256_empty() {
        let hash = Sha256::hash(b"");
        let expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(HashUtils::to_hex(&hash), expected);
    }
    
    #[test]
    fn test_sha256_abc() {
        let hash = Sha256::hash(b"abc");
        let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        assert_eq!(HashUtils::to_hex(&hash), expected);
    }
    
    #[test]
    fn test_sha512_empty() {
        let hash = Sha512::hash(b"");
        let expected = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
        assert_eq!(HashUtils::to_hex(&hash), expected);
    }
    
    #[test]
    fn test_md5_empty() {
        let hash = Md5::hash(b"");
        let expected = "d41d8cd98f00b204e9800998ecf8427e";
        assert_eq!(HashUtils::to_hex(&hash), expected);
    }
    
    #[test]
    fn test_constant_time_compare() {
        assert!(HashUtils::constant_time_compare(b"hello", b"hello"));
        assert!(!HashUtils::constant_time_compare(b"hello", b"world"));
        assert!(!HashUtils::constant_time_compare(b"hello", b"hi"));
    }
}
