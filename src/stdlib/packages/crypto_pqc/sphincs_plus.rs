/// fr fr SPHINCS+ signature scheme implementation
/// 
/// SPHINCS+ is a stateless hash-based signature scheme that was selected by NIST
/// for standardization. It combines few-time signature schemes (FORS) with a hypertree
/// of many-time signature schemes (WOTS+) to provide a secure, stateless signature system.

use crate::error::CursedError;
// use crate::stdlib::value::Value;
// use crate::stdlib::packages::crypto_pqc::lattice_crypto::{SecureRng, LatticeRng};
// use crate::stdlib::packages::crypto_pqc::hash_crypto::{HashFunction, HashFunctionImpl, HashError};
use std::collections::HashMap;
use std::fmt;

/// fr fr SPHINCS+ configuration parameters
#[derive(Debug, Clone)]
pub struct SphincsConfig {
    pub security_level: SphincsSecurityLevel,
    pub variant: SphincsVariant,
    pub hash_function: HashFunction,
    pub n: usize,           // Security parameter (hash output length)
    pub h: usize,           // Height of the hypertree
    pub d: usize,           // Number of layers in the hypertree
    pub fors_trees: usize,  // Number of FORS trees (k)
    pub fors_height: usize, // Height of each FORS tree (a)
    pub wots_w: u32,        // Winternitz parameter
}

impl SphincsConfig {
    /// slay Create SPHINCS+ config with secure defaults (SPHINCS+-128s)
    pub fn new() -> Self {
        Self {
            security_level: SphincsSecurityLevel::Level128,
            variant: SphincsVariant::Small,
            hash_function: HashFunction::Sha256,
            n: 16,          // 128-bit security
            h: 63,          // Hypertree height
            d: 7,           // Layers
            fors_trees: 10, // k
            fors_height: 12, // a
            wots_w: 16,     // Winternitz parameter
        }
    }
    
    /// bestie Create SPHINCS+ config for specific security level and variant
    pub fn with_params(security_level: SphincsSecurityLevel, variant: SphincsVariant) -> Self {
        match (security_level, variant) {
            (SphincsSecurityLevel::Level128, SphincsVariant::Small) => Self {
                security_level, variant, hash_function: HashFunction::Sha256,
                n: 16, h: 63, d: 7, fors_trees: 10, fors_height: 12, wots_w: 16,
            },
            (SphincsSecurityLevel::Level128, SphincsVariant::Fast) => Self {
                security_level, variant, hash_function: HashFunction::Sha256,
                n: 16, h: 60, d: 12, fors_trees: 14, fors_height: 9, wots_w: 16,
            },
            (SphincsSecurityLevel::Level192, SphincsVariant::Small) => Self {
                security_level, variant, hash_function: HashFunction::Sha256,
                n: 24, h: 63, d: 7, fors_trees: 16, fors_height: 14, wots_w: 16,
            },
            (SphincsSecurityLevel::Level192, SphincsVariant::Fast) => Self {
                security_level, variant, hash_function: HashFunction::Sha256,
                n: 24, h: 66, d: 22, fors_trees: 16, fors_height: 8, wots_w: 16,
            },
            (SphincsSecurityLevel::Level256, SphincsVariant::Small) => Self {
                security_level, variant, hash_function: HashFunction::Sha256,
                n: 32, h: 64, d: 8, fors_trees: 14, fors_height: 22, wots_w: 16,
            },
            (SphincsSecurityLevel::Level256, SphincsVariant::Fast) => Self {
                security_level, variant, hash_function: HashFunction::Sha256,
                n: 32, h: 68, d: 17, fors_trees: 14, fors_height: 13, wots_w: 16,
            },
        }
    }
    
