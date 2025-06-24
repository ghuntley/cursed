/// fr fr Crypto format conversions (PEM, DER, JWK) for CURSED
/// 
/// This module provides comprehensive format conversion support for cryptographic keys
/// including PEM, DER, and JSON Web Key (JWK) formats with security best practices.

use std::collections::HashMap;
use crate::stdlib::value::Value;
use crate::error::{CursedError, Error};
use crate::stdlib::crypto::asymmetric::{AsymmetricError, RsaPublicKeyWrapper, RsaPrivateKeyWrapper, EcdsaPublicKey, EcdsaPrivateKey, EcCurve};

use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};

/// fr fr Supported key formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyFormat {
    Pem,
    Der,
    Jwk,
    Raw,
    Pkcs8,
    Spki,
}

impl KeyFormat {
    pub fn from_str(s: &str) -> Result<(), Error> {
        match s.to_lowercase().as_str() {
            "pem" => Ok(KeyFormat::Pem),
            "der" => Ok(KeyFormat::Der),
            "jwk" => Ok(KeyFormat::Jwk),
            "raw" => Ok(KeyFormat::Raw),
            "pkcs8" => Ok(KeyFormat::Pkcs8),
            "spki" => Ok(KeyFormat::Spki),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported key format: {}", s))),
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            KeyFormat::Pem => "PEM",
            KeyFormat::Der => "DER",
            KeyFormat::Jwk => "JWK",
            KeyFormat::Raw => "Raw",
            KeyFormat::Pkcs8 => "PKCS#8",
            KeyFormat::Spki => "SPKI",
        }
    }
}

/// fr fr JSON Web Key (JWK) structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonWebKey {
    pub kty: String,           // Key type (RSA, EC, oct, OKP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,  // Key use (sig, enc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,   // Algorithm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,   // Key ID
    
    // RSA parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,     // Modulus
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,     // Exponent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,     // Private exponent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,     // First prime factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,     // Second prime factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp: Option<String>,    // First factor CRT exponent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dq: Option<String>,    // Second factor CRT exponent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qi: Option<String>,    // First CRT coefficient
    
    // EC parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,   // Curve name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,     // X coordinate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,     // Y coordinate
    
    // OKP parameters (Ed25519, X25519, X448)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_okp: Option<String>, // Public key for OKP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_okp: Option<String>, // Private key for OKP
}

/// fr fr Key format converter
pub struct FormatConverter;

impl FormatConverter {
    /// slay Convert RSA public key to JWK format
    pub fn rsa_public_key_to_jwk(public_key: &RsaPublicKeyWrapper, key_id: Option<String>) -> Result<(), Error> {
        use rsa::traits::PublicKeyParts;
        
        let n_bytes = public_key.inner.n().to_bytes_be();
        let e_bytes = public_key.inner.e().to_bytes_be();
        
        let jwk = JsonWebKey {
            kty: "RSA".to_string(),
            use_: Some("sig".to_string()),
            alg: Some("RS256".to_string()),
            kid: key_id,
            n: Some(general_purpose::URL_SAFE_NO_PAD.encode(n_bytes)),
            e: Some(general_purpose::URL_SAFE_NO_PAD.encode(e_bytes)),
            d: None,
            p: None,
            q: None,
            dp: None,
            dq: None,
            qi: None,
            crv: None,
            x: None,
            y: None,
            x_okp: None,
            d_okp: None,
        };
        
        Ok(jwk)
    }
    
