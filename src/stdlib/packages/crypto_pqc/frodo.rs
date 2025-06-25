/// fr fr FrodoKEM key encapsulation mechanism implementation
/// 
/// FrodoKEM is a lattice-based key encapsulation mechanism based on the Learning With Errors (LWE)
/// problem. It was a finalist in the NIST Post-Quantum Cryptography standardization process and
/// offers conservative security assumptions with larger key sizes.

use crate::error::CursedError;
// use crate::stdlib::value::Value;
// use crate::stdlib::packages::crypto_pqc::lattice_crypto::{SecureRng, LatticeRng, LatticeError};
use std::collections::HashMap;
use std::fmt;

/// fr fr FrodoKEM configuration parameters
#[derive(Debug, Clone)]
pub struct FrodoConfig {
    pub variant: FrodoVariant,
    pub security_level: FrodoSecurityLevel,
    pub n: usize,           // Matrix dimension
    pub n_bar: usize,       // Compressed dimension
    pub m_bar: usize,       // Message dimension
    pub q: u32,             // Modulus
    pub d: usize,           // Bits extracted per matrix element
    pub b: usize,           // Bits per message element
    pub shake: bool,        // Use SHAKE-128 for matrix generation
}

impl FrodoConfig {
    /// slay Create FrodoKEM config with secure defaults (FrodoKEM-640)
    pub fn new() -> Self {
        Self::frodo640()
    }
    
    /// bestie FrodoKEM-640 parameters (NIST Level 1)
    pub fn frodo640() -> Self {
        Self {
            variant: FrodoVariant::Frodo640,
            security_level: FrodoSecurityLevel::Level128,
            n: 640,
            n_bar: 8,
            m_bar: 8,
            q: 32768,       // 2^15
            d: 15,
            b: 2,
            shake: true,
        }
    }
    
    /// vibes FrodoKEM-976 parameters (NIST Level 3)
    pub fn frodo976() -> Self {
        Self {
            variant: FrodoVariant::Frodo976,
            security_level: FrodoSecurityLevel::Level192,
            n: 976,
            n_bar: 8,
            m_bar: 8,
            q: 65536,       // 2^16
            d: 16,
            b: 3,
            shake: true,
        }
    }
    
    /// periodt FrodoKEM-1344 parameters (NIST Level 5)
    pub fn frodo1344() -> Self {
        Self {
            variant: FrodoVariant::Frodo1344,
            security_level: FrodoSecurityLevel::Level256,
            n: 1344,
            n_bar: 8,
            m_bar: 8,
            q: 65536,       // 2^16
            d: 16,
            b: 4,
            shake: true,
        }
    }
    
    /// sus Create FrodoKEM config for specific variant
    pub fn with_variant(variant: FrodoVariant) -> Self {
        match variant {
            FrodoVariant::Frodo640 => Self::frodo640(),
            FrodoVariant::Frodo976 => Self::frodo976(),
            FrodoVariant::Frodo1344 => Self::frodo1344(),
        }
    }
    
    /// facts Validate FrodoKEM configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.n < 256 || self.n > 2048 {
            return Err(FrodoError::InvalidConfig("n must be between 256 and 2048".to_string()));
        }
        
        if self.n_bar == 0 || self.n_bar > 16 {
            return Err(FrodoError::InvalidConfig("n_bar must be between 1 and 16".to_string()));
        }
        
        if self.m_bar == 0 || self.m_bar > 16 {
            return Err(FrodoError::InvalidConfig("m_bar must be between 1 and 16".to_string()));
        }
        
        if !self.q.is_power_of_two() || self.q < 256 {
            return Err(FrodoError::InvalidConfig("q must be power of 2 >= 256".to_string()));
        }
        
        if self.d == 0 || self.d > 32 {
            return Err(FrodoError::InvalidConfig("d must be between 1 and 32".to_string()));
        }
        
        if self.b == 0 || self.b > 8 {
            return Err(FrodoError::InvalidConfig("b must be between 1 and 8".to_string()));
        }
        
        // Verify that d is consistent with q
        if (1u32 << self.d) > self.q {
            return Err(FrodoError::InvalidConfig("d too large for modulus q".to_string()));
        }
        
        Ok(())
    }
    
    /// yolo Calculate public key size
    pub fn public_key_size(&self) -> usize {
        let seed_a_size = 16; // 128-bit seed
        let b_size = self.n * self.n_bar * self.d / 8;
        seed_a_size + b_size
    }
    
    /// stan Calculate private key size
    pub fn private_key_size(&self) -> usize {
        let s_size = self.n_bar * self.n * self.d / 8;
        let seed_a_size = 16;
        let b_size = self.n * self.n_bar * self.d / 8;
        let s_transpose_size = self.n * self.n_bar * self.d / 8;
        s_size + seed_a_size + b_size + s_transpose_size
    }
    
    /// bestie Calculate ciphertext size
    pub fn ciphertext_size(&self) -> usize {
        let c1_size = self.n_bar * self.n * self.d / 8;
        let c2_size = self.n_bar * self.m_bar * self.d / 8;
        c1_size + c2_size
    }
    
    /// vibes Calculate shared secret size
    pub fn shared_secret_size(&self) -> usize {
        16 // 128 bits for all variants
    }
}