    /// vibes Validate SPHINCS+ configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.n < 16 || self.n > 64 {
            return Err(SphincsError::InvalidConfig("n must be between 16 and 64".to_string()));
        }
        
        if self.h == 0 || self.h > 100 {
            return Err(SphincsError::InvalidConfig("h must be between 1 and 100".to_string()));
        }
        
        if self.d == 0 || self.d > self.h {
            return Err(SphincsError::InvalidConfig("d must be between 1 and h".to_string()));
        }
        
        if self.h % self.d != 0 {
            return Err(SphincsError::InvalidConfig("h must be divisible by d".to_string()));
        }
        
        if self.fors_trees == 0 || self.fors_trees > 64 {
            return Err(SphincsError::InvalidConfig("FORS trees must be between 1 and 64".to_string()));
        }
        
        if self.fors_height == 0 || self.fors_height > 32 {
            return Err(SphincsError::InvalidConfig("FORS height must be between 1 and 32".to_string()));
        }
        
        if !self.wots_w.is_power_of_two() || self.wots_w < 4 || self.wots_w > 256 {
            return Err(SphincsError::InvalidConfig("WOTS w must be power of 2 between 4 and 256".to_string()));
        }
        
        Ok(())
    }
    
    /// periodt Get tree height per layer
    pub fn tree_height(&self) -> usize {
        self.h / self.d
    }
    
    /// sus Calculate signature size
    pub fn signature_size(&self) -> usize {
        // FORS signature size
        let fors_sig_size = self.fors_trees * (self.fors_height + 1) * self.n;
        
        // WOTS+ signatures for each layer
        let wots_len = self.wots_chain_length();
        let wots_sig_size = self.d * wots_len * self.n;
        
        // Authentication paths
        let auth_path_size = self.d * self.tree_height() * self.n;
        
        fors_sig_size + wots_sig_size + auth_path_size
    }
    
    /// facts Calculate WOTS+ chain length
    fn wots_chain_length(&self) -> usize {
        let lg_w = self.wots_w.ilog2() as usize;
        let len1 = (8 * self.n + lg_w - 1) / lg_w;
        let len2 = ((len1 * (self.wots_w as usize - 1)).ilog2() as usize + lg_w) / lg_w;
        len1 + len2
    }
}

impl Default for SphincsConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr SPHINCS+ security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SphincsSecurityLevel {
    Level128, // 128-bit classical security
    Level192, // 192-bit classical security
    Level256, // 256-bit classical security
}

impl SphincsSecurityLevel {
    pub fn bits(&self) -> u32 {
        match self {
            SphincsSecurityLevel::Level128 => 128,
            SphincsSecurityLevel::Level192 => 192,
            SphincsSecurityLevel::Level256 => 256,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            SphincsSecurityLevel::Level128 => "SPHINCS+-128",
            SphincsSecurityLevel::Level192 => "SPHINCS+-192",
            SphincsSecurityLevel::Level256 => "SPHINCS+-256",
        }
    }
}

/// fr fr SPHINCS+ variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SphincsVariant {
    Small, // Smaller signatures, slower signing/verification
    Fast,  // Larger signatures, faster signing/verification
}

impl SphincsVariant {
    pub fn name(&self) -> &'static str {
        match self {
            SphincsVariant::Small => "small",
            SphincsVariant::Fast => "fast",
        }
    }
}

/// fr fr SPHINCS+ engine
#[derive(Debug)]
pub struct SphincsEngine {
    config: SphincsConfig,
    rng: Box<dyn LatticeRng>,
    hasher: HashFunctionImpl,
    fors_engine: ForsEngine,
    wots_engine: WotsEngine,
}

impl SphincsEngine {
    /// slay Create new SPHINCS+ engine
    pub fn new(config: SphincsConfig) -> crate::error::Result<()> {
        config.validate()?;
        
        let rng = Box::new(SecureRng::new()
            .map_err(|e| SphincsError::InitializationError(format!("RNG initialization failed: {}", e)))?);
        let hasher = HashFunctionImpl::new(config.hash_function, config.n)
            .map_err(|e| SphincsError::InitializationError(format!("Hash function initialization failed: {}", e)))?;
        let fors_engine = ForsEngine::new(&config)?;
        let wots_engine = WotsEngine::new(&config)?;
        
        Ok(Self {
            config,
            rng,
            hasher,
            fors_engine,
            wots_engine,
        })
    }
    
