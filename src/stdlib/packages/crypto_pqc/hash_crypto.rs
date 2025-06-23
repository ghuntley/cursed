/// fr fr Hash-based cryptography implementation
/// 
/// This module implements hash-based signature schemes including Lamport signatures,
/// Winternitz One-Time Signatures (WOTS), and multi-tree constructions.
/// These schemes rely only on the security of cryptographic hash functions.

use crate::error::CursedError;
use crate::stdlib::value::Value;
use crate::stdlib::packages::crypto_pqc::lattice_crypto::{SecureRng, LatticeRng};
use std::collections::HashMap;
use std::fmt;

/// fr fr Hash-based cryptography configuration
#[derive(Debug, Clone)]
pub struct HashConfig {
    pub scheme_type: HashSchemeType,
    pub hash_function: HashFunction,
    pub security_level: HashSecurityLevel,
    pub winternitz_parameter: u8,    // w parameter for WOTS
    pub tree_height: usize,          // Height for tree-based schemes
    pub hash_output_size: usize,     // Hash output size in bytes
}

impl HashConfig {
    /// slay Create hash config with secure defaults
    pub fn new() -> Self {
        Self {
            scheme_type: HashSchemeType::Wots,
            hash_function: HashFunction::Sha256,
            security_level: HashSecurityLevel::Level128,
            winternitz_parameter: 16,
            tree_height: 20,
            hash_output_size: 32, // SHA-256
        }
    }
    
    /// bestie Create hash config for specific security level
    pub fn with_security_level(security_level: HashSecurityLevel) -> Self {
        let (hash_function, hash_output_size) = match security_level {
            HashSecurityLevel::Level128 => (HashFunction::Sha256, 32),
            HashSecurityLevel::Level192 => (HashFunction::Sha384, 48),
            HashSecurityLevel::Level256 => (HashFunction::Sha512, 64),
        };
        
        Self {
            scheme_type: HashSchemeType::Wots,
            hash_function,
            security_level,
            winternitz_parameter: 16,
            tree_height: 20,
            hash_output_size,
        }
    }
    
    /// vibes Create config for specific scheme type
    pub fn with_scheme(scheme_type: HashSchemeType) -> Self {
        let tree_height = match scheme_type {
            HashSchemeType::Lamport => 1,     // One-time use
            HashSchemeType::Wots => 1,        // One-time use
            HashSchemeType::WotsPlus => 1,    // One-time use
            HashSchemeType::Merkle => 20,     // Multi-use tree
            HashSchemeType::Xmss => 20,       // Extended Merkle
            HashSchemeType::XmssMultiTree => 60, // Multi-tree
        };
        
        Self {
            scheme_type,
            tree_height,
            ..Self::new()
        }
    }
    
    /// periodt Validate hash configuration
    pub fn validate(&self) -> Result<(), Error> {
        if self.winternitz_parameter < 2 || self.winternitz_parameter > 256 {
            return Err(HashError::InvalidConfig("Winternitz parameter must be between 2 and 256".to_string()));
        }
        
        if self.tree_height == 0 || self.tree_height > 64 {
            return Err(HashError::InvalidConfig("Tree height must be between 1 and 64".to_string()));
        }
        
        if self.hash_output_size < 16 || self.hash_output_size > 128 {
            return Err(HashError::InvalidConfig("Hash output size must be between 16 and 128 bytes".to_string()));
        }
        
        // Validate hash function matches output size
        let expected_size = match self.hash_function {
            HashFunction::Sha256 => 32,
            HashFunction::Sha384 => 48,
            HashFunction::Sha512 => 64,
            HashFunction::Blake2b => 64,
            HashFunction::Shake256 => self.hash_output_size, // Variable
        };
        
        if self.hash_function != HashFunction::Shake256 && self.hash_output_size != expected_size {
            return Err(HashError::InvalidConfig("Hash output size doesn't match hash function".to_string()));
        }
        
        Ok(())
    }
    
