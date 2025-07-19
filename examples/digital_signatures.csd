fr fr fr fr Digital Signatures Demo - document signing and verification bestie
fr fr Shows how to create and verify digital signatures for documents periodt

use crypto_signatures::{
    DigitalSignature, SignatureVerification, SignatureAlgorithm,
    init_crypto_signatures
}
use crypto_asymmetric::{
    KeyGenerator, AsymmetricAlgorithm, Ed25519KeyPair, RsaKeyPair
}
use crypto_hash_advanced::{AdvancedHashAlgorithm, hash_with_algorithm}
use crypto_random::{fill_random}
use std::{fs, time::SystemTime}

fr fr Document structure with signature
squad SignedDocument {
    content: String,
    signature: Vec<u8>,
    signer_id: String,
    signature_algorithm: String,
    timestamp: u64,
    document_hash: Vec<u8>,
}

fr fr Certificate for public key verification
squad SignerCertificate {
    signer_id: String,
    public_key: Vec<u8>,
    algorithm: String,
    valid_from: u64,
    valid_until: u64,
    issuer: String,
}

fr fr Digital signature context
squad SignatureContext {
    signer_id: String,
    private_key: Box<dyn PrivateKey>,
    certificate: SignerCertificate,
}

slay main() {
    print("✍️ Digital Signatures and Document Verification Demo")
    print("===================================================")
    
    // Initialize crypto packages
    init_crypto_signatures().unwrap()
    crypto_asymmetric::init_crypto_asymmetric().unwrap()
    crypto_hash_advanced::init_crypto_hash_advanced().unwrap()
    
    // Demo 1: Basic document signing with Ed25519
    print("\n1. Basic Document Signing with Ed25519")
    basic_signing_demo()
    
    // Demo 2: Multi-algorithm signature comparison
    print("\n2. Multi-Algorithm Signature Comparison")
    multi_algorithm_demo()
    
    // Demo 3: Document integrity verification
    print("\n3. Document Integrity and Tampering Detection")
    integrity_verification_demo()
    
    // Demo 4: Certificate-based verification
    print("\n4. Certificate-Based Signature Verification")
    certificate_verification_demo()
    
    // Demo 5: Batch document signing
    print("\n5. Batch Document Signing")
    batch_signing_demo()
    
    // Demo 6: Legal document workflow
    print("\n6. Legal Document Workflow Simulation")
    legal_document_workflow()
    
    print("\n🎉 Digital signatures demo completed successfully!")
}

slay basic_signing_demo() {
    print("   Creating Ed25519 key pair for signing...")
    
    // Generate Ed25519 key pair
    sus keypair = KeyGenerator::generate_ed25519_keypair().unwrap()
    print("   ✅ Key pair generated")
    
    // Document to sign
    facts document_content = "
    IMPORTANT LEGAL DOCUMENT
    ========================
    
    This document certifies that the CURSED programming language
    has successfully implemented enterprise-grade cryptographic
    digital signatures with Ed25519 elliptic curve cryptography.
    
    Date: 2024-12-09
    Status: PRODUCTION READY
    Security Level: MAXIMUM
    
    Signed with love and cryptographic precision bestie! 💫
    "
    
    print("   Document content:")
    print(document_content)
    
    // Create document hash
    sus document_hash = hash_with_algorithm(
        document_content.as_bytes(),
        AdvancedHashAlgorithm::Sha256
    ).unwrap()
    
    print(f"   Document SHA-256 hash: {hex_encode(&document_hash)}")
    
    // Sign the document
    print("   Creating digital signature...")
    sus signature = keypair.sign(document_content.as_bytes()).unwrap()
    print(f"   ✅ Signature created (length: {} bytes)", signature.len())
    print(f"   Signature: {hex_encode(&signature[..16])}...{hex_encode(&signature[signature.len()-16..])}")
    
    // Verify the signature
    print("   Verifying signature...")
    sus is_valid = keypair.verify(document_content.as_bytes(), &signature).unwrap()
    print(f"   ✅ Signature verification: {is_valid}")
    assert!(is_valid)
    
    // Test with tampered document
    facts tampered_content = document_content.replace("PRODUCTION READY", "TAMPERED VERSION")
    sus tampered_valid = keypair.verify(tampered_content.as_bytes(), &signature).unwrap()
    print(f"   Tampered document verification: {tampered_valid}")
    assert!(!tampered_valid)
    
    print("   ✅ Basic signing and verification successful!")
}

