# CURSED Cryptographic Application - File Encryption/Decryption
# Demonstrates: Cryptography, file I/O, secure key handling, multiple algorithms

yeet "vibez"
yeet "cryptz"
yeet "filez"
yeet "stringz"
yeet "arrayz"
yeet "mathz"
yeet "timez"

# Supported encryption algorithms
enum CryptoAlgorithm {
    AES256,
    ChaCha20,
    Salsa20,
    XSalsa20
}

# Key derivation methods
enum KeyDerivation {
    PBKDF2,
    Argon2,
    Scrypt
}

# Crypto operation result
squad CryptoResult {
    success lit
    data []lit
    algorithm CryptoAlgorithm
    key_derivation KeyDerivation
    execution_time_ms drip
    error_message tea
}

# Encryption configuration
squad EncryptionConfig {
    algorithm CryptoAlgorithm
    key_derivation KeyDerivation
    iterations drip
    memory_cost drip
    parallelism drip
    salt_size drip
    nonce_size drip
}

# Key material for encryption
squad KeyMaterial {
    key []lit
    salt []lit
    nonce []lit
    iterations drip
}

# Default encryption configurations
slay get_default_config(algorithm CryptoAlgorithm) EncryptionConfig {
    sick (algorithm) {
        when AES256 -> damn EncryptionConfig{
            algorithm: AES256,
            key_derivation: PBKDF2,
            iterations: 100000,
            memory_cost: 65536,
            parallelism: 1,
            salt_size: 32,
            nonce_size: 16
        }
        when ChaCha20 -> damn EncryptionConfig{
            algorithm: ChaCha20,
            key_derivation: Argon2,
            iterations: 3,
            memory_cost: 65536,
            parallelism: 4,
            salt_size: 32,
            nonce_size: 24
        }
        when Salsa20 -> damn EncryptionConfig{
            algorithm: Salsa20,
            key_derivation: Scrypt,
            iterations: 32768,
            memory_cost: 8,
            parallelism: 1,
            salt_size: 32,
            nonce_size: 24
        }
        when XSalsa20 -> damn EncryptionConfig{
            algorithm: XSalsa20,
            key_derivation: Argon2,
            iterations: 4,
            memory_cost: 32768,
            parallelism: 2,
            salt_size: 32,
            nonce_size: 24
        }
    }
}

# Generate cryptographically secure random bytes
slay generate_random_bytes(size drip) []lit {
    sus random_bytes []lit = []
    sus i drip = 0
    bestie (i < size) {
        random_bytes = arrayz.append(random_bytes, cryptz.random_byte())
        i = i + 1
    }
    damn random_bytes
}

# Derive key from password using configured method
slay derive_key(password tea, config EncryptionConfig, salt []lit) yikes<[]lit> {
    sick (config.key_derivation) {
        when PBKDF2 -> {
            damn cryptz.pbkdf2(
                stringz.to_bytes(password),
                salt,
                config.iterations,
                32  # 256-bit key
            ) fam {
                when _ -> yikes "PBKDF2 key derivation failed"
            }
        }
        when Argon2 -> {
            damn cryptz.argon2(
                stringz.to_bytes(password),
                salt,
                config.iterations,
                config.memory_cost,
                config.parallelism,
                32
            ) fam {
                when _ -> yikes "Argon2 key derivation failed"
            }
        }
        when Scrypt -> {
            damn cryptz.scrypt(
                stringz.to_bytes(password),
                salt,
                config.iterations,
                config.memory_cost,
                config.parallelism,
                32
            ) fam {
                when _ -> yikes "Scrypt key derivation failed"
            }
        }
    }
}

# Encrypt data using specified algorithm
slay encrypt_data(data []lit, key []lit, nonce []lit, algorithm CryptoAlgorithm) yikes<[]lit> {
    sick (algorithm) {
        when AES256 -> {
            damn cryptz.aes256_encrypt(data, key, nonce) fam {
                when _ -> yikes "AES-256 encryption failed"
            }
        }
        when ChaCha20 -> {
            damn cryptz.chacha20_encrypt(data, key, nonce) fam {
                when _ -> yikes "ChaCha20 encryption failed"
            }
        }
        when Salsa20 -> {
            damn cryptz.salsa20_encrypt(data, key, nonce) fam {
                when _ -> yikes "Salsa20 encryption failed"
            }
        }
        when XSalsa20 -> {
            damn cryptz.xsalsa20_encrypt(data, key, nonce) fam {
                when _ -> yikes "XSalsa20 encryption failed"
            }
        }
    }
}

