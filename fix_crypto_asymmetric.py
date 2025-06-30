#!/usr/bin/env python3

import os

def add_missing_types_to_file(file_path, content_to_add):
    """Add missing types to a file if they don't already exist"""
    try:
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                existing = f.read()
            
            # Check if content already exists
            if content_to_add.strip() not in existing:
                with open(file_path, 'a') as f:
                    f.write('\n\n' + content_to_add)
                print(f"Added missing types to {file_path}")
        else:
            print(f"File {file_path} does not exist")
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")

def fix_crypto_asymmetric():
    """Fix crypto asymmetric modules with missing implementations"""
    
    # Fix key generator types
    add_missing_types_to_file('src/stdlib/packages/crypto_asymmetric/key_generator.rs', '''
// Key Generator specific types
#[derive(Debug, Clone)]
pub struct KeyGenerator {
    pub algorithm: AsymmetricAlgorithm,
}

#[derive(Debug, Clone)]
pub enum AsymmetricAlgorithm {
    RSA,
    ECC,
    Ed25519,
    X25519,
}

#[derive(Debug, Clone)]
pub struct GeneratedKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub algorithm: AsymmetricAlgorithm,
}

#[derive(Debug, Clone)]
pub enum KeyGeneratorError {
    InvalidParameters,
    GenerationFailed,
    UnsupportedAlgorithm,
}
''')
    
    # Fix RSA types
    add_missing_types_to_file('src/stdlib/packages/crypto_asymmetric/rsa.rs', '''
// RSA specific types
#[derive(Debug, Clone)]
pub struct RsaEngine {
    pub key_size: usize,
}

#[derive(Debug, Clone)]
pub struct CursedRsaKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub key_size: usize,
}

#[derive(Debug, Clone)]
pub enum RsaError {
    InvalidKey,
    SignatureFailed,
    VerificationFailed,
    InvalidPadding,
}

#[derive(Debug, Clone)]
pub enum RsaPadding {
    PKCS1v15,
    PSS,
    OAEP,
}

#[derive(Debug, Clone)]
pub enum KeyFormat {
    PEM,
    DER,
    JWK,
}
''')
    
    # Fix ECC types
    add_missing_types_to_file('src/stdlib/packages/crypto_asymmetric/ecc.rs', '''
// ECC specific types
#[derive(Debug, Clone)]
pub struct EccEngine {
    pub curve: EccCurve,
}

#[derive(Debug, Clone)]
pub struct EccKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub curve: EccCurve,
}

#[derive(Debug, Clone)]
pub enum EccError {
    InvalidKey,
    InvalidCurve,
    SignatureFailed,
    VerificationFailed,
}

#[derive(Debug, Clone)]
pub enum EccCurve {
    P256,
    P384,
    P521,
    Secp256k1,
}

#[derive(Debug, Clone)]
pub enum EccKeyFormat {
    SEC1,
    PKCS8,
    SubjectPublicKeyInfo,
}

#[derive(Debug, Clone)]
pub enum EccHashAlgorithm {
    SHA256,
    SHA384,
    SHA512,
}
''')
    
    # Fix Ed25519 types
    add_missing_types_to_file('src/stdlib/packages/crypto_asymmetric/ed25519.rs', '''
// Ed25519 specific types
#[derive(Debug, Clone)]
pub struct Ed25519Engine {
    pub context: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Ed25519KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Ed25519Error {
    InvalidKey,
    InvalidSignature,
    SignatureFailed,
    VerificationFailed,
}

#[derive(Debug, Clone)]
pub enum Ed25519KeyFormat {
    Raw,
    PKCS8,
    SubjectPublicKeyInfo,
}
''')
    
    # Fix X25519 types
    add_missing_types_to_file('src/stdlib/packages/crypto_asymmetric/x25519.rs', '''
// X25519 specific types
#[derive(Debug, Clone)]
pub struct X25519Engine;

#[derive(Debug, Clone)]
pub struct X25519KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum X25519Error {
    InvalidKey,
    KeyExchangeFailed,
}

#[derive(Debug, Clone)]
pub struct X25519SharedSecret {
    pub secret: Vec<u8>,
}
''')
    
    # Fix elliptic curve types
    add_missing_types_to_file('src/stdlib/packages/crypto_asymmetric/elliptic_curve.rs', '''
// Elliptic Curve specific types
#[derive(Debug, Clone)]
pub struct EllipticCurveEngine {
    pub curve_type: CurveType,
}

#[derive(Debug, Clone)]
pub enum CurveType {
    Prime,
    Binary,
    Edwards,
    Montgomery,
}

#[derive(Debug, Clone)]
pub struct CurveParameters {
    pub field_size: u32,
    pub order: Vec<u8>,
    pub generator: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum CurveError {
    InvalidParameters,
    UnsupportedCurve,
    PointNotOnCurve,
}
''')
    
    # Fix public key types
    add_missing_types_to_file('src/stdlib/packages/crypto_asymmetric/public_key.rs', '''
// Public Key specific types
#[derive(Debug, Clone)]
pub struct PublicKeyEngine {
    pub key_type: PublicKeyType,
}

#[derive(Debug, Clone)]
pub enum PublicKeyType {
    RSA,
    ECC,
    Ed25519,
    X25519,
}

#[derive(Debug, Clone)]
pub struct PublicKeyInfo {
    pub algorithm: String,
    pub key_size: usize,
    pub key_data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum PublicKeyError {
    InvalidFormat,
    UnsupportedAlgorithm,
    DecodingFailed,
}
''')
    
    # Fix private key types
    add_missing_types_to_file('src/stdlib/packages/crypto_asymmetric/private_key.rs', '''
// Private Key specific types
#[derive(Debug, Clone)]
pub struct PrivateKeyEngine {
    pub key_type: PrivateKeyType,
}

#[derive(Debug, Clone)]
pub enum PrivateKeyType {
    RSA,
    ECC,
    Ed25519,
    X25519,
}

#[derive(Debug, Clone)]
pub struct PrivateKeyInfo {
    pub algorithm: String,
    pub key_size: usize,
    pub key_data: Vec<u8>,
    pub is_encrypted: bool,
}

#[derive(Debug, Clone)]
pub enum PrivateKeyError {
    InvalidFormat,
    DecryptionFailed,
    UnsupportedAlgorithm,
}
''')

    print("All crypto asymmetric module fixes applied!")

if __name__ == "__main__":
    fix_crypto_asymmetric()
