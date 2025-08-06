// Test stdlib modules for edge cases and bugs
yeet "testz"
yeet "vibez"
yeet "cryptz"
yeet "stringz"
yeet "arrayz"
yeet "hashz"

slay test_cryptz_edge_cases() {
    test_start("Cryptz Edge Cases")
    
    // Test empty string encryption
    sus empty_result = cryptz.sha256("")
    assert_eq_string(empty_result, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    
    // Test null/invalid inputs
    ready {
        sus invalid = cryptz.sha256(nil)  // Should handle gracefully
        vibez.spill("Handled nil input")
    } catch(e) {
        vibez.spill("Cryptz nil handling error")
    }
    
    print_test_summary()
}

slay test_stringz_edge_cases() {
    test_start("Stringz Edge Cases")
    
    // Test empty string operations
    sus empty = ""
    sus len_result = stringz.length(empty)
    assert_eq_int(len_result, 0)
    
    // Test string concatenation edge cases
    sus concat_result = stringz.concat("", "test")
    assert_eq_string(concat_result, "test")
    
    // Test unicode handling
    sus unicode = "🚀"
    sus unicode_len = stringz.length(unicode)
    // Should handle unicode properly
    
    print_test_summary()
}

slay test_arrayz_edge_cases() {
    test_start("Arrayz Edge Cases")
    
    // Test empty array operations
    sus empty_arr = []drip{}
    sus arr_len = arrayz.length(empty_arr)
    assert_eq_int(arr_len, 0)
    
    // Test out-of-bounds access
    ready {
        sus value = arrayz.get(empty_arr, 0)  // Should fail
        vibez.spill("This should not print")
    } catch(e) {
        vibez.spill("Caught array bounds error")
    }
    
    // Test large array operations
    sus large_arr = []drip{}
    sus i drip = 0
    bestie (i < 1000) {
        arrayz.append(large_arr, i)
        i = i + 1
    }
    
    print_test_summary()
}

slay test_hashz_edge_cases() {
    test_start("Hashz Edge Cases")
    
    // Test empty hash map
    sus empty_map = hashz.new()
    sus size = hashz.size(empty_map)
    assert_eq_int(size, 0)
    
    // Test null key/value handling
    ready {
        hashz.set(empty_map, nil, "value")  // Should handle gracefully
    } catch(e) {
        vibez.spill("Caught nil key error")
    }
    
    // Test collision handling
    sus i drip = 0
    bestie (i < 100) {
        sus key = "key_" + tea(i)
        sus value = "value_" + tea(i)
        hashz.set(empty_map, key, value)
        i = i + 1
    }
    
    print_test_summary()
}

slay main() {
    vibez.spill("Testing stdlib edge cases...")
    
    test_cryptz_edge_cases()
    test_stringz_edge_cases()
    test_arrayz_edge_cases()
    test_hashz_edge_cases()
    
    vibez.spill("Stdlib edge case tests completed")
}
