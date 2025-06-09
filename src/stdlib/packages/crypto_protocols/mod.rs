/// fr fr Cryptographic protocol implementations for CURSED - secure communications bestie
/// 
/// This module provides high-level cryptographic protocols including
/// key exchange, authentication, and secure communication protocols.

// Core cryptographic protocols
pub mod diffie_hellman;
pub mod ecdh;
pub mod key_exchange;
pub mod authentication;

// Secure communication protocols
pub mod tls_handshake;
pub mod secure_channels;
pub mod noise_protocol;
pub mod signal_protocol;

// Key agreement and management
pub mod key_agreement;
pub mod key_derivation;
pub mod session_management;
pub mod forward_secrecy;

// Protocol security
pub mod protocol_verification;
pub mod attack_resistance;
pub mod side_channel_protection;

// Re-export main types
pub use diffie_hellman::{
    DiffieHellmanKeyExchange, DhKeyPair, DhPublicKey, DhPrivateKey,
    DhParameters, DhError, DhResult
};
pub use ecdh::{
    EcdhKeyExchange, EcdhKeyPair, EcdhPublicKey, EcdhPrivateKey,
    EcdhParameters, EcdhError, EcdhResult
};
pub use key_exchange::{
    KeyExchangeProtocol, KeyExchangeResult, ExchangedKey, KeyExchangeError,
    KeyExchangeMethod, ProtocolSecurity
};
pub use authentication::{
    AuthenticationProtocol, AuthenticationResult, AuthChallenge, AuthResponse,
    AuthError, AuthResult, AuthMethod
};

use std::collections::HashMap;
use std::time::Duration;

/// fr fr Supported cryptographic protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CryptoProtocol {
    // Key exchange protocols
    DiffieHellmanModp,
    EcdhP256,
    EcdhP384,
    EcdhP521,
    X25519,
    X448,
    
    // Authentication protocols
    Challenge_Response,
    SRP, // Secure Remote Password
    PAKE, // Password Authenticated Key Exchange
    OPAQUE,
    
    // Secure communication protocols
    TLS13,
    NoiseXX,
    NoiseIK,
    NoiseNK,
    SignalProtocol,
    
    // Hybrid protocols
    HybridPQC, // Hybrid classical + post-quantum
}

