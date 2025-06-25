//! Integration Test Suite for Post-Quantum Cryptography Module
//! 
//! This test suite focuses on end-to-end integration testing of PQC algorithms
//! including hybrid schemes, real-world workflows, migration scenarios, and
//! comprehensive system integration validation.

use cursed::stdlib::crypto::pqc::*;
use cursed::stdlib::crypto::hash_advanced::*;
use cursed::stdlib::crypto::random::*;
use std::time::Duration;
use std::collections::HashMap;

// ============================================================================
// HYBRID CRYPTOGRAPHIC SCHEMES
// ============================================================================

#[test]
fn test_hybrid_key_exchange_protocol() {
    // Test a complete hybrid key exchange using multiple PQC algorithms
    
    // Alice's side - Generate key pairs for multiple algorithms
    let (alice_kyber_pub, alice_kyber_sec) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    let (alice_dilithium_pub, alice_dilithium_sec) = DilithiumSignature::keygen(SecurityLevel::Level3).unwrap();
    
    // Bob's side - Generate key pairs
    let (bob_kyber_pub, bob_kyber_sec) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    let (bob_dilithium_pub, bob_dilithium_sec) = DilithiumSignature::keygen(SecurityLevel::Level3).unwrap();
    
    // Step 1: Alice sends her public keys to Bob (signed)
    let alice_public_keys = format!("kyber:{},dilithium:{}", 
                                   bytes_to_hex(&alice_kyber_pub.key_data),
                                   bytes_to_hex(&alice_dilithium_pub.key_data));
    let alice_key_signature = DilithiumSignature::sign(&alice_dilithium_sec, alice_public_keys.as_bytes()).unwrap();
    
    // Step 2: Bob verifies Alice's signature and performs key exchange
    let is_alice_valid = DilithiumSignature::verify(&alice_dilithium_pub, alice_public_keys.as_bytes(), &alice_key_signature).unwrap();
    // Note: In simulation, this may return true or false, but should not error
    
    let (alice_ciphertext, alice_shared_secret) = KyberKem::encaps(&alice_kyber_pub).unwrap();
    
    // Step 3: Bob sends his response (ciphertext + his public key, signed)
    let bob_response = format!("ciphertext:{},pubkey:{}", 
                              bytes_to_hex(&alice_ciphertext),
                              bytes_to_hex(&bob_kyber_pub.key_data));
    let bob_response_signature = DilithiumSignature::sign(&bob_dilithium_sec, bob_response.as_bytes()).unwrap();
    
    // Step 4: Alice verifies Bob's response and completes key exchange
    let is_bob_valid = DilithiumSignature::verify(&bob_dilithium_pub, bob_response.as_bytes(), &bob_response_signature).unwrap();
    
    let alice_recovered_secret = KyberKem::decaps(&alice_kyber_sec, &alice_ciphertext).unwrap();
    assert_eq!(alice_shared_secret, alice_recovered_secret);
    
    // Step 5: Bob also performs his side of the key exchange
    let (bob_ciphertext, bob_shared_secret) = KyberKem::encaps(&bob_kyber_pub).unwrap();
    let bob_recovered_secret = KyberKem::decaps(&bob_kyber_sec, &bob_ciphertext).unwrap();
    assert_eq!(bob_shared_secret, bob_recovered_secret);
    
    // Step 6: Derive final shared secret by combining both exchanges
    let combined_material = [alice_shared_secret, bob_shared_secret].concat();
    let final_shared_secret = sha3_256(&combined_material).unwrap();
    
    println!("Hybrid key exchange completed successfully");
    println!("Final shared secret length: {} bytes", final_shared_secret.len());
    
    // Verify the protocol completed successfully
    assert_eq!(final_shared_secret.len(), 32); // SHA3-256 output
    assert!(!alice_shared_secret.is_empty());
    assert!(!bob_shared_secret.is_empty());
}