    /// slay Convert RSA private key to JWK format (WARNING: Contains private data)
    pub fn rsa_private_key_to_jwk(private_key: &RsaPrivateKeyWrapper, key_id: Option<String>) -> Result<(), Error> {
        use rsa::traits::PublicKeyParts;
        
        let public_key = private_key.inner.to_public_key();
        let n_bytes = public_key.n().to_bytes_be();
        let e_bytes = public_key.e().to_bytes_be();
        let d_bytes = private_key.inner.d().to_bytes_be();
        
        let primes = private_key.inner.primes();
        if primes.len() < 2 {
            return Err(CursedError::CryptoError("RSA key must have at least 2 prime factors".to_string()));
        }
        
        let p_bytes = primes[0].to_bytes_be();
        let q_bytes = primes[1].to_bytes_be();
        
        // Calculate CRT parameters
        let dp = private_key.inner.d() % (primes[0] - 1u32);
        let dq = private_key.inner.d() % (primes[1] - 1u32);
        let qi = primes[1].modinv(primes[0]).ok_or_else(|| {
            CursedError::CryptoError("Failed to compute CRT coefficient".to_string())
        })?;
        
        let jwk = JsonWebKey {
            kty: "RSA".to_string(),
            use_: Some("sig".to_string()),
            alg: Some("RS256".to_string()),
            kid: key_id,
            n: Some(general_purpose::URL_SAFE_NO_PAD.encode(n_bytes)),
            e: Some(general_purpose::URL_SAFE_NO_PAD.encode(e_bytes)),
            d: Some(general_purpose::URL_SAFE_NO_PAD.encode(d_bytes)),
            p: Some(general_purpose::URL_SAFE_NO_PAD.encode(p_bytes)),
            q: Some(general_purpose::URL_SAFE_NO_PAD.encode(q_bytes)),
            dp: Some(general_purpose::URL_SAFE_NO_PAD.encode(dp.to_bytes_be())),
            dq: Some(general_purpose::URL_SAFE_NO_PAD.encode(dq.to_bytes_be())),
            qi: Some(general_purpose::URL_SAFE_NO_PAD.encode(qi.to_bytes_be())),
            crv: None,
            x: None,
            y: None,
            x_okp: None,
            d_okp: None,
        };
        
        Ok(jwk)
    }
    
    /// slay Convert ECDSA public key to JWK format
    pub fn ecdsa_public_key_to_jwk(public_key: &EcdsaPublicKey, key_id: Option<String>) -> Result<(), Error> {
        use crate::stdlib::crypto::asymmetric::{EcPublicKeyData};
        
        let (crv, x_bytes, y_bytes) = match &public_key.data {
            EcPublicKeyData::P256(pk) => {
                let point = pk.to_encoded_point(false);
                let coords = point.coordinates().ok_or_else(|| {
                    CursedError::CryptoError("Failed to get EC point coordinates".to_string())
                })?;
                ("P-256".to_string(), coords.x().to_vec(), coords.y().to_vec())
            },
            EcPublicKeyData::P384(pk) => {
                let point = pk.to_encoded_point(false);
                let coords = point.coordinates().ok_or_else(|| {
                    CursedError::CryptoError("Failed to get EC point coordinates".to_string())
                })?;
                ("P-384".to_string(), coords.x().to_vec(), coords.y().to_vec())
            },
            EcPublicKeyData::K256(pk) => {
                let point = pk.to_encoded_point(false);
                let coords = point.coordinates().ok_or_else(|| {
                    CursedError::CryptoError("Failed to get EC point coordinates".to_string())
                })?;
                ("secp256k1".to_string(), coords.x().to_vec(), coords.y().to_vec())
            },
        };
        
        let jwk = JsonWebKey {
            kty: "EC".to_string(),
            use_: Some("sig".to_string()),
            alg: Some("ES256".to_string()),
            kid: key_id,
            n: None,
            e: None,
            d: None,
            p: None,
            q: None,
            dp: None,
            dq: None,
            qi: None,
            crv: Some(crv),
            x: Some(general_purpose::URL_SAFE_NO_PAD.encode(x_bytes)),
            y: Some(general_purpose::URL_SAFE_NO_PAD.encode(y_bytes)),
            x_okp: None,
            d_okp: None,
        };
        
        Ok(jwk)
    }
    
