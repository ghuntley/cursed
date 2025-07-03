//! SECURITY WARNING MODULE
//! 
//! This module contains security warnings for disabled cryptographic functions
//! that were found to have unsafe placeholder implementations.

use crate::error::CursedError;
use crate::stdlib::packages::CryptoError;

/// Print security warning for disabled crypto functions
pub fn print_crypto_security_warning() {
    eprintln!("⚠️  CRYPTO SECURITY WARNING ⚠️");
    eprintln!("╔═══════════════════════════════════════════════════════════════╗");
    eprintln!("║ CRITICAL CRYPTOGRAPHIC MODULES HAVE BEEN DISABLED FOR SECURITY");
    eprintln!("║");
    eprintln!("║ The following functions contained security vulnerabilities:");
    eprintln!("║ • X25519/X448/DH key generation used public_key.reverse()");
    eprintln!("║ • Signature verification functions returned Ok(true) bypass");
    eprinteln!("║");
    eprintln!("║ These modules are now DISABLED and will return security errors.");
    eprintln!("║ Do NOT re-enable without proper cryptographic implementations.");
    eprintln!("╚═══════════════════════════════════════════════════════════════╝");
}

/// Check if crypto functions are safe to use
pub fn verify_crypto_safety() -> Result<(), CursedError> {
    Err(CryptoError::KeyGenerationFailed)
}

/// Security audit result
pub struct SecurityAuditResult {
    pub vulnerabilities_found: usize,
    pub modules_disabled: Vec<String>,
    pub security_status: SecurityStatus,
}

#[derive(Debug, PartialEq)]
pub enum SecurityStatus {
    Critical,
    Warning,
    Safe,
}

/// Perform security audit of crypto modules
pub fn audit_crypto_modules() -> SecurityAuditResult {
    SecurityAuditResult {
        vulnerabilities_found: 6, // X25519, X448, DH key gen + 3 signature bypasses
        modules_disabled: vec![
            "crypto_asymmetric::x25519_generate_keypair".to_string(),
            "crypto_asymmetric::x448_generate_keypair".to_string(),
            "crypto_asymmetric::dh_generate_keypair".to_string(),
            "crypto_signatures::quick_ed25519_sign_verify".to_string(),
            "crypto_signatures::quick_ecdsa_sign_verify".to_string(),
            "crypto_signatures::quick_rsa_sign_verify".to_string(),
        ],
        security_status: SecurityStatus::Critical,
    }
}
