// XChaCha20-Poly1305 AEAD (Authenticated Encryption with Associated Data) Implementation
// 
// This module provides a complete XChaCha20-Poly1305 implementation with both streaming
// and one-shot APIs for authenticated encryption and decryption operations.
// 
// XChaCha20-Poly1305 is a variant of ChaCha20-Poly1305 that uses a 192-bit nonce
// (compared to 96-bit for standard ChaCha20-Poly1305), providing better security
// properties for long-running sessions and high-volume applications.

use chacha20poly1305::{
    XChaCha20Poly1305, KeyInit, AeadCore, AeadInPlace,
    aead::{Aead, OsRng, Payload},
    XNonce, Key
};
use crate::error::Error;
use crate::error::CursedError;
use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};
use rand::{RngCore, CryptoRng};

/// XChaCha20-Poly1305 key size in bytes (256 bits)
pub const XCHACHA20_KEY_SIZE: usize = 32;

/// XChaCha20-Poly1305 nonce size in bytes (192 bits)
pub const XCHACHA20_NONCE_SIZE: usize = 24;

/// XChaCha20-Poly1305 authentication tag size in bytes (128 bits)
pub const XCHACHA20_TAG_SIZE: usize = 16;

/// Maximum plaintext size for XChaCha20-Poly1305 (2^38 - 64 bytes)
pub const XCHACHA20_MAX_PLAINTEXT_SIZE: u64 = (1u64 << 38) - 64;

/// XChaCha20-Poly1305 cryptographic key
#[derive(Clone, ZeroizeOnDrop)]
pub struct XChaCha20Key {
    key: [u8; XCHACHA20_KEY_SIZE],
}

impl XChaCha20Key {
    /// Generate a new random key using a cryptographically secure RNG
    pub fn generate() -> Result<(), Error> {
        Self::generate_with_rng(&mut OsRng)
    }

    /// Generate a new random key using the provided RNG
    pub fn generate_with_rng<R: RngCore + CryptoRng>(rng: &mut R) -> Result<(), Error> {
        let mut key = [0u8; XCHACHA20_KEY_SIZE];
        rng.fill_bytes(&mut key);
        Ok(Self { key })
    }

    /// Create a key from existing bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<(), Error> {
        if bytes.len() != XCHACHA20_KEY_SIZE {
            return Err(CursedError::Crypto(format!(
                "Invalid key size: expected {}, got {}", 
                XCHACHA20_KEY_SIZE, 
                bytes.len()
            )));
        }
        let mut key = [0u8; XCHACHA20_KEY_SIZE];
        key.copy_from_slice(bytes);
        Ok(Self { key })
    }

    /// Get the key as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }

    /// Convert to the underlying chacha20poly1305 Key type
    fn to_chacha_key(&self) -> Key {
        Key::from_slice(&self.key).clone()
    }
}

impl fmt::Debug for XChaCha20Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XChaCha20Key")
            .field("key", &"[REDACTED]")
            .finish()
    }
}

/// XChaCha20-Poly1305 nonce (192-bit)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XChaCha20Nonce {
    nonce: [u8; XCHACHA20_NONCE_SIZE],
}

impl XChaCha20Nonce {
    /// Generate a new random nonce using a cryptographically secure RNG
    pub fn generate() -> Result<(), Error> {
        Self::generate_with_rng(&mut OsRng)
    }

    /// Generate a new random nonce using the provided RNG
    pub fn generate_with_rng<R: RngCore + CryptoRng>(rng: &mut R) -> Result<(), Error> {
        let mut nonce = [0u8; XCHACHA20_NONCE_SIZE];
        rng.fill_bytes(&mut nonce);
        Ok(Self { nonce })
    }