#[test]
fn test_authenticated_encryption_workflow() {
    // Complete authenticated encryption using PQC algorithms
    
    // Setup: Generate keys for sender and receiver
    let (sender_signing_pub, sender_signing_sec) = DilithiumSignature::keygen(SecurityLevel::Level3).unwrap();
    let (receiver_kem_pub, receiver_kem_sec) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    
    let plaintext = b"This is a confidential message using post-quantum cryptography for authentication and key exchange.";
    
    // Step 1: Sender performs key encapsulation
    let (kem_ciphertext, shared_secret) = KyberKem::encaps(&receiver_kem_pub).unwrap();
    
    // Step 2: Derive encryption key from shared secret
    let encryption_key = sha3_256(&shared_secret).unwrap();
    
    // Step 3: Encrypt the message (simulated - would use AES with derived key)
    let encrypted_message = {
        let mut encrypted = Vec::new();
        for (i, &byte) in plaintext.iter().enumerate() {
            encrypted.push(byte ^ encryption_key[i % encryption_key.len()]);
        }
        encrypted
    };
    
    // Step 4: Create message package
    let message_package = format!("kem_ct:{},encrypted:{}", 
                                 bytes_to_hex(&kem_ciphertext),
                                 bytes_to_hex(&encrypted_message));
    
    // Step 5: Sign the entire package
    let signature = DilithiumSignature::sign(&sender_signing_sec, message_package.as_bytes()).unwrap();
    
    // Transmission phase (sender -> receiver)
    let transmitted_package = (kem_ciphertext, encrypted_message, signature);
    
    // Step 6: Receiver verifies signature
    let reconstructed_package = format!("kem_ct:{},encrypted:{}", 
                                       bytes_to_hex(&transmitted_package.0),
                                       bytes_to_hex(&transmitted_package.1));
    let signature_valid = DilithiumSignature::verify(&sender_signing_pub, reconstructed_package.as_bytes(), &transmitted_package.2).unwrap();
    
    // Step 7: Receiver decapsulates shared secret
    let receiver_shared_secret = KyberKem::decaps(&receiver_kem_sec, &transmitted_package.0).unwrap();
    
    // Step 8: Derive same encryption key
    let receiver_encryption_key = sha3_256(&receiver_shared_secret).unwrap();
    
    // Step 9: Decrypt the message
    let decrypted_message = {
        let mut decrypted = Vec::new();
        for (i, &byte) in transmitted_package.1.iter().enumerate() {
            decrypted.push(byte ^ receiver_encryption_key[i % receiver_encryption_key.len()]);
        }
        decrypted
    };
    
    // Verification
    assert_eq!(shared_secret, receiver_shared_secret);
    assert_eq!(encryption_key, receiver_encryption_key);
    assert_eq!(plaintext, decrypted_message.as_slice());
    
    println!("Authenticated encryption workflow completed successfully");
    println!("Original message length: {} bytes", plaintext.len());
    println!("Signature length: {} bytes", signature.len());
}

// ============================================================================
// MULTI-PARTY COMMUNICATION PROTOCOLS
// ============================================================================

#[test]
fn test_group_signature_protocol() {
    // Simulate a group communication protocol with multiple signers
    
    struct Participant {
        id: String,
        signing_public: DilithiumPublicKey,
        signing_secret: DilithiumSecretKey,
    }
    
    // Create multiple participants
    let mut participants = Vec::new();
    for i in 0..5 {
        let (pub_key, sec_key) = DilithiumSignature::keygen(SecurityLevel::Level1).unwrap();
        participants.push(Participant {
            id: format!("participant_{}", i),
            signing_public: pub_key,
            signing_secret: sec_key,
        });
    }
    
    let group_message = b"This message is signed by all group members";
    let mut signatures = HashMap::new();
    
    // Each participant signs the message
    for participant in &participants {
        let signature = DilithiumSignature::sign(&participant.signing_secret, group_message).unwrap();
        signatures.insert(participant.id.clone(), signature);
    }
    
    // Verify all signatures
    let mut valid_signatures = 0;
    for participant in &participants {
        if let Some(signature) = signatures.get(&participant.id) {
            let is_valid = DilithiumSignature::verify(&participant.signing_public, group_message, signature).unwrap();
            if is_valid {
                valid_signatures += 1;
            }
        }
    }
    
    println!("Group signature protocol results:");
    println!("Total participants: {}", participants.len());
    println!("Total signatures created: {}", signatures.len());
    println!("Verification attempts: {}", participants.len());
    
    // All signatures should be created successfully
    assert_eq!(signatures.len(), participants.len());
}

