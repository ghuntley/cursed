# Cryptz Module - Modern Cryptographic Library for CURSED

## Overview

The `cryptz` module provides a comprehensive suite of modern cryptographic primitives and functions for securing data in CURSED applications. It implements industry-standard algorithms with a security-conscious Gen Z enhanced API, focusing on ease of use while maintaining cryptographic security best practices.

## Features

### 🔐 Hash Functions
- **SHA-256**: Industry standard, secure hash algorithm
- **SHA-512**: Extended version with higher security margin
- **BLAKE3**: Modern, high-performance cryptographic hash
- **HMAC**: Keyed-hash message authentication codes

### 🔒 Symmetric Encryption
- **AES**: Advanced Encryption Standard (128/192/256-bit keys)
- **AES-GCM**: Authenticated encryption with associated data
- **ChaCha20-Poly1305**: Modern stream cipher with authentication

### 🗝️ Asymmetric Cryptography
- **RSA**: Public key encryption and digital signatures
- **ECDSA**: Elliptic Curve Digital Signature Algorithm
- **Ed25519**: Edwards-curve digital signatures (recommended)

### 🛡️ Password Security
- **Argon2**: Modern password hashing (memory-hard)
- **PBKDF2**: Key derivation with configurable iterations
- **Secure password verification with timing attack protection

### 🎲 Cryptographically Secure Random
- **RandomBytes**: Generate cryptographically secure random data
- **RandomString**: Generate random strings for tokens/passwords
- **RandomInt**: Generate random integers within specified ranges

### 🔧 Utility Functions
- **Hex encoding/decoding**: Convert binary data to/from hex strings
- **Base64 encoding**: Convert binary data to Base64 format
- **Constant-time comparison**: Prevent timing attacks on secrets
- **Key stretching**: Strengthen weak passwords
- **Zero-knowledge proofs**: Basic ZK proof generation/verification

## Security Warnings ⚠️

1. **Key Management**: Always use cryptographically secure random keys
2. **Timing Attacks**: Use `ConstantTimeCompare` for secret comparisons
3. **Password Hashing**: Always use Argon2 or PBKDF2 with sufficient iterations
4. **Random Generation**: Use `RandomBytes` for cryptographic purposes only
5. **Key Sizes**: Follow minimum key size recommendations (RSA ≥ 2048 bits)
6. **Salt Usage**: Always use unique salts for password hashing
7. **Secure Wiping**: Use `SecureWipe` to clear sensitive data from memory

## Quick Start Examples

### Hash Data
```cursed
yeet "cryptz"

# Simple hashing
sus data tea = "Hello, World!"
sus hash tea = cryptz.Sum256(data)
vibez.spill("SHA-256:", cryptz.ToHex(hash))

# HMAC for message authentication
sus key tea = cryptz.RandomBytes(32)
sus hmac tea = cryptz.ComputeHMAC("sha256", key, data)
```

### Symmetric Encryption
```cursed
yeet "cryptz"

# AES-GCM authenticated encryption
sus key tea = cryptz.RandomBytes(32)  # AES-256
sus cipher tea = cryptz.NewAESCipher(key)
sus gcm tea = cryptz.NewGCM(cipher)

sus nonce tea = cryptz.RandomBytes(12)
sus plaintext tea = "Secret message"
sus ciphertext tea = cryptz.GCMSeal(gcm, nonce, plaintext, "")

# Decrypt
sus decrypted tea = cryptz.GCMOpen(gcm, nonce, ciphertext, "")
```

### Asymmetric Cryptography
```cursed
yeet "cryptz"

# Ed25519 digital signatures (recommended)
sus (private_key, public_key) = cryptz.GenerateEd25519Key()
sus message tea = "Important document"
sus signature tea = cryptz.SignEd25519(private_key, message)
sus valid lit = cryptz.VerifyEd25519(public_key, message, signature)

# RSA encryption
sus rsa_private tea = cryptz.GenerateRSAKey(2048)
sus rsa_public tea = cryptz.RSAPublicKey(rsa_private)
sus encrypted tea = cryptz.EncryptRSA(rsa_public, "Secret data")
sus decrypted tea = cryptz.DecryptRSA(rsa_private, encrypted)
```

### Password Security
```cursed
yeet "cryptz"

# Hash password for storage
sus password tea = "user_password_123"
sus hashed tea = cryptz.HashPassword(password)

# Verify password during login
sus is_valid lit = cryptz.VerifyPassword(hashed, password)

# Key derivation for encryption keys
sus salt tea = cryptz.RandomBytes(32)
sus encryption_key tea = cryptz.Argon2(password, salt, 3, 1024, 1, 32)
```