    /// bestie Generate SPHINCS+ key pair
    pub fn generate_keypair(&mut self) -> crate::error::Result<()> {
        // Generate random seed SK.seed
        let mut sk_seed = vec![0u8; self.config.n];
        for byte in &mut sk_seed {
            *byte = (self.rng.next_u32() & 0xFF) as u8;
        }
        
        // Generate random SK.prf
        let mut sk_prf = vec![0u8; self.config.n];
        for byte in &mut sk_prf {
            *byte = (self.rng.next_u32() & 0xFF) as u8;
        }
        
        // Generate public seed PK.seed
        let mut pk_seed = vec![0u8; self.config.n];
        for byte in &mut pk_seed {
            *byte = (self.rng.next_u32() & 0xFF) as u8;
        }
        
        // Compute root of top-level tree
        let pk_root = self.compute_root(&sk_seed, &pk_seed)?;
        
        let public_key = SphincsPublicKey {
            pk_seed,
            pk_root,
            config: self.config.clone(),
        };
        
        let private_key = SphincsPrivateKey {
            sk_seed,
            sk_prf,
            pk_seed: public_key.pk_seed.clone(),
            pk_root: public_key.pk_root.clone(),
            config: self.config.clone(),
        };
        
        Ok(SphincsKeyPair {
            public_key,
            private_key,
            config: self.config.clone(),
        })
    }
    
    /// vibes Sign message using SPHINCS+
    pub fn sign(&mut self, message: &[u8], private_key: &SphincsPrivateKey) -> crate::error::Result<()> {
        // Step 1: Generate randomizer
        let randomizer = self.generate_randomizer(message, private_key)?;
        
        // Step 2: Compute message digest
        let digest = self.compute_message_digest(message, &randomizer, &private_key.pk_seed, &private_key.pk_root)?;
        
        // Step 3: Extract indices from digest
        let (tree_index, leaf_index, fors_indices) = self.extract_indices(&digest)?;
        
        // Step 4: Generate FORS signature
        let fors_signature = self.fors_engine.sign(&digest, &fors_indices, &private_key.sk_seed, &private_key.pk_seed)?;
        
        // Step 5: Compute FORS public key
        let fors_pk = self.fors_engine.public_key_from_signature(&digest, &fors_signature, &fors_indices, &private_key.pk_seed)?;
        
        // Step 6: Generate authentication path through hypertree
        let mut auth_path = Vec::new();
        let mut wots_signatures = Vec::new();
        let mut current_pk = fors_pk;
        let mut current_tree_index = tree_index;
        let mut current_leaf_index = leaf_index;
        
        for layer in 0..self.config.d {
            // Generate WOTS+ signature for current public key
            let wots_sk = self.derive_wots_private_key(&private_key.sk_seed, layer, current_tree_index, current_leaf_index)?;
            let wots_sig = self.wots_engine.sign(&current_pk, &wots_sk, &private_key.pk_seed)?;
            wots_signatures.push(wots_sig);
            
            // Generate authentication path for this layer
            let layer_auth_path = self.generate_auth_path(layer, current_tree_index, current_leaf_index, &private_key.sk_seed, &private_key.pk_seed)?;
            auth_path.extend(layer_auth_path);
            
            // Move to parent node for next layer
            if layer < self.config.d - 1 {
                current_pk = self.compute_parent_public_key(&current_pk, layer, current_tree_index, current_leaf_index)?;
                current_tree_index /= 1 << self.config.tree_height();
                current_leaf_index /= 1 << self.config.tree_height();
            }
        }
        
        Ok(SphincsSignature {
            randomizer,
            fors_signature,
            wots_signatures,
            auth_path,
            config: self.config.clone(),
        })
    }
    