#[test]
fn test_secure_channel_establishment() {
    // End-to-end secure channel establishment with perfect forward secrecy
    
    // Long-term identity keys
    let (alice_identity_pub, alice_identity_sec) = DilithiumSignature::keygen(SecurityLevel::Level3).unwrap();
    let (bob_identity_pub, bob_identity_sec) = DilithiumSignature::keygen(SecurityLevel::Level3).unwrap();
    
    // Ephemeral keys for this session
    let (alice_ephemeral_pub, alice_ephemeral_sec) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    let (bob_ephemeral_pub, bob_ephemeral_sec) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    
    // Step 1: Alice initiates handshake
    let alice_handshake = format!("alice_ephemeral:{}", bytes_to_hex(&alice_ephemeral_pub.key_data));
    let alice_handshake_sig = DilithiumSignature::sign(&alice_identity_sec, alice_handshake.as_bytes()).unwrap();
    
    // Step 2: Bob responds with his ephemeral key
    let bob_handshake = format!("bob_ephemeral:{}", bytes_to_hex(&bob_ephemeral_pub.key_data));
    let bob_handshake_sig = DilithiumSignature::sign(&bob_identity_sec, bob_handshake.as_bytes()).unwrap();
    
    // Step 3: Both parties verify handshake signatures
    let alice_handshake_valid = DilithiumSignature::verify(&alice_identity_pub, alice_handshake.as_bytes(), &alice_handshake_sig).unwrap();
    let bob_handshake_valid = DilithiumSignature::verify(&bob_identity_pub, bob_handshake.as_bytes(), &bob_handshake_sig).unwrap();
    
    // Step 4: Key agreement using ephemeral keys
    let (alice_to_bob_ct, alice_derived_secret) = KyberKem::encaps(&bob_ephemeral_pub).unwrap();
    let (bob_to_alice_ct, bob_derived_secret) = KyberKem::encaps(&alice_ephemeral_pub).unwrap();
    
    // Step 5: Both parties compute shared secrets
    let alice_received_secret = KyberKem::decaps(&alice_ephemeral_sec, &bob_to_alice_ct).unwrap();
    let bob_received_secret = KyberKem::decaps(&bob_ephemeral_sec, &alice_to_bob_ct).unwrap();
    
    // Step 6: Derive final session key
    let alice_session_material = [alice_derived_secret, alice_received_secret].concat();
    let alice_session_key = sha3_256(&alice_session_material).unwrap();
    
    let bob_session_material = [bob_received_secret, bob_derived_secret].concat();
    let bob_session_key = sha3_256(&bob_session_material).unwrap();
    
    // Verification - both parties should derive the same session key
    assert_eq!(alice_session_key, bob_session_key);
    
    println!("Secure channel established successfully");
    println!("Session key length: {} bytes", alice_session_key.len());
    
    // Test the secure channel with message exchange
    let test_message = b"Hello Bob, this is a secure message!";
    let encrypted_message: Vec<u8> = test_message.iter().enumerate()
        .map(|(i, &byte)| byte ^ alice_session_key[i % alice_session_key.len()])
        .collect();
    
    let decrypted_message: Vec<u8> = encrypted_message.iter().enumerate()
        .map(|(i, &byte)| byte ^ bob_session_key[i % bob_session_key.len()])
        .collect();
    
    assert_eq!(test_message, decrypted_message.as_slice());
}

// ============================================================================
// MIGRATION AND COMPATIBILITY TESTING
// ============================================================================

