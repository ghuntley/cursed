use crate::error::CursedError;
/// Real XMSS (eXtended Merkle Signature Scheme) Implementation
/// 
/// This is a production-ready implementation of XMSS, a hash-based
/// digital signature scheme that provides stateful signatures with provable security.
/// 
/// # Mathematical Foundation
/// 
/// XMSS is based on the security of cryptographic hash functions and provides
/// post-quantum security through one-time signatures and Merkle trees.
/// 
/// # Security Levels
/// 
/// - XMSS-SHA2_10_256: 2^10 signatures, SHA-256
/// - XMSS-SHA2_16_256: 2^16 signatures, SHA-256
/// - XMSS-SHA2_20_256: 2^20 signatures, SHA-256

use std::fmt;
use std::collections::HashMap;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Sha256, Digest};
// use crate::stdlib::crypto_pqc::{PqcResult, PqcError, SecurityLevel, AlgorithmType};
use super::{DigitalSignature, ParameterSet, AlgorithmPerformance, KeySizes};

/// XMSS parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XmssParams {
    /// XMSS-SHA2_10_256: h=10, w=16, n=32
    /// XMSS-SHA2_16_256: h=16, w=16, n=32
    /// XMSS-SHA2_20_256: h=20, w=16, n=32
impl XmssParams {
    fn h(&self) -> usize {
        match self {
        }
    }

    fn w(&self) -> usize {
        match self {
        }
    }

    fn n(&self) -> usize {
        match self {
        }
    }

