yeet "testz"
yeet "crypto_secure"

test_start("Crypto Secure Module Tests")

fr fr === Secure RNG Tests ===
test_case("Secure RNG Initialization") {
    sus entropy1 normie = 0x12345678
    sus entropy2 normie = 0x87654321
    sus entropy3 normie = 0xabcdef00
    
    crypto_secure_seed(entropy1, entropy2, entropy3)
    
    fr fr Should initialize internal state
    fr fr Test by generating values - they should be different
    sus val1 normie = crypto_secure_random_u32()
    sus val2 normie = crypto_secure_random_u32()
    
    assert_not_equal(val1, val2)
}

test_case("Secure Random Number Generation") {
    fr fr Initialize with known entropy
    crypto_secure_seed(0x11111111, 0x22222222, 0x33333333)
    
    fr fr Generate multiple random values
    sus values normie[10] = [0; 10]
    bestie (sus i normie = 0; i < 10; i++) {
        values[i] = crypto_secure_random_u32()
    }
    
    fr fr Check that values are different (extremely high probability)
    bestie (sus i normie = 0; i < 9; i++) {
        bestie (sus j normie = i + 1; j < 10; j++) {
            assert_not_equal(values[i], values[j])
        }
    }
}

test_case("Secure Random Bytes Generation") {
    sus buffer drip[32] = [0; 32]
    
    crypto_secure_random_bytes(buffer.ptr(), 32)
    
    fr fr Check that not all bytes are zero
    sus non_zero_count normie = 0
    bestie (sus i normie = 0; i < 32; i++) {
        yo buffer[i] != 0 {
            non_zero_count = non_zero_count + 1
        }
    }
    
    fr fr Very high probability that at least some bytes are non-zero
    assert_greater_than(non_zero_count, 0)
}

test_case("Secure Random Range") {
    fr fr Test random values within range
    bestie (sus i normie = 0; i < 100; i++) {
        sus val normie = crypto_secure_random_range(1, 10)
        assert_greater_than_or_equal(val, 1)
        assert_less_than_or_equal(val, 10)
    }
}

fr fr === ChaCha20 Encryption Tests ===
test_case("ChaCha20 Basic Encryption") {
    sus key drip[32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
    ]
    
    sus nonce drip[12] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a,
        0x00, 0x00, 0x00, 0x00
    ]
    
    sus plaintext tea = "Hello, CURSED Crypto!"
    sus ciphertext tea = chacha20_encrypt(plaintext, key, nonce)
    
    assert_not_equal(ciphertext, plaintext)
    assert_eq_int(ciphertext.len(), plaintext.len())
    
    fr fr Decrypt and verify
    sus decrypted tea = chacha20_decrypt(ciphertext, key, nonce)
    assert_eq_string(decrypted, plaintext)
}

test_case("ChaCha20 Empty Data") {
    sus key drip[32] = [0x42; 32]
    sus nonce drip[12] = [0x00; 12]
    sus empty_data tea = ""
    
    sus encrypted tea = chacha20_encrypt(empty_data, key, nonce)
    assert_eq_string(encrypted, "")
    
    sus decrypted tea = chacha20_decrypt(encrypted, key, nonce)
    assert_eq_string(decrypted, "")
}

test_case("ChaCha20 Large Data") {
    sus key drip[32] = [0x55; 32]
    sus nonce drip[12] = [0xaa; 12]
    
    fr fr Create large plaintext
    sus large_text tea = ""
    bestie (sus i normie = 0; i < 1000; i++) {
        large_text = large_text + "CURSED encryption test data block " + string(i) + "\n"
    }
    
    sus encrypted tea = chacha20_encrypt(large_text, key, nonce)
    sus decrypted tea = chacha20_decrypt(encrypted, key, nonce)
    
    assert_eq_string(decrypted, large_text)
    assert_not_equal(encrypted, large_text)
}