    /// Create a nonce from existing bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<(), Error> {
        if bytes.len() != XCHACHA20_NONCE_SIZE {
            return Err(CursedError::Crypto(format!(
                "Invalid nonce size: expected {}, got {}", 
                XCHACHA20_NONCE_SIZE, 
                bytes.len()
            )));
        }
        let mut nonce = [0u8; XCHACHA20_NONCE_SIZE];
        nonce.copy_from_slice(bytes);
        Ok(Self { nonce })
    }

    /// Get the nonce as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.nonce
    }

    /// Convert to the underlying chacha20poly1305 XNonce type
    fn to_chacha_nonce(&self) -> &XNonce {
        XNonce::from_slice(&self.nonce)
    }
}

/// XChaCha20-Poly1305 cipher instance
pub struct XChaCha20Poly1305Cipher {
    cipher: XChaCha20Poly1305,
}

impl XChaCha20Poly1305Cipher {
    /// Create a new cipher instance with the given key
    pub fn new(key: &XChaCha20Key) -> Self {
        let cipher = XChaCha20Poly1305::new(&key.to_chacha_key());
        Self { cipher }
    }

    /// Encrypt plaintext with associated data (one-shot API)
    pub fn encrypt(
        &self,
        nonce: &XChaCha20Nonce,
        plaintext: &[u8],
        associated_data: &[u8],
    ) -> Result<(), Error> {
        if plaintext.len() as u64 > XCHACHA20_MAX_PLAINTEXT_SIZE {
            return Err(CursedError::Crypto(format!(
                "Plaintext too large: {} bytes exceeds maximum of {} bytes",
                plaintext.len(),
                XCHACHA20_MAX_PLAINTEXT_SIZE
            )));
        }

        let payload = Payload {
            msg: plaintext,
            aad: associated_data,
        };

        self.cipher
            .encrypt(nonce.to_chacha_nonce(), payload)
            .map_err(|e| CursedError::Crypto(format!("Encryption failed: {}", e)))
    }

    /// Decrypt ciphertext with associated data (one-shot API)
    pub fn decrypt(
        &self,
        nonce: &XChaCha20Nonce,
        ciphertext: &[u8],
        associated_data: &[u8],
    ) -> Result<(), Error> {
        let payload = Payload {
            msg: ciphertext,
            aad: associated_data,
        };

        self.cipher
            .decrypt(nonce.to_chacha_nonce(), payload)
            .map_err(|e| CursedError::Crypto(format!("Decryption failed: {}", e)))
    }

    /// Encrypt plaintext in-place with associated data
    pub fn encrypt_in_place(
        &self,
        nonce: &XChaCha20Nonce,
        associated_data: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        if buffer.len() as u64 > XCHACHA20_MAX_PLAINTEXT_SIZE {
            return Err(CursedError::Crypto(format!(
                "Buffer too large: {} bytes exceeds maximum of {} bytes",
                buffer.len(),
                XCHACHA20_MAX_PLAINTEXT_SIZE
            )));
        }

        self.cipher
            .encrypt_in_place(nonce.to_chacha_nonce(), associated_data, buffer)
            .map_err(|e| CursedError::Crypto(format!("In-place encryption failed: {}", e)))
    }

    /// Decrypt ciphertext in-place with associated data
    pub fn decrypt_in_place(
        &self,
        nonce: &XChaCha20Nonce,
        associated_data: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        self.cipher
            .decrypt_in_place(nonce.to_chacha_nonce(), associated_data, buffer)
            .map_err(|e| CursedError::Crypto(format!("In-place decryption failed: {}", e)))
    }
}

/// Streaming encryption context for large data
pub struct XChaCha20Poly1305StreamingEncoder {
    cipher: XChaCha20Poly1305,
    nonce: XChaCha20Nonce,
    processed_bytes: u64,
}

impl XChaCha20Poly1305StreamingEncoder {
    /// Create a new streaming encoder
    pub fn new(key: &XChaCha20Key, nonce: XChaCha20Nonce) -> Self {
        let cipher = XChaCha20Poly1305::new(&key.to_chacha_key());
        Self {
            cipher,
            nonce,
            processed_bytes: 0,
        }
    }