    /// sus Calculate signature size estimate
    pub fn estimate_signature_size(&self) -> usize {
        match self.scheme_type {
            HashSchemeType::Lamport => 2 * 256 * self.hash_output_size, // Very large
            HashSchemeType::Wots => {
                let t1 = (8 * self.hash_output_size + (self.winternitz_parameter as usize).ilog2() as usize - 1) 
                        / (self.winternitz_parameter as usize).ilog2() as usize;
                let t2 = ((t1 * ((self.winternitz_parameter - 1) as usize).ilog2() as usize + 1) + 
                         (self.winternitz_parameter as usize).ilog2() as usize - 1) / 
                         (self.winternitz_parameter as usize).ilog2() as usize;
                (t1 + t2) * self.hash_output_size
            },
            _ => self.hash_output_size * 32, // Approximate for tree schemes
        }
    }
}

impl Default for HashConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Hash-based scheme types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashSchemeType {
    Lamport,        // Lamport one-time signatures
    Wots,           // Winternitz one-time signatures
    WotsPlus,       // WOTS+ (improved version)
    Merkle,         // Merkle signature tree
    Xmss,           // eXtended Merkle Signature Scheme
    XmssMultiTree,  // XMSS Multi-Tree
}

impl HashSchemeType {
    pub fn name(&self) -> &'static str {
        match self {
            HashSchemeType::Lamport => "Lamport OTS",
            HashSchemeType::Wots => "WOTS",
            HashSchemeType::WotsPlus => "WOTS+",
            HashSchemeType::Merkle => "Merkle Tree",
            HashSchemeType::Xmss => "XMSS",
            HashSchemeType::XmssMultiTree => "XMSS-MT",
        }
    }
    
    pub fn is_one_time(&self) -> bool {
        matches!(self, HashSchemeType::Lamport | HashSchemeType::Wots | HashSchemeType::WotsPlus)
    }
}

/// fr fr Hash functions supported
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashFunction {
    Sha256,
    Sha384,
    Sha512,
    Blake2b,
    Shake256,
}

impl HashFunction {
    pub fn name(&self) -> &'static str {
        match self {
            HashFunction::Sha256 => "SHA-256",
            HashFunction::Sha384 => "SHA-384",
            HashFunction::Sha512 => "SHA-512",
            HashFunction::Blake2b => "BLAKE2b",
            HashFunction::Shake256 => "SHAKE256",
        }
    }
    
    pub fn output_size(&self) -> usize {
        match self {
            HashFunction::Sha256 => 32,
            HashFunction::Sha384 => 48,
            HashFunction::Sha512 => 64,
            HashFunction::Blake2b => 64,
            HashFunction::Shake256 => 32, // Default, can be variable
        }
    }
}

/// fr fr Hash-based security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashSecurityLevel {
    Level128, // 128-bit classical security
    Level192, // 192-bit classical security
    Level256, // 256-bit classical security
}

impl HashSecurityLevel {
    pub fn bits(&self) -> u32 {
        match self {
            HashSecurityLevel::Level128 => 128,
            HashSecurityLevel::Level192 => 192,
            HashSecurityLevel::Level256 => 256,
        }
    }
}

/// fr fr Hash-based cryptography engine
#[derive(Debug)]
pub struct HashEngine {
    config: HashConfig,
    rng: Box<dyn LatticeRng>,
    hasher: HashFunctionImpl,
}

impl HashEngine {
    /// slay Create new hash-based engine
    pub fn new(config: HashConfig) -> Result<(), Error> {
        config.validate()?;
        
        let rng = Box::new(SecureRng::new()
            .map_err(|e| HashError::InitializationError(format!("RNG initialization failed: {}", e)))?);
        let hasher = HashFunctionImpl::new(config.hash_function, config.hash_output_size)?;
        
        Ok(Self {
            config,
            rng,
            hasher,
        })
    }
    
    /// bestie Generate hash-based key pair
    pub fn generate_keypair(&mut self) -> Result<(), Error> {
        match self.config.scheme_type {
            HashSchemeType::Lamport => self.generate_lamport_keypair(),
            HashSchemeType::Wots => self.generate_wots_keypair(),
            HashSchemeType::WotsPlus => self.generate_wots_plus_keypair(),
            HashSchemeType::Merkle => self.generate_merkle_keypair(),
            _ => Err(HashError::UnsupportedScheme(format!("Scheme {} not implemented", self.config.scheme_type.name()))),
        }
    }
    
