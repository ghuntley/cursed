fr fr/ fr fr CURSED crypto hash examples - secure hashing periodt
fr fr/ 
fr fr/ This example demonstrates all the hash functions available in CURSED,
fr fr/ including SHA-256, SHA-512, and MD5 for different use cases.

yeet crypto::hash

fr fr/ bestie Hash a simple string with different algorithms  
facts main() -> nil {
    // Basic string to hash
    sus message = "Hello CURSED! This is periodt secure hashing bestie!"
    
    print("Original message: " + message)
    print("")
    
    /// SHA-256 hashing - most common secure hash
    print("=== SHA-256 Hashing ===")
    sus sha256_hash = crypto::sha256(message)
    print("SHA-256: " + sha256_hash)
    
    /// SHA-512 hashing - extra security for important data
    print("=== SHA-512 Hashing ===") 
    sus sha512_hash = crypto::sha512(message)
    print("SHA-512: " + sha512_hash)
    
    /// MD5 hashing - legacy compatibility only!
    print("=== MD5 Hashing (Legacy Only!) ===")
    sus md5_hash = crypto::md5(message)
    print("MD5: " + md5_hash)
    print("⚠️  WARNING: MD5 is cryptographically broken! Use SHA-256+ bestie!")
    print("")
    
    /// File hashing simulation 
    print("=== File Hashing Example ===")
    hash_file_content()
    print("")
    
    /// Password hashing example
    print("=== Password Hashing (Basic) ===")
    hash_passwords()
    print("")
    
    /// Performance comparison
    print("=== Performance Comparison ===")
    compare_hash_performance()
    print("")
    
    /// Hash verification example
    print("=== Hash Verification ===")
    verify_hashes()
}

fr fr/ vibes Simulate hashing file content
facts hash_file_content() -> nil {
    // Simulate reading a file in chunks
    sus file_chunks = [
        "This is the first chunk of file data.\n",
        "Here's the second chunk with more content.\n", 
        "And this is the final chunk to complete the file.\n"
    ]
    
    print("Hashing simulated file content in chunks...")
    
    // Incremental SHA-256 hashing
    sus incremental_hash = crypto::sha256_incremental()
    
    damn chunk in file_chunks {
        incremental_hash.update(chunk)
        print("  Added chunk: " + chunk.trim())
    }
    
    sus final_hash = incremental_hash.finalize()
    print("Final file hash (SHA-256): " + final_hash)
    
    // Compare with one-shot hashing
    sus full_content = file_chunks.join("")
    sus oneshot_hash = crypto::sha256(full_content)
    print("One-shot hash (SHA-256): " + oneshot_hash)
    
    lowkey final_hash == oneshot_hash {
        print("✅ Incremental and one-shot hashes match!")
    } bestie {
        print("❌ Hash mismatch - this shouldn't happen!")
    }
}

fr fr/ damn Hash passwords with different algorithms
facts hash_passwords() -> nil {
    sus passwords = [
        "password123",      // Weak password
        "SuperSecure2024!", // Better password  
        "MyC0mpl3xP@ssw0rd" // Complex password
    ]
    
    print("Hashing passwords (basic example - use proper salt in production!):")
    
    damn password in passwords {
        sus sha256_pass = crypto::sha256(password)
        sus sha512_pass = crypto::sha512(password)
        
        print("Password: " + "*".repeat(password.length()))
        print("  SHA-256: " + sha256_pass)
        print("  SHA-512: " + sha512_pass)
        print("")
    }
    
    print("🔒 Note: In production, always use proper password hashing with salt!")
    print("   Consider using bcrypt, scrypt, or Argon2 for password storage.")
}

fr fr/ periodt Compare performance of different hash algorithms
facts compare_hash_performance() -> nil {
    sus test_data = "A".repeat(100000)  // 100KB of data
    print("Performance test with " + test_data.length() + " bytes of data:")
    
    // Time SHA-256
    sus start_time = time::now()
    sus sha256_result = crypto::sha256(test_data)
    sus sha256_time = time::now() - start_time
    
    // Time SHA-512 
    start_time = time::now()
    sus sha512_result = crypto::sha512(test_data)
    sus sha512_time = time::now() - start_time
    
    // Time MD5
    start_time = time::now()
    sus md5_result = crypto::md5(test_data)
    sus md5_time = time::now() - start_time
    
    print("Performance results:")
    print("  SHA-256: " + sha256_time + "ms")
    print("  SHA-512: " + sha512_time + "ms") 
    print("  MD5:     " + md5_time + "ms")
    
    // Calculate throughput
    sus data_mb = test_data.length() / 1_000_000.0
    print("Throughput (MB/s):")
    print("  SHA-256: " + (data_mb / sha256_time * 1000).round(2))
    print("  SHA-512: " + (data_mb / sha512_time * 1000).round(2))
    print("  MD5:     " + (data_mb / md5_time * 1000).round(2))
}