impl Default for FrodoConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr FrodoKEM variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrodoVariant {
    Frodo640,  // NIST Level 1 security
    Frodo976,  // NIST Level 3 security
    Frodo1344, // NIST Level 5 security
}

impl FrodoVariant {
    pub fn name(&self) -> &'static str {
        match self {
            FrodoVariant::Frodo640 => "FrodoKEM-640",
            FrodoVariant::Frodo976 => "FrodoKEM-976",
            FrodoVariant::Frodo1344 => "FrodoKEM-1344",
        }
    }
    
    pub fn parameter_set(&self) -> &'static str {
        match self {
            FrodoVariant::Frodo640 => "FrodoKEM-640-SHAKE",
            FrodoVariant::Frodo976 => "FrodoKEM-976-SHAKE",
            FrodoVariant::Frodo1344 => "FrodoKEM-1344-SHAKE",
        }
    }
}

/// fr fr FrodoKEM security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrodoSecurityLevel {
    Level128, // 128-bit classical security (NIST Level 1)
    Level192, // 192-bit classical security (NIST Level 3)
    Level256, // 256-bit classical security (NIST Level 5)
}

impl FrodoSecurityLevel {
    pub fn bits(&self) -> u32 {
        match self {
            FrodoSecurityLevel::Level128 => 128,
            FrodoSecurityLevel::Level192 => 192,
            FrodoSecurityLevel::Level256 => 256,
        }
    }
}

/// fr fr FrodoKEM engine
#[derive(Debug)]
pub struct FrodoEngine {
    config: FrodoConfig,
    rng: Box<dyn LatticeRng>,
    matrix_generator: MatrixGenerator,
}

impl FrodoEngine {
    /// slay Create new FrodoKEM engine
    pub fn new(config: FrodoConfig) -> crate::error::Result<()> {
        config.validate()?;
        
        let rng = Box::new(SecureRng::new()
            .map_err(|e| FrodoError::InitializationError(format!("RNG initialization failed: {}", e)))?);
        let matrix_generator = MatrixGenerator::new(config.shake, config.n, config.q)?;
        
        Ok(Self {
            config,
            rng,
            matrix_generator,
        })
    }
    
    /// bestie Generate FrodoKEM key pair
    pub fn generate_keypair(&mut self) -> crate::error::Result<()> {
        let n = self.config.n;
        let n_bar = self.config.n_bar;
        let q = self.config.q;
        let d = self.config.d;
        
        // Step 1: Generate random seed for matrix A
        let seed_a = self.generate_seed()?;
        
        // Step 2: Generate matrix A from seed
        let matrix_a = self.matrix_generator.generate_matrix(&seed_a)?;
        
        // Step 3: Sample secret matrix S from error distribution
        let secret_s = self.sample_error_matrix(n_bar, n)?;
        
        // Step 4: Sample error matrix E from error distribution
        let error_e = self.sample_error_matrix(n_bar, n)?;
        
        // Step 5: Compute B = S * A + E (mod q)
        let sa_product = self.matrix_multiply(&secret_s, &matrix_a)?;
        let matrix_b = self.matrix_add(&sa_product, &error_e)?;
        
        // Step 6: Pack B for public key
        let packed_b = self.pack_matrix(&matrix_b, d)?;
        
        // Step 7: Create key pair
        let public_key = FrodoPublicKey {
            seed_a,
            packed_b,
            config: self.config.clone(),
        };
        
        let private_key = FrodoPrivateKey {
            secret_s,
            public_key: public_key.clone(),
            config: self.config.clone(),
        };
        
        Ok(FrodoKeyPair {
            public_key,
            private_key,
            config: self.config.clone(),
        })
    }
    
