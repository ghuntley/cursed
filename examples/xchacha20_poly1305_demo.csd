fr fr! XChaCha20-Poly1305 AEAD Demo
fr fr! 
fr fr! This example demonstrates the complete XChaCha20-Poly1305 AEAD functionality
fr fr! including basic encryption/decryption, streaming APIs, key derivation, and
fr fr! security best practices.

yeet "stdlib::crypto"

fr fr Basic encryption/decryption example
slay demo_basic_encryption() {
    println("🔐 XChaCha20-Poly1305 Basic Encryption Demo");
    
    // Generate a cryptographically secure key
    sus key = crypto::generate_xchacha20_key()?;
    println("✅ Generated XChaCha20 key");
    
    // Prepare data
    sus plaintext = "Hello, XChaCha20-Poly1305! This is a secure AEAD cipher.";
    sus associated_data = "Demo metadata - not encrypted but authenticated";
    
    println("📝 Plaintext: {}", plaintext);
    println("📋 Associated data: {}", associated_data);
    
    // Encrypt using high-level API
    sus (nonce, ciphertext) = crypto::xchacha20_encrypt(key, plaintext, associated_data)?;
    println("🔒 Encrypted {} bytes -> {} bytes (includes 16-byte auth tag)", 
             plaintext.len(), ciphertext.len());
    
    // Decrypt
    sus decrypted = crypto::xchacha20_decrypt(key, nonce, ciphertext, associated_data)?;
    println("🔓 Decrypted: {}", decrypted);
    
    // Verify round-trip
    lowkey (decrypted == plaintext) {
        println("✅ Encryption/decryption round-trip successful!");
    } flex {
        println("❌ Round-trip failed!");
        return Err("Decryption verification failed");
    }
    
    Ok(())
}

fr fr Streaming encryption for large files
slay demo_streaming_encryption() {
    println("\n📡 XChaCha20-Poly1305 Streaming Encryption Demo");
    
    sus key = crypto::generate_xchacha20_key()?;
    sus large_message = "This is a very long message that will be processed in chunks to demonstrate the streaming encryption capabilities of XChaCha20-Poly1305. ".repeat(100);
    
    println("📊 Processing {} bytes in streaming mode", large_message.len());
    
    // Create streaming encoder
    sus encoder = crypto::create_xchacha20_streaming_encoder(key)?;
    sus nonce = encoder.get_nonce();
    sus associated_data = "streaming demo";
    
    // Process in chunks
    sus chunk_size = 1024;
    sus encrypted_chunks = [];
    sus bytes_processed = 0;
    
    lowkey (bytes_processed < large_message.len()) {
        sus end_pos = min(bytes_processed + chunk_size, large_message.len());
        sus chunk = large_message.slice(bytes_processed, end_pos);
        
        sus encrypted_chunk = encoder.process_chunk(chunk, associated_data)?;
        encrypted_chunks.push(encrypted_chunk);
        
        bytes_processed = end_pos;
        
        lowkey (bytes_processed % (chunk_size * 10) == 0) {
            println("🔄 Processed {} / {} bytes", bytes_processed, large_message.len());
        }
    }
    
    println("✅ Streaming encryption completed");
    
    // Create streaming decoder
    sus decoder = crypto::create_xchacha20_streaming_decoder(key, nonce);
    sus decrypted_parts = [];
    
    bestie chunk in encrypted_chunks {
        sus decrypted_chunk = decoder.process_chunk(chunk, associated_data)?;
        decrypted_parts.push(decrypted_chunk);
    }
    
    sus reconstructed = decrypted_parts.join("");
    
    lowkey (reconstructed == large_message) {
        println("✅ Streaming decryption successful!");
        println("📊 Original: {} bytes, Reconstructed: {} bytes", 
                 large_message.len(), reconstructed.len());
    } flex {
        println("❌ Streaming decryption failed!");
        return Err("Streaming verification failed");
    }
    
    Ok(())
}