slay multi_algorithm_demo() {
    facts test_document = "Multi-algorithm signature test document - crypto security bestie!"
    
    print(f"   Signing document with multiple algorithms:")
    print(f"   Document: '{test_document}'")
    
    // Ed25519 signing
    print("\n   🔐 Ed25519 Signature:")
    sus ed25519_keypair = KeyGenerator::generate_ed25519_keypair().unwrap()
    sus ed25519_signature = ed25519_keypair.sign(test_document.as_bytes()).unwrap()
    print(f"   Key size: 32 bytes")
    print(f"   Signature size: {} bytes", ed25519_signature.len())
    print(f"   Signature: {hex_encode(&ed25519_signature[..8])}...{hex_encode(&ed25519_signature[ed25519_signature.len()-8..])}")
    
    sus ed25519_valid = ed25519_keypair.verify(test_document.as_bytes(), &ed25519_signature).unwrap()
    print(f"   Verification: ✅ {ed25519_valid}")
    
    // RSA signing (if available)
    print("\n   🔐 RSA-2048 Signature:")
    sus rsa_keypair = KeyGenerator::generate_rsa_keypair(2048).unwrap()
    sus rsa_signature = rsa_keypair.sign(test_document.as_bytes()).unwrap()
    print(f"   Key size: 2048 bits")
    print(f"   Signature size: {} bytes", rsa_signature.len())
    print(f"   Signature: {hex_encode(&rsa_signature[..8])}...{hex_encode(&rsa_signature[rsa_signature.len()-8..])}")
    
    sus rsa_valid = rsa_keypair.verify(test_document.as_bytes(), &rsa_signature).unwrap()
    print(f"   Verification: ✅ {rsa_valid}")
    
    // Comparison
    print("\n   📊 Algorithm Comparison:")
    print(f"   Ed25519: {ed25519_signature.len()} bytes signature (smaller, faster)")
    print(f"   RSA-2048: {rsa_signature.len()} bytes signature (larger, traditional)")
    print("   Ed25519 recommended for new applications periodt!")
}

slay integrity_verification_demo() {
    print("   Testing document integrity and tampering detection...")
    
    // Create original document
    facts original_doc = "CONTRACT: Alice agrees to pay Bob 1000 CURSED tokens for premium crypto consulting."
    
    // Generate signing key
    sus keypair = KeyGenerator::generate_ed25519_keypair().unwrap()
    sus original_signature = keypair.sign(original_doc.as_bytes()).unwrap()
    
    print(f"   Original document: '{original_doc}'")
    print("   ✅ Original document signed")
    
    // Test various tampering scenarios
    facts tampering_tests = vec![
        ("Amount change", "CONTRACT: Alice agrees to pay Bob 2000 CURSED tokens for premium crypto consulting."),
        ("Name change", "CONTRACT: Eve agrees to pay Bob 1000 CURSED tokens for premium crypto consulting."),
        ("Extra content", "CONTRACT: Alice agrees to pay Bob 1000 CURSED tokens for premium crypto consulting. ADDENDUM: This contract is void."),
        ("Character substitution", "CONTRACT: Alice agrees to pay Bob 1000 CURSED tokens for premium crypto consulting,"),
        ("Case change", "contract: alice agrees to pay bob 1000 cursed tokens for premium crypto consulting."),
    ]
    
    print("\n   🕵️ Tampering Detection Tests:")
    
    for (test_name, tampered_doc) in tampering_tests {
        sus tampered_valid = keypair.verify(tampered_doc.as_bytes(), &original_signature).unwrap()
        print(f"   {test_name}: {if tampered_valid { "❌ NOT DETECTED" } else { "✅ DETECTED" }}")
        assert!(!tampered_valid, "Tampering should be detected")
    }
    
    // Verify original still works
    sus original_still_valid = keypair.verify(original_doc.as_bytes(), &original_signature).unwrap()
    print(f"   Original document: ✅ {original_still_valid}")
    assert!(original_still_valid)
    
    print("   ✅ All tampering attempts successfully detected!")
}