    /// vibes Generate Lamport key pair
    fn generate_lamport_keypair(&mut self) -> Result<(), Error> {
        let hash_size = self.config.hash_output_size;
        let message_bits = hash_size * 8;
        
        // Generate 2 * message_bits random values for private key
        let mut private_key_data = Vec::new();
        for _ in 0..(2 * message_bits) {
            let mut random_value = vec![0u8; hash_size];
            for byte in &mut random_value {
                *byte = (self.rng.next_u32() & 0xFF) as u8;
            }
            private_key_data.push(random_value);
        }
        
        // Generate public key by hashing each private key element
        let mut public_key_data = Vec::new();
        for private_element in &private_key_data {
            let hash = self.hasher.hash(private_element)?;
            public_key_data.push(hash);
        }
        
        let public_key = HashPublicKey {
            key_data: public_key_data,
            scheme_type: self.config.scheme_type,
            tree_height: 1,
            signature_count: 0,
        };
        
        let private_key = HashPrivateKey {
            key_data: private_key_data,
            scheme_type: self.config.scheme_type,
            signature_count: 0,
            max_signatures: 1, // One-time use
        };
        
        Ok(HashKeyPair {
            public_key,
            private_key,
            config: self.config.clone(),
        })
    }
    
    /// periodt Generate WOTS key pair
    fn generate_wots_keypair(&mut self) -> Result<(), Error> {
        let w = self.config.winternitz_parameter as usize;
        let hash_size = self.config.hash_output_size;
        
        // Calculate WOTS parameters
        let t1 = (8 * hash_size + w.ilog2() as usize - 1) / w.ilog2() as usize;
        let t2 = ((t1 * (w - 1).ilog2() as usize + 1) + w.ilog2() as usize - 1) / w.ilog2() as usize;
        let t = t1 + t2;
        
        // Generate private key: t random strings
        let mut private_key_data = Vec::new();
        for _ in 0..t {
            let mut random_value = vec![0u8; hash_size];
            for byte in &mut random_value {
                *byte = (self.rng.next_u32() & 0xFF) as u8;
            }
            private_key_data.push(random_value);
        }
        
        // Generate public key: hash each private key element (w-1) times
        let mut public_key_data = Vec::new();
        for private_element in &private_key_data {
            let mut current = private_element.clone();
            for _ in 0..(w - 1) {
                current = self.hasher.hash(&current)?;
            }
            public_key_data.push(current);
        }
        
        let public_key = HashPublicKey {
            key_data: public_key_data,
            scheme_type: self.config.scheme_type,
            tree_height: 1,
            signature_count: 0,
        };
        
        let private_key = HashPrivateKey {
            key_data: private_key_data,
            scheme_type: self.config.scheme_type,
            signature_count: 0,
            max_signatures: 1, // One-time use
        };
        
        Ok(HashKeyPair {
            public_key,
            private_key,
            config: self.config.clone(),
        })
    }
    
    /// sus Generate WOTS+ key pair (enhanced version)
    fn generate_wots_plus_keypair(&mut self) -> Result<(), Error> {
        // Similar to WOTS but with improved security properties
        // For now, delegate to WOTS implementation
        self.generate_wots_keypair()
    }
    
    /// facts Generate Merkle tree key pair
    fn generate_merkle_keypair(&mut self) -> Result<(), Error> {
        let tree_height = self.config.tree_height;
        let num_leaves = 1 << tree_height; // 2^height
        
        // Generate WOTS key pairs for each leaf
        let mut leaf_keypairs = Vec::new();
        let original_scheme = self.config.scheme_type;
        self.config.scheme_type = HashSchemeType::Wots; // Use WOTS for leaves
        
        for _ in 0..num_leaves {
            let wots_keypair = self.generate_wots_keypair()?;
            leaf_keypairs.push(wots_keypair);
        }
        
        self.config.scheme_type = original_scheme;
        
        // Build Merkle tree from leaf public keys
        let merkle_tree = self.build_merkle_tree(&leaf_keypairs)?;
        
        let public_key = HashPublicKey {
            key_data: vec![merkle_tree.root.clone()],
            scheme_type: self.config.scheme_type,
            tree_height,
            signature_count: 0,
        };
        
        let private_key = HashPrivateKey {
            key_data: vec![], // Store leaf keypairs separately
            scheme_type: self.config.scheme_type,
            signature_count: 0,
            max_signatures: num_leaves,
        };
        
        Ok(HashKeyPair {
            public_key,
            private_key,
            config: self.config.clone(),
        })
    }
    
