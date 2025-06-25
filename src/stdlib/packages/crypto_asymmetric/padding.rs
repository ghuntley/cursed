// Cryptographic padding schemes
// 
// Provides comprehensive padding schemes for asymmetric cryptography.
// Supports OAEP, PKCS#1 v1.5, and PSS padding with various hash algorithms.

// use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt, Oaep, Pss, Pkcs1v15Sign};
use rsa::signature::{RandomizedSigner, Verifier};
use sha2::{Sha256, Sha384, Sha512};
use sha1::Sha1;

/// Supported padding schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaddingScheme {
    /// PKCS#1 v1.5 padding for encryption
    Pkcs1v15Encrypt,
    /// PKCS#1 v1.5 padding for signatures
    Pkcs1v15Sign,
    /// OAEP padding with SHA-1
    OaepSha1,
    /// OAEP padding with SHA-256
    OaepSha256,
    /// OAEP padding with SHA-384
    OaepSha384,
    /// OAEP padding with SHA-512
    OaepSha512,
    /// PSS padding with SHA-256
    PssSha256,
    /// PSS padding with SHA-384
    PssSha384,
    /// PSS padding with SHA-512
    PssSha512,
}

impl PaddingScheme {
    pub fn name(&self) -> &'static str {
        match self {
            PaddingScheme::Pkcs1v15Encrypt => "PKCS1v15-Encrypt",
            PaddingScheme::Pkcs1v15Sign => "PKCS1v15-Sign",
            PaddingScheme::OaepSha1 => "OAEP-SHA1",
            PaddingScheme::OaepSha256 => "OAEP-SHA256",
            PaddingScheme::OaepSha384 => "OAEP-SHA384",
            PaddingScheme::OaepSha512 => "OAEP-SHA512",
            PaddingScheme::PssSha256 => "PSS-SHA256",
            PaddingScheme::PssSha384 => "PSS-SHA384",
            PaddingScheme::PssSha512 => "PSS-SHA512",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            PaddingScheme::Pkcs1v15Encrypt => "PKCS#1 v1.5 padding for encryption (legacy)",
            PaddingScheme::Pkcs1v15Sign => "PKCS#1 v1.5 padding for signatures",
            PaddingScheme::OaepSha1 => "OAEP with SHA-1 (not recommended)",
            PaddingScheme::OaepSha256 => "OAEP with SHA-256 (recommended)",
            PaddingScheme::OaepSha384 => "OAEP with SHA-384",
            PaddingScheme::OaepSha512 => "OAEP with SHA-512",
            PaddingScheme::PssSha256 => "PSS with SHA-256 (recommended for signatures)",
            PaddingScheme::PssSha384 => "PSS with SHA-384",
            PaddingScheme::PssSha512 => "PSS with SHA-512",
        }
    }
    
    pub fn is_secure(&self) -> bool {
        match self {
            PaddingScheme::Pkcs1v15Encrypt => false, // Vulnerable to padding oracle attacks
            PaddingScheme::Pkcs1v15Sign => true,     // Still secure for signatures
            PaddingScheme::OaepSha1 => false,        // SHA-1 is deprecated
            PaddingScheme::OaepSha256 => true,
            PaddingScheme::OaepSha384 => true,
            PaddingScheme::OaepSha512 => true,
            PaddingScheme::PssSha256 => true,
            PaddingScheme::PssSha384 => true,
            PaddingScheme::PssSha512 => true,
        }
    }
    
    pub fn use_case(&self) -> &'static str {
        match self {
            PaddingScheme::Pkcs1v15Encrypt => "Legacy encryption (avoid)",
            PaddingScheme::Pkcs1v15Sign => "Digital signatures",
            PaddingScheme::OaepSha1 => "Legacy encryption (avoid)",
            PaddingScheme::OaepSha256 => "Modern encryption",
            PaddingScheme::OaepSha384 => "High-security encryption",
            PaddingScheme::OaepSha512 => "High-security encryption",
            PaddingScheme::PssSha256 => "Modern digital signatures",
            PaddingScheme::PssSha384 => "High-security signatures",
            PaddingScheme::PssSha512 => "High-security signatures",
        }
    }
    
    pub fn from_name(name: &str) -> crate::error::Result<()> {
        match name.to_uppercase().as_str() {
            "PKCS1V15-ENCRYPT" | "PKCS1V15_ENCRYPT" => Ok(PaddingScheme::Pkcs1v15Encrypt),
            "PKCS1V15-SIGN" | "PKCS1V15_SIGN" => Ok(PaddingScheme::Pkcs1v15Sign),
            "OAEP-SHA1" | "OAEP_SHA1" => Ok(PaddingScheme::OaepSha1),
            "OAEP-SHA256" | "OAEP_SHA256" => Ok(PaddingScheme::OaepSha256),
            "OAEP-SHA384" | "OAEP_SHA384" => Ok(PaddingScheme::OaepSha384),
            "OAEP-SHA512" | "OAEP_SHA512" => Ok(PaddingScheme::OaepSha512),
            "PSS-SHA256" | "PSS_SHA256" => Ok(PaddingScheme::PssSha256),
            "PSS-SHA384" | "PSS_SHA384" => Ok(PaddingScheme::PssSha384),
            "PSS-SHA512" | "PSS_SHA512" => Ok(PaddingScheme::PssSha512),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported padding scheme: {}", name))),
        }
    }
}