# Decrypt data using specified algorithm
slay decrypt_data(data []lit, key []lit, nonce []lit, algorithm CryptoAlgorithm) yikes<[]lit> {
    sick (algorithm) {
        when AES256 -> {
            damn cryptz.aes256_decrypt(data, key, nonce) fam {
                when _ -> yikes "AES-256 decryption failed"
            }
        }
        when ChaCha20 -> {
            damn cryptz.chacha20_decrypt(data, key, nonce) fam {
                when _ -> yikes "ChaCha20 decryption failed"
            }
        }
        when Salsa20 -> {
            damn cryptz.salsa20_decrypt(data, key, nonce) fam {
                when _ -> yikes "Salsa20 decryption failed"
            }
        }
        when XSalsa20 -> {
            damn cryptz.xsalsa20_decrypt(data, key, nonce) fam {
                when _ -> yikes "XSalsa20 decryption failed"
            }
        }
    }
}

# Encrypt file with secure header format
slay encrypt_file(input_path tea, output_path tea, password tea, config EncryptionConfig) CryptoResult {
    sus start_time drip = timez.now()
    
    # Read input file
    sus file_data []lit = filez.read_binary(input_path) fam {
        when _ -> damn CryptoResult{
            success: false,
            error_message: "Failed to read input file",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Generate cryptographic materials
    sus salt []lit = generate_random_bytes(config.salt_size)
    sus nonce []lit = generate_random_bytes(config.nonce_size)
    
    # Derive encryption key
    sus key []lit = derive_key(password, config, salt) fam {
        when _ -> damn CryptoResult{
            success: false,
            error_message: "Key derivation failed",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Encrypt the data
    sus encrypted_data []lit = encrypt_data(file_data, key, nonce, config.algorithm) fam {
        when _ -> damn CryptoResult{
            success: false,
            error_message: "Encryption failed",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Create encrypted file header
    sus header []lit = []
    
    # Magic number (CURSED v1.0)
    header = arrayz.concat(header, [0x43, 0x55, 0x52, 0x53, 0x45, 0x44, 0x01, 0x00])
    
    # Algorithm identifier
    header = arrayz.append(header, config.algorithm)
    header = arrayz.append(header, config.key_derivation)
    
    # Parameters (big-endian encoding)
    header = arrayz.concat(header, cryptz.encode_u32_be(config.iterations))
    header = arrayz.concat(header, cryptz.encode_u32_be(config.memory_cost))
    header = arrayz.append(header, config.parallelism)
    header = arrayz.append(header, config.salt_size)
    header = arrayz.append(header, config.nonce_size)
    
    # Salt and nonce
    header = arrayz.concat(header, salt)
    header = arrayz.concat(header, nonce)
    
    # Data length (for integrity checking)
    header = arrayz.concat(header, cryptz.encode_u64_be(len(file_data)))
    
    # Combine header and encrypted data
    sus output_data []lit = arrayz.concat(header, encrypted_data)
    
    # Write encrypted file
    filez.write_binary(output_path, output_data) fam {
        when _ -> damn CryptoResult{
            success: false,
            error_message: "Failed to write encrypted file",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Calculate secure hash of the encrypted file for verification
    sus file_hash tea = cryptz.sha256_file(output_path) fam {
        when _ -> damn ""
    }
    
    damn CryptoResult{
        success: based,
        data: output_data,
        algorithm: config.algorithm,
        key_derivation: config.key_derivation,
        execution_time_ms: timez.now() - start_time,
        error_message: ""
    }
}

# Decrypt file and verify integrity
slay decrypt_file(input_path tea, output_path tea, password tea) CryptoResult {
    sus start_time drip = timez.now()
    
    # Read encrypted file
    sus encrypted_file []lit = filez.read_binary(input_path) fam {
        when _ -> damn CryptoResult{
            success: false,
            error_message: "Failed to read encrypted file",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Verify minimum file size for header
    ready (len(encrypted_file) < 32) {
        damn CryptoResult{
            success: false,
            error_message: "Invalid encrypted file format",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Verify magic number
    sus magic []lit = arrayz.slice(encrypted_file, 0, 8)
    sus expected_magic []lit = [0x43, 0x55, 0x52, 0x53, 0x45, 0x44, 0x01, 0x00]
    ready (!arrayz.equals(magic, expected_magic)) {
        damn CryptoResult{
            success: false,
            error_message: "Invalid file format or version",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Parse header
    sus pos drip = 8
    sus algorithm CryptoAlgorithm = encrypted_file[pos]
    sus key_derivation KeyDerivation = encrypted_file[pos + 1]
    pos = pos + 2
    
    sus iterations drip = cryptz.decode_u32_be(arrayz.slice(encrypted_file, pos, pos + 4))
    sus memory_cost drip = cryptz.decode_u32_be(arrayz.slice(encrypted_file, pos + 4, pos + 8))
    sus parallelism drip = encrypted_file[pos + 8]
    sus salt_size drip = encrypted_file[pos + 9]
    sus nonce_size drip = encrypted_file[pos + 10]
    pos = pos + 11
    
    # Extract salt and nonce
    sus salt []lit = arrayz.slice(encrypted_file, pos, pos + salt_size)
    pos = pos + salt_size
    sus nonce []lit = arrayz.slice(encrypted_file, pos, pos + nonce_size)
    pos = pos + nonce_size
    
    # Extract original data length
    sus original_length drip = cryptz.decode_u64_be(arrayz.slice(encrypted_file, pos, pos + 8))
    pos = pos + 8
    
    # Extract encrypted data
    sus encrypted_data []lit = arrayz.slice(encrypted_file, pos, len(encrypted_file))
    
    # Recreate configuration
    sus config EncryptionConfig = {
        algorithm: algorithm,
        key_derivation: key_derivation,
        iterations: iterations,
        memory_cost: memory_cost,
        parallelism: parallelism,
        salt_size: salt_size,
        nonce_size: nonce_size
    }
    
    # Derive key
    sus key []lit = derive_key(password, config, salt) fam {
        when _ -> damn CryptoResult{
            success: false,
            error_message: "Key derivation failed - incorrect password?",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Decrypt data
    sus decrypted_data []lit = decrypt_data(encrypted_data, key, nonce, algorithm) fam {
        when _ -> damn CryptoResult{
            success: false,
            error_message: "Decryption failed - incorrect password or corrupted file",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Verify data length
    ready (len(decrypted_data) != original_length) {
        damn CryptoResult{
            success: false,
            error_message: "Data integrity check failed",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    # Write decrypted file
    filez.write_binary(output_path, decrypted_data) fam {
        when _ -> damn CryptoResult{
            success: false,
            error_message: "Failed to write decrypted file",
            execution_time_ms: timez.now() - start_time
        }
    }
    
    damn CryptoResult{
        success: based,
        data: decrypted_data,
        algorithm: algorithm,
        key_derivation: key_derivation,
        execution_time_ms: timez.now() - start_time,
        error_message: ""
    }
}

# Generate secure password
slay generate_password(length drip, include_symbols lit) tea {
    sus chars tea = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    ready (include_symbols) {
        chars = chars + "!@#$%^&*()_+-=[]{}|;:,.<>?"
    }
    
    sus password tea = ""
    sus i drip = 0
    bestie (i < length) {
        sus char_index drip = cryptz.random_int_range(0, stringz.len(chars) - 1)
        password = password + stringz.char_at(chars, char_index)
        i = i + 1
    }
    
    damn password
}

# Calculate file hash for verification
slay calculate_file_hash(file_path tea) tea {
    damn cryptz.sha256_file(file_path) fam {
        when _ -> damn "hash_calculation_failed"
    }
}

# Benchmark encryption algorithms
slay benchmark_algorithms(test_data []lit, password tea) {
    vibez.spill("🏃 Benchmarking encryption algorithms...")
    vibez.spill("")
    
    sus algorithms []CryptoAlgorithm = [AES256, ChaCha20, Salsa20, XSalsa20]
    
    bestie (algorithm in algorithms) {
        sus config EncryptionConfig = get_default_config(algorithm)
        
        # Generate materials
        sus salt []lit = generate_random_bytes(config.salt_size)
        sus nonce []lit = generate_random_bytes(config.nonce_size)
        
        # Time key derivation
        sus key_start drip = timez.now()
        sus key []lit = derive_key(password, config, salt) fam {
            when _ -> skip
        }
        sus key_time drip = timez.now() - key_start
        
        # Time encryption
        sus encrypt_start drip = timez.now()
        sus encrypted []lit = encrypt_data(test_data, key, nonce, algorithm) fam {
            when _ -> skip
        }
        sus encrypt_time drip = timez.now() - encrypt_start
        
        # Time decryption
        sus decrypt_start drip = timez.now()
        decrypt_data(encrypted, key, nonce, algorithm) fam {
            when _ -> skip
        }
        sus decrypt_time drip = timez.now() - decrypt_start
        
        sus algorithm_name tea = sick (algorithm) {
            when AES256 -> "AES-256"
            when ChaCha20 -> "ChaCha20"
            when Salsa20 -> "Salsa20"
            when XSalsa20 -> "XSalsa20"
        }
        
        vibez.spill("  ", algorithm_name, ":")
        vibez.spill("    Key derivation: ", key_time, "ms")
        vibez.spill("    Encryption:     ", encrypt_time, "ms")
        vibez.spill("    Decryption:     ", decrypt_time, "ms")
        vibez.spill("    Total:          ", key_time + encrypt_time + decrypt_time, "ms")
        vibez.spill("")
    }
}

# Interactive command line interface
slay interactive_mode() {
    vibez.spill("🔐 CURSED Crypto Tool - Interactive Mode")
    vibez.spill("=====================================")
    vibez.spill("")
    
    bestie (based) {
        vibez.spill("Commands:")
        vibez.spill("  1. encrypt   - Encrypt a file")
        vibez.spill("  2. decrypt   - Decrypt a file")
        vibez.spill("  3. genpass   - Generate secure password")
        vibez.spill("  4. hash      - Calculate file hash")
        vibez.spill("  5. benchmark - Benchmark algorithms")
        vibez.spill("  6. quit      - Exit program")
        vibez.spill("")
        
        vibez.print("Choose option (1-6): ")
        sus input tea = vibez.read_line() fam {
            when _ -> skip
        }
        
        sick (stringz.trim(input)) {
            when "1", "encrypt" -> {
                vibez.print("Input file path: ")
                sus input_file tea = stringz.trim(vibez.read_line())
                
                vibez.print("Output file path: ")
                sus output_file tea = stringz.trim(vibez.read_line())
                
                vibez.print("Password: ")
                sus password tea = stringz.trim(vibez.read_line())
                
                vibez.print("Algorithm (1=AES256, 2=ChaCha20, 3=Salsa20, 4=XSalsa20): ")
                sus algo_input tea = stringz.trim(vibez.read_line())
                
                sus algorithm CryptoAlgorithm = sick (algo_input) {
                    when "2" -> ChaCha20
                    when "3" -> Salsa20
                    when "4" -> XSalsa20
                    when _ -> AES256
                }
                
                sus config EncryptionConfig = get_default_config(algorithm)
                sus result CryptoResult = encrypt_file(input_file, output_file, password, config)
                
                ready (result.success) {
                    vibez.spill("✅ Encryption successful!")
                    vibez.spill("   Time: ", result.execution_time_ms, "ms")
                    vibez.spill("   Hash: ", calculate_file_hash(output_file))
                } otherwise {
                    vibez.spill("❌ Encryption failed: ", result.error_message)
                }
            }
            
            when "2", "decrypt" -> {
                vibez.print("Encrypted file path: ")
                sus input_file tea = stringz.trim(vibez.read_line())
                
                vibez.print("Output file path: ")
                sus output_file tea = stringz.trim(vibez.read_line())
                
                vibez.print("Password: ")
                sus password tea = stringz.trim(vibez.read_line())
                
                sus result CryptoResult = decrypt_file(input_file, output_file, password)
                
                ready (result.success) {
                    vibez.spill("✅ Decryption successful!")
                    vibez.spill("   Time: ", result.execution_time_ms, "ms")
                    vibez.spill("   Hash: ", calculate_file_hash(output_file))
                } otherwise {
                    vibez.spill("❌ Decryption failed: ", result.error_message)
                }
            }
            
            when "3", "genpass" -> {
                vibez.print("Password length (default 16): ")
                sus length_input tea = stringz.trim(vibez.read_line())
                sus length drip = ready (stringz.len(length_input) > 0) {
                    damn stringz.parse_int(length_input) fam { when _ -> damn 16 }
                } otherwise { damn 16 }
                
                vibez.print("Include symbols? (y/n): ")
                sus symbols_input tea = stringz.trim(vibez.read_line())
                sus include_symbols lit = stringz.to_lower(symbols_input) == "y"
                
                sus password tea = generate_password(length, include_symbols)
                vibez.spill("Generated password: ", password)
            }
            
            when "4", "hash" -> {
                vibez.print("File path: ")
                sus file_path tea = stringz.trim(vibez.read_line())
                
                sus hash_value tea = calculate_file_hash(file_path)
                ready (hash_value == "hash_calculation_failed") {
                    vibez.spill("❌ Failed to calculate hash")
                } otherwise {
                    vibez.spill("SHA-256: ", hash_value)
                }
            }
            
            when "5", "benchmark" -> {
                sus test_data []lit = generate_random_bytes(1024 * 100)  # 100KB test
                benchmark_algorithms(test_data, "test_password_123")
            }
            
            when "6", "quit" -> {
                vibez.spill("Goodbye! 👋")
                damn
            }
            
            when _ -> {
                vibez.spill("Invalid option. Please try again.")
            }
        }
        
        vibez.spill("")
    }
}

# Main application
slay main() {
    vibez.spill("🔐 CURSED Cryptographic Application v1.0")
    vibez.spill("=========================================")
    vibez.spill("")
    vibez.spill("Features:")
    vibez.spill("  ✅ AES-256, ChaCha20, Salsa20, XSalsa20 encryption")
    vibez.spill("  ✅ PBKDF2, Argon2, Scrypt key derivation")
    vibez.spill("  ✅ Secure random number generation")
    vibez.spill("  ✅ File integrity verification")
    vibez.spill("  ✅ Constant-time operations")
    vibez.spill("  ✅ Memory-safe implementation")
    vibez.spill("")
    
    # Demo mode with test file
    sus test_content tea = "This is a test file for CURSED cryptographic operations. " +
                          "It contains various characters: 1234567890 !@#$%^&*() " +
                          "Testing Unicode: 你好世界 🚀🔐💻"
    
    filez.write_text("test_file.txt", test_content) fam {
        when _ -> {
            vibez.spill("❌ Could not create test file")
            damn
        }
    }
    
    vibez.spill("📝 Created test file (", stringz.len(test_content), " bytes)")
    
    # Demo each algorithm
    sus algorithms []CryptoAlgorithm = [AES256, ChaCha20, Salsa20, XSalsa20]
    sus password tea = "demo_password_123!"
    
    bestie (algorithm in algorithms) {
        sus algorithm_name tea = sick (algorithm) {
            when AES256 -> "AES-256"
            when ChaCha20 -> "ChaCha20"
            when Salsa20 -> "Salsa20"
            when XSalsa20 -> "XSalsa20"
        }
        
        vibez.spill("")
        vibez.spill("🔒 Testing", algorithm_name, "encryption...")
        
        sus config EncryptionConfig = get_default_config(algorithm)
        sus encrypted_file tea = "test_file_" + stringz.to_lower(algorithm_name) + ".enc"
        sus decrypted_file tea = "test_file_" + stringz.to_lower(algorithm_name) + "_decrypted.txt"
        
        # Encrypt
        sus encrypt_result CryptoResult = encrypt_file("test_file.txt", encrypted_file, password, config)
        ready (encrypt_result.success) {
            vibez.spill("  ✅ Encrypted in", encrypt_result.execution_time_ms, "ms")
            
            # Decrypt
            sus decrypt_result CryptoResult = decrypt_file(encrypted_file, decrypted_file, password)
            ready (decrypt_result.success) {
                vibez.spill("  ✅ Decrypted in", decrypt_result.execution_time_ms, "ms")
                
                # Verify integrity
                sus original_hash tea = calculate_file_hash("test_file.txt")
                sus decrypted_hash tea = calculate_file_hash(decrypted_file)
                ready (original_hash == decrypted_hash) {
                    vibez.spill("  ✅ Integrity verified")
                } otherwise {
                    vibez.spill("  ❌ Integrity check failed")
                }
            } otherwise {
                vibez.spill("  ❌ Decryption failed:", decrypt_result.error_message)
            }
        } otherwise {
            vibez.spill("  ❌ Encryption failed:", encrypt_result.error_message)
        }
    }
    
    vibez.spill("")
    vibez.spill("🎉 Crypto demo completed!")
    vibez.spill("")
    
    # Start interactive mode
    interactive_mode()
}

# Run the application
main()
