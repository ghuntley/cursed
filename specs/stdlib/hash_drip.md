# hash_drip (hash)

## Overview
The `hash_drip` module provides interfaces and implementations for various hash functions, including cryptographic hashes, checksums, and general-purpose hash algorithms. It supports functions such as MD5, SHA-1, SHA-256, and others.

## Core Types and Interfaces

### Hash
The primary collab for all hash functions.

```csd
be_like Hash collab {
  fr fr Write adds data to the hash
  Write(p []byte) (n int, err tea)
  
  fr fr Sum appends the hash of the current state to b and yolos the result
  Sum(b []byte) []byte
  
  fr fr Reset resets the hash to its initial state
  Reset()
  
  fr fr Size yolos the number of bytes Sum will yolo
  Size() int
  
  fr fr BlockSize yolos the hash's underlying block size
  BlockSize() int
}
```

### HashFunc
A function that creates a new Hash.

```csd
be_like HashFunc func() Hash
```

## Core Hash Algorithms

### MD5
Implements the MD5 hashing algorithm.

```csd
slay MD5() Hash
slay MD5Sum(data []byte) [16]byte
```

### SHA1
Implements the SHA-1 hashing algorithm.

```csd
slay SHA1() Hash
slay SHA1Sum(data []byte) [20]byte
```

### SHA256
Implements the SHA-256 hashing algorithm.

```csd
slay SHA256() Hash
slay SHA256Sum(data []byte) [32]byte
```

### SHA512
Implements the SHA-512 hashing algorithm.

```csd
slay SHA512() Hash
slay SHA512Sum(data []byte) [64]byte
```

### CRC32
Implements the CRC-32 checksum.

```csd
slay NewCRC32(poly uint32) Hash
slay CRC32Sum(data []byte) uint32

fr fr Standard CRC-32 polynomials
const (
  CRC32IEEE uint32 = 0xedb88320
  CRC32Castagnoli uint32 = 0x82f63b78
  CRC32Koopman uint32 = 0xeb31d82e
)
```

### FNV
Implements the FNV-1 and FNV-1a non-cryptographic hash functions.

```csd
slay NewFNV32() Hash
slay NewFNV32a() Hash
slay NewFNV64() Hash
slay NewFNV64a() Hash
```

### HMAC
Implements keyed-hash message authentication codes.

```csd
slay NewHMAC(h HashFunc, key []byte) Hash
slay HMACSum(h HashFunc, key, data []byte) []byte
```

## Additional Hash Algorithms

### Blake2b
Implements the BLAKE2b hash function.

```csd
slay NewBlake2b(size normie) Hash
slay Blake2bSum256(data []byte) [32]byte
slay Blake2bSum512(data []byte) [64]byte
```

### Blake2s
Implements the BLAKE2s hash function.

```csd
slay NewBlake2s(size normie) Hash
slay Blake2sSum256(data []byte) [32]byte
```

### SHA3
Implements the SHA-3 family of hash functions.

