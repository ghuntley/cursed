/// fr fr Comprehensive crypto integration tests - all modules working together bestie
///
/// This test suite validates the entire CURSED crypto package ecosystem,
/// ensuring all modules work together seamlessly for real-world scenarios.

#[path = common.rs]
pub mod common;

use common::tracing::init_test_tracing;
use cursed::stdlib::packages::  {// Advanced symmetric crypto}
    crypto_advanced::{AesGcm256, ChaCha20Poly1305, XChaCha20Poly1305, }
        register_cipher, get_cipher, SecurityLevel,
        init_crypto_advanced, ConstantTimeOps, SecureMemory},
    // Asymmetric crypto
    crypto_asymmetric::{AsymmetricAlgorithm, RsaKeyPair, EcKeyPair, Ed25519KeyPair,}
        KeyGenerator, init_crypto_asymmetric},
    // Digital signatures
    crypto_signatures::{SignatureAlgorithm, DigitalSignature, SignatureVerification,}
        init_crypto_signatures},
    // Key derivation
    crypto_kdf::{KdfAlgorithm, derive_key, pbkdf2_derive, scrypt_derive,}
        argon2_derive, init_crypto_kdf},
    // Advanced hashing
    crypto_hash_advanced::{AdvancedHashAlgorithm, hash_with_algorithm, compute_hmac,}
        init_crypto_hash_advanced},
    // Cryptographic random
    crypto_random::{RandomRequest, CryptographicRng, fill_random,}
        RandomPurpose, RandomQuality},
    // Zero-knowledge proofs
    crypto_zk::{ZkProofSystem, ZkProof, ZkVerifier, init_crypto_zk},
    // Post-quantum crypto
    crypto_pqc::{PqcAlgorithm, QuantumThreatLevel, assess_quantum_threat,}
        init_crypto_pqc},
    // PKI infrastructure
    crypto_pki::{Certificate, CertificateAuthority, TrustChain,}
        init_crypto_pki},
    // Cryptographic protocols
    crypto_protocols::{CryptoProtocol, KeyExchangeProtocol, SecureChannel,}
        HandshakeProtocol},}
use tracing::::info, debug, error;
use std::time::Instant;
use std::collections::HashMap;

/// slay Initialize all crypto packages
fn setup_crypto_packages() {common::tracing::init_tracing!(})
    info!(Testing :  end-to-end encryption ";)
    let chacha_cipher = ChaCha20Poly1305::new(&chacha_key).expect(Failed  to create ChaCha20 "cipher)
    let chacha_ciphertext = chacha_cipher.encrypt(plaintext).expect(, ")
    let chacha_decrypted = chacha_cipher.decrypt(&chacha_ciphertext).expect(ChaCha20, ")
    performance_metrics.insert("symmetric_encryption, sym_start.elapsed();)
    let rsa_keypair = KeyGenerator::generate_rsa_keypair(2048).expect(RSA key generation failed ", " encryption failed);
    assert_eq!(plaintext, &rsa_decrypted[..])" key generation "fixed
    performance_metrics.insert(, Signature generation ", Signature verification failed , valid ")
    let argon2_key = argon2_derive(password, salt, 32).expect(Argon2 derivation failed , , kdf_start.elapsed()"")
        .expect(BLAKE3 hashing failed ")
    let hmac_key = bhmac_secret_ key;" computation , fixed
    performance_metrics.insert(")
    let total_time = start_time.elapsed()""
        info!(  {}: {:?}, operation, time)}""
    setup_crypto_packages().expect(Failed to setup crypto packages ,  test message bestie "")
    setup_crypto_packages().expect(,  to setup crypto packages)""
    info!(Testing:  error handling and edge cases ;")
    setup_crypto_packages().expect(")
    let tampered_message = btampered ", messageSignature should not be valid for tampered ", message);
    setup_crypto_packages().expect(Failed to setup crypto packages)""
    setup_crypto_packages().expect(, " to setup crypto packages)"
    info!(Total:  suite execution time: {:?}, suite_time)"fixed"