    /// Process a chunk of data
    pub fn process_chunk(
        &mut self,
        chunk: &[u8],
        associated_data: &[u8],
    ) -> Result<(), Error> {
        if self.processed_bytes + chunk.len() as u64 > XCHACHA20_MAX_PLAINTEXT_SIZE {
            return Err(CursedError::Crypto(format!(
                "Total data size would exceed maximum: {} + {} > {}",
                self.processed_bytes,
                chunk.len(),
                XCHACHA20_MAX_PLAINTEXT_SIZE
            )));
        }

        let payload = Payload {
            msg: chunk,
            aad: associated_data,
        };

        let result = self.cipher
            .encrypt(self.nonce.to_chacha_nonce(), payload)
            .map_err(|e| CursedError::Crypto(format!("Streaming encryption failed: {}", e)))?;

        self.processed_bytes += chunk.len() as u64;
        Ok(result)
    }

    /// Get the current nonce
    pub fn nonce(&self) -> &XChaCha20Nonce {
        &self.nonce
    }

    /// Get the number of bytes processed
    pub fn processed_bytes(&self) -> u64 {
        self.processed_bytes
    }
}

/// Streaming decryption context for large data
pub struct XChaCha20Poly1305StreamingDecoder {
    cipher: XChaCha20Poly1305,
    nonce: XChaCha20Nonce,
    processed_bytes: u64,
}

impl XChaCha20Poly1305StreamingDecoder {
    /// Create a new streaming decoder
    pub fn new(key: &XChaCha20Key, nonce: XChaCha20Nonce) -> Self {
        let cipher = XChaCha20Poly1305::new(&key.to_chacha_key());
        Self {
            cipher,
            nonce,
            processed_bytes: 0,
        }
    }

    /// Process a chunk of encrypted data
    pub fn process_chunk(
        &mut self,
        chunk: &[u8],
        associated_data: &[u8],
    ) -> Result<(), Error> {
        let payload = Payload {
            msg: chunk,
            aad: associated_data,
        };

        let result = self.cipher
            .decrypt(self.nonce.to_chacha_nonce(), payload)
            .map_err(|e| CursedError::Crypto(format!("Streaming decryption failed: {}", e)))?;

        self.processed_bytes += result.len() as u64;
        Ok(result)
    }

    /// Get the current nonce
    pub fn nonce(&self) -> &XChaCha20Nonce {
        &self.nonce
    }

    /// Get the number of bytes processed
    pub fn processed_bytes(&self) -> u64 {
        self.processed_bytes
    }
}

/// High-level API for XChaCha20-Poly1305 operations
pub struct XChaCha20Poly1305Api;

impl XChaCha20Poly1305Api {
    /// Generate a new random key
    pub fn generate_key() -> Result<(), Error> {
        XChaCha20Key::generate()
    }

    /// Generate a new random nonce
    pub fn generate_nonce() -> Result<(), Error> {
        XChaCha20Nonce::generate()
    }

    /// Encrypt data with XChaCha20-Poly1305 (convenience function)
    pub fn encrypt(
        key: &XChaCha20Key,
        plaintext: &[u8],
        associated_data: Option<&[u8]>,
    ) -> Result<(), Error> {
        let nonce = Self::generate_nonce()?;
        let cipher = XChaCha20Poly1305Cipher::new(key);
        let ciphertext = cipher.encrypt(&nonce, plaintext, associated_data.unwrap_or(&[]))?;
        Ok((nonce, ciphertext))
    }

    /// Decrypt data with XChaCha20-Poly1305 (convenience function)
    pub fn decrypt(
        key: &XChaCha20Key,
        nonce: &XChaCha20Nonce,
        ciphertext: &[u8],
        associated_data: Option<&[u8]>,
    ) -> Result<(), Error> {
        let cipher = XChaCha20Poly1305Cipher::new(key);
        cipher.decrypt(nonce, ciphertext, associated_data.unwrap_or(&[]))
    }

