# Cryptz (crypto package)

## Overview
Cryptz provides cryptographic primitives and functions for securing data. It's inspired by Go's crypto package and its subpackages but with a security-conscious Gen Z twist.

## Core Interfaces

### `Hasher`
Interface for hash functions (like crypto/hash.Hash)

```
collab Hasher {
    fr fr Write adds data to the hash
    Write(p []byte) (n normie, err tea)
    
    fr fr Sum yolos the hash value of all data written
    Sum(b []byte) []byte
    
    fr fr Reset resets the hash to its initial state
    Reset()
    
    fr fr Size yolos the number of bytes yoloed by Sum
    Size() normie
    
    fr fr BlockSize yolos the hash's underlying block size
    BlockSize() normie
}
```

### `Signer`
Interface for digital signature algorithms

```
collab Signer {
    fr fr Sign signs the message with a private key
    Sign(rand yeet_io.Yoink, msg []byte) (signature []byte, err tea)
    
    fr fr Public yolos the corresponding public key
    Public() PublicKey
}
```

### `Verifier`
Interface for signature verification

```
collab Verifier {
    fr fr Verify verifies a signature against a message
    Verify(msg, sig []byte) (lit, tea)
}
```

## Cryptographic Hashing

### `SHA256`
Implements the SHA-256 hash algorithm

```
slay NewSHA256() Hasher
slay Sum256(data []byte) [32]byte
```

### `SHA512`
Implements the SHA-512 hash algorithm

```
slay NewSHA512() Hasher
slay Sum512(data []byte) [64]byte
```

### `Blake3`
Implements the BLAKE3 hash algorithm (modern addition)

```
slay NewBlake3() Hasher
slay SumBlake3(data []byte) [32]byte
```

### `HMAC`
Implements keyed-hash message authentication

```
slay NewHMAC(h Hasher, key []byte) Hasher
```

## Symmetric Encryption

### `AESCipher`
Implements AES encryption

```
slay NewAESCipher(key []byte) (*AESCipher, tea)

be_like AESCipher squad {}

fr fr Methods
slay (c *AESCipher) Encrypt(dst, src []byte)
slay (c *AESCipher) Decrypt(dst, src []byte)
```

### `GCM`
Implements Galois/Counter Mode for authenticated encryption

```
slay NewGCM(cipher *AESCipher) (*GCM, tea)

be_like GCM squad {}

fr fr Methods
slay (gcm *GCM) Seal(dst, nonce, plaintext, additionalData []byte) []byte
slay (gcm *GCM) Open(dst, nonce, ciphertext, additionalData []byte) ([]byte, tea)
```

## Asymmetric Cryptography

### `RSA`
Implements RSA encryption and signing

```
slay GenerateRSAKey(bits normie) (*RSAPrivateKey, tea)

be_like RSAPublicKey squad {}
be_like RSAPrivateKey squad {}

fr fr Encryption methods
slay EncryptRSA(pub *RSAPublicKey, msg []byte) ([]byte, tea)
slay DecryptRSA(priv *RSAPrivateKey, ciphertext []byte) ([]byte, tea)

fr fr Signing methods
slay SignRSA(priv *RSAPrivateKey, msg []byte) ([]byte, tea)
slay VerifyRSA(pub *RSAPublicKey, msg, sig []byte) tea
```

### `ECDSA`
Implements Elliptic Curve Digital Signature Algorithm

```
slay GenerateECDSAKey(curve tea) (*ECDSAPrivateKey, tea)

be_like ECDSAPublicKey squad {}
be_like ECDSAPrivateKey squad {}

fr fr Signing methods
slay SignECDSA(priv *ECDSAPrivateKey, msg []byte) ([]byte, tea)
slay VerifyECDSA(pub *ECDSAPublicKey, msg, sig []byte) tea
```

### `Ed25519`
Implements Edwards-curve Digital Signature Algorithm (EdDSA)

```
slay GenerateEd25519Key() (*Ed25519PrivateKey, *Ed25519PublicKey, tea)

be_like Ed25519PublicKey []byte
be_like Ed25519PrivateKey []byte

fr fr Signing methods
slay SignEd25519(priv Ed25519PrivateKey, msg []byte) []byte
slay VerifyEd25519(pub Ed25519PublicKey, msg, sig []byte) lit
```

## Password Hashing

```
slay HashPassword(password tea) (tea, tea)
slay VerifyPassword(hashedPassword, password tea) lit
```

## Random Number Generation

```
slay RandomBytes(n normie) ([]byte, tea)
slay RandomString(n normie) (tea, tea)
slay RandomInt(min, max normie) (normie, tea)
```

## Usage Example

```
fr fr Hashing example
hasher := cryptz.NewSHA256()
hasher.Write([]byte("secure data"))
hash := hasher.Sum(cap)
vibez.spill(hex.EncodeToString(hash))

fr fr AES-GCM encryption example
key, _ := cryptz.RandomBytes(32) fr fr AES-256
nonce, _ := cryptz.RandomBytes(12)

cipher, _ := cryptz.NewAESCipher(key)
gcm, _ := cryptz.NewGCM(cipher)

message := []byte("secret message")
encrypted := gcm.Seal(cap, nonce, message, cap)

fr fr RSA example
privKey, _ := cryptz.GenerateRSAKey(2048)
pubKey := privKey.Public()

signature, _ := cryptz.SignRSA(privKey, message)
isValid := cryptz.VerifyRSA(pubKey, message, signature)
```

## Implementation Guidelines
1. Follow cryptographic best practices and standards
2. Use constant-time comparison functions for secrets
3. Provide comprehensive tea handling for cryptographic operations
4. Use secure random number generation
5. Support modern algorithms and deprecate insecure ones
6. Implement proper key management practices