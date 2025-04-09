vibe main

yeet "vibez"  fr fr For printing results
yeet "cryptz" fr fr Cryptography functions

slay main() {
    vibez.spill("Testing cryptz package")
    
    fr fr Test hash functions
    test_hash_functions()
    
    fr fr Test HMAC
    test_hmac()
    
    fr fr Test password hashing
    test_password_hashing()
    
    fr fr Test encryption and decryption
    test_encryption()
    
    vibez.spill("All cryptz tests passed!")
}

fr fr Test hash functions
slay test_hash_functions() {
    vibez.spill("Testing hash functions...")
    
    tea testData := "Hello, cryptz!"
    
    fr fr Test MD5
    tea md5Hash := cryptz.MD5(testData)
    vibez.spill("MD5:", md5Hash)
    lowkey len(md5Hash) != 32 {
        vibez.spill("Invalid MD5 hash length. Expected 32, got", len(md5Hash))
        yolo
    }
    
    fr fr Test SHA-1
    tea sha1Hash := cryptz.SHA1(testData)
    vibez.spill("SHA-1:", sha1Hash)
    lowkey len(sha1Hash) != 40 {
        vibez.spill("Invalid SHA-1 hash length. Expected 40, got", len(sha1Hash))
        yolo
    }
    
    fr fr Test SHA-256
    tea sha256Hash := cryptz.SHA256(testData)
    vibez.spill("SHA-256:", sha256Hash)
    lowkey len(sha256Hash) != 64 {
        vibez.spill("Invalid SHA-256 hash length. Expected 64, got", len(sha256Hash))
        yolo
    }
    
    fr fr Test SHA-512
    tea sha512Hash := cryptz.SHA512(testData)
    vibez.spill("SHA-512:", sha512Hash)
    lowkey len(sha512Hash) != 128 {
        vibez.spill("Invalid SHA-512 hash length. Expected 128, got", len(sha512Hash))
        yolo
    }
    
    fr fr Ensure deterministic results
    tea md5Hash2 := cryptz.MD5(testData)
    lowkey md5Hash != md5Hash2 {
        vibez.spill("MD5 hash not deterministic!")
        yolo
    }
    
    fr fr Different data should produce different hashes
    tea md5Hash3 := cryptz.MD5(testData + "X")
    lowkey md5Hash == md5Hash3 {
        vibez.spill("MD5 hash collision detected!")
        yolo
    }
    
    vibez.spill("Hash function tests passed!")
}

fr fr Test HMAC
slay test_hmac() {
    vibez.spill("Testing HMAC functions...")
    
    tea testData := "HMAC test message"
    tea testKey := "secret-key-1234"
    
    fr fr Test HMAC-MD5
    tea hmacMD5 := cryptz.HmacMD5(testKey, testData)
    vibez.spill("HMAC-MD5:", hmacMD5)
    lowkey len(hmacMD5) != 32 {
        vibez.spill("Invalid HMAC-MD5 length. Expected 32, got", len(hmacMD5))
        yolo
    }
    
    fr fr Test HMAC-SHA1
    tea hmacSHA1 := cryptz.HmacSHA1(testKey, testData)
    vibez.spill("HMAC-SHA1:", hmacSHA1)
    lowkey len(hmacSHA1) != 40 {
        vibez.spill("Invalid HMAC-SHA1 length. Expected 40, got", len(hmacSHA1))
        yolo
    }
    
    fr fr Test HMAC-SHA256
    tea hmacSHA256 := cryptz.HmacSHA256(testKey, testData)
    vibez.spill("HMAC-SHA256:", hmacSHA256)
    lowkey len(hmacSHA256) != 64 {
        vibez.spill("Invalid HMAC-SHA256 length. Expected 64, got", len(hmacSHA256))
        yolo
    }
    
    fr fr Different keys should produce different HMACs
    tea hmacSHA256_2 := cryptz.HmacSHA256("different-key", testData)
    lowkey hmacSHA256 == hmacSHA256_2 {
        vibez.spill("HMAC key sensitivity failed!")
        yolo
    }
    
    fr fr Different messages should produce different HMACs
    tea hmacSHA256_3 := cryptz.HmacSHA256(testKey, testData + "X")
    lowkey hmacSHA256 == hmacSHA256_3 {
        vibez.spill("HMAC message sensitivity failed!")
        yolo
    }
    
    fr fr Same inputs should produce same HMACs (deterministic)
    tea hmacSHA256_4 := cryptz.HmacSHA256(testKey, testData)
    lowkey hmacSHA256 != hmacSHA256_4 {
        vibez.spill("HMAC not deterministic!")
        yolo
    }
    
    vibez.spill("HMAC tests passed!")
}

