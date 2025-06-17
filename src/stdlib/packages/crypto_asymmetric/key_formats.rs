/// fr fr Comprehensive key format support for PKI operations
/// 
/// This module implements missing key format conversions including:
/// - PEM encoding/decoding for RSA, ECDSA, and Ed25519 keys
/// - JWK (JSON Web Key) format support
/// - SSH public key format support  
/// - OpenSSL format compatibility
/// - Cross-format conversion utilities

use crate::error::CursedError;
use crate::stdlib::value::Value;
use std::collections::HashMap;

// RSA key handling
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, DecodeRsaPrivateKey, DecodeRsaPublicKey, LineEnding};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey};

// ECC key handling
use p256::{SecretKey as P256SecretKey, PublicKey as P256PublicKey};
use p384::{SecretKey as P384SecretKey, PublicKey as P384PublicKey};
use p521::{SecretKey as P521SecretKey, PublicKey as P521PublicKey};
use elliptic_curve::{
    sec1::{ToEncodedPoint, FromEncodedPoint},
    pkcs8::{EncodePrivateKey as EcEncodePrivateKey, DecodePrivateKey as EcDecodePrivateKey},
};

// Ed25519 key handling
use ed25519_dalek::{SigningKey, VerifyingKey};

use crate::stdlib::packages::crypto_asymmetric::public_key::{PublicKeyAlgorithm, PublicKeyFormat};
use crate::stdlib::packages::crypto_asymmetric::private_key::PrivateKeyFormat;
use crate::stdlib::packages::crypto_asymmetric::ecc::EccCurve;

// Additional dependencies for enhanced format support
use base64::{Engine as _, engine::general_purpose};
use num_bigint::BigUint;

/// fr fr Enhanced public key format converter with full format support
pub fn convert_public_key_format_enhanced(
    public_key_hex: &str,
    algorithm: PublicKeyAlgorithm,
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    match algorithm {
        PublicKeyAlgorithm::Rsa => convert_rsa_public_key_enhanced(&public_key_bytes, from_format, to_format),
        PublicKeyAlgorithm::EcdsaP256 => convert_ecc_public_key_enhanced(&public_key_bytes, EccCurve::P256, from_format, to_format),
        PublicKeyAlgorithm::EcdsaP384 => convert_ecc_public_key_enhanced(&public_key_bytes, EccCurve::P384, from_format, to_format),
        PublicKeyAlgorithm::EcdsaP521 => convert_ecc_public_key_enhanced(&public_key_bytes, EccCurve::P521, from_format, to_format),
        PublicKeyAlgorithm::Ed25519 => convert_ed25519_public_key_enhanced(&public_key_bytes, from_format, to_format),
        PublicKeyAlgorithm::X25519 => {
            // X25519 only supports raw format currently
            if from_format == PublicKeyFormat::Raw && to_format == PublicKeyFormat::Raw {
                create_conversion_result("X25519", from_format, to_format, public_key_bytes.to_vec())
            } else {
                Err(CursedError::InvalidArgument("X25519 only supports raw format".to_string()))
            }
        },
    }
}

/// fr fr Enhanced private key format converter with full format support
pub fn convert_private_key_format_enhanced(
    private_key_hex: &str,
    algorithm: PublicKeyAlgorithm,
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    match algorithm {
        PublicKeyAlgorithm::Rsa => convert_rsa_private_key_enhanced(&private_key_bytes, from_format, to_format),
        PublicKeyAlgorithm::EcdsaP256 => convert_ecc_private_key_enhanced(&private_key_bytes, EccCurve::P256, from_format, to_format),
        PublicKeyAlgorithm::EcdsaP384 => convert_ecc_private_key_enhanced(&private_key_bytes, EccCurve::P384, from_format, to_format),
        PublicKeyAlgorithm::EcdsaP521 => convert_ecc_private_key_enhanced(&private_key_bytes, EccCurve::P521, from_format, to_format),
        PublicKeyAlgorithm::Ed25519 => convert_ed25519_private_key_enhanced(&private_key_bytes, from_format, to_format),
        PublicKeyAlgorithm::X25519 => {
            // X25519 only supports raw format currently
            if from_format == PrivateKeyFormat::Raw && to_format == PrivateKeyFormat::Raw {
                create_conversion_result("X25519", from_format.into(), to_format.into(), private_key_bytes.to_vec())
            } else {
                Err(CursedError::InvalidArgument("X25519 only supports raw format".to_string()))
            }
        },
    }
}

