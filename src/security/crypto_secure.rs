//! Production-grade cryptographic implementations for CURSED
//! Uses industry-standard libraries with side-channel protection

use crate::error::CursedError;
use ring::{
    aead::{self, AES_256_GCM, NONCE_LEN},
    digest::{self, SHA256, SHA512},
    hkdf,
    hmac,
    pbkdf2,
    rand::{SecureRandom, SystemRandom},
    signature::{self, Ed25519KeyPair, KeyPair, VerificationAlgorithm},
};
use std::num::NonZeroU32;

pub type CryptoResult<T> = Result<T, CursedError>;

/// Secure random number generator
pub struct SecureRng {
    rng: SystemRandom,
}

impl SecureRng {
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }

    /// Generate cryptographically secure random bytes
    pub fn random_bytes(&self, size: usize) -> CryptoResult<Vec<u8>> {
        let mut bytes = vec![0u8; size];
        self.rng.fill(&mut bytes)
            .map_err(|_| CursedError::runtime_error("Failed to generate random bytes"))?;
        Ok(bytes)
    }

    /// Generate a secure nonce for AEAD
    pub fn generate_nonce(&self) -> CryptoResult<[u8; NONCE_LEN]> {
        let mut nonce = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce)
            .map_err(|_| CursedError::runtime_error("Failed to generate nonce"))?;
        Ok(nonce)
    }
}

/// Secure hash functions with proper error handling
pub struct SecureHash;

impl SecureHash {
    /// SHA-256 hash with timing attack protection
    pub fn sha256(data: &[u8]) -> Vec<u8> {
        digest::digest(&SHA256, data).as_ref().to_vec()
    }

    /// SHA-512 hash
    pub fn sha512(data: &[u8]) -> Vec<u8> {
        digest::digest(&SHA512, data).as_ref().to_vec()
    }

    /// HMAC-SHA256 with key validation
    pub fn hmac_sha256(key: &[u8], data: &[u8]) -> CryptoResult<Vec<u8>> {
        if key.len() < 32 {
            return Err(CursedError::runtime_error("HMAC key must be at least 32 bytes"));
        }

        let key = hmac::Key::new(hmac::HMAC_SHA256, key);
        let tag = hmac::sign(&key, data);
        Ok(tag.as_ref().to_vec())
    }
}

/// Key derivation with proper parameters
pub struct KeyDerivation;

impl KeyDerivation {
    /// PBKDF2 with secure iteration count
    pub fn pbkdf2_derive(
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        output_len: usize,
    ) -> CryptoResult<Vec<u8>> {
        if salt.len() < 16 {
            return Err(CursedError::runtime_error("Salt must be at least 16 bytes"));
        }
        
        if iterations < 100000 {
            return Err(CursedError::runtime_error("Iteration count too low (min 100,000)"));
        }

        let mut output = vec![0u8; output_len];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(iterations).unwrap(),
            salt,
            password,
            &mut output,
        );
        Ok(output)
    }

    /// HKDF for key expansion
    pub fn hkdf_expand(
        prk: &[u8],
        info: &[u8],
        output_len: usize,
    ) -> CryptoResult<Vec<u8>> {
        let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, &[]);
        let prk = salt.extract(prk);
        
        let mut output = vec![0u8; output_len];
        prk.expand(info, hkdf::HKDF_SHA256)
            .map_err(|_| CursedError::runtime_error("HKDF expand failed"))?
            .fill(&mut output)
            .map_err(|_| CursedError::runtime_error("HKDF fill failed"))?;
        
        Ok(output)
    }
}

/// Authenticated encryption with AES-256-GCM
pub struct AuthenticatedEncryption {
    rng: SecureRng,
}

impl AuthenticatedEncryption {
    pub fn new() -> Self {
        Self {
            rng: SecureRng::new(),
        }
    }

    /// Encrypt with authenticated encryption (AES-256-GCM)
    pub fn encrypt(&self, key: &[u8], plaintext: &[u8], associated_data: &[u8]) -> CryptoResult<EncryptedData> {
        if key.len() != 32 {
            return Err(CursedError::runtime_error("Key must be exactly 32 bytes"));
        }

        let unbound_key = aead::UnboundKey::new(&AES_256_GCM, key)
            .map_err(|_| CursedError::runtime_error("Invalid encryption key"))?;
        
        let nonce = self.rng.generate_nonce()?;
        let nonce_seq = aead::Nonce::assume_unique_for_key(nonce);
        
        let mut in_out = plaintext.to_vec();
        let sealing_key = aead::LessSafeKey::new(unbound_key);
        
        let tag = sealing_key
            .seal_in_place_append_tag(nonce_seq, aead::Aad::from(associated_data), &mut in_out)
            .map_err(|_| CursedError::runtime_error("Encryption failed"))?;

        Ok(EncryptedData {
            nonce,
            ciphertext: in_out,
            associated_data: associated_data.to_vec(),
        })
    }

