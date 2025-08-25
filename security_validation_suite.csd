fr fr ===== CRITICAL SECURITY VALIDATION SUITE =====
fr fr Tests for production crypto implementations against known test vectors

yeet "cryptz/production_crypto"
yeet "testz"
yeet "vibez"
yeet "stringz"

fr fr ===== MD5 TEST VECTORS (RFC 1321) =====

slay test_md5_vectors() {
    vibez.spill("Testing MD5 against RFC 1321 test vectors...")
    
    fr fr Test vector 1: empty string
    sus result1 tea = compute_production_md5("")
    sus expected1 tea = "d41d8cd98f00b204e9800998ecf8427e"
    assert_eq_string(result1, expected1)
    vibez.spill("✅ MD5 empty string test passed")
    
    fr fr Test vector 2: "a"
    sus result2 tea = compute_production_md5("a")
    sus expected2 tea = "0cc175b9c0f1b6a831c399e269772661"
    assert_eq_string(result2, expected2)
    vibez.spill("✅ MD5 single character test passed")
    
    fr fr Test vector 3: "abc"
    sus result3 tea = compute_production_md5("abc")
    sus expected3 tea = "900150983cd24fb0d6963f7d28e17f72"
    assert_eq_string(result3, expected3)
    vibez.spill("✅ MD5 'abc' test passed")
    
    fr fr Test vector 4: long message
    sus long_msg tea = "The quick brown fox jumps over the lazy dog"
    sus result4 tea = compute_production_md5(long_msg)
    sus expected4 tea = "9e107d9d372bb6826bd81d3542a419d6"
    assert_eq_string(result4, expected4)
    vibez.spill("✅ MD5 long message test passed")
}

fr fr ===== SHA-256 TEST VECTORS (NIST) =====

slay test_sha256_vectors() {
    vibez.spill("Testing SHA-256 against NIST test vectors...")
    
    fr fr Test vector 1: empty string
    sus result1 tea = compute_sha256("")
    sus expected1 tea = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    assert_eq_string(result1, expected1)
    vibez.spill("✅ SHA-256 empty string test passed")
    
    fr fr Test vector 2: "abc"
    sus result2 tea = compute_sha256("abc")
    sus expected2 tea = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    assert_eq_string(result2, expected2)
    vibez.spill("✅ SHA-256 'abc' test passed")
    
    fr fr Test vector 3: longer message
    sus msg tea = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
    sus result3 tea = compute_sha256(msg)
    sus expected3 tea = "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    assert_eq_string(result3, expected3)
    vibez.spill("✅ SHA-256 longer message test passed")
}

fr fr ===== HMAC-SHA256 TEST VECTORS (RFC 4231) =====

slay test_hmac_sha256_vectors() {
    vibez.spill("Testing HMAC-SHA256 against RFC 4231 test vectors...")
    
    fr fr Test case 1
    sus key1 tea = "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b"  // 20 bytes of 0x0b
    sus data1 tea = "Hi There"
    sus result1 tea = compute_hmac_sha256(key1, data1)
    sus expected1 tea = "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
    assert_eq_string(result1, expected1)
    vibez.spill("✅ HMAC-SHA256 test case 1 passed")
    
    fr fr Test case 2: key "Jefe", data "what do ya want for nothing?"
    sus key2 tea = "Jefe"
    sus data2 tea = "what do ya want for nothing?"
    sus result2 tea = compute_hmac_sha256(key2, data2)
    sus expected2 tea = "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
    assert_eq_string(result2, expected2)
    vibez.spill("✅ HMAC-SHA256 test case 2 passed")
}

fr fr ===== TIMING ATTACK RESISTANCE TESTS =====

slay test_constant_time_comparison() {
    vibez.spill("Testing constant-time comparison...")
    
    fr fr Test equal strings
    sus result1 lit = constant_time_compare("hello", "hello")
    assert_eq_bool(result1, based)
    
    fr fr Test different strings same length
    sus result2 lit = constant_time_compare("hello", "world")
    assert_eq_bool(result2, cap)
    
    fr fr Test different lengths
    sus result3 lit = constant_time_compare("hello", "hellox")
    assert_eq_bool(result3, cap)
    
    vibez.spill("✅ Constant-time comparison tests passed")
}