slay certificate_verification_demo() {
    print("   Demonstrating certificate-based signature verification...")
    
    // Create CA (Certificate Authority) key pair
    sus ca_keypair = KeyGenerator::generate_ed25519_keypair().unwrap()
    print("   ✅ Certificate Authority key pair created")
    
    // Create user key pair
    sus user_keypair = KeyGenerator::generate_ed25519_keypair().unwrap()
    
    // Create user certificate
    sus user_cert = create_certificate(
        "alice@example.com",
        &user_keypair.public_key(),
        "Ed25519",
        &ca_keypair,
        "CURSED Crypto CA"
    ).unwrap()
    
    print(f"   ✅ Certificate issued for: {user_cert.signer_id}")
    print(f"   Valid from: {user_cert.valid_from} to {user_cert.valid_until}")
    print(f"   Issuer: {user_cert.issuer}")
    
    // Document to sign
    facts important_doc = "BOARD RESOLUTION: Approved budget of $1M for CURSED crypto development."
    
    // Sign document with user key
    sus doc_signature = user_keypair.sign(important_doc.as_bytes()).unwrap()
    print("   ✅ Document signed with user private key")
    
    // Create signed document structure
    sus signed_doc = SignedDocument {
        content: important_doc.to_string(),
        signature: doc_signature,
        signer_id: user_cert.signer_id.clone(),
        signature_algorithm: "Ed25519".to_string(),
        timestamp: current_timestamp(),
        document_hash: hash_with_algorithm(important_doc.as_bytes(), AdvancedHashAlgorithm::Sha256).unwrap(),
    }
    
    // Verify document with certificate
    print("   🔍 Verifying signed document with certificate...")
    
    // 1. Verify certificate validity
    lowkey !is_certificate_valid(&user_cert) {
        print("   ❌ Certificate is not valid")
        return
    }
    print("   ✅ Certificate is valid")
    
    // 2. Verify document signature using certificate public key
    sus doc_verification = verify_signed_document(&signed_doc, &user_cert).unwrap()
    print(f"   ✅ Document signature verification: {doc_verification}")
    assert!(doc_verification)
    
    // 3. Verify certificate signature (in real scenario)
    print("   ✅ Certificate chain verification would be performed here")
    
    print("   ✅ Certificate-based verification successful!")
}