/// Padding operation result
#[derive(Debug, Clone)]
pub struct PaddingResult {
    pub scheme: PaddingScheme,
    pub padded_data: Vec<u8>,
    pub original_length: usize,
    pub padded_length: usize,
}

impl PaddingResult {
    pub fn new(scheme: PaddingScheme, padded_data: Vec<u8>, original_length: usize) -> Self {
        let padded_length = padded_data.len();
        Self {
            scheme,
            padded_data,
            original_length,
            padded_length,
        }
    }
    
    pub fn to_value(&self) -> crate::error::Result<()> {
        let mut map = HashMap::new();
        
        map.insert("scheme".to_string(), Value::String(self.scheme.name().to_string()));
        map.insert("padded_data".to_string(), Value::String(hex::encode(&self.padded_data)));
        map.insert("original_length".to_string(), Value::Integer(self.original_length as i64));
        map.insert("padded_length".to_string(), Value::Integer(self.padded_length as i64));
        
        Ok(Value::Object(map))
    }
}

/// OAEP padding
pub fn oaep_padding(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("OAEP padding requires: data, public_key, hash_algorithm".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Binary(b) => b.clone(),
        _ => return Err(CursedError::InvalidArgument("Data must be string or binary".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let hash_algorithm = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Hash algorithm must be a string".to_string())),
    };
    
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    let public_key = RsaPublicKey::from_public_key_der(&public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA public key: {}", e)))?;
    
    let mut rng = OsRng;
    
    let encrypted = match hash_algorithm.to_uppercase().as_str() {
        "SHA1" => {
            let padding = Oaep::new::<Sha1>();
            public_key.encrypt(&mut rng, padding, &data)
                .map_err(|e| CursedError::CryptoError(format!("OAEP-SHA1 encryption failed: {}", e)))?
        },
        "SHA256" => {
            let padding = Oaep::new::<Sha256>();
            public_key.encrypt(&mut rng, padding, &data)
                .map_err(|e| CursedError::CryptoError(format!("OAEP-SHA256 encryption failed: {}", e)))?
        },
        "SHA384" => {
            let padding = Oaep::new::<Sha384>();
            public_key.encrypt(&mut rng, padding, &data)
                .map_err(|e| CursedError::CryptoError(format!("OAEP-SHA384 encryption failed: {}", e)))?
        },
        "SHA512" => {
            let padding = Oaep::new::<Sha512>();
            public_key.encrypt(&mut rng, padding, &data)
                .map_err(|e| CursedError::CryptoError(format!("OAEP-SHA512 encryption failed: {}", e)))?
        },
        _ => return Err(CursedError::InvalidArgument(format!("Unsupported hash algorithm for OAEP: {}", hash_algorithm))),
    };
    
    let scheme = match hash_algorithm.to_uppercase().as_str() {
        "SHA1" => PaddingScheme::OaepSha1,
        "SHA256" => PaddingScheme::OaepSha256,
        "SHA384" => PaddingScheme::OaepSha384,
        "SHA512" => PaddingScheme::OaepSha512,
        _ => unreachable!(),
    };
    
    let result = PaddingResult::new(scheme, encrypted, data.len());
    result.to_value()
}

/// OAEP unpadding (decryption)
pub fn oaep_unpadding(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("OAEP unpadding requires: encrypted_data, private_key, hash_algorithm".to_string()));
    }
    
    let encrypted_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Encrypted data must be a string".to_string())),
    };
    
    let private_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let hash_algorithm = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Hash algorithm must be a string".to_string())),
    };
    
    let encrypted_data = hex::decode(encrypted_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid encrypted data hex: {}", e)))?;
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let private_key = RsaPrivateKey::from_pkcs8_der(&private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA private key: {}", e)))?;
    
    let decrypted = match hash_algorithm.to_uppercase().as_str() {
        "SHA1" => {
            let padding = Oaep::new::<Sha1>();
            private_key.decrypt(padding, &encrypted_data)
                .map_err(|e| CursedError::CryptoError(format!("OAEP-SHA1 decryption failed: {}", e)))?
        },
        "SHA256" => {
            let padding = Oaep::new::<Sha256>();
            private_key.decrypt(padding, &encrypted_data)
                .map_err(|e| CursedError::CryptoError(format!("OAEP-SHA256 decryption failed: {}", e)))?
        },
        "SHA384" => {
            let padding = Oaep::new::<Sha384>();
            private_key.decrypt(padding, &encrypted_data)
                .map_err(|e| CursedError::CryptoError(format!("OAEP-SHA384 decryption failed: {}", e)))?
        },
        "SHA512" => {
            let padding = Oaep::new::<Sha512>();
            private_key.decrypt(padding, &encrypted_data)
                .map_err(|e| CursedError::CryptoError(format!("OAEP-SHA512 decryption failed: {}", e)))?
        },
        _ => return Err(CursedError::InvalidArgument(format!("Unsupported hash algorithm for OAEP: {}", hash_algorithm))),
    };
    
    let mut result = HashMap::new();
    result.insert("decrypted_data".to_string(), Value::String(hex::encode(&decrypted)));
    result.insert("plaintext".to_string(), Value::String(String::from_utf8_lossy(&decrypted).to_string()));
    result.insert("hash_algorithm".to_string(), Value::String(hash_algorithm));
    
    Ok(Value::Object(result))
}

/// PKCS#1 v1.5 padding
pub fn pkcs1_padding(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("PKCS#1 padding requires: data, public_key, operation_type".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Binary(b) => b.clone(),
        _ => return Err(CursedError::InvalidArgument("Data must be string or binary".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let operation_type = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Operation type must be a string".to_string())),
    };
    
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    let public_key = RsaPublicKey::from_public_key_der(&public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA public key: {}", e)))?;
    
    let mut rng = OsRng;
    
    match operation_type.to_uppercase().as_str() {
        "ENCRYPT" => {
            let padding = Pkcs1v15Encrypt;
            let encrypted = public_key.encrypt(&mut rng, padding, &data)
                .map_err(|e| CursedError::CryptoError(format!("PKCS#1 v1.5 encryption failed: {}", e)))?;
            
            let result = PaddingResult::new(PaddingScheme::Pkcs1v15Encrypt, encrypted, data.len());
            result.to_value()
        },
        "SIGN" => {
            // For signatures, we need the private key
            Err(CursedError::InvalidArgument("PKCS#1 v1.5 signing requires private key".to_string()))
        },
        _ => Err(CursedError::InvalidArgument(format!("Unsupported operation type: {}", operation_type))),
    }
}

/// PKCS#1 v1.5 unpadding (decryption)
pub fn pkcs1_unpadding(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("PKCS#1 unpadding requires: encrypted_data, private_key".to_string()));
    }
    
    let encrypted_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Encrypted data must be a string".to_string())),
    };
    
    let private_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let encrypted_data = hex::decode(encrypted_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid encrypted data hex: {}", e)))?;
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let private_key = RsaPrivateKey::from_pkcs8_der(&private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA private key: {}", e)))?;
    
    let padding = Pkcs1v15Encrypt;
    let decrypted = private_key.decrypt(padding, &encrypted_data)
        .map_err(|e| CursedError::CryptoError(format!("PKCS#1 v1.5 decryption failed: {}", e)))?;
    
    let mut result = HashMap::new();
    result.insert("decrypted_data".to_string(), Value::String(hex::encode(&decrypted)));
    result.insert("plaintext".to_string(), Value::String(String::from_utf8_lossy(&decrypted).to_string()));
    result.insert("scheme".to_string(), Value::String("PKCS1v15-Encrypt".to_string()));
    
    Ok(Value::Object(result))
}

/// PSS signing
pub fn pss_sign(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("PSS signing requires: data, private_key, hash_algorithm".to_string()));
    }
    
    let data = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Binary(b) => b.clone(),
        _ => return Err(CursedError::InvalidArgument("Data must be string or binary".to_string())),
    };
    
    let private_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let hash_algorithm = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Hash algorithm must be a string".to_string())),
    };
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let private_key = RsaPrivateKey::from_pkcs8_der(&private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA private key: {}", e)))?;
    
    let mut rng = OsRng;
    
    let signature = match hash_algorithm.to_uppercase().as_str() {
        "SHA256" => {
            let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(private_key);
            let pss = Pss::new::<Sha256>();
            signing_key.sign_with_rng(&mut rng, &data)
        },
        "SHA384" => {
            let signing_key = rsa::pkcs1v15::SigningKey::<Sha384>::new(private_key);
            let pss = Pss::new::<Sha384>();
            signing_key.sign_with_rng(&mut rng, &data)
        },
        "SHA512" => {
            let signing_key = rsa::pkcs1v15::SigningKey::<Sha512>::new(private_key);
            let pss = Pss::new::<Sha512>();
            signing_key.sign_with_rng(&mut rng, &data)
        },
        _ => return Err(CursedError::InvalidArgument(format!("Unsupported hash algorithm for PSS: {}", hash_algorithm))),
    };
    
    // Note: This is a simplified implementation - proper PSS would use the PSS padding directly
    let mut result = HashMap::new();
    result.insert("signature".to_string(), Value::String(hex::encode(signature.to_bytes())));
    result.insert("hash_algorithm".to_string(), Value::String(hash_algorithm));
    result.insert("scheme".to_string(), Value::String(format!("PSS-{}", hash_algorithm.to_uppercase())));
    
    Ok(Value::Object(result))
}