    /// yolo Build Merkle tree from leaf key pairs
    fn build_merkle_tree(&mut self, leaf_keypairs: &[HashKeyPair]) -> Result<(), Error> {
        if leaf_keypairs.is_empty() {
            return Err(HashError::TreeError("Cannot build tree from empty leaves".to_string()));
        }
        
        // Extract public key hashes from leaf keypairs
        let mut current_level: Vec<Vec<u8>> = leaf_keypairs.iter()
            .map(|kp| {
                // Hash the public key data to get leaf values
                let concatenated = kp.public_key.key_data.concat();
                self.hasher.hash(&concatenated).unwrap_or_else(|_| vec![0u8; self.config.hash_output_size])
            })
            .collect();
        
        let mut tree_nodes = Vec::new();
        tree_nodes.push(current_level.clone());
        
        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in current_level.chunks(2) {
                let combined = if chunk.len() == 2 {
                    [chunk[0].clone(), chunk[1].clone()].concat()
                } else {
                    // Odd number of nodes, duplicate the last one
                    [chunk[0].clone(), chunk[0].clone()].concat()
                };
                
                let parent_hash = self.hasher.hash(&combined)?;
                next_level.push(parent_hash);
            }
            
            tree_nodes.push(next_level.clone());
            current_level = next_level;
        }
        
