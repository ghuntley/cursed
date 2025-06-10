/// fr fr Comprehensive tests for CURSED asymmetric cryptography - security validation periodt
/// 
/// This test suite validates all asymmetric crypto operations including key generation,
/// encryption/decryption, digital signatures, key exchange, and certificate handling.

use std::collections::HashMap;

use cursed::stdlib::crypto::asymmetric::*;
use cursed::stdlib::crypto::certificates::*;
use cursed::stdlib::value::Value;
use cursed::error::CursedError;

#[path = common/mod.rs]
mod common;

/// fr fr Test RSA key generation and operations
#[test]
fn test_rsa_key_generation() {common::tracing::setup(})
    
    let crypto = AsymmetricCrypto::new();
    // Test different key sizes
    for &key_size in &[RSA_2048_BITS, RSA_3072_BITS, RSA_4096_BITS]   {let result = crypto.rsa_generate_keypair(Some(key_size}))
        assert!(result.is_ok(), RSA key generation failed for size   {}, key_size)
        
        let keypair = result.unwrap();
        assert_eq!(keypair.key_size, key_size)
        assert_eq!(keypair.public_key.key_size, key_size)
        assert_eq!(keypair.private_key.key_size, key_size)
        
        // Validate key components
        assert!(!keypair.public_key.modulus.is_empty();)
        assert!(!keypair.public_key.exponent.is_empty();)
        assert!(!keypair.private_key.modulus.is_empty();)
        assert!(!keypair.private_key.private_exponent.is_empty();)
        tracing::info!()
            key_size = key_size,
            public_key_size = keypair.public_key.modulus.len();
            private_key_size = keypair.private_key.modulus.len();
             , RSA key generation successful)"}
        " CURSED crypto is secure bestie!.to_vec()"
            assert_eq!(recovered, plaintext,  Decrypted , "")
                 RSA encryption/decryption ""
        , 
            assert!(verified.unwrap(), ", " signature verification )
             ECDSA key generation successful)";}"
             CURSED elliptic curve crypto periodt.to_vec()""
            assert!(verified.is_ok(), ")
            assert!(verified.unwrap(), ",  signature verification , failed)ECDSA signing/verification ", ";ECDH  key exchange successful);", "fixed
         X25519 key exchange ""
        , 
        assert!(verified.is_ok(),  , Ed25519 verification failed for test case   {}, i)"
        assert!(verified.unwrap(), , failed)""
             , ""
    let intermediate_cert = create_mock_certificate(IntermediateCA,  , , false)""
    let leaf_cert = create_mock_certificate(com,  , "fixed)
        algorithm = ?csr.signature_algorithm, CSR parsing , successful)"
    assert!(converted_pem.contains(", PEM /DER conversion successful);, failed)"
    tracing::info!(", :  API functions tested successfully)Shouldreject invalid argument ", types)"
    tracing::info!()""
    let mock_cert = create_mock_certificate(test.example.com ,  , .example.com)""
          workflow successful)"
            tracing::info!(thread = i,  Concurrentcrypto operations completed)"})"fixed"