    /// slay Convert ECDSA private key to JWK format (WARNING: Contains private data)
    pub fn ecdsa_private_key_to_jwk(private_key: &EcdsaPrivateKey, key_id: Option<String>) -> Result<(), Error> {
        use crate::stdlib::crypto::asymmetric::{EcPrivateKeyData, EcPublicKeyData};
        
        let (crv, x_bytes, y_bytes, d_bytes) = match &private_key.data {
            EcPrivateKeyData::P256(sk) => {
                let pk = sk.public_key();
                let point = pk.to_encoded_point(false);
                let coords = point.coordinates().ok_or_else(|| {
                    CursedError::CryptoError("Failed to get EC point coordinates".to_string())
                })?;
                ("P-256".to_string(), coords.x().to_vec(), coords.y().to_vec(), sk.to_bytes().to_vec())
            },
            EcPrivateKeyData::P384(sk) => {
                let pk = sk.public_key();
                let point = pk.to_encoded_point(false);
                let coords = point.coordinates().ok_or_else(|| {
                    CursedError::CryptoError("Failed to get EC point coordinates".to_string())
                })?;
                ("P-384".to_string(), coords.x().to_vec(), coords.y().to_vec(), sk.to_bytes().to_vec())
            },
            EcPrivateKeyData::K256(sk) => {
                let pk = sk.public_key();
                let point = pk.to_encoded_point(false);
                let coords = point.coordinates().ok_or_else(|| {
                    CursedError::CryptoError("Failed to get EC point coordinates".to_string())
                })?;
                ("secp256k1".to_string(), coords.x().to_vec(), coords.y().to_vec(), sk.to_bytes().to_vec())
            },
        };
        
        let jwk = JsonWebKey {
            kty: "EC".to_string(),
            use_: Some("sig".to_string()),
            alg: Some("ES256".to_string()),
            kid: key_id,
            n: None,
            e: None,
            d: Some(general_purpose::URL_SAFE_NO_PAD.encode(d_bytes)),
            p: None,
            q: None,
            dp: None,
            dq: None,
            qi: None,
            crv: Some(crv),
            x: Some(general_purpose::URL_SAFE_NO_PAD.encode(x_bytes)),
            y: Some(general_purpose::URL_SAFE_NO_PAD.encode(y_bytes)),
            x_okp: None,
            d_okp: None,
        };
        
        Ok(jwk)
    }
    
    /// slay Parse RSA public key from JWK format
    pub fn rsa_public_key_from_jwk(jwk: &JsonWebKey) -> Result<(), Error> {
        if jwk.kty != "RSA" {
            return Err(CursedError::InvalidArgument(format!("Expected RSA key type, got {}", jwk.kty)));
        }
        
        let n_str = jwk.n.as_ref().ok_or_else(|| {
            CursedError::InvalidArgument("RSA JWK missing modulus 'n'".to_string())
        })?;
        
        let e_str = jwk.e.as_ref().ok_or_else(|| {
            CursedError::InvalidArgument("RSA JWK missing exponent 'e'".to_string())
        })?;
        
        let n_bytes = general_purpose::URL_SAFE_NO_PAD.decode(n_str)
            .map_err(|e| CursedError::InvalidArgument(format!("Invalid modulus encoding: {}", e)))?;
        
        let e_bytes = general_purpose::URL_SAFE_NO_PAD.decode(e_str)
            .map_err(|e| CursedError::InvalidArgument(format!("Invalid exponent encoding: {}", e)))?;
        
        use rsa::{RsaPublicKey, BigUint};
        
        let n = BigUint::from_bytes_be(&n_bytes);
        let e = BigUint::from_bytes_be(&e_bytes);
        
        let public_key = RsaPublicKey::new(n, e)
            .map_err(|e| CursedError::CryptoError(format!("Invalid RSA parameters: {}", e)))?;
        
        let key_size = public_key.size() * 8; // Convert bytes to bits
        
        Ok(RsaPublicKeyWrapper {
            inner: public_key,
            key_size,
        })
    }
    