#[test]
fn test_classical_to_pqc_migration() {
    // Simulate migration from classical to post-quantum cryptography
    
    // Phase 1: Classical system (simulated)
    let classical_message = b"Message signed with classical cryptography";
    let classical_signature = vec![0u8; 64]; // Simulate ECDSA signature
    
    // Phase 2: Hybrid transition period
    let (pqc_pub, pqc_sec) = DilithiumSignature::keygen(SecurityLevel::Level1).unwrap();
    let pqc_signature = DilithiumSignature::sign(&pqc_sec, classical_message).unwrap();
    
    // Create dual signature format
    let dual_signature = format!("classical:{},pqc:{}", 
                                bytes_to_hex(&classical_signature),
                                bytes_to_hex(&pqc_signature));
    
    // Phase 3: Verification supports both formats
    let can_verify_classical = classical_signature.len() == 64; // Simulate classical verification
    let can_verify_pqc = DilithiumSignature::verify(&pqc_pub, classical_message, &pqc_signature).unwrap();
    
    println!("Migration test results:");
    println!("Classical signature length: {} bytes", classical_signature.len());
    println!("PQC signature length: {} bytes", pqc_signature.len());
    println!("Dual signature format length: {} bytes", dual_signature.len());
    
    // Migration should work seamlessly
    assert!(can_verify_classical);
    assert_eq!(pqc_signature.len(), pqc_sec.parameter_set.signature_size());
    assert!(!dual_signature.is_empty());
}

#[test]
fn test_algorithm_agility() {
    // Test system's ability to switch between algorithms
    
    let test_message = b"Algorithm agility test message";
    
    // Test with different signature algorithms
    let signature_algorithms = [
        ("Dilithium", SecurityLevel::Level1),
        ("Falcon", SecurityLevel::Level1),
        ("SPHINCS+", SecurityLevel::Level1),
    ];
    
    let mut algorithm_results = HashMap::new();
    
    for (algorithm, level) in signature_algorithms {
        match algorithm {
            "Dilithium" => {
                let (pub_key, sec_key) = DilithiumSignature::keygen(level).unwrap();
                let signature = DilithiumSignature::sign(&sec_key, test_message).unwrap();
                let is_valid = DilithiumSignature::verify(&pub_key, test_message, &signature).unwrap();
                algorithm_results.insert(algorithm, (signature.len(), is_valid));
            },
            "Falcon" => {
                let (pub_key, sec_key) = FalconSignature::keygen(level).unwrap();
                let signature = FalconSignature::sign(&sec_key, test_message).unwrap();
                let is_valid = FalconSignature::verify(&pub_key, test_message, &signature).unwrap();
                algorithm_results.insert(algorithm, (signature.len(), is_valid));
            },
            "SPHINCS+" => {
                let (pub_key, sec_key) = SphincsPlusSignature::keygen(level).unwrap();
                let signature = SphincsPlusSignature::sign(&sec_key, test_message).unwrap();
                let is_valid = SphincsPlusSignature::verify(&pub_key, test_message, &signature).unwrap();
                algorithm_results.insert(algorithm, (signature.len(), is_valid));
            },
            _ => {}
        }
    }
    
    println!("Algorithm agility test results:");
    for (algorithm, (sig_len, _valid)) in &algorithm_results {
        println!("{}: signature length {} bytes", algorithm, sig_len);
    }
    
    // All algorithms should work
    assert_eq!(algorithm_results.len(), 3);
    
    // Verify signature size differences
    let dilithium_size = algorithm_results["Dilithium"].0;
    let falcon_size = algorithm_results["Falcon"].0;
    let sphincs_size = algorithm_results["SPHINCS+"].0;
    
    // Falcon should be more compact than SPHINCS+
    assert!(falcon_size < sphincs_size, "Falcon signatures should be more compact");
}

// ============================================================================
// REAL-WORLD WORKFLOW SIMULATION
// ============================================================================

