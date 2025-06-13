/// fr fr Comprehensive Digital Signatures Test Suite
/// 
/// Tests all digital signature algorithms, key management, multi-signatures,
/// and verification functionality for the CURSED crypto package.

use cursed::stdlib::packages::crypto_signatures::*;

#[test]
fn test_ed25519_signature_generation_and_verification() {
    // Test Ed25519 key generation
    let mut generator = KeyGenerator::new();
    let keypair = generator.generate_keypair(KeyType::Ed25519)
        .expect("Failed to generate Ed25519 keypair");
    
    // Test signer creation
    let signer = Ed25519Signer::new(keypair.clone())
        .expect("Failed to create Ed25519 signer");
    
    // Test message signing
    let message = b"Hello, CURSED crypto signatures!";
    let signature = signer.sign(message)
        .expect("Failed to sign message");
    
    assert_eq!(signature.len(), ED25519_SIGNATURE_SIZE);
    
    // Test signature verification with signer
    let is_valid = signer.verify(message, &signature)
        .expect("Failed to verify signature");
    assert!(is_valid, "Signature should be valid");
    
    // Test signature verification with separate verifier
    let public_key = PublicKey::from_keypair(&keypair);
    let verifier = Ed25519Verifier::new(public_key)
        .expect("Failed to create Ed25519 verifier");
    
    let is_valid = verifier.verify(message, &signature)
        .expect("Failed to verify signature with verifier");
    assert!(is_valid, "Signature should be valid with verifier");
    
    // Test invalid signature
    let mut invalid_signature = signature.clone();
    invalid_signature[0] ^= 1; // Flip a bit
    
    let is_valid = verifier.verify(message, &invalid_signature)
        .expect("Failed to verify invalid signature");
    assert!(!is_valid, "Invalid signature should not verify");
}

#[test]
fn test_ecdsa_signature_algorithms() {
    for curve in [EcdsaCurve::Secp256k1, EcdsaCurve::Secp256r1] {
        let mut generator = KeyGenerator::new();
        let key_type = match curve {
            EcdsaCurve::Secp256k1 => KeyType::EcdsaSecp256k1,
            EcdsaCurve::Secp256r1 => KeyType::EcdsaSecp256r1,
        };
        
        let keypair = generator.generate_keypair(key_type)
            .expect("Failed to generate ECDSA keypair");
        
        let mut signer = EcdsaSigner::new(keypair.clone())
            .expect("Failed to create ECDSA signer");
        
        let message = b"ECDSA test message";
        let signature = signer.sign(message)
            .expect("Failed to sign message with ECDSA");
        
        assert_eq!(signature.len(), ECDSA_SIGNATURE_SIZE);
        
        let is_valid = signer.verify(message, &signature)
            .expect("Failed to verify ECDSA signature");
        assert!(is_valid, "ECDSA signature should be valid for curve {:?}", curve);
        
        // Test with separate verifier
        let public_key = PublicKey::from_keypair(&keypair);
        let verifier = EcdsaVerifier::new(public_key)
            .expect("Failed to create ECDSA verifier");
        
        let is_valid = verifier.verify(message, &signature)
            .expect("Failed to verify ECDSA signature with verifier");
        assert!(is_valid, "ECDSA signature should be valid with verifier for curve {:?}", curve);
    }
}