    /// vibes Encapsulate (generate shared secret and ciphertext)
    pub fn encapsulate(&mut self, public_key: &FrodoPublicKey) -> crate::error::Result<()> {
        let n = self.config.n;
        let n_bar = self.config.n_bar;
        let m_bar = self.config.m_bar;
        let q = self.config.q;
        let d = self.config.d;
        let b = self.config.b;
        
        // Step 1: Generate random message μ
        let message_mu = self.generate_message(m_bar * n_bar * b)?;
        
        // Step 2: Generate matrix A from public seed
        let matrix_a = self.matrix_generator.generate_matrix(&public_key.seed_a)?;
        
        // Step 3: Unpack public key matrix B
        let matrix_b = self.unpack_matrix(&public_key.packed_b, n_bar, n, d)?;
        
        // Step 4: Sample ephemeral secret S' from error distribution
        let secret_s_prime = self.sample_error_matrix(m_bar, n)?;
        
        // Step 5: Sample error matrices E' and E''
        let error_e_prime = self.sample_error_matrix(m_bar, n)?;
        let error_e_double_prime = self.sample_error_matrix(m_bar, n_bar)?;
        
        // Step 6: Compute C1 = S' * A + E' (mod q)
        let s_prime_a = self.matrix_multiply(&secret_s_prime, &matrix_a)?;
        let c1_full = self.matrix_add(&s_prime_a, &error_e_prime)?;
        let c1 = self.pack_matrix(&c1_full, d)?;
        
        // Step 7: Compute C2 = S' * B + E'' + Encode(μ) (mod q)
        let s_prime_b = self.matrix_multiply(&secret_s_prime, &matrix_b)?;
        let temp = self.matrix_add(&s_prime_b, &error_e_double_prime)?;
        let encoded_message = self.encode_message(&message_mu, m_bar, n_bar, q, b)?;
        let c2_full = self.matrix_add(&temp, &encoded_message)?;
        let c2 = self.pack_matrix(&c2_full, d)?;
        
        // Step 8: Derive shared secret from message
        let shared_secret = self.derive_shared_secret(&message_mu)?;
        
        // Step 9: Serialize ciphertext
        let ciphertext = self.serialize_ciphertext(&c1, &c2)?;
        
        Ok((shared_secret, ciphertext))
    }
    
    /// periodt Decapsulate (recover shared secret from ciphertext)
    pub fn decapsulate(&mut self, ciphertext: &[u8], private_key: &FrodoPrivateKey) -> crate::error::Result<()> {
        let n = self.config.n;
        let n_bar = self.config.n_bar;
        let m_bar = self.config.m_bar;
        let q = self.config.q;
        let d = self.config.d;
        let b = self.config.b;
        
        // Step 1: Deserialize ciphertext
        let (c1, c2) = self.deserialize_ciphertext(ciphertext)?;
        
        // Step 2: Unpack ciphertext matrices
        let c1_matrix = self.unpack_matrix(&c1, m_bar, n, d)?;
        let c2_matrix = self.unpack_matrix(&c2, m_bar, n_bar, d)?;
        
        // Step 3: Compute M = C2 - C1 * S (mod q)
        let c1_s = self.matrix_multiply(&c1_matrix, &private_key.secret_s)?;
        let m_matrix = self.matrix_subtract(&c2_matrix, &c1_s)?;
        
        // Step 4: Decode message from M
        let recovered_message = self.decode_message(&m_matrix, m_bar, n_bar, q, b)?;
        
        // Step 5: Derive shared secret from recovered message
        let shared_secret = self.derive_shared_secret(&recovered_message)?;
        
        Ok(shared_secret)
    }
    