    /// periodt Verify SPHINCS+ signature
    pub fn verify(&mut self, message: &[u8], signature: &SphincsSignature, public_key: &SphincsPublicKey) -> crate::error::Result<()> {
        // Step 1: Compute message digest
        let digest = self.compute_message_digest(message, &signature.randomizer, &public_key.pk_seed, &public_key.pk_root)?;
        
        // Step 2: Extract indices from digest
        let (tree_index, leaf_index, fors_indices) = self.extract_indices(&digest)?;
        
        // Step 3: Verify FORS signature and compute FORS public key
        let fors_pk = self.fors_engine.public_key_from_signature(&digest, &signature.fors_signature, &fors_indices, &public_key.pk_seed)?;
        
        // Step 4: Verify authentication path through hypertree
        let mut current_pk = fors_pk;
        let mut current_tree_index = tree_index;
        let mut current_leaf_index = leaf_index;
        let mut auth_path_offset = 0;
        
        for (layer, wots_sig) in signature.wots_signatures.iter().enumerate() {
            // Verify WOTS+ signature
            let wots_pk = self.wots_engine.public_key_from_signature(&current_pk, wots_sig, &public_key.pk_seed)?;
            
            // Verify authentication path for this layer
            let tree_height = self.config.tree_height();
            let layer_auth_path = &signature.auth_path[auth_path_offset..auth_path_offset + tree_height];
            auth_path_offset += tree_height;
            
            let computed_root = self.compute_tree_root(&wots_pk, current_leaf_index, layer_auth_path)?;
            
            if layer == self.config.d - 1 {
                // Check against public key root
                if computed_root != public_key.pk_root {
                    return Ok(false);
                }
            } else {
                // Prepare for next layer
                current_pk = computed_root;
                current_tree_index /= 1 << tree_height;
                current_leaf_index /= 1 << tree_height;
            }
        }
        
        Ok(true)
    }
    
    /// sus Generate randomizer for signatures
    fn generate_randomizer(&mut self, message: &[u8], private_key: &SphincsPrivateKey) -> crate::error::Result<()> {
        // PRF(SK.prf, opt_rand, M) where opt_rand can be random or deterministic
        let mut input = private_key.sk_prf.clone();
        input.extend_from_slice(message);
        
        self.hasher.hash(&input)
            .map_err(|e| SphincsError::SigningError(format!("Failed to generate randomizer: {}", e)))
    }
    
    /// facts Compute message digest
    fn compute_message_digest(&mut self, message: &[u8], randomizer: &[u8], pk_seed: &[u8], pk_root: &[u8]) -> crate::error::Result<()> {
        let mut input = Vec::new();
        input.extend_from_slice(randomizer);
        input.extend_from_slice(pk_seed);
        input.extend_from_slice(pk_root);
        input.extend_from_slice(message);
        
        self.hasher.hash(&input)
            .map_err(|e| SphincsError::SigningError(format!("Failed to compute message digest: {}", e)))
    }
    
    /// yolo Extract indices from message digest
    fn extract_indices(&self, digest: &[u8]) -> crate::error::Result<()> {
        if digest.len() < 4 {
            return Err(SphincsError::InvalidInput("Digest too short".to_string()));
        }
        
        // Extract tree and leaf indices (simplified)
        let tree_index = u32::from_le_bytes([digest[0], digest[1], digest[2], digest[3]]) as usize;
        let leaf_index = tree_index % (1 << self.config.tree_height());
        let tree_index = tree_index / (1 << self.config.tree_height());
        
        // Extract FORS indices
        let mut fors_indices = Vec::new();
        let fors_mask = (1 << self.config.fors_height) - 1;
        
        for i in 0..self.config.fors_trees {
            let byte_offset = 4 + i * 2;
            if byte_offset + 1 < digest.len() {
                let index = u16::from_le_bytes([digest[byte_offset], digest[byte_offset + 1]]) as usize;
                fors_indices.push(index & fors_mask);
            } else {
                fors_indices.push(0);
            }
        }
        
        Ok((tree_index, leaf_index, fors_indices))
    }
    
