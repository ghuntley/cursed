// Test script to verify MD5 has been removed for security
yeet "crypto"

vibez.spill("🔒 Testing crypto security hot-fix...")

// Test 1: Verify MD5 is removed (should fail)
vibez.spill("Test 1: Attempting to use MD5 (should fail)")
sus test_result tea = "PASS"

// This should fail with a security error
try {
    sus result tea = crypto_md5("test")
    vibez.spill("❌ SECURITY FAILURE: MD5 still works!")
    test_result = "FAIL"
} catch {
    vibez.spill("✅ SECURITY SUCCESS: MD5 correctly removed")
}

// Test 2: Verify SHA-256 still works
vibez.spill("Test 2: Testing SHA-256 (should work)")
sus sha256_result tea = crypto_sha256("hello world")
vibez.spill("SHA-256 result: " + sha256_result)

// Test 3: Verify BLAKE3 still works
vibez.spill("Test 3: Testing BLAKE3 (should work)")
sus blake3_result tea = crypto_blake3("hello world")
vibez.spill("BLAKE3 result: " + blake3_result)

vibez.spill("🔐 Security hot-fix test complete: " + test_result)