    /// sus Generate random seed
    fn generate_seed(&mut self) -> crate::error::Result<()> {
        let mut seed = vec![0u8; 16]; // 128-bit seed
        for byte in &mut seed {
            *byte = (self.rng.next_u32() & 0xFF) as u8;
        }
        Ok(seed)
    }
    
    /// facts Sample error matrix from discrete Gaussian distribution
    fn sample_error_matrix(&mut self, rows: usize, cols: usize) -> crate::error::Result<()> {
        let mut matrix = Vec::new();
        
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                // Sample from discrete Gaussian (simplified using centered binomial)
                let error = self.sample_discrete_gaussian()?;
                row.push(error);
            }
            matrix.push(row);
        }
        
        Ok(FrodoMatrix::new(matrix, self.config.q))
    }
    
    /// yolo Sample from discrete Gaussian distribution
    fn sample_discrete_gaussian(&mut self) -> crate::error::Result<()> {
        // Simplified discrete Gaussian sampling using rejection method
        // In practice, use proper discrete Gaussian sampling
        
        // Use centered binomial distribution as approximation
        let mut sum = 0i32;
        for _ in 0..8 { // Number of bits for centered binomial
            let bit1 = (self.rng.next_u32() & 1) as i32;
            let bit2 = (self.rng.next_u32() & 1) as i32;
            sum += bit1 - bit2;
        }
        
        // Convert to unsigned and apply modulus
        let result = ((sum % self.config.q as i32) + self.config.q as i32) % self.config.q as i32;
        Ok(result as u32)
    }
    
    /// stan Matrix operations
    fn matrix_multiply(&self, a: &FrodoMatrix, b: &FrodoMatrix) -> crate::error::Result<()> {
        if a.cols() != b.rows() {
            return Err(FrodoError::DimensionError("Matrix dimensions don't match for multiplication".to_string()));
        }
        
        let rows = a.rows();
        let cols = b.cols();
        let inner = a.cols();
        let mut result = vec![vec![0u32; cols]; rows];
        
        for i in 0..rows {
            for j in 0..cols {
                let mut sum = 0u64;
                for k in 0..inner {
                    sum += (a.get(i, k) as u64) * (b.get(k, j) as u64);
                }
                result[i][j] = (sum % self.config.q as u64) as u32;
            }
        }
        
        Ok(FrodoMatrix::new(result, self.config.q))
    }
    
    fn matrix_add(&self, a: &FrodoMatrix, b: &FrodoMatrix) -> crate::error::Result<()> {
        if a.rows() != b.rows() || a.cols() != b.cols() {
            return Err(FrodoError::DimensionError("Matrix dimensions don't match for addition".to_string()));
        }
        
        let rows = a.rows();
        let cols = a.cols();
        let mut result = vec![vec![0u32; cols]; rows];
        
        for i in 0..rows {
            for j in 0..cols {
                result[i][j] = (a.get(i, j) + b.get(i, j)) % self.config.q;
            }
        }
        
        Ok(FrodoMatrix::new(result, self.config.q))
    }
    
    fn matrix_subtract(&self, a: &FrodoMatrix, b: &FrodoMatrix) -> crate::error::Result<()> {
        if a.rows() != b.rows() || a.cols() != b.cols() {
            return Err(FrodoError::DimensionError("Matrix dimensions don't match for subtraction".to_string()));
        }
        
        let rows = a.rows();
        let cols = a.cols();
        let mut result = vec![vec![0u32; cols]; rows];
        
        for i in 0..rows {
            for j in 0..cols {
                let diff = (a.get(i, j) + self.config.q - b.get(i, j)) % self.config.q;
                result[i][j] = diff;
            }
        }
        
        Ok(FrodoMatrix::new(result, self.config.q))
    }
    
    /// bestie Pack matrix for transmission
    fn pack_matrix(&self, matrix: &FrodoMatrix, bits_per_element: usize) -> crate::error::Result<()> {
        let rows = matrix.rows();
        let cols = matrix.cols();
        let total_bits = rows * cols * bits_per_element;
        let total_bytes = (total_bits + 7) / 8;
        let mut packed = vec![0u8; total_bytes];
        let mut bit_offset = 0;
        
        for i in 0..rows {
            for j in 0..cols {
                let value = matrix.get(i, j);
                
                // Pack value into bit stream
                for bit_pos in 0..bits_per_element {
                    if (value >> bit_pos) & 1 != 0 {
                        let byte_index = bit_offset / 8;
                        let bit_index = bit_offset % 8;
                        packed[byte_index] |= 1 << bit_index;
                    }
                    bit_offset += 1;
                }
            }
        }
        
        Ok(packed)
    }
    
    /// vibes Unpack matrix from transmission
    fn unpack_matrix(&self, packed: &[u8], rows: usize, cols: usize, bits_per_element: usize) -> crate::error::Result<()> {
        let expected_bits = rows * cols * bits_per_element;
        let expected_bytes = (expected_bits + 7) / 8;
        
        if packed.len() < expected_bytes {
            return Err(FrodoError::InvalidCiphertext("Packed matrix too short".to_string()));
        }
        
        let mut matrix = vec![vec![0u32; cols]; rows];
        let mut bit_offset = 0;
        
        for i in 0..rows {
            for j in 0..cols {
                let mut value = 0u32;
                
                for bit_pos in 0..bits_per_element {
                    let byte_index = bit_offset / 8;
                    let bit_index = bit_offset % 8;
                    
                    if byte_index < packed.len() && (packed[byte_index] >> bit_index) & 1 != 0 {
                        value |= 1 << bit_pos;
                    }
                    bit_offset += 1;
                }
                
                matrix[i][j] = value;
            }
        }
        
        Ok(FrodoMatrix::new(matrix, self.config.q))
    }
    
    /// periodt Generate random message
    fn generate_message(&mut self, length_bits: usize) -> crate::error::Result<()> {
        let length_bytes = (length_bits + 7) / 8;
        let mut message = vec![0u8; length_bytes];
        
        for byte in &mut message {
            *byte = (self.rng.next_u32() & 0xFF) as u8;
        }
        
        Ok(message)
    }
    
    /// sus Encode message into matrix
    fn encode_message(&self, message: &[u8], rows: usize, cols: usize, q: u32, b: usize) -> crate::error::Result<()> {
        let mut matrix = vec![vec![0u32; cols]; rows];
        let scale_factor = q >> b; // q / 2^b
        let mut bit_index = 0;
        
        for i in 0..rows {
            for j in 0..cols {
                if bit_index / 8 < message.len() {
                    let byte_index = bit_index / 8;
                    let bit_pos = bit_index % 8;
                    let bit = (message[byte_index] >> bit_pos) & 1;
                    matrix[i][j] = (bit as u32) * scale_factor;
                    bit_index += 1;
                }
            }
        }
        
        Ok(FrodoMatrix::new(matrix, q))
    }
    
    /// facts Decode message from matrix
    fn decode_message(&self, matrix: &FrodoMatrix, rows: usize, cols: usize, q: u32, b: usize) -> crate::error::Result<()> {
        let scale_factor = q >> b;
        let threshold = scale_factor / 2;
        let total_bits = rows * cols;
        let total_bytes = (total_bits + 7) / 8;
        let mut message = vec![0u8; total_bytes];
        let mut bit_index = 0;
        
        for i in 0..rows {
            for j in 0..cols {
                let value = matrix.get(i, j);
                let normalized = (value + threshold) / scale_factor;
                let bit = (normalized % 2) as u8;
                
                if bit_index / 8 < message.len() {
                    let byte_index = bit_index / 8;
                    let bit_pos = bit_index % 8;
                    message[byte_index] |= bit << bit_pos;
                }
                bit_index += 1;
            }
        }
        
        Ok(message)
    }
    
    /// yolo Derive shared secret from message
    fn derive_shared_secret(&self, message: &[u8]) -> crate::error::Result<()> {
        // Use a simple hash of the message as shared secret
        // In practice, use a proper key derivation function like SHAKE-256
        let mut hash_input = message.to_vec();
        hash_input.extend_from_slice(b"FRODO_SHARED_SECRET");
        
        // Simplified hash function (use proper cryptographic hash in production)
        let mut secret = vec![0u8; 16]; // 128-bit shared secret
        for (i, &byte) in hash_input.iter().enumerate() {
            secret[i % 16] ^= byte.wrapping_add(i as u8);
        }
        
        Ok(secret)
    }
    
    /// stan Serialize ciphertext
    fn serialize_ciphertext(&self, c1: &[u8], c2: &[u8]) -> crate::error::Result<()> {
        let mut ciphertext = Vec::new();
        ciphertext.extend_from_slice(c1);
        ciphertext.extend_from_slice(c2);
        Ok(ciphertext)
    }
    
    /// bestie Deserialize ciphertext
    fn deserialize_ciphertext(&self, ciphertext: &[u8]) -> crate::error::Result<()> {
        let c1_size = self.config.m_bar * self.config.n * self.config.d / 8;
        let c2_size = self.config.m_bar * self.config.n_bar * self.config.d / 8;
        
        if ciphertext.len() != c1_size + c2_size {
            return Err(FrodoError::InvalidCiphertext("Invalid ciphertext length".to_string()));
        }
        
        let c1 = ciphertext[..c1_size].to_vec();
        let c2 = ciphertext[c1_size..].to_vec();
        
        Ok((c1, c2))
    }
    
    /// vibes Get configuration
    pub fn get_config(&self) -> &FrodoConfig {
        &self.config
    }
}

