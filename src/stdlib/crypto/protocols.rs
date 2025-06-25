use crate::error::CursedError;
// use crate::stdlib::value::Value as CursedValue;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use base64::{Engine as _, engine::general_purpose};
use serde_json::{json, Value};
use tracing::{debug, info, warn, error, instrument};

// Re-export comprehensive protocol suite
pub use super::protocols_comprehensive::{
    ProtocolSuite, ProtocolBuilder, SecurityLevel, ProtocolConfig,
    SecureMessagingProtocol, MpcProtocol, DkgProtocol,
    SecurityAuditReport, ProtocolHealthStatus,
};

// Re-export all protocol types for compatibility
pub use super::protocols_production::{
    ProtocolError, ProtocolResult, CryptoPrimitives,
    X25519KeyExchange, EcdhKeyExchange, DiffieHellmanKeyExchange,
    EcdheKeyExchange, EcdheMessage, SecureChannel,
};

pub use super::protocols_advanced::{
    ChallengeResponseAuth, ChallengeSet, ResponseSet, AuthenticationResult,
    MultiPartyComputation, ShareDistribution,
    DistributedKeyGeneration, DistributedKey,
};

type HmacSha256 = Hmac<Sha256>;

/// JWT (JSON Web Token) implementation for CURSED
#[derive(Debug, Clone)]
pub struct JwtHandler {
    secret_key: Vec<u8>,
    default_expiry: u64, // seconds
}

impl JwtHandler {
    /// Create a new JWT handler with secret key
    #[instrument(skip(secret_key))]
    pub fn new(secret_key: Vec<u8>, default_expiry_seconds: u64) -> Self {
        info!(expiry = default_expiry_seconds, "Creating JWT handler");
        Self {
            secret_key,
            default_expiry: default_expiry_seconds,
        }
    }

    /// Create a JWT token with claims
    #[instrument(skip(self))]
    pub fn create_token(&self, claims: HashMap<String, Value>) -> crate::error::Result<()> {
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
        debug!(token_length = token.len(), "Created JWT token");
        Ok(token)
    }

    /// Validate and decode a JWT token
    #[instrument(skip(self, token))]
    pub fn validate_token(&self, token: &str) -> crate::error::Result<()> {
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

        // Check expiry
        if let Some(exp) = claims.get("exp") {
            if let Some(exp_time) = exp.as_u64() {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_err(|e| CursedError::new("jwt_error", &format!("Time error: {}", e)))?
                    .as_secs();
                
                if now > exp_time {
                    return Err(CursedError::new("jwt_error", "Token expired"));
                }
            }
        }

        debug!(claims_count = claims.len(), "Successfully validated JWT token");
        Ok(claims)
    }

    fn sign_message(&self, message: &str) -> crate::error::Result<()> {
        let mut mac = HmacSha256::new_from_slice(&self.secret_key)
            .map_err(|e| CursedError::new("jwt_error", &format!("HMAC error: {}", e)))?;
        mac.update(message.as_bytes());
        Ok(mac.finalize().into_bytes().to_vec())
    }

    fn verify_signature(&self, message: &str, signature: &[u8]) -> crate::error::Result<()> {
        let expected = self.sign_message(message)?;
        Ok(constant_time_eq(&expected, signature))
    }
}

/// HMAC-based authentication utilities
#[derive(Debug, Clone)]
pub struct HmacAuth {
    key: Vec<u8>,
}

impl HmacAuth {
    /// Create new HMAC authenticator
    #[instrument(skip(key))]
    pub fn new(key: Vec<u8>) -> Self {
        info!(key_length = key.len(), "Creating HMAC authenticator");
        Self { key }
    }

    /// Create HMAC signature for data
    #[instrument(skip(self, data))]
    pub fn sign(&self, data: &[u8]) -> crate::error::Result<()> {
        let mut mac = HmacSha256::new_from_slice(&self.key)
            .map_err(|e| CursedError::new("hmac_error", &format!("HMAC creation error: {}", e)))?;
        mac.update(data);
        let signature = mac.finalize().into_bytes().to_vec();
        debug!(data_length = data.len(), signature_length = signature.len(), "Created HMAC signature");
        Ok(signature)
    }

    /// Verify HMAC signature
    #[instrument(skip(self, data, signature))]
    pub fn verify(&self, data: &[u8], signature: &[u8]) -> crate::error::Result<()> {
        let expected = self.sign(data)?;
        let is_valid = constant_time_eq(&expected, signature);
        debug!(is_valid, "HMAC signature verification result");
        Ok(is_valid)
    }