fr fr Key derivation demonstration
slay demo_key_derivation() {
    println("\n🔑 XChaCha20-Poly1305 Key Derivation Demo");
    
    // Shared secret (could be from key exchange)
    sus shared_secret = "shared_secret_from_key_exchange_12345";
    sus salt = "unique_application_salt_67890";
    sus context_info = "XChaCha20-Poly1305 Demo Application v1.0";
    
    println("🤝 Shared secret: {} bytes", shared_secret.len());
    println("🧂 Salt: {}", salt);
    println("ℹ️  Context: {}", context_info);
    
    // Derive encryption key
    sus encryption_key = crypto::derive_xchacha20_key(shared_secret, salt, context_info)?;
    println("✅ Derived encryption key");
    
    // Derive multiple keys for different purposes
    sus keys = crypto::derive_multiple_xchacha20_keys(shared_secret, salt, 3)?;
    println("✅ Derived {} keys for different purposes", keys.len());
    
    // Demonstrate that keys are different
    bestie i in 0..keys.len() {
        bestie j in (i+1)..keys.len() {
            lowkey (keys[i] != keys[j]) {
                println("✅ Key {} and {} are different (good!)", i, j);
            } flex {
                println("❌ Key collision detected!");
                return Err("Key derivation produced identical keys");
            }
        }
    }
    
    // Test key derivation determinism
    sus key1 = crypto::derive_xchacha20_key(shared_secret, salt, context_info)?;
    sus key2 = crypto::derive_xchacha20_key(shared_secret, salt, context_info)?;
    
    lowkey (key1 == key2) {
        println("✅ Key derivation is deterministic");
    } flex {
        println("❌ Key derivation is not deterministic!");
        return Err("Key derivation inconsistency");
    }
    
    Ok(())
}

fr fr Security best practices demonstration
slay demo_security_practices() {
    println("\n🛡️  XChaCha20-Poly1305 Security Best Practices Demo");
    
    sus key = crypto::generate_xchacha20_key()?;
    sus message = "Confidential financial data: Account 123456789, Balance: $50,000";
    sus metadata = "transaction_id:tx_789, timestamp:2024-06-14T10:30:00Z";
    
    println("💰 Original message: {}", message);
    
    // Encrypt
    sus (nonce, ciphertext) = crypto::xchacha20_encrypt(key, message, metadata)?;
    println("🔒 Encrypted with authentication");
    
    // Demonstrate nonce uniqueness (security critical!)
    sus nonces = [];
    bestie i in 0..100 {
        sus test_nonce = crypto::generate_xchacha20_nonce()?;
        lowkey (nonces.contains(test_nonce)) {
            println("❌ SECURITY ISSUE: Nonce collision detected!");
            return Err("Nonce reuse detected");
        }
        nonces.push(test_nonce);
    }
    println("✅ Verified nonce uniqueness (100 samples)");
    
    // Demonstrate tamper detection
    println("\n🔍 Testing tamper detection:");
    
    // Test 1: Tamper with ciphertext
    sus tampered_ciphertext = ciphertext.clone();
    tampered_ciphertext[0] = tampered_ciphertext[0] ^ 1; // Flip one bit
    
    sus tamper_result = crypto::xchacha20_decrypt(key, nonce, tampered_ciphertext, metadata);
    lowkey (tamper_result.is_err()) {
        println("✅ Ciphertext tampering detected and rejected");
    } flex {
        println("❌ SECURITY ISSUE: Tampered ciphertext accepted!");
        return Err("Tamper detection failed");
    }
    
    // Test 2: Wrong associated data
    sus wrong_metadata = "tampered_metadata";
    sus wrong_aad_result = crypto::xchacha20_decrypt(key, nonce, ciphertext, wrong_metadata);
    lowkey (wrong_aad_result.is_err()) {
        println("✅ Wrong associated data detected and rejected");
    } flex {
        println("❌ SECURITY ISSUE: Wrong associated data accepted!");
        return Err("Associated data verification failed");
    }
    
    // Test 3: Verify original data still decrypts correctly
    sus correct_decryption = crypto::xchacha20_decrypt(key, nonce, ciphertext, metadata)?;
    lowkey (correct_decryption == message) {
        println("✅ Original data still decrypts correctly");
    } flex {
        println("❌ Original decryption failed");
        return Err("Original data verification failed");
    }
    
    Ok(())
}

