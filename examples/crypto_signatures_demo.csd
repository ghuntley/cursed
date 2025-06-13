/// fr fr CURSED Digital Signatures Demo - Comprehensive showcase bestie!
/// 
/// This program demonstrates all the digital signature capabilities of CURSED:
/// - Ed25519, ECDSA, and RSA signatures
/// - Key generation and management
/// - Multi-signature schemes
/// - Universal verification interface
/// - Batch processing
/// - Real-world use cases

import "stdlib::crypto_signatures";
import "stdlib::io";

// Main demo function
func main() -> yolo {
    println("🔐 CURSED Digital Signatures Demo - Let's secure some data!")?;
    println("=" * 60)?;
    
    // Initialize the crypto signatures package
    init_crypto_signatures()?;
    
    // Run demo sections
    demo_ed25519_signatures()?;
    demo_ecdsa_signatures()?;
    demo_rsa_signatures()?;
    demo_universal_interface()?;
    demo_key_management()?;
    demo_multi_signatures()?;
    demo_batch_verification()?;
    demo_real_world_scenarios()?;
    
    println("\n✅ All digital signature demos completed successfully!")?;
    println("🎉 CURSED crypto signatures are ready for production use!")?;
}

// Demonstrate Ed25519 signatures
func demo_ed25519_signatures() -> yolo {
    println("\n📝 Ed25519 Digital Signatures Demo")?;
    println("-" * 40)?;
    
    // Generate Ed25519 key pair
    sus mut generator = KeyGenerator::new();
    sus keypair = generator.generate_keypair(KeyType::Ed25519)?;
    
    println("✅ Generated Ed25519 key pair")?;
    printf("   🔑 Key ID: {}\n", &[keypair.key_id])?;
    printf("   📏 Private key size: {} bytes\n", &[keypair.private_key.len()])?;
    printf("   📏 Public key size: {} bytes\n", &[keypair.public_key.len()])?;
    
    // Create signer
    sus signer = Ed25519Signer::new(keypair.clone())?;
    
    // Sign a message
    sus message = "Hello, CURSED digital signatures! 🔐".as_bytes();
    sus signature = signer.sign(message)?;
    
    printf("✅ Signed message ({} bytes)\n", &[message.len()])?;
    printf("   📝 Signature size: {} bytes\n", &[signature.len()])?;
    
    // Verify signature
    sus is_valid = signer.verify(message, &signature)?;
    printf("✅ Signature verification: {}\n", &[is_valid])?;
    
    // Test with separate verifier
    sus public_key = PublicKey::from_keypair(&keypair);
    sus verifier = Ed25519Verifier::new(public_key)?;
    sus is_valid_external = verifier.verify(message, &signature)?;
    printf("✅ External verifier result: {}\n", &[is_valid_external])?;
    
    // Get statistics
    sus stats = signer.get_stats();
    printf("📊 Statistics: {} signatures, {} verifications\n", 
           &[stats.signatures_created, stats.signatures_verified])?;
}

// Demonstrate ECDSA signatures
func demo_ecdsa_signatures() -> yolo {
    println("\n🌐 ECDSA Digital Signatures Demo")?;
    println("-" * 40)?;
    
    facts curves = [EcdsaCurve::Secp256k1, EcdsaCurve::Secp256r1];
    
    for curve in curves {
        printf("Testing ECDSA with curve: {}\n", &[curve.name()])?;
        
        sus mut generator = KeyGenerator::new();
        sus key_type = match curve {
            EcdsaCurve::Secp256k1 => KeyType::EcdsaSecp256k1,
            EcdsaCurve::Secp256r1 => KeyType::EcdsaSecp256r1,
        };
        
        sus keypair = generator.generate_keypair(key_type)?;
        sus mut signer = EcdsaSigner::new(keypair.clone())?;
        
        sus message = format!("ECDSA test with {} curve", curve.name()).as_bytes();
        sus signature = signer.sign(message)?;
        
        printf("   ✅ Generated signature: {} bytes\n", &[signature.len()])?;
        
        sus is_valid = signer.verify(message, &signature)?;
        printf("   ✅ Verification result: {}\n", &[is_valid])?;
        
        sus stats = signer.get_stats();
        printf("   📊 Nonce generations: {}\n", &[stats.nonce_generations])?;
    }
}