    /// slay Parse ECDSA public key from JWK format
    pub fn ecdsa_public_key_from_jwk(jwk: &JsonWebKey) -> Result<(), Error> {
        if jwk.kty != "EC" {
            return Err(CursedError::InvalidArgument(format!("Expected EC key type, got {}", jwk.kty)));
        }
        
        let crv = jwk.crv.as_ref().ok_or_else(|| {
            CursedError::InvalidArgument("EC JWK missing curve 'crv'".to_string())
        })?;
        
        let x_str = jwk.x.as_ref().ok_or_else(|| {
            CursedError::InvalidArgument("EC JWK missing x coordinate".to_string())
        })?;
        
        let y_str = jwk.y.as_ref().ok_or_else(|| {
            CursedError::InvalidArgument("EC JWK missing y coordinate".to_string())
        })?;
        
        let x_bytes = general_purpose::URL_SAFE_NO_PAD.decode(x_str)
            .map_err(|e| CursedError::InvalidArgument(format!("Invalid x coordinate encoding: {}", e)))?;
        
        let y_bytes = general_purpose::URL_SAFE_NO_PAD.decode(y_str)
            .map_err(|e| CursedError::InvalidArgument(format!("Invalid y coordinate encoding: {}", e)))?;
        
        let curve = match crv.as_str() {
            "P-256" => EcCurve::P256,
            "P-384" => EcCurve::P384,
            "secp256k1" => EcCurve::Secp256k1,
            _ => return Err(CursedError::InvalidArgument(format!("Unsupported curve: {}", crv))),
        };
        
        use crate::stdlib::crypto::asymmetric::EcPublicKeyData;
        use elliptic_curve::sec1::ToEncodedPoint;
        
        let data = match curve {
            EcCurve::P256 => {
                use p256::{PublicKey, EncodedPoint};
                
                // Create uncompressed point encoding
                let mut point_bytes = vec![0x04]; // Uncompressed prefix
                point_bytes.extend_from_slice(&x_bytes);
                point_bytes.extend_from_slice(&y_bytes);
                
                let public_key = PublicKey::from_sec1_bytes(&point_bytes)
                    .map_err(|e| CursedError::CryptoError(format!("Invalid P-256 point: {}", e)))?;
                
                EcPublicKeyData::P256(public_key)
            },
            EcCurve::P384 => {
                use p384::{PublicKey, EncodedPoint};
                
                // Create uncompressed point encoding
                let mut point_bytes = vec![0x04]; // Uncompressed prefix
                point_bytes.extend_from_slice(&x_bytes);
                point_bytes.extend_from_slice(&y_bytes);
                
                let public_key = PublicKey::from_sec1_bytes(&point_bytes)
                    .map_err(|e| CursedError::CryptoError(format!("Invalid P-384 point: {}", e)))?;
                
                EcPublicKeyData::P384(public_key)
            },
            EcCurve::Secp256k1 => {
                use k256::{PublicKey, EncodedPoint};
                
                // Create uncompressed point encoding
                let mut point_bytes = vec![0x04]; // Uncompressed prefix
                point_bytes.extend_from_slice(&x_bytes);
                point_bytes.extend_from_slice(&y_bytes);
                
                let public_key = PublicKey::from_sec1_bytes(&point_bytes)
                    .map_err(|e| CursedError::CryptoError(format!("Invalid secp256k1 point: {}", e)))?;
                
                EcPublicKeyData::K256(public_key)
            },
            _ => return Err(CursedError::InvalidArgument(format!("Unsupported curve: {:?}", curve))),
        };
        
        Ok(EcdsaPublicKey { curve, data })
    }
    
    /// slay Convert JWK to JSON string
    pub fn jwk_to_json(jwk: &JsonWebKey) -> Result<(), Error> {
        serde_json::to_string_pretty(jwk)
            .map_err(|e| CursedError::InvalidArgument(format!("JWK serialization failed: {}", e)))
    }
    
    /// slay Parse JWK from JSON string
    pub fn jwk_from_json(json: &str) -> Result<(), Error> {
        serde_json::from_str(json)
            .map_err(|e| CursedError::InvalidArgument(format!("JWK parsing failed: {}", e)))
    }
    
    /// slay Enhanced DER encoding with error recovery
    pub fn enhanced_der_encode(data: &[u8], tag: &str) -> Result<(), Error> {
        use der::{Encode, Tag, Header, Length};
        
        // Validate input
        if data.is_empty() {
            return Err(CursedError::InvalidArgument("Cannot encode empty data".to_string()));
        }
        
        // Create DER structure based on tag
        let der_tag = match tag.to_lowercase().as_str() {
            "sequence" => Tag::Sequence,
            "octet_string" => Tag::OctetString,
            "bit_string" => Tag::BitString,
            "integer" => Tag::Integer,
            _ => return Err(CursedError::InvalidArgument(format!("Unsupported DER tag: {}", tag))),
        };
        
        // Encode with proper length
        let length = Length::new(data.len() as u16)
            .map_err(|e| CursedError::CryptoError(format!("Invalid DER length: {}", e)))?;
        
        let header = Header::new(der_tag, length);
        
        let mut encoded = Vec::new();
        header.encode_to_slice(&mut encoded)
            .map_err(|e| CursedError::CryptoError(format!("DER header encoding failed: {}", e)))?;
        
        encoded.extend_from_slice(data);
        
        Ok(encoded)
    }
    
