//! The cryptz package provides cryptographic functions for the CURSED language.
//!
//! This module is equivalent to the crypto package in Go, providing functions
//! for secure hashing, message authentication, and other cryptographic operations.
//!
//! # Features
//!
//! - Hash functions (MD5, SHA-1, SHA-256)
//! - Message authentication codes (HMAC)
//! - Secure random bytes generation
//!
//! # Security Note
//!
//! This module uses standard Rust cryptographic libraries to ensure proper security.
//! Note that MD5 and SHA-1 are provided for compatibility but are considered
//! cryptographically weak for security-sensitive applications.
//!
//! # Examples
//!
//! ```cursed
//! import "cryptz"
//!
//! // Create a hash
//! passwordHash := cryptz.sha256sum("hunter2")
//!
//! // Create an HMAC
//! authCode := cryptz.hmac("message", "secretKey", "sha256")
//!
//! // Generate random bytes (16 bytes)
//! randomData := cryptz.random_bytes(16)
//! ```

use crate::error::Error;
use crate::object::Object;
use std::rc::Rc;

// Import crypto libraries
use md5;
use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha256, Digest as Sha2Digest};
use hmac::{Hmac, Mac};
use rand::{RngCore, rngs::OsRng};

/// Creates an MD5 hash of the input data.
///
/// # Arguments
///
/// * `args[0]` - The data to hash as a String Object
///
/// # Returns
///
/// A String Object containing the hexadecimal representation of the MD5 hash
///
/// # Errors
///
/// Returns a Runtime error if no argument is provided or if the argument is not a string
pub fn md5sum(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("md5sum requires 1 argument".to_string()));
    }

    // Get the string to hash
    let input = match &*args[0] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("Argument to md5sum must be a string".to_string())),
    };

    // Create MD5 hasher and compute hash
    let result = md5::compute(input.as_bytes()).0;

    // Convert to hex string
    let hex_string = hex::encode(result);
    Ok(Rc::new(Object::String(hex_string)))
}

/// Creates a SHA-1 hash of the input data.
///
/// # Arguments
///
/// * `args[0]` - The data to hash as a String Object
///
/// # Returns
///
/// A String Object containing the hexadecimal representation of the SHA-1 hash
///
/// # Errors
///
/// Returns a Runtime error if no argument is provided or if the argument is not a string
pub fn sha1sum(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("sha1sum requires 1 argument".to_string()));
    }

    // Get the string to hash
    let input = match &*args[0] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("Argument to sha1sum must be a string".to_string())),
    };

    // Create SHA-1 hasher and compute hash
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    // Convert to hex string
    let hex_string = hex::encode(result);
    Ok(Rc::new(Object::String(hex_string)))
}

/// Creates a SHA-256 hash of the input data.
///
/// # Arguments
///
/// * `args[0]` - The data to hash as a String Object
///
/// # Returns
///
/// A String Object containing the hexadecimal representation of the SHA-256 hash
///
/// # Errors
///
/// Returns a Runtime error if no argument is provided or if the argument is not a string
pub fn sha256sum(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("sha256sum requires 1 argument".to_string()));
    }

    // Get the string to hash
    let input = match &*args[0] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("Argument to sha256sum must be a string".to_string())),
    };

    // Create SHA-256 hasher and compute hash
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    // Convert to hex string
    let hex_string = hex::encode(result);
    Ok(Rc::new(Object::String(hex_string)))
}