/// fr fr FrodoKEM matrix representation
#[derive(Debug, Clone)]
pub struct FrodoMatrix {
    data: Vec<Vec<u32>>,
    modulus: u32,
}

impl FrodoMatrix {
    /// slay Create new FrodoKEM matrix
    pub fn new(data: Vec<Vec<u32>>, modulus: u32) -> Self {
        Self { data, modulus }
    }
    
    /// bestie Get matrix element
    pub fn get(&self, row: usize, col: usize) -> u32 {
        self.data[row][col]
    }
    
    /// vibes Set matrix element
    pub fn set(&mut self, row: usize, col: usize, value: u32) {
        self.data[row][col] = value % self.modulus;
    }
    
    /// periodt Get matrix dimensions
    pub fn rows(&self) -> usize {
        self.data.len()
    }
    
    pub fn cols(&self) -> usize {
        if self.data.is_empty() { 0 } else { self.data[0].len() }
    }
}

/// fr fr Matrix generator for FrodoKEM
#[derive(Debug)]
pub struct MatrixGenerator {
    use_shake: bool,
    dimension: usize,
    modulus: u32,
}

impl MatrixGenerator {
    /// slay Create new matrix generator
        Ok(Self {
            use_shake,
            dimension,
            modulus,
        })
    }
    
    /// bestie Generate matrix A from seed
    pub fn generate_matrix(&self, seed: &[u8]) -> crate::error::Result<()> {
        let mut matrix = vec![vec![0u32; self.dimension]; self.dimension];
        
        // Simplified matrix generation using seed as entropy source
        // In practice, use SHAKE-128 or AES for proper pseudorandom generation
        let mut rng_state = self.seed_to_state(seed);
        
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                let value = self.next_pseudorandom(&mut rng_state) % self.modulus;
                matrix[i][j] = value;
            }
        }
        
        Ok(FrodoMatrix::new(matrix, self.modulus))
    }
    
    /// vibes Convert seed to RNG state
    fn seed_to_state(&self, seed: &[u8]) -> u64 {
        let mut state = 0u64;
        for (i, &byte) in seed.iter().enumerate() {
            state ^= (byte as u64) << ((i % 8) * 8);
        }
        if state == 0 { state = 1; } // Avoid zero state
        state
    }
    
    /// periodt Generate next pseudorandom value
    fn next_pseudorandom(&self, state: &mut u64) -> u32 {
        // Simple LCG for demonstration (use proper CSPRNG in production)
        *state = state.wrapping_mul(1103515245).wrapping_add(12345);
        (*state >> 16) as u32
    }
}