/// fr fr RSA public key format conversion with full format support
fn convert_rsa_public_key_enhanced(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Parse RSA public key from source format
    let public_key = match from_format {
        PublicKeyFormat::Pkcs8Der => {
            RsaPublicKey::from_public_key_der(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#8 DER: {}", e)))?
        },
        PublicKeyFormat::Pkcs1Der => {
            RsaPublicKey::from_pkcs1_der(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#1 DER: {}", e)))?
        },
        PublicKeyFormat::Pkcs8Pem => {
            let pem_str = String::from_utf8(public_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            RsaPublicKey::from_public_key_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#8 PEM: {}", e)))?
        },
        PublicKeyFormat::Pkcs1Pem => {
            let pem_str = String::from_utf8(public_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            RsaPublicKey::from_pkcs1_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#1 PEM: {}", e)))?
        },
        PublicKeyFormat::Jwk => {
            parse_rsa_public_key_from_jwk(public_key_bytes)?
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PublicKeyFormat::Pkcs8Der => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#8 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PublicKeyFormat::Pkcs1Der => {
            let der = public_key.to_pkcs1_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#1 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PublicKeyFormat::Pkcs8Pem => {
            let pem = public_key.to_public_key_pem(LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#8 PEM: {}", e)))?;
            pem.as_bytes().to_vec()
        },
        PublicKeyFormat::Pkcs1Pem => {
            let pem = public_key.to_pkcs1_pem(LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#1 PEM: {}", e)))?;
            pem.as_bytes().to_vec()
        },
        PublicKeyFormat::Jwk => {
            encode_rsa_public_key_to_jwk(&public_key)?.as_bytes().to_vec()
        },
        PublicKeyFormat::SshPublicKey => {
            encode_rsa_public_key_to_ssh(&public_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Encoding {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("RSA", from_format, to_format, converted_data)
}

/// fr fr RSA private key format conversion with full format support
fn convert_rsa_private_key_enhanced(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Parse RSA private key from source format
    let private_key = match from_format {
        PrivateKeyFormat::Pkcs8Der => {
            RsaPrivateKey::from_pkcs8_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#8 DER: {}", e)))?
        },
        PrivateKeyFormat::Pkcs1Der => {
            RsaPrivateKey::from_pkcs1_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#1 DER: {}", e)))?
        },
        PrivateKeyFormat::Pkcs8Pem => {
            let pem_str = String::from_utf8(private_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            RsaPrivateKey::from_pkcs8_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#8 PEM: {}", e)))?
        },
        PrivateKeyFormat::Pkcs1Pem => {
            let pem_str = String::from_utf8(private_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            RsaPrivateKey::from_pkcs1_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#1 PEM: {}", e)))?
        },
        PrivateKeyFormat::Jwk => {
            parse_rsa_private_key_from_jwk(private_key_bytes)?
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PrivateKeyFormat::Pkcs8Der => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#8 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PrivateKeyFormat::Pkcs1Der => {
            let der = private_key.to_pkcs1_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#1 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PrivateKeyFormat::Pkcs8Pem => {
            let pem = private_key.to_pkcs8_pem(LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#8 PEM: {}", e)))?;
            pem.to_string().as_bytes().to_vec()
        },
        PrivateKeyFormat::Pkcs1Pem => {
            let pem = private_key.to_pkcs1_pem(LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#1 PEM: {}", e)))?;
            pem.to_string().as_bytes().to_vec()
        },
        PrivateKeyFormat::Jwk => {
            encode_rsa_private_key_to_jwk(&private_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Encoding {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("RSA", from_format.into(), to_format.into(), converted_data)
}

/// fr fr ECC public key format conversion with full format support
fn convert_ecc_public_key_enhanced(
    public_key_bytes: &[u8],
    curve: EccCurve,
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    match curve {
        EccCurve::P256 => convert_p256_public_key(public_key_bytes, from_format, to_format),
        EccCurve::P384 => convert_p384_public_key(public_key_bytes, from_format, to_format),
        EccCurve::P521 => convert_p521_public_key(public_key_bytes, from_format, to_format),
    }
}

/// fr fr ECC private key format conversion with full format support
fn convert_ecc_private_key_enhanced(
    private_key_bytes: &[u8],
    curve: EccCurve,
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    match curve {
        EccCurve::P256 => convert_p256_private_key(private_key_bytes, from_format, to_format),
        EccCurve::P384 => convert_p384_private_key(private_key_bytes, from_format, to_format),
        EccCurve::P521 => convert_p521_private_key(private_key_bytes, from_format, to_format),
    }
}

/// fr fr P-256 public key conversion
fn convert_p256_public_key(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Parse P-256 public key
    let public_key = match from_format {
        PublicKeyFormat::Pkcs8Der => {
            P256PublicKey::from_public_key_der(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 PKCS#8 DER: {}", e)))?
        },
        PublicKeyFormat::Sec1Der => {
            let encoded_point = p256::EncodedPoint::from_bytes(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 SEC1 point: {}", e)))?;
            P256PublicKey::from_encoded_point(&encoded_point)
                .ok_or_else(|| CursedError::CryptoError("Invalid P-256 public key point".to_string()))?
        },
        PublicKeyFormat::Pkcs8Pem => {
            let pem_str = String::from_utf8(public_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            P256PublicKey::from_public_key_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 PEM: {}", e)))?
        },
        PublicKeyFormat::Jwk => {
            parse_p256_public_key_from_jwk(public_key_bytes)?
        },
        _ => return Err(CursedError::NotImplemented(format!("P-256 parsing for {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PublicKeyFormat::Pkcs8Der => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 PKCS#8 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PublicKeyFormat::Sec1Der => {
            public_key.to_encoded_point(false).as_bytes().to_vec()
        },
        PublicKeyFormat::Pkcs8Pem => {
            let pem = public_key.to_public_key_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 PEM: {}", e)))?;
            pem.as_bytes().to_vec()
        },
        PublicKeyFormat::Jwk => {
            encode_p256_public_key_to_jwk(&public_key)?.as_bytes().to_vec()
        },
        PublicKeyFormat::SshPublicKey => {
            encode_p256_public_key_to_ssh(&public_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("P-256 encoding for {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("ECDSA P-256", from_format, to_format, converted_data)
}

/// fr fr P-256 private key conversion
fn convert_p256_private_key(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Parse P-256 private key
    let private_key = match from_format {
        PrivateKeyFormat::Pkcs8Der => {
            P256SecretKey::from_pkcs8_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 PKCS#8 DER: {}", e)))?
        },
        PrivateKeyFormat::Sec1Der => {
            P256SecretKey::from_sec1_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 SEC1 DER: {}", e)))?
        },
        PrivateKeyFormat::Pkcs8Pem => {
            let pem_str = String::from_utf8(private_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            P256SecretKey::from_pkcs8_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 PEM: {}", e)))?
        },
        PrivateKeyFormat::Jwk => {
            parse_p256_private_key_from_jwk(private_key_bytes)?
        },
        _ => return Err(CursedError::NotImplemented(format!("P-256 private key parsing for {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PrivateKeyFormat::Pkcs8Der => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 PKCS#8 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PrivateKeyFormat::Sec1Der => {
            let der = private_key.to_sec1_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 SEC1 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PrivateKeyFormat::Pkcs8Pem => {
            let pem = private_key.to_pkcs8_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 PEM: {}", e)))?;
            pem.to_string().as_bytes().to_vec()
        },
        PrivateKeyFormat::Jwk => {
            encode_p256_private_key_to_jwk(&private_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("P-256 private key encoding for {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("ECDSA P-256", from_format.into(), to_format.into(), converted_data)
}

/// fr fr Similar implementations for P-384 and P-521 (condensed for brevity)
fn convert_p384_public_key(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Parse P-384 public key
    let public_key = match from_format {
        PublicKeyFormat::Pkcs8Der => {
            P384PublicKey::from_public_key_der(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 PKCS#8 DER: {}", e)))?
        },
        PublicKeyFormat::Sec1Der => {
            let encoded_point = p384::EncodedPoint::from_bytes(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 SEC1 point: {}", e)))?;
            P384PublicKey::from_encoded_point(&encoded_point)
                .ok_or_else(|| CursedError::CryptoError("Invalid P-384 public key point".to_string()))?
        },
        PublicKeyFormat::Pkcs8Pem => {
            let pem_str = String::from_utf8(public_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            P384PublicKey::from_public_key_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 PEM: {}", e)))?
        },
        PublicKeyFormat::Jwk => {
            parse_p384_public_key_from_jwk(public_key_bytes)?
        },
        _ => return Err(CursedError::NotImplemented(format!("P-384 parsing for {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PublicKeyFormat::Pkcs8Der => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 PKCS#8 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PublicKeyFormat::Sec1Der => {
            public_key.to_encoded_point(false).as_bytes().to_vec()
        },
        PublicKeyFormat::Pkcs8Pem => {
            let pem = public_key.to_public_key_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 PEM: {}", e)))?;
            pem.as_bytes().to_vec()
        },
        PublicKeyFormat::Jwk => {
            encode_p384_public_key_to_jwk(&public_key)?.as_bytes().to_vec()
        },
        PublicKeyFormat::SshPublicKey => {
            encode_p384_public_key_to_ssh(&public_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("P-384 encoding for {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("ECDSA P-384", from_format, to_format, converted_data)
}

fn convert_p384_private_key(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Parse P-384 private key
    let private_key = match from_format {
        PrivateKeyFormat::Pkcs8Der => {
            P384SecretKey::from_pkcs8_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 PKCS#8 DER: {}", e)))?
        },
        PrivateKeyFormat::Sec1Der => {
            P384SecretKey::from_sec1_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 SEC1 DER: {}", e)))?
        },
        PrivateKeyFormat::Pkcs8Pem => {
            let pem_str = String::from_utf8(private_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            P384SecretKey::from_pkcs8_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 PEM: {}", e)))?
        },
        PrivateKeyFormat::Jwk => {
            parse_p384_private_key_from_jwk(private_key_bytes)?
        },
        _ => return Err(CursedError::NotImplemented(format!("P-384 private key parsing for {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PrivateKeyFormat::Pkcs8Der => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 PKCS#8 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PrivateKeyFormat::Sec1Der => {
            let der = private_key.to_sec1_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 SEC1 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PrivateKeyFormat::Pkcs8Pem => {
            let pem = private_key.to_pkcs8_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 PEM: {}", e)))?;
            pem.to_string().as_bytes().to_vec()
        },
        PrivateKeyFormat::Jwk => {
            encode_p384_private_key_to_jwk(&private_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("P-384 private key encoding for {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("ECDSA P-384", from_format.into(), to_format.into(), converted_data)
}

fn convert_p521_public_key(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Parse P-521 public key
    let public_key = match from_format {
        PublicKeyFormat::Pkcs8Der => {
            P521PublicKey::from_public_key_der(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-521 PKCS#8 DER: {}", e)))?
        },
        PublicKeyFormat::Sec1Der => {
            let encoded_point = p521::EncodedPoint::from_bytes(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-521 SEC1 point: {}", e)))?;
            P521PublicKey::from_encoded_point(&encoded_point)
                .ok_or_else(|| CursedError::CryptoError("Invalid P-521 public key point".to_string()))?
        },
        PublicKeyFormat::Pkcs8Pem => {
            let pem_str = String::from_utf8(public_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            P521PublicKey::from_public_key_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-521 PEM: {}", e)))?
        },
        PublicKeyFormat::Jwk => {
            parse_p521_public_key_from_jwk(public_key_bytes)?
        },
        _ => return Err(CursedError::NotImplemented(format!("P-521 parsing for {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PublicKeyFormat::Pkcs8Der => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-521 PKCS#8 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PublicKeyFormat::Sec1Der => {
            public_key.to_encoded_point(false).as_bytes().to_vec()
        },
        PublicKeyFormat::Pkcs8Pem => {
            let pem = public_key.to_public_key_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-521 PEM: {}", e)))?;
            pem.as_bytes().to_vec()
        },
        PublicKeyFormat::Jwk => {
            encode_p521_public_key_to_jwk(&public_key)?.as_bytes().to_vec()
        },
        PublicKeyFormat::SshPublicKey => {
            encode_p521_public_key_to_ssh(&public_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("P-521 encoding for {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("ECDSA P-521", from_format, to_format, converted_data)
}

fn convert_p521_private_key(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Parse P-521 private key
    let private_key = match from_format {
        PrivateKeyFormat::Pkcs8Der => {
            P521SecretKey::from_pkcs8_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-521 PKCS#8 DER: {}", e)))?
        },
        PrivateKeyFormat::Sec1Der => {
            P521SecretKey::from_sec1_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-521 SEC1 DER: {}", e)))?
        },
        PrivateKeyFormat::Pkcs8Pem => {
            let pem_str = String::from_utf8(private_key_bytes.to_vec())
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid PEM string: {}", e)))?;
            P521SecretKey::from_pkcs8_pem(&pem_str)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-521 PEM: {}", e)))?
        },
        PrivateKeyFormat::Jwk => {
            parse_p521_private_key_from_jwk(private_key_bytes)?
        },
        _ => return Err(CursedError::NotImplemented(format!("P-521 private key parsing for {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PrivateKeyFormat::Pkcs8Der => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-521 PKCS#8 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PrivateKeyFormat::Sec1Der => {
            let der = private_key.to_sec1_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-521 SEC1 DER: {}", e)))?;
            der.as_bytes().to_vec()
        },
        PrivateKeyFormat::Pkcs8Pem => {
            let pem = private_key.to_pkcs8_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-521 PEM: {}", e)))?;
            pem.to_string().as_bytes().to_vec()
        },
        PrivateKeyFormat::Jwk => {
            encode_p521_private_key_to_jwk(&private_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("P-521 private key encoding for {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("ECDSA P-521", from_format.into(), to_format.into(), converted_data)
}

/// fr fr Ed25519 key format conversion
fn convert_ed25519_public_key_enhanced(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Parse Ed25519 public key
    let public_key = match from_format {
        PublicKeyFormat::Raw => {
            if public_key_bytes.len() != 32 {
                return Err(CursedError::InvalidArgument("Ed25519 public key must be 32 bytes".to_string()));
            }
            let mut key_bytes = [0u8; 32];
            key_bytes.copy_from_slice(public_key_bytes);
            VerifyingKey::from_bytes(&key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Invalid Ed25519 public key: {}", e)))?
        },
        PublicKeyFormat::Pkcs8Der => {
            // Parse Ed25519 public key from PKCS#8 DER
            if public_key_bytes.len() == 32 {
                // Raw 32-byte key
                let mut key_bytes = [0u8; 32];
                key_bytes.copy_from_slice(public_key_bytes);
                VerifyingKey::from_bytes(&key_bytes)
                    .map_err(|e| CursedError::CryptoError(format!("Invalid Ed25519 public key: {}", e)))?
            } else {
                // Try PKCS#8 DER format
                return Err(CursedError::NotImplemented("Ed25519 PKCS#8 DER parsing not fully implemented yet".to_string()));
            }
        },
        _ => return Err(CursedError::NotImplemented(format!("Ed25519 public key parsing for {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PublicKeyFormat::Raw => {
            public_key.as_bytes().to_vec()
        },
        PublicKeyFormat::Jwk => {
            encode_ed25519_public_key_to_jwk(&public_key)?.as_bytes().to_vec()
        },
        PublicKeyFormat::SshPublicKey => {
            encode_ed25519_public_key_to_ssh(&public_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Ed25519 public key encoding for {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("Ed25519", from_format, to_format, converted_data)
}

fn convert_ed25519_private_key_enhanced(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Parse Ed25519 private key
    let private_key = match from_format {
        PrivateKeyFormat::Raw => {
            if private_key_bytes.len() != 32 {
                return Err(CursedError::InvalidArgument("Ed25519 private key must be 32 bytes".to_string()));
            }
            let mut key_bytes = [0u8; 32];
            key_bytes.copy_from_slice(private_key_bytes);
            SigningKey::from_bytes(&key_bytes)
        },
        _ => return Err(CursedError::NotImplemented(format!("Ed25519 private key parsing for {} format not implemented", from_format.name()))),
    };
    
    // Convert to target format
    let converted_data = match to_format {
        PrivateKeyFormat::Raw => {
            private_key.to_bytes().to_vec()
        },
        PrivateKeyFormat::Jwk => {
            encode_ed25519_private_key_to_jwk(&private_key)?.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Ed25519 private key encoding for {} format not implemented", to_format.name()))),
    };
    
    create_conversion_result("Ed25519", from_format.into(), to_format.into(), converted_data)
}

/// fr fr JWK format parsers and encoders

fn parse_rsa_public_key_from_jwk(jwk_bytes: &[u8]) -> Result<RsaPublicKey, CursedError> {
    let jwk_str = String::from_utf8(jwk_bytes.to_vec())
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK string: {}", e)))?;
    
    // Basic JWK parsing (in production, would use proper JSON parsing)
    // This is a simplified implementation for demonstration
    if jwk_str.contains("\"kty\":\"RSA\"") {
        // Extract n and e parameters from JWK
        // For now, return error as full JWK parsing is complex
        Err(CursedError::NotImplemented("Full JWK parsing not yet implemented".to_string()))
    } else {
        Err(CursedError::InvalidArgument("Invalid RSA JWK format".to_string()))
    }
}

fn parse_rsa_private_key_from_jwk(jwk_bytes: &[u8]) -> Result<RsaPrivateKey, CursedError> {
    use serde_json::Value as JsonValue;
    
    let jwk_str = String::from_utf8(jwk_bytes.to_vec())
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK string: {}", e)))?;
    
    let jwk: JsonValue = serde_json::from_str(&jwk_str)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK JSON: {}", e)))?;
    
    // Validate key type
    if jwk["kty"] != "RSA" {
        return Err(CursedError::InvalidArgument("JWK must be RSA key type".to_string()));
    }
    
    // Extract required parameters
    let n_b64 = jwk["n"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'n' parameter in RSA JWK".to_string()))?;
    let e_b64 = jwk["e"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'e' parameter in RSA JWK".to_string()))?;
    let d_b64 = jwk["d"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'd' parameter in RSA private JWK".to_string()))?;
    
    // Decode base64url parameters
    let n_bytes = base64::decode_config(n_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'n' base64: {}", e)))?;
    let e_bytes = base64::decode_config(e_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'e' base64: {}", e)))?;
    let d_bytes = base64::decode_config(d_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'd' base64: {}", e)))?;
    
    // Convert to RSA components
    use num_bigint::BigUint;
    let n = BigUint::from_bytes_be(&n_bytes);
    let e = BigUint::from_bytes_be(&e_bytes);
    let d = BigUint::from_bytes_be(&d_bytes);
    
    // Create RSA private key
    let private_key = RsaPrivateKey::from_components(n, e, d, vec![], vec![])
        .map_err(|e| CursedError::CryptoError(format!("Failed to create RSA private key from JWK: {}", e)))?;
    
    Ok(private_key)
}

fn encode_rsa_public_key_to_jwk(public_key: &RsaPublicKey) -> Result<String, CursedError> {
    // Basic JWK encoding for RSA public key
    // In production, would use proper base64url encoding and JSON formatting
    let n = public_key.n();
    let e = public_key.e();
    
    let jwk = format!(
        r#"{{"kty":"RSA","use":"sig","n":"{}","e":"{}"}}"#,
        base64::encode_config(n.to_bytes_be(), base64::URL_SAFE_NO_PAD),
        base64::encode_config(e.to_bytes_be(), base64::URL_SAFE_NO_PAD)
    );
    
    Ok(jwk)
}

fn encode_rsa_private_key_to_jwk(private_key: &RsaPrivateKey) -> Result<String, CursedError> {
    // Extract RSA private key components
    let n = private_key.n();
    let e = private_key.e();
    let d = private_key.d();
    let p = &private_key.primes()[0];
    let q = &private_key.primes()[1];
    
    // Compute additional CRT parameters
    let dp = d % (p - 1u32);
    let dq = d % (q - 1u32);
    let qi = p.modinv(q).ok_or_else(|| CursedError::CryptoError("Failed to compute modular inverse for qi".to_string()))?;
    
    let jwk = format!(
        r#"{{"kty":"RSA","use":"sig","n":"{}","e":"{}","d":"{}","p":"{}","q":"{}","dp":"{}","dq":"{}","qi":"{}"}}"#,
        base64::encode_config(n.to_bytes_be(), base64::URL_SAFE_NO_PAD),
        base64::encode_config(e.to_bytes_be(), base64::URL_SAFE_NO_PAD),
        base64::encode_config(d.to_bytes_be(), base64::URL_SAFE_NO_PAD),
        base64::encode_config(p.to_bytes_be(), base64::URL_SAFE_NO_PAD),
        base64::encode_config(q.to_bytes_be(), base64::URL_SAFE_NO_PAD),
        base64::encode_config(dp.to_bytes_be(), base64::URL_SAFE_NO_PAD),
        base64::encode_config(dq.to_bytes_be(), base64::URL_SAFE_NO_PAD),
        base64::encode_config(qi.to_bytes_be(), base64::URL_SAFE_NO_PAD)
    );
    
    Ok(jwk)
}

fn encode_rsa_public_key_to_ssh(public_key: &RsaPublicKey) -> Result<String, CursedError> {
    // Get RSA parameters
    let n = public_key.n();
    let e = public_key.e();
    
    // Convert to SSH wire format
    let mut ssh_data = Vec::new();
    
    // Algorithm identifier
    let algorithm = b"ssh-rsa";
    ssh_data.extend_from_slice(&(algorithm.len() as u32).to_be_bytes());
    ssh_data.extend_from_slice(algorithm);
    
    // Public exponent (e)
    let e_bytes = e.to_bytes_be();
    ssh_data.extend_from_slice(&(e_bytes.len() as u32).to_be_bytes());
    ssh_data.extend_from_slice(&e_bytes);
    
    // Modulus (n) 
    let n_bytes = n.to_bytes_be();
    ssh_data.extend_from_slice(&(n_bytes.len() as u32).to_be_bytes());
    ssh_data.extend_from_slice(&n_bytes);
    
    // Base64 encode
    let b64_data = general_purpose::STANDARD.encode(&ssh_data);
    
    // Format as SSH public key
    Ok(format!("ssh-rsa {} cursed-generated-key", b64_data))
}

fn encode_p256_public_key_to_ssh(
    public_key: &P256PublicKey
) -> Result<String, CursedError> {
    encode_ecdsa_point_to_ssh(&public_key.to_encoded_point(false).as_bytes(), "nistp256")
}

fn encode_p384_public_key_to_ssh(
    public_key: &P384PublicKey
) -> Result<String, CursedError> {
    encode_ecdsa_point_to_ssh(&public_key.to_encoded_point(false).as_bytes(), "nistp384")
}

fn encode_p521_public_key_to_ssh(
    public_key: &P521PublicKey
) -> Result<String, CursedError> {
    encode_ecdsa_point_to_ssh(&public_key.to_encoded_point(false).as_bytes(), "nistp521")
}

fn encode_ecdsa_point_to_ssh(
    point_bytes: &[u8], 
    curve_name: &str
) -> Result<String, CursedError> {
    let algorithm = format!("ecdsa-sha2-{}", curve_name);
    
    let mut ssh_data = Vec::new();
    
    // Algorithm identifier
    ssh_data.extend_from_slice(&(algorithm.len() as u32).to_be_bytes());
    ssh_data.extend_from_slice(algorithm.as_bytes());
    
    // Curve identifier  
    ssh_data.extend_from_slice(&(curve_name.len() as u32).to_be_bytes());
    ssh_data.extend_from_slice(curve_name.as_bytes());
    
    // Public key point
    ssh_data.extend_from_slice(&(point_bytes.len() as u32).to_be_bytes());
    ssh_data.extend_from_slice(point_bytes);
    
    // Base64 encode
    let b64_data = general_purpose::STANDARD.encode(&ssh_data);
    
    Ok(format!("{} {} cursed-generated-key", algorithm, b64_data))
}

fn encode_ed25519_public_key_to_ssh(public_key: &VerifyingKey) -> Result<String, CursedError> {
    let algorithm = b"ssh-ed25519";
    
    let mut ssh_data = Vec::new();
    
    // Algorithm identifier
    ssh_data.extend_from_slice(&(algorithm.len() as u32).to_be_bytes());
    ssh_data.extend_from_slice(algorithm);
    
    // Public key bytes
    let key_bytes = public_key.as_bytes();
    ssh_data.extend_from_slice(&(key_bytes.len() as u32).to_be_bytes());
    ssh_data.extend_from_slice(key_bytes);
    
    // Base64 encode
    let b64_data = general_purpose::STANDARD.encode(&ssh_data);
    
    Ok(format!("ssh-ed25519 {} cursed-generated-key", b64_data))
}

fn parse_p256_public_key_from_jwk(jwk_bytes: &[u8]) -> Result<P256PublicKey, CursedError> {
    use serde_json::Value as JsonValue;
    
    let jwk_str = String::from_utf8(jwk_bytes.to_vec())
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK string: {}", e)))?;
    
    let jwk: JsonValue = serde_json::from_str(&jwk_str)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK JSON: {}", e)))?;
    
    // Validate key type and curve
    if jwk["kty"] != "EC" {
        return Err(CursedError::InvalidArgument("JWK must be EC key type".to_string()));
    }
    if jwk["crv"] != "P-256" {
        return Err(CursedError::InvalidArgument("JWK must be P-256 curve".to_string()));
    }
    
    // Extract x and y coordinates
    let x_b64 = jwk["x"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'x' parameter in EC JWK".to_string()))?;
    let y_b64 = jwk["y"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'y' parameter in EC JWK".to_string()))?;
    
    // Decode base64url coordinates
    let x_bytes = base64::decode_config(x_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'x' base64: {}", e)))?;
    let y_bytes = base64::decode_config(y_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'y' base64: {}", e)))?;
    
    // Create uncompressed point (0x04 prefix + x + y)
    let mut point_bytes = vec![0x04];
    point_bytes.extend_from_slice(&x_bytes);
    point_bytes.extend_from_slice(&y_bytes);
    
    // Parse as P-256 public key
    P256PublicKey::from_sec1_bytes(&point_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-256 public key from JWK: {}", e)))
}

fn parse_p256_private_key_from_jwk(jwk_bytes: &[u8]) -> Result<P256SecretKey, CursedError> {
    use serde_json::Value as JsonValue;
    
    let jwk_str = String::from_utf8(jwk_bytes.to_vec())
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK string: {}", e)))?;
    
    let jwk: JsonValue = serde_json::from_str(&jwk_str)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK JSON: {}", e)))?;
    
    // Validate key type and curve
    if jwk["kty"] != "EC" {
        return Err(CursedError::InvalidArgument("JWK must be EC key type".to_string()));
    }
    if jwk["crv"] != "P-256" {
        return Err(CursedError::InvalidArgument("JWK must be P-256 curve".to_string()));
    }
    
    // Extract private scalar
    let d_b64 = jwk["d"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'd' parameter in EC private JWK".to_string()))?;
    
    // Decode base64url scalar
    let d_bytes = base64::decode_config(d_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'd' base64: {}", e)))?;
    
    // Parse as P-256 private key
    P256SecretKey::from_bytes(&d_bytes.into())
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-256 private key from JWK: {}", e)))
}

fn encode_p256_public_key_to_jwk(public_key: &P256PublicKey) -> Result<String, CursedError> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let encoded_point = public_key.to_encoded_point(false);
    let point_bytes = encoded_point.as_bytes();
    
    // Extract x and y coordinates (skip 0x04 prefix)
    if point_bytes.len() != 65 || point_bytes[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-256 public key point encoding".to_string()));
    }
    
    let x_bytes = &point_bytes[1..33];
    let y_bytes = &point_bytes[33..65];
    
    let jwk = format!(
        r#"{{"kty":"EC","crv":"P-256","use":"sig","x":"{}","y":"{}"}}"#,
        base64::encode_config(x_bytes, base64::URL_SAFE_NO_PAD),
        base64::encode_config(y_bytes, base64::URL_SAFE_NO_PAD)
    );
    
    Ok(jwk)
}

fn encode_p256_private_key_to_jwk(private_key: &P256SecretKey) -> Result<String, CursedError> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let public_key = P256PublicKey::from(private_key);
    let encoded_point = public_key.to_encoded_point(false);
    let point_bytes = encoded_point.as_bytes();
    
    // Extract coordinates
    if point_bytes.len() != 65 || point_bytes[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-256 public key point encoding".to_string()));
    }
    
    let x_bytes = &point_bytes[1..33];
    let y_bytes = &point_bytes[33..65];
    let d_bytes = private_key.to_bytes();
    
    let jwk = format!(
        r#"{{"kty":"EC","crv":"P-256","use":"sig","x":"{}","y":"{}","d":"{}"}}"#,
        base64::encode_config(x_bytes, base64::URL_SAFE_NO_PAD),
        base64::encode_config(y_bytes, base64::URL_SAFE_NO_PAD),
        base64::encode_config(&d_bytes, base64::URL_SAFE_NO_PAD)
    );
    
    Ok(jwk)
}

fn encode_ed25519_public_key_to_jwk(public_key: &VerifyingKey) -> Result<String, CursedError> {
    let jwk = format!(
        r#"{{"kty":"OKP","crv":"Ed25519","x":"{}"}}"#,
        base64::encode_config(public_key.as_bytes(), base64::URL_SAFE_NO_PAD)
    );
    Ok(jwk)
}

fn encode_ed25519_private_key_to_jwk(private_key: &SigningKey) -> Result<String, CursedError> {
    let jwk = format!(
        r#"{{"kty":"OKP","crv":"Ed25519","d":"{}","x":"{}"}}"#,
        base64::encode_config(&private_key.to_bytes(), base64::URL_SAFE_NO_PAD),
        base64::encode_config(&private_key.verifying_key().as_bytes(), base64::URL_SAFE_NO_PAD)
    );
    Ok(jwk)
}

/// P-384 JWK functions
fn parse_p384_public_key_from_jwk(jwk_bytes: &[u8]) -> Result<P384PublicKey, CursedError> {
    use serde_json::Value as JsonValue;
    
    let jwk_str = String::from_utf8(jwk_bytes.to_vec())
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK string: {}", e)))?;
    
    let jwk: JsonValue = serde_json::from_str(&jwk_str)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK JSON: {}", e)))?;
    
    // Validate key type and curve
    if jwk["kty"] != "EC" {
        return Err(CursedError::InvalidArgument("JWK must be EC key type".to_string()));
    }
    if jwk["crv"] != "P-384" {
        return Err(CursedError::InvalidArgument("JWK must be P-384 curve".to_string()));
    }
    
    // Extract x and y coordinates
    let x_b64 = jwk["x"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'x' parameter in EC JWK".to_string()))?;
    let y_b64 = jwk["y"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'y' parameter in EC JWK".to_string()))?;
    
    // Decode base64url coordinates
    let x_bytes = base64::decode_config(x_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'x' base64: {}", e)))?;
    let y_bytes = base64::decode_config(y_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'y' base64: {}", e)))?;
    
    // Create uncompressed point (0x04 prefix + x + y)
    let mut point_bytes = vec![0x04];
    point_bytes.extend_from_slice(&x_bytes);
    point_bytes.extend_from_slice(&y_bytes);
    
    // Parse as P-384 public key
    P384PublicKey::from_sec1_bytes(&point_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-384 public key from JWK: {}", e)))
}

fn parse_p384_private_key_from_jwk(jwk_bytes: &[u8]) -> Result<P384SecretKey, CursedError> {
    use serde_json::Value as JsonValue;
    
    let jwk_str = String::from_utf8(jwk_bytes.to_vec())
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK string: {}", e)))?;
    
    let jwk: JsonValue = serde_json::from_str(&jwk_str)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK JSON: {}", e)))?;
    
    // Validate key type and curve
    if jwk["kty"] != "EC" {
        return Err(CursedError::InvalidArgument("JWK must be EC key type".to_string()));
    }
    if jwk["crv"] != "P-384" {
        return Err(CursedError::InvalidArgument("JWK must be P-384 curve".to_string()));
    }
    
    // Extract private scalar
    let d_b64 = jwk["d"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'd' parameter in EC private JWK".to_string()))?;
    
    // Decode base64url scalar
    let d_bytes = base64::decode_config(d_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'd' base64: {}", e)))?;
    
    // Parse as P-384 private key
    P384SecretKey::from_bytes(&d_bytes.into())
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-384 private key from JWK: {}", e)))
}

fn encode_p384_public_key_to_jwk(public_key: &P384PublicKey) -> Result<String, CursedError> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let encoded_point = public_key.to_encoded_point(false);
    let point_bytes = encoded_point.as_bytes();
    
    // Extract x and y coordinates (skip 0x04 prefix)
    if point_bytes.len() != 97 || point_bytes[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-384 public key point encoding".to_string()));
    }
    
    let x_bytes = &point_bytes[1..49];
    let y_bytes = &point_bytes[49..97];
    
    let jwk = format!(
        r#"{{"kty":"EC","crv":"P-384","use":"sig","x":"{}","y":"{}"}}"#,
        base64::encode_config(x_bytes, base64::URL_SAFE_NO_PAD),
        base64::encode_config(y_bytes, base64::URL_SAFE_NO_PAD)
    );
    
    Ok(jwk)
}

fn encode_p384_private_key_to_jwk(private_key: &P384SecretKey) -> Result<String, CursedError> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let public_key = P384PublicKey::from(private_key);
    let encoded_point = public_key.to_encoded_point(false);
    let point_bytes = encoded_point.as_bytes();
    
    // Extract coordinates
    if point_bytes.len() != 97 || point_bytes[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-384 public key point encoding".to_string()));
    }
    
    let x_bytes = &point_bytes[1..49];
    let y_bytes = &point_bytes[49..97];
    let d_bytes = private_key.to_bytes();
    
    let jwk = format!(
        r#"{{"kty":"EC","crv":"P-384","use":"sig","x":"{}","y":"{}","d":"{}"}}"#,
        base64::encode_config(x_bytes, base64::URL_SAFE_NO_PAD),
        base64::encode_config(y_bytes, base64::URL_SAFE_NO_PAD),
        base64::encode_config(&d_bytes, base64::URL_SAFE_NO_PAD)
    );
    
    Ok(jwk)
}

/// P-521 JWK functions
fn parse_p521_public_key_from_jwk(jwk_bytes: &[u8]) -> Result<P521PublicKey, CursedError> {
    use serde_json::Value as JsonValue;
    
    let jwk_str = String::from_utf8(jwk_bytes.to_vec())
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK string: {}", e)))?;
    
    let jwk: JsonValue = serde_json::from_str(&jwk_str)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK JSON: {}", e)))?;
    
    // Validate key type and curve
    if jwk["kty"] != "EC" {
        return Err(CursedError::InvalidArgument("JWK must be EC key type".to_string()));
    }
    if jwk["crv"] != "P-521" {
        return Err(CursedError::InvalidArgument("JWK must be P-521 curve".to_string()));
    }
    
    // Extract x and y coordinates
    let x_b64 = jwk["x"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'x' parameter in EC JWK".to_string()))?;
    let y_b64 = jwk["y"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'y' parameter in EC JWK".to_string()))?;
    
    // Decode base64url coordinates
    let x_bytes = base64::decode_config(x_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'x' base64: {}", e)))?;
    let y_bytes = base64::decode_config(y_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'y' base64: {}", e)))?;
    
    // Create uncompressed point (0x04 prefix + x + y)
    let mut point_bytes = vec![0x04];
    point_bytes.extend_from_slice(&x_bytes);
    point_bytes.extend_from_slice(&y_bytes);
    
    // Parse as P-521 public key
    P521PublicKey::from_sec1_bytes(&point_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-521 public key from JWK: {}", e)))
}

fn parse_p521_private_key_from_jwk(jwk_bytes: &[u8]) -> Result<P521SecretKey, CursedError> {
    use serde_json::Value as JsonValue;
    
    let jwk_str = String::from_utf8(jwk_bytes.to_vec())
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK string: {}", e)))?;
    
    let jwk: JsonValue = serde_json::from_str(&jwk_str)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK JSON: {}", e)))?;
    
    // Validate key type and curve
    if jwk["kty"] != "EC" {
        return Err(CursedError::InvalidArgument("JWK must be EC key type".to_string()));
    }
    if jwk["crv"] != "P-521" {
        return Err(CursedError::InvalidArgument("JWK must be P-521 curve".to_string()));
    }
    
    // Extract private scalar
    let d_b64 = jwk["d"].as_str()
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'd' parameter in EC private JWK".to_string()))?;
    
    // Decode base64url scalar
    let d_bytes = base64::decode_config(d_b64, base64::URL_SAFE_NO_PAD)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid 'd' base64: {}", e)))?;
    
    // Parse as P-521 private key
    P521SecretKey::from_bytes(&d_bytes.into())
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-521 private key from JWK: {}", e)))
}

fn encode_p521_public_key_to_jwk(public_key: &P521PublicKey) -> Result<String, CursedError> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let encoded_point = public_key.to_encoded_point(false);
    let point_bytes = encoded_point.as_bytes();
    
    // Extract x and y coordinates (skip 0x04 prefix)
    if point_bytes.len() != 133 || point_bytes[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-521 public key point encoding".to_string()));
    }
    
    let x_bytes = &point_bytes[1..67];
    let y_bytes = &point_bytes[67..133];
    
    let jwk = format!(
        r#"{{"kty":"EC","crv":"P-521","use":"sig","x":"{}","y":"{}"}}"#,
        base64::encode_config(x_bytes, base64::URL_SAFE_NO_PAD),
        base64::encode_config(y_bytes, base64::URL_SAFE_NO_PAD)
    );
    
    Ok(jwk)
}

fn encode_p521_private_key_to_jwk(private_key: &P521SecretKey) -> Result<String, CursedError> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let public_key = P521PublicKey::from(private_key);
    let encoded_point = public_key.to_encoded_point(false);
    let point_bytes = encoded_point.as_bytes();
    
    // Extract coordinates
    if point_bytes.len() != 133 || point_bytes[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-521 public key point encoding".to_string()));
    }
    
    let x_bytes = &point_bytes[1..67];
    let y_bytes = &point_bytes[67..133];
    let d_bytes = private_key.to_bytes();
    
    let jwk = format!(
        r#"{{"kty":"EC","crv":"P-521","use":"sig","x":"{}","y":"{}","d":"{}"}}"#,
        base64::encode_config(x_bytes, base64::URL_SAFE_NO_PAD),
        base64::encode_config(y_bytes, base64::URL_SAFE_NO_PAD),
        base64::encode_config(&d_bytes, base64::URL_SAFE_NO_PAD)
    );
    
    Ok(jwk)
}

/// fr fr Helper function to create conversion result
fn create_conversion_result(
    algorithm: &str,
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
    converted_data: Vec<u8>,
) -> Result<Value, CursedError> {
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String(algorithm.to_string()));
    result.insert("from_format".to_string(), Value::String(from_format.name().to_string()));
    result.insert("to_format".to_string(), Value::String(to_format.name().to_string()));
    result.insert("converted_key".to_string(), Value::String(hex::encode(converted_data)));
    result.insert("success".to_string(), Value::Bool(true));
    
    Ok(Value::Object(result))
}

/// fr fr Implement From trait for format conversion
impl From<PrivateKeyFormat> for PublicKeyFormat {
    fn from(private_format: PrivateKeyFormat) -> Self {
        match private_format {
            PrivateKeyFormat::Pkcs8Der => PublicKeyFormat::Pkcs8Der,
            PrivateKeyFormat::Pkcs1Der => PublicKeyFormat::Pkcs1Der,
            PrivateKeyFormat::Pkcs8Pem => PublicKeyFormat::Pkcs8Pem,
            PrivateKeyFormat::Pkcs1Pem => PublicKeyFormat::Pkcs1Pem,
            PrivateKeyFormat::Sec1Der => PublicKeyFormat::Sec1Der,
            PrivateKeyFormat::Jwk => PublicKeyFormat::Jwk,
            PrivateKeyFormat::Raw => PublicKeyFormat::Raw,
            PrivateKeyFormat::OpenSsh => PublicKeyFormat::SshPublicKey,
        }
    }
}

// Enhanced conversion functions are defined above and automatically available
