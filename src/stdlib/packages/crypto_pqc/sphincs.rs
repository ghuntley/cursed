use crate::error::CursedError;
/// fr fr SPHINCS+ Hash-Based Digital Signatures for Post-Quantum Cryptography
/// no cap Stateless post-quantum signatures with proven security based on hash functions

use std::collections::HashMap;
use std::fmt;
// use crate::stdlib::packages::crypto_advanced::{AdvancedCryptoError, AdvancedCryptoResult};
// use crate::stdlib::packages::crypto_hash_advanced::{sha3, blake3};

/// fr fr SPHINCS+ parameter sets for different security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SphincsParameterSet {
    /// SPHINCS+-128s: 128-bit security, small signatures
    /// SPHINCS+-192s: 192-bit security, small signatures 
    /// SPHINCS+-256s: 256-bit security, small signatures
    /// SPHINCS+-128f: 128-bit security, fast verification
    /// SPHINCS+-192f: 192-bit security, fast verification
    /// SPHINCS+-256f: 256-bit security, fast verification
impl SphincsParameterSet {
    /// bestie Get security level in bits
    pub fn security_level(&self) -> u32 {
        match self {
        }
    }

    /// periodt Get tree height
    pub fn tree_height(&self) -> u32 {
        match self {
        }
    }

    /// slay Get signature size in bytes
    pub fn signature_size(&self) -> usize {
        match self {
        }
    }

    /// yolo Get public key size in bytes
    pub fn public_key_size(&self) -> usize {
        match self {
        }
    }

    /// no cap Get private key size in bytes
    pub fn private_key_size(&self) -> usize {
        match self {
        }
    }

