# Cursed Standard Library

This directory contains the standard library modules for the Cursed programming language. Each module provides essential functionality for common programming tasks.

## Modules

### Collections (`collections/mod.csd`)
Comprehensive collection data structures and algorithms:
- **Arrays/Vectors**: Creation, manipulation, searching, sorting, functional operations
- **HashMaps**: Key-value storage with full CRUD operations
- **Sets**: Unique collections with set operations (union, intersection, difference)
- **Queues**: FIFO data structure operations
- **Stacks**: LIFO data structure operations
- **Utilities**: Range generation, zipping, flattening, grouping, partitioning

### Cryptography (`crypto/mod.csd`)
Security and cryptographic functions:
- **Hash Functions**: SHA-256, SHA-512, MD5, BLAKE3
- **Random Generation**: Secure random bytes, integers, strings
- **Encoding**: Base64 and hexadecimal encoding/decoding
- **Symmetric Encryption**: AES encryption/decryption
- **Key Derivation**: PBKDF2, Scrypt
- **Digital Signatures**: Ed25519 signing and verification
- **Message Authentication**: HMAC-SHA256/512
- **Password Hashing**: Argon2, bcrypt
- **Security Utilities**: Constant-time comparison, secure random, salt generation

### Input/Output (`io/mod.csd`)
File system and I/O operations:
- **Console I/O**: Print, read input with formatting
- **File Operations**: Read, write, copy, move, delete files
- **Directory Operations**: Create, remove, list directories
- **Path Utilities**: Join, split, resolve paths
- **Stream I/O**: File handles for buffered operations
- **Temporary Files**: Create temporary files and directories

### String Utilities (`string/mod.csd`)
String manipulation and processing:
- **Basic Operations**: Length, trimming, case conversion, reversal
- **Search/Match**: Contains, starts/ends with, indexing, counting
- **Slicing/Splitting**: Substring extraction, splitting by delimiters
- **Formatting**: Replace, repeat, padding, templating
- **Validation**: Numeric, alphabetic, whitespace checks
- **Type Conversion**: String to/from numbers and booleans
- **Regular Expressions**: Pattern matching, finding, replacing
- **Utilities**: Join arrays, edit distance, similarity scoring

### Mathematics (`math/mod.csd`)
Mathematical functions and utilities:
- **Constants**: π, e, τ
- **Basic Operations**: Absolute value, min/max, clamping, sign
- **Power/Logarithms**: Powers, roots, natural/base-10 logarithms
- **Trigonometry**: Sin, cos, tan and their inverses, hyperbolic functions
- **Rounding**: Floor, ceil, round, truncate
- **Statistics**: Sum, mean, median, variance, standard deviation
- **Random Numbers**: Seeded and unseeded random generation
- **Utilities**: NaN/infinity checks, GCD, LCM, factorial, Fibonacci
- **Geometry**: Distance calculations, dot/cross products, normalization

### Time and Date (`time/mod.csd`)
Date, time, and duration handling:
- **Current Time**: Unix timestamps in various precisions
- **Date Creation**: From timestamps, components, parsing
- **Formatting**: Custom formats, ISO8601, RFC3339
- **Components**: Extract year, month, day, hour, minute, second
- **Arithmetic**: Add/subtract time periods, calculate differences
- **Durations**: Create and manipulate time spans
- **Time Zones**: UTC/local conversion, timezone offsets
- **Utilities**: Leap year checks, date validation, weekend detection
- **Performance**: Sleep functions, benchmarking, timing

## Implementation Notes

These modules define the public API for the standard library. Each function calls a corresponding implementation function (prefixed with the module name) that should be provided by the runtime system.

For example:
- `array_push(arr, item)` calls `collections_array_push(arr, item)`
- `sha256(data)` calls `crypto_sha256(data)`
- `print(message)` calls `io_print(message)`

This design allows the standard library API to remain stable while the underlying implementation can be optimized or replaced as needed.

## Usage

Import modules using the standard import syntax:

```cursed
import collections::*
import crypto::sha256
import io::{read_file, write_file}
import string::string_contains as contains
import math::math_pi as PI
import time::time_now
```

Then use the functions directly:

```cursed
let arr = array_new();
array_push(arr, "Hello");
array_push(arr, "World");

let hash = sha256("sensitive data");
let content = read_file("config.txt");

if contains(content, "debug") {
    println("Debug mode enabled");
}
```

## Testing

Each module should have comprehensive test coverage to ensure correctness and reliability. Tests should cover:
- Normal operation with valid inputs
- Edge cases and boundary conditions
- Error handling with invalid inputs
- Performance characteristics for large inputs
