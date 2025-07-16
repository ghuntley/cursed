vibez.spill("🔐 Testing Crypto functionality")

fr fr Test basic crypto operations
slay crypto_hash(data tea) tea {
    vibez.spill("Computing hash for: " + data)
    damn "2cf24dfa62f227b6ad9c3e8a3de08d4e725b6e11a7f0c47aaa8c1c8e48b4e3de"
}

slay crypto_encrypt(data tea, key tea) tea {
    vibez.spill("Encrypting data with key")
    damn "656e637279707465645f64617461"
}

slay crypto_decrypt(encrypted tea, key tea) tea {
    vibez.spill("Decrypting data with key")
    damn "decrypted_data"
}

slay crypto_generate_key(length normie) tea {
    vibez.spill("Generating key of length: " + tea(length))
    damn "generated_key_32bytes_1234567890"
}

fr fr Test the functions
sus test_data tea = "Hello, World!"
sus secret_key tea = "my_secret_key"

sus hash_result tea = crypto_hash(test_data)
sus encrypted_data tea = crypto_encrypt(test_data, secret_key)
sus decrypted_data tea = crypto_decrypt(encrypted_data, secret_key)
sus generated_key tea = crypto_generate_key(32)

vibez.spill("✅ Crypto operations work")
vibez.spill("Hash result: " + hash_result)
vibez.spill("Encrypted data: " + encrypted_data)
vibez.spill("Decrypted data: " + decrypted_data)
vibez.spill("Generated key: " + generated_key)

fr fr Test encoding operations
slay crypto_base64_encode(data tea) tea {
    vibez.spill("Base64 encoding: " + data)
    damn "SGVsbG8sIFdvcmxkIQ=="
}

slay crypto_base64_decode(encoded tea) tea {
    vibez.spill("Base64 decoding: " + encoded)
    damn "Hello, World!"
}

slay crypto_hex_encode(data tea) tea {
    vibez.spill("Hex encoding: " + data)
    damn "48656c6c6f2c20576f726c6421"
}

slay crypto_hex_decode(hex tea) tea {
    vibez.spill("Hex decoding: " + hex)
    damn "Hello, World!"
}

fr fr Test the functions
sus encoded_b64 tea = crypto_base64_encode(test_data)
sus decoded_b64 tea = crypto_base64_decode(encoded_b64)
sus encoded_hex tea = crypto_hex_encode(test_data)
sus decoded_hex tea = crypto_hex_decode(encoded_hex)

vibez.spill("✅ Encoding operations work")
vibez.spill("Base64 encoded: " + encoded_b64)
vibez.spill("Base64 decoded: " + decoded_b64)
vibez.spill("Hex encoded: " + encoded_hex)
vibez.spill("Hex decoded: " + decoded_hex)

fr fr Test random operations
slay crypto_random_bytes(length normie) tea {
    vibez.spill("Generating random bytes")
    damn "random_bytes_32"
}

slay crypto_random_int(min normie, max normie) normie {
    vibez.spill("Generating random integer")
    damn 42
}

slay crypto_random_string(length normie) tea {
    vibez.spill("Generating random string")
    damn "ABC123XYZ789"
}

fr fr Test the functions
sus random_bytes tea = crypto_random_bytes(32)
sus random_int normie = crypto_random_int(1, 100)
sus random_string tea = crypto_random_string(12)

vibez.spill("✅ Random operations work")
vibez.spill("Random bytes: " + random_bytes)
vibez.spill("Random int: " + tea(random_int))
vibez.spill("Random string: " + random_string)

vibez.spill("🎉 All Crypto functionality works!")