#[test]
fn test_rsa_signature_schemes() {
    for scheme in [RsaSignatureScheme::Pss, RsaSignatureScheme::Pkcs1v15] {
        for key_size in [RsaKeySize::Bits2048, RsaKeySize::Bits3072] {
            let mut generator = KeyGenerator::new();
            let key_type = match (&scheme, &key_size) {
                (RsaSignatureScheme::Pss, RsaKeySize::Bits2048) => KeyType::RsaPss2048,
                (RsaSignatureScheme::Pss, RsaKeySize::Bits3072) => KeyType::RsaPss3072,
                (RsaSignatureScheme::Pkcs1v15, RsaKeySize::Bits2048) => KeyType::RsaPkcs1v15_2048,
                (RsaSignatureScheme::Pkcs1v15, RsaKeySize::Bits3072) => KeyType::RsaPkcs1v15_3072,
                _ => continue,
            };
            
            let keypair = generator.generate_keypair(key_type)
                .expect("Failed to generate RSA keypair");
            
            let signer = RsaSigner::new(keypair.clone(), scheme.clone(), RsaHashAlgorithm::Sha256)
                .expect("Failed to create RSA signer");
            
            let message = b"RSA test message";
            let signature = signer.sign(message)
                .expect("Failed to sign message with RSA");
            
            assert_eq!(signature.len(), key_size.signature_size());
            
            let is_valid = signer.verify(message, &signature)
                .expect("Failed to verify RSA signature");
            assert!(is_valid, "RSA signature should be valid for scheme {:?} and key size {:?}", scheme, key_size);
            
            // Test with separate verifier
            let public_key = PublicKey::from_keypair(&keypair);
            let verifier = RsaVerifier::new(public_key, scheme.clone(), RsaHashAlgorithm::Sha256)
                .expect("Failed to create RSA verifier");
            
            let is_valid = verifier.verify(message, &signature)
                .expect("Failed to verify RSA signature with verifier");
            assert!(is_valid, "RSA signature should be valid with verifier for scheme {:?} and key size {:?}", scheme, key_size);
        }
    }
}

#[test]
fn test_universal_signer_and_verifier() {
    let mut generator = KeyGenerator::new();
    
    // Test Ed25519 through universal interface
    let ed25519_keypair = generator.generate_keypair(KeyType::Ed25519)
        .expect("Failed to generate Ed25519 keypair");
    
    let universal_signer = UniversalSigner::new(ed25519_keypair.clone())
        .expect("Failed to create universal signer");
    
    let message = b"Universal signature test";
    let signature = universal_signer.sign(message)
        .expect("Failed to sign with universal signer");
    
    let is_valid = universal_signer.verify(message, &signature)
        .expect("Failed to verify with universal signer");
    assert!(is_valid, "Universal signer verification should succeed");
    
    // Test universal verifier
    let public_key = PublicKey::from_keypair(&ed25519_keypair);
    let universal_verifier = UniversalVerifier::new(public_key)
        .expect("Failed to create universal verifier");
    
    let is_valid = universal_verifier.verify(message, &signature)
        .expect("Failed to verify with universal verifier");
    assert!(is_valid, "Universal verifier should succeed");
    
    assert_eq!(universal_verifier.algorithm(), "Ed25519");
}

#[test]
fn test_batch_verification() {
    let mut batch_verifier = BatchVerifier::new();
    let mut generator = KeyGenerator::new();
    
    // Generate multiple signatures
    for i in 0..5 {
        let keypair = generator.generate_keypair(KeyType::Ed25519)
            .expect("Failed to generate keypair");
        
        let signer = Ed25519Signer::new(keypair.clone())
            .expect("Failed to create signer");
        
        let message = format!("Batch test message {}", i);
        let signature = signer.sign(message.as_bytes())
            .expect("Failed to sign message");
        
        let public_key = PublicKey::from_keypair(&keypair);
        batch_verifier.add_verification(
            public_key,
            message.as_bytes(),
            &signature,
            Some(format!("req-{}", i))
        );
    }
    
    assert_eq!(batch_verifier.pending_count(), 5);
    
    let results = batch_verifier.verify_batch()
        .expect("Failed to verify batch");
    
    assert_eq!(results.len(), 5);
    
    for result in &results {
        assert!(result.is_valid, "All batch signatures should be valid");
        assert_eq!(result.algorithm, "Ed25519");
        assert!(result.error.is_none(), "No errors should occur");
    }
    
    let stats = batch_verifier.get_stats();
    assert_eq!(stats.total_requests, 5);
    assert_eq!(stats.successful_verifications, 5);
    assert_eq!(stats.failed_verifications, 0);
}