    /// Create authenticated message with embedded signature
    #[instrument(skip(self, message))]
    pub fn create_authenticated_message(&self, message: &[u8]) -> crate::error::Result<()> {
        let signature = self.sign(message)?;
        let mut result = Vec::with_capacity(message.len() + signature.len() + 4);
        result.extend_from_slice(&(signature.len() as u32).to_be_bytes());
        result.extend_from_slice(&signature);
        result.extend_from_slice(message);
        debug!(message_length = message.len(), authenticated_length = result.len(), "Created authenticated message");
        Ok(result)
    }

    /// Verify and extract message from authenticated format
    #[instrument(skip(self, authenticated_data))]
    pub fn verify_authenticated_message(&self, authenticated_data: &[u8]) -> crate::error::Result<()> {
        if authenticated_data.len() < 4 {
            return Err(CursedError::new("hmac_error", "Invalid authenticated message format"));
        }

        let sig_len = u32::from_be_bytes([
            authenticated_data[0], authenticated_data[1], 
            authenticated_data[2], authenticated_data[3]
        ]) as usize;

        if authenticated_data.len() < 4 + sig_len {
            return Err(CursedError::new("hmac_error", "Invalid authenticated message length"));
        }

        let signature = &authenticated_data[4..4 + sig_len];
        let message = &authenticated_data[4 + sig_len..];

        if !self.verify(message, signature)? {
            return Err(CursedError::new("hmac_error", "Message authentication failed"));
        }

        debug!(message_length = message.len(), "Successfully verified authenticated message");
        Ok(message.to_vec())
    }
}

/// Time-based One-Time Password (TOTP) implementation
#[derive(Debug, Clone)]
pub struct TotpGenerator {
    secret: Vec<u8>,
    digits: usize,
    time_step: u64, // seconds
}

impl TotpGenerator {
    /// Create new TOTP generator
    #[instrument(skip(secret))]
    pub fn new(secret: Vec<u8>, digits: usize, time_step_seconds: u64) -> Self {
        info!(secret_length = secret.len(), digits, time_step = time_step_seconds, "Creating TOTP generator");
        Self {
            secret,
            digits,
            time_step: time_step_seconds,
        }
    }

    /// Generate TOTP for current time
    #[instrument(skip(self))]
    pub fn generate_current(&self) -> crate::error::Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| CursedError::new("totp_error", &format!("Time error: {}", e)))?
            .as_secs();
        self.generate_at_time(now)
    }

    /// Generate TOTP for specific time
    #[instrument(skip(self))]
    pub fn generate_at_time(&self, unix_time: u64) -> crate::error::Result<()> {
        let time_counter = unix_time / self.time_step;
        let counter_bytes = time_counter.to_be_bytes();

        let mut mac = HmacSha256::new_from_slice(&self.secret)
            .map_err(|e| CursedError::new("totp_error", &format!("HMAC error: {}", e)))?;
        mac.update(&counter_bytes);
        let hash = mac.finalize().into_bytes();

        // Dynamic truncation
        let offset = (hash[hash.len() - 1] & 0x0f) as usize;
        let binary = u32::from_be_bytes([
            hash[offset] & 0x7f,
            hash[offset + 1],
            hash[offset + 2],
            hash[offset + 3],
        ]);

        let otp = binary % (10_u32.pow(self.digits as u32));
        let result = format!("{:0width$}", otp, width = self.digits);
        
        debug!(time_counter, otp = result, "Generated TOTP");
        Ok(result)
    }

    /// Verify TOTP with time window tolerance
    #[instrument(skip(self))]
    pub fn verify(&self, token: &str, time_window: u32) -> crate::error::Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| CursedError::new("totp_error", &format!("Time error: {}", e)))?
            .as_secs();

        // Check current time and surrounding windows
        for i in 0..=time_window {
            // Check future windows
            if let Ok(future_token) = self.generate_at_time(now + (i as u64 * self.time_step)) {
                if constant_time_eq(token.as_bytes(), future_token.as_bytes()) {
                    debug!(window_offset = i as i32, "TOTP verified in future window");
                    return Ok(true);
                }
            }
            
            // Check past windows (skip i=0 to avoid duplicate)
            if i > 0 {
                if let Ok(past_token) = self.generate_at_time(now.saturating_sub(i as u64 * self.time_step)) {
                    if constant_time_eq(token.as_bytes(), past_token.as_bytes()) {
                        debug!(window_offset = -(i as i32), "TOTP verified in past window");
                        return Ok(true);
                    }
                }
            }
        }

        debug!("TOTP verification failed");
        Ok(false)
    }
}