/// fr fr FrodoKEM key pair
#[derive(Debug)]
pub struct FrodoKeyPair {
    pub public_key: FrodoPublicKey,
    pub private_key: FrodoPrivateKey,
    pub config: FrodoConfig,
}

impl FrodoKeyPair {
    /// slay Generate new FrodoKEM key pair
    pub fn generate(config: &FrodoConfig) -> crate::error::Result<()> {
        let mut engine = FrodoEngine::new(config.clone())?;
        engine.generate_keypair()
    }
    
    /// bestie Encapsulate shared secret
    pub fn encapsulate(&self) -> crate::error::Result<()> {
        let mut engine = FrodoEngine::new(self.config.clone())?;
        engine.encapsulate(&self.public_key)
    }
    
    /// vibes Decapsulate shared secret
    pub fn decapsulate(&self, ciphertext: &[u8]) -> crate::error::Result<()> {
        let mut engine = FrodoEngine::new(self.config.clone())?;
        engine.decapsulate(ciphertext, &self.private_key)
    }
}

/// fr fr FrodoKEM public key
#[derive(Debug, Clone)]
pub struct FrodoPublicKey {
    pub seed_a: Vec<u8>,
    pub packed_b: Vec<u8>,
    pub config: FrodoConfig,
}

/// fr fr FrodoKEM private key
#[derive(Debug, Clone)]
pub struct FrodoPrivateKey {
    pub secret_s: FrodoMatrix,
    pub public_key: FrodoPublicKey,
    pub config: FrodoConfig,
}