    /// Create a streaming encoder for large data
    pub fn create_streaming_encoder(
        key: &XChaCha20Key,
    ) -> Result<(), Error> {
        let nonce = Self::generate_nonce()?;
        Ok(XChaCha20Poly1305StreamingEncoder::new(key, nonce))
    }

    /// Create a streaming decoder for large data
    pub fn create_streaming_decoder(
        key: &XChaCha20Key,
        nonce: XChaCha20Nonce,
    ) -> XChaCha20Poly1305StreamingDecoder {
        XChaCha20Poly1305StreamingDecoder::new(key, nonce)
    }
}

/// Key derivation using HKDF for XChaCha20 keys
pub mod key_derivation {
    use super::*;
    use hkdf::Hkdf;
    use sha2::Sha256;

    /// Derive an XChaCha20 key from input key material using HKDF-SHA256
    pub fn derive_key(
        input_key_material: &[u8],
        salt: Option<&[u8]>,
        info: &[u8],
    ) -> Result<(), Error> {
        let hk = Hkdf::<Sha256>::new(salt, input_key_material);
        let mut derived_key = [0u8; XCHACHA20_KEY_SIZE];
        
        hk.expand(info, &mut derived_key)
            .map_err(|e| CursedError::Crypto(format!("Key derivation failed: {}", e)))?;

        XChaCha20Key::from_bytes(&derived_key)
    }

    /// Derive multiple keys from input key material
    pub fn derive_keys(
        input_key_material: &[u8],
        salt: Option<&[u8]>,
        count: usize,
    ) -> Result<(), Error> {
        let mut keys = Vec::with_capacity(count);
        
        for i in 0..count {
            let info = format!("XChaCha20-Key-{}", i);
            let key = derive_key(input_key_material, salt, info.as_bytes())?;
            keys.push(key);
        }

        Ok(keys)
    }
}

/// Utility functions for XChaCha20-Poly1305
pub mod utils {
    use super::*;

    /// Validate that the data size is within limits for XChaCha20-Poly1305
    pub fn validate_data_size(size: usize) -> Result<(), Error> {
        if size as u64 > XCHACHA20_MAX_PLAINTEXT_SIZE {
            return Err(CursedError::Crypto(format!(
                "Data size {} exceeds maximum allowed size of {} bytes",
                size,
                XCHACHA20_MAX_PLAINTEXT_SIZE
            )));
        }
        Ok(())
    }

    /// Calculate the ciphertext size including authentication tag
    pub fn calculate_ciphertext_size(plaintext_size: usize) -> usize {
        plaintext_size + XCHACHA20_TAG_SIZE
    }

    /// Calculate the plaintext size from ciphertext size
    pub fn calculate_plaintext_size(ciphertext_size: usize) -> Result<(), Error> {
        if ciphertext_size < XCHACHA20_TAG_SIZE {
            return Err(CursedError::Crypto(format!(
                "Ciphertext too small: {} bytes, minimum {} bytes required",
                ciphertext_size,
                XCHACHA20_TAG_SIZE
            )));
        }
        Ok(ciphertext_size - XCHACHA20_TAG_SIZE)
    }

