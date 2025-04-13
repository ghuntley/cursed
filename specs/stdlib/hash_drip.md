# hash_drip (hash)

## Overview
The `hash_drip` module provides interfaces and implementations for various hash functions, including cryptographic hashes, checksums, and general-purpose hash algorithms. It supports functions such as MD5, SHA-1, SHA-256, and others.

## Core Types and Interfaces

### Hash
The primary interface for all hash functions.

```csd
type Hash interface {
  // Write adds data to the hash
  Write(p []byte) (n int, err error)
  
  // Sum appends the hash of the current state to b and returns the result
  Sum(b []byte) []byte
  
  // Reset resets the hash to its initial state
  Reset()
  
  // Size returns the number of bytes Sum will return
  Size() int
  
  // BlockSize returns the hash's underlying block size
  BlockSize() int
}
```

### HashFunc
A function that creates a new Hash.

```csd
type HashFunc func() Hash
```

## Core Hash Algorithms

### MD5
Implements the MD5 hashing algorithm.

```csd
func MD5() Hash
func MD5Sum(data []byte) [16]byte
```

### SHA1
Implements the SHA-1 hashing algorithm.

```csd
func SHA1() Hash
func SHA1Sum(data []byte) [20]byte
```

### SHA256
Implements the SHA-256 hashing algorithm.

```csd
func SHA256() Hash
func SHA256Sum(data []byte) [32]byte
```

### SHA512
Implements the SHA-512 hashing algorithm.

```csd
func SHA512() Hash
func SHA512Sum(data []byte) [64]byte
```

### CRC32
Implements the CRC-32 checksum.

```csd
func NewCRC32(poly uint32) Hash
func CRC32Sum(data []byte) uint32

// Standard CRC-32 polynomials
const (
  CRC32IEEE uint32 = 0xedb88320
  CRC32Castagnoli uint32 = 0x82f63b78
  CRC32Koopman uint32 = 0xeb31d82e
)
```

### FNV
Implements the FNV-1 and FNV-1a non-cryptographic hash functions.

```csd
func NewFNV32() Hash
func NewFNV32a() Hash
func NewFNV64() Hash
func NewFNV64a() Hash
```

### HMAC
Implements keyed-hash message authentication codes.

```csd
func NewHMAC(h HashFunc, key []byte) Hash
func HMACSum(h HashFunc, key, data []byte) []byte
```

## Additional Hash Algorithms

### Blake2b
Implements the BLAKE2b hash function.

```csd
func NewBlake2b(size int) Hash
func Blake2bSum256(data []byte) [32]byte
func Blake2bSum512(data []byte) [64]byte
```

### Blake2s
Implements the BLAKE2s hash function.

```csd
func NewBlake2s(size int) Hash
func Blake2sSum256(data []byte) [32]byte
```

### SHA3
Implements the SHA-3 family of hash functions.

```csd
func NewSHA3_224() Hash
func NewSHA3_256() Hash
func NewSHA3_384() Hash
func NewSHA3_512() Hash
func SHA3_256Sum(data []byte) [32]byte
func SHA3_512Sum(data []byte) [64]byte
```

## Enhanced Features

- **High-Performance Hashing**: Optimized implementations for speed
  ```csd
  fastHasher := hash_drip.NewOptimizedSHA256()
  ```

- **Streaming Hash Computation**: Process large data in chunks
  ```csd
  streamer := hash_drip.NewStreamHasher(hash_drip.SHA256)
  streamer.Write(chunk1)
  streamer.Write(chunk2)
  hash := streamer.Sum(nil)
  ```

- **Hash Trees (Merkle Trees)**: Build and verify hash trees
  ```csd
  tree := hash_drip.NewMerkleTree(data, hash_drip.SHA256)
  root := tree.Root()
  proof := tree.ProofFor(index)
  ```

- **Hash File Content**: Compute hashes of files efficiently
  ```csd
  fileHash, err := hash_drip.FileSum("filename.txt", hash_drip.SHA256)
  ```

- **Concurrent Hashing**: Process multiple inputs simultaneously
  ```csd
  results := hash_drip.ComputeAll(inputs, hash_drip.SHA256)
  ```

## Usage Examples