impl CryptoProtocol {
    /// slay Get protocol name
    pub fn name(&self) -> &'static str {
        match self {
            CryptoProtocol::DiffieHellmanModp => "Diffie-Hellman MODP",
            CryptoProtocol::EcdhP256 => "ECDH P-256",
            CryptoProtocol::EcdhP384 => "ECDH P-384",
            CryptoProtocol::EcdhP521 => "ECDH P-521",
            CryptoProtocol::X25519 => "X25519",
            CryptoProtocol::X448 => "X448",
            CryptoProtocol::Challenge_Response => "Challenge-Response",
            CryptoProtocol::SRP => "SRP",
            CryptoProtocol::PAKE => "PAKE",
            CryptoProtocol::OPAQUE => "OPAQUE",
            CryptoProtocol::TLS13 => "TLS 1.3",
            CryptoProtocol::NoiseXX => "Noise XX",
            CryptoProtocol::NoiseIK => "Noise IK",
            CryptoProtocol::NoiseNK => "Noise NK",
            CryptoProtocol::SignalProtocol => "Signal Protocol",
            CryptoProtocol::HybridPQC => "Hybrid PQC",
        }
    }
    
    /// slay Get security level (in bits)
    pub fn security_level(&self) -> u32 {
        match self {
            CryptoProtocol::DiffieHellmanModp => 112, // Depends on group
            CryptoProtocol::EcdhP256 => 128,
            CryptoProtocol::EcdhP384 => 192,
            CryptoProtocol::EcdhP521 => 256,
            CryptoProtocol::X25519 => 128,
            CryptoProtocol::X448 => 224,
            CryptoProtocol::Challenge_Response => 128,
            CryptoProtocol::SRP => 128,
            CryptoProtocol::PAKE => 128,
            CryptoProtocol::OPAQUE => 128,
            CryptoProtocol::TLS13 => 128,
            CryptoProtocol::NoiseXX => 128,
            CryptoProtocol::NoiseIK => 128,
            CryptoProtocol::NoiseNK => 128,
            CryptoProtocol::SignalProtocol => 128,
            CryptoProtocol::HybridPQC => 256, // Post-quantum secure
        }
    }
    
    /// slay Check if protocol provides forward secrecy
    pub fn provides_forward_secrecy(&self) -> bool {
        match self {
            CryptoProtocol::DiffieHellmanModp |
            CryptoProtocol::EcdhP256 |
            CryptoProtocol::EcdhP384 |
            CryptoProtocol::EcdhP521 |
            CryptoProtocol::X25519 |
            CryptoProtocol::X448 |
            CryptoProtocol::TLS13 |
            CryptoProtocol::NoiseXX |
            CryptoProtocol::NoiseIK |
            CryptoProtocol::SignalProtocol |
            CryptoProtocol::HybridPQC => true,
            CryptoProtocol::Challenge_Response |
            CryptoProtocol::SRP |
            CryptoProtocol::PAKE |
            CryptoProtocol::OPAQUE |
            CryptoProtocol::NoiseNK => false,
        }
    }
    
    /// slay Check if protocol is standardized
    pub fn is_standardized(&self) -> bool {
        match self {
            CryptoProtocol::DiffieHellmanModp |
            CryptoProtocol::EcdhP256 |
            CryptoProtocol::EcdhP384 |
            CryptoProtocol::EcdhP521 |
            CryptoProtocol::X25519 |
            CryptoProtocol::X448 |
            CryptoProtocol::TLS13 => true, // RFC standards
            CryptoProtocol::SRP |
            CryptoProtocol::OPAQUE => true, // IETF standards
            CryptoProtocol::NoiseXX |
            CryptoProtocol::NoiseIK |
            CryptoProtocol::NoiseNK => true, // Noise framework
            CryptoProtocol::Challenge_Response |
            CryptoProtocol::PAKE |
            CryptoProtocol::SignalProtocol |
            CryptoProtocol::HybridPQC => false, // Various implementations
        }
    }
    
    /// slay Get protocol category
    pub fn category(&self) -> ProtocolCategory {
        match self {
            CryptoProtocol::DiffieHellmanModp |
            CryptoProtocol::EcdhP256 |
            CryptoProtocol::EcdhP384 |
            CryptoProtocol::EcdhP521 |
            CryptoProtocol::X25519 |
            CryptoProtocol::X448 => ProtocolCategory::KeyExchange,
            
            CryptoProtocol::Challenge_Response |
            CryptoProtocol::SRP |
            CryptoProtocol::PAKE |
            CryptoProtocol::OPAQUE => ProtocolCategory::Authentication,
            
            CryptoProtocol::TLS13 |
            CryptoProtocol::NoiseXX |
            CryptoProtocol::NoiseIK |
            CryptoProtocol::NoiseNK |
            CryptoProtocol::SignalProtocol => ProtocolCategory::SecureCommunication,
            
            CryptoProtocol::HybridPQC => ProtocolCategory::Hybrid,
        }
    }
}

/// fr fr Protocol categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolCategory {
    KeyExchange,
    Authentication,
    SecureCommunication,
    Hybrid,
}