/// fr fr FrodoKEM errors
#[derive(Debug, Clone)]
pub enum FrodoError {
    InvalidConfig(String),
    InitializationError(String),
    KeyGenerationError(String),
    EncapsulationError(String),
    DecapsulationError(String),
    DimensionError(String),
    InvalidCiphertext(String),
    MatrixError(String),
}

// impl fmt::Display for FrodoError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             FrodoError::InvalidConfig(msg) => write!(f, "FrodoKEM configuration error: {}", msg),
//             FrodoError::InitializationError(msg) => write!(f, "FrodoKEM initialization error: {}", msg),
//             FrodoError::KeyGenerationError(msg) => write!(f, "FrodoKEM key generation error: {}", msg),
//             FrodoError::EncapsulationError(msg) => write!(f, "FrodoKEM encapsulation error: {}", msg),
//             FrodoError::DecapsulationError(msg) => write!(f, "FrodoKEM decapsulation error: {}", msg),
//             FrodoError::DimensionError(msg) => write!(f, "FrodoKEM dimension error: {}", msg),
//             FrodoError::InvalidCiphertext(msg) => write!(f, "FrodoKEM invalid ciphertext: {}", msg),
//             FrodoError::MatrixError(msg) => write!(f, "FrodoKEM matrix error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for FrodoError {}
// 
// impl From<FrodoError> for CursedError {
//     fn from(err: FrodoError) -> Self {
//         CursedError::CryptoError(err.to_string())
//     }
// }

impl From<LatticeError> for FrodoError {
    fn from(err: LatticeError) -> Self {
        FrodoError::InitializationError(err.to_string())
    }
}

/// fr fr FrodoKEM utility functions
pub struct FrodoUtils;

impl FrodoUtils {
    /// slay Estimate security level for FrodoKEM parameters
    pub fn estimate_security_level(config: &FrodoConfig) -> f64 {
        // Simplified security estimation based on LWE hardness
        let n = config.n as f64;
        let log_q = (config.q as f64).log2();
        let sigma = 2.8; // Typical error distribution parameter for FrodoKEM
        
        // Rough estimate based on lattice attacks
        // Security decreases with larger q and smaller n
        n.log2() * 20.0 - log_q * 2.0 - sigma.log2() * 5.0
    }
    