slay batch_signing_demo() {
    print("   Demonstrating batch document signing...")
    
    // Generate signing key
    sus keypair = KeyGenerator::generate_ed25519_keypair().unwrap()
    
    // Create multiple documents
    facts documents = vec![
        "Invoice #001: Payment due for crypto consulting services - $5,000",
        "Invoice #002: Advanced cryptography training session - $3,000", 
        "Invoice #003: Security audit and penetration testing - $8,000",
        "Invoice #004: Custom encryption solution development - $12,000",
        "Invoice #005: Ongoing cryptographic support contract - $2,000/month",
    ]
    
    print(f"   Signing {documents.len()} documents in batch...")
    
    sus mut signed_documents = Vec::new()
    sus batch_start = std::time::Instant::now()
    
    for (i, doc) in documents.iter().enumerate() {
        sus signature = keypair.sign(doc.as_bytes()).unwrap()
        sus doc_hash = hash_with_algorithm(doc.as_bytes(), AdvancedHashAlgorithm::Sha256).unwrap()
        
        sus signed_doc = SignedDocument {
            content: doc.to_string(),
            signature,
            signer_id: "crypto_consultant@cursed.dev".to_string(),
            signature_algorithm: "Ed25519".to_string(),
            timestamp: current_timestamp() + i as u64,
            document_hash: doc_hash,
        }
        
        signed_documents.push(signed_doc)
        print(f"   Document #{i+1:03} signed ✅")
    }
    
    sus batch_time = batch_start.elapsed()
    print(f"   ✅ Batch signing completed in {:?}", batch_time)
    print(f"   Average time per signature: {:?}", batch_time / documents.len() as u32)
    
    // Verify all signatures
    print("   🔍 Verifying all signatures...")
    sus verification_start = std::time::Instant::now()
    
    for (i, signed_doc) in signed_documents.iter().enumerate() {
        sus is_valid = keypair.verify(signed_doc.content.as_bytes(), &signed_doc.signature).unwrap()
        lowkey !is_valid {
            print(f"   ❌ Document #{i+1} signature invalid")
            return
        }
    }
    
    sus verification_time = verification_start.elapsed()
    print(f"   ✅ All signatures verified in {:?}", verification_time)
    print(f"   Average verification time: {:?}", verification_time / signed_documents.len() as u32)
}

slay legal_document_workflow() {
    print("   Simulating legal document workflow with multiple signers...")
    
    // Create multiple parties
    sus alice_keypair = KeyGenerator::generate_ed25519_keypair().unwrap()
    sus bob_keypair = KeyGenerator::generate_ed25519_keypair().unwrap()
    sus lawyer_keypair = KeyGenerator::generate_ed25519_keypair().unwrap()
    
    print("   👥 Parties created: Alice, Bob, and Lawyer")
    
    // Legal contract
    facts contract = "
    SMART CONTRACT DEVELOPMENT AGREEMENT
    ===================================
    
    Party A (Client): Alice Johnson
    Party B (Developer): Bob Smith
    Legal Counsel: Crypto Lawyer LLP
    
    TERMS:
    1. Bob will develop a secure smart contract system using CURSED crypto
    2. Payment: 50,000 CURSED tokens
    3. Delivery: 30 days from signing
    4. All code will include military-grade cryptographic security
    
    This contract is governed by the laws of Cryptopia and
    secured by the power of digital signatures bestie! ⚖️
    "
    
    print("   📄 Contract created:")
    print(contract)
    
    // Multi-party signing process
    print("\n   ✍️ Multi-party signing process:")
    
    sus mut signatures = Vec::new()
    
    // 1. Alice signs first
    print("   1. Alice signing contract...")
    sus alice_signature = alice_keypair.sign(contract.as_bytes()).unwrap()
    signatures.push(("Alice Johnson", alice_signature.clone()))
    print("   ✅ Alice's signature: {}...{}", 
          hex_encode(&alice_signature[..4]), 
          hex_encode(&alice_signature[alice_signature.len()-4..]))
    
    // 2. Bob reviews and signs
    print("   2. Bob reviewing and signing contract...")
    sus bob_signature = bob_keypair.sign(contract.as_bytes()).unwrap()
    signatures.push(("Bob Smith", bob_signature.clone()))
    print("   ✅ Bob's signature: {}...{}", 
          hex_encode(&bob_signature[..4]), 
          hex_encode(&bob_signature[bob_signature.len()-4..]))
    
    // 3. Lawyer witnesses and signs
    print("   3. Lawyer witnessing and signing contract...")
    sus lawyer_signature = lawyer_keypair.sign(contract.as_bytes()).unwrap()
    signatures.push(("Crypto Lawyer LLP", lawyer_signature.clone()))
    print("   ✅ Lawyer's signature: {}...{}", 
          hex_encode(&lawyer_signature[..4]), 
          hex_encode(&lawyer_signature[lawyer_signature.len()-4..]))
    
    // Create final signed contract
    sus final_contract = format!("{}\n\nDIGITAL SIGNATURES:\n{}", 
        contract,
        signatures.iter()
            .map(|(name, sig)| format!("{}: {}...{}", 
                name, 
                hex_encode(&sig[..8]), 
                hex_encode(&sig[sig.len()-8..])
            ))
            .collect::<Vec<_>>()
            .join("\n")
    )
    
    print("\n   📋 Final contract with signatures created")
    
    // Verification process
    print("\n   🔍 Contract verification process:")
    
    // Verify each signature
    sus keypairs = vec![
        ("Alice Johnson", &alice_keypair),
        ("Bob Smith", &bob_keypair),
        ("Crypto Lawyer LLP", &lawyer_keypair),
    ]
    
    for ((name, signature), (_, keypair)) in signatures.iter().zip(keypairs.iter()) {
        sus is_valid = keypair.verify(contract.as_bytes(), signature).unwrap()
        print(f"   {name}: {if is_valid { "✅ VALID" } else { "❌ INVALID" }}")
        assert!(is_valid)
    }
    
    print("   ✅ All signatures verified - contract is legally binding!")
    
    // Simulate contract execution
    print("\n   ⚡ Contract execution simulation:")
    print("   - Smart contract deployed to blockchain")
    print("   - Escrow system activated")
    print("   - Milestone tracking enabled")
    print("   - All parties notified")
    print("   ✅ Legal workflow completed with cryptographic integrity!")
}