fr fr === Poly1305 MAC Tests ===
test_case("Poly1305 MAC Generation") {
    sus key drip[32] = [
        0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33,
        0x7f, 0x44, 0x52, 0xfe, 0x42, 0xd5, 0x06, 0xa8,
        0x01, 0x03, 0x80, 0x8a, 0xfb, 0x0d, 0xb2, 0xfd,
        0x4a, 0xbf, 0xf6, 0xaf, 0x41, 0x49, 0xf5, 0x1b
    ]
    
    sus message tea = "Cryptographic Message Authentication"
    sus mac drip[16] = poly1305_generate_mac(message, key)
    
    fr fr MAC should not be all zeros
    sus non_zero_bytes normie = 0
    bestie (sus i normie = 0; i < 16; i++) {
        yo mac[i] != 0 {
            non_zero_bytes = non_zero_bytes + 1
        }
    }
    assert_greater_than(non_zero_bytes, 0)
}

test_case("Poly1305 MAC Verification") {
    sus key drip[32] = [0x77; 32]
    sus message tea = "Test message for MAC verification"
    
    sus mac drip[16] = poly1305_generate_mac(message, key)
    
    fr fr Verify correct MAC
    sus is_valid lit = poly1305_verify_mac(message, key, mac)
    assert_eq_bool(is_valid, based)
    
    fr fr Modify message and verify it fails
    sus modified_message tea = "Test message for MAC verification!"
    sus is_invalid lit = poly1305_verify_mac(modified_message, key, mac)
    assert_eq_bool(is_invalid, cap)
}

test_case("Poly1305 Different Keys Different MACs") {
    sus key1 drip[32] = [0x11; 32]
    sus key2 drip[32] = [0x22; 32]
    sus message tea = "Same message, different keys"
    
    sus mac1 drip[16] = poly1305_generate_mac(message, key1)
    sus mac2 drip[16] = poly1305_generate_mac(message, key2)
    
    fr fr MACs should be different with different keys
    sus are_different lit = cap
    bestie (sus i normie = 0; i < 16; i++) {
        yo mac1[i] != mac2[i] {
            are_different = based
            vibes
        }
    }
    assert_eq_bool(are_different, based)
}

fr fr === SHA-512 Hash Tests ===
test_case("SHA-512 Basic Hashing") {
    sus input tea = "The quick brown fox jumps over the lazy dog"
    sus hash drip[64] = sha512_hash(input)
    
    fr fr Check hash is not all zeros
    sus non_zero_bytes normie = 0
    bestie (sus i normie = 0; i < 64; i++) {
        yo hash[i] != 0 {
            non_zero_bytes = non_zero_bytes + 1
        }
    }
    assert_greater_than(non_zero_bytes, 32)  fr fr Should have plenty of non-zero bytes
}

test_case("SHA-512 Deterministic") {
    sus input tea = "Deterministic test input"
    
    sus hash1 drip[64] = sha512_hash(input)
    sus hash2 drip[64] = sha512_hash(input)
    
    fr fr Should be identical
    bestie (sus i normie = 0; i < 64; i++) {
        assert_eq_int(hash1[i], hash2[i])
    }
}

test_case("SHA-512 Different Inputs") {
    sus input1 tea = "Input 1"
    sus input2 tea = "Input 2"
    
    sus hash1 drip[64] = sha512_hash(input1)
    sus hash2 drip[64] = sha512_hash(input2)
    
    fr fr Hashes should be different
    sus are_different lit = cap
    bestie (sus i normie = 0; i < 64; i++) {
        yo hash1[i] != hash2[i] {
            are_different = based
            vibes
        }
    }
    assert_eq_bool(are_different, based)
}

test_case("SHA-512 Empty Input") {
    sus empty_input tea = ""
    sus hash drip[64] = sha512_hash(empty_input)
    
    fr fr Should produce a valid hash even for empty input
    sus is_all_zeros lit = based
    bestie (sus i normie = 0; i < 64; i++) {
        yo hash[i] != 0 {
            is_all_zeros = cap
            vibes
        }
    }
    assert_eq_bool(is_all_zeros, cap)  fr fr Should not be all zeros
}

