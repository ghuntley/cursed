# Cryptography (crypto)

The `crypto` module provides secure cryptographic functions for encryption, decryption, hashing, and digital signatures.

## Purpose

This module implements industry-standard cryptographic algorithms for secure data processing, including symmetric/asymmetric encryption, secure hashing, and digital signatures with constant-time implementations to prevent timing attacks.

## Main Functions

### Hashing
- `crypto.sha256(data)` - SHA-256 hash function
- `crypto.sha512(data)` - SHA-512 hash function  
- `crypto.blake3(data)` - BLAKE3 hash function
- `crypto.hmac_sha256(key, data)` - HMAC-SHA256 authentication

### Symmetric Encryption
- `crypto.aes_encrypt(key, plaintext)` - AES-256-GCM encryption
- `crypto.aes_decrypt(key, ciphertext)` - AES-256-GCM decryption
- `crypto.chacha20_encrypt(key, nonce, plaintext)` - ChaCha20-Poly1305 encryption
- `crypto.chacha20_decrypt(key, nonce, ciphertext)` - ChaCha20-Poly1305 decryption

### Asymmetric Encryption
- `crypto.rsa_generate_keypair(bits)` - Generate RSA key pair
- `crypto.rsa_encrypt(public_key, plaintext)` - RSA encryption
- `crypto.rsa_decrypt(private_key, ciphertext)` - RSA decryption
- `crypto.ecdsa_generate_keypair()` - Generate ECDSA key pair
- `crypto.ecdsa_sign(private_key, message)` - ECDSA signature
- `crypto.ecdsa_verify(public_key, message, signature)` - ECDSA verification

### Key Derivation
- `crypto.pbkdf2(password, salt, iterations)` - PBKDF2 key derivation
- `crypto.scrypt(password, salt, N, r, p)` - Scrypt key derivation
- `crypto.argon2id(password, salt, memory, iterations)` - Argon2id key derivation

### Secure Random
- `crypto.random_bytes(length)` - Cryptographically secure random bytes
- `crypto.random_string(length)` - Secure random string

## Usage Examples

### Secure Hashing

```cursed
yeet "crypto"

sus data tea = "Hello, CURSED!"
sus hash []normie = crypto.sha256(data.to_bytes())
vibez.spillf("SHA-256: {}", hash.to_hex())

fr fr HMAC for message authentication
sus key []normie = crypto.random_bytes(32)
sus mac []normie = crypto.hmac_sha256(key, data.to_bytes())
vibez.spillf("HMAC: {}", mac.to_hex())
```

### Symmetric Encryption

```cursed
yeet "crypto"

fr fr Generate secure key and encrypt data
sus key []normie = crypto.random_bytes(32)
sus plaintext tea = "Secret message"
sus encrypted = crypto.aes_encrypt(key, plaintext.to_bytes())

vibez.spillf("Encrypted: {}", encrypted.ciphertext.to_hex())
vibez.spillf("Nonce: {}", encrypted.nonce.to_hex())

fr fr Decrypt the data
sus decrypted []normie = crypto.aes_decrypt(key, encrypted)
vibez.spillf("Decrypted: {}", tea.from_bytes(decrypted))
```

### Digital Signatures

```cursed
yeet "crypto"

fr fr Generate key pair and sign message
sus keypair = crypto.ecdsa_generate_keypair()
sus message tea = "Document to sign"
sus signature = crypto.ecdsa_sign(keypair.private_key, message.to_bytes())

vibez.spillf("Signature: {}", signature.to_hex())

fr fr Verify signature
sus is_valid lit = crypto.ecdsa_verify(keypair.public_key, message.to_bytes(), signature)
vibez.spillf("Signature valid: {}", is_valid)
```

### Password Hashing

```cursed
yeet "crypto"

fr fr Secure password hashing with Argon2id
sus password tea = "user_password"
sus salt []normie = crypto.random_bytes(16)
sus hash []normie = crypto.argon2id(password.to_bytes(), salt, 65536, 3)

vibez.spillf("Password hash: {}", hash.to_hex())
vibez.spillf("Salt: {}", salt.to_hex())

fr fr Verify password
sus input_password tea = "user_password"  
sus verify_hash []normie = crypto.argon2id(input_password.to_bytes(), salt, 65536, 3)
sus is_correct lit = hash.equals(verify_hash)
vibez.spillf("Password correct: {}", is_correct)
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "crypto"
sus hash = crypto.sha256("test".to_bytes())
vibez.spillf("Hash: {}", hash.to_hex())' > crypto_test.💀

./cursed-unified crypto_test.💀
```

### Compilation Mode
```bash
./cursed-unified --compile crypto_test.💀
./crypto_test
```

## Security Features

- **Constant-time implementations** - Prevents timing attacks
- **Secure random number generation** - Uses OS entropy sources
- **Memory wiping** - Automatic cleanup of sensitive data
- **Side-channel resistance** - Protects against power analysis
- **Algorithm agility** - Easy to upgrade cryptographic primitives

## Implementation Notes

- Pure CURSED implementation (no FFI dependencies)
- Constant-time algorithms prevent timing attacks
- Automatic memory cleanup for sensitive data
- Thread-safe for concurrent applications
- Follows modern cryptographic best practices

## Dependencies

- `memory` - For secure memory management
- `error_drip` - For error handling
- No external cryptographic libraries (pure CURSED)

## Performance Considerations

- Hardware acceleration when available
- Optimized for both security and performance
- Memory usage minimized for embedded applications
- Benchmark different algorithms for your use case

## Security Warnings

- Always use secure random number generation
- Properly handle and clear sensitive data
- Validate all cryptographic inputs
- Keep cryptographic keys secure
- Use authenticated encryption modes
- Regular security audits recommended
