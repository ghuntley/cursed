//! Security module for CURSED - Production-ready security implementations
//! 
//! This module provides comprehensive security primitives including:
//! - Memory safety with guard pages and canaries
//! - Cryptographic operations with side-channel protection  
//! - SQL injection prevention with prepared statements
//! - TLS/SSL network security with certificate validation
//! - Input validation and sanitization

pub mod memory_safety;
pub mod crypto_secure;
pub mod database_secure;
pub mod network_secure;
pub mod input_validation;

pub use memory_safety::{SecureMemoryRegion, safe_transmute};
pub use crypto_secure::{
    SecureRng, SecureHash, KeyDerivation, AuthenticatedEncryption, 
    DigitalSignature, KeyManager, EncryptedData
};
pub use database_secure::{
    PreparedStatement, SqlParameter, SecureQueryBuilder, SecureConnection
};
pub use network_secure::{
    TlsConfig, SecurityLevel, SecureTlsClient, SecureTlsServer, NetworkSecurity
};
pub use input_validation::{
    InputValidator, HtmlSanitizer, SqlSanitizer, PathSanitizer, 
    EmailValidator, CommandSanitizer
};

use crate::error::CursedError;

/// Security policy configuration
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub enforce_memory_safety: bool,
    pub require_encryption: bool,
    pub validate_all_inputs: bool,
    pub require_tls: bool,
    pub log_security_events: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            enforce_memory_safety: true,
            require_encryption: true,
            validate_all_inputs: true,
            require_tls: true,
            log_security_events: true,
        }
    }
}

/// Central security manager
pub struct SecurityManager {
    policy: SecurityPolicy,
    crypto_handler: AuthenticatedEncryption,
    input_validator: InputValidator,
}

impl SecurityManager {
    /// Create new security manager with policy
    pub fn new(policy: SecurityPolicy) -> Result<Self, CursedError> {
        let crypto_handler = AuthenticatedEncryption::new();
        let input_validator = InputValidator::new();

        Ok(Self {
            policy,
            crypto_handler,
            input_validator,
        })
    }

    /// Validate and sanitize input according to policy
    pub fn validate_input(&self, input: &str) -> Result<String, CursedError> {
        if !self.policy.validate_all_inputs {
            return Ok(input.to_string());
        }

        self.input_validator.validate_string(input)
    }

    /// Encrypt sensitive data if required by policy
    pub fn protect_data(&self, key: &[u8], data: &[u8]) -> Result<EncryptedData, CursedError> {
        if !self.policy.require_encryption {
            // Return unencrypted data wrapped in structure
            return Ok(EncryptedData {
                nonce: [0u8; 12], // Empty nonce indicates no encryption
                ciphertext: data.to_vec(),
                associated_data: b"unencrypted".to_vec(),
            });
        }

        self.crypto_handler.encrypt(key, data, b"cursed_runtime")
    }

    /// Log security event if enabled
    pub fn log_security_event(&self, event: &str, severity: SecuritySeverity) {
        if self.policy.log_security_events {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            println!("[SECURITY {}] {}: {}", 
                severity.as_str(), 
                timestamp, 
                event
            );
        }
    }
}

/// Security event severity levels
#[derive(Debug, Clone, Copy)]
pub enum SecuritySeverity {
    Info,
    Warning,
    Critical,
}

impl SecuritySeverity {
    fn as_str(&self) -> &'static str {
        match self {
            SecuritySeverity::Info => "INFO",
            SecuritySeverity::Warning => "WARNING", 
            SecuritySeverity::Critical => "CRITICAL",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_manager_creation() {
        let policy = SecurityPolicy::default();
        let manager = SecurityManager::new(policy).unwrap();
        
        // Test input validation
        let result = manager.validate_input("Hello, World!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_security_policy() {
        let mut policy = SecurityPolicy::default();
        policy.validate_all_inputs = false;
        
        let manager = SecurityManager::new(policy).unwrap();
        
        // Should pass even with potentially dangerous input when validation disabled
        let result = manager.validate_input("<script>alert('test')</script>");
        assert!(result.is_ok());
    }
}