fr fr === BLAKE2b Hash Tests ===
test_case("BLAKE2b Basic Hashing") {
    sus input tea = "BLAKE2b test input"
    sus hash drip[64] = blake2b_hash(input)
    
    fr fr Verify non-trivial hash
    sus non_zero_count normie = 0
    bestie (sus i normie = 0; i < 64; i++) {
        yo hash[i] != 0 {
            non_zero_count = non_zero_count + 1
        }
    }
    assert_greater_than(non_zero_count, 20)
}

test_case("BLAKE2b Keyed Hash") {
    sus key drip[32] = [0xab; 32]
    sus input tea = "Keyed BLAKE2b test"
    
    sus hash1 drip[64] = blake2b_hash_keyed(input, key)
    sus hash2 drip[64] = blake2b_hash(input)
    
    fr fr Keyed and unkeyed hashes should be different
    sus are_different lit = cap
    bestie (sus i normie = 0; i < 64; i++) {
        yo hash1[i] != hash2[i] {
            are_different = based
            vibes
        }
    }
    assert_eq_bool(are_different, based)
}

fr fr === X25519 Key Exchange Tests ===
test_case("X25519 Key Generation") {
    sus private_key drip[32] = [0; 32]
    sus public_key drip[32] = [0; 32]
    
    x25519_generate_keypair(private_key, public_key)
    
    fr fr Keys should not be all zeros
    sus private_non_zero lit = cap
    sus public_non_zero lit = cap
    
    bestie (sus i normie = 0; i < 32; i++) {
        yo private_key[i] != 0 {
            private_non_zero = based
        }
        yo public_key[i] != 0 {
            public_non_zero = based
        }
    }
    
    assert_eq_bool(private_non_zero, based)
    assert_eq_bool(public_non_zero, based)
}

test_case("X25519 Key Exchange") {
    sus alice_private drip[32] = [0; 32]
    sus alice_public drip[32] = [0; 32]
    sus bob_private drip[32] = [0; 32]
    sus bob_public drip[32] = [0; 32]
    
    x25519_generate_keypair(alice_private, alice_public)
    x25519_generate_keypair(bob_private, bob_public)
    
    sus shared_secret1 drip[32] = [0; 32]
    sus shared_secret2 drip[32] = [0; 32]
    
    x25519_compute_shared_secret(alice_private, bob_public, shared_secret1)
    x25519_compute_shared_secret(bob_private, alice_public, shared_secret2)
    
    fr fr Shared secrets should be identical
    bestie (sus i normie = 0; i < 32; i++) {
        assert_eq_int(shared_secret1[i], shared_secret2[i])
    }
}

fr fr === Ed25519 Signature Tests ===
test_case("Ed25519 Signature Generation") {
    sus private_key drip[32] = [0; 32]
    sus public_key drip[32] = [0; 32]
    
    ed25519_generate_keypair(private_key, public_key)
    
    sus message tea = "Message to be signed"
    sus signature drip[64] = ed25519_sign(message, private_key)
    
    fr fr Signature should not be all zeros
    sus non_zero_bytes normie = 0
    bestie (sus i normie = 0; i < 64; i++) {
        yo signature[i] != 0 {
            non_zero_bytes = non_zero_bytes + 1
        }
    }
    assert_greater_than(non_zero_bytes, 0)
}

test_case("Ed25519 Signature Verification") {
    sus private_key drip[32] = [0; 32]
    sus public_key drip[32] = [0; 32]
    
    ed25519_generate_keypair(private_key, public_key)
    
    sus message tea = "Authentic message"
    sus signature drip[64] = ed25519_sign(message, private_key)
    
    fr fr Valid signature should verify
    sus is_valid lit = ed25519_verify(message, signature, public_key)
    assert_eq_bool(is_valid, based)
    
    fr fr Modified message should not verify
    sus modified_message tea = "Authentic message!"
    sus is_invalid lit = ed25519_verify(modified_message, signature, public_key)
    assert_eq_bool(is_invalid, cap)
}