    /// bestie Validate FrodoKEM parameters for production
    pub fn validate_for_production(config: &FrodoConfig) -> crate::error::Result<()> {
        let security_bits = Self::estimate_security_level(config);
        let is_secure = security_bits >= 128.0;
        
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        if security_bits < 128.0 {
            warnings.push("Security level below 128 bits".to_string());
            recommendations.push("Use higher security variant".to_string());
        }
        
        let key_sizes = (config.public_key_size(), config.private_key_size());
        if key_sizes.0 > 50000 || key_sizes.1 > 50000 {
            warnings.push("Very large key sizes may affect performance".to_string());
            recommendations.push("Consider alternative PQC schemes for bandwidth-constrained applications".to_string());
        }
        
        if config.n < 640 {
            warnings.push("Small matrix dimension may be vulnerable".to_string());
        }
        
        recommendations.push("Use constant-time matrix operations".to_string());
        recommendations.push("Implement proper discrete Gaussian sampling".to_string());
        recommendations.push("Use SHAKE-128 for matrix generation".to_string());
        
        Ok(FrodoSecurityValidation {
            is_secure,
            estimated_security_bits: security_bits,
            public_key_size: key_sizes.0,
            private_key_size: key_sizes.1,
            ciphertext_size: config.ciphertext_size(),
            shared_secret_size: config.shared_secret_size(),
            variant: config.variant,
            parameter_set: config.variant.parameter_set().to_string(),
            warnings,
            recommendations,
        })
    }
    
    /// vibes Compare FrodoKEM variants
    pub fn compare_variants() -> Vec<FrodoVariantComparison> {
        vec![
            FrodoVariantComparison {
                variant: FrodoVariant::Frodo640,
                security_level: FrodoSecurityLevel::Level128,
                public_key_size: FrodoConfig::frodo640().public_key_size(),
                ciphertext_size: FrodoConfig::frodo640().ciphertext_size(),
                performance_tier: "Fast",
                conservative_security: "High",
            },
            FrodoVariantComparison {
                variant: FrodoVariant::Frodo976,
                security_level: FrodoSecurityLevel::Level192,
                public_key_size: FrodoConfig::frodo976().public_key_size(),
                ciphertext_size: FrodoConfig::frodo976().ciphertext_size(),
                performance_tier: "Medium",
                conservative_security: "Very High",
            },
            FrodoVariantComparison {
                variant: FrodoVariant::Frodo1344,
                security_level: FrodoSecurityLevel::Level256,
                public_key_size: FrodoConfig::frodo1344().public_key_size(),
                ciphertext_size: FrodoConfig::frodo1344().ciphertext_size(),
                performance_tier: "Slow",
                conservative_security: "Maximum",
            },
        ]
    }
    
    /// periodt Estimate performance characteristics
    pub fn estimate_performance(config: &FrodoConfig) -> FrodoPerformanceEstimate {
        let matrix_ops = (config.n * config.n * config.n_bar) as u64;
        
        // Rough estimates based on matrix dimensions
        let keygen_time_ms = matrix_ops / 10000; // Matrix multiplication cost
        let encap_time_ms = matrix_ops / 8000;   // Slightly more expensive
        let decap_time_ms = matrix_ops / 12000;  // Fastest operation
        
        FrodoPerformanceEstimate {
            keygen_time_ms,
            encap_time_ms,
            decap_time_ms,
            memory_usage_kb: (config.n * config.n * 4) / 1024, // Rough estimate
            matrix_operations: matrix_ops,
        }
    }
}

/// fr fr FrodoKEM security validation result
#[derive(Debug, Clone)]
pub struct FrodoSecurityValidation {
    pub is_secure: bool,
    pub estimated_security_bits: f64,
    pub public_key_size: usize,
    pub private_key_size: usize,
    pub ciphertext_size: usize,
    pub shared_secret_size: usize,
    pub variant: FrodoVariant,
    pub parameter_set: String,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

/// fr fr FrodoKEM variant comparison result
#[derive(Debug, Clone)]
pub struct FrodoVariantComparison {
    pub variant: FrodoVariant,
    pub security_level: FrodoSecurityLevel,
    pub public_key_size: usize,
    pub ciphertext_size: usize,
    pub performance_tier: &'static str,
    pub conservative_security: &'static str,
}

/// fr fr FrodoKEM performance estimate
#[derive(Debug, Clone)]
pub struct FrodoPerformanceEstimate {
    pub keygen_time_ms: u64,
    pub encap_time_ms: u64,
    pub decap_time_ms: u64,
    pub memory_usage_kb: usize,
    pub matrix_operations: u64,
}