/// fr fr Protocol errors
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolError {
    UnsupportedProtocol(String),
    ProtocolFailure(String),
    AuthenticationFailed,
    KeyExchangeFailed,
    InvalidMessage,
    InvalidState,
    TimeoutError,
    ReplayAttackDetected,
    ManInTheMiddleDetected,
    WeakParameters,
    Internal(String),
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::UnsupportedProtocol(name) => 
                write!(f, "Unsupported protocol: {}", name),
            ProtocolError::ProtocolFailure(msg) => write!(f, "Protocol failure: {}", msg),
            ProtocolError::AuthenticationFailed => write!(f, "Authentication failed"),
            ProtocolError::KeyExchangeFailed => write!(f, "Key exchange failed"),
            ProtocolError::InvalidMessage => write!(f, "Invalid protocol message"),
            ProtocolError::InvalidState => write!(f, "Invalid protocol state"),
            ProtocolError::TimeoutError => write!(f, "Protocol timeout"),
            ProtocolError::ReplayAttackDetected => write!(f, "Replay attack detected"),
            ProtocolError::ManInTheMiddleDetected => write!(f, "Man-in-the-middle attack detected"),
            ProtocolError::WeakParameters => write!(f, "Weak protocol parameters"),
            ProtocolError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ProtocolError {}

/// fr fr Protocol result type
pub type ProtocolResult<T> = Result<T, ProtocolError>;

/// fr fr Protocol execution context
#[derive(Debug, Clone)]
pub struct ProtocolContext {
    pub protocol: CryptoProtocol,
    pub role: ProtocolRole,
    pub parameters: ProtocolParameters,
    pub security_policy: SecurityPolicy,
    pub timeout: Duration,
}

impl ProtocolContext {
    /// slay Create new protocol context
    pub fn new(protocol: CryptoProtocol, role: ProtocolRole) -> Self {
        Self {
            protocol,
            role,
            parameters: ProtocolParameters::default_for_protocol(protocol),
            security_policy: SecurityPolicy::default(),
            timeout: Duration::from_secs(30),
        }
    }
}

/// fr fr Protocol role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolRole {
    Initiator,
    Responder,
    Server,
    Client,
}

/// fr fr Protocol parameters
#[derive(Debug, Clone)]
pub struct ProtocolParameters {
    pub key_size: usize,
    pub hash_algorithm: String,
    pub cipher_suite: String,
    pub custom_parameters: HashMap<String, String>,
}

impl ProtocolParameters {
    /// slay Get default parameters for protocol
    pub fn default_for_protocol(protocol: CryptoProtocol) -> Self {
        match protocol {
            CryptoProtocol::EcdhP256 => Self {
                key_size: 256,
                hash_algorithm: "SHA256".to_string(),
                cipher_suite: "ECDH-P256".to_string(),
                custom_parameters: HashMap::new(),
            },
            CryptoProtocol::X25519 => Self {
                key_size: 255,
                hash_algorithm: "SHA256".to_string(),
                cipher_suite: "X25519".to_string(),
                custom_parameters: HashMap::new(),
            },
            CryptoProtocol::TLS13 => Self {
                key_size: 256,
                hash_algorithm: "SHA256".to_string(),
                cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
                custom_parameters: HashMap::new(),
            },
            _ => Self {
                key_size: 256,
                hash_algorithm: "SHA256".to_string(),
                cipher_suite: "default".to_string(),
                custom_parameters: HashMap::new(),
            },
        }
    }
}

/// fr fr Security policy for protocols
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub minimum_security_level: u32,
    pub require_forward_secrecy: bool,
    pub allow_weak_parameters: bool,
    pub require_authentication: bool,
    pub defend_against_quantum: bool,
    pub maximum_protocol_age: Duration,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            minimum_security_level: 128,
            require_forward_secrecy: true,
            allow_weak_parameters: false,
            require_authentication: true,
            defend_against_quantum: false,
            maximum_protocol_age: Duration::from_secs(365 * 24 * 3600), // 1 year
        }
    }
}

/// fr fr Utilities and helper functions


pub mod utils {
    use super::*;
    
    /// slay Placeholder utility function
    pub fn placeholder() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

/// fr fr Initialize the crypto_protocols package
pub fn init_crypto_protocols() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 crypto_protocols package initialized - ready bestie!");
    Ok(())
}