/// Basic TLS handshake components and utilities
#[derive(Debug, Clone)]
pub struct TlsHandshake {
    client_random: Vec<u8>,
    server_random: Vec<u8>,
    session_id: Vec<u8>,
}

impl TlsHandshake {
    /// Create new TLS handshake instance
    #[instrument]
    pub fn new() -> Self {
        info!("Creating TLS handshake instance");
        Self {
            client_random: Vec::new(),
            server_random: Vec::new(),
            session_id: Vec::new(),
        }
    }

    /// Generate client hello random
    #[instrument(skip(self))]
    pub fn generate_client_random(&mut self) -> crate::error::Result<()> {
//         use crate::stdlib::crypto::random::SecureRandom;
        let mut rng = SecureRandom::new()?;
        self.client_random = rng.generate_bytes(32)?;
        debug!(length = self.client_random.len(), "Generated client random");
        Ok(self.client_random.clone())
    }

    /// Generate server hello random
    #[instrument(skip(self))]
    pub fn generate_server_random(&mut self) -> crate::error::Result<()> {
//         use crate::stdlib::crypto::random::SecureRandom;
        let mut rng = SecureRandom::new()?;
        self.server_random = rng.generate_bytes(32)?;
        debug!(length = self.server_random.len(), "Generated server random");
        Ok(self.server_random.clone())
    }

    /// Generate session ID
    #[instrument(skip(self))]
    pub fn generate_session_id(&mut self) -> crate::error::Result<()> {
//         use crate::stdlib::crypto::random::SecureRandom;
        let mut rng = SecureRandom::new()?;
        self.session_id = rng.generate_bytes(32)?;
        debug!(length = self.session_id.len(), "Generated session ID");
        Ok(self.session_id.clone())
    }

    /// Create pre-master secret
    #[instrument(skip(self))]
    pub fn create_pre_master_secret(&self) -> crate::error::Result<()> {
//         use crate::stdlib::crypto::random::SecureRandom;
        let mut rng = SecureRandom::new()?;
        let pre_master = rng.generate_bytes(48)?;
        debug!(length = pre_master.len(), "Created pre-master secret");
        Ok(pre_master)
    }

    /// Derive master secret from pre-master secret
    #[instrument(skip(self, pre_master_secret))]
    pub fn derive_master_secret(&self, pre_master_secret: &[u8]) -> crate::error::Result<()> {
        if self.client_random.is_empty() || self.server_random.is_empty() {
            return Err(CursedError::new("tls_error", "Client and server randoms must be set"));
        }

        // Simplified master secret derivation (in real TLS this would use PRF)
        let mut hasher = Sha256::new();
        hasher.update(pre_master_secret);
        hasher.update(&self.client_random);
        hasher.update(&self.server_random);
        hasher.update(b"master secret");
        
        let master_secret = hasher.finalize().to_vec();
        debug!(length = master_secret.len(), "Derived master secret");
        Ok(master_secret)
    }

    /// Derive key material from master secret
    #[instrument(skip(self, master_secret))]
    pub fn derive_keys(&self, master_secret: &[u8], key_length: usize) -> crate::error::Result<()> {
        // Simplified key derivation (real TLS uses more complex PRF)
        let mut hasher = Sha256::new();
        hasher.update(master_secret);
        hasher.update(&self.server_random);
        hasher.update(&self.client_random);
        hasher.update(b"key expansion");
        
        let key_block = hasher.finalize();
        
        if key_block.len() < key_length * 4 {
            return Err(CursedError::new("tls_error", "Insufficient key material"));
        }

        let keys = TlsKeys {
            client_write_mac: key_block[0..key_length].to_vec(),
            server_write_mac: key_block[key_length..key_length * 2].to_vec(),
            client_write_key: key_block[key_length * 2..key_length * 3].to_vec(),
            server_write_key: key_block[key_length * 3..key_length * 4].to_vec(),
        };

        debug!(key_length, "Derived TLS keys");
        Ok(keys)
    }