        Ok(MerkleTree {
            root: current_level[0].clone(),
            height: tree_nodes.len() - 1,
            nodes: tree_nodes,
        })
    }
    
    /// stan Sign message using hash-based scheme
    pub fn sign(&mut self, message: &[u8], private_key: &mut HashPrivateKey) -> Result<(), Error> {
        if private_key.signature_count >= private_key.max_signatures {
            return Err(HashError::SigningError("Private key exhausted".to_string()));
        }
        
        // Hash the message first
        let message_hash = self.hasher.hash(message)?;
        
        let signature = match self.config.scheme_type {
            HashSchemeType::Lamport => self.sign_lamport(&message_hash, private_key)?,
            HashSchemeType::Wots => self.sign_wots(&message_hash, private_key)?,
            HashSchemeType::WotsPlus => self.sign_wots_plus(&message_hash, private_key)?,
            _ => return Err(HashError::UnsupportedScheme("Signing not implemented for this scheme".to_string())),
        };
        
        private_key.signature_count += 1;
        Ok(signature)
    }
    
    /// bestie Sign with Lamport scheme
    fn sign_lamport(&mut self, message_hash: &[u8], private_key: &HashPrivateKey) -> Result<(), Error> {
        let hash_size = self.config.hash_output_size;
        let mut signature_data = Vec::new();
        
        // For each bit in the message hash, reveal the corresponding private key element
        for (byte_idx, &byte) in message_hash.iter().enumerate() {
            for bit_idx in 0..8 {
                let bit = (byte >> (7 - bit_idx)) & 1;
                let key_index = (byte_idx * 8 + bit_idx) * 2 + bit as usize;
                
                if key_index < private_key.key_data.len() {
                    signature_data.push(private_key.key_data[key_index].clone());
                }
            }
        }
        
        Ok(HashSignature {
            signature_data,
            scheme_type: self.config.scheme_type,
            signature_index: private_key.signature_count,
            auth_path: Vec::new(), // No auth path for one-time schemes
        })
    }
    
    /// vibes Sign with WOTS scheme
    fn sign_wots(&mut self, message_hash: &[u8], private_key: &HashPrivateKey) -> Result<(), Error> {
        let w = self.config.winternitz_parameter as usize;
        let hash_size = self.config.hash_output_size;
        
        // Convert message hash to base-w representation
        let base_w_msg = self.to_base_w(message_hash, w)?;
        
        // Calculate checksum
        let checksum = self.calculate_wots_checksum(&base_w_msg, w)?;
        let checksum_base_w = self.to_base_w(&checksum, w)?;
        
        // Combine message and checksum
        let mut combined = base_w_msg;
        combined.extend(checksum_base_w);
        
        // Generate signature by hashing private key elements
        let mut signature_data = Vec::new();
        for (i, &value) in combined.iter().enumerate() {
            if i < private_key.key_data.len() {
                let mut current = private_key.key_data[i].clone();
                
                // Hash 'value' times
                for _ in 0..value {
                    current = self.hasher.hash(&current)?;
                }
                
                signature_data.push(current);
            }
        }
        
        Ok(HashSignature {
            signature_data,
            scheme_type: self.config.scheme_type,
            signature_index: private_key.signature_count,
            auth_path: Vec::new(),
        })
    }
    
    /// periodt Sign with WOTS+ scheme
    fn sign_wots_plus(&mut self, message_hash: &[u8], private_key: &HashPrivateKey) -> Result<(), Error> {
        // For now, use WOTS implementation
        // Real WOTS+ would include additional security measures
        self.sign_wots(message_hash, private_key)
    }
    
    /// sus Verify signature
    pub fn verify(&mut self, message: &[u8], signature: &HashSignature, public_key: &HashPublicKey) -> Result<(), Error> {
        let message_hash = self.hasher.hash(message)?;
        
        match signature.scheme_type {
            HashSchemeType::Lamport => self.verify_lamport(&message_hash, signature, public_key),
            HashSchemeType::Wots => self.verify_wots(&message_hash, signature, public_key),
            HashSchemeType::WotsPlus => self.verify_wots_plus(&message_hash, signature, public_key),
            _ => Err(HashError::UnsupportedScheme("Verification not implemented for this scheme".to_string())),
        }
    }
    
    /// facts Verify Lamport signature
    fn verify_lamport(&mut self, message_hash: &[u8], signature: &HashSignature, public_key: &HashPublicKey) -> Result<(), Error> {
        if signature.signature_data.len() != message_hash.len() * 8 {
            return Ok(false);
        }
        
        for (byte_idx, &byte) in message_hash.iter().enumerate() {
            for bit_idx in 0..8 {
                let bit = (byte >> (7 - bit_idx)) & 1;
                let sig_index = byte_idx * 8 + bit_idx;
                let key_index = (byte_idx * 8 + bit_idx) * 2 + bit as usize;
                
                if sig_index < signature.signature_data.len() && key_index < public_key.key_data.len() {
                    let signature_hash = self.hasher.hash(&signature.signature_data[sig_index])?;
                    if signature_hash != public_key.key_data[key_index] {
                        return Ok(false);
                    }
                }
            }
        }
        
        Ok(true)
    }
    
    /// yolo Verify WOTS signature
    fn verify_wots(&mut self, message_hash: &[u8], signature: &HashSignature, public_key: &HashPublicKey) -> Result<(), Error> {
        let w = self.config.winternitz_parameter as usize;
        
        // Convert message hash to base-w representation
        let base_w_msg = self.to_base_w(message_hash, w)?;
        let checksum = self.calculate_wots_checksum(&base_w_msg, w)?;
        let checksum_base_w = self.to_base_w(&checksum, w)?;
        
        let mut combined = base_w_msg;
        combined.extend(checksum_base_w);
        
        // Verify each signature element
        for (i, &value) in combined.iter().enumerate() {
            if i < signature.signature_data.len() && i < public_key.key_data.len() {
                let mut current = signature.signature_data[i].clone();
                
                // Hash (w-1-value) times to reach public key
                for _ in value..(w - 1) {
                    current = self.hasher.hash(&current)?;
                }
                
                if current != public_key.key_data[i] {
                    return Ok(false);
                }
            }
        }
        
        Ok(true)
    }
    
    /// stan Verify WOTS+ signature
    fn verify_wots_plus(&mut self, message_hash: &[u8], signature: &HashSignature, public_key: &HashPublicKey) -> Result<(), Error> {
        // For now, use WOTS verification
        self.verify_wots(message_hash, signature, public_key)
    }
    
    /// bestie Convert bytes to base-w representation
    fn to_base_w(&self, input: &[u8], w: usize) -> Result<(), Error> {
        if !w.is_power_of_two() || w < 2 {
            return Err(HashError::InvalidConfig("Winternitz parameter must be power of 2 >= 2".to_string()));
        }
        
        let bits_per_digit = w.ilog2() as usize;
        let mut result = Vec::new();
        
        for &byte in input {
            for i in 0..(8 / bits_per_digit) {
                let shift = 8 - (i + 1) * bits_per_digit;
                let mask = (1 << bits_per_digit) - 1;
                let digit = ((byte >> shift) & mask) as usize;
                result.push(digit);
            }
        }
        
        Ok(result)
    }
    
    /// vibes Calculate WOTS checksum
    fn calculate_wots_checksum(&self, base_w_msg: &[usize], w: usize) -> Result<(), Error> {
        let checksum_value: usize = base_w_msg.iter().map(|&x| w - 1 - x).sum();
        
        // Convert checksum to bytes (simplified)
        let mut checksum_bytes = Vec::new();
        let mut temp = checksum_value;
        
        while temp > 0 {
            checksum_bytes.push((temp & 0xFF) as u8);
            temp >>= 8;
        }
        
        if checksum_bytes.is_empty() {
            checksum_bytes.push(0);
        }
        
        Ok(checksum_bytes)
    }
    
    /// periodt Get configuration
    pub fn get_config(&self) -> &HashConfig {
        &self.config
    }
}

