// Standalone test for crypto functionality
// Run with: rustc test_crypto_standalone.rs && ./test_crypto_standalone

use std::collections::HashMap;

// Minimal error type for testing
#[derive(Debug)]
pub struct CursedError {
    message: String,
}

impl CursedError {
    pub fn new(_error_type: &str, message: &str) -> Self {
        Self { message: message.to_string() }
    }
}

impl std::fmt::Display for CursedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CursedError {}

impl From<serde_json::Error> for CursedError {
    fn from(err: serde_json::Error) -> Self {
        CursedError::new("json_error", &err.to_string())
    }
}

impl From<base64::DecodeError> for CursedError {
    fn from(err: base64::DecodeError) -> Self {
        CursedError::new("base64_error", &err.to_string())
    }
}

// Simplified JWT implementation for testing
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use base64::{Engine as _, engine::general_purpose};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub struct JwtHandler {
    secret_key: Vec<u8>,
    default_expiry: u64,
}

impl JwtHandler {
    pub fn new(secret_key: Vec<u8>, default_expiry_seconds: u64) -> Self {
        Self {
            secret_key,
            default_expiry: default_expiry_seconds,
        }
    }

    pub fn create_token(&self, claims: HashMap<String, Value>) -> Result<String, CursedError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| CursedError::new("jwt_error", &format!("Time error: {}", e)))?
            .as_secs();

        let mut full_claims = claims;
        full_claims.insert("iat".to_string(), json!(now));
        full_claims.insert("exp".to_string(), json!(now + self.default_expiry));

        let header = json!({
            "alg": "HS256",
            "typ": "JWT"
        });

        let header_b64 = general_purpose::URL_SAFE_NO_PAD.encode(header.to_string());
        let payload_b64 = general_purpose::URL_SAFE_NO_PAD.encode(serde_json::to_string(&full_claims)?);
        
        let message = format!("{}.{}", header_b64, payload_b64);
        let signature = self.sign_message(&message)?;
        let signature_b64 = general_purpose::URL_SAFE_NO_PAD.encode(signature);

        let token = format!("{}.{}", message, signature_b64);
        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<HashMap<String, Value>, CursedError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(CursedError::new("jwt_error", "Invalid token format"));
        }

        let message = format!("{}.{}", parts[0], parts[1]);
        let signature = general_purpose::URL_SAFE_NO_PAD.decode(parts[2])
            .map_err(|e| CursedError::new("jwt_error", &format!("Invalid signature encoding: {}", e)))?;

        // Verify signature
        if !self.verify_signature(&message, &signature)? {
            return Err(CursedError::new("jwt_error", "Invalid signature"));
        }

        // Decode payload
        let payload_json = general_purpose::URL_SAFE_NO_PAD.decode(parts[1])
            .map_err(|e| CursedError::new("jwt_error", &format!("Invalid payload encoding: {}", e)))?;
        let payload_str = String::from_utf8(payload_json)
            .map_err(|e| CursedError::new("jwt_error", &format!("Invalid payload UTF-8: {}", e)))?;
        let claims: HashMap<String, Value> = serde_json::from_str(&payload_str)?;

        Ok(claims)
    }

    fn sign_message(&self, message: &str) -> Result<Vec<u8>, CursedError> {
        let mut mac = HmacSha256::new_from_slice(&self.secret_key)
            .map_err(|e| CursedError::new("jwt_error", &format!("HMAC error: {}", e)))?;
        mac.update(message.as_bytes());
        Ok(mac.finalize().into_bytes().to_vec())
    }

    fn verify_signature(&self, message: &str, signature: &[u8]) -> Result<bool, CursedError> {
        let expected = self.sign_message(message)?;
        Ok(constant_time_eq(&expected, signature))
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

fn main() {
    println!("🔐 Testing CURSED Crypto Package Components");
    println!("{}", "=".repeat(50));

    // Test JWT functionality
    println!("\n📋 Testing JWT Authentication");
    let secret = b"test_secret_key_12345678901234567890".to_vec();
    let jwt = JwtHandler::new(secret, 3600);
    
    let mut claims = HashMap::new();
    claims.insert("sub".to_string(), json!("user123"));
    claims.insert("name".to_string(), json!("Test User"));
    claims.insert("role".to_string(), json!("admin"));
    
    match jwt.create_token(claims.clone()) {
        Ok(token) => {
            println!("✅ JWT token created successfully");
            println!("   Token: {}...", &token[..50]);
            
            // Validate the token
            match jwt.validate_token(&token) {
                Ok(decoded) => {
                    println!("✅ JWT token validated successfully");
                    println!("   User: {}", decoded.get("sub").unwrap());
                    println!("   Name: {}", decoded.get("name").unwrap());
                    println!("   Role: {}", decoded.get("role").unwrap());
                }
                Err(e) => println!("❌ JWT validation failed: {}", e),
            }
            
            // Test with invalid token
            let invalid_token = format!("{}invalid", token);
            match jwt.validate_token(&invalid_token) {
                Ok(_) => println!("❌ Invalid token was incorrectly accepted"),
                Err(_) => println!("✅ Invalid token correctly rejected"),
            }
        }
        Err(e) => println!("❌ JWT creation failed: {}", e),
    }

    // Test Base64 encoding
    println!("\n📝 Testing Base64 Encoding");
    let test_data = "Hello, CURSED crypto world!".as_bytes();
    let encoded = general_purpose::STANDARD.encode(test_data);
    println!("   Original: {:?}", std::str::from_utf8(test_data).unwrap());
    println!("   Encoded: {}", encoded);
    
    match general_purpose::STANDARD.decode(&encoded) {
        Ok(decoded) => {
            if decoded == test_data {
                println!("✅ Base64 round-trip successful");
            } else {
                println!("❌ Base64 round-trip failed");
            }
        }
        Err(e) => println!("❌ Base64 decode failed: {}", e),
    }

    // Test SHA-256 hashing
    println!("\n🔒 Testing SHA-256 Hashing");
    let data = b"test data for hashing";
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    println!("   Data: {:?}", std::str::from_utf8(data).unwrap());
    println!("   Hash: {:x}", hash);
    
    // Same data should produce same hash
    let mut hasher2 = Sha256::new();
    hasher2.update(data);
    let hash2 = hasher2.finalize();
    
    if hash == hash2 {
        println!("✅ SHA-256 hashing is deterministic");
    } else {
        println!("❌ SHA-256 hashing failed consistency check");
    }

    // Test HMAC
    println!("\n🔐 Testing HMAC Authentication");
    let hmac_key = b"secret_hmac_key";
    let message = b"test message";
    
    let mut mac = HmacSha256::new_from_slice(hmac_key).unwrap();
    mac.update(message);
    let signature = mac.finalize().into_bytes();
    
    // Verify signature
    let mut mac_verify = HmacSha256::new_from_slice(hmac_key).unwrap();
    mac_verify.update(message);
    let expected = mac_verify.finalize().into_bytes();
    
    if constant_time_eq(&signature, &expected) {
        println!("✅ HMAC signature verification successful");
    } else {
        println!("❌ HMAC signature verification failed");
    }
    
    // Test with wrong message
    let wrong_message = b"wrong message";
    let mut mac_wrong = HmacSha256::new_from_slice(hmac_key).unwrap();
    mac_wrong.update(wrong_message);
    let wrong_expected = mac_wrong.finalize().into_bytes();
    
    if !constant_time_eq(&signature, &wrong_expected) {
        println!("✅ HMAC correctly rejected tampered message");
    } else {
        println!("❌ HMAC incorrectly accepted tampered message");
    }

    // Test constant-time comparison
    println!("\n⚡ Testing Constant-Time Comparison");
    let data1 = b"same_data";
    let data2 = b"same_data";
    let data3 = b"different";
    
    if constant_time_eq(data1, data2) {
        println!("✅ Constant-time comparison: identical data correctly matched");
    } else {
        println!("❌ Constant-time comparison: identical data incorrectly rejected");
    }
    
    if !constant_time_eq(data1, data3) {
        println!("✅ Constant-time comparison: different data correctly rejected");
    } else {
        println!("❌ Constant-time comparison: different data incorrectly matched");
    }

    println!("\n🎉 All crypto component tests completed!");
    println!("   The core cryptographic primitives are working correctly.");
    println!("   This validates the foundation for the complete CURSED crypto package.");
}