// Demonstrate RSA signatures
func demo_rsa_signatures() -> yolo {
    println("\n🔒 RSA Digital Signatures Demo")?;
    println("-" * 40)?;
    
    facts schemes = [RsaSignatureScheme::Pss, RsaSignatureScheme::Pkcs1v15];
    facts key_sizes = [RsaKeySize::Bits2048, RsaKeySize::Bits3072];
    
    for scheme in schemes {
        for key_size in key_sizes {
            printf("Testing RSA {} with {}-bit keys\n", &[scheme.name(), key_size.bits()])?;
            
            sus mut generator = KeyGenerator::new();
            sus key_type = match (&scheme, &key_size) {
                (RsaSignatureScheme::Pss, RsaKeySize::Bits2048) => KeyType::RsaPss2048,
                (RsaSignatureScheme::Pss, RsaKeySize::Bits3072) => KeyType::RsaPss3072,
                (RsaSignatureScheme::Pkcs1v15, RsaKeySize::Bits2048) => KeyType::RsaPkcs1v15_2048,
                (RsaSignatureScheme::Pkcs1v15, RsaKeySize::Bits3072) => KeyType::RsaPkcs1v15_3072,
                _ => continue,
            };
            
            sus keypair = generator.generate_keypair(key_type)?;
            sus signer = RsaSigner::new(keypair.clone(), scheme.clone(), RsaHashAlgorithm::Sha256)?;
            
            sus message = format!("RSA {} test message", scheme.name()).as_bytes();
            sus signature = signer.sign(message)?;
            
            printf("   ✅ Signature size: {} bytes\n", &[signature.len()])?;
            
            sus is_valid = signer.verify(message, &signature)?;
            printf("   ✅ Verification: {}\n", &[is_valid])?;
            
            // Test max message size
            sus max_size = signer.calculate_max_message_size();
            printf("   📏 Max message size: {} bytes\n", &[max_size])?;
        }
    }
}

// Demonstrate universal signature interface
func demo_universal_interface() -> yolo {
    println("\n🌟 Universal Signature Interface Demo")?;
    println("-" * 40)?;
    
    sus mut generator = KeyGenerator::new();
    
    // Test different algorithms through universal interface
    facts key_types = [KeyType::Ed25519, KeyType::EcdsaSecp256k1, KeyType::RsaPss2048];
    
    for key_type in key_types {
        printf("Testing {} through universal interface\n", &[key_type.name()])?;
        
        sus keypair = generator.generate_keypair(key_type)?;
        sus universal_signer = UniversalSigner::new(keypair.clone())?;
        
        sus message = format!("Universal test for {}", key_type.name()).as_bytes();
        sus signature = universal_signer.sign(message)?;
        
        printf("   ✅ Algorithm: {}\n", &[universal_signer.algorithm_name()])?;
        printf("   📝 Signature size: {} bytes\n", &[signature.len()])?;
        
        sus is_valid = universal_signer.verify(message, &signature)?;
        printf("   ✅ Verification: {}\n", &[is_valid])?;
        
        // Test universal verifier
        sus public_key = PublicKey::from_keypair(&keypair);
        sus universal_verifier = UniversalVerifier::new(public_key)?;
        
        sus is_valid_external = universal_verifier.verify(message, &signature)?;
        printf("   ✅ Universal verifier: {}\n", &[is_valid_external])?;
    }
}

// Demonstrate key management
func demo_key_management() -> yolo {
    println("\n🔑 Key Management Demo")?;
    println("-" * 40)?;
    
    sus key_manager = KeyManager::new();
    
    // Generate and store different types of keys
    facts key_types = [
        KeyType::Ed25519,
        KeyType::EcdsaSecp256k1,
        KeyType::RsaPss2048,
    ];
    
    sus mut key_ids = Vec::new();
    
    for key_type in key_types {
        sus key_id = key_manager.generate_and_store(key_type, None)?;
        key_ids.push(key_id.clone());
        printf("✅ Generated and stored {} key: {}\n", &[key_type.name(), key_id])?;
    }
    
    printf("📊 Total keys stored: {}\n", &[key_manager.key_count()])?;
    
    // List all keys
    sus all_keys = key_manager.list_keys()?;
    println("📋 All stored keys:")?;
    for key_id in all_keys {
        printf("   🔑 {}\n", &[key_id])?;
    }
    
    // Retrieve and validate a key
    sus retrieved_key = key_manager.get_keypair(&key_ids[0])?;
    retrieved_key.validate()?;
    printf("✅ Retrieved and validated key: {}\n", &[retrieved_key.key_id])?;
    
    // Remove a key
    sus removed = key_manager.remove_key(&key_ids[0])?;
    printf("✅ Removed key: {}\n", &[removed])?;
    printf("📊 Keys remaining: {}\n", &[key_manager.key_count()])?;
}