    /// Decrypt and verify authenticity
    pub fn decrypt(&self, key: &[u8], encrypted: &EncryptedData) -> CryptoResult<Vec<u8>> {
        if key.len() != 32 {
            return Err(CursedError::runtime_error("Key must be exactly 32 bytes"));
        }

        let unbound_key = aead::UnboundKey::new(&AES_256_GCM, key)
            .map_err(|_| CursedError::runtime_error("Invalid decryption key"))?;
        
        let nonce_seq = aead::Nonce::assume_unique_for_key(encrypted.nonce);
        let opening_key = aead::LessSafeKey::new(unbound_key);
        
        let mut in_out = encrypted.ciphertext.clone();
        let plaintext = opening_key
            .open_in_place(nonce_seq, aead::Aad::from(&encrypted.associated_data), &mut in_out)
            .map_err(|_| CursedError::runtime_error("Decryption/authentication failed"))?;

        Ok(plaintext.to_vec())
    }
}

/// Encrypted data container
#[derive(Clone)]
pub struct EncryptedData {
    pub nonce: [u8; NONCE_LEN],
    pub ciphertext: Vec<u8>,
    pub associated_data: Vec<u8>,
}

/// Digital signatures with Ed25519
pub struct DigitalSignature;

impl DigitalSignature {
    /// Generate Ed25519 key pair
    pub fn generate_keypair() -> CryptoResult<Ed25519KeyPair> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|_| CursedError::runtime_error("Key generation failed"))?;
        
        Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|_| CursedError::runtime_error("Key pair creation failed"))
    }

    /// Sign data with Ed25519
    pub fn sign(keypair: &Ed25519KeyPair, data: &[u8]) -> Vec<u8> {
        keypair.sign(data).as_ref().to_vec()
    }

    /// Verify Ed25519 signature
    pub fn verify(public_key: &[u8], signature: &[u8], data: &[u8]) -> CryptoResult<()> {
        let public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key);
        
        public_key.verify(data, signature)
            .map_err(|_| CursedError::runtime_error("Signature verification failed"))
    }
}

/// Secure key management
pub struct KeyManager {
    rng: SecureRng,
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            rng: SecureRng::new(),
        }
    }

    /// Generate AES-256 key
    pub fn generate_aes_key(&self) -> CryptoResult<[u8; 32]> {
        let mut key = [0u8; 32];
        self.rng.rng.fill(&mut key)
            .map_err(|_| CryptoError::runtime_error("Key generation failed"))?;
        Ok(key)
    }

    /// Securely compare keys (constant-time)
    pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        use ring::constant_time;
        constant_time::verify_slices_are_equal(a, b).is_ok()
    }

    /// Secure key erasure
    pub fn secure_erase(key: &mut [u8]) {
        for byte in key.iter_mut() {
            unsafe {
                std::ptr::write_volatile(byte, 0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_encryption_round_trip() {
        let ae = AuthenticatedEncryption::new();
        let key = [0u8; 32]; // In practice, use KeyManager::generate_aes_key()
        let plaintext = b"Hello, secure world!";
        let aad = b"associated_data";

        let encrypted = ae.encrypt(&key, plaintext, aad).unwrap();
        let decrypted = ae.decrypt(&key, &encrypted).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_signature_verification() {
        let keypair = DigitalSignature::generate_keypair().unwrap();
        let data = b"Sign this message";
        
        let signature = DigitalSignature::sign(&keypair, data);
        let public_key = keypair.public_key().as_ref();
        
        DigitalSignature::verify(public_key, &signature, data).unwrap();
    }

    #[test]
    fn test_key_derivation() {
        let password = b"secure_password";
        let salt = [0u8; 16];
        let iterations = 100000;
        
        let derived_key = KeyDerivation::pbkdf2_derive(password, &salt, iterations, 32).unwrap();
        assert_eq!(derived_key.len(), 32);
    }
}
