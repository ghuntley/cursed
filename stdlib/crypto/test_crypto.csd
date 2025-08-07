yeet "testz"
yeet "crypto"

test_start("Crypto module initialization test")
assert_true(based)

test_start("SHA-256 hash test")
sus hash tea = sha256("hello world")
assert_true(string_length(hash) > 0)

test_start("AES encryption test") 
sus key tea = generate_key(256)
sus encrypted tea = aes_encrypt("secret message", key)
sus decrypted tea = aes_decrypt(encrypted, key)
assert_eq_string(decrypted, "secret message")

test_start("HMAC authentication test")
sus message tea = "test message"
sus secret tea = "secret key"
sus mac tea = hmac_sha256(secret, message)
assert_true(string_length(mac) > 0)

test_start("Secure random generation test")
sus random_num normie = secure_random_int(1, 100)
assert_true(random_num >= 1)
assert_true(random_num <= 100)

test_start("Random string generation test")
sus random_str tea = secure_random_string(10)
assert_true(string_length(random_str) >= 10)

test_start("Key generation test")
sus generated_key tea = generate_key(128)
assert_true(string_length(generated_key) > 0)

print_test_summary()