// Demonstrate multi-signature schemes
func demo_multi_signatures() -> yolo {
    println("\n🤝 Multi-Signature Demo")?;
    println("-" * 40)?;
    
    // Create 2-of-3 threshold multi-signature
    sus config = MultiSigConfig::new(2, 3, MultiSigScheme::Threshold, MultiSigAlgorithm::Ed25519)?;
    sus mut multisig_signer = MultiSigSigner::new(config.clone())?;
    
    printf("Created {}-of-{} {} multi-signature\n", 
           &[config.threshold, config.total_signers, config.scheme.name()])?;
    
    // Generate keypairs for signers
    sus mut generator = KeyGenerator::new();
    sus mut keypairs = Vec::new();
    
    for i in 0..3 {
        sus keypair = generator.generate_keypair(KeyType::Ed25519)?;
        sus public_key = PublicKey::from_keypair(&keypair);
        sus signer_id = format!("signer-{}", i + 1);
        
        multisig_signer.add_signer(signer_id.clone(), public_key)?;
        keypairs.push(keypair);
        printf("✅ Added signer: {}\n", &[signer_id])?;
    }
    
    // Create multi-signature
    sus message = "Multi-signature test document".as_bytes();
    sus mut multisig = multisig_signer.create_multisig(message)?;
    
    printf("📝 Created multi-signature for message ({} bytes)\n", &[message.len()])?;
    printf("   🎯 Completion: {:.1}%\n", &[multisig.completion_percentage()])?;
    
    // Add signatures one by one
    for i in 0..2 {  // Only need 2 for threshold
        sus signer_id = format!("signer-{}", i + 1);
        multisig_signer.sign_with_keypair(&mut multisig, &signer_id, &keypairs[i], message)?;
        
        printf("✅ Added signature from {}\n", &[signer_id])?;
        printf("   🎯 Completion: {:.1}%\n", &[multisig.completion_percentage()])?;
        printf("   📊 Remaining needed: {}\n", &[multisig.remaining_needed()])?;
    }
    
    printf("✅ Multi-signature complete: {}\n", &[multisig.is_complete()])?;
    
    // Verify multi-signature
    sus is_valid = multisig_signer.verify_multisig(&multisig, message)?;
    printf("✅ Multi-signature verification: {}\n", &[is_valid])?;
    
    // Get final signature
    sus final_signature = multisig.get_signature()?;
    printf("📝 Final signature size: {} bytes\n", &[final_signature.len()])?;
    
    // Show statistics
    sus stats = multisig_signer.get_stats();
    printf("📊 Multi-sig stats: {} created, {} verified\n",
           &[stats.completed_multisigs, stats.verification_successes])?;
}

// Demonstrate batch verification
func demo_batch_verification() -> yolo {
    println("\n📦 Batch Verification Demo")?;
    println("-" * 40)?;
    
    sus mut batch_verifier = BatchVerifier::new();
    sus mut generator = KeyGenerator::new();
    
    // Generate multiple signatures for batch processing
    for i in 0..5 {
        sus keypair = generator.generate_keypair(KeyType::Ed25519)?;
        sus signer = Ed25519Signer::new(keypair.clone())?;
        
        sus message = format!("Batch message {}", i + 1);
        sus signature = signer.sign(message.as_bytes())?;
        
        sus public_key = PublicKey::from_keypair(&keypair);
        batch_verifier.add_verification(
            public_key,
            message.as_bytes(),
            &signature,
            Some(format!("batch-req-{}", i + 1))
        );
        
        printf("✅ Added signature {} to batch\n", &[i + 1])?;
    }
    
    printf("📦 Batch size: {} signatures\n", &[batch_verifier.pending_count()])?;
    
    // Process batch
    sus results = batch_verifier.verify_batch()?;
    
    printf("✅ Batch verification completed\n")?;
    printf("📊 Results: {} total\n", &[results.len()])?;
    
    for result in results {
        printf("   🔍 {}: {} ({}ms)\n", 
               &[result.request_id, result.is_valid, result.verification_time.as_millis()])?;
    }
    
    // Show batch statistics
    sus batch_stats = batch_verifier.get_stats();
    printf("📊 Batch stats: {} successful, {} failed\n",
           &[batch_stats.successful_verifications, batch_stats.failed_verifications])?;
}

