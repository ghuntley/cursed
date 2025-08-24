# Crypto Modules Functionality Test
yeet "cryptz"
yeet "tls_vibe"

vibez.spill("=== CRYPTO MODULES TEST ===")

# Test cryptz module
vibez.spill("Testing cryptz.hash_sha256...")
sus test_data tea = "Hello, World!"
sus hash_result tea = cryptz.hash_sha256(test_data)
vibez.spill("cryptz.hash_sha256 result:", hash_result)

vibez.spill("Testing cryptz.hash_md5...")
sus md5_result tea = cryptz.hash_md5(test_data)
vibez.spill("cryptz.hash_md5 result:", md5_result)

vibez.spill("Testing cryptz.encrypt_aes...")
sus key tea = "secretkey1234567"  # 16 bytes
sus plaintext tea = "This is secret"
sus encrypted tea = cryptz.encrypt_aes(plaintext, key)
vibez.spill("cryptz.encrypt_aes result length:", stringz.len(encrypted))

vibez.spill("Testing cryptz.decrypt_aes...")
sus decrypted tea = cryptz.decrypt_aes(encrypted, key)
vibez.spill("cryptz.decrypt_aes result:", decrypted)

vibez.spill("Testing cryptz.generate_key...")
sus generated_key tea = cryptz.generate_key(32)
vibez.spill("cryptz.generate_key(32) length:", stringz.len(generated_key))

# Test tls_vibe module
vibez.spill("Testing tls_vibe.create_context...")
sus tls_context dict = tls_vibe.create_context()
vibez.spill("tls_vibe.create_context completed")

vibez.spill("Testing tls_vibe.set_cert...")
sus cert_result lit = tls_vibe.set_cert(tls_context, "dummy_cert.pem")
vibez.spill("tls_vibe.set_cert result:", cert_result)

vibez.spill("Testing tls_vibe.handshake...")
sus handshake_result lit = tls_vibe.handshake(tls_context, "example.com", 443)
vibez.spill("tls_vibe.handshake result:", handshake_result)

vibez.spill("=== CRYPTO MODULES TEST COMPLETE ===")