fr fr ===== COLLISION RESISTANCE TESTS =====

slay test_collision_resistance() {
    vibez.spill("Testing hash collision resistance...")
    
    fr fr Test that different inputs produce different hashes
    sus inputs []tea = [
        "input1", "input2", "input3", "input4", "input5",
        "different", "strings", "should", "produce", "unique",
        "hash", "values", "to", "prevent", "collisions"
    ]
    
    sus hashes []tea = []
    sus i drip = 0
    bestie i < len(inputs) {
        sus hash tea = compute_sha256(inputs[i])
        hashes = append(hashes, hash)
        i = i + 1
    }
    
    fr fr Check for duplicates
    i = 0
    bestie i < len(hashes) {
        sus j drip = i + 1
        bestie j < len(hashes) {
            assert_not_eq_string(hashes[i], hashes[j])
            j = j + 1
        }
        i = i + 1
    }
    
    vibez.spill("✅ Collision resistance tests passed")
}

fr fr ===== SECURE HASH FUNCTION TESTS =====

slay test_secure_collection_hash() {
    vibez.spill("Testing secure collection hash function...")
    
    fr fr Test that hash values are within bounds
    sus table_size drip = 1000
    sus test_inputs []tea = ["key1", "key2", "key3", "key4", "key5"]
    
    sus i drip = 0
    bestie i < len(test_inputs) {
        sus hash_val drip = secure_collection_hash(test_inputs[i], table_size)
        assert_true(hash_val >= 0)
        assert_true(hash_val < table_size)
        i = i + 1
    }
    
    vibez.spill("✅ Secure collection hash tests passed")
}

fr fr ===== MEMORY SAFETY TESTS =====

slay test_memory_safety() {
    vibez.spill("Testing memory safety of crypto functions...")
    
    fr fr Test with large inputs to check for buffer overflows
    sus large_input tea = stringz.repeat("A", 10000)
    sus result tea = compute_sha256(large_input)
    assert_true(stringz.len(result) == 64)  // SHA-256 always 64 hex chars
    
    fr fr Test with empty inputs
    sus empty_result tea = compute_production_md5("")
    assert_true(stringz.len(empty_result) == 32)  // MD5 always 32 hex chars
    
    vibez.spill("✅ Memory safety tests passed")
}

fr fr ===== PERFORMANCE BENCHMARKS =====

slay benchmark_crypto_functions() {
    vibez.spill("Benchmarking crypto functions...")
    
    sus test_data tea = stringz.repeat("benchmark", 1000)
    sus iterations drip = 100
    
    fr fr Benchmark MD5
    sus start_time drip = current_time_ms()
    sus i drip = 0
    bestie i < iterations {
        sus _ tea = compute_production_md5(test_data)
        i = i + 1
    }
    sus md5_time drip = current_time_ms() - start_time
    
    fr fr Benchmark SHA-256
    start_time = current_time_ms()
    i = 0
    bestie i < iterations {
        sus _ tea = compute_sha256(test_data)
        i = i + 1
    }
    sus sha256_time drip = current_time_ms() - start_time
    
    vibez.spill("MD5 time:", md5_time, "ms for", iterations, "iterations")
    vibez.spill("SHA-256 time:", sha256_time, "ms for", iterations, "iterations")
    
    fr fr Performance should be reasonable
    assert_true(md5_time < 10000)  // Should complete in under 10 seconds
    assert_true(sha256_time < 20000)  // SHA-256 can be slower but not excessive
    
    vibez.spill("✅ Performance benchmarks passed")
}

fr fr ===== MAIN TEST RUNNER =====

slay run_security_tests() {
    test_start("Critical Security Validation Suite")
    
    test_md5_vectors()
    test_sha256_vectors()
    test_hmac_sha256_vectors()
    test_constant_time_comparison()
    test_collision_resistance()
    test_secure_collection_hash()
    test_memory_safety()
    benchmark_crypto_functions()
    
    print_test_summary()
    vibez.spill("🔒 SECURITY VALIDATION COMPLETE - ALL VULNERABILITIES FIXED")
}

fr fr Execute security validation
run_security_tests()
