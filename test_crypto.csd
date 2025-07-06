#!/usr/bin/env cursed

// Test crypto functions in CURSED
sus message tea = "Hello, CURSED crypto!"

// Test hash functions
vibez.spill("=== Hash Functions ===")
vibez.spill("Original message: " + message)

sus sha256Hash tea = crypto.sha256(message)
vibez.spill("SHA256: " + sha256Hash)

sus md5Hash tea = crypto.md5(message)  
vibez.spill("MD5: " + md5Hash)

sus blake3Hash tea = crypto.blake3(message)
vibez.spill("BLAKE3: " + blake3Hash)

// Test base64 encoding
vibez.spill("\n=== Base64 Encoding ===")
sus encoded tea = crypto.base64_encode(message)
vibez.spill("Base64 encoded: " + encoded)

sus decoded tea = crypto.base64_decode(encoded)
vibez.spill("Base64 decoded: " + decoded)

// Test random generation
vibez.spill("\n=== Random Generation ===")
sus randomBytes tea = crypto.random_bytes(16)
vibez.spill("Random bytes (hex): " + randomBytes)

sus randomNum normie = crypto.random_int(1, 100)
vibez.spill("Random number (1-100): " + randomNum.toString())

sus randomStr tea = crypto.random_string(12)
vibez.spill("Random string: " + randomStr)

// Test HMAC
vibez.spill("\n=== HMAC ===")
sus key tea = "secret_key"
sus hmacSha256 tea = crypto.hmac_sha256(message, key)
vibez.spill("HMAC-SHA256: " + hmacSha256)

// Test constant-time comparison
vibez.spill("\n=== Constant-time comparison ===")
sus isEqual lit = crypto.constant_time_eq(message, message)
vibez.spill("Messages equal: " + isEqual.toString())

vibez.spill("\nCrypto tests completed!")