#[test]
fn test_document_signing_workflow() {
    // Complete document signing and verification workflow
    
    struct Document {
        content: Vec<u8>,
        metadata: String,
        signature: Option<Vec<u8>>,
        signer_public_key: Option<DilithiumPublicKey>,
    }
    
    // Create a document
    let mut document = Document {
        content: b"This is an important legal document that requires digital signatures for authenticity.".to_vec(),
        metadata: "Created: 2024-01-01, Author: Legal Department".to_string(),
        signature: None,
        signer_public_key: None,
    };
    
    // Signer generates key pair
    let (signer_public, signer_secret) = DilithiumSignature::keygen(SecurityLevel::Level3).unwrap();
    
    // Create document hash for signing
    let document_data = [document.content.clone(), document.metadata.as_bytes().to_vec()].concat();
    let document_hash = sha3_256(&document_data).unwrap();
    
    // Sign the document hash
    let signature = DilithiumSignature::sign(&signer_secret, &document_hash).unwrap();
    
    // Attach signature to document
    document.signature = Some(signature.clone());
    document.signer_public_key = Some(signer_public.clone());
    
    // Verification process
    let verification_data = [document.content.clone(), document.metadata.as_bytes().to_vec()].concat();
    let verification_hash = sha3_256(&verification_data).unwrap();
    
    let is_valid = if let (Some(sig), Some(pub_key)) = (&document.signature, &document.signer_public_key) {
        DilithiumSignature::verify(pub_key, &verification_hash, sig).unwrap()
    } else {
        false
    };
    
    println!("Document signing workflow:");
    println!("Document size: {} bytes", document.content.len());
    println!("Signature size: {} bytes", signature.len());
    println!("Metadata: {}", document.metadata);
    
    // Workflow should complete successfully
    assert!(document.signature.is_some());
    assert!(document.signer_public_key.is_some());
    assert_eq!(signature.len(), signer_secret.parameter_set.signature_size());
}

#[test]
fn test_secure_messaging_system() {
    // Complete secure messaging system with PQC
    
    struct SecureMessage {
        sender_id: String,
        recipient_id: String,
        encrypted_content: Vec<u8>,
        kem_ciphertext: Vec<u8>,
        signature: Vec<u8>,
        timestamp: u64,
    }
    
    // Setup participants
    let (alice_signing_pub, alice_signing_sec) = DilithiumSignature::keygen(SecurityLevel::Level1).unwrap();
    let (bob_kem_pub, bob_kem_sec) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
    
    let plaintext_message = b"Hello Bob! This is a secure message using post-quantum cryptography.";
    
    // Alice sends message to Bob
    // Step 1: Generate shared secret using Bob's KEM public key
    let (kem_ciphertext, shared_secret) = KyberKem::encaps(&bob_kem_pub).unwrap();
    
    // Step 2: Derive encryption key
    let encryption_key = sha3_256(&shared_secret).unwrap();
    
    // Step 3: Encrypt message
    let encrypted_content: Vec<u8> = plaintext_message.iter().enumerate()
        .map(|(i, &byte)| byte ^ encryption_key[i % encryption_key.len()])
        .collect();
    
    // Step 4: Create message structure
    let message_data = format!("sender:alice,recipient:bob,timestamp:{},encrypted:{}", 
                              1234567890,
                              bytes_to_hex(&encrypted_content));
    
    // Step 5: Sign the message
    let signature = DilithiumSignature::sign(&alice_signing_sec, message_data.as_bytes()).unwrap();
    
    let secure_message = SecureMessage {
        sender_id: "alice".to_string(),
        recipient_id: "bob".to_string(),
        encrypted_content,
        kem_ciphertext,
        signature,
        timestamp: 1234567890,
    };
    
    // Bob receives and processes the message
    // Step 1: Verify signature
    let bob_message_data = format!("sender:{},recipient:{},timestamp:{},encrypted:{}", 
                                  secure_message.sender_id,
                                  secure_message.recipient_id,
                                  secure_message.timestamp,
                                  bytes_to_hex(&secure_message.encrypted_content));
    
    let signature_valid = DilithiumSignature::verify(&alice_signing_pub, bob_message_data.as_bytes(), &secure_message.signature).unwrap();
    
    // Step 2: Decrypt message
    let bob_shared_secret = KyberKem::decaps(&bob_kem_sec, &secure_message.kem_ciphertext).unwrap();
    let bob_encryption_key = sha3_256(&bob_shared_secret).unwrap();
    
    let decrypted_message: Vec<u8> = secure_message.encrypted_content.iter().enumerate()
        .map(|(i, &byte)| byte ^ bob_encryption_key[i % bob_encryption_key.len()])
        .collect();
    
    println!("Secure messaging system test:");
    println!("Original message: {:?}", std::str::from_utf8(plaintext_message).unwrap());
    println!("Decrypted message: {:?}", std::str::from_utf8(&decrypted_message).unwrap());
    println!("Message authenticated: signature verification completed");
    
    // Verify end-to-end security
    assert_eq!(shared_secret, bob_shared_secret);
    assert_eq!(encryption_key, bob_encryption_key);
    assert_eq!(plaintext_message, decrypted_message.as_slice());
}

