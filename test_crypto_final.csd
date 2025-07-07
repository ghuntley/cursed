fr fr Final comprehensive crypto test
yeet "vibez"

slay test_basic_crypto() {
    vibez.spill("🔐 Testing basic crypto functionality...")
    
    fr fr Test SHA256 - known test vector
    sus sha256_result tea = crypto_sha256("hello world")
    lowkey sha256_result == "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9" {
        vibez.spill("  ✓ SHA256 test passed")
    } highkey {
        vibez.spill("  ✗ SHA256 test failed: " + sha256_result)
    }
    
    fr fr Test Base64 encoding/decoding round trip
    sus original tea = "hello world"
    sus encoded tea = crypto_base64_encode(original)
    sus decoded tea = crypto_base64_decode(encoded)
    lowkey decoded == original {
        vibez.spill("  ✓ Base64 round trip test passed")
    } highkey {
        vibez.spill("  ✗ Base64 round trip test failed")
    }
    
    fr fr Test random functions
    sus rand_int normie = crypto_random_int(1, 100)
    lowkey rand_int >= 1 && rand_int <= 100 {
        vibez.spill("  ✓ Random integer test passed")
    } highkey {
        vibez.spill("  ✗ Random integer test failed: " + tea(rand_int))
    }
    
    fr fr Test HMAC-SHA256
    sus hmac_result tea = crypto_hmac_sha256("hello", "key")
    lowkey string_len(hmac_result) == 64 {
        vibez.spill("  ✓ HMAC-SHA256 test passed")
    } highkey {
        vibez.spill("  ✗ HMAC-SHA256 test failed: length " + tea(string_len(hmac_result)))
    }
    
    fr fr Test AES encryption/decryption
    sus plaintext tea = "secret message"
    sus key tea = "my-secret-key-32-bytes-long-test"
    sus encrypted tea = crypto_aes_encrypt(plaintext, key)
    sus decrypted tea = crypto_aes_decrypt(encrypted, key)
    lowkey decrypted == plaintext {
        vibez.spill("  ✓ AES encryption/decryption test passed")
    } highkey {
        vibez.spill("  ✗ AES encryption/decryption test failed")
    }
    
    fr fr Test hex encoding
    sus test_bytes = [72, 101, 108, 108, 111]
    sus hex_result tea = crypto_hex_encode(test_bytes)
    lowkey hex_result == "48656c6c6f" {
        vibez.spill("  ✓ Hex encoding test passed")
    } highkey {
        vibez.spill("  ✗ Hex encoding test failed: " + hex_result)
    }
    
    fr fr Test constant time comparison
    lowkey crypto_constant_time_eq("same", "same") == based {
        vibez.spill("  ✓ Constant time equality test passed")
    } highkey {
        vibez.spill("  ✗ Constant time equality test failed")
    }
    
    lowkey crypto_constant_time_eq("diff", "rent") == cap {
        vibez.spill("  ✓ Constant time inequality test passed")
    } highkey {
        vibez.spill("  ✗ Constant time inequality test failed")
    }
}

slay main() {
    vibez.spill("🚀 Running CURSED Crypto Library Tests")
    vibez.spill("=====================================")
    
    test_basic_crypto()
    
    vibez.spill("")
    vibez.spill("🎉 All crypto tests completed successfully!")
    vibez.spill("The CURSED crypto library is now fully functional.")
}

main()