    /// stan Compute root of hypertree
    fn compute_root(&mut self, sk_seed: &[u8], pk_seed: &[u8]) -> crate::error::Result<()> {
        // Simplified root computation
        // In practice, this would build the actual hypertree
        let mut input = Vec::new();
        input.extend_from_slice(sk_seed);
        input.extend_from_slice(pk_seed);
        input.extend_from_slice(b"SPHINCS+_ROOT");
        
        self.hasher.hash(&input)
            .map_err(|e| SphincsError::KeyGenerationError(format!("Failed to compute root: {}", e)))
    }
    
    /// bestie Helper methods (simplified implementations)
    fn derive_wots_private_key(&mut self, sk_seed: &[u8], layer: usize, tree: usize, leaf: usize) -> crate::error::Result<()> {
        // Derive WOTS+ private key for specific tree position
        let chain_length = self.config.wots_chain_length();
        let mut wots_sk = Vec::new();
        
        for i in 0..chain_length {
            let mut input = sk_seed.to_vec();
            input.extend_from_slice(&layer.to_le_bytes());
            input.extend_from_slice(&tree.to_le_bytes());
            input.extend_from_slice(&leaf.to_le_bytes());
            input.extend_from_slice(&i.to_le_bytes());
            
            let sk_element = self.hasher.hash(&input)
                .map_err(|e| SphincsError::KeyGenerationError(format!("Failed to derive WOTS key: {}", e)))?;
            wots_sk.push(sk_element);
        }
        
        Ok(wots_sk)
    }
    
    fn generate_auth_path(&mut self, layer: usize, tree: usize, leaf: usize, sk_seed: &[u8], pk_seed: &[u8]) -> crate::error::Result<()> {
        // Generate authentication path for given position (simplified)
        let tree_height = self.config.tree_height();
        let mut auth_path = Vec::new();
        
        for i in 0..tree_height {
            let mut input = sk_seed.to_vec();
            input.extend_from_slice(pk_seed);
            input.extend_from_slice(&layer.to_le_bytes());
            input.extend_from_slice(&tree.to_le_bytes());
            input.extend_from_slice(&leaf.to_le_bytes());
            input.extend_from_slice(&i.to_le_bytes());
            input.extend_from_slice(b"AUTH_PATH");
            
            let auth_node = self.hasher.hash(&input)
                .map_err(|e| SphincsError::SigningError(format!("Failed to generate auth path: {}", e)))?;
            auth_path.push(auth_node);
        }
        
        Ok(auth_path)
    }
    
    fn compute_parent_public_key(&mut self, child_pk: &[u8], layer: usize, tree: usize, leaf: usize) -> crate::error::Result<()> {
        // Compute parent public key (simplified)
        let mut input = child_pk.to_vec();
        input.extend_from_slice(&layer.to_le_bytes());
        input.extend_from_slice(&tree.to_le_bytes());
        input.extend_from_slice(&leaf.to_le_bytes());
        input.extend_from_slice(b"PARENT_PK");
        
        self.hasher.hash(&input)
            .map_err(|e| SphincsError::SigningError(format!("Failed to compute parent PK: {}", e)))
    }
    
    fn compute_tree_root(&mut self, leaf: &[u8], leaf_index: usize, auth_path: &[Vec<u8>]) -> crate::error::Result<()> {
        let mut current = leaf.to_vec();
        let mut index = leaf_index;
        
        for sibling in auth_path {
            let mut input = Vec::new();
            if index % 2 == 0 {
                input.extend_from_slice(&current);
                input.extend_from_slice(sibling);
            } else {
                input.extend_from_slice(sibling);
                input.extend_from_slice(&current);
            }
            
            current = self.hasher.hash(&input)
                .map_err(|e| SphincsError::VerificationError(format!("Failed to compute tree root: {}", e)))?;
            index /= 2;
        }
        
        Ok(current)
    }
    
    /// vibes Get configuration
    pub fn get_config(&self) -> &SphincsConfig {
        &self.config
    }
}

/// fr fr FORS (Forest of Random Subsets) engine
#[derive(Debug)]
pub struct ForsEngine {
    k: usize, // Number of trees
    a: usize, // Height of each tree
    n: usize, // Security parameter
}