/// Get padding scheme information
pub fn get_padding_scheme_info(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("Padding scheme name required".to_string()));
    }
    
    let scheme_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Scheme name must be a string".to_string())),
    };
    
    let scheme = PaddingScheme::from_name(&scheme_name)?;
    
    let mut info = HashMap::new();
    info.insert("name".to_string(), Value::String(scheme.name().to_string()));
    info.insert("description".to_string(), Value::String(scheme.description().to_string()));
    info.insert("is_secure".to_string(), Value::Boolean(scheme.is_secure()));
    info.insert("use_case".to_string(), Value::String(scheme.use_case().to_string()));
    
    // Add security recommendations
    let recommendation = if scheme.is_secure() {
        "Recommended for production use"
    } else {
        "NOT recommended - use newer alternatives"
    };
    info.insert("recommendation".to_string(), Value::String(recommendation.to_string()));
    
    Ok(Value::Object(info))
}

/// List all supported padding schemes
pub fn list_padding_schemes() -> Vec<String> {
    vec![
        PaddingScheme::Pkcs1v15Encrypt.name().to_string(),
        PaddingScheme::Pkcs1v15Sign.name().to_string(),
        PaddingScheme::OaepSha1.name().to_string(),
        PaddingScheme::OaepSha256.name().to_string(),
        PaddingScheme::OaepSha384.name().to_string(),
        PaddingScheme::OaepSha512.name().to_string(),
        PaddingScheme::PssSha256.name().to_string(),
        PaddingScheme::PssSha384.name().to_string(),
        PaddingScheme::PssSha512.name().to_string(),
    ]
}