fr fr Test password hashing
slay test_password_hashing() {
    vibez.spill("Testing password hashing...")
    
    tea password := "secure-password-123"
    
    fr fr Generate password hash
    tea hash, err := cryptz.HashPassword(password)
    lowkey err != cap {
        vibez.spill("Error hashing password:", err)
        yolo
    }
    
    vibez.spill("Password hash:", hash)
    lowkey len(hash) < 20 {
        vibez.spill("Invalid password hash length. Got", len(hash))
        yolo
    }
    
    fr fr Verify password
    tea valid := cryptz.CheckPassword(password, hash)
    lowkey !valid {
        vibez.spill("Password verification failed!")
        yolo
    }
    
    fr fr Wrong password should fail
    tea invalid := cryptz.CheckPassword("wrong-password", hash)
    lowkey invalid {
        vibez.spill("Wrong password verification succeeded when it should fail!")
        yolo
    }
    
    fr fr Generate a new hash for the same password
    tea hash2, _ := cryptz.HashPassword(password)
    
    fr fr Two hashes of the same password should be different
    lowkey hash == hash2 {
        vibez.spill("Password hashing salt not working!")
        yolo
    }
    
    fr fr Both hashes should verify correctly
    tea valid2 := cryptz.CheckPassword(password, hash2)
    lowkey !valid2 {
        vibez.spill("Second password hash verification failed!")
        yolo
    }
    
    vibez.spill("Password hashing tests passed!")
}

fr fr Test encryption and decryption
slay test_encryption() {
    vibez.spill("Testing encryption and decryption...")
    
    tea plaintext := "Secret message to be encrypted"
    tea key := "encryption-key-01234567890123456789012345678901"  fr fr 32 bytes key for AES-256
    
    fr fr Encrypt
    tea ciphertext, err := cryptz.Encrypt(plaintext, key)
    lowkey err != cap {
        vibez.spill("Encryption error:", err)
        yolo
    }
    
    vibez.spill("Ciphertext:", ciphertext)
    lowkey ciphertext == plaintext {
        vibez.spill("Encryption did not change the plaintext!")
        yolo
    }
    
    fr fr Decrypt
    tea decrypted, err := cryptz.Decrypt(ciphertext, key)
    lowkey err != cap {
        vibez.spill("Decryption error:", err)
        yolo
    }
    
    vibez.spill("Decrypted:", decrypted)
    lowkey decrypted != plaintext {
        vibez.spill("Decryption failed! Expected '", plaintext, "' but got '", decrypted, "'")
        yolo
    }
    
    fr fr Wrong key should fail decryption
    tea wrongKey := "wrong-key-00000000000000000000000000000000"
    tea _, err2 := cryptz.Decrypt(ciphertext, wrongKey)
    lowkey err2 == cap {
        vibez.spill("Decryption with wrong key succeeded when it should fail!")
        yolo
    }
    
    fr fr Different plaintexts should produce different ciphertexts
    tea plaintext2 := "Different message"
    tea ciphertext2, _ := cryptz.Encrypt(plaintext2, key)
    lowkey ciphertext == ciphertext2 {
        vibez.spill("Encryption produced same ciphertext for different plaintexts!")
        yolo
    }
    
    vibez.spill("Encryption and decryption tests passed!")
}