impl ForsEngine {
    pub fn new(config: &SphincsConfig) -> crate::error::Result<()> {
        Ok(Self {
            k: config.fors_trees,
            a: config.fors_height,
            n: config.n,
        })
    }
    
    pub fn sign(&mut self, _digest: &[u8], _indices: &[usize], _sk_seed: &[u8], _pk_seed: &[u8]) -> crate::error::Result<()> {
        // FORS signature generation (simplified)
        Ok(vec![vec![0u8; self.n]; self.k])
    }
    
    pub fn public_key_from_signature(&mut self, _digest: &[u8], _signature: &[Vec<u8>], _indices: &[usize], _pk_seed: &[u8]) -> crate::error::Result<()> {
        // Compute FORS public key from signature (simplified)
        Ok(vec![0u8; self.n])
    }
}

/// fr fr WOTS+ engine
#[derive(Debug)]
pub struct WotsEngine {
    w: u32,   // Winternitz parameter
    len: usize, // Chain length
    n: usize,   // Security parameter
}

impl WotsEngine {
    pub fn new(config: &SphincsConfig) -> crate::error::Result<()> {
        Ok(Self {
            w: config.wots_w,
            len: config.wots_chain_length(),
            n: config.n,
        })
    }
    
    pub fn sign(&mut self, _message: &[u8], _private_key: &[Vec<u8>], _pk_seed: &[u8]) -> crate::error::Result<()> {
        // WOTS+ signature generation (simplified)
        Ok(vec![vec![0u8; self.n]; self.len])
    }
    
    pub fn public_key_from_signature(&mut self, _message: &[u8], _signature: &[Vec<u8>], _pk_seed: &[u8]) -> crate::error::Result<()> {
        // Compute WOTS+ public key from signature (simplified)
        Ok(vec![0u8; self.n])
    }
}

/// fr fr SPHINCS+ key pair
#[derive(Debug)]
pub struct SphincsKeyPair {
    pub public_key: SphincsPublicKey,
    pub private_key: SphincsPrivateKey,
    pub config: SphincsConfig,
}

impl SphincsKeyPair {
    /// slay Generate new SPHINCS+ key pair
    pub fn generate(config: &SphincsConfig) -> crate::error::Result<()> {
        let mut engine = SphincsEngine::new(config.clone())?;
        engine.generate_keypair()
    }
    
    /// bestie Sign message with private key
    pub fn sign(&self, message: &[u8]) -> crate::error::Result<()> {
        let mut engine = SphincsEngine::new(self.config.clone())?;
        engine.sign(message, &self.private_key)
    }
    
    /// vibes Verify signature with public key
    pub fn verify(&self, message: &[u8], signature: &SphincsSignature) -> crate::error::Result<()> {
        let mut engine = SphincsEngine::new(self.config.clone())?;
        engine.verify(message, signature, &self.public_key)
    }
}

/// fr fr SPHINCS+ public key
#[derive(Debug, Clone)]
pub struct SphincsPublicKey {
    pub pk_seed: Vec<u8>,
    pub pk_root: Vec<u8>,
    pub config: SphincsConfig,
}

/// fr fr SPHINCS+ private key
#[derive(Debug, Clone)]
pub struct SphincsPrivateKey {
    pub sk_seed: Vec<u8>,
    pub sk_prf: Vec<u8>,
    pub pk_seed: Vec<u8>,
    pub pk_root: Vec<u8>,
    pub config: SphincsConfig,
}

/// fr fr SPHINCS+ signature
#[derive(Debug, Clone)]
pub struct SphincsSignature {
    pub randomizer: Vec<u8>,
    pub fors_signature: Vec<Vec<u8>>,
    pub wots_signatures: Vec<Vec<Vec<u8>>>,
    pub auth_path: Vec<Vec<u8>>,
    pub config: SphincsConfig,
}

/// fr fr SPHINCS+ errors
#[derive(Debug, Clone)]
pub enum SphincsError {
    InvalidConfig(String),
    InitializationError(String),
    KeyGenerationError(String),
    SigningError(String),
    VerificationError(String),
    InvalidInput(String),
}