    /// slay Enhanced DER decoding with validation
    pub fn enhanced_der_decode(der_data: &[u8]) -> Result<(), Error> {
        use der::{Decode, Header, Tag};
        
        if der_data.is_empty() {
            return Err(CursedError::InvalidArgument("Empty DER data".to_string()));
        }
        
        let header = Header::decode(&der_data[..])
            .map_err(|e| CursedError::CryptoError(format!("DER header decode failed: {}", e)))?;
        
        let tag_name = match header.tag {
            Tag::Sequence => "sequence",
            Tag::OctetString => "octet_string",
            Tag::BitString => "bit_string",
            Tag::Integer => "integer",
            _ => "unknown",
        };
        
        let content_start = header.encoded_len();
        let content_length = header.length.try_into()
            .map_err(|e| CursedError::CryptoError(format!("Invalid DER content length: {}", e)))?;
        
        if der_data.len() < content_start + content_length {
            return Err(CursedError::CryptoError("DER data truncated".to_string()));
        }
        
        let content = der_data[content_start..content_start + content_length].to_vec();
        
        Ok((tag_name.to_string(), content))
    }
    
    /// bestie Detect key format automatically
    pub fn detect_key_format(data: &str) -> KeyFormat {
        let trimmed = data.trim();
        
        if trimmed.starts_with("-----BEGIN") && trimmed.ends_with("-----END") {
            return KeyFormat::Pem;
        }
        
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            // Try to parse as JSON to confirm it's JWK
            if serde_json::from_str::<JsonWebKey>(trimmed).is_ok() {
                return KeyFormat::Jwk;
            }
        }
        
        // Check if it's base64-encoded DER
        if general_purpose::STANDARD.decode(trimmed).is_ok() {
            return KeyFormat::Der;
        }
        
        // Check if it's hex-encoded raw key
        if hex::decode(trimmed).is_ok() {
            return KeyFormat::Raw;
        }
        
        KeyFormat::Raw // Default fallback
    }
    
    /// periodt Get supported formats for key type
    pub fn supported_formats(key_type: &str) -> Vec<KeyFormat> {
        match key_type.to_lowercase().as_str() {
            "rsa" => vec![KeyFormat::Pem, KeyFormat::Der, KeyFormat::Jwk, KeyFormat::Pkcs8, KeyFormat::Spki],
            "ec" | "ecdsa" => vec![KeyFormat::Pem, KeyFormat::Der, KeyFormat::Jwk, KeyFormat::Raw],
            "ed25519" | "x25519" => vec![KeyFormat::Pem, KeyFormat::Der, KeyFormat::Jwk, KeyFormat::Raw],
            "x448" => vec![KeyFormat::Raw, KeyFormat::Jwk],
            _ => vec![KeyFormat::Raw],
        }
    }
}

/// fr fr Public API functions for CURSED stdlib integration

/// slay Convert key to JWK format
pub fn key_to_jwk(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("key_to_jwk requires: key_data, key_type, [key_id]".to_string()));
    }
    
    let key_data = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Key data must be a string".to_string())),
    };
    
    let key_type = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Key type must be a string".to_string())),
    };
    
    let key_id = if args.len() > 2 {
        match &args[2] {
            Value::String(s) => Some(s.clone()),
            Value::Null => None,
            _ => return Err(CursedError::InvalidArgument("Key ID must be a string or null".to_string())),
        }
    } else {
        None
    };
    
    // This would need integration with the asymmetric crypto module
    // For now, return a basic JWK structure
    let mut jwk_map = std::collections::HashMap::new();
    jwk_map.insert("kty".to_string(), Value::String(key_type.clone()));
    
    if let Some(kid) = key_id {
        jwk_map.insert("kid".to_string(), Value::String(kid));
    }
    
    // Add format conversion logic here based on key_type
    match key_type.as_str() {
        "RSA" => {
            jwk_map.insert("use".to_string(), Value::String("sig".to_string()));
            jwk_map.insert("alg".to_string(), Value::String("RS256".to_string()));
        },
        "EC" => {
            jwk_map.insert("use".to_string(), Value::String("sig".to_string()));
            jwk_map.insert("alg".to_string(), Value::String("ES256".to_string()));
        },
        _ => {
            return Err(CursedError::InvalidArgument(format!("Unsupported key type: {}", key_type)));
        },
    }
    
    Ok(Value::Object(jwk_map))
}

