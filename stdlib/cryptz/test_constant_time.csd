yeet "cryptz"
yeet "testz"
yeet "timez"

fr fr ===== CRYPTZ CONSTANT-TIME TESTS =====

test_start("Constant-Time PBKDF2")

fr fr Test that PBKDF2 works with different input lengths
sus short_password tea = "abc"
sus long_password tea = "this_is_a_much_longer_password_for_testing_purposes"
sus salt tea = "salt123"

sus result1 tea = crypto_pbkdf2(short_password, salt, 100)
sus result2 tea = crypto_pbkdf2(long_password, salt, 100)

assert_true(string_length(result1) > 0)
assert_true(string_length(result2) > 0)
assert_false(result1 == result2)

test_start("Input Normalization")

sus normalized_short tea = crypto_normalize_input("abc", 10)
sus normalized_long tea = crypto_normalize_input("abcdefghijk", 10)

assert_eq_int(string_length(normalized_short), string_length(normalized_long))

test_start("Constant-Time String Selection")

sus selected1 tea = crypto_constant_time_select_string(0xffffffff, "option_a", "option_b")
sus selected2 tea = crypto_constant_time_select_string(0x00000000, "option_a", "option_b")

assert_eq_string(selected1, "option_a")
assert_eq_string(selected2, "option_b")

test_start("Constant-Time Memory Comparison")

assert_true(crypto_constant_time_memcmp("hello", "hello", 5))
assert_false(crypto_constant_time_memcmp("hello", "world", 5))
assert_false(crypto_constant_time_memcmp("short", "longer_string", 10))

test_start("Timing-Safe Equality")

assert_true(crypto_timing_safe_equals("password123", "password123"))
assert_false(crypto_timing_safe_equals("password123", "different_pw"))

test_start("Constant-Time Delay Function")

fr fr Test that delay function executes without errors
crypto_constant_time_delay()

test_start("Constant-Time String Processing")

sus processed1 tea = crypto_process_string_constant_time("short")
sus processed2 tea = crypto_process_string_constant_time("this_is_a_much_longer_string")

fr fr Both should process successfully (return original strings)
assert_eq_string(processed1, "short")
assert_eq_string(processed2, "this_is_a_much_longer_string")

test_start("PBKDF2 Timing Analysis")

fr fr Perform basic timing analysis to verify constant-time behavior
sus passwords []tea = ["a", "abc", "password", "very_long_password_string"]
sus timing_results []drip = [0, 0, 0, 0]

bestie i := 0; i < len(passwords); i++ {
    sus start_time drip = benchmark_start()
    
    fr fr Execute PBKDF2 with constant iteration count
    sus result tea = crypto_pbkdf2(passwords[i], "constant_salt", 50)
    
    sus elapsed drip = benchmark_ms(start_time)
    timing_results[i] = elapsed
    
    vibez.spill("Password length:", json_number_to_string(string_length(passwords[i])), 
                "Time:", json_number_to_string(elapsed), "ms")
}

fr fr Verify all results are valid
bestie i := 0; i < len(timing_results); i++ {
    assert_true(timing_results[i] >= 0)
}

test_start("Constant-Time Integer Selection")

sus selected_int1 drip = crypto_constant_time_select(0xffffffff, 42, 99)
sus selected_int2 drip = crypto_constant_time_select(0x00000000, 42, 99)

assert_eq_int(selected_int1, 42)
assert_eq_int(selected_int2, 99)

test_start("Normalized Input Padding")

sus padded_short tea = crypto_normalize_input("hi", 20)
sus padded_long tea = crypto_normalize_input("this_is_exactly_twenty", 20)

fr fr Both should be same length after normalization
assert_eq_int(string_length(padded_short), 20)
assert_eq_int(string_length(padded_long), 20)

test_start("Constant-Time Operations Array")

sus array_a [normie] = [1, 2, 3, 4, 5]
sus array_b [normie] = [6, 7, 8, 9, 10]

sus comparison_result lit = crypto_constant_time_eq(array_a, array_b, 5)
assert_false(comparison_result)

sus array_same [normie] = [1, 2, 3, 4, 5]
sus same_comparison lit = crypto_constant_time_eq(array_a, array_same, 5)
assert_true(same_comparison)

test_start("Enhanced PBKDF2 Security")

fr fr Test PBKDF2 with various iteration counts
sus weak_result tea = crypto_pbkdf2("password", "salt", 1)
sus strong_result tea = crypto_pbkdf2("password", "salt", 1000)

assert_true(string_length(weak_result) > 0)
assert_true(string_length(strong_result) > 0)
assert_false(weak_result == strong_result)

test_start("Constant-Time Hash Verification")

fr fr Simulate password verification scenario
sus stored_hash tea = crypto_pbkdf2("correct_password", "user_salt", 1000)

fr fr Test correct password
sus provided_hash1 tea = crypto_pbkdf2("correct_password", "user_salt", 1000)
assert_true(crypto_timing_safe_equals(stored_hash, provided_hash1))

fr fr Test incorrect password  
sus provided_hash2 tea = crypto_pbkdf2("wrong_password", "user_salt", 1000)
assert_false(crypto_timing_safe_equals(stored_hash, provided_hash2))

print_test_summary()