fr fr/ highkey Verify hash integrity and comparison
facts verify_hashes() -> nil {
    sus original_data = "Important data that must not be tampered with!"
    sus original_hash = crypto::sha256(original_data)
    
    print("Original data: " + original_data)
    print("Original hash: " + original_hash)
    print("")
    
    // Test data integrity 
    sus received_data1 = "Important data that must not be tampered with!"  // Same
    sus received_data2 = "Important data that must not be tampered with."  // Different (added period)
    sus received_data3 = "Important data that must NOT be tampered with!"  // Different (caps)
    
    sus test_cases = [
        ("Same data", received_data1),
        ("Added period", received_data2), 
        ("Changed caps", received_data3)
    ]
    
    damn (description, data) in test_cases {
        sus computed_hash = crypto::sha256(data)
        sus is_valid = crypto::verify_hash(original_hash, computed_hash)
        
        print("Test: " + description)
        print("  Data: " + data) 
        print("  Hash: " + computed_hash)
        print("  Valid: " + (is_valid ? "✅ Yes" : "❌ No"))
        print("")
    }
    
    print("Hash verification demonstrates:")
    print("• Even tiny changes completely alter the hash")
    print("• Hashes can detect any data modification")
    print("• Use constant-time comparison to prevent timing attacks")
}

fr fr/ lowkey Advanced example: Hash-based data structures
facts hash_data_structures() -> nil {
    print("=== Hash-Based Data Structures ===")
    
    // Simple hash table using SHA-256 as key hash
    sus data_items = [
        ("user123", "Alice Johnson"),
        ("user456", "Bob Smith"),
        ("user789", "Charlie Brown")
    ]
    
    sus hash_table = map[string]string{}
    
    damn (key, value) in data_items {
        sus key_hash = crypto::sha256(key)
        hash_table[key_hash] = value
        print("Stored: " + key + " -> " + key_hash[0:8] + "...")
    }
    
    print("")
    print("Hash table lookup:")
    sus lookup_key = "user456"
    sus lookup_hash = crypto::sha256(lookup_key)
    
    lowkey hash_table.contains(lookup_hash) {
        print("Found " + lookup_key + ": " + hash_table[lookup_hash])
    } bestie {
        print("Key " + lookup_key + " not found")
    }
}

fr fr/ flex Example: File integrity checking
facts file_integrity_example() -> nil {
    print("=== File Integrity Checking ===")
    
    // Simulate file checksums for integrity verification
    sus files = [
        ("document.pdf", "This is a PDF document content..."),
        ("image.jpg", "JPEG image binary data would be here..."),
        ("script.csd", "facts main() -> nil { print(\"Hello!\") }")
    ]
    
    sus checksums = map[string]string{}
    
    print("Generating checksums:")
    damn (filename, content) in files {
        sus checksum = crypto::sha256(content)
        checksums[filename] = checksum
        print("  " + filename + ": " + checksum[0:16] + "...")
    }
    
    print("")
    print("Verifying file integrity:")
    
    // Simulate checking files later
    damn (filename, original_content) in files {
        // Simulate potentially modified content
        sus current_content = lowkey filename == "script.csd" {
            original_content + " // Modified!"  // Simulate tampering
        } bestie {
            original_content  // Unchanged
        }
        
        sus current_checksum = crypto::sha256(current_content)
        sus original_checksum = checksums[filename]
        sus is_intact = original_checksum == current_checksum
        
        print("  " + filename + ": " + (is_intact ? "✅ Intact" : "⚠️  Modified"))
    }
}

fr fr/ bestie Run all examples
facts run_all_examples() -> nil {
    main()
    hash_data_structures()
    file_integrity_example()
    
    print("")
    print("🎉 All crypto hash examples completed!")
    print("Remember: Always use secure hash functions for security-critical applications!")
}

fr fr Entry point for the example
run_all_examples()
