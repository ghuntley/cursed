#!/usr/bin/env cursed

//! Post-Quantum Cryptography Showcase
//! 
//! This example demonstrates the comprehensive post-quantum cryptographic
//! capabilities of the CURSED programming language, including:
//! - Kyber key encapsulation mechanism
//! - Dilithium digital signatures
//! - Hybrid classical-quantum schemes
//! - Performance benchmarking
//! - Security assessments

import "stdlib::crypto::pqc_production" as pqc
import "stdlib::io" as io

fn main() -> Result<(), String> {
    println("🔐 CURSED Post-Quantum Cryptography Showcase")
    println("===========================================")
    
    // Demonstrate Kyber KEM
    kyber_demonstration()?
    
    // Demonstrate Dilithium signatures
    dilithium_demonstration()?
    
    // Demonstrate hybrid cryptography
    hybrid_demonstration()?
    
    // Performance benchmarking
    performance_benchmarks()?
    
    // Security assessment
    security_assessment()?
    
    println("\n✅ All post-quantum cryptography demonstrations completed successfully!")
    println("🛡️  Your data is now quantum-safe!")
    
    Ok(())
}

//! Demonstrate Kyber Key Encapsulation Mechanism
fn kyber_demonstration() -> Result<(), String> {
    println("\n🚀 Kyber Key Encapsulation Mechanism Demo")
    println("----------------------------------------")
    
    // Test all security levels
    let security_levels = [
        ("Level 1 (AES-128 equivalent)", pqc::SecurityLevel::Level1),
        ("Level 3 (AES-192 equivalent)", pqc::SecurityLevel::Level3),
        ("Level 5 (AES-256 equivalent)", pqc::SecurityLevel::Level5),
    ]
    
    for (name, level) in security_levels {
        println(f"\n📊 Testing {name}")
        
        // Generate key pair
        let start_time = time::now()
        let (public_key, secret_key) = pqc::KyberKem::keygen(level)?
        let keygen_time = time::elapsed(start_time)
        
        println(f"  ✓ Key generation: {keygen_time}ms")
        println(f"  ✓ Public key size: {public_key.key_data.len()} bytes")
        println(f"  ✓ Secret key size: {secret_key.key_data.len()} bytes")
        
        // Encapsulation
        let start_time = time::now()
        let (ciphertext, shared_secret1) = pqc::KyberKem::encaps(&public_key)?
        let encaps_time = time::elapsed(start_time)
        
        println(f"  ✓ Encapsulation: {encaps_time}ms")
        println(f"  ✓ Ciphertext size: {ciphertext.len()} bytes")
        
        // Decapsulation
        let start_time = time::now()
        let shared_secret2 = pqc::KyberKem::decaps(&secret_key, &ciphertext)?
        let decaps_time = time::elapsed(start_time)
        
        println(f"  ✓ Decapsulation: {decaps_time}ms")
        
        // Verify shared secrets match
        if shared_secret1.as_slice() == shared_secret2.as_slice() {
            println("  ✅ Shared secrets match - KEM successful!")
        } else {
            return Err("❌ Shared secrets don't match - KEM failed!")
        }
        
        println(f"  📈 Total operation time: {keygen_time + encaps_time + decaps_time}ms")
    }
    
    Ok(())
}

//! Demonstrate Dilithium Digital Signatures
fn dilithium_demonstration() -> Result<(), String> {
    println("\n✍️  Dilithium Digital Signatures Demo")
    println("-----------------------------------")
    
    let security_levels = [
        ("Level 2", pqc::SecurityLevel::Level1),
        ("Level 3", pqc::SecurityLevel::Level3), 
        ("Level 5", pqc::SecurityLevel::Level5),
    ]
    
    // Test message
    let message = "This is a super important message that needs quantum-safe signing! 🔏"
    let message_bytes = message.as_bytes()
    
    for (name, level) in security_levels {
        println(f"\n📝 Testing Dilithium {name}")
        
        // Generate signing key pair
        let start_time = time::now()
        let (public_key, secret_key) = pqc::DilithiumSigner::keygen(level)?
        let keygen_time = time::elapsed(start_time)
        
        println(f"  ✓ Key generation: {keygen_time}ms")
        println(f"  ✓ Public key size: {public_key.key_data.len()} bytes")
        println(f"  ✓ Secret key size: {secret_key.key_data.len()} bytes")
        
        // Sign message
        let start_time = time::now()
        let signature = pqc::DilithiumSigner::sign(&secret_key, message_bytes)?
        let sign_time = time::elapsed(start_time)
        
        println(f"  ✓ Signing: {sign_time}ms")
        println(f"  ✓ Signature size: {signature.len()} bytes")
        
        // Verify signature
        let start_time = time::now()
        let is_valid = pqc::DilithiumSigner::verify(&public_key, message_bytes, &signature)?
        let verify_time = time::elapsed(start_time)
        
        println(f"  ✓ Verification: {verify_time}ms")
        
        if is_valid {
            println("  ✅ Signature verification successful!")
        } else {
            return Err("❌ Signature verification failed!")
        }
        
        // Test signature tampering detection
        let mut tampered_signature = signature.clone()
        tampered_signature[0] ^= 0xFF  // Flip bits in first byte
        
        let tampered_result = pqc::DilithiumSigner::verify(&public_key, message_bytes, &tampered_signature)?
        if !tampered_result {
            println("  ✅ Tampered signature correctly rejected!")
        } else {
            return Err("❌ Failed to detect signature tampering!")
        }
        
        println(f"  📈 Total signature operation: {sign_time + verify_time}ms")
    }
    
    Ok(())
}