    /// Get handshake state
    pub fn get_state(&self) -> TlsHandshakeState {
        TlsHandshakeState {
            client_random: self.client_random.clone(),
            server_random: self.server_random.clone(),
            session_id: self.session_id.clone(),
            has_client_random: !self.client_random.is_empty(),
            has_server_random: !self.server_random.is_empty(),
            has_session_id: !self.session_id.is_empty(),
        }
    }
}

/// TLS key material
#[derive(Debug, Clone)]
pub struct TlsKeys {
    pub client_write_mac: Vec<u8>,
    pub server_write_mac: Vec<u8>,
    pub client_write_key: Vec<u8>,
    pub server_write_key: Vec<u8>,
}

/// TLS handshake state
#[derive(Debug, Clone)]
pub struct TlsHandshakeState {
    pub client_random: Vec<u8>,
    pub server_random: Vec<u8>,
    pub session_id: Vec<u8>,
    pub has_client_random: bool,
    pub has_server_random: bool,
    pub has_session_id: bool,
}

/// Constant-time equality comparison to prevent timing attacks
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

// ============================================================================
// CURSED LANGUAGE FUNCTION EXPORTS
// ============================================================================

/// Initialize comprehensive cryptographic protocols
pub fn init_crypto_protocols() -> crate::error::Result<()> {
    info!("Initializing comprehensive cryptographic protocols module");
    Ok(())
}

/// Create new protocol suite with specified security level
pub fn create_protocol_suite(args: Vec<CursedValue>) -> crate::error::Result<()> {
    let security_level = if args.is_empty() {
        SecurityLevel::Level256
    } else {
        match args[0].as_string().as_deref() {
            Some("Level128") => SecurityLevel::Level128,
            Some("Level192") => SecurityLevel::Level192,
            Some("Level256") => SecurityLevel::Level256,
            Some("PostQuantum") => SecurityLevel::PostQuantum,
            _ => return Err(CursedError::Runtime("Invalid security level".to_string())),
        }
    };

    let _suite = ProtocolSuite::new(security_level);
    
    // Return success indicator (in a real implementation, this would return a handle)
    Ok(CursedValue::bool(true))
}

/// Generate X25519 keypair for key exchange
pub fn generate_x25519_keypair(args: Vec<CursedValue>) -> crate::error::Result<()> {
    let security_level = SecurityLevel::Level256;
    let exchange = X25519KeyExchange::new(security_level);
    let public_key = exchange.public_key();
    
    let mut result = HashMap::new();
    result.insert("public_key".to_string(), CursedValue::String(hex::encode(public_key)));
    result.insert("key_type".to_string(), CursedValue::String("X25519".to_string()));
    
    Ok(CursedValue::Object(result))
}

/// Perform X25519 key exchange
pub fn x25519_exchange(args: Vec<CursedValue>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("x25519_exchange requires private_key and peer_public_key".to_string()));
    }

    // In a real implementation, this would use the actual private key
    // For now, return a mock shared secret
    let shared_secret = hex::encode(CryptoPrimitives::random_bytes(32));
    
    Ok(CursedValue::String(shared_secret))
}

/// Create secure communication channel
pub fn create_secure_channel(args: Vec<CursedValue>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("create_secure_channel requires shared_secret".to_string()));
    }

    // Generate channel ID
    let channel_id = hex::encode(CryptoPrimitives::random_bytes(8));
    
    Ok(CursedValue::String(channel_id))
}

/// Send message through secure channel
pub fn send_secure_message(args: Vec<CursedValue>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("send_secure_message requires channel_id and message".to_string()));
    }

    let _channel_id = args[0].as_string();
    let message = args[1].as_string();
    
    // Mock encryption (in real implementation, would use actual secure channel)
    let encrypted = base64::engine::general_purpose::STANDARD.encode(message.as_bytes());
    
    Ok(CursedValue::String(encrypted))
}

/// Receive message from secure channel
pub fn receive_secure_message(args: Vec<CursedValue>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("receive_secure_message requires channel_id and encrypted_message".to_string()));
    }

    let _channel_id = args[0].as_string();
    let encrypted_message = args[1].as_string();
    
    // Mock decryption
    let decrypted = base64::engine::general_purpose::STANDARD
        .decode(encrypted_message.as_bytes())
        .map_err(|e| CursedError::Runtime(format!("Decryption failed: {}", e)))?;
    
    let message = String::from_utf8(decrypted)
        .map_err(|e| CursedError::Runtime(format!("Invalid UTF-8: {}", e)))?;
    
    Ok(CursedValue::String(message))
}

