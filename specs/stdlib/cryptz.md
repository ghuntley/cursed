# Cryptz (crypto package)

## Overview
Cryptz provides cryptographic primitives and functions for securing data. It's inspired by Go's crypto package and its subpackages but with a security-conscious Gen Z twist.

## Core Interfaces

### `Hasher`
Interface for hash functions (like crypto/hash.Hash)

```go
interface Hasher {
    // Write adds data to the hash
    Write(p []byte) (n int, err error)
    
    // Sum returns the hash value of all data written
    Sum(b []byte) []byte
    
    // Reset resets the hash to its initial state
    Reset()
    
    // Size returns the number of bytes returned by Sum
    Size() int
    
    // BlockSize returns the hash's underlying block size
    BlockSize() int
}
```

### `Signer`
Interface for digital signature algorithms

```go
interface Signer {
    // Sign signs the message with a private key
    Sign(rand YeetIO.Yoink, msg []byte) (signature []byte, err error)
    
    // Public returns the corresponding public key
    Public() PublicKey
}
```

### `Verifier`
Interface for signature verification

```go
interface Verifier {
    // Verify verifies a signature against a message
    Verify(msg, sig []byte) (bool, error)
}
```

## Cryptographic Hashing

### `SHA256`
Implements the SHA-256 hash algorithm

```go
func NewSHA256() Hasher
func Sum256(data []byte) [32]byte
```

### `SHA512`
Implements the SHA-512 hash algorithm

```go
func NewSHA512() Hasher
func Sum512(data []byte) [64]byte
```

### `Blake3`
Implements the BLAKE3 hash algorithm (modern addition)

```go
func NewBlake3() Hasher
func SumBlake3(data []byte) [32]byte
```

### `HMAC`
Implements keyed-hash message authentication

```go
func NewHMAC(h Hasher, key []byte) Hasher
```

## Symmetric Encryption

### `AESCipher`
Implements AES encryption

```go
func NewAESCipher(key []byte) (*AESCipher, error)

type AESCipher struct {}

// Methods
func (c *AESCipher) Encrypt(dst, src []byte)
func (c *AESCipher) Decrypt(dst, src []byte)
```

### `GCM`
Implements Galois/Counter Mode for authenticated encryption

```go
func NewGCM(cipher *AESCipher) (*GCM, error)

type GCM struct {}

// Methods
func (gcm *GCM) Seal(dst, nonce, plaintext, additionalData []byte) []byte
func (gcm *GCM) Open(dst, nonce, ciphertext, additionalData []byte) ([]byte, error)
```

## Asymmetric Cryptography

### `RSA`
Implements RSA encryption and signing

```go
func GenerateRSAKey(bits int) (*RSAPrivateKey, error)

type RSAPublicKey struct {}
type RSAPrivateKey struct {}

// Encryption methods
func EncryptRSA(pub *RSAPublicKey, msg []byte) ([]byte, error)
func DecryptRSA(priv *RSAPrivateKey, ciphertext []byte) ([]byte, error)

// Signing methods
func SignRSA(priv *RSAPrivateKey, msg []byte) ([]byte, error)
func VerifyRSA(pub *RSAPublicKey, msg, sig []byte) error
```

### `ECDSA`
Implements Elliptic Curve Digital Signature Algorithm

```go
func GenerateECDSAKey(curve string) (*ECDSAPrivateKey, error)

type ECDSAPublicKey struct {}
type ECDSAPrivateKey struct {}

// Signing methods
func SignECDSA(priv *ECDSAPrivateKey, msg []byte) ([]byte, error)
func VerifyECDSA(pub *ECDSAPublicKey, msg, sig []byte) error
```

### `Ed25519`
Implements Edwards-curve Digital Signature Algorithm (EdDSA)

```go
func GenerateEd25519Key() (*Ed25519PrivateKey, *Ed25519PublicKey, error)

type Ed25519PublicKey []byte
type Ed25519PrivateKey []byte

// Signing methods
func SignEd25519(priv Ed25519PrivateKey, msg []byte) []byte
func VerifyEd25519(pub Ed25519PublicKey, msg, sig []byte) bool
```

## Password Hashing

```go
func HashPassword(password string) (string, error)
func VerifyPassword(hashedPassword, password string) bool
```

## Random Number Generation

```go
func RandomBytes(n int) ([]byte, error)
func RandomString(n int) (string, error)
func RandomInt(min, max int) (int, error)
```

## Usage Example

```go
// Hashing example
hasher := cryptz.NewSHA256()
hasher.Write([]byte("secure data"))
hash := hasher.Sum(nil)
vibez.spill(hex.EncodeToString(hash))

// AES-GCM encryption example
key, _ := cryptz.RandomBytes(32) // AES-256
nonce, _ := cryptz.RandomBytes(12)

cipher, _ := cryptz.NewAESCipher(key)
gcm, _ := cryptz.NewGCM(cipher)

message := []byte("secret message")
encrypted := gcm.Seal(nil, nonce, message, nil)

// RSA example
privKey, _ := cryptz.GenerateRSAKey(2048)
pubKey := privKey.Public()

signature, _ := cryptz.SignRSA(privKey, message)
isValid := cryptz.VerifyRSA(pubKey, message, signature)
```

## Implementation Guidelines
1. Follow cryptographic best practices and standards
2. Use constant-time comparison functions for secrets
3. Provide comprehensive error handling for cryptographic operations
4. Use secure random number generation
5. Support modern algorithms and deprecate insecure ones
6. Implement proper key management practices