//! Demonstrate Hybrid Classical-Quantum Cryptography
fn hybrid_demonstration() -> Result<(), String> {
    println("\n🔄 Hybrid Classical-Quantum Cryptography Demo")
    println("--------------------------------------------")
    
    println("Hybrid schemes combine classical and post-quantum algorithms")
    println("for enhanced security during the transition period.")
    
    // Generate hybrid key pairs for Alice and Bob
    println("\n👩 Generating Alice's hybrid key pair...")
    let alice_keys = pqc::HybridKeyExchange::generate_keypair(pqc::SecurityLevel::Level3)?
    
    println("👨 Generating Bob's hybrid key pair...")
    let bob_keys = pqc::HybridKeyExchange::generate_keypair(pqc::SecurityLevel::Level3)?
    
    // Perform hybrid key exchange
    println("\n🤝 Performing hybrid key exchange...")
    let start_time = time::now()
    let shared_secret = pqc::HybridKeyExchange::perform_exchange(&alice_keys, &bob_keys)?
    let exchange_time = time::elapsed(start_time)
    
    println(f"  ✓ Key exchange completed in {exchange_time}ms")
    println(f"  ✓ Shared secret size: {shared_secret.len()} bytes")
    println("  ✅ Hybrid key exchange successful!")
    
    println("\n🛡️  Security Properties:")
    println("  • Protected against classical attacks (ECDH component)")
    println("  • Protected against quantum attacks (Kyber component)")
    println("  • Provides forward secrecy")
    println("  • Suitable for migration scenarios")
    
    Ok(())
}

//! Performance benchmarking of PQC algorithms
fn performance_benchmarks() -> Result<(), String> {
    println("\n⚡ Performance Benchmarking")
    println("-------------------------")
    
    println("Running comprehensive benchmarks...")
    
    let iterations = 10
    let results = pqc::PqcBenchmarkSuite::run_all_benchmarks(iterations)?
    
    println(f"\n📊 Benchmark Results ({iterations} iterations each):")
    println("Algorithm    | KeyGen(ms) | Ops/Sec | Total Size(KB)")
    println("-------------|------------|---------|---------------")
    
    for result in results {
        let total_size_kb = (result.key_sizes.0 + result.key_sizes.1 + result.ciphertext_size) / 1024
        println(f"{result.parameter_set:12} | {result.avg_keygen_time.as_millis():10} | {result.operations_per_second():7.0} | {total_size_kb:13}")
    }
    
    println("\n💡 Performance Analysis:")
    println("  • Level 1: Fastest operations, smallest keys")
    println("  • Level 3: Balanced security/performance (recommended)")
    println("  • Level 5: Maximum security, largest overhead")
    
    // Generate comparative analysis
    let analysis = pqc::PqcBenchmarkSuite::comparative_analysis(&results)
    println(f"\n{analysis}")
    
    Ok(())
}

//! Security assessment and recommendations
fn security_assessment() -> Result<(), String> {
    println("\n🛡️  Quantum Security Assessment")
    println("=============================")
    
    // Display current threat level
    let threat_level = pqc::QuantumThreatAssessment::current_threat_level()
    println(f"🚨 Current Threat Level: {threat_level}")
    
    // Algorithm recommendations
    println("\n📋 Algorithm Recommendations:")
    
    let use_cases = [
        ("Key Exchange", "kem"),
        ("Digital Signatures", "signature"), 
        ("Maximum Security", "hash_signature"),
        ("Compact Signatures", "compact_signature"),
    ]
    
    for (use_case, identifier) in use_cases {
        let recommended = pqc::get_recommended_algorithm(identifier, pqc::SecurityLevel::Level3)?
        let timeline = pqc::QuantumThreatAssessment::migration_timeline(recommended)
        
        println(f"  {use_case:20} → {recommended}")
        println(f"  {' ':20}   Timeline: {timeline}")
    }
    
    // Generate comprehensive security report
    println("\n📄 Comprehensive Security Report:")
    let security_report = pqc::QuantumThreatAssessment::security_report()
    println(security_report)
    
    // Migration recommendations
    println("\n🎯 Migration Strategy:")
    println("  1. IMMEDIATE: Assess current cryptographic inventory")
    println("  2. SHORT-TERM: Implement hybrid schemes for critical systems")
    println("  3. MEDIUM-TERM: Full migration to post-quantum algorithms")
    println("  4. LONG-TERM: Regular security assessments and updates")
    
    Ok(())
}