#[test]
fn test_multi_signature_threshold() {
    // Create 2-of-3 threshold multi-signature
    let config = MultiSigConfig::new(2, 3, MultiSigScheme::Threshold, MultiSigAlgorithm::Ed25519)
        .expect("Failed to create multisig config");
    
    let mut multisig_signer = MultiSigSigner::new(config.clone())
        .expect("Failed to create multisig signer");
    
    // Generate keypairs for 3 signers
    let mut generator = KeyGenerator::new();
    let mut keypairs = Vec::new();
    
    for i in 0..3 {
        let keypair = generator.generate_keypair(KeyType::Ed25519)
            .expect("Failed to generate keypair");
        
        let public_key = PublicKey::from_keypair(&keypair);
        let signer_id = format!("signer-{}", i + 1);
        
        multisig_signer.add_signer(signer_id, public_key)
            .expect("Failed to add signer");
        
        keypairs.push(keypair);
    }
    
    let message = b"Multi-signature test message";
    let mut multisig = multisig_signer.create_multisig(message)
        .expect("Failed to create multisig");
    
    assert!(!multisig.is_complete(), "Multisig should not be complete initially");
    assert_eq!(multisig.remaining_needed(), 2);
    
    // Add first signature
    multisig_signer.sign_with_keypair(&mut multisig, "signer-1", &keypairs[0], message)
        .expect("Failed to add first signature");
    
    assert!(!multisig.is_complete(), "Multisig should not be complete with 1 signature");
    assert_eq!(multisig.remaining_needed(), 1);
    assert_eq!(multisig.completion_percentage(), 50.0);
    
    // Add second signature (threshold reached)
    multisig_signer.sign_with_keypair(&mut multisig, "signer-2", &keypairs[1], message)
        .expect("Failed to add second signature");
    
    assert!(multisig.is_complete(), "Multisig should be complete with 2 signatures");
    assert_eq!(multisig.remaining_needed(), 0);
    assert_eq!(multisig.completion_percentage(), 100.0);
    
    // Verify the multi-signature
    let is_valid = multisig_signer.verify_multisig(&multisig, message)
        .expect("Failed to verify multisig");
    assert!(is_valid, "Multi-signature should be valid");
    
    // Test that we can get the final signature
    let final_signature = multisig.get_signature()
        .expect("Failed to get final signature");
    assert!(!final_signature.is_empty(), "Final signature should not be empty");
}

#[test]
fn test_key_management() {
    let key_manager = KeyManager::new();
    let mut generator = KeyGenerator::new();
    
    // Generate and store multiple key pairs
    let key_types = [
        KeyType::Ed25519,
        KeyType::EcdsaSecp256k1,
        KeyType::RsaPss2048,
    ];
    
    let mut key_ids = Vec::new();
    
    for key_type in key_types {
        let key_id = key_manager.generate_and_store(key_type, None)
            .expect("Failed to generate and store key");
        key_ids.push(key_id);
    }
    
    assert_eq!(key_manager.key_count(), 3);
    
    // Retrieve and validate keys
    for key_id in &key_ids {
        let keypair = key_manager.get_keypair(key_id)
            .expect("Failed to retrieve keypair");
        
        keypair.validate()
            .expect("Retrieved keypair should be valid");
    }
    
    // List all keys
    let all_keys = key_manager.list_keys()
        .expect("Failed to list keys");
    assert_eq!(all_keys.len(), 3);
    
    // Remove a key
    let removed = key_manager.remove_key(&key_ids[0])
        .expect("Failed to remove key");
    assert!(removed, "Key should have been removed");
    assert_eq!(key_manager.key_count(), 2);
}

#[test]
fn test_signature_manager() {
    let signature_manager = SignatureManager::new();
    let mut generator = KeyGenerator::new();
    
    // Add Ed25519 signer
    let ed25519_keypair = generator.generate_keypair(KeyType::Ed25519)
        .expect("Failed to generate Ed25519 keypair");
    
    signature_manager.add_signer_from_keypair("ed25519-signer".to_string(), ed25519_keypair)
        .expect("Failed to add Ed25519 signer");
    
    // Add RSA signer
    let rsa_keypair = generator.generate_keypair(KeyType::RsaPss2048)
        .expect("Failed to generate RSA keypair");
    
    signature_manager.add_signer_from_keypair("rsa-signer".to_string(), rsa_keypair)
        .expect("Failed to add RSA signer");
    
    let signers = signature_manager.list_signers()
        .expect("Failed to list signers");
    assert_eq!(signers.len(), 2);
    
    // Test signing with Ed25519 signer
    let message = b"Signature manager test";
    let signature = signature_manager.sign_with("ed25519-signer", message)
        .expect("Failed to sign with Ed25519 signer");
    
    let is_valid = signature_manager.verify_with("ed25519-signer", message, &signature)
        .expect("Failed to verify signature");
    assert!(is_valid, "Signature should be valid");
    
    // Check statistics
    let stats = signature_manager.get_stats();
    assert_eq!(stats.total_signatures, 1);
    assert_eq!(stats.total_verifications, 1);
    assert_eq!(stats.successful_verifications, 1);
}

