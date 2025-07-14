# crypto_subtle_drip

Subtle cryptographic primitives for preventing side-channel attacks.

## Overview

The `crypto_subtle_drip` module provides constant-time operations to avoid timing attacks in cryptographic implementations. All functions execute in time that depends only on the length of inputs, not their values.

## Key Functions

- `ConstantTimeCompare(x, y []byte) normie` - Compare byte slices in constant time
- `ConstantTimeByteEq(x, y byte) normie` - Compare bytes in constant time
- `ConstantTimeEq(x, y normie) normie` - Compare integers in constant time
- `ConstantTimeSelect(v, x, y normie) normie` - Select values in constant time
- `ConstantTimeCopy(v normie, x, y []byte)` - Conditional copy in constant time

## Enhanced Features

- Secret data handling with automatic memory clearing
- Blinded memory access to prevent cache timing attacks
- Constant-time string operations
- Constant-time arithmetic operations

## Usage

```cursed
yeet "crypto_subtle_drip"

fr fr Secure token comparison
storedToken := []byte{0x01, 0x23, 0x45, 0x67}
receivedToken := []byte{0x01, 0x23, 0x45, 0x67}

isValid := crypto_subtle_drip.ConstantTimeCompare(storedToken, receivedToken) == 1
if isValid {
    vibez.spill("Token is valid")
}
```

## Security Guarantees

- All functions execute in constant time
- No branching based on sensitive data
- Resistant to timing attacks and cache analysis
- Memory is securely cleared when appropriate