```csd
// Basic hashing with MD5
data := []byte("Hello, World!")
md5Hash := hash_drip.MD5Sum(data)
vibez.spill("MD5: %x", md5Hash)

// SHA-256 hashing
sha256Hash := hash_drip.SHA256Sum(data)
vibez.spill("SHA-256: %x", sha256Hash)

// Using the Hash interface with SHA-1
h := hash_drip.SHA1()
_, err := h.Write([]byte("Hello, "))
if err != nil {
  vibez.spill("Write error: %v", err)
  return
}
_, err = h.Write([]byte("World!"))
if err != nil {
  vibez.spill("Write error: %v", err)
  return
}

hashValue := h.Sum(nil)
vibez.spill("SHA-1 (incremental): %x", hashValue)

// Comparing with direct computation
directHash := hash_drip.SHA1Sum(data)
vibez.spill("SHA-1 (direct): %x", directHash)

// Using CRC32 for checksums
crcHash := hash_drip.NewCRC32(hash_drip.CRC32IEEE)
_, err = crcHash.Write(data)
if err != nil {
  vibez.spill("Write error: %v", err)
  return
}

// CRC32 Sum returns a 4-byte array that we convert to uint32
checksum := crcHash.Sum(nil)
checksumUint32 := uint32(checksum[0]) | uint32(checksum[1])<<8 | uint32(checksum[2])<<16 | uint32(checksum[3])<<24
vibez.spill("CRC32: %08x", checksumUint32)

// Direct CRC32 calculation
directCRC := hash_drip.CRC32Sum(data)
vibez.spill("CRC32 (direct): %08x", directCRC)

// HMAC with SHA-256
key := []byte("secret-key")
hmacHash := hash_drip.NewHMAC(hash_drip.SHA256, key)
_, err = hmacHash.Write(data)
if err != nil {
  vibez.spill("Write error: %v", err)
  return
}

hmacValue := hmacHash.Sum(nil)
vibez.spill("HMAC-SHA256: %x", hmacValue)

// Direct HMAC calculation
directHMAC := hash_drip.HMACSum(hash_drip.SHA256, key, data)
vibez.spill("HMAC-SHA256 (direct): %x", directHMAC)

// Blake2b hashing
blake2b := hash_drip.NewBlake2b(32) // 32 bytes (256 bits)
_, err = blake2b.Write(data)
if err != nil {
  vibez.spill("Write error: %v", err)
  return
}

blake2bHash := blake2b.Sum(nil)
vibez.spill("BLAKE2b-256: %x", blake2bHash)

// Direct Blake2b calculation
directBlake2b := hash_drip.Blake2bSum256(data)
vibez.spill("BLAKE2b-256 (direct): %x", directBlake2b)

// SHA-3 hashing
sha3_256 := hash_drip.NewSHA3_256()
_, err = sha3_256.Write(data)
if err != nil {
  vibez.spill("Write error: %v", err)
  return
}

sha3Hash := sha3_256.Sum(nil)
vibez.spill("SHA3-256: %x", sha3Hash)

// Direct SHA-3 calculation
directSHA3 := hash_drip.SHA3_256Sum(data)
vibez.spill("SHA3-256 (direct): %x", directSHA3)

// Computing file hash
filename := "example.txt"

// First create a test file
file, err := dropz.file.Create(filename)
if err != nil {
  vibez.spill("Error creating file: %v", err)
  return
}
_, err = file.Write(data)
if err != nil {
  vibez.spill("Error writing to file: %v", err)
  file.Close()
  return
}
file.Close()

// Now compute the file hash
fileHash, err := hash_drip.FileSum(filename, hash_drip.SHA256)
if err != nil {
  vibez.spill("Error computing file hash: %v", err)
  return
}
vibez.spill("File SHA-256: %x", fileHash)

// Verify that it matches the original data hash
if string(fileHash) == string(sha256Hash) {
  vibez.spill("File hash matches the data hash")
} else {
  vibez.spill("File hash does not match the data hash")
}

// Creating a Merkle tree
items := [][]byte{
  []byte("item 1"),
  []byte("item 2"),
  []byte("item 3"),
  []byte("item 4"),
}

merkleTree := hash_drip.NewMerkleTree(items, hash_drip.SHA256)
rootHash := merkleTree.Root()
vibez.spill("Merkle tree root: %x", rootHash)

// Generate a proof for item index 2
proof := merkleTree.ProofFor(2)

// Verify the proof
valid := hash_drip.VerifyProof(items[2], proof, rootHash, hash_drip.SHA256)
vibez.spill("Proof verification: %v", valid)

// Using FNV hash for fast, non-cryptographic hashing
fnv1 := hash_drip.NewFNV64()
_, err = fnv1.Write(data)
if err != nil {
  vibez.spill("Write error: %v", err)
  return
}
fnv1Hash := fnv1.Sum(nil)

fnv1a := hash_drip.NewFNV64a()
_, err = fnv1a.Write(data)
if err != nil {
  vibez.spill("Write error: %v", err)
  return
}
fnv1aHash := fnv1a.Sum(nil)

vibez.spill("FNV-1 64-bit: %x", fnv1Hash)
vibez.spill("FNV-1a 64-bit: %x", fnv1aHash)

// Advanced usage: multipart hash computation
files := []string{"file1.txt", "file2.txt", "file3.txt"}

// Create some test files
for i, filename := range files {
  file, err := dropz.file.Create(filename)
  if err != nil {
    vibez.spill("Error creating file %s: %v", filename, err)
    continue
  }
  
  _, err = file.Write([]byte(vibez.spill_to_string("Content of file %d", i+1)))
  if err != nil {
    vibez.spill("Error writing to file %s: %v", filename, err)
  }
  
  file.Close()
}

// Compute hashes of all files concurrently
fileHashes, err := hash_drip.ComputeAllFiles(files, hash_drip.SHA256)
if err != nil {
  vibez.spill("Error computing file hashes: %v", err)
  return
}

// Print all file hashes
for i, hash := range fileHashes {
  vibez.spill("%s SHA-256: %x", files[i], hash)
}

// Cleanup
for _, filename := range append(files, "example.txt") {
  err := main_character.Remove(filename)
  if err != nil {
    vibez.spill("Error removing file %s: %v", filename, err)
  }
}
```

## Implementation Guidelines

- Implement hash functions according to their respective standards
- Optimize performance for both small and large data inputs
- Ensure thread-safety for all hash operations
- Provide both streaming and one-shot interfaces for all hash functions
- Implement proper padding and finalization for all algorithms
- Validate hash output against test vectors
- Support incremental hashing for large data sets
- Keep memory usage minimal, especially for large data
- Provide clear documentation on security properties of each hash function
- Include performance characteristics and recommendations