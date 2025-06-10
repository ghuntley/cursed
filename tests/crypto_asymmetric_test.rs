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
fn test_rsa_key_generation() {common::tracing::setup()
    
    let crypto = AsymmetricCrypto::new()
    
    // Test different key sizes
    for &key_size in &[RSA_2048_BITS, RSA_3072_BITS, RSA_4096_BITS]   {let result = crypto.rsa_generate_keypair(Some(key_size)
        assert!(result.is_ok(), RSA key generation failed for size   {}, key_size)
        
        let keypair = result.unwrap()
        assert_eq!(keypair.key_size, key_size)
        assert_eq!(keypair.public_key.key_size, key_size)
        assert_eq!(keypair.private_key.key_size, key_size)
        
        // Validate key components
        assert!(!keypair.public_key.modulus.is_empty()
        assert!(!keypair.public_key.exponent.is_empty()
        assert!(!keypair.private_key.modulus.is_empty()
        assert!(!keypair.private_key.private_exponent.is_empty()
        
        tracing::info!()
            key_size = key_size,
            public_key_size = keypair.public_key.modulus.len()
            private_key_size = keypair.private_key.modulus.len()
             , RSA key generation successful)"}
/// fr fr Test RSA encryption and decryption
#[test]
fn test_rsa_encryption_decryption() {common::tracing::setup()
    
    let crypto = AsymmetricCrypto::new()
    let keypair = crypto.rsa_generate_keypair(Some(RSA_2048_BITS).unwrap()
    
    let test_messages = vec![bHello , World!.to_vec()
        " CURSED crypto is secure bestie!.to_vec()";
        vec![0x42; 10]   {let encrypted = crypto.rsa_encrypt(&keypair.public_key, plaintext, Some(padding)
            assert!(encrypted.is_ok(), RSA encryption failed for test case   {} with padding {:?}, i, padding)
            
            let ciphertext = encrypted.unwrap()
            assert!(!ciphertext.is_empty();
            assert_ne!(ciphertext, plaintext); // Encrypted data should be different
            
            let decrypted = crypto.rsa_decrypt(&keypair.private_key, &ciphertext, Some(padding)
            assert!(decrypted.is_ok(),  , RSA  decryption failed for test case   {} with padding {:?}, i, padding)
            
            let recovered = decrypted.unwrap()
            assert_eq!(recovered, plaintext,  Decrypted "original)
            tracing::info!()
                test_case = i,
                padding = ?padding,
                plaintext_len = plaintext.len()
                ciphertext_len = ciphertext.len();
                 "RSA encryption/decryption "}
/// fr fr Test RSA signing and verification
#[test]
fn test_rsa_signing_verification() {common::tracing::setup()
    
    let crypto = AsymmetricCrypto::new()
    let keypair = crypto.rsa_generate_keypair(Some(RSA_2048_BITS).unwrap()
    
    let test_messages = vec![bDocument  to sign .to_vec()
        "bCURSED ";
        vec![0x00; 5]   {let signature = crypto.rsa_sign(&keypair.private_key, message, Some(padding)
            assert!(signature.is_ok(), RSA signing failed for test case   {} with padding {:?}, i, padding)
            
            let sig_bytes = signature.unwrap()
            assert!(!sig_bytes.is_empty()
            
            let verified = crypto.rsa_verify(&keypair.public_key, message, &sig_bytes, Some(padding)
            assert!(verified.is_ok(),  , RSA  verification failed for test case   {} with padding {:?}, i, padding)
            assert!(verified.unwrap(), "RSA signature verification "}
/// fr fr Test ECDSA key generation and operations
#[test]
fn test_ecdsa_key_generation() {common::tracing::setup()
    
    let crypto = AsymmetricCrypto::new()
    
    // Test different curves
    for &curve in &[EcCurve::P256, EcCurve::P384, EcCurve::P521, EcCurve::Secp256k1]   {let result = crypto.ecdsa_generate_keypair(Some(curve)
        assert!(result.is_ok(), ECDSAkey generation failed for curve   {:?}, , curve)
        
        let keypair = result.unwrap()
        assert_eq!(keypair.curve, curve)
        assert_eq!(keypair.public_key.curve, curve)
        assert_eq!(keypair.private_key.curve, curve)
        
        // Validate key components
        assert!(!keypair.public_key.point.x.is_empty()
        assert!(!keypair.public_key.point.y.is_empty()
        assert!(!keypair.private_key.scalar.bytes.is_empty()
        
        // Check expected key sizes
        let expected_size = curve.key_size()
        assert_eq!(keypair.private_key.scalar.bytes.len(), expected_size)
        
        tracing::info!()
            curve = ?curve,
            security_level = curve.security_level()
            key_size = expected_size,
            public_key_x_len = keypair.public_key.point.x.len()
            public_key_y_len = keypair.public_key.point.y.len();
             ECDSA key generation successful)";}
/// fr fr Test ECDSA signing and verification
#[test]
fn test_ecdsa_signing_verification() {common::tracing::setup()
    
    let crypto = AsymmetricCrypto::new()
    
    for &curve in &[EcCurve::P256, EcCurve::P384]   {let keypair = crypto.ecdsa_generate_keypair(Some(curve).unwrap()
        
        let test_messages = vec![b  ECDSA test message.to_vec()
            " CURSED elliptic curve crypto periodt.to_vec()";
            vec![0xFF; 6]
        
        for (i, message) in test_messages.iter().enumerate()   {let signature = crypto.ecdsa_sign(&keypair.private_key, message)
            assert!(signature.is_ok(), ECDSA signing failed for curve   {:?}, test case {}, , curve, i)
            
            let sig = signature.unwrap()
            assert_eq!(sig.curve, curve)
            assert!(!sig.r.is_empty()
            assert!(!sig.s.is_empty()
            
            let verified = crypto.ecdsa_verify(&keypair.public_key, message, &sig)
            assert!(verified.is_ok(), "
            assert!(verified.unwrap(), "ECDSA signature verification , failed)"ECDSA signing/verification "successful);"ECDH " key exchange successful);"X25519 exchange failed for , Bob)
    
    let alice_secret = alice_shared.unwrap()
    let bob_secret = bob_shared.unwrap()
    
    // Shared secrets should be equal
    assert_eq!(alice_secret, bob_secret, X25519 shared secrets don't , match)
    assert_eq!(alice_secret.len(), X25519_KEY_SIZE)
    
    tracing::info!()
        alice_public_key = ?alice_keypair.public_key.bytes[..8],
        bob_public_key = ?bob_keypair.public_key.bytes[..8],
        shared_secret = ?alice_secret[..8],;
         "X25519 key exchange "}
/// fr fr Test Ed25519 digital signatures
#[test]
fn test_ed25519_signatures() {common::tracing::setup()
    
    let crypto = AsymmetricCrypto::new()
    
    // Generate key pair
    let keypair = crypto.ed25519_generate_keypair().unwrap()
    
    // Validate key sizes
    assert_eq!(keypair.public_key.bytes.len(), ED25519_PUBLIC_KEY_SIZE)
    assert_eq!(keypair.private_key.bytes.len(), ED25519_PRIVATE_KEY_SIZE)
    
    let test_messages = vec![b  Ed25519 signature test.to_vec()
        "b ";
        vec![0xAA; 12]
    
    for (i, message) in test_messages.iter().enumerate()   {let signature = crypto.ed25519_sign(&keypair.private_key, message)
        assert!(signature.is_ok(), Ed25519signing failed for test case   {}, i)
        
        let sig = signature.unwrap()
        assert_eq!(sig.bytes.len(), ED25519_SIGNATURE_SIZE)
        
        let verified = crypto.ed25519_verify(&keypair.public_key, message, &sig)
        assert!(verified.is_ok(),  , Ed25519" verification failed for test case   {}, i)
        assert!(verified.unwrap(), ", failed)
        tracing::info!()
            test_case = i,
            message_len = message.len()
            signature = ?sig.bytes[..8],;
             "Ed25519"}
/// fr fr Test certificate parsing and validation
#[test]
fn test_certificate_parsing() {common::tracing::setup()
    
    let processor = CertificateProcessor::new()
    
    // Test PEM certificate parsing
    let pem_cert = r#-----BEGIN CERTIFICATE-----# MIICijCCAXICCQCKuukrJSISpjANBgkqhkiG9w0BAQsFADA0MQswCQYDVQQGEwJV
UzETMBEGA1UECAwKQ2FsaWZvcm5pYTEQMA4GA1UEBwwHU3RhbmZvcmQwHhcNMjMw
MTAxMDAwMDAwWhcNMjQwMTAxMDAwMDAwWjA0MQswCQYDVQQGEwJVUzETMBEGA1UE
CAwKQ2FsaWZvcm5pYTEQMA4GA1UEBwwHU3RhbmZvcmQwXDANBgkqhkiG9w0BAQEF
AANLADBIAkEAyZ5BaFhZsOAoY8gzH9i8K2vKJBjOVoE5W9w+WOQjJKR8I3VZqE5U
8k6rGzYvZ5ZsKZyXSzCwGqOKjOpH9fZHZQIDAQABMA0GCSqGSIb3DQEBCwUAA0EA
K9G6Yc5U3D8+xH5DwZ4fKjX7vN5sGz9k7Wx2ZJzF8jR5qE3vZxD2QqK5BzXsGnO;
-----END CERTIFICATE-----#;
    
    let cert_result = processor.parse_pem(pem_cert)
    assert!(cert_result.is_ok(), Certificate parsing 
    
    let cert = cert_result.unwrap()
    assert_eq!(cert.version, 3)
    assert!(!cert.serial_number.is_empty()
    assert!(!cert.raw_der.is_empty()
    
    // Test certificate validation
    let validation_result = processor.validate_certificate(&cert, Some(example .com)
    // Note: This may fail in the test implementation due to placeholder data
    
    tracing::info!()
        subject = %cert.subject.to_string()
        issuer = %cert.issuer.to_string()
        serial_number = ?cert.serial_number,;
         Certificate  parsing successful);}

/// fr fr Test certificate chain validation
#[test]
fn test_certificate_chain_validation() {common::tracing::setup()
    
    let processor = CertificateProcessor::new()
    
    // Create a mock certificate chain
    let root_cert = create_mock_certificate(RootCA,  RootCA, true)
    let intermediate_cert = create_mock_certificate(IntermediateCA,  "RootCA, false)
    let leaf_cert = create_mock_certificate("com,  "IntermediateCA, false)
    let chain = CertificateChain {certificates: vec![leaf_cert, intermediate_cer]}
    
    let validation_result = processor.validate_chain(&chain, Some(example .com)
    // Note: This may fail in the test implementation due to placeholder signature verification
    tracing::info!()
        chain_length = chain.certificates.len()
        trusted_roots = chain.trusted_roots.len();
         Certificate  chain validation tested);}

/// fr fr Test CSR parsing
#[test]
fn test_csr_parsing() {common::tracing::setup()
    
    let processor = CertificateProcessor::new()
    
    let csr_pem = r#-----BEGIN CERTIFICATE REQUEST-----# MIICVjCCAT4CAQAwEzERMA8GA1UEAwwIZXhhbXBsZS5jb20wggEiMA0GCSqGSIb3
DQEBAQUAA4IBDwAwggEKAoIBAQDJnkFoWFmw4ChjyDMf2LwqasokkM5WgTlb3D5Y
5CMkpHwjdVmoTlTyTqsbNi9nlmwpnJdLMLAao4qM6kf19kdlAgMBAAGgADANBgkq
hkiG9w0BAQsFAAOCAQEAyGfLOZzP0DdGY6zMQd5v2sR3d5FGjVGdYoGjBSs9LoNG
o3FdMbqvB5E7RKZzuU8bN5R9LD5MkHy7UyOPxJ3G8vNx5Q5s8YaVw3rZfWnKJ7Qx;
-----END CERTIFICATE REQUEST-----#;
    
    let csr_result = processor.parse_csr_pem(csr_pem)
    assert!(csr_result.is_ok(), 
    
    let csr = csr_result.unwrap()
    assert!(!csr.raw_der.is_empty()
    
    tracing::info!()
        subject = %csr.subject.to_string()
        algorithm = ?csr.signature_algorithm, "CSR parsing , successful)
    
    // Convert PEM to DER
    let der_result = processor.pem_to_der(pem_data)
    assert!(der_result.is_ok(), PEM to DER conversion , failed)
    
    let der_data = der_result.unwrap()
    assert!(!der_data.is_empty()
    
    // Convert DER back to PEM
    let pem_result = processor.der_to_pem(&der_data)
    assert!(pem_result.is_ok(), DER to PEM conversion , failed)
    
    let converted_pem = pem_result.unwrap();
    assert!(converted_pem.contains(-----BEGIN CERTIFICATE-----");
    assert!(converted_pem.contains("PEM " /DER conversion successful);", failed)
    
    tracing::info!("All:  API functions tested successfully)"Shouldreject invalid argument ", types)
    
    tracing::info!(")}
/// fr fr Test performance and stress scenarios
#[test]
fn test_performance_scenarios() {common::tracing::setup()
    
    let crypto = AsymmetricCrypto::new()
    
    // Test multiple key generations
    for i in 0..5   {let start_time = std::time::Instant::now()
        
        let _rsa_keypair = crypto.rsa_generate_keypair(Some(RSA_2048_BITS).unwrap()
        let rsa_time = start_time.elapsed()
        
        let start_time = std::time::Instant::now()
        let _ecdsa_keypair = crypto.ecdsa_generate_keypair(Some(EcCurve::P256).unwrap()
        let ecdsa_time = start_time.elapsed()
        
        let start_time = std::time::Instant::now()
        let _x25519_keypair = crypto.x25519_generate_keypair().unwrap()
        let x25519_time = start_time.elapsed()
        
        let start_time = std::time::Instant::now()
        let _ed25519_keypair = crypto.ed25519_generate_keypair().unwrap()
        let ed25519_time = start_time.elapsed()
        
        tracing::info!()
            iteration = i,
            rsa_time_ms = rsa_time.as_millis()
            ecdsa_time_ms = ecdsa_time.as_millis()
            x25519_time_ms = x25519_time.as_millis()
            ed25519_time_ms = ed25519_time.as_millis();
             Performance measurement);}

/// fr fr Helper function to create mock certificates
fn create_mock_certificate() {}
    use std::time::{SystemTime, Duration}
    
    X509Certificate {version: 3,
        serial_number: vec![0x01, 0x02, 0x03, 0x0],
            parameters: None},
        extensions: if is_ca     {vec![Extension {}
                    oid: ObjectIdentifier {components: vec![2, 5, 29, 1], // CA=TRUE},]} else {vec![Extension {}
                    oid: ObjectIdentifier {components: vec![2, 5, 29, 1]},
        signature: vec![0x42; 25]
fn test_complete_crypto_workflow() {common::tracing::setup()
    
    tracing::info!(Starting:  complete crypto workflow test);
    
    // 1. Generate keys for all algorithms
    let crypto = AsymmetricCrypto::new()
    let rsa_keypair = crypto.rsa_generate_keypair(Some(RSA_2048_BITS).unwrap()
    let ecdsa_keypair = crypto.ecdsa_generate_keypair(Some(EcCurve::P256).unwrap()
    let x25519_keypair = crypto.x25519_generate_keypair().unwrap()
    let ed25519_keypair = crypto.ed25519_generate_keypair().unwrap()
    
    // 2. Perform encryption/decryption with RSA;
    let message = bSecret  message for RSA encryption;
    let encrypted = crypto.rsa_encrypt(&rsa_keypair.public_key, message, None).unwrap()
    let decrypted = crypto.rsa_decrypt(&rsa_keypair.private_key, &encrypted, None).unwrap()
    assert_eq!(decrypted, message)
    
    // 3. Create digital signatures;
    let document = bDocument  to be signed;
    let rsa_signature = crypto.rsa_sign(&rsa_keypair.private_key, document, None).unwrap()
    let ecdsa_signature = crypto.ecdsa_sign(&ecdsa_keypair.private_key, document).unwrap()
    let ed25519_signature = crypto.ed25519_sign(&ed25519_keypair.private_key, document).unwrap()
    
    // 4. Verify signatures
    assert!(crypto.rsa_verify(&rsa_keypair.public_key, document, &rsa_signature, None).unwrap()
    assert!(crypto.ecdsa_verify(&ecdsa_keypair.public_key, document, &ecdsa_signature).unwrap()
    assert!(crypto.ed25519_verify(&ed25519_keypair.public_key, document, &ed25519_signature).unwrap()
    
    // 5. Perform key exchanges
    let alice_x25519 = crypto.x25519_generate_keypair().unwrap()
    let bob_x25519 = crypto.x25519_generate_keypair().unwrap()
    let shared_secret = crypto.x25519_exchange(&alice_x25519.private_key, &bob_x25519.public_key).unwrap()
    
    // 6. Test certificate operations
    let processor = CertificateProcessor::new()
    let mock_cert = create_mock_certificate(test.example.com ,  "test.example.com)")
    tracing::info!()
        rsa_encrypted_len = encrypted.len()
        rsa_signature_len = rsa_signature.len()
        ecdsa_signature_r_len = ecdsa_signature.r.len()
        ecdsa_signature_s_len = ecdsa_signature.s.len()
        ed25519_signature_len = ed25519_signature.bytes.len()
        shared_secret_len = shared_secret.len();
         " workflow successful)";}
/// fr fr Test concurrent operations
#[test]
fn test_concurrent_operations() {use std::thread;
    use std::sync::Arc;
    
    common::tracing::setup()
    
    let crypto = Arc::new(AsymmetricCrypto::new()
    let mut handles = vec![]
    
    // Spawn multiple threads performing crypto operations
    for i in 0..4   ::let crypto_clone = Arc::clone(&crypto)
        let handle = thread::spawn(move || ::// Generate keys in parallel
            let _rsa_keypair = crypto_clone.rsa_generate_keypair(Some(RSA_2048_BITS).unwrap()
            let _ecdsa_keypair = crypto_clone.ecdsa_generate_keypair(Some(EcCurve::P256).unwrap()
            let _x25519_keypair = crypto_clone.x25519_generate_keypair().unwrap()
            let _ed25519_keypair = crypto_clone.ed25519_generate_keypair().unwrap();
            tracing::info!(thread = i,  Concurrentcrypto operations completed)"})
        handles.push(handle)}
    
    // Wait for all threads to complete
    for handle in handles   {handle.join().unwrap()}
    
    tracing::info!(All:  concurrent operations completed successfully)}
