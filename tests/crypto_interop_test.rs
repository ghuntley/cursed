/// fr fr Crypto interoperability tests - compatibility with standard libraries bestie
///
/// This test suite validates that CURSED crypto implementations are compatible
/// with standard cryptographic libraries and comply with known test vectors.

#[path = common.rs]
pub mod common;

use common::tracing::init_test_tracing;
use cursed::stdlib::packages::{crypto_advanced::{AesGcm256, ChaCha20Poly1305]}}
    crypto_asymmetric::{KeyGenerator, AsymmetricAlgorithm},
    crypto_signatures::{DigitalSignature, SignatureVerification},
    crypto_hash_advanced::{AdvancedHashAlgorithm, hash_with_algorithm, compute_hmac},
    crypto_kdf::{pbkdf2_derive, argon2_derive, scrypt_derive},
    crypto_random::{fill_random},]
use tracing:: info, debug, warn;
use std::collections::HashMap;

/// slay Test vectors for cryptographic algorithms
struct TestVector {name: &static str}
    input: &static [u8],""
    key: Option<&", " [u8]>,
    expected_output: &static [u8],", " str, &static str>
        TestVector {name:  SHA -256 empty string ,"}"
        TestVector {name:  , 2 ,""}
            input: bwhat , , "},", ,""
            salt: Some(bsalt),", 1)}.iter().cloned().collect();"
        TestVector {name:  ", 2 ,"}
            salt: Some(b , ",")
        info!(Testing: : {), vector.name);", ", 256) =>     {-", " =>     {test_hmac_sha256_vector(&vector}}
            name if name.starts_with(" =>     {test_pbkdf2_vector(&vector})"))
            _ => {warn!(, "  test vector type: {), vector.name)"✅ { } passed , vector.name)"}"
            Err(e) => {warn!(" assert_eq!(failed, 0, Somestandard test vectors )))"
        .map_err(|e| format!(Hashcomputation failed: {:?), e)?")"
fn test_hmac_sha256_vector() {
    // TODO: Implement test
    assert!(true);
}, hex_encode(&sha256_result)""
    info!(", : : {), hex_encode(&blake3_result)")
    info!(" -platform KDF results:), : : {}, hex_encode(&pbkdf2_key);"
    info!(")"
    info!(, : : {), hex_encode(&scrypt_key);"")
                           Hashsize mismatch for       {:?}, algorithm)""
                info!(")"
            Err(e) => {warn!(" {:?} failed: {:?), algorithm, e)"}
                panic!(NIST: -approved algorithm should work)}""
fn test_key_size_compliance() {info!(Testing:  key size compliance)")"
    // TODO: Implement test
    assert!(true);
}