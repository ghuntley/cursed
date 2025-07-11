yeet "testz"
yeet "crypto"

fr fr Minimal crypto test
test_start("Crypto SHA-256 Basic Test")

sus hash1 tea = crypto_sha256("hello")
sus hash2 tea = crypto_sha256("hello")

assert_eq_string(hash1, hash2)
assert_true(hash1 != "")

print_test_summary()

vibez.spill("✅ Crypto basic functionality test complete")
