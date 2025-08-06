fr fr Test hashz module with new string runtime support

yeet "testz"
yeet "stringz"
yeet "hashz"

vibez.spill("=== Testing hashz module with string runtime ===")

test_start("hashz string operations")

fr fr Test string hashing with character access
sus test_string tea = "hello world"
sus hash_value normie = hashz.hash_string(test_string)

vibez.spill("String: " + test_string)
vibez.spill("Hash value: " + string(hash_value))

fr fr Test multiple strings with different hashes
sus str1 tea = "test1"
sus str2 tea = "test2"
sus str3 tea = "test1"

sus hash1 normie = hashz.hash_string(str1)
sus hash2 normie = hashz.hash_string(str2)
sus hash3 normie = hashz.hash_string(str3)

vibez.spill("String1: " + str1 + " -> Hash: " + string(hash1))
vibez.spill("String2: " + str2 + " -> Hash: " + string(hash2))
vibez.spill("String3: " + str3 + " -> Hash: " + string(hash3))

fr fr Test that identical strings have identical hashes
assert_eq_int(hash1, hash3)

fr fr Test that different strings have different hashes
assert_false(hash1 == hash2)

test_start("hashz with string processing")

fr fr Test hashing processed strings
sus original tea = "Hello World"
sus lower_case tea = stringz.to_lower(original)
sus upper_case tea = stringz.to_upper(original)

sus original_hash normie = hashz.hash_string(original)
sus lower_hash normie = hashz.hash_string(lower_case)
sus upper_hash normie = hashz.hash_string(upper_case)

vibez.spill("Original: " + original + " -> " + string(original_hash))
vibez.spill("Lower: " + lower_case + " -> " + string(lower_hash))
vibez.spill("Upper: " + upper_case + " -> " + string(upper_hash))

fr fr Different cases should have different hashes
assert_false(original_hash == lower_hash)
assert_false(original_hash == upper_hash)
assert_false(lower_hash == upper_hash)

test_start("hashz with substrings")

fr fr Test hashing substrings
sus full_string tea = "programming language"
sus sub1 tea = stringz.substring(full_string, 0, 11)
sus sub2 tea = stringz.substring(full_string, 12, 8)

sus full_hash normie = hashz.hash_string(full_string)
sus sub1_hash normie = hashz.hash_string(sub1)
sus sub2_hash normie = hashz.hash_string(sub2)

vibez.spill("Full: " + full_string + " -> " + string(full_hash))
vibez.spill("Sub1: " + sub1 + " -> " + string(sub1_hash))
vibez.spill("Sub2: " + sub2 + " -> " + string(sub2_hash))

fr fr All should be different
assert_false(full_hash == sub1_hash)
assert_false(full_hash == sub2_hash)
assert_false(sub1_hash == sub2_hash)

print_test_summary()

vibez.spill("✅ hashz module now works with string runtime!")
vibez.spill("✅ String operations enable proper hash calculation!")
vibez.spill("✅ This unblocks hash tables and key-value operations!")
