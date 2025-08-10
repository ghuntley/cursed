//! WASM-compatible cryptography
//! 
//! Provides cryptographic operations that work in WebAssembly
//! using RustCrypto crates which are WASM-compatible.

use std::io::{Error, ErrorKind, Result};

/// WASM-compatible crypto operations using RustCrypto
pub struct WasmCrypto;

impl WasmCrypto {
    pub fn new() -> Self {
        Self
    }

    /// SHA-256 hashing
    #[cfg(feature = "crypto-rustcrypto")]
    pub fn sha256(&self, data: &[u8]) -> Result<Vec<u8>> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }

    #[cfg(not(feature = "crypto-rustcrypto"))]
    pub fn sha256(&self, _data: &[u8]) -> Result<Vec<u8>> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "SHA-256 not available - enable crypto-rustcrypto feature"
        ))
    }

    /// BLAKE3 hashing
    #[cfg(feature = "crypto-rustcrypto")]
    pub fn blake3(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(blake3::hash(data).as_bytes().to_vec())
    }

    #[cfg(not(feature = "crypto-rustcrypto"))]
    pub fn blake3(&self, _data: &[u8]) -> Result<Vec<u8>> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "BLAKE3 not available - enable crypto-rustcrypto feature"
        ))
    }

    /// AES-GCM encryption
    #[cfg(feature = "crypto-rustcrypto")]
    pub fn aes_gcm_encrypt(&self, key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
        
        if key.len() != 32 {
            return Err(Error::new(ErrorKind::InvalidInput, "Key must be 32 bytes"));
        }
        if nonce.len() != 12 {
            return Err(Error::new(ErrorKind::InvalidInput, "Nonce must be 12 bytes"));
        }

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Invalid key: {}", e)))?;
        
        let nonce = Nonce::from_slice(nonce);
        
        cipher.encrypt(nonce, plaintext)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Encryption failed: {}", e)))
    }

    #[cfg(not(feature = "crypto-rustcrypto"))]
    pub fn aes_gcm_encrypt(&self, _key: &[u8], _nonce: &[u8], _plaintext: &[u8]) -> Result<Vec<u8>> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "AES-GCM not available - enable crypto-rustcrypto feature"
        ))
    }

    /// AES-GCM decryption
    #[cfg(feature = "crypto-rustcrypto")]
    pub fn aes_gcm_decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
        
        if key.len() != 32 {
            return Err(Error::new(ErrorKind::InvalidInput, "Key must be 32 bytes"));
        }
        if nonce.len() != 12 {
            return Err(Error::new(ErrorKind::InvalidInput, "Nonce must be 12 bytes"));
        }

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Invalid key: {}", e)))?;
        
        let nonce = Nonce::from_slice(nonce);
        
        cipher.decrypt(nonce, ciphertext)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Decryption failed: {}", e)))
    }

    #[cfg(not(feature = "crypto-rustcrypto"))]
    pub fn aes_gcm_decrypt(&self, _key: &[u8], _nonce: &[u8], _ciphertext: &[u8]) -> Result<Vec<u8>> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "AES-GCM not available - enable crypto-rustcrypto feature"
        ))
    }

    /// HMAC-SHA256
    #[cfg(feature = "crypto-rustcrypto")]
    pub fn hmac_sha256(&self, key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Invalid key: {}", e)))?;
        
        mac.update(data);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    #[cfg(not(feature = "crypto-rustcrypto"))]
    pub fn hmac_sha256(&self, _key: &[u8], _data: &[u8]) -> Result<Vec<u8>> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "HMAC-SHA256 not available - enable crypto-rustcrypto feature"
        ))
    }

    /// Ed25519 key generation
    #[cfg(feature = "crypto-rustcrypto")]
    pub fn ed25519_keygen(&self) -> Result<(Vec<u8>, Vec<u8>)> {
        use ed25519_dalek::{SigningKey, VerifyingKey};
        use rand::rngs::OsRng;
        
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key: VerifyingKey = signing_key.verifying_key();
        
        Ok((signing_key.to_bytes().to_vec(), verifying_key.to_bytes().to_vec()))
    }

    #[cfg(not(feature = "crypto-rustcrypto"))]
    pub fn ed25519_keygen(&self) -> Result<(Vec<u8>, Vec<u8>)> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Ed25519 not available - enable crypto-rustcrypto feature"
        ))
    }

    /// Password hashing with Argon2
    #[cfg(feature = "crypto-rustcrypto")]
    pub fn argon2_hash(&self, password: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
        use argon2::{Argon2, PasswordHasher};
        use argon2::password_hash::{SaltString, PasswordHash};
        
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Invalid salt: {}", e)))?;
        
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt_string)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Hashing failed: {}", e)))?;
        
        Ok(password_hash.to_string().into_bytes())
    }

    #[cfg(not(feature = "crypto-rustcrypto"))]
    pub fn argon2_hash(&self, _password: &[u8], _salt: &[u8]) -> Result<Vec<u8>> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Argon2 not available - enable crypto-rustcrypto feature"
        ))
    }
}

/// Generate cryptographically secure random bytes
#[cfg(target_arch = "wasm32")]
pub fn random_bytes(len: usize) -> Result<Vec<u8>> {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; len];
    rng.fill_bytes(&mut bytes);
    Ok(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_bytes(len: usize) -> Result<Vec<u8>> {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; len];
    rng.fill_bytes(&mut bytes);
    Ok(bytes)
}