fr fr Performance demonstration
slay demo_performance() {
    println("\n⚡ XChaCha20-Poly1305 Performance Demo");
    
    sus key = crypto::generate_xchacha20_key()?;
    
    // Test different data sizes
    sus test_sizes = [1024, 10240, 102400, 1048576]; // 1KB, 10KB, 100KB, 1MB
    
    bestie size in test_sizes {
        sus test_data = "A".repeat(size);
        sus metadata = format!("performance_test_{}_bytes", size);
        
        sus start_time = time::now();
        
        // Encrypt
        sus (nonce, ciphertext) = crypto::xchacha20_encrypt(key, test_data, metadata)?;
        sus encrypt_time = time::now() - start_time;
        
        sus start_time = time::now();
        
        // Decrypt
        sus decrypted = crypto::xchacha20_decrypt(key, nonce, ciphertext, metadata)?;
        sus decrypt_time = time::now() - start_time;
        
        // Calculate throughput
        sus encrypt_mbps = (size as f64 / encrypt_time.as_secs_f64()) / (1024.0 * 1024.0);
        sus decrypt_mbps = (size as f64 / decrypt_time.as_secs_f64()) / (1024.0 * 1024.0);
        
        println("📊 {} bytes: Encrypt {:.2} MB/s, Decrypt {:.2} MB/s", 
                 size, encrypt_mbps, decrypt_mbps);
        
        // Verify correctness
        lowkey (decrypted == test_data) {
            println("✅ Data integrity verified for {} bytes", size);
        } flex {
            println("❌ Data integrity failed for {} bytes", size);
            return Err("Performance test data integrity failed");
        }
    }
    
    Ok(())
}

fr fr File encryption utility example
slay demo_file_encryption() {
    println("\n📁 XChaCha20-Poly1305 File Encryption Demo");
    
    // Simulate file encryption workflow
    sus key = crypto::generate_xchacha20_key()?;
    sus filename = "confidential_document.txt";
    sus file_content = "This is a confidential document containing sensitive information.\n".repeat(1000);
    
    println("📄 File: {} ({} bytes)", filename, file_content.len());
    
    // Create file metadata for authentication
    sus file_metadata = format!("filename:{},size:{},timestamp:{}", 
                               filename, file_content.len(), time::now());
    
    // Encrypt file
    println("🔒 Encrypting file...");
    sus (nonce, encrypted_content) = crypto::xchacha20_encrypt(key, file_content, file_metadata)?;
    
    // Simulate saving encrypted file with nonce
    sus encrypted_file_data = format!("XCHACHA20_NONCE:{}\nDATA:{}", 
                                     base64::encode(nonce.as_bytes()), 
                                     base64::encode(encrypted_content));
    
    println("💾 Encrypted file size: {} bytes", encrypted_file_data.len());
    
    // Simulate loading and decrypting file
    println("🔓 Decrypting file...");
    
    // Parse the saved format
    sus lines = encrypted_file_data.split('\n');
    sus nonce_line = lines[0];
    sus data_line = lines[1];
    
    sus nonce_base64 = nonce_line.replace("XCHACHA20_NONCE:", "");
    sus data_base64 = data_line.replace("DATA:", "");
    
    sus loaded_nonce = crypto::xchacha20_nonce_from_bytes(base64::decode(nonce_base64)?)?;
    sus loaded_encrypted = base64::decode(data_base64)?;
    
    // Decrypt
    sus decrypted_content = crypto::xchacha20_decrypt(key, loaded_nonce, loaded_encrypted, file_metadata)?;
    
    // Verify
    lowkey (decrypted_content == file_content) {
        println("✅ File encryption/decryption successful!");
        println("📊 Original: {} bytes, Decrypted: {} bytes", 
                 file_content.len(), decrypted_content.len());
    } flex {
        println("❌ File encryption/decryption failed!");
        return Err("File encryption verification failed");
    }
    
    Ok(())
}

fr fr Main demo function
slay main() {
    println("🚀 XChaCha20-Poly1305 AEAD Comprehensive Demo");
    println("================================================");
    
    // Run all demonstrations
    demo_basic_encryption()?;
    demo_streaming_encryption()?;
    demo_key_derivation()?;
    demo_security_practices()?;
    demo_performance()?;
    demo_file_encryption()?;
    
    println("\n🎉 All XChaCha20-Poly1305 demos completed successfully!");
    println("📚 Key takeaways:");
    println("   • XChaCha20-Poly1305 provides authenticated encryption (confidentiality + integrity)");
    println("   • 192-bit nonces provide better security than ChaCha20-Poly1305's 96-bit nonces");
    println("   • Always use unique nonces for each encryption operation");
    println("   • Associated data is authenticated but not encrypted");
    println("   • Streaming APIs enable efficient processing of large data");
    println("   • Key derivation allows generating multiple keys from shared secrets");
    println("   • Built-in tamper detection protects against modification attacks");
    
    Ok(())
}