// impl fmt::Display for SphincsError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             SphincsError::InvalidConfig(msg) => write!(f, "SPHINCS+ configuration error: {}", msg),
//             SphincsError::InitializationError(msg) => write!(f, "SPHINCS+ initialization error: {}", msg),
//             SphincsError::KeyGenerationError(msg) => write!(f, "SPHINCS+ key generation error: {}", msg),
//             SphincsError::SigningError(msg) => write!(f, "SPHINCS+ signing error: {}", msg),
//             SphincsError::VerificationError(msg) => write!(f, "SPHINCS+ verification error: {}", msg),
//             SphincsError::InvalidInput(msg) => write!(f, "SPHINCS+ invalid input: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for SphincsError {}
// 
// impl From<SphincsError> for CursedError {
//     fn from(err: SphincsError) -> Self {
//         CursedError::CryptoError(err.to_string())
//     }
// }

impl From<HashError> for SphincsError {
    fn from(err: HashError) -> Self {
        SphincsError::InitializationError(err.to_string())
    }
}

/// fr fr SPHINCS+ utility functions
pub struct SphincsUtils;

impl SphincsUtils {
    /// slay Get parameter set name
    pub fn parameter_set_name(security_level: SphincsSecurityLevel, variant: SphincsVariant) -> String {
        format!("{}-{}", security_level.name(), variant.name())
    }
    
    /// bestie Validate SPHINCS+ parameters for production
    pub fn validate_for_production(config: &SphincsConfig) -> crate::error::Result<()> {
        let is_secure = config.security_level.bits() >= 128;
        let signature_size = config.signature_size();
        
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        if config.security_level.bits() < 128 {
            warnings.push("Security level below 128 bits".to_string());
        }
        
        if signature_size > 50_000 {
            warnings.push("Very large signature sizes".to_string());
            if config.variant == SphincsVariant::Small {
                recommendations.push("Consider using fast variant for smaller signatures".to_string());
            }
        }
        
        if config.h > 70 {
            warnings.push("Very large hypertree height may affect performance".to_string());
        }
        
        recommendations.push("Use hardware acceleration for hash functions".to_string());
        recommendations.push("Implement constant-time operations".to_string());
        recommendations.push("Consider stateful alternatives for high-volume signing".to_string());
        
        Ok(SphincsSecurityValidation {
            is_secure,
            security_bits: config.security_level.bits(),
            signature_size,
            variant: config.variant,
            parameter_set: Self::parameter_set_name(config.security_level, config.variant),
            estimated_keygen_time_ms: Self::estimate_keygen_time(config),
            estimated_sign_time_ms: Self::estimate_sign_time(config),
            estimated_verify_time_ms: Self::estimate_verify_time(config),
            warnings,
            recommendations,
        })
    }
    
    /// vibes Estimate key generation time
    fn estimate_keygen_time(config: &SphincsConfig) -> u64 {
        // Simplified time estimation based on tree size
        let tree_nodes = (1 << config.h) * config.d;
        (tree_nodes as u64) / 1000 // Rough estimate in milliseconds
    }
    
    /// periodt Estimate signing time
    fn estimate_sign_time(config: &SphincsConfig) -> u64 {
        match config.variant {
            SphincsVariant::Small => 100, // Slower but smaller
            SphincsVariant::Fast => 50,   // Faster signing
        }
    }
    
    /// sus Estimate verification time
    fn estimate_verify_time(config: &SphincsConfig) -> u64 {
        match config.variant {
            SphincsVariant::Small => 10,  // Faster verification
            SphincsVariant::Fast => 20,   // Slower verification
        }
    }
}

/// fr fr SPHINCS+ security validation result
#[derive(Debug, Clone)]
pub struct SphincsSecurityValidation {
    pub is_secure: bool,
    pub security_bits: u32,
    pub signature_size: usize,
    pub variant: SphincsVariant,
    pub parameter_set: String,
    pub estimated_keygen_time_ms: u64,
    pub estimated_sign_time_ms: u64,
    pub estimated_verify_time_ms: u64,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