fr fr === PBKDF2 Key Derivation Tests ===
test_case("PBKDF2 Key Derivation") {
    sus password tea = "secure_password_123"
    sus salt drip[16] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
                        0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10]
    sus iterations normie = 10000
    sus derived_key drip[32] = [0; 32]
    
    pbkdf2_derive_key(password, salt, iterations, derived_key)
    
    fr fr Derived key should not be all zeros
    sus non_zero_count normie = 0
    bestie (sus i normie = 0; i < 32; i++) {
        yo derived_key[i] != 0 {
            non_zero_count = non_zero_count + 1
        }
    }
    assert_greater_than(non_zero_count, 16)
}

test_case("PBKDF2 Different Passwords") {
    sus password1 tea = "password1"
    sus password2 tea = "password2"
    sus salt drip[8] = [0x12; 8]
    sus iterations normie = 1000
    
    sus key1 drip[32] = [0; 32]
    sus key2 drip[32] = [0; 32]
    
    pbkdf2_derive_key(password1, salt, iterations, key1)
    pbkdf2_derive_key(password2, salt, iterations, key2)
    
    fr fr Keys should be different
    sus are_different lit = cap
    bestie (sus i normie = 0; i < 32; i++) {
        yo key1[i] != key2[i] {
            are_different = based
            vibes
        }
    }
    assert_eq_bool(are_different, based)
}

fr fr === Constant Time Operations Tests ===
test_case("Constant Time Comparison") {
    sus data1 drip[16] = [0x42; 16]
    sus data2 drip[16] = [0x42; 16]
    sus data3 drip[16] = [0x43; 16]
    
    fr fr Equal arrays should return true
    sus equal lit = crypto_constant_time_equal(data1, data2, 16)
    assert_eq_bool(equal, based)
    
    fr fr Different arrays should return false
    sus different lit = crypto_constant_time_equal(data1, data3, 16)
    assert_eq_bool(different, cap)
}

test_case("Constant Time Memory Clear") {
    sus sensitive_data drip[32] = [0xff; 32]
    
    crypto_secure_zero_memory(sensitive_data, 32)
    
    fr fr All bytes should be zero
    bestie (sus i normie = 0; i < 32; i++) {
        assert_eq_int(sensitive_data[i], 0)
    }
}

fr fr === Security Validation Tests ===
test_case("Entropy Quality Check") {
    sus buffer drip[256] = [0; 256]
    crypto_secure_random_bytes(buffer.ptr(), 256)
    
    fr fr Count unique bytes
    sus byte_counts normie[256] = [0; 256]
    bestie (sus i normie = 0; i < 256; i++) {
        byte_counts[buffer[i]] = byte_counts[buffer[i]] + 1
    }
    
    fr fr Should have reasonable distribution (not all same value)
    sus unique_values normie = 0
    bestie (sus i normie = 0; i < 256; i++) {
        yo byte_counts[i] > 0 {
            unique_values = unique_values + 1
        }
    }
    
    assert_greater_than(unique_values, 50)  fr fr Should have many different values
}

test_case("Timing Attack Resistance") {
    fr fr This test ensures constant-time operations don't leak timing info
    sus correct_mac drip[16] = [0x11; 16]
    sus wrong_mac1 drip[16] = [0x22; 16] 
    sus wrong_mac2 drip[16] = [0x11; 15] + [0x33]  fr fr Differs in last byte
    
    fr fr Both comparisons should take similar time (we can't measure here,
    fr fr but the implementation should use constant-time comparison)
    sus result1 lit = crypto_constant_time_equal(correct_mac, wrong_mac1, 16)
    sus result2 lit = crypto_constant_time_equal(correct_mac, wrong_mac2, 16)
    
    assert_eq_bool(result1, cap)
    assert_eq_bool(result2, cap)
}

print_test_summary()