## API Reference

### Random Number Generation

#### `RandomBytes(n normie) tea`
Generates `n` cryptographically secure random bytes.
- **Parameters**: `n` - number of bytes to generate
- **Returns**: String containing random bytes
- **Usage**: Key generation, nonces, salts

#### `RandomString(n normie) tea`
Generates random alphanumeric string of length `n`.
- **Parameters**: `n` - length of string
- **Returns**: Random string with [A-Za-z0-9] characters
- **Usage**: Tokens, temporary passwords

#### `RandomInt(min normie, max normie) normie`
Generates random integer in range [min, max].
- **Parameters**: `min`, `max` - range bounds (inclusive)
- **Returns**: Random integer in specified range

### Hash Functions

#### `Sum256(data tea) tea`
Computes SHA-256 hash of input data.
- **Parameters**: `data` - input data to hash
- **Returns**: SHA-256 hash as hex string
- **Security**: Collision-resistant, suitable for integrity checks

#### `Sum512(data tea) tea`
Computes SHA-512 hash of input data.
- **Parameters**: `data` - input data to hash
- **Returns**: SHA-512 hash as hex string
- **Security**: Higher security margin than SHA-256

#### `SumBlake3(data tea) tea`
Computes BLAKE3 hash of input data.
- **Parameters**: `data` - input data to hash
- **Returns**: BLAKE3 hash as hex string
- **Security**: Modern, high-performance algorithm

#### `ComputeHMAC(hasher tea, key tea, message tea) tea`
Computes keyed-hash message authentication code.
- **Parameters**: `hasher` - hash algorithm, `key` - secret key, `message` - data
- **Returns**: HMAC as hex string
- **Security**: Prevents tampering, requires secret key

### Symmetric Encryption

#### `NewAESCipher(key tea) tea`
Creates new AES cipher with given key.
- **Parameters**: `key` - AES key (16, 24, or 32 bytes)
- **Returns**: AES cipher instance
- **Usage**: Block encryption, use with GCM for security

#### `AESEncrypt(cipher tea, plaintext tea) tea`
Encrypts data using AES cipher.
- **Parameters**: `cipher` - AES cipher, `plaintext` - data to encrypt
- **Returns**: Encrypted data
- **Warning**: Use AES-GCM for authenticated encryption

#### `NewGCM(cipher tea) tea`
Creates Galois/Counter Mode wrapper for AES.
- **Parameters**: `cipher` - AES cipher instance
- **Returns**: GCM instance
- **Security**: Provides authentication and encryption

#### `GCMSeal(gcm tea, nonce tea, plaintext tea, additional_data tea) tea`
Encrypts and authenticates data using AES-GCM.
- **Parameters**: `gcm` - GCM instance, `nonce` - unique nonce, `plaintext` - data, `additional_data` - associated data
- **Returns**: Encrypted and authenticated data
- **Security**: Industry standard for authenticated encryption

### Asymmetric Cryptography

#### `GenerateEd25519Key() (tea, tea)`
Generates Ed25519 key pair.
- **Returns**: (private_key, public_key) tuple
- **Security**: Modern, fast, secure digital signatures
- **Recommended**: Use for new applications

#### `SignEd25519(private_key tea, message tea) tea`
Signs message using Ed25519 private key.
- **Parameters**: `private_key` - Ed25519 private key, `message` - data to sign
- **Returns**: Digital signature
- **Security**: Deterministic, secure signatures

#### `VerifyEd25519(public_key tea, message tea, signature tea) lit`
Verifies Ed25519 signature.
- **Parameters**: `public_key` - Ed25519 public key, `message` - original data, `signature` - signature to verify
- **Returns**: `based` if valid, `cap` if invalid

#### `GenerateRSAKey(bits normie) tea`
Generates RSA private key.
- **Parameters**: `bits` - key size (minimum 2048)
- **Returns**: RSA private key
- **Security**: Use minimum 2048 bits, 3072+ recommended

### Password Hashing

#### `HashPassword(password tea) tea`
Hashes password using Argon2.
- **Parameters**: `password` - user password
- **Returns**: Argon2 hash with embedded salt
- **Security**: Memory-hard, resistant to GPU attacks

#### `VerifyPassword(hashed_password tea, password tea) lit`
Verifies password against stored hash.
- **Parameters**: `hashed_password` - stored hash, `password` - user input
- **Returns**: `based` if correct, `cap` if incorrect
- **Security**: Constant-time comparison