#[test]
fn test_algorithm_registry() {
    // Test listing algorithms
    let algorithms = list_algorithms();
    assert!(!algorithms.is_empty(), "Should have registered algorithms");
    
    // Test getting algorithm info
    let ed25519_info = get_algorithm_info("Ed25519");
    assert!(ed25519_info.is_some(), "Ed25519 should be registered");
    
    if let Some(info) = ed25519_info {
        assert_eq!(info.name, "Ed25519");
        assert_eq!(info.signature_size, 64);
        assert!(!info.is_quantum_resistant);
        assert_eq!(info.performance_tier, PerformanceTier::Fast);
        assert_eq!(info.security_level, SecurityLevel::Standard);
    }
    
    // Test utility functions
    assert!(utils::is_algorithm_supported("Ed25519"));
    assert!(utils::is_algorithm_supported("ECDSA-secp256k1"));
    assert!(!utils::is_algorithm_supported("NonExistent"));
    
    assert_eq!(utils::get_recommended_algorithm("speed"), "Ed25519");
    assert_eq!(utils::get_recommended_algorithm("bitcoin"), "ECDSA-secp256k1");
    assert_eq!(utils::get_recommended_algorithm("nist"), "ECDSA-secp256r1");
}

#[test]
fn test_signature_validation() {
    let mut generator = KeyGenerator::new();
    
    // Test signature size validation
    let keypair = generator.generate_keypair(KeyType::Ed25519)
        .expect("Failed to generate keypair");
    
    let signer = Ed25519Signer::new(keypair)
        .expect("Failed to create signer");
    
    let message = b"Test message";
    let signature = signer.sign(message)
        .expect("Failed to sign message");
    
    // Test with correct signature
    let is_valid = signer.verify(message, &signature)
        .expect("Failed to verify signature");
    assert!(is_valid, "Valid signature should verify");
    
    // Test with wrong signature size
    let wrong_size_signature = vec![0u8; 32]; // Wrong size
    let result = signer.verify(message, &wrong_size_signature);
    assert!(result.is_err(), "Wrong size signature should fail");
    
    // Test with empty message
    let result = signer.sign(&[]);
    assert!(result.is_err(), "Empty message should fail");
}

#[test]
fn test_error_handling() {
    // Test invalid key size
    let result = KeyPair::new(
        KeyType::Ed25519,
        vec![0u8; 16], // Wrong size
        vec![0u8; 32],
        None,
    );
    assert!(result.is_err(), "Invalid key size should fail");
    
    // Test invalid multi-sig config
    let result = MultiSigConfig::new(3, 2, MultiSigScheme::Threshold, MultiSigAlgorithm::Ed25519);
    assert!(result.is_err(), "Invalid threshold config should fail");
    
    // Test zero threshold
    let result = MultiSigConfig::new(0, 3, MultiSigScheme::Threshold, MultiSigAlgorithm::Ed25519);
    assert!(result.is_err(), "Zero threshold should fail");
}

#[test]
fn test_utils_quick_functions() {
    // Test quick Ed25519 sign and verify
    let message = b"Quick test message";
    let result = utils::quick_ed25519_sign_verify(message)
        .expect("Quick Ed25519 test should succeed");
    assert!(result, "Quick Ed25519 verification should succeed");
    
    // Test quick ECDSA sign and verify
    let result = utils::quick_ecdsa_sign_verify(message, EcdsaCurve::Secp256k1)
        .expect("Quick ECDSA test should succeed");
    assert!(result, "Quick ECDSA verification should succeed");
    
    // Test quick RSA sign and verify
    let result = utils::quick_rsa_sign_verify(message, RsaKeySize::Bits2048, RsaSignatureScheme::Pss)
        .expect("Quick RSA test should succeed");
    assert!(result, "Quick RSA verification should succeed");
}

#[test]
fn test_package_initialization() {
    // Test package initialization
    let result = init_crypto_signatures();
    assert!(result.is_ok(), "Package initialization should succeed");
    
    // Test that global stats are available after initialization
    let stats = get_global_stats();
    assert!(stats.is_some(), "Global stats should be available");
}