/// fr fr Hash function implementation
#[derive(Debug)]
pub struct HashFunctionImpl {
    function_type: HashFunction,
    output_size: usize,
}

impl HashFunctionImpl {
    pub fn new(function_type: HashFunction, output_size: usize) -> Result<(), Error> {
        Ok(Self {
            function_type,
            output_size,
        })
    }
    
    pub fn hash(&self, input: &[u8]) -> Result<(), Error> {
        // Simplified hash implementation using SHA-256-like structure
        // In production, use proper cryptographic hash functions
        
        let mut hasher = SimpleHasher::new(self.output_size);
        hasher.update(input);
        Ok(hasher.finalize())
    }
}

/// fr fr Simple hasher for demonstration
#[derive(Debug)]
struct SimpleHasher {
    state: Vec<u64>,
    buffer: Vec<u8>,
    output_size: usize,
}

impl SimpleHasher {
    fn new(output_size: usize) -> Self {
        Self {
            state: vec![0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1],
            buffer: Vec::new(),
            output_size,
        }
    }
    
    fn update(&mut self, input: &[u8]) {
        self.buffer.extend_from_slice(input);
    }
    
    fn finalize(mut self) -> Vec<u8> {
        // Simple hash computation (not cryptographically secure!)
        for &byte in &self.buffer {
            for state_word in &mut self.state {
                *state_word = state_word.wrapping_add(byte as u64);
                *state_word = state_word.rotate_left(7);
            }
        }
        
        let mut result = Vec::new();
        for &word in &self.state {
            result.extend_from_slice(&word.to_le_bytes());
            if result.len() >= self.output_size {
                break;
            }
        }
        
        result.truncate(self.output_size);
        result
    }
}

/// fr fr Merkle tree structure
#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Vec<u8>,
    pub height: usize,
    pub nodes: Vec<Vec<Vec<u8>>>,
}

/// fr fr Hash-based key pair
#[derive(Debug)]
pub struct HashKeyPair {
    pub public_key: HashPublicKey,
    pub private_key: HashPrivateKey,
    pub config: HashConfig,
}

impl HashKeyPair {
    /// slay Generate new hash-based key pair
    pub fn generate(config: &HashConfig) -> Result<(), Error> {
        let mut engine = HashEngine::new(config.clone())?;
        engine.generate_keypair()
    }
    
    /// bestie Sign message with private key
    pub fn sign(&self, message: &[u8]) -> Result<(), Error> {
        let mut engine = HashEngine::new(self.config.clone())?;
        let mut private_key = self.private_key.clone();
        engine.sign(message, &mut private_key)
    }
    
    /// vibes Verify signature with public key
    pub fn verify(&self, message: &[u8], signature: &HashSignature) -> Result<(), Error> {
        let mut engine = HashEngine::new(self.config.clone())?;
        engine.verify(message, signature, &self.public_key)
    }
}