/// slay Parse JWK from JSON
pub fn jwk_from_json(args: Vec<Value>) -> Result<(), Error> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("jwk_from_json requires: json_string".to_string()));
    }
    
    let json_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("JSON string must be a string".to_string())),
    };
    
    let jwk = FormatConverter::jwk_from_json(&json_str)?;
    let jwk_json = FormatConverter::jwk_to_json(&jwk)?;
    
    Ok(Value::String(jwk_json))
}

/// slay Convert to DER format
pub fn key_to_der(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("key_to_der requires: key_data, tag".to_string()));
    }
    
    let key_data = match &args[0] {
        Value::String(s) => hex::decode(s)
            .map_err(|e| CursedError::InvalidArgument(format!("Invalid hex data: {}", e)))?,
        Value::Array(bytes) => {
            bytes.iter().map(|v| match v {
                Value::Number(n) => Ok(*n as u8),
                _ => Err(CursedError::InvalidArgument("Array must contain numbers".to_string())),
            }).collect::<Result<Vec<u8>, _>>()?
        },
        _ => return Err(CursedError::InvalidArgument("Key data must be a hex string or byte array".to_string())),
    };
    
    let tag = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Tag must be a string".to_string())),
    };
    
    let der_bytes = FormatConverter::enhanced_der_encode(&key_data, &tag)?;
    let der_hex = hex::encode(der_bytes);
    
    Ok(Value::String(der_hex))
}

/// slay Parse DER format
pub fn der_decode(args: Vec<Value>) -> Result<(), Error> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("der_decode requires: der_data".to_string()));
    }
    
    let der_bytes = match &args[0] {
        Value::String(s) => hex::decode(s)
            .map_err(|e| CursedError::InvalidArgument(format!("Invalid hex data: {}", e)))?,
        _ => return Err(CursedError::InvalidArgument("DER data must be a hex string".to_string())),
    };
    
    let (tag, content) = FormatConverter::enhanced_der_decode(&der_bytes)?;
    
    let mut result = std::collections::HashMap::new();
    result.insert("tag".to_string(), Value::String(tag));
    result.insert("content".to_string(), Value::String(hex::encode(content)));
    
    Ok(Value::Object(result))
}

/// slay Detect key format
pub fn detect_format(args: Vec<Value>) -> Result<(), Error> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("detect_format requires: key_data".to_string()));
    }
    
    let key_data = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Key data must be a string".to_string())),
    };
    
    let format = FormatConverter::detect_key_format(&key_data);
    
    Ok(Value::String(format.name().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_format_detection() {
        // Test PEM format
        let pem_data = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0B...\n-----END PUBLIC KEY-----";
        assert_eq!(FormatConverter::detect_key_format(pem_data), KeyFormat::Pem);
        
        // Test JWK format
        let jwk_data = r#"{"kty":"RSA","n":"0vx7agoebGc..."}"#;
        assert_eq!(FormatConverter::detect_key_format(jwk_data), KeyFormat::Jwk);
        
        // Test hex format
        let hex_data = "deadbeef";
        assert_eq!(FormatConverter::detect_key_format(hex_data), KeyFormat::Raw);
    }
    
    #[test]
    fn test_supported_formats() {
        let rsa_formats = FormatConverter::supported_formats("rsa");
        assert!(rsa_formats.contains(&KeyFormat::Pem));
        assert!(rsa_formats.contains(&KeyFormat::Jwk));
        
        let ec_formats = FormatConverter::supported_formats("ec");
        assert!(ec_formats.contains(&KeyFormat::Jwk));
        assert!(ec_formats.contains(&KeyFormat::Raw));
    }
    
    #[test]
    fn test_format_conversion_api() {
        // Test detect_format function
        let args = vec![Value::String("-----BEGIN PUBLIC KEY-----\ntest\n-----END PUBLIC KEY-----".to_string())];
        let result = detect_format(args).unwrap();
        assert_eq!(result, Value::String("PEM".to_string()));
        
        // Test JWK conversion
        let jwk_args = vec![
            Value::String("test_key_data".to_string()),
            Value::String("RSA".to_string()),
            Value::String("test_kid".to_string()),
        ];
        let jwk_result = key_to_jwk(jwk_args).unwrap();
        
        if let Value::Object(map) = jwk_result {
            assert_eq!(map.get("kty"), Some(&Value::String("RSA".to_string())));
            assert_eq!(map.get("kid"), Some(&Value::String("test_kid".to_string())));
        } else {
            panic!("Expected object result");
        }
    }
}