/// Creates an HMAC (Hash-based Message Authentication Code) for the provided data and key.
///
/// # Arguments
///
/// * `args[0]` - The data to authenticate as a String Object
/// * `args[1]` - The secret key as a String Object
/// * `args[2]` - The hash algorithm to use ("md5", "sha1", or "sha256") as a String Object
///
/// # Returns
///
/// A String Object containing the hexadecimal representation of the HMAC
///
/// # Errors
///
/// Returns a Runtime error if fewer than 3 arguments are provided or if
/// arguments are invalid types or if an unsupported algorithm is specified
pub fn hmac(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime(
            "hmac requires 3 arguments: data, key, and algorithm".to_string(),
        ));
    }

    // Extract data, key, and algorithm from arguments
    let data = match &*args[0] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("First argument to hmac must be a string".to_string())),
    };

    let key = match &*args[1] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("Second argument to hmac must be a string".to_string())),
    };

    let algorithm = match &*args[2] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("Third argument to hmac must be a string".to_string())),
    };

    // Compute HMAC based on selected algorithm
    let hex_string = match algorithm.as_str() {
        "md5" => {
            // Since the HMAC crate doesn't work directly with md5 crate v0.7,
            // we'll implement HMAC-MD5 manually
            let key_bytes = key.as_bytes();
            let data_bytes = data.as_bytes();
            
            // Simplified HMAC-MD5 implementation
            let mut key_padded = vec![0u8; 64]; // Block size for MD5 is 64 bytes
            if key_bytes.len() <= 64 {
                key_padded[..key_bytes.len()].copy_from_slice(key_bytes);
            } else {
                let key_hash = md5::compute(key_bytes).0;
                key_padded[..16].copy_from_slice(&key_hash);
            }
            
            // Create inner padded key
            let mut inner_key = vec![0u8; 64];
            for i in 0..64 {
                inner_key[i] = key_padded[i] ^ 0x36; // XOR with 0x36 (ipad)
            }
            
            // Create outer padded key
            let mut outer_key = vec![0u8; 64];
            for i in 0..64 {
                outer_key[i] = key_padded[i] ^ 0x5c; // XOR with 0x5c (opad)
            }
            
            // Inner hash
            let mut inner_data = inner_key.clone();
            inner_data.extend_from_slice(data_bytes);
            let inner_hash = md5::compute(&inner_data).0;
            
            // Outer hash
            let mut outer_data = outer_key.clone();
            outer_data.extend_from_slice(&inner_hash);
            let result = md5::compute(&outer_data).0;
            
            hex::encode(result)
        },
        "sha1" => {
            // Create HMAC with SHA-1
            let mut mac = Hmac::<Sha1>::new_from_slice(key.as_bytes())
                .map_err(|_| Error::Runtime("Failed to create HMAC-SHA1".to_string()))?;
            mac.update(data.as_bytes());
            let result = mac.finalize().into_bytes();
            hex::encode(result)
        },
        "sha256" => {
            // Create HMAC with SHA-256
            let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())
                .map_err(|_| Error::Runtime("Failed to create HMAC-SHA256".to_string()))?;
            mac.update(data.as_bytes());
            let result = mac.finalize().into_bytes();
            hex::encode(result)
        },
        _ => return Err(Error::Runtime(format!(
            "Unsupported hash algorithm for HMAC: {}", algorithm
        ))),
    };

    Ok(Rc::new(Object::String(hex_string)))
}

/// Generates cryptographically secure random bytes.
///
/// # Arguments
///
/// * `args[0]` - The number of bytes to generate as an Integer Object
///
/// # Returns
///
/// An Array Object containing the random bytes as Integer Objects (0-255)
///
/// # Errors
///
/// Returns a Runtime error if no argument is provided, if the argument is not an integer,
/// or if the requested length is negative or too large
pub fn random_bytes(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("random_bytes requires 1 argument".to_string()));
    }

    // Get the number of bytes to generate
    let length = match &*args[0] {
        Object::Integer(n) => {
            if *n < 0 {
                return Err(Error::Runtime("Length cannot be negative".to_string()));
            }
            if *n > 1_000_000 {
                return Err(Error::Runtime("Requested length is too large".to_string()));
            }
            *n as usize
        },
        _ => return Err(Error::Runtime("Argument to random_bytes must be an integer".to_string())),
    };

    // Generate random bytes
    let mut bytes = vec![0u8; length];
    OsRng.fill_bytes(&mut bytes);

    // Convert to array of integer objects
    let byte_objects = bytes.into_iter()
        .map(|b| Object::Integer(b as i64))
        .collect();

    Ok(Rc::new(Object::Array(byte_objects)))
}
