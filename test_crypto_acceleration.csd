yeet "testz"

test_start("Crypto Hardware Acceleration")

sus data tea = "Hello, CURSED!"
sus key tea = "secret_key_12345"

// Should use AES-NI on x86_64 or hardware AES on ARM64
sus encrypted tea = aes_encrypt(data, key)
sus decrypted tea = aes_decrypt(encrypted, key)

assert_eq_string(data, decrypted)

// Test SHA acceleration
sus hash tea = sha256(data)
assert_true(length(hash) == 64) // 32 bytes * 2 hex chars

print_test_summary()