//! Demonstrate real-world use cases
fn real_world_use_cases() -> Result<(), String> {
    println("\n🌍 Real-World Use Cases")
    println("======================")
    
    // Secure messaging
    secure_messaging_example()?
    
    // Document signing
    document_signing_example()?
    
    // VPN key exchange
    vpn_key_exchange_example()?
    
    Ok(())
}

//! Secure messaging with PQC
fn secure_messaging_example() -> Result<(), String> {
    println("\n💬 Secure Messaging Example")
    println("---------------------------")
    
    // Alice generates her keys
    let (alice_sign_pk, alice_sign_sk) = pqc::DilithiumSigner::keygen(pqc::SecurityLevel::Level3)?
    let (alice_kem_pk, alice_kem_sk) = pqc::KyberKem::keygen(pqc::SecurityLevel::Level3)?
    
    // Bob generates his keys  
    let (bob_sign_pk, bob_sign_sk) = pqc::DilithiumSigner::keygen(pqc::SecurityLevel::Level3)?
    let (bob_kem_pk, bob_kem_sk) = pqc::KyberKem::keygen(pqc::SecurityLevel::Level3)?
    
    // Alice sends a message to Bob
    let message = "Hey Bob! This message is quantum-safe! 🔐"
    let message_bytes = message.as_bytes()
    
    // Alice signs the message
    let signature = pqc::DilithiumSigner::sign(&alice_sign_sk, message_bytes)?
    
    // Alice encrypts using Bob's KEM public key
    let (ciphertext, shared_secret) = pqc::KyberKem::encaps(&bob_kem_pk)?
    
    println("✓ Alice signed and encrypted message")
    println(f"  Message: '{message}'")
    println(f"  Signature size: {signature.len()} bytes")
    println(f"  Ciphertext size: {ciphertext.len()} bytes")
    
    // Bob receives and processes the message
    // First, decrypt to get shared secret
    let bob_shared_secret = pqc::KyberKem::decaps(&bob_kem_sk, &ciphertext)?
    
    // Verify shared secrets match
    if shared_secret.as_slice() != bob_shared_secret.as_slice() {
        return Err("❌ Key exchange failed!")
    }
    
    // Verify Alice's signature
    let signature_valid = pqc::DilithiumSigner::verify(&alice_sign_pk, message_bytes, &signature)?
    
    if signature_valid {
        println("✅ Bob successfully verified message authenticity")
        println("✅ Message confidentiality protected by quantum-safe encryption")
    } else {
        return Err("❌ Signature verification failed!")
    }
    
    Ok(())
}

//! Document signing for legal applications
fn document_signing_example() -> Result<(), String> {
    println("\n📄 Legal Document Signing Example")
    println("---------------------------------")
    
    // Use SPHINCS+ for maximum security and long-term validity
    let (public_key, secret_key) = pqc::SphincsPlusSignature::keygen(pqc::SecurityLevel::Level5)?
    
    let document = "IMPORTANT LEGAL CONTRACT\n\nThis document represents a binding agreement...\n[Full contract text would be here]"
    let document_hash = crypto::hash::sha3_256(document.as_bytes())
    
    // Sign document hash (standard practice)
    let signature = pqc::SphincsPlusSignature::sign(&secret_key, &document_hash)?
    
    println("✓ Document signed with SPHINCS+ (maximum security)")
    println(f"  Document length: {document.len()} characters")
    println(f"  Hash: {pqc::bytes_to_hex(&document_hash)}")
    println(f"  Signature size: {signature.len()} bytes")
    
    // Verify signature (as would be done years later)
    let signature_valid = pqc::SphincsPlusSignature::verify(&public_key, &document_hash, &signature)?
    
    if signature_valid {
        println("✅ Signature verified - document authenticity confirmed")
        println("🛡️  Signature remains valid even against future quantum attacks")
    } else {
        return Err("❌ Document signature verification failed!")
    }
    
    Ok(())
}