#### `PBKDF2(password tea, salt tea, iterations normie, key_length normie) tea`
Derives key using PBKDF2.
- **Parameters**: `password` - base password, `salt` - unique salt, `iterations` - iteration count, `key_length` - output length
- **Returns**: Derived key
- **Security**: Use minimum 100,000 iterations

#### `Argon2(password tea, salt tea, time normie, memory normie, threads normie, key_length normie) tea`
Derives key using Argon2.
- **Parameters**: `password` - base password, `salt` - unique salt, `time` - time cost, `memory` - memory cost, `threads` - parallelism, `key_length` - output length
- **Returns**: Derived key
- **Security**: Memory-hard, recommended for new applications

### Security Utilities

#### `ConstantTimeCompare(a tea, b tea) lit`
Compares two values in constant time.
- **Parameters**: `a`, `b` - values to compare
- **Returns**: `based` if equal, `cap` if different
- **Security**: Prevents timing attacks on secrets

#### `SecureWipe(data tea)`
Securely overwrites sensitive data in memory.
- **Parameters**: `data` - sensitive data to wipe
- **Security**: Prevents memory dumps from revealing secrets

#### `GenerateSecureKey(key_size normie) tea`
Generates cryptographically secure key.
- **Parameters**: `key_size` - key length in bytes
- **Returns**: Secure random key
- **Usage**: Encryption keys, API keys

### Encoding Utilities

#### `ToHex(data tea) tea`
Converts binary data to hexadecimal string.
- **Parameters**: `data` - binary data
- **Returns**: Hex string (lowercase)

#### `FromHex(hex_string tea) tea`
Converts hexadecimal string to binary data.
- **Parameters**: `hex_string` - hex string
- **Returns**: Binary data

#### `ToBase64(data tea) tea`
Converts binary data to Base64 string.
- **Parameters**: `data` - binary data
- **Returns**: Base64 encoded string

## Best Practices

### 1. Key Management
- Generate keys using `RandomBytes` or `GenerateSecureKey`
- Store keys securely, separate from application code
- Rotate keys regularly
- Use hardware security modules (HSMs) for high-value keys

### 2. Password Security
- Always hash passwords with Argon2 or PBKDF2
- Use unique salts for each password
- Set appropriate cost parameters (time, memory)
- Never store plaintext passwords

### 3. Encryption
- Use authenticated encryption (AES-GCM, ChaCha20-Poly1305)
- Generate unique nonces/IVs for each encryption
- Protect encryption keys with same rigor as encrypted data
- Consider key derivation from user passwords

### 4. Digital Signatures
- Use Ed25519 for new applications
- Verify signatures before trusting signed data
- Protect private signing keys rigorously
- Consider certificate chains for public key distribution

### 5. Random Number Generation
- Use `RandomBytes` for all cryptographic purposes
- Never use predictable seeds or weak PRNGs
- Generate sufficient entropy (minimum 128 bits for security)

## Security Considerations

### Threat Model
This implementation provides security against:
- Passive eavesdropping
- Data tampering
- Password cracking (with proper parameters)
- Timing attacks (when using constant-time functions)

### Limitations
- Simplified implementation for educational/prototype use
- Production systems should use battle-tested crypto libraries
- Side-channel attacks may be possible
- No hardware security module integration

### Compliance
- Implements NIST-approved algorithms (AES, SHA-2, ECDSA)
- Follows cryptographic best practices
- Suitable for FIPS 140-2 Level 1 compliance (with proper validation)

## Migration Guide

### From Legacy Crypto Module
```cursed
# Old crypto module
yeet "crypto"
sus hash tea = crypto.sha256("data")

# New cryptz module
yeet "cryptz"
sus hash tea = cryptz.Sum256("data")
sus hex_hash tea = cryptz.ToHex(hash)
```

### Key Size Upgrades
- RSA: Upgrade from 1024 to 2048+ bits
- AES: Use 256-bit keys instead of 128-bit
- Hash: Migrate from MD5/SHA-1 to SHA-256+

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/cryptz/test_cryptz.csd
```

Tests cover:
- All cryptographic functions
- Error conditions and edge cases
- Security validations
- Performance characteristics
- Cross-function compatibility

## Contributing

When contributing to the cryptz module:
1. Follow cryptographic best practices
2. Add comprehensive tests for new functions
3. Document security considerations
4. Validate against known test vectors
5. Consider timing attack resistance

## License

This cryptographic implementation is provided for educational and research purposes. Production use should employ verified, audited cryptographic libraries.

---

**Remember**: Cryptography is hard to get right. When in doubt, consult cryptographic experts and use well-established, audited libraries for production systems.