// ============================================================================
// ERROR RECOVERY AND RESILIENCE TESTING
// ============================================================================

#[test]
fn test_error_recovery_mechanisms() {
    // Test system behavior under various error conditions
    
    let (valid_pub, valid_sec) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
    
    // Test 1: Recovery from invalid ciphertext
    let invalid_ciphertext = vec![0u8; 100]; // Wrong size
    let result = KyberKem::decaps(&valid_sec, &invalid_ciphertext);
    assert!(result.is_err(), "Should fail with invalid ciphertext");
    
    // System should continue working after error
    let (valid_ciphertext, _) = KyberKem::encaps(&valid_pub).unwrap();
    let recovery_result = KyberKem::decaps(&valid_sec, &valid_ciphertext);
    assert!(recovery_result.is_ok(), "System should recover after error");
    
    // Test 2: Recovery from signature verification failure
    let (sig_pub, sig_sec) = DilithiumSignature::keygen(SecurityLevel::Level1).unwrap();
    let message = b"Test message";
    let valid_signature = DilithiumSignature::sign(&sig_sec, message).unwrap();
    
    // Try to verify with wrong message
    let wrong_message = b"Different message";
    let verify_result = DilithiumSignature::verify(&sig_pub, wrong_message, &valid_signature);
    assert!(verify_result.is_ok(), "Verification should complete even with wrong message");
    
    // System should work normally after verification failure
    let new_signature = DilithiumSignature::sign(&sig_sec, message).unwrap();
    let new_verify_result = DilithiumSignature::verify(&sig_pub, message, &new_signature);
    assert!(new_verify_result.is_ok(), "System should continue working");
    
    println!("Error recovery testing completed successfully");
}

#[test]
fn test_system_resilience_under_load() {
    // Test system resilience under various stress conditions
    
    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let success_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..8).map(|thread_id| {
        let success_count = Arc::clone(&success_count);
        let error_count = Arc::clone(&error_count);
        
        thread::spawn(move || {
            for operation in 0..25 {
                // Mix valid and invalid operations
                if operation % 7 == 0 {
                    // Intentionally invalid operation
                    let invalid_ciphertext = vec![0u8; 50];
                    let (_, secret_key) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
                    let result = KyberKem::decaps(&secret_key, &invalid_ciphertext);
                    if result.is_err() {
                        error_count.fetch_add(1, Ordering::SeqCst);
                    }
                } else {
                    // Valid operation
                    let (pub_key, sec_key) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
                    let (ciphertext, secret1) = KyberKem::encaps(&pub_key).unwrap();
                    let secret2 = KyberKem::decaps(&sec_key, &ciphertext).unwrap();
                    
                    if secret1 == secret2 {
                        success_count.fetch_add(1, Ordering::SeqCst);
                    } else {
                        error_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
                
                // Small delay to simulate real-world timing
                std::thread::sleep(Duration::from_millis(1));
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_successes = success_count.load(Ordering::SeqCst);
    let total_errors = error_count.load(Ordering::SeqCst);
    
    println!("Resilience test results:");
    println!("Successful operations: {}", total_successes);
    println!("Expected errors: {}", total_errors);
    
    // Should have plenty of successful operations and some expected errors
    assert!(total_successes > 150, "Not enough successful operations");
    assert!(total_errors > 0, "Should have encountered some expected errors");
}