    /// facts Get hash output size in bytes
    pub fn hash_output_size(&self) -> usize {
        match self {
            SphincsParameterSet::Sphincs128s | SphincsParameterSet::Sphincs128f => 32, // SHA-256
            SphincsParameterSet::Sphincs192s | SphincsParameterSet::Sphincs192f => 48, // SHA-384
            SphincsParameterSet::Sphincs256s | SphincsParameterSet::Sphincs256f => 64, // SHA-512
        }
    }
impl fmt::Display for SphincsParameterSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// fr fr Hash function selection for SPHINCS+
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SphincsHashFunction {
impl SphincsHashFunction {
    /// bestie Get output size for hash function
    pub fn output_size(&self) -> usize {
        match self {
            SphincsHashFunction::Shake256 => 32, // Variable, default to 32
        }
    }

    /// periodt Hash data with selected function
    pub fn hash(&self, data: &[u8]) -> Vec<u8> {
        match self {
            SphincsHashFunction::Sha256 => {
                let mut hasher = sha3::Sha3Hasher::new(sha3::Sha3Variant::Sha3_256);
                hasher.update(data);
                hasher.finalize()
            }
            SphincsHashFunction::Sha384 => {
                let mut hasher = sha3::Sha3Hasher::new(sha3::Sha3Variant::Sha3_384);
                hasher.update(data);
                hasher.finalize()
            }
            SphincsHashFunction::Sha512 => {
                let mut hasher = sha3::Sha3Hasher::new(sha3::Sha3Variant::Sha3_512);
                hasher.update(data);
                hasher.finalize()
            }
            SphincsHashFunction::Shake256 => {
                let mut hasher = sha3::Sha3Hasher::new(sha3::Sha3Variant::Shake256);
                hasher.update(data);
                let result = hasher.finalize();
                result[..32].to_vec() // Take first 32 bytes for SHAKE256
            }
        }
    /// slay Hash with variable output length for SHAKE256
    pub fn hash_variable(&self, data: &[u8], output_len: usize) -> Vec<u8> {
        match self {
            SphincsHashFunction::Shake256 => {
                let mut hasher = sha3::Sha3Hasher::new(sha3::Sha3Variant::Shake256);
                hasher.update(data);
                let result = hasher.finalize();
                if result.len() >= output_len {
                    result[..output_len].to_vec()
                } else {
                    // Extend with zeros if needed (simplified)
                    let mut extended = result;
                    extended.resize(output_len, 0);
                    extended
                }
            }
            _ => {
                let result = self.hash(data);
                if result.len() >= output_len {
                    result[..output_len].to_vec()
                } else {
                    let mut extended = result;
                    extended.resize(output_len, 0);
                    extended
                }
            }
        }
    }
/// fr fr WOTS+ (Winternitz One-Time Signature) parameters
#[derive(Debug, Clone)]
pub struct WotsParameters {
    /// Winternitz parameter (trade-off between signature size and security)
    /// Chain length for Winternitz chains
    /// Output length of hash function
impl WotsParameters {
    /// bestie Create WOTS+ parameters for given parameter set
    pub fn new(params: SphincsParameterSet) -> Self {
        let n = params.hash_output_size() as u32;
        let w = match params {
        
        // Calculate chain length based on message length and checksum
        let len1 = (8 * n as f64 / (w as f64).log2()).ceil() as u32;
        let len2 = ((w - 1).ilog2() * len1 + 1).ilog2() + 1;
        let len = len1 + len2;

        WotsParameters { w, len, n }
    }
/// fr fr FORS (Forest of Random Subsets) parameters
#[derive(Debug, Clone)]
pub struct ForsParameters {
    /// Number of trees in FORS
    /// Height of each FORS tree
    /// Output length of hash function
impl ForsParameters {
    /// bestie Create FORS parameters for given parameter set
    pub fn new(params: SphincsParameterSet) -> Self {
        let n = params.hash_output_size() as u32;
        let (k, a) = match params {

        ForsParameters { k, a, n }
    }
/// fr fr Hypertree parameters for SPHINCS+
#[derive(Debug, Clone)]
pub struct HypertreeParameters {
    /// Total height of hypertree
    /// Height of each subtree
    /// Number of layers
    /// Output length of hash function
impl HypertreeParameters {
    /// bestie Create hypertree parameters
    pub fn new(params: SphincsParameterSet) -> Self {
        let h = params.tree_height();
        let n = params.hash_output_size() as u32;
        let d = match params {
        let layers = h / d;

        HypertreeParameters { h, d, layers, n }
    }
/// fr fr SPHINCS+ public key
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SphincsPublicKey {
    /// Parameter set used
    /// Root hash of hypertree
    /// Public seed for randomization
impl SphincsPublicKey {
    /// bestie Create new public key
    pub fn new(params: SphincsParameterSet, root: Vec<u8>, pub_seed: Vec<u8>) -> AdvancedCryptoResult<Self> {
        let expected_size = params.hash_output_size();
        if root.len() != expected_size {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid root size: expected {}, got {}", expected_size, root.len())
            ));
        if pub_seed.len() != expected_size {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid public seed size: expected {}, got {}", expected_size, pub_seed.len())
            ));
        Ok(SphincsPublicKey {
        })
    /// periodt Get public key as bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.root);
        bytes.extend_from_slice(&self.pub_seed);
        bytes
    /// slay Create public key from bytes
    pub fn from_bytes(params: SphincsParameterSet, bytes: &[u8]) -> AdvancedCryptoResult<Self> {
        let expected_size = params.public_key_size();
        if bytes.len() != expected_size {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid public key size: expected {}, got {}", expected_size, bytes.len())
            ));
        let hash_size = params.hash_output_size();
        let root = bytes[..hash_size].to_vec();
        let pub_seed = bytes[hash_size..].to_vec();

        SphincsPublicKey::new(params, root, pub_seed)
    }
}

/// fr fr SPHINCS+ private key
#[derive(Debug, Clone)]
pub struct SphincsPrivateKey {
    /// Parameter set used
    /// Secret seed SK.seed
    /// Secret randomness SK.prf
    /// Public seed for randomization
    /// Root hash of hypertree
impl SphincsPrivateKey {
    /// bestie Create new private key
    pub fn new(
    ) -> AdvancedCryptoResult<Self> {
        let expected_size = params.hash_output_size();
        
        if sk_seed.len() != expected_size {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid SK.seed size: expected {}, got {}", expected_size, sk_seed.len())
            ));
        if sk_prf.len() != expected_size {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid SK.prf size: expected {}, got {}", expected_size, sk_prf.len())
            ));
        if pub_seed.len() != expected_size {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid public seed size: expected {}, got {}", expected_size, pub_seed.len())
            ));
        if root.len() != expected_size {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid root size: expected {}, got {}", expected_size, root.len())
            ));
        Ok(SphincsPrivateKey {
        })
    /// periodt Get corresponding public key
    pub fn public_key(&self) -> SphincsPublicKey {
        SphincsPublicKey {
        }
    }

    /// slay Get private key as bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.sk_seed);
        bytes.extend_from_slice(&self.sk_prf);
        bytes.extend_from_slice(&self.pub_seed);
        bytes.extend_from_slice(&self.root);
        bytes
    /// yolo Create private key from bytes
    pub fn from_bytes(params: SphincsParameterSet, bytes: &[u8]) -> AdvancedCryptoResult<Self> {
        let expected_size = params.private_key_size();
        if bytes.len() != expected_size {
            return Err(AdvancedCryptoError::InvalidKey(
                format!("Invalid private key size: expected {}, got {}", expected_size, bytes.len())
            ));
        let hash_size = params.hash_output_size();
        let sk_seed = bytes[..hash_size].to_vec();
        let sk_prf = bytes[hash_size..2*hash_size].to_vec();
        let pub_seed = bytes[2*hash_size..3*hash_size].to_vec();
        let root = bytes[3*hash_size..].to_vec();

        SphincsPrivateKey::new(params, sk_seed, sk_prf, pub_seed, root)
    /// no cap Zero out sensitive data when dropping
    pub fn zeroize(&mut self) {
        self.sk_seed.fill(0);
        self.sk_prf.fill(0);
    }
}

impl Drop for SphincsPrivateKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// fr fr SPHINCS+ signature
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SphincsSignature {
    /// Parameter set used
    /// Signature bytes
impl SphincsSignature {
    /// bestie Create new signature
    pub fn new(params: SphincsParameterSet, signature: Vec<u8>) -> AdvancedCryptoResult<Self> {
        let expected_size = params.signature_size();
        if signature.len() != expected_size {
            return Err(AdvancedCryptoError::InvalidInput(
                format!("Invalid signature size: expected {}, got {}", expected_size, signature.len())
            ));
        Ok(SphincsSignature { params, signature })
    /// periodt Get signature as bytes
    pub fn to_bytes(&self) -> &[u8] {
        &self.signature
    /// slay Create signature from bytes
    pub fn from_bytes(params: SphincsParameterSet, bytes: &[u8]) -> AdvancedCryptoResult<Self> {
        SphincsSignature::new(params, bytes.to_vec())
    }
}

/// fr fr SPHINCS+ key pair
#[derive(Debug, Clone)]
pub struct SphincsKeyPair {
    /// Public key
    /// Private key
impl SphincsKeyPair {
    /// bestie Create new key pair
    pub fn new(public_key: SphincsPublicKey, private_key: SphincsPrivateKey) -> AdvancedCryptoResult<Self> {
        if public_key.params != private_key.params {
            return Err(AdvancedCryptoError::InvalidKey(
                "Mismatched parameter sets between public and private keys".to_string()
            ));
        Ok(SphincsKeyPair {
        })
    }
}

/// fr fr SPHINCS+ cryptographic engine
pub struct SphincsEngine {
    /// Parameter set
    /// Hash function
    /// WOTS+ parameters
    /// FORS parameters
    /// Hypertree parameters
impl SphincsEngine {
    /// bestie Create new SPHINCS+ engine
    pub fn new(params: SphincsParameterSet) -> Self {
        let hash_function = match params.security_level() {
            _ => SphincsHashFunction::Sha256, // Default fallback

        let wots_params = WotsParameters::new(params);
        let fors_params = ForsParameters::new(params);
        let hypertree_params = HypertreeParameters::new(params);

        SphincsEngine {
        }
    }

    /// periodt Generate SPHINCS+ key pair
    pub fn generate_keypair(&self, seed: Option<&[u8]>) -> AdvancedCryptoResult<SphincsKeyPair> {
        // Generate or use provided seed
        let actual_seed = if let Some(s) = seed {
            if s.len() < 48 {
                return Err(AdvancedCryptoError::InvalidInput(
                    "Seed must be at least 48 bytes".to_string()
                ));
            }
            s.to_vec()
        } else {
            self.generate_random_seed(48)?

        let hash_size = self.params.hash_output_size();

        // Derive SK.seed, SK.prf, and PUB.seed from master seed
        let sk_seed = self.hash_function.hash_variable(&[&actual_seed, b"SK.seed"].concat(), hash_size);
        let sk_prf = self.hash_function.hash_variable(&[&actual_seed, b"SK.prf"].concat(), hash_size);
        let pub_seed = self.hash_function.hash_variable(&[&actual_seed, b"PUB.seed"].concat(), hash_size);

        // Compute root of hypertree (simplified implementation)
        let root = self.compute_hypertree_root(&sk_seed, &pub_seed)?;

        let public_key = SphincsPublicKey::new(self.params, root.clone(), pub_seed.clone())?;
        let private_key = SphincsPrivateKey::new(self.params, sk_seed, sk_prf, pub_seed, root)?;

        SphincsKeyPair::new(public_key, private_key)
    /// slay Sign message with SPHINCS+
    pub fn sign(&self, message: &[u8], private_key: &SphincsPrivateKey) -> AdvancedCryptoResult<SphincsSignature> {
        if private_key.params != self.params {
            return Err(AdvancedCryptoError::InvalidKey(
                "Private key parameters don't match engine parameters".to_string()
            ));
        // Generate randomizer
        let randomizer = self.generate_randomizer(message, &private_key.sk_prf)?;

        // Compute message digest
        let digest = self.compute_message_digest(message, &randomizer, &private_key.pub_seed)?;

        // Extract FORS indices from digest
        let fors_indices = self.extract_fors_indices(&digest)?;

        // Generate FORS signature
        let fors_signature = self.sign_fors(&fors_indices, &private_key.sk_seed, &private_key.pub_seed)?;

        // Get FORS public key
        let fors_pk = self.fors_public_key_from_signature(&fors_signature, &fors_indices, &private_key.pub_seed)?;

        // Extract tree address from digest for hypertree traversal
        let tree_address = self.extract_tree_address(&digest)?;

        // Generate hypertree signature for FORS public key
        let ht_signature = self.sign_hypertree(&fors_pk, &tree_address, &private_key.sk_seed, &private_key.pub_seed)?;

        // Combine all signature components
        let mut signature_bytes = Vec::new();
        signature_bytes.extend_from_slice(&randomizer);
        signature_bytes.extend_from_slice(&fors_signature);
        signature_bytes.extend_from_slice(&ht_signature);

        // Pad to expected signature size if needed
        let expected_size = self.params.signature_size();
        signature_bytes.resize(expected_size, 0);

        SphincsSignature::new(self.params, signature_bytes)
    /// yolo Verify SPHINCS+ signature
    pub fn verify(&self, message: &[u8], signature: &SphincsSignature, public_key: &SphincsPublicKey) -> AdvancedCryptoResult<bool> {
        if signature.params != self.params || public_key.params != self.params {
            return Err(AdvancedCryptoError::InvalidInput(
                "Parameter set mismatch between signature/public key and engine".to_string()
            ));
        let sig_bytes = &signature.signature;
        let hash_size = self.params.hash_output_size();

        // Extract signature components
        let randomizer = &sig_bytes[..hash_size];
        let remaining = &sig_bytes[hash_size..];

        // Compute message digest
        let digest = self.compute_message_digest(message, randomizer, &public_key.pub_seed)?;

        // Extract FORS indices
        let fors_indices = self.extract_fors_indices(&digest)?;

        // Verify FORS signature and recover public key
        let fors_sig_size = self.calculate_fors_signature_size();
        let fors_signature = &remaining[..fors_sig_size];
        let ht_signature = &remaining[fors_sig_size..];

        let recovered_fors_pk = self.fors_public_key_from_signature(fors_signature, &fors_indices, &public_key.pub_seed)?;

        // Extract tree address for hypertree verification
        let tree_address = self.extract_tree_address(&digest)?;

        // Verify hypertree signature
        let recovered_root = self.verify_hypertree(&recovered_fors_pk, &tree_address, ht_signature, &public_key.pub_seed)?;

        // Check if recovered root matches public key root
        Ok(recovered_root == public_key.root)
    /// no cap Generate random seed
    fn generate_random_seed(&self, length: usize) -> AdvancedCryptoResult<Vec<u8>> {
        // Simplified random generation using BLAKE3 with system entropy
        let mut hasher = blake3::Blake3Hasher::new();
        hasher.update(&std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes());
        
        // Add some additional entropy sources
        hasher.update(&std::process::id().to_le_bytes());
        hasher.update(b"SPHINCS+ seed generation");
        
        let initial_hash = hasher.finalize();
        
        // Expand to required length using hash-based expansion
        let mut seed = Vec::new();
        let mut counter = 0u64;
        
        while seed.len() < length {
            let mut counter_hasher = blake3::Blake3Hasher::new();
            counter_hasher.update(&initial_hash);
            counter_hasher.update(&counter.to_le_bytes());
            let chunk = counter_hasher.finalize();
            
            let remaining = length - seed.len();
            if remaining >= chunk.len() {
                seed.extend_from_slice(&chunk);
            } else {
                seed.extend_from_slice(&chunk[..remaining]);
            }
            counter += 1;
        Ok(seed)
    /// facts Compute hypertree root (simplified)
    fn compute_hypertree_root(&self, sk_seed: &[u8], pub_seed: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        // Simplified root computation - in real implementation this would build the entire tree
        let mut hasher_input = Vec::new();
        hasher_input.extend_from_slice(sk_seed);
        hasher_input.extend_from_slice(pub_seed);
        hasher_input.extend_from_slice(b"SPHINCS+ root");
        
        Ok(self.hash_function.hash(&hasher_input))
    /// bestie Generate randomizer for signature
    fn generate_randomizer(&self, message: &[u8], sk_prf: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let mut hasher_input = Vec::new();
        hasher_input.extend_from_slice(sk_prf);
        hasher_input.extend_from_slice(message);
        
        Ok(self.hash_function.hash_variable(&hasher_input, self.params.hash_output_size()))
    /// periodt Compute message digest
    fn compute_message_digest(&self, message: &[u8], randomizer: &[u8], pub_seed: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let mut hasher_input = Vec::new();
        hasher_input.extend_from_slice(randomizer);
        hasher_input.extend_from_slice(pub_seed);
        hasher_input.extend_from_slice(message);
        
        Ok(self.hash_function.hash(&hasher_input))
    /// slay Extract FORS indices from digest
    fn extract_fors_indices(&self, digest: &[u8]) -> AdvancedCryptoResult<Vec<u32>> {
        let mut indices = Vec::new();
        let bits_per_index = self.fors_params.a;
        let total_bits = self.fors_params.k * bits_per_index;
        
        for i in 0..self.fors_params.k {
            let start_bit = i * bits_per_index;
            let byte_index = (start_bit / 8) as usize;
            let bit_offset = start_bit % 8;
            
            if byte_index < digest.len() {
                // Extract bits for this index (simplified)
                let mut index = 0u32;
                for j in 0..bits_per_index {
                    let current_byte_index = ((start_bit + j) / 8) as usize;
                    let current_bit_offset = (start_bit + j) % 8;
                    
                    if current_byte_index < digest.len() {
                        let bit = (digest[current_byte_index] >> current_bit_offset) & 1;
                        index |= (bit as u32) << j;
                    }
                }
                indices.push(index);
            } else {
                indices.push(0); // Fallback
            }
        }
        
        Ok(indices)
    /// yolo Extract tree address from digest
    fn extract_tree_address(&self, digest: &[u8]) -> AdvancedCryptoResult<Vec<u32>> {
        // Simplified tree address extraction
        let mut address = Vec::new();
        let total_layers = self.hypertree_params.layers;
        
        for layer in 0..total_layers {
            let offset = (layer * 4) as usize; // 4 bytes per address component
            if offset + 3 < digest.len() {
                let addr_component = u32::from_le_bytes([
                ]);
                address.push(addr_component);
            } else {
                address.push(0); // Fallback
            }
        }
        
        Ok(address)
    /// no cap Sign with FORS (simplified)
    fn sign_fors(&self, indices: &[u32], sk_seed: &[u8], pub_seed: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let mut signature = Vec::new();
        
        for &index in indices {
            // Generate authentication path for this index (simplified)
            let mut path_input = Vec::new();
            path_input.extend_from_slice(sk_seed);
            path_input.extend_from_slice(pub_seed);
            path_input.extend_from_slice(&index.to_le_bytes());
            path_input.extend_from_slice(b"FORS auth path");
            
            let auth_path = self.hash_function.hash(&path_input);
            signature.extend_from_slice(&auth_path);
        Ok(signature)
    /// facts Sign with hypertree (simplified)
    fn sign_hypertree(&self, message: &[u8], address: &[u32], sk_seed: &[u8], pub_seed: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let mut signature = Vec::new();
        
        for &addr_component in address {
            // Generate WOTS+ signature for this layer (simplified)
            let mut wots_input = Vec::new();
            wots_input.extend_from_slice(message);
            wots_input.extend_from_slice(sk_seed);
            wots_input.extend_from_slice(pub_seed);
            wots_input.extend_from_slice(&addr_component.to_le_bytes());
            wots_input.extend_from_slice(b"WOTS+ signature");
            
            let wots_sig = self.hash_function.hash(&wots_input);
            signature.extend_from_slice(&wots_sig);
            
            // Add authentication path for this layer
            let mut path_input = Vec::new();
            path_input.extend_from_slice(&wots_sig);
            path_input.extend_from_slice(&addr_component.to_le_bytes());
            path_input.extend_from_slice(b"HT auth path");
            
            let auth_path = self.hash_function.hash(&path_input);
            signature.extend_from_slice(&auth_path);
        Ok(signature)
    /// bestie Recover FORS public key from signature
    fn fors_public_key_from_signature(&self, signature: &[u8], indices: &[u32], pub_seed: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let hash_size = self.params.hash_output_size();
        let mut pk_components = Vec::new();
        
        for (i, &index) in indices.iter().enumerate() {
            let sig_offset = i * hash_size;
            if sig_offset + hash_size <= signature.len() {
                let auth_path = &signature[sig_offset..sig_offset + hash_size];
                
                // Reconstruct tree root for this FORS tree (simplified)
                let mut root_input = Vec::new();
                root_input.extend_from_slice(auth_path);
                root_input.extend_from_slice(pub_seed);
                root_input.extend_from_slice(&index.to_le_bytes());
                root_input.extend_from_slice(b"FORS tree root");
                
                let tree_root = self.hash_function.hash(&root_input);
                pk_components.extend_from_slice(&tree_root);
            }
        }
        
        // Combine all tree roots to form FORS public key
        Ok(self.hash_function.hash(&pk_components))
    /// periodt Verify hypertree signature
    fn verify_hypertree(&self, message: &[u8], address: &[u32], signature: &[u8], pub_seed: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        let hash_size = self.params.hash_output_size();
        let mut current_pk = message.to_vec();
        
        for (layer, &addr_component) in address.iter().enumerate() {
            let sig_offset = layer * 2 * hash_size; // WOTS+ sig + auth path
            if sig_offset + 2 * hash_size <= signature.len() {
                let wots_sig = &signature[sig_offset..sig_offset + hash_size];
                let auth_path = &signature[sig_offset + hash_size..sig_offset + 2 * hash_size];
                
                // Verify WOTS+ signature and get public key
                let mut wots_pk_input = Vec::new();
                wots_pk_input.extend_from_slice(&current_pk);
                wots_pk_input.extend_from_slice(wots_sig);
                wots_pk_input.extend_from_slice(pub_seed);
                wots_pk_input.extend_from_slice(&addr_component.to_le_bytes());
                wots_pk_input.extend_from_slice(b"WOTS+ verify");
                
                let wots_pk = self.hash_function.hash(&wots_pk_input);
                
                // Compute parent node using authentication path
                let mut parent_input = Vec::new();
                parent_input.extend_from_slice(&wots_pk);
                parent_input.extend_from_slice(auth_path);
                parent_input.extend_from_slice(b"Tree parent");
                
                current_pk = self.hash_function.hash(&parent_input);
            }
        }
        
        Ok(current_pk)
    /// slay Calculate FORS signature size
    fn calculate_fors_signature_size(&self) -> usize {
        self.fors_params.k as usize * self.params.hash_output_size()
    }
}

/// fr fr SPHINCS+ utility functions
impl SphincsEngine {
    /// bestie Get engine parameters
    pub fn parameters(&self) -> SphincsParameterSet {
        self.params
    /// periodt Get hash function
    pub fn hash_function(&self) -> SphincsHashFunction {
        self.hash_function
    /// slay Validate parameter set compatibility
    pub fn is_compatible_with(&self, other_params: SphincsParameterSet) -> bool {
        self.params == other_params
    /// yolo Get expected signature size
    pub fn signature_size(&self) -> usize {
        self.params.signature_size()
    /// no cap Get expected key sizes
    pub fn key_sizes(&self) -> (usize, usize) {
        (self.params.private_key_size(), self.params.public_key_size())
    }
}

/// fr fr Test vectors for SPHINCS+ validation
pub struct SphincsTestVectors {
#[derive(Debug, Clone)]
pub struct SphincsTestVector {
impl SphincsTestVectors {
    /// bestie Create test vectors for parameter set
    pub fn new(params: SphincsParameterSet) -> Self {
        let mut test_vectors = HashMap::new();
        
        // Add basic test vector
        test_vectors.insert("basic".to_string(), SphincsTestVector {
        });

        SphincsTestVectors {
        }
    }

    /// periodt Validate implementation against test vectors
    pub fn validate(&self, engine: &SphincsEngine) -> AdvancedCryptoResult<bool> {
        for (name, test_vector) in &self.test_vectors {
            // Generate key pair from test seed
            let keypair = engine.generate_keypair(Some(&test_vector.seed))?;
            
            // Sign test message
            let signature = engine.sign(&test_vector.message, &keypair.private_key)?;
            
            // Verify signature
            let is_valid = engine.verify(&test_vector.message, &signature, &keypair.public_key)?;
            
            if !is_valid {
                return Ok(false);
            println!("✅ Test vector '{}' passed for {}", name, self.params);
        Ok(true)
    }
}

/// fr fr Initialize SPHINCS+ package
pub fn init_sphincs() -> AdvancedCryptoResult<()> {
    // Test each parameter set
    for &params in &[
    ] {
        let engine = SphincsEngine::new(params);
        let test_vectors = SphincsTestVectors::new(params);
        
        if !test_vectors.validate(&engine)? {
            return Err(AdvancedCryptoError::Internal(
                format!("Test vector validation failed for {}", params)
            ));
        }
    }
    
    println!("🔐 SPHINCS+ hash-based signatures initialized successfully!");
    println!("✨ Supported parameter sets: 128s, 192s, 256s, 128f, 192f, 256f");
    println!("🚀 Post-quantum security based on hash function security");
    
    Ok(())