    /// Securely compare two byte slices in constant time
    pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        use subtle::ConstantTimeEq;
        a.ct_eq(b).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_key_generation() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        assert_eq!(key.as_bytes().len(), XCHACHA20_KEY_SIZE);
    }

    #[test]
    fn test_key_from_bytes() {
        let bytes = [42u8; XCHACHA20_KEY_SIZE];
        let key = XChaCha20Key::from_bytes(&bytes).expect("Key creation should succeed");
        assert_eq!(key.as_bytes(), &bytes);
    }

    #[test]
    fn test_key_from_invalid_bytes() {
        let bytes = [42u8; 16]; // Wrong size
        assert!(XChaCha20Key::from_bytes(&bytes).is_err());
    }

    #[test]
    fn test_nonce_generation() {
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        assert_eq!(nonce.as_bytes().len(), XCHACHA20_NONCE_SIZE);
    }

    #[test]
    fn test_nonce_from_bytes() {
        let bytes = [42u8; XCHACHA20_NONCE_SIZE];
        let nonce = XChaCha20Nonce::from_bytes(&bytes).expect("Nonce creation should succeed");
        assert_eq!(nonce.as_bytes(), &bytes);
    }

    #[test]
    fn test_basic_encryption_decryption() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Hello, XChaCha20-Poly1305!";
        let associated_data = b"Additional authenticated data";

        let ciphertext = cipher
            .encrypt(&nonce, plaintext, associated_data)
            .expect("Encryption should succeed");

        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, associated_data)
            .expect("Decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encryption_without_associated_data() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Hello, XChaCha20-Poly1305!";

        let ciphertext = cipher
            .encrypt(&nonce, plaintext, &[])
            .expect("Encryption should succeed");

        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, &[])
            .expect("Decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_authentication_failure() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Hello, XChaCha20-Poly1305!";
        let associated_data = b"Additional authenticated data";

        let mut ciphertext = cipher
            .encrypt(&nonce, plaintext, associated_data)
            .expect("Encryption should succeed");

        // Tamper with the ciphertext
        ciphertext[0] ^= 1;

        // Decryption should fail due to authentication error
        assert!(cipher.decrypt(&nonce, &ciphertext, associated_data).is_err());
    }

    #[test]
    fn test_wrong_associated_data() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Hello, XChaCha20-Poly1305!";
        let associated_data = b"Additional authenticated data";
        let wrong_associated_data = b"Wrong associated data";

        let ciphertext = cipher
            .encrypt(&nonce, plaintext, associated_data)
            .expect("Encryption should succeed");

        // Decryption should fail with wrong associated data
        assert!(cipher.decrypt(&nonce, &ciphertext, wrong_associated_data).is_err());
    }

    #[test]
    fn test_in_place_encryption_decryption() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"Hello, XChaCha20-Poly1305!";
        let associated_data = b"Additional authenticated data";

        // Create buffer with space for authentication tag
        let mut buffer = vec![0u8; plaintext.len() + XCHACHA20_TAG_SIZE];
        buffer[..plaintext.len()].copy_from_slice(plaintext);

        cipher
            .encrypt_in_place(&nonce, associated_data, &mut buffer)
            .expect("In-place encryption should succeed");

        cipher
            .decrypt_in_place(&nonce, associated_data, &mut buffer)
            .expect("In-place decryption should succeed");

        assert_eq!(&buffer[..plaintext.len()], plaintext);
    }

    #[test]
    fn test_streaming_encryption_decryption() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let plaintext = b"This is a long message that will be processed in chunks for streaming encryption and decryption testing.";
        let associated_data = b"streaming";

        let mut encoder = XChaCha20Poly1305StreamingEncoder::new(&key, XChaCha20Nonce::generate().unwrap());
        let nonce = encoder.nonce().clone();

        // Encrypt in chunks
        let chunk_size = 32;
        let mut encrypted_chunks = Vec::new();
        
        for chunk in plaintext.chunks(chunk_size) {
            let encrypted = encoder
                .process_chunk(chunk, associated_data)
                .expect("Streaming encryption should succeed");
            encrypted_chunks.push(encrypted);
        }

        // Decrypt in chunks
        let mut decoder = XChaCha20Poly1305StreamingDecoder::new(&key, nonce);
        let mut decrypted = Vec::new();

        for encrypted_chunk in encrypted_chunks {
            let chunk = decoder
                .process_chunk(&encrypted_chunk, associated_data)
                .expect("Streaming decryption should succeed");
            decrypted.extend_from_slice(&chunk);
        }

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_high_level_api() {
        let key = XChaCha20Poly1305Api::generate_key().expect("Key generation should succeed");
        let plaintext = b"Hello from high-level API!";
        let associated_data = Some(b"high-level".as_slice());

        let (nonce, ciphertext) = XChaCha20Poly1305Api::encrypt(&key, plaintext, associated_data)
            .expect("High-level encryption should succeed");

        let decrypted = XChaCha20Poly1305Api::decrypt(&key, &nonce, &ciphertext, associated_data)
            .expect("High-level decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_key_derivation() {
        let input_key_material = b"shared secret";
        let salt = Some(b"unique salt".as_slice());
        let info = b"XChaCha20-Poly1305 key derivation";

        let key = key_derivation::derive_key(input_key_material, salt, info)
            .expect("Key derivation should succeed");

        assert_eq!(key.as_bytes().len(), XCHACHA20_KEY_SIZE);

        // Derive the same key again to test determinism
        let key2 = key_derivation::derive_key(input_key_material, salt, info)
            .expect("Key derivation should succeed");

        assert_eq!(key.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_multiple_key_derivation() {
        let input_key_material = b"shared secret for multiple keys";
        let salt = Some(b"multi-key salt".as_slice());

        let keys = key_derivation::derive_keys(input_key_material, salt, 3)
            .expect("Multiple key derivation should succeed");

        assert_eq!(keys.len(), 3);
        
        // Ensure all keys are different
        assert_ne!(keys[0].as_bytes(), keys[1].as_bytes());
        assert_ne!(keys[1].as_bytes(), keys[2].as_bytes());
        assert_ne!(keys[0].as_bytes(), keys[2].as_bytes());
    }

    #[test]
    fn test_data_size_validation() {
        assert!(utils::validate_data_size(1000).is_ok());
        assert!(utils::validate_data_size(XCHACHA20_MAX_PLAINTEXT_SIZE as usize).is_ok());
        assert!(utils::validate_data_size((XCHACHA20_MAX_PLAINTEXT_SIZE + 1) as usize).is_err());
    }

    #[test]
    fn test_ciphertext_size_calculations() {
        let plaintext_size = 100;
        let ciphertext_size = utils::calculate_ciphertext_size(plaintext_size);
        assert_eq!(ciphertext_size, plaintext_size + XCHACHA20_TAG_SIZE);

        let recovered_plaintext_size = utils::calculate_plaintext_size(ciphertext_size)
            .expect("Plaintext size calculation should succeed");
        assert_eq!(recovered_plaintext_size, plaintext_size);
    }

    #[test]
    fn test_constant_time_comparison() {
        let a = b"hello";
        let b = b"hello";
        let c = b"world";

        assert!(utils::constant_time_eq(a, b));
        assert!(!utils::constant_time_eq(a, c));
    }

    #[test]
    fn test_large_data_encryption() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        // Test with 1MB of data
        let plaintext = vec![42u8; 1024 * 1024];
        let associated_data = b"large data test";

        let ciphertext = cipher
            .encrypt(&nonce, &plaintext, associated_data)
            .expect("Large data encryption should succeed");

        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, associated_data)
            .expect("Large data decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_empty_data_encryption() {
        let key = XChaCha20Key::generate().expect("Key generation should succeed");
        let nonce = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let cipher = XChaCha20Poly1305Cipher::new(&key);

        let plaintext = b"";
        let associated_data = b"empty data test";

        let ciphertext = cipher
            .encrypt(&nonce, plaintext, associated_data)
            .expect("Empty data encryption should succeed");

        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, associated_data)
            .expect("Empty data decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_nonce_uniqueness() {
        let nonce1 = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        let nonce2 = XChaCha20Nonce::generate().expect("Nonce generation should succeed");
        
        // Nonces should be different (extremely high probability)
        assert_ne!(nonce1.as_bytes(), nonce2.as_bytes());
    }

    #[test]
    fn test_key_zeroization() {
        let key_bytes = [42u8; XCHACHA20_KEY_SIZE];
        let mut key = XChaCha20Key::from_bytes(&key_bytes).expect("Key creation should succeed");
        
        // Manually zeroize to test
        key.key.zeroize();
        assert_ne!(key.as_bytes(), &key_bytes);
        assert_eq!(key.as_bytes(), &[0u8; XCHACHA20_KEY_SIZE]);
    }
}