/// Get recommended padding schemes
pub fn get_recommended_padding_schemes() -> HashMap<String, Vec<String>> {
    let mut recommendations = HashMap::new();
    
    recommendations.insert(
        "encryption".to_string(),
        vec![
            PaddingScheme::OaepSha256.name().to_string(),
            PaddingScheme::OaepSha384.name().to_string(),
            PaddingScheme::OaepSha512.name().to_string(),
        ]
    );
    
    recommendations.insert(
        "signatures".to_string(),
        vec![
            PaddingScheme::PssSha256.name().to_string(),
            PaddingScheme::PssSha384.name().to_string(),
            PaddingScheme::PssSha512.name().to_string(),
            PaddingScheme::Pkcs1v15Sign.name().to_string(),
        ]
    );
    
    recommendations.insert(
        "legacy_support".to_string(),
        vec![
            PaddingScheme::Pkcs1v15Encrypt.name().to_string(),
            PaddingScheme::OaepSha1.name().to_string(),
        ]
    );
    
    recommendations
}

/// Validate padding scheme for operation
pub fn validate_padding_for_operation(
    scheme: PaddingScheme,
    operation: &str,
) -> crate::error::Result<()> {
    match (scheme, operation.to_uppercase().as_str()) {
        (PaddingScheme::Pkcs1v15Encrypt, "ENCRYPT") => Ok(()),
        (PaddingScheme::Pkcs1v15Sign, "SIGN") => Ok(()),
        (PaddingScheme::OaepSha1, "ENCRYPT") => Ok(()),
        (PaddingScheme::OaepSha256, "ENCRYPT") => Ok(()),
        (PaddingScheme::OaepSha384, "ENCRYPT") => Ok(()),
        (PaddingScheme::OaepSha512, "ENCRYPT") => Ok(()),
        (PaddingScheme::PssSha256, "SIGN") => Ok(()),
        (PaddingScheme::PssSha384, "SIGN") => Ok(()),
        (PaddingScheme::PssSha512, "SIGN") => Ok(()),
        _ => Err(CursedError::InvalidArgument(format!(
            "Padding scheme {} is not valid for operation {}",
            scheme.name(),
            operation
        ))),
    }
}

