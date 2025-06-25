/// fr fr HKDF (HMAC-based Key Derivation Function) implementation
/// 
/// This module provides a production-ready implementation of HKDF (RFC 5869)
/// for key derivation and key stretching using HMAC.

use crate::error::CursedError;
// use crate::stdlib::value::Value;
// use crate::stdlib::packages::crypto_kdf::{KdfResult, KdfError};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha512};

/// fr fr HKDF hash algorithm variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HkdfAlgorithm {
    Sha256,
    Sha512,
}

impl HkdfAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            HkdfAlgorithm::Sha256 => "SHA-256",
            HkdfAlgorithm::Sha512 => "SHA-512",
        }
    }
    
    pub fn output_len(&self) -> usize {
        match self {
            HkdfAlgorithm::Sha256 => 32,
            HkdfAlgorithm::Sha512 => 64,
        }
    }
}

/// fr fr HKDF configuration
#[derive(Debug, Clone)]
pub struct HkdfConfig {
    pub algorithm: HkdfAlgorithm,
    pub max_output_len: usize,
}

impl HkdfConfig {
    /// slay Create HKDF config with defaults
    pub fn new() -> Self {
        Self {
            algorithm: HkdfAlgorithm::Sha256,
            max_output_len: 32 * 255, // 255 * hash_len is max per RFC 5869
        }
    }
    
    /// bestie Create HKDF config with SHA-512
    pub fn sha512() -> Self {
        Self {
            algorithm: HkdfAlgorithm::Sha512,
            max_output_len: 64 * 255,
        }
    }
    
    /// vibes Validate HKDF configuration
    pub fn validate(&self) -> KdfResult<()> {
        let max_allowed = self.algorithm.output_len() * 255;
        if self.max_output_len > max_allowed {
            return Err(KdfError::InvalidConfig(format!(
                "Maximum output length {} exceeds algorithm limit {}",
                self.max_output_len, max_allowed
            )));
        }
        Ok(())
    }
}

impl Default for HkdfConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr HKDF engine for key derivation
pub struct HkdfEngine {
    config: HkdfConfig,
}

impl HkdfEngine {
    /// slay Create new HKDF engine
    pub fn new() -> Self {
        Self {
            config: HkdfConfig::new(),
        }
    }
    
    /// bestie Create HKDF engine with custom config
    pub fn with_config(config: HkdfConfig) -> KdfResult<Self> {
        config.validate()?;
        Ok(Self { config })
    }
    
    /// vibes HKDF Extract phase - derive pseudorandom key
    pub fn extract(&self, salt: Option<&[u8]>, input_key_material: &[u8]) -> KdfResult<Vec<u8>> {
        if input_key_material.is_empty() {
            return Err(KdfError::InvalidInput("Input key material cannot be empty".to_string()));
        }
        
        match self.config.algorithm {
            HkdfAlgorithm::Sha256 => {
                type HmacSha256 = Hmac<Sha256>;
                let salt = salt.unwrap_or(&vec![0u8; 32]);
                let mut mac = HmacSha256::new_from_slice(salt)
                    .map_err(|_| KdfError::CryptographicError("HMAC initialization failed".to_string()))?;
                mac.update(input_key_material);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            HkdfAlgorithm::Sha512 => {
                type HmacSha512 = Hmac<Sha512>;
                let salt = salt.unwrap_or(&vec![0u8; 64]);
                let mut mac = HmacSha512::new_from_slice(salt)
                    .map_err(|_| KdfError::CryptographicError("HMAC initialization failed".to_string()))?;
                mac.update(input_key_material);
                Ok(mac.finalize().into_bytes().to_vec())
            }
        }
    }
    
    /// periodt HKDF Expand phase - derive output key material
    pub fn expand(&self, prk: &[u8], info: Option<&[u8]>, length: usize) -> KdfResult<Vec<u8>> {
        if prk.is_empty() {
            return Err(KdfError::InvalidInput("Pseudorandom key cannot be empty".to_string()));
        }
        
        if length == 0 {
            return Err(KdfError::InvalidInput("Output length must be greater than 0".to_string()));
        }
        
        if length > self.config.max_output_len {
            return Err(KdfError::InvalidInput(format!(
                "Requested length {} exceeds maximum {}",
                length, self.config.max_output_len
            )));
        }
        
        let hash_len = self.config.algorithm.output_len();
        let n = (length + hash_len - 1) / hash_len; // Ceiling division
        
        if n > 255 {
            return Err(KdfError::InvalidInput("Too many rounds required".to_string()));
        }
        
        let info = info.unwrap_or(&[]);
        let mut output = Vec::new();
        let mut t = Vec::new();
        
        match self.config.algorithm {
            HkdfAlgorithm::Sha256 => {
                type HmacSha256 = Hmac<Sha256>;
                
                for i in 1..=n {
                    let mut mac = HmacSha256::new_from_slice(prk)
                        .map_err(|_| KdfError::CryptographicError("HMAC initialization failed".to_string()))?;
                    
                    if i > 1 {
                        mac.update(&t);
                    }
                    mac.update(info);
                    mac.update(&[i as u8]);
                    
                    t = mac.finalize().into_bytes().to_vec();
                    output.extend_from_slice(&t);
                }
            }
            HkdfAlgorithm::Sha512 => {
                type HmacSha512 = Hmac<Sha512>;
                
                for i in 1..=n {
                    let mut mac = HmacSha512::new_from_slice(prk)
                        .map_err(|_| KdfError::CryptographicError("HMAC initialization failed".to_string()))?;
                    
                    if i > 1 {
                        mac.update(&t);
                    }
                    mac.update(info);
                    mac.update(&[i as u8]);
                    
                    t = mac.finalize().into_bytes().to_vec();
                    output.extend_from_slice(&t);
                }
            }
        }
        
        output.truncate(length);
        Ok(output)
    }
    
    /// facts HKDF full process - extract then expand
    pub fn derive_key(&self, salt: Option<&[u8]>, input_key_material: &[u8], info: Option<&[u8]>, length: usize) -> KdfResult<Vec<u8>> {
        let prk = self.extract(salt, input_key_material)?;
        self.expand(&prk, info, length)
    }
    
    /// bestie Derive multiple keys from the same input
    pub fn derive_multiple_keys(
        &self,
        salt: Option<&[u8]>,
        input_key_material: &[u8],
        key_specs: &[(Option<&[u8]>, usize)], // (info, length) pairs
    ) -> KdfResult<Vec<Vec<u8>>> {
        let prk = self.extract(salt, input_key_material)?;
        
        let mut keys = Vec::new();
        for (info, length) in key_specs {
            let key = self.expand(&prk, *info, *length)?;
            keys.push(key);
        }
        
        Ok(keys)
    }
}

impl Default for HkdfEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr HKDF utilities
pub struct HkdfUtils;

impl HkdfUtils {
    /// bestie Create info parameter for HKDF from context strings
    pub fn build_info(context: &str, purpose: Option<&str>) -> Vec<u8> {
        let mut info = context.as_bytes().to_vec();
        if let Some(p) = purpose {
            info.extend_from_slice(b":");
            info.extend_from_slice(p.as_bytes());
        }
        info
    }
    
