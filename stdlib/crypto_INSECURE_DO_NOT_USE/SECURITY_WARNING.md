# ⚠️ CRITICAL SECURITY WARNING ⚠️

## THIS MODULE CONTAINS INSECURE PLACEHOLDER IMPLEMENTATIONS

**DO NOT USE IN PRODUCTION**

This module has been moved and marked as insecure due to critical security vulnerabilities:

### Critical Issues Found:
- ❌ **Linear Congruential Generator**: Predictable "random" numbers
- ❌ **Fake SHA-256**: Simplified hash with no security properties
- ❌ **Mock AES Encryption**: Trivial XOR operation, easily broken
- ❌ **Hardcoded Return Values**: Functions that ignore inputs
- ❌ **Placeholder Implementations**: No actual cryptographic security

### Use Instead:
```cursed
# SECURE MODULE
yeet "crypto_secure"

# Instead of old insecure functions:
# crypto_sha256() -> crypto_sha256_secure()
# crypto_aes_encrypt() -> crypto_aes256_encrypt_secure()
# next_random() -> crypto_secure_random_u32()
```

### Migration Path:
1. Replace all imports of "crypto" with "crypto_secure"
2. Update function names to use "_secure" variants
3. Re-test all cryptographic operations
4. Review application code for crypto assumptions

**This module will be deleted in a future release.**

For security audit details, see: `CRYPTO_SECURITY_AUDIT_COMPLETE.md`