// Demonstrate real-world scenarios
func demo_real_world_scenarios() -> yolo {
    println("\n🌍 Real-World Scenarios Demo")?;
    println("-" * 40)?;
    
    // Scenario 1: Document signing workflow
    println("📄 Scenario 1: Document Signing Workflow")?;
    demo_document_signing()?;
    
    // Scenario 2: API authentication
    println("\n🔗 Scenario 2: API Authentication")?;
    demo_api_authentication()?;
    
    // Scenario 3: Blockchain transaction signing
    println("\n⛓️ Scenario 3: Blockchain Transaction Signing")?;
    demo_blockchain_signing()?;
}

// Document signing workflow
func demo_document_signing() -> yolo {
    sus mut generator = KeyGenerator::new();
    sus keypair = generator.generate_keypair(KeyType::Ed25519)?;
    sus signer = Ed25519Signer::new(keypair.clone())?;
    
    // Simulate document content
    sus document = "CONFIDENTIAL CONTRACT\n\nThis agreement between parties...\n[Document content]";
    sus document_hash = document.as_bytes(); // In reality, this would be SHA-256
    
    printf("📄 Document size: {} bytes\n", &[document.len()])?;
    
    // Sign document
    sus signature = signer.sign(document_hash)?;
    printf("✅ Document signed\n")?;
    printf("   📝 Signature: {} bytes\n", &[signature.len()])?;
    printf("   🔑 Signer: {}\n", &[signer.key_id()])?;
    
    // Verify document signature
    sus is_valid = signer.verify(document_hash, &signature)?;
    printf("✅ Document verification: {}\n", &[is_valid])?;
    
    println("📋 Document signing workflow completed")?;
}

// API authentication
func demo_api_authentication() -> yolo {
    sus mut generator = KeyGenerator::new();
    sus keypair = generator.generate_keypair(KeyType::EcdsaSecp256k1)?;
    sus mut signer = EcdsaSigner::new(keypair)?;
    
    // Simulate API request
    sus api_endpoint = "/api/v1/users";
    sus timestamp = "1640995200";
    sus request_body = "{\"name\":\"Alice\",\"email\":\"alice@example.com\"}";
    
    // Create signature payload (method + endpoint + timestamp + body)
    sus payload = format!("POST{}{}{}",api_endpoint, timestamp, request_body);
    
    printf("🔗 API endpoint: {}\n", &[api_endpoint])?;
    printf("⏰ Timestamp: {}\n", &[timestamp])?;
    printf("📨 Request size: {} bytes\n", &[request_body.len()])?;
    
    // Sign API request
    sus signature = signer.sign(payload.as_bytes())?;
    printf("✅ API request signed\n")?;
    printf("   📝 Signature: {} bytes\n", &[signature.len()])?;
    
    // Verify API signature (server-side)
    sus is_valid = signer.verify(payload.as_bytes(), &signature)?;
    printf("✅ API signature verification: {}\n", &[is_valid])?;
    
    println("🔗 API authentication completed")?;
}

// Blockchain transaction signing
func demo_blockchain_signing() -> yolo {
    sus mut generator = KeyGenerator::new();
    sus keypair = generator.generate_keypair(KeyType::EcdsaSecp256k1)?;
    sus mut signer = EcdsaSigner::new(keypair)?;
    
    // Simulate blockchain transaction
    sus from_address = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
    sus to_address = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
    sus amount = "0.005";
    sus fee = "0.0001";
    sus nonce = "42";
    
    // Create transaction payload
    sus transaction = format!("{},{},{},{},{}", from_address, to_address, amount, fee, nonce);
    
    printf("⛓️ Transaction: {} -> {}\n", &[from_address, to_address])?;
    printf("💰 Amount: {} BTC\n", &[amount])?;
    printf("💸 Fee: {} BTC\n", &[fee])?;
    printf("🔢 Nonce: {}\n", &[nonce])?;
    
    // Sign transaction
    sus signature = signer.sign(transaction.as_bytes())?;
    printf("✅ Transaction signed\n")?;
    printf("   📝 Signature: {} bytes\n", &[signature.len()])?;
    printf("   🔑 Algorithm: {}\n", &[signer.curve().name()])?;
    
    // Verify transaction signature
    sus is_valid = signer.verify(transaction.as_bytes(), &signature)?;
    printf("✅ Transaction verification: {}\n", &[is_valid])?;
    
    println("⛓️ Blockchain transaction signing completed")?;
}