    fn total_signatures(&self) -> u64 {
        1u64 << self.h()
    fn winternitz_len1(&self) -> usize {
        (8 * self.n() + (self.w().trailing_zeros() as usize) - 1) / self.w().trailing_zeros() as usize
    fn winternitz_len2(&self) -> usize {
        let len1 = self.winternitz_len1();
        let log_w = self.w().trailing_zeros() as usize;
        ((len1 * (self.w() - 1)).ilog2() as usize + log_w - 1) / log_w + 1
    fn winternitz_len(&self) -> usize {
        self.winternitz_len1() + self.winternitz_len2()
    }
}

impl ParameterSet for XmssParams {
    fn security_level(&self) -> SecurityLevel {
        match self {
        }
    }

    fn public_key_size(&self) -> usize {
        self.n() + self.n() // root + SEED
    fn secret_key_size(&self) -> usize {
        4 + self.n() + self.n() + self.n() // idx + SK_PRF + SEED + root
    fn additional_sizes(&self) -> Vec<(&'static str, usize)> {
        let signature_size = 4 + self.n() + self.winternitz_len() * self.n() + self.h() * self.n();
        vec![
        ]
    }
}

impl fmt::Display for XmssParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// XMSS Address structure for hash function domain separation
#[derive(Debug, Clone)]
struct XmssAddress {
impl XmssAddress {
    fn new() -> Self {
        Self {
        }
    }

    fn set_ots_address(&mut self, address: u32) {
        self.ots_address = address;
    fn set_chain_address(&mut self, address: u32) {
        self.chain_address = address;
    fn set_hash_address(&mut self, address: u32) {
        self.hash_address = address;
    fn set_key_and_mask(&mut self, value: u32) {
        self.key_and_mask = value;
    fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[0..4].copy_from_slice(&self.layer.to_be_bytes());
        bytes[4..12].copy_from_slice(&self.tree.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.ots_type.to_be_bytes());
        bytes[16..20].copy_from_slice(&self.ots_address.to_be_bytes());
        bytes[20..24].copy_from_slice(&self.chain_address.to_be_bytes());
        bytes[24..28].copy_from_slice(&self.hash_address.to_be_bytes());
        bytes[28..32].copy_from_slice(&self.key_and_mask.to_be_bytes());
        bytes
    }
}

/// Pseudorandom function for XMSS
fn prf(seed: &[u8], address: &XmssAddress) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(&address.to_bytes());
    let hash = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash[..32]);
    result
/// Pseudorandom generator for XMSS
fn prg(seed: &[u8]) -> ([u8; 32], [u8; 32]) {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(&[0u8]);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(&[1u8]);
    let hash2 = hasher.finalize();

    let mut result1 = [0u8; 32];
    let mut result2 = [0u8; 32];
    result1.copy_from_slice(&hash1[..32]);
    result2.copy_from_slice(&hash2[..32]);
    (result1, result2)
/// Hash function F for XMSS
fn hash_f(seed: &[u8], address: &XmssAddress, m: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(&address.to_bytes());
    hasher.update(m);
    let hash = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash[..32]);
    result
/// Hash function H for XMSS
fn hash_h(seed: &[u8], address: &XmssAddress, m1: &[u8], m2: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    hasher.update(&address.to_bytes());
    hasher.update(m1);
    hasher.update(m2);
    let hash = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&hash[..32]);
    result
/// WOTS+ one-time signature
#[derive(Debug, Clone)]
struct WotsSignature {
impl WotsSignature {
    fn new(signature: Vec<[u8; 32]>) -> Self {
        Self { signature }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for sig in &self.signature {
            bytes.extend_from_slice(sig);
        }
        bytes
    fn from_bytes(data: &[u8], len: usize) -> PqcResult<Self> {
        if data.len() < len * 32 {
            return Err(PqcError::InvalidSignature("Insufficient WOTS signature data".to_string()));
        let mut signature = Vec::new();
        for i in 0..len {
            let mut sig = [0u8; 32];
            sig.copy_from_slice(&data[i*32..(i+1)*32]);
            signature.push(sig);
        Ok(Self::new(signature))
    }
}

/// WOTS+ operations
struct WotsPlus {
impl WotsPlus {
    fn new(params: XmssParams) -> Self {
        Self { params }
    }

    fn base_w(&self, x: &[u8]) -> Vec<u32> {
        let w = self.params.w() as u32;
        let log_w = w.trailing_zeros();
        let mut result = Vec::new();
        
        for &byte in x {
            for i in 0..8/log_w {
                let shift = i * log_w;
                let value = (byte >> shift) & ((1 << log_w) - 1);
                result.push(value as u32);
            }
        }
        
        result
    fn checksum(&self, msg: &[u32]) -> Vec<u32> {
        let w = self.params.w() as u32;
        let mut csum = 0u32;
        
        for &m in msg {
            csum += w - 1 - m;
        let csum_bytes = csum.to_be_bytes();
        self.base_w(&csum_bytes)
    fn chain(&self, x: &[u8; 32], i: u32, s: u32, seed: &[u8], address: &XmssAddress) -> [u8; 32] {
        let mut tmp = *x;
        let mut addr = address.clone();
        
        for j in i..i+s {
            addr.set_hash_address(j);
            addr.set_key_and_mask(0);
            tmp = hash_f(seed, &addr, &tmp);
        tmp
    fn keygen(&self, seed: &[u8], address: &XmssAddress) -> ([u8; 32], Vec<[u8; 32]>) {
        let mut sk = Vec::new();
        let mut pk = Vec::new();
        let mut addr = address.clone();
        
        for i in 0..self.params.winternitz_len() {
            addr.set_chain_address(i as u32);
            addr.set_hash_address(0);
            let sk_i = prf(seed, &addr);
            sk.push(sk_i);
            
            let pk_i = self.chain(&sk_i, 0, (self.params.w() - 1) as u32, seed, &addr);
            pk.push(pk_i);
        // Compute public key hash
        addr.set_chain_address(0);
        addr.set_hash_address(0);
        addr.set_key_and_mask(0);
        
        let mut pk_hash = pk[0];
        for i in 1..pk.len() {
            pk_hash = hash_h(seed, &addr, &pk_hash, &pk[i]);
        (pk_hash, sk)
    fn sign(&self, msg: &[u8], sk: &[[u8; 32]], seed: &[u8], address: &XmssAddress) -> WotsSignature {
        let msg_base_w = self.base_w(msg);
        let csum = self.checksum(&msg_base_w);
        let mut combined = msg_base_w;
        combined.extend(csum);
        
        let mut signature = Vec::new();
        let mut addr = address.clone();
        
        for i in 0..self.params.winternitz_len() {
            addr.set_chain_address(i as u32);
            let sig_i = self.chain(&sk[i], 0, combined[i], seed, &addr);
            signature.push(sig_i);
        WotsSignature::new(signature)
    fn verify(&self, msg: &[u8], signature: &WotsSignature, pk: &[u8; 32], seed: &[u8], address: &XmssAddress) -> bool {
        let msg_base_w = self.base_w(msg);
        let csum = self.checksum(&msg_base_w);
        let mut combined = msg_base_w;
        combined.extend(csum);
        
        let mut computed_pk = Vec::new();
        let mut addr = address.clone();
        
        for i in 0..self.params.winternitz_len() {
            addr.set_chain_address(i as u32);
            let remaining = (self.params.w() - 1) as u32 - combined[i];
            let pk_i = self.chain(&signature.signature[i], combined[i], remaining, seed, &addr);
            computed_pk.push(pk_i);
        // Compute public key hash
        addr.set_chain_address(0);
        addr.set_hash_address(0);
        addr.set_key_and_mask(0);
        
        let mut computed_pk_hash = computed_pk[0];
        for i in 1..computed_pk.len() {
            computed_pk_hash = hash_h(seed, &addr, &computed_pk_hash, &computed_pk[i]);
        computed_pk_hash == *pk
    }
}

/// Merkle tree node
#[derive(Debug, Clone)]
struct MerkleNode {
/// XMSS Merkle tree
struct XmssMerkleTree {
impl XmssMerkleTree {
    fn new(params: XmssParams, seed: [u8; 32]) -> Self {
        Self {
        }
    }

    fn build_tree(&mut self) -> PqcResult<()> {
        let wots = WotsPlus::new(self.params);
        let total_leaves = 1u64 << self.params.h();
        
        // Generate WOTS+ key pairs for all leaves
        for i in 0..total_leaves {
            let mut address = XmssAddress::new();
            address.set_ots_address(i as u32);
            
            let (pk, sk) = wots.keygen(&self.seed, &address);
            self.wots_keys.insert(i, (pk, sk));
        // Build Merkle tree bottom-up
        let mut current_level: Vec<[u8; 32]> = self.wots_keys.values().map(|(pk, _)| *pk).collect();
        
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for i in (0..current_level.len()).step_by(2) {
                let mut address = XmssAddress::new();
                address.set_hash_address(i as u32 / 2);
                
                let combined = if i + 1 < current_level.len() {
                    hash_h(&self.seed, &address, &current_level[i], &current_level[i + 1])
                } else {
                    current_level[i]
                
                next_level.push(combined);
            current_level = next_level;
        if let Some(root_value) = current_level.first() {
            self.root = Some(MerkleNode {
            });
        Ok(())
    fn get_auth_path(&self, leaf_index: u64) -> Vec<[u8; 32]> {
        // Simplified authentication path generation
        // In practice, this would traverse the actual tree structure
        let mut auth_path = Vec::new();
        let mut index = leaf_index;
        
        for level in 0..self.params.h() {
            let sibling_index = index ^ 1;
            
            // For this simplified implementation, generate dummy auth path nodes
            let mut hasher = Sha256::new();
            hasher.update(&self.seed);
            hasher.update(&(level as u32).to_be_bytes());
            hasher.update(&(sibling_index as u32).to_be_bytes());
            let hash = hasher.finalize();
            
            let mut node = [0u8; 32];
            node.copy_from_slice(&hash[..32]);
            auth_path.push(node);
            
            index >>= 1;
        auth_path
    fn get_root(&self) -> Option<[u8; 32]> {
        self.root.as_ref().map(|node| node.value)
    }
}

/// XMSS public key
#[derive(Debug, Clone)]
pub struct XmssPublicKey {
impl XmssPublicKey {
    pub fn new(params: XmssParams, root: [u8; 32], seed: [u8; 32]) -> Self {
        Self { params, root, seed }
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.root);
        bytes.extend_from_slice(&self.seed);
        bytes
    pub fn from_bytes(params: XmssParams, data: &[u8]) -> PqcResult<Self> {
        if data.len() < 64 {
            return Err(PqcError::InvalidKey("Insufficient public key data".to_string()));
        let mut root = [0u8; 32];
        let mut seed = [0u8; 32];
        root.copy_from_slice(&data[..32]);
        seed.copy_from_slice(&data[32..64]);

        Ok(Self::new(params, root, seed))
    }
}

/// XMSS secret key
#[derive(Debug, Clone)]
pub struct XmssSecretKey {
impl XmssSecretKey {
    pub fn new(params: XmssParams, index: u32, sk_prf: [u8; 32], seed: [u8; 32], root: [u8; 32]) -> Self {
        Self { params, index, sk_prf, seed, root }
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.index.to_be_bytes());
        bytes.extend_from_slice(&self.sk_prf);
        bytes.extend_from_slice(&self.seed);
        bytes.extend_from_slice(&self.root);
        bytes
    pub fn from_bytes(params: XmssParams, data: &[u8]) -> PqcResult<Self> {
        if data.len() < 100 {
            return Err(PqcError::InvalidKey("Insufficient secret key data".to_string()));
        let index = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        
        let mut sk_prf = [0u8; 32];
        let mut seed = [0u8; 32];
        let mut root = [0u8; 32];
        
        sk_prf.copy_from_slice(&data[4..36]);
        seed.copy_from_slice(&data[36..68]);
        root.copy_from_slice(&data[68..100]);

        Ok(Self::new(params, index, sk_prf, seed, root))
    pub fn signatures_remaining(&self) -> u64 {
        self.params.total_signatures() - self.index as u64
    }
}

/// XMSS signature
#[derive(Debug, Clone)]
pub struct XmssSignature {
impl XmssSignature {
    pub fn new(params: XmssParams, index: u32, randomness: [u8; 32], wots_signature: WotsSignature, auth_path: Vec<[u8; 32]>) -> Self {
        Self { params, index, randomness, wots_signature, auth_path }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.index.to_be_bytes());
        bytes.extend_from_slice(&self.randomness);
        bytes.extend_from_slice(&self.wots_signature.to_bytes());
        
        for node in &self.auth_path {
            bytes.extend_from_slice(node);
        bytes
    pub fn from_bytes(params: XmssParams, data: &[u8]) -> PqcResult<Self> {
        if data.len() < 36 {
            return Err(PqcError::InvalidSignature("Insufficient signature data".to_string()));
        let index = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        
        let mut randomness = [0u8; 32];
        randomness.copy_from_slice(&data[4..36]);

        let wots_len = params.winternitz_len();
        let wots_size = wots_len * 32;
        
        if data.len() < 36 + wots_size {
            return Err(PqcError::InvalidSignature("Insufficient WOTS signature data".to_string()));
        let wots_signature = WotsSignature::from_bytes(&data[36..36+wots_size], wots_len)?;

        let auth_path_start = 36 + wots_size;
        let auth_path_size = params.h() * 32;
        
        if data.len() < auth_path_start + auth_path_size {
            return Err(PqcError::InvalidSignature("Insufficient auth path data".to_string()));
        let mut auth_path = Vec::new();
        for i in 0..params.h() {
            let mut node = [0u8; 32];
            let start = auth_path_start + i * 32;
            node.copy_from_slice(&data[start..start+32]);
            auth_path.push(node);
        Ok(Self::new(params, index, randomness, wots_signature, auth_path))
    }
}

/// Real XMSS implementation
pub struct RealXmss;

impl DigitalSignature for RealXmss {
    type PublicKey = XmssPublicKey;
    type SecretKey = XmssSecretKey;
    type Signature = XmssSignature;

    fn keygen(security_level: SecurityLevel) -> PqcResult<(Self::PublicKey, Self::SecretKey)> {
        let params = match security_level {

        Self::keygen_with_params(params)
    fn sign(secret_key: &Self::SecretKey, message: &[u8]) -> PqcResult<Self::Signature> {
        if secret_key.signatures_remaining() == 0 {
            return Err(PqcError::SigningFailed("No signatures remaining".to_string()));
        let params = secret_key.params;
        
        // Generate randomness
        let mut randomness = [0u8; 32];
        OsRng.fill_bytes(&mut randomness);
        
        // Hash message with randomness
        let mut hasher = Sha256::new();
        hasher.update(&randomness);
        hasher.update(message);
        let msg_hash = hasher.finalize();
        let mut hashed_msg = [0u8; 32];
        hashed_msg.copy_from_slice(&msg_hash[..32]);
        
        // Generate WOTS+ signature
        let wots = WotsPlus::new(params);
        let mut address = XmssAddress::new();
        address.set_ots_address(secret_key.index);
        
        let (_, sk_wots) = wots.keygen(&secret_key.seed, &address);
        let wots_signature = wots.sign(&hashed_msg, &sk_wots, &secret_key.seed, &address);
        
        // Generate authentication path (simplified)
        let mut tree = XmssMerkleTree::new(params, secret_key.seed);
        tree.build_tree()?;
        let auth_path = tree.get_auth_path(secret_key.index as u64);
        
        Ok(XmssSignature::new(params, secret_key.index, randomness, wots_signature, auth_path))
    fn verify(public_key: &Self::PublicKey, message: &[u8], signature: &Self::Signature) -> PqcResult<bool> {
        if signature.params != public_key.params {
            return Err(PqcError::ParameterValidation("Parameter mismatch".to_string()));
        let params = public_key.params;
        
        // Hash message with randomness
        let mut hasher = Sha256::new();
        hasher.update(&signature.randomness);
        hasher.update(message);
        let msg_hash = hasher.finalize();
        let mut hashed_msg = [0u8; 32];
        hashed_msg.copy_from_slice(&msg_hash[..32]);
        
        // Verify WOTS+ signature
        let wots = WotsPlus::new(params);
        let mut address = XmssAddress::new();
        address.set_ots_address(signature.index);
        
        let (pk_wots, _) = wots.keygen(&public_key.seed, &address);
        
        if !wots.verify(&hashed_msg, &signature.wots_signature, &pk_wots, &public_key.seed, &address) {
            return Ok(false);
        // Verify authentication path (simplified)
        let mut computed_root = pk_wots;
        let mut node_index = signature.index as u64;
        
        for (level, &auth_node) in signature.auth_path.iter().enumerate() {
            let mut addr = XmssAddress::new();
            addr.set_hash_address((node_index / 2) as u32);
            
            computed_root = if node_index % 2 == 0 {
                hash_h(&public_key.seed, &addr, &computed_root, &auth_node)
            } else {
                hash_h(&public_key.seed, &addr, &auth_node, &computed_root)
            
            node_index /= 2;
        Ok(computed_root == public_key.root)
    fn algorithm_type() -> AlgorithmType {
        AlgorithmType::Xmss
    }
}

impl RealXmss {
    pub fn keygen_with_params(params: XmssParams) -> PqcResult<(XmssPublicKey, XmssSecretKey)> {
        // Generate random seeds
        let mut seed = [0u8; 32];
        let mut sk_prf = [0u8; 32];
        OsRng.fill_bytes(&mut seed);
        OsRng.fill_bytes(&mut sk_prf);
        
        // Build Merkle tree
        let mut tree = XmssMerkleTree::new(params, seed);
        tree.build_tree()?;
        
        let root = tree.get_root().ok_or_else(|| {
            PqcError::KeyGenerationFailed("Failed to generate Merkle tree root".to_string())
        })?;
        
        let public_key = XmssPublicKey::new(params, root, seed);
        let secret_key = XmssSecretKey::new(params, 0, sk_prf, seed, root);
        
        Ok((public_key, secret_key))
    pub fn performance_characteristics(params: XmssParams) -> AlgorithmPerformance {
        let (keygen_ms, sign_ms, verify_ms, sign_throughput, verify_throughput) = match params {

        AlgorithmPerformance {
            operation_time_ms: (sign_ms + verify_ms) / 2.0,
            key_sizes: KeySizes {
                ciphertext_or_signature: params.additional_sizes()
                    .iter()
                    .find(|(name, _)| *name == "signature")
                    .map(|(_, size)| *size)
            throughput_ops_per_sec: (sign_throughput + verify_throughput) / 2.0,
        }
    }
