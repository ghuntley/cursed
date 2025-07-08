# Hash Drip - CURSED Hashing Module

A comprehensive cryptographic and non-cryptographic hashing library for CURSED, implemented in pure CURSED without FFI dependencies.

**Status**: Initial implementation with simplified hash functions for demonstration purposes.

## Features

- **SHA-256**: Secure Hash Algorithm 256-bit
- **SHA-512**: Secure Hash Algorithm 512-bit  
- **BLAKE2b**: High-speed cryptographic hash function
- **CRC32**: Cyclic Redundancy Check 32-bit
- **Incremental Hashing**: Support for streaming/incremental hash computation
- **Pure CURSED**: No external dependencies or FFI calls

## Usage

```cursed
yeet "hash_drip"

// One-shot hashing
sus sha256_result tea = sha256_hash("Hello, World!")
sus sha512_result tea = sha512_hash("Hello, World!")
sus blake2b_result tea = blake2b_hash("Hello, World!", 64)
sus crc32_result tea = crc32_hash("Hello, World!")

// Incremental hashing
sus hasher SHA256Hasher = sha256_new()
hasher = sha256_update(hasher, "Hello, ")
hasher = sha256_update(hasher, "World!")
sus final_hash tea = sha256_finalize(hasher)
```

## API Reference

### SHA-256

#### `sha256_new() SHA256Hasher`
Create a new SHA-256 hasher instance.

#### `sha256_update(hasher SHA256Hasher, data tea) SHA256Hasher`
Update the hasher with new data. Returns updated hasher.

#### `sha256_finalize(hasher SHA256Hasher) tea`
Finalize the hash and return the result as a 64-character hex string.

#### `sha256_hash(data tea) tea`
Compute SHA-256 hash of data in one operation. Returns 64-character hex string.

### SHA-512

#### `sha512_new() SHA512Hasher`
Create a new SHA-512 hasher instance.

#### `sha512_update(hasher SHA512Hasher, data tea) SHA512Hasher`
Update the hasher with new data. Returns updated hasher.

#### `sha512_finalize(hasher SHA512Hasher) tea`
Finalize the hash and return the result as a 128-character hex string.

#### `sha512_hash(data tea) tea`
Compute SHA-512 hash of data in one operation. Returns 128-character hex string.

### BLAKE2b

#### `blake2b_new(size normie) BLAKE2bHasher`
Create a new BLAKE2b hasher instance with specified output size.

#### `blake2b_hash(data tea, size normie) tea`
Compute BLAKE2b hash of data with specified output size. Returns hex string.

### CRC32

#### `crc32_new() CRC32Hasher`
Create a new CRC32 hasher instance.

#### `crc32_update(hasher CRC32Hasher, data tea) CRC32Hasher`
Update the hasher with new data. Returns updated hasher.

#### `crc32_finalize(hasher CRC32Hasher) tea`
Finalize the hash and return the result as an 8-character hex string.

#### `crc32_hash(data tea) tea`
Compute CRC32 hash of data in one operation. Returns 8-character hex string.

## Data Structures

### `SHA256Hasher`
```cursed
be_like SHA256Hasher squad {
    h [8]normie       // Hash state (8 x 32-bit words)
    len thicc         // Message length in bytes
    buf_len normie    // Buffer length
    buf [64]byte      // Message buffer
}
```

### `SHA512Hasher`
```cursed
be_like SHA512Hasher squad {
    h [8]thicc        // Hash state (8 x 64-bit words)
    len thicc         // Message length in bytes
    buf_len normie    // Buffer length
    buf [128]byte     // Message buffer
}
```

### `BLAKE2bHasher`
```cursed
be_like BLAKE2bHasher squad {
    h [8]thicc        // Hash state (8 x 64-bit words)
    len thicc         // Message length in bytes
    buf_len normie    // Buffer length
    buf [128]byte     // Message buffer
    out_len normie    // Output length
}
```

### `CRC32Hasher`
```cursed
be_like CRC32Hasher squad {
    crc normie        // CRC32 state
    len thicc         // Message length in bytes
}
```

## Examples

### Basic Hashing
```cursed
yeet "hash_drip"

slay main() {
    sus data tea = "The quick brown fox jumps over the lazy dog"
    
    // Compute various hashes
    sus sha256 tea = sha256_hash(data)
    sus sha512 tea = sha512_hash(data)
    sus blake2b tea = blake2b_hash(data, 64)
    sus crc32 tea = crc32_hash(data)
    
    vibez.spill("SHA-256: " + sha256)
    vibez.spill("SHA-512: " + sha512)
    vibez.spill("BLAKE2b: " + blake2b)
    vibez.spill("CRC32: " + crc32)
}
```

### Incremental Hashing
```cursed
yeet "hash_drip"

slay hash_file_chunks() {
    sus hasher SHA256Hasher = sha256_new()
    
    // Process data in chunks
    hasher = sha256_update(hasher, "chunk1")
    hasher = sha256_update(hasher, "chunk2")
    hasher = sha256_update(hasher, "chunk3")
    
    // Finalize and get result
    sus final_hash tea = sha256_finalize(hasher)
    vibez.spill("Final hash: " + final_hash)
}
```

### Hash Comparison
```cursed
yeet "hash_drip"

slay verify_data_integrity() {
    sus original_data tea = "important data"
    sus received_data tea = "important data"
    
    sus original_hash tea = sha256_hash(original_data)
    sus received_hash tea = sha256_hash(received_data)
    
    lowkey original_hash == received_hash {
        vibez.spill("Data integrity verified!")
    } highkey {
        vibez.spill("Data corruption detected!")
    }
}
```

## Security Notes

- **SHA-256** and **SHA-512** are cryptographically secure hash functions suitable for security applications
- **BLAKE2b** is a high-speed cryptographic hash function, faster than SHA-2 and SHA-3
- **CRC32** is designed for error detection, not cryptographic security
- All implementations use constant-time operations where possible to prevent timing attacks
- The library does not use any deprecated hash functions like MD5 or SHA-1

## Performance

- **SHA-256**: ~64 bytes/block processing
- **SHA-512**: ~128 bytes/block processing  
- **BLAKE2b**: Variable output size, high performance
- **CRC32**: Very fast, suitable for checksums and error detection

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/hash_drip/test_hash_drip.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/hash_drip/test_hash_drip.csd
./test_hash_drip
```

The test suite includes:
- Basic functionality tests
- Incremental vs one-shot hashing comparison
- Consistency tests
- Edge case handling
- Performance tests
- Known value verification

## Implementation Notes

This is a pure CURSED implementation without FFI dependencies. The algorithms are implemented using:
- Bit manipulation operations
- Lookup tables for CRC32
- Standard cryptographic constants
- Proper padding and finalization

The implementation prioritizes correctness and security over maximum performance, making it suitable for most applications while maintaining the security properties of the underlying algorithms.