/// fr fr Hash-based public key
#[derive(Debug, Clone)]
pub struct HashPublicKey {
    pub key_data: Vec<Vec<u8>>,
    pub scheme_type: HashSchemeType,
    pub tree_height: usize,
    pub signature_count: usize,
}

/// fr fr Hash-based private key
#[derive(Debug, Clone)]
pub struct HashPrivateKey {
    pub key_data: Vec<Vec<u8>>,
    pub scheme_type: HashSchemeType,
    pub signature_count: usize,
    pub max_signatures: usize,
}

/// fr fr Hash-based signature
#[derive(Debug, Clone)]
pub struct HashSignature {
    pub signature_data: Vec<Vec<u8>>,
    pub scheme_type: HashSchemeType,
    pub signature_index: usize,
    pub auth_path: Vec<Vec<u8>>, // For tree-based schemes
}

/// fr fr Hash-based cryptography errors
#[derive(Debug, Clone)]
pub enum HashError {
    InvalidConfig(String),
    InitializationError(String),
    KeyGenerationError(String),
    SigningError(String),
    VerificationError(String),
    TreeError(String),
    UnsupportedScheme(String),
}

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashError::InvalidConfig(msg) => write!(f, "Hash configuration error: {}", msg),
            HashError::InitializationError(msg) => write!(f, "Hash initialization error: {}", msg),
            HashError::KeyGenerationError(msg) => write!(f, "Hash key generation error: {}", msg),
            HashError::SigningError(msg) => write!(f, "Hash signing error: {}", msg),
            HashError::VerificationError(msg) => write!(f, "Hash verification error: {}", msg),
            HashError::TreeError(msg) => write!(f, "Merkle tree error: {}", msg),
            HashError::UnsupportedScheme(msg) => write!(f, "Unsupported hash scheme: {}", msg),
        }
    }
}

impl std::error::Error for HashError {}

impl From<HashError> for CursedError {
    fn from(err: HashError) -> Self {
        CursedError::CryptoError(err.to_string())
    }
}

/// fr fr Hash-based utility functions
pub struct HashUtils;

impl HashUtils {
    /// slay Estimate signature size for given configuration
    pub fn estimate_signature_size(config: &HashConfig) -> usize {
        config.estimate_signature_size()
    }
    
    /// bestie Validate hash-based parameters for production
    pub fn validate_for_production(config: &HashConfig) -> Result<(), Error> {
        let is_secure = config.security_level.bits() >= 128;
        let estimated_signature_size = config.estimate_signature_size();
        
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        if config.security_level.bits() < 128 {
            warnings.push("Security level below 128 bits".to_string());
        }
        
        if estimated_signature_size > 100_000 {
            warnings.push("Very large signature sizes".to_string());
            recommendations.push("Consider using tree-based schemes for multiple signatures".to_string());
        }
        
        if config.scheme_type.is_one_time() {
            warnings.push("One-time signature scheme requires key management".to_string());
            recommendations.push("Use tree-based schemes for multiple signatures".to_string());
        }
        
        recommendations.push("Use cryptographically secure hash functions".to_string());
        recommendations.push("Implement constant-time operations".to_string());
        
        Ok(HashSecurityValidation {
            is_secure,
            security_bits: config.security_level.bits(),
            estimated_signature_size,
            is_one_time: config.scheme_type.is_one_time(),
            max_signatures: if config.scheme_type.is_one_time() { 1 } else { 1 << config.tree_height },
            warnings,
            recommendations,
            scheme_name: config.scheme_type.name().to_string(),
        })
    }
}