    /// vibes Generate salt from random source
    pub fn generate_salt(length: usize) -> KdfResult<Vec<u8>> {
        use rand::RngCore;
        
        if length == 0 || length > 1024 {
            return Err(KdfError::InvalidInput("Salt length must be between 1 and 1024 bytes".to_string()));
        }
        
        let mut salt = vec![0u8; length];
        rand::thread_rng().fill_bytes(&mut salt);
        Ok(salt)
    }
}

/// fr fr Public API functions for CURSED integration

/// slay HKDF key derivation function
pub fn hkdf_derive_key(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("hkdf_derive_key requires at least input_key_material and length arguments".to_string()));
    }
    
    let input_key_material = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Input key material must be a string".to_string())),
    };
    
    let length = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err(CursedError::Runtime("Length must be a number".to_string())),
    };
    
    let salt = if args.len() > 2 {
        match &args[2] {
            Value::String(s) => Some(s.as_bytes()),
            _ => None,
        }
    } else {
        None
    };
    
    let info = if args.len() > 3 {
        match &args[3] {
            Value::String(s) => Some(s.as_bytes()),
            _ => None,
        }
    } else {
        None
    };
    
    let engine = HkdfEngine::new();
    let key = engine.derive_key(salt, input_key_material, info, length)
        .map_err(|e| CursedError::Runtime(format!("HKDF key derivation failed: {}", e)))?;
    
    Ok(Value::String(hex::encode(key)))
}

/// slay HKDF extract phase
pub fn hkdf_extract(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("hkdf_extract requires input_key_material argument".to_string()));
    }
    
    let input_key_material = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Input key material must be a string".to_string())),
    };
    
    let salt = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => Some(s.as_bytes()),
            _ => None,
        }
    } else {
        None
    };
    
    let engine = HkdfEngine::new();
    let prk = engine.extract(salt, input_key_material)
        .map_err(|e| CursedError::Runtime(format!("HKDF extract failed: {}", e)))?;
    
    Ok(Value::String(hex::encode(prk)))
}

/// slay HKDF expand phase
pub fn hkdf_expand(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("hkdf_expand requires prk and length arguments".to_string()));
    }
    
    let prk_hex = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("PRK must be a string".to_string())),
    };
    
    let prk = hex::decode(prk_hex)
        .map_err(|_| CursedError::Runtime("PRK must be valid hex string".to_string()))?;
    
    let length = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err(CursedError::Runtime("Length must be a number".to_string())),
    };
    
    let info = if args.len() > 2 {
        match &args[2] {
            Value::String(s) => Some(s.as_bytes()),
            _ => None,
        }
    } else {
        None
    };
    
    let engine = HkdfEngine::new();
    let key = engine.expand(&prk, info, length)
        .map_err(|e| CursedError::Runtime(format!("HKDF expand failed: {}", e)))?;
    
    Ok(Value::String(hex::encode(key)))
}