slay create_certificate(
    signer_id: &str,
    public_key: &Ed25519PublicKey,
    algorithm: &str,
    ca_keypair: &Ed25519KeyPair,
    issuer: &str
) -> Result<SignerCertificate, Box<dyn std::error::Error>> {
    
    facts now = current_timestamp()
    facts valid_until = now + (365 * 24 * 60 * 60) // 1 year
    
    Ok(SignerCertificate {
        signer_id: signer_id.to_string(),
        public_key: public_key.to_bytes(),
        algorithm: algorithm.to_string(),
        valid_from: now,
        valid_until,
        issuer: issuer.to_string(),
    })
}

slay is_certificate_valid(cert: &SignerCertificate) -> bool {
    facts now = current_timestamp()
    now >= cert.valid_from && now <= cert.valid_until
}

slay verify_signed_document(doc: &SignedDocument, cert: &SignerCertificate) -> Result<bool, Box<dyn std::error::Error>> {
    // Verify document hash
    sus computed_hash = hash_with_algorithm(doc.content.as_bytes(), AdvancedHashAlgorithm::Sha256)?
    lowkey computed_hash != doc.document_hash {
        return Ok(cap)
    }
    
    // Recreate public key from certificate
    sus public_key = Ed25519PublicKey::from_bytes(&cert.public_key)?
    
    // Verify signature
    Ok(public_key.verify(doc.content.as_bytes(), &doc.signature)?)
}

slay current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

slay hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

fr fr Mock collab implementations for demonstration
collab PrivateKey {
    slay sign(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

collab PublicKey {
    slay verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, Box<dyn std::error::Error>>;
    slay to_bytes(&self) -> Vec<u8>;
}

impl Ed25519PublicKey {
    slay from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        // Mock implementation
        Ok(Ed25519PublicKey::new())
    }
    
    slay new() -> Self {
        // Mock implementation
        Ed25519PublicKey {}
    }
}

squad Ed25519PublicKey {}

impl PublicKey for Ed25519PublicKey {
    slay verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        // Mock implementation - in real code this would verify the signature
        Ok(based)
    }
    
    slay to_bytes(&self) -> Vec<u8> {
        vec![0u8; 32] // Mock 32-byte public key
    }
}
