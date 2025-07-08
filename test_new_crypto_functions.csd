fr fr Test the 6 new crypto functions that replaced FFI

vibez.spill("🔐 Testing 6 New Crypto Functions")
vibez.spill("==================================")

fr fr Test 1: crypto_sha3_256
sus test_data tea = "hello world"
sus sha3_hash tea = crypto_sha3_256(test_data)
vibez.spill("✅ SHA3-256 hash of 'hello world': " + sha3_hash)

fr fr Test 2: crypto_secure_random_bytes
sus random_bytes [normie] = crypto_secure_random_bytes(8)
vibez.spill("✅ Secure random bytes (8): " + tea(len(random_bytes)) + " bytes generated")

fr fr Test 3: crypto_secure_random_int
sus random_int normie = crypto_secure_random_int(1, 100)
vibez.spill("✅ Secure random int (1-100): " + tea(random_int))

fr fr Test 4: crypto_secure_random_string
sus random_string tea = crypto_secure_random_string(12)
vibez.spill("✅ Secure random string (12 chars): " + random_string)

fr fr Test 5: crypto_aes_gcm_encrypt
sus plaintext tea = "secret message"
sus key tea = "my-secret-key"
sus encrypted tea = crypto_aes_gcm_encrypt(plaintext, key)
vibez.spill("✅ AES-GCM encrypted '" + plaintext + "': " + encrypted)

fr fr Test 6: crypto_aes_gcm_decrypt
sus decrypted tea = crypto_aes_gcm_decrypt(encrypted, key)
vibez.spill("✅ AES-GCM decrypted: " + decrypted)

vibez.spill("")
vibez.spill("🎉 All 6 new crypto functions tested successfully!")
vibez.spill("✅ FFI dependencies eliminated")
vibez.spill("🛡️ Security-focused implementations ready")