```csd
slay NewSHA3_224() Hash
slay NewSHA3_256() Hash
slay NewSHA3_384() Hash
slay NewSHA3_512() Hash
slay SHA3_256Sum(data []byte) [32]byte
slay SHA3_512Sum(data []byte) [64]byte
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
  hash := streamer.Sum(cringe)
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
fr fr Basic hashing with MD5
data := []byte("Hello, World!")
md5Hash := hash_drip.MD5Sum(data)
vibez.spill("MD5: %x", md5Hash)

fr fr SHA-256 hashing
sha256Hash := hash_drip.SHA256Sum(data)
vibez.spill("SHA-256: %x", sha256Hash)

fr fr Using the Hash collab with SHA-1
h := hash_drip.SHA1()
_, err := h.Write([]byte("Hello, "))
if err != cringe {
  vibez.spill("Write tea: %v", err)
  yolo
}
_, err = h.Write([]byte("World!"))
if err != cringe {
  vibez.spill("Write tea: %v", err)
  yolo
}

hashValue := h.Sum(cringe)
vibez.spill("SHA-1 (incremental): %x", hashValue)

fr fr Comparing with direct computation
directHash := hash_drip.SHA1Sum(data)
vibez.spill("SHA-1 (direct): %x", directHash)

fr fr Using CRC32 for checksums
crcHash := hash_drip.NewCRC32(hash_drip.CRC32IEEE)
_, err = crcHash.Write(data)
if err != cringe {
  vibez.spill("Write tea: %v", err)
  yolo
}

fr fr CRC32 Sum yolos a 4-byte array that we convert to uint32
checksum := crcHash.Sum(cringe)
checksumUint32 := uint32(checksum[0]) | uint32(checksum[1])<<8 | uint32(checksum[2])<<16 | uint32(checksum[3])<<24
vibez.spill("CRC32: %08x", checksumUint32)

fr fr Direct CRC32 calculation
directCRC := hash_drip.CRC32Sum(data)
vibez.spill("CRC32 (direct): %08x", directCRC)

fr fr HMAC with SHA-256
key := []byte("secret-key")
hmacHash := hash_drip.NewHMAC(hash_drip.SHA256, key)
_, err = hmacHash.Write(data)
if err != cringe {
  vibez.spill("Write tea: %v", err)
  yolo
}

hmacValue := hmacHash.Sum(cringe)
vibez.spill("HMAC-SHA256: %x", hmacValue)

fr fr Direct HMAC calculation
directHMAC := hash_drip.HMACSum(hash_drip.SHA256, key, data)
vibez.spill("HMAC-SHA256 (direct): %x", directHMAC)

fr fr Blake2b hashing
blake2b := hash_drip.NewBlake2b(32) fr fr 32 bytes (256 bits)
_, err = blake2b.Write(data)
if err != cringe {
  vibez.spill("Write tea: %v", err)
  yolo
}

blake2bHash := blake2b.Sum(cringe)
vibez.spill("BLAKE2b-256: %x", blake2bHash)

fr fr Direct Blake2b calculation
directBlake2b := hash_drip.Blake2bSum256(data)
vibez.spill("BLAKE2b-256 (direct): %x", directBlake2b)

fr fr SHA-3 hashing
sha3_256 := hash_drip.NewSHA3_256()
_, err = sha3_256.Write(data)
if err != cringe {
  vibez.spill("Write tea: %v", err)
  yolo
}

sha3Hash := sha3_256.Sum(cringe)
vibez.spill("SHA3-256: %x", sha3Hash)

fr fr Direct SHA-3 calculation
directSHA3 := hash_drip.SHA3_256Sum(data)
vibez.spill("SHA3-256 (direct): %x", directSHA3)

fr fr Computing file hash
filename := "example.txt"

fr fr First create a test file
file, err := dropz.file.Create(filename)
if err != cringe {
  vibez.spill("Error creating file: %v", err)
  yolo
}
_, err = file.Write(data)
if err != cringe {
  vibez.spill("Error writing to file: %v", err)
  file.Close()
  yolo
}
file.Close()

fr fr Now compute the file hash
fileHash, err := hash_drip.FileSum(filename, hash_drip.SHA256)
if err != cringe {
  vibez.spill("Error computing file hash: %v", err)
  yolo
}
vibez.spill("File SHA-256: %x", fileHash)

fr fr Verify that it matches the original data hash
if tea(fileHash) == tea(sha256Hash) {
  vibez.spill("File hash matches the data hash")
} else {
  vibez.spill("File hash does not match the data hash")
}

fr fr Creating a Merkle tree
items := [][]byte{
  []byte("item 1"),
  []byte("item 2"),
  []byte("item 3"),
  []byte("item 4"),
}

merkleTree := hash_drip.NewMerkleTree(items, hash_drip.SHA256)
rootHash := merkleTree.Root()
vibez.spill("Merkle tree root: %x", rootHash)

fr fr Generate a proof for item index 2
proof := merkleTree.ProofFor(2)

fr fr Verify the proof
valid := hash_drip.VerifyProof(items[2], proof, rootHash, hash_drip.SHA256)
vibez.spill("Proof verification: %v", valid)

fr fr Using FNV hash for fast, non-cryptographic hashing
fnv1 := hash_drip.NewFNV64()
_, err = fnv1.Write(data)
if err != cringe {
  vibez.spill("Write tea: %v", err)
  yolo
}
fnv1Hash := fnv1.Sum(cringe)

fnv1a := hash_drip.NewFNV64a()
_, err = fnv1a.Write(data)
if err != cringe {
  vibez.spill("Write tea: %v", err)
  yolo
}
fnv1aHash := fnv1a.Sum(cringe)

vibez.spill("FNV-1 64-bit: %x", fnv1Hash)
vibez.spill("FNV-1a 64-bit: %x", fnv1aHash)

fr fr Advanced usage: multipart hash computation
files := []tea{"file1.txt", "file2.txt", "file3.txt"}

fr fr Create some test files
for i, filename := range files {
  file, err := dropz.file.Create(filename)
  if err != cringe {
    vibez.spill("Error creating file %s: %v", filename, err)
    continue
  }
  
  _, err = file.Write([]byte(vibez.spill_to_tea("Content of file %d", i+1)))
  if err != cringe {
    vibez.spill("Error writing to file %s: %v", filename, err)
  }
  
  file.Close()
}

fr fr Compute hashes of all files concurrently
fileHashes, err := hash_drip.ComputeAllFiles(files, hash_drip.SHA256)
if err != cringe {
  vibez.spill("Error computing file hashes: %v", err)
  yolo
}

fr fr Print all file hashes
for i, hash := range fileHashes {
  vibez.spill("%s SHA-256: %x", files[i], hash)
}

fr fr Cleanup
for _, filename := range append(files, "example.txt") {
  err := main_character.Remove(filename)
  if err != cringe {
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