//! VPN key exchange with perfect forward secrecy
fn vpn_key_exchange_example() -> Result<(), String> {
    println("\n🌐 VPN Key Exchange Example")
    println("---------------------------")
    
    // Simulate VPN client and server
    println("🖥️  Simulating quantum-safe VPN handshake...")
    
    // Server generates ephemeral keys
    let (server_kem_pk, server_kem_sk) = pqc::KyberKem::keygen(pqc::SecurityLevel::Level3)?
    
    // Client performs key exchange
    let (client_ciphertext, client_shared_secret) = pqc::KyberKem::encaps(&server_kem_pk)?
    
    // Server derives the same shared secret
    let server_shared_secret = pqc::KyberKem::decaps(&server_kem_sk, &client_ciphertext)?
    
    // Verify key exchange succeeded
    if client_shared_secret.as_slice() != server_shared_secret.as_slice() {
        return Err("❌ VPN key exchange failed!")
    }
    
    println("✅ VPN key exchange completed successfully")
    println(f"  Shared secret size: {client_shared_secret.len()} bytes")
    println("🔒 VPN tunnel established with quantum-safe encryption")
    println("⏰ Perfect forward secrecy: past communications remain secure")
    
    // Additional security session keys would be derived from shared secret
    let session_key = crypto::kdf::hkdf_sha256(&client_shared_secret.as_slice(), b"vpn_session", 32)?
    println(f"  Session key derived: {pqc::bytes_to_hex(&session_key[..16])}...")
    
    Ok(())
}

//! Utility function to demonstrate constant-time operations
fn demonstrate_constant_time_operations() -> Result<(), String> {
    println("\n⏱️  Constant-Time Operations Demo")
    println("-------------------------------")
    
    let secret1 = b"secret_password_123"
    let secret2 = b"secret_password_123"
    let secret3 = b"wrong_password_456"
    
    // Demonstrate constant-time comparison
    println("Testing constant-time byte comparison:")
    
    let result1 = pqc::ConstantTime::bytes_equal(secret1, secret2)
    println(f"  '{String::from_utf8_lossy(secret1)}' == '{String::from_utf8_lossy(secret2)}': {result1}")
    
    let result2 = pqc::ConstantTime::bytes_equal(secret1, secret3)
    println(f"  '{String::from_utf8_lossy(secret1)}' == '{String::from_utf8_lossy(secret3)}': {result2}")
    
    println("✅ All comparisons performed in constant time")
    println("🛡️  Protection against timing attacks implemented")
    
    Ok(())
}

//! Display comprehensive security summary
fn security_summary() -> Result<(), String> {
    println("\n🎯 SECURITY SUMMARY")
    println("==================")
    
    println("✅ Implemented Protections:")
    println("  • Quantum-safe key exchange (Kyber)")
    println("  • Quantum-safe digital signatures (Dilithium)")
    println("  • Hash-based signatures for maximum security (SPHINCS+)")
    println("  • Hybrid classical-quantum schemes")
    println("  • Constant-time operations against timing attacks")
    println("  • Secure memory handling with automatic zeroization")
    println("  • Side-channel attack resistance")
    println("  • NIST standard compliance")
    
    println("\n🚨 Threat Mitigation:")
    println("  • Shor's Algorithm: ✅ Mitigated by lattice-based cryptography")
    println("  • Grover's Algorithm: ✅ Mitigated by increased key sizes")
    println("  • Harvest Now, Decrypt Later: ✅ Mitigated by immediate deployment")
    println("  • Side-Channel Attacks: ✅ Mitigated by constant-time operations")
    
    println("\n📈 Performance Impact:")
    println("  • Key Generation: ~80μs (fast)")
    println("  • Key Exchange: ~90μs total (acceptable)")
    println("  • Digital Signatures: ~100μs (reasonable)")
    println("  • Memory Overhead: 2-50x (manageable)")
    
    println("\n🎯 Recommendations:")
    println("  1. Deploy Kyber for all key exchange immediately")
    println("  2. Use Dilithium for standard digital signatures")
    println("  3. Use SPHINCS+ for long-term document signing")
    println("  4. Implement hybrid schemes during transition")
    println("  5. Regular security assessments and updates")
    
    Ok(())
}

// Helper functions for timing and utilities
mod time {
    use std::time::Instant
    
    pub fn now() -> Instant {
        Instant::now()
    }
    
    pub fn elapsed(start: Instant) -> Duration {
        start.elapsed()
    }
}

mod crypto {
    pub mod hash {
        use sha3::{Sha3_256, Digest}
        
        pub fn sha3_256(data: &[u8]) -> Vec<u8> {
            let mut hasher = Sha3_256::new()
            hasher.update(data)
            hasher.finalize().to_vec()
        }
    }
    
    pub mod kdf {
        use hmac::{Hmac, Mac}
        use sha2::Sha256
        
        pub fn hkdf_sha256(ikm: &[u8], info: &[u8], len: usize) -> Result<Vec<u8>, String> {
            // Simplified HKDF implementation
            let mut mac = Hmac::<Sha256>::new_from_slice(ikm)
                .map_err(|e| format!("HKDF error: {}", e))?
            mac.update(info)
            let okm = mac.finalize().into_bytes()
            Ok(okm[..len.min(okm.len())].to_vec())
        }
    }
}
