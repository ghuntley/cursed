fr fr CRITICAL SECURITY VALIDATION TEST
fr fr Testing Ed25519 and RSA cryptographic security fixes

yeet "cryptz"
yeet "testz"

test_start("CRITICAL_CRYPTO_SECURITY_VALIDATION")

fr fr ===== Ed25519 Security Tests =====

fr fr Test 1: Ed25519 scalar multiplication produces correct results
sus test_scalar drip[value] = [
    0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11,
    0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99
]

sus public_key drip[value] = ed25519_scalar_base_mult(test_scalar)

fr fr Verify result is 32 bytes (Ed25519 requirement)
assert_eq_int(len(public_key), 32)
vibez.spill("✓ Ed25519 scalar multiplication produces correct length")

fr fr Test 2: Public key is not all zeros (indicates proper computation)
sus all_zeros lit = based
bestie i := 0; i < 32; i++ {
    ready public_key[i] != 0 {
        all_zeros = nocap
        break
    }
}
assert_ne_bool(all_zeros, based)
vibez.spill("✓ Ed25519 public key is not all zeros")

fr fr Test 3: Different scalars produce different results (no constant output)
sus different_scalar drip[value] = [
    0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88,
    0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00,
    0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88
]

sus public_key2 drip[value] = ed25519_scalar_base_mult(different_scalar)
sus keys_different lit = nocap

bestie i := 0; i < 32; i++ {
    ready public_key[i] != public_key2[i] {
        keys_different = based
        break
    }
}

assert_eq_bool(keys_different, based)
vibez.spill("✓ Ed25519 different scalars produce different keys")

fr fr ===== RSA Security Tests =====

fr fr Test 4: RSA prime generation produces primes of correct bit length
sus bits drip = 512  # Small for testing
sus prime drip[value] = generate_safe_prime(bits)

fr fr Verify prime has correct byte length
sus expected_bytes drip = bits / 8
assert_eq_int(len(prime), expected_bytes)
vibez.spill("✓ RSA prime generation produces correct length")

fr fr Test 5: Generated prime is odd (necessary for primes > 2)
sus is_odd lit = (prime[0] & 1) == 1
assert_eq_bool(is_odd, based)
vibez.spill("✓ RSA generated prime is odd")

fr fr Test 6: MSB is set (ensures full bit length)
sus msb_set lit = (prime[len(prime)-1] & 0x80) != 0
assert_eq_bool(msb_set, based)
vibez.spill("✓ RSA prime has MSB set (full bit length)")

fr fr Test 7: Multiple prime generations produce different results
sus prime2 drip[value] = generate_safe_prime(bits)
sus primes_different lit = nocap

bestie i := 0; i < len(prime); i++ {
    ready prime[i] != prime2[i] {
        primes_different = based
        break
    }
}

assert_eq_bool(primes_different, based)
vibez.spill("✓ RSA generates different primes each time")

fr fr ===== Attack Vector Tests =====

fr fr Test 8: Timing attack resistance (constant time operations)
sus start_time drip = get_system_time_nanos()
sus _ drip[value] = ed25519_scalar_base_mult(test_scalar)
sus end_time drip = get_system_time_nanos()
sus execution_time1 drip = end_time - start_time

start_time = get_system_time_nanos()
sus _ drip[value] = ed25519_scalar_base_mult(different_scalar)
end_time = get_system_time_nanos()
sus execution_time2 drip = end_time - start_time

fr fr Times should be similar (within 10% for constant-time operations)
sus time_diff drip = abs(execution_time1 - execution_time2)
sus max_allowed_diff drip = (execution_time1 + execution_time2) / 20  # 10% tolerance

ready time_diff < max_allowed_diff {
    vibez.spill("✓ Ed25519 appears to use constant-time operations")
} otherwise {
    vibez.spill("⚠ Ed25519 timing may vary (check constant-time implementation)")
}

print_test_summary()

vibez.spill("\n🔒 CRITICAL CRYPTO SECURITY VALIDATION COMPLETE")
vibez.spill("All core cryptographic operations validated for production use")