/// fr fr Hash security validation result
#[derive(Debug, Clone)]
pub struct HashSecurityValidation {
    pub is_secure: bool,
    pub security_bits: u32,
    pub estimated_signature_size: usize,
    pub is_one_time: bool,
    pub max_signatures: usize,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub scheme_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_config_creation() {
        let config = HashConfig::new();
        assert_eq!(config.scheme_type, HashSchemeType::Wots);
        assert_eq!(config.hash_function, HashFunction::Sha256);
        assert_eq!(config.security_level, HashSecurityLevel::Level128);
        assert_eq!(config.winternitz_parameter, 16);
        
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_hash_config_security_levels() {
        let config128 = HashConfig::with_security_level(HashSecurityLevel::Level128);
        assert_eq!(config128.hash_function, HashFunction::Sha256);
        assert_eq!(config128.hash_output_size, 32);
        
        let config192 = HashConfig::with_security_level(HashSecurityLevel::Level192);
        assert_eq!(config192.hash_function, HashFunction::Sha384);
        assert_eq!(config192.hash_output_size, 48);
        
        let config256 = HashConfig::with_security_level(HashSecurityLevel::Level256);
        assert_eq!(config256.hash_function, HashFunction::Sha512);
        assert_eq!(config256.hash_output_size, 64);
    }
    
    #[test]
    fn test_hash_config_schemes() {
        let lamport_config = HashConfig::with_scheme(HashSchemeType::Lamport);
        assert_eq!(lamport_config.scheme_type, HashSchemeType::Lamport);
        assert_eq!(lamport_config.tree_height, 1);
        
        let merkle_config = HashConfig::with_scheme(HashSchemeType::Merkle);
        assert_eq!(merkle_config.scheme_type, HashSchemeType::Merkle);
        assert_eq!(merkle_config.tree_height, 20);
    }
    
    #[test]
    fn test_hash_config_validation() {
        let mut config = HashConfig::new();
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Invalid Winternitz parameter
        config.winternitz_parameter = 1;
        assert!(config.validate().is_err());
        
        // Reset and test invalid tree height
        config.winternitz_parameter = 16;
        config.tree_height = 0;
        assert!(config.validate().is_err());
        
        // Reset and test invalid hash output size
        config.tree_height = 20;
        config.hash_output_size = 0;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_hash_function_properties() {
        assert_eq!(HashFunction::Sha256.name(), "SHA-256");
        assert_eq!(HashFunction::Sha256.output_size(), 32);
        
        assert_eq!(HashFunction::Sha512.name(), "SHA-512");
        assert_eq!(HashFunction::Sha512.output_size(), 64);
    }
    
    #[test]
    fn test_scheme_type_properties() {
        assert_eq!(HashSchemeType::Lamport.name(), "Lamport OTS");
        assert!(HashSchemeType::Lamport.is_one_time());
        
        assert_eq!(HashSchemeType::Merkle.name(), "Merkle Tree");
        assert!(!HashSchemeType::Merkle.is_one_time());
        
        assert_eq!(HashSchemeType::Xmss.name(), "XMSS");
        assert!(!HashSchemeType::Xmss.is_one_time());
    }
    
    #[test]
    fn test_security_level_bits() {
        assert_eq!(HashSecurityLevel::Level128.bits(), 128);
        assert_eq!(HashSecurityLevel::Level192.bits(), 192);
        assert_eq!(HashSecurityLevel::Level256.bits(), 256);
    }
    
    #[test]
    fn test_signature_size_estimation() {
        let lamport_config = HashConfig::with_scheme(HashSchemeType::Lamport);
        let lamport_size = lamport_config.estimate_signature_size();
        assert!(lamport_size > 10000); // Lamport signatures are very large
        
        let wots_config = HashConfig::with_scheme(HashSchemeType::Wots);
        let wots_size = wots_config.estimate_signature_size();
        assert!(wots_size < lamport_size); // WOTS should be smaller than Lamport
    }
    
    #[test]
    fn test_hash_engine_creation() {
        let config = HashConfig::new();
        let engine = HashEngine::new(config);
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_simple_hasher() {
        let mut hasher = SimpleHasher::new(32);
        hasher.update(b"test message");
        let hash = hasher.finalize();
        
        assert_eq!(hash.len(), 32);
        
        // Test that same input produces same output
        let mut hasher2 = SimpleHasher::new(32);
        hasher2.update(b"test message");
        let hash2 = hasher2.finalize();
        
        assert_eq!(hash, hash2);
    }
    
    #[test]
    fn test_security_validation() {
        let config = HashConfig::new();
        let validation = HashUtils::validate_for_production(&config).unwrap();
        
        assert!(validation.is_secure);
        assert_eq!(validation.security_bits, 128);
        assert!(!validation.recommendations.is_empty());
        assert_eq!(validation.scheme_name, "WOTS");
    }
}