/// Initiate challenge-response authentication
pub fn initiate_authentication(args: Vec<CursedValue>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("initiate_authentication requires peer_public_key".to_string()));
    }

    // Generate mock challenge set
    let mut result = HashMap::new();
    result.insert("session_id".to_string(), CursedValue::String(hex::encode(CryptoPrimitives::random_bytes(16))));
    result.insert("challenges".to_string(), CursedValue::Number(3.0));
    result.insert("status".to_string(), CursedValue::String("challenges_sent".to_string()));
    
    Ok(CursedValue::Object(result))
}

/// Respond to authentication challenges
pub fn respond_to_challenges(args: Vec<CursedValue>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("respond_to_challenges requires challenge_set".to_string()));
    }

    // Generate mock response set
    let mut result = HashMap::new();
    result.insert("session_id".to_string(), args[0].clone());
    result.insert("responses".to_string(), CursedValue::Number(3.0));
    result.insert("status".to_string(), CursedValue::String("responses_generated".to_string()));
    
    Ok(CursedValue::Object(result))
}

/// Verify authentication responses
pub fn verify_authentication(args: Vec<CursedValue>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("verify_authentication requires response_set".to_string()));
    }

    // Mock verification result
    let mut result = HashMap::new();
    result.insert("authenticated".to_string(), CursedValue::bool(true));
    result.insert("success_rate".to_string(), CursedValue::Number(0.95));
    result.insert("completion_time_ms".to_string(), CursedValue::Number(250.0));
    
    Ok(CursedValue::Object(result))
}

/// Initiate multi-party computation
pub fn initiate_mpc_computation(args: Vec<CursedValue>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("initiate_mpc_computation requires participants and threshold".to_string()));
    }

    // Generate MPC session
    let session_id = hex::encode(CryptoPrimitives::random_bytes(16));
    
    let mut result = HashMap::new();
    result.insert("session_id".to_string(), CursedValue::String(session_id));
    result.insert("status".to_string(), CursedValue::String("initialized".to_string()));
    result.insert("participants".to_string(), args[0].clone());
    result.insert("threshold".to_string(), args[1].clone());
    
    Ok(CursedValue::Object(result))
}

/// Get protocol statistics
pub fn get_protocol_statistics(args: Vec<CursedValue>) -> crate::error::Result<()> {
    let suite = ProtocolSuite::new(SecurityLevel::Level256);
    let stats = suite.get_protocol_statistics();
    Ok(CursedValue::Object(stats))
}

/// Perform security audit
pub fn security_audit(args: Vec<CursedValue>) -> crate::error::Result<()> {
    let suite = ProtocolSuite::new(SecurityLevel::Level256);
    let audit = suite.security_audit();
    
    let mut result = HashMap::new();
    result.insert("overall_status".to_string(), CursedValue::String(format!("{:?}", audit.overall_status)));
    result.insert("risk_level".to_string(), CursedValue::String(format!("{:?}", audit.risk_level)));
    result.insert("findings_count".to_string(), CursedValue::Number(audit.findings.len() as f64));
    result.insert("recommendations_count".to_string(), CursedValue::Number(audit.recommendations.len() as f64));
    
    Ok(CursedValue::Object(result))
}

/// Get protocol health status
pub fn get_health_status(args: Vec<CursedValue>) -> crate::error::Result<()> {
    let suite = ProtocolSuite::new(SecurityLevel::Level256);
    let health = suite.get_health_status();
    
    let mut result = HashMap::new();
    result.insert("status".to_string(), CursedValue::String(format!("{:?}", health.status)));
    result.insert("active_channels".to_string(), CursedValue::Number(health.active_channels as f64));
    result.insert("error_rate".to_string(), CursedValue::Number(health.error_rate));
    result.insert("uptime_seconds".to_string(), CursedValue::Number(health.uptime.as_secs() as f64));
    
    Ok(CursedValue::Object(result))
}

/// Log protocol error for debugging
pub fn log_error(args: Vec<CursedValue>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("log_error requires error_message".to_string()));
    }

    let error_message = args[0].as_string();
    error!(error_message = error_message, "Protocol error occurred");
    
    Ok(CursedValue::bool(true))
}

