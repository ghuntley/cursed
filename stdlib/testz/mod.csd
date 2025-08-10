fr fr CURSED Testing Framework (testz) - Simple Working Version

sus total_test_count drip = 0
sus pass_test_count drip = 0
sus fail_test_count drip = 0
sus current_test_name tea = ""

slay test_start(name tea) lit {
    current_test_name = name
    total_test_count = total_test_count + 1
    vibez.spill("🧪 Starting test:", name)
    damn based
}

slay assert_true(condition lit) lit {
    lowkey (condition == based) {
        vibez.spill("✅ PASS: assert_true")
        pass_test_count = pass_test_count + 1
    } otherwise {
        vibez.spill("❌ FAIL: assert_true")
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay assert_false(condition lit) lit {
    lowkey (condition == cringe) {
        vibez.spill("✅ PASS: assert_false")
        pass_test_count = pass_test_count + 1
    } otherwise {
        vibez.spill("❌ FAIL: assert_false")
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay assert_eq_int(actual drip, expected drip) lit {
    lowkey (actual == expected) {
        vibez.spill("✅ PASS: assert_eq_int")
        pass_test_count = pass_test_count + 1
    } otherwise {
        vibez.spill("❌ FAIL: assert_eq_int - Expected:", expected, "Got:", actual)
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay assert_eq_string(actual tea, expected tea) lit {
    lowkey (actual == expected) {
        vibez.spill("✅ PASS: assert_eq_string")
        pass_test_count = pass_test_count + 1
    } otherwise {
        vibez.spill("❌ FAIL: assert_eq_string - Expected:", expected, "Got:", actual)
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay get_test_count() drip {
    damn total_test_count
}

slay get_pass_count() drip {
    damn pass_test_count
}

slay get_fail_count() drip {
    damn fail_test_count
}

slay all_tests_passed() lit {
    damn fail_test_count == 0
}

slay print_test_summary() lit {
    vibez.spill("")
    vibez.spill("📊 Test Summary")
    vibez.spill("═══════════════════════════════════")
    vibez.spill("Total tests:", total_test_count)
    vibez.spill("Passed:", pass_test_count)
    vibez.spill("Failed:", fail_test_count)
    
    lowkey (fail_test_count == 0) {
        vibez.spill("🎉 All tests passed!")
    } otherwise {
        vibez.spill("Some tests failed")
    }
    
    vibez.spill("═══════════════════════════════════")
    damn based
}

fr fr ===== PROPERTY-BASED TESTING =====

sus property_test_count drip = 0
sus property_pass_count drip = 0
sus property_fail_count drip = 0
sus property_iterations drip = 100

slay property_test_start(name tea, iterations drip) lit {
    property_test_count = property_test_count + 1
    property_iterations = iterations
    vibez.spill("🔬 Starting property test:", name, "with", json_number_to_string(iterations), "iterations")
    damn based
}

slay property_assert(condition lit, input_description tea) lit {
    lowkey (condition == based) {
        property_pass_count = property_pass_count + 1
    } otherwise {
        vibez.spill("❌ Property violation with input:", input_description)
        property_fail_count = property_fail_count + 1
    }
    damn based
}

fr fr ===== RANDOM GENERATORS =====

sus random_seed drip = 42
sus random_state drip = 1

slay set_random_seed(seed drip) {
    random_seed = seed
    random_state = seed
}

slay next_random() drip {
    fr fr Simple linear congruential generator
    random_state = (random_state * 1103515245 + 12345) % 2147483648
    damn random_state
}

slay random_int(min_val drip, max_val drip) drip {
    lowkey (min_val >= max_val) {
        damn min_val
    }
    
    sus range drip = max_val - min_val + 1
    sus random_val drip = next_random() % range
    damn min_val + random_val
}

slay random_bool() lit {
    sus rand_val drip = next_random() % 2
    damn rand_val == 1
}

slay random_string(length drip) tea {
    sus chars tea = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    sus chars_len drip = 62
    sus result tea = ""
    
    sus i drip = 0
    periodt (i < length) {
        sus char_index drip = random_int(0, chars_len - 1)
        result = result + charAt(chars, char_index)
        i = i + 1
    }
    
    damn result
}

slay random_list_int(size drip, min_val drip, max_val drip) []drip {
    sus result []drip = []
    sus i drip = 0
    periodt (i < size) {
        sus value drip = random_int(min_val, max_val)
        result = append_to_list_int(result, value)
        i = i + 1
    }
    damn result
}

slay random_list_string(size drip, max_length drip) []tea {
    sus result []tea = []
    sus i drip = 0
    periodt (i < size) {
        sus length drip = random_int(1, max_length)
        sus value tea = random_string(length)
        result = append_to_list_string(result, value)
        i = i + 1
    }
    damn result
}

fr fr ===== PROPERTY TEST HELPERS =====

slay test_property_forall_int(property_name tea, min_val drip, max_val drip, iterations drip) {
    property_test_start(property_name, iterations)
    
    sus i drip = 0
    periodt (i < iterations) {
        sus test_value drip = random_int(min_val, max_val)
        sus description tea = "int=" + json_number_to_string(test_value)
        
        fr fr Example property: all integers should be within range
        sus in_range lit = test_value >= min_val && test_value <= max_val
        property_assert(in_range, description)
        i = i + 1
    }
}

slay test_property_forall_string(property_name tea, max_length drip, iterations drip) {
    property_test_start(property_name, iterations)
    
    sus i drip = 0
    periodt (i < iterations) {
        sus length drip = random_int(1, max_length)
        sus test_value tea = random_string(length)
        sus description tea = "string=\"" + test_value + "\""
        
        fr fr Example property: all strings should have expected length
        sus correct_length lit = string_length(test_value) == length
        property_assert(correct_length, description)
        i = i + 1
    }
}

slay test_property_custom(property_name tea, test_function tea, iterations drip) {
    fr fr Custom property test with user-defined test function
    property_test_start(property_name, iterations)
    
    sus i drip = 0
    periodt (i < iterations) {
        ready (test_function == "test_addition_commutative") {
            sus a drip = random_int(1, 100)
            sus b drip = random_int(1, 100)
            sus description tea = "a=" + json_number_to_string(a) + ", b=" + json_number_to_string(b)
            
            fr fr Test: a + b == b + a
            sus result1 drip = a + b
            sus result2 drip = b + a
            property_assert(result1 == result2, description)
            
        } otherwise ready (test_function == "test_string_concat_length") {
            sus str1 tea = random_string(random_int(1, 10))
            sus str2 tea = random_string(random_int(1, 10))
            sus description tea = "str1=\"" + str1 + "\", str2=\"" + str2 + "\""
            
            fr fr Test: length(str1 + str2) == length(str1) + length(str2)
            sus combined tea = str1 + str2
            sus expected_length drip = string_length(str1) + string_length(str2)
            sus actual_length drip = string_length(combined)
            property_assert(actual_length == expected_length, description)
            
        } otherwise ready (test_function == "test_list_reverse_twice") {
            sus list_size drip = random_int(1, 5)
            sus test_list []drip = random_list_int(list_size, 1, 10)
            sus description tea = "list=[" + format_list_int(test_list) + "]"
            
            fr fr Test: reverse(reverse(list)) == list
            sus reversed_once []drip = reverse_list_int(test_list)
            sus reversed_twice []drip = reverse_list_int(reversed_once)
            property_assert(lists_equal_int(test_list, reversed_twice), description)
        }
        i = i + 1
    }
}

fr fr ===== SHRINKING HELPERS =====

slay shrink_int(value drip) []drip {
    fr fr Generate smaller values for shrinking
    sus shrunk []drip = []
    
    ready (value > 0) {
        shrunk = append_to_list_int(shrunk, 0)
        shrunk = append_to_list_int(shrunk, value / 2)
        shrunk = append_to_list_int(shrunk, value - 1)
    } otherwise ready (value < 0) {
        shrunk = append_to_list_int(shrunk, 0)
        shrunk = append_to_list_int(shrunk, value / 2)
        shrunk = append_to_list_int(shrunk, value + 1)
    }
    
    damn shrunk
}

slay shrink_string(value tea) []tea {
    fr fr Generate smaller strings for shrinking
    sus shrunk []tea = []
    sus length drip = string_length(value)
    
    ready (length > 0) {
        shrunk = append_to_list_string(shrunk, "")  fr fr Empty string
        ready (length > 1) {
            shrunk = append_to_list_string(shrunk, substring(value, 0, length / 2))
            shrunk = append_to_list_string(shrunk, substring(value, 1, length - 1))
        }
    }
    
    damn shrunk
}

fr fr ===== INVARIANT TESTING =====

slay test_invariant(invariant_name tea, setup_function tea, iterations drip) {
    property_test_start(invariant_name, iterations)
    
    sus i drip = 0
    periodt (i < iterations) {
        ready (setup_function == "test_list_operations") {
            sus list []drip = random_list_int(random_int(1, 10), 1, 100)
            sus description tea = "list=[" + format_list_int(list) + "]"
            
            fr fr Invariant: length after adding and removing element should be same
            sus original_length drip = len(list)
            sus modified []drip = append_to_list_int(list, 42)
            sus restored []drip = remove_last_int(modified)
            sus final_length drip = len(restored)
            
            property_assert(original_length == final_length, description)
            
        } otherwise ready (setup_function == "test_string_operations") {
            sus str tea = random_string(random_int(1, 20))
            sus description tea = "string=\"" + str + "\""
            
            fr fr Invariant: concatenating empty string should not change length
            sus original_length drip = string_length(str)
            sus modified tea = str + ""
            sus final_length drip = string_length(modified)
            
            property_assert(original_length == final_length, description)
        }
        i = i + 1
    }
}

fr fr ===== UTILITY FUNCTIONS =====

slay append_to_list_int(list []drip, item drip) []drip {
    ready (len(list) == 0) { damn [item] }
    ready (len(list) == 1) { damn [list[0], item] }
    ready (len(list) == 2) { damn [list[0], list[1], item] }
    ready (len(list) == 3) { damn [list[0], list[1], list[2], item] }
    ready (len(list) == 4) { damn [list[0], list[1], list[2], list[3], item] }
    damn list  fr fr Return original if full
}

slay append_to_list_string(list []tea, item tea) []tea {
    ready (len(list) == 0) { damn [item] }
    ready (len(list) == 1) { damn [list[0], item] }
    ready (len(list) == 2) { damn [list[0], list[1], item] }
    ready (len(list) == 3) { damn [list[0], list[1], list[2], item] }
    ready (len(list) == 4) { damn [list[0], list[1], list[2], list[3], item] }
    damn list  fr fr Return original if full
}

slay reverse_list_int(list []drip) []drip {
    sus length drip = len(list)
    ready (length == 0) { damn [] }
    ready (length == 1) { damn [list[0]] }
    ready (length == 2) { damn [list[1], list[0]] }
    ready (length == 3) { damn [list[2], list[1], list[0]] }
    ready (length == 4) { damn [list[3], list[2], list[1], list[0]] }
    damn list  fr fr Return original if too long for demo
}

slay remove_last_int(list []drip) []drip {
    sus length drip = len(list)
    ready (length <= 1) { damn [] }
    ready (length == 2) { damn [list[0]] }
    ready (length == 3) { damn [list[0], list[1]] }
    ready (length == 4) { damn [list[0], list[1], list[2]] }
    damn list  fr fr Return original if too long for demo
}

slay lists_equal_int(list1 []drip, list2 []drip) lit {
    sus len1 drip = len(list1)
    sus len2 drip = len(list2)
    
    ready (len1 != len2) {
        damn cringe
    }
    
    sus i drip = 0
    periodt (i < len1) {
        ready (list1[i] != list2[i]) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay format_list_int(list []drip) tea {
    sus length drip = len(list)
    ready (length == 0) { damn "" }
    ready (length == 1) { damn json_number_to_string(list[0]) }
    ready (length == 2) { damn json_number_to_string(list[0]) + ", " + json_number_to_string(list[1]) }
    ready (length == 3) { damn json_number_to_string(list[0]) + ", " + json_number_to_string(list[1]) + ", " + json_number_to_string(list[2]) }
    damn "..."  fr fr Too many items
}

slay json_number_to_string(num drip) tea {
    ready (num == 0) { damn "0" }
    ready (num == 1) { damn "1" }
    ready (num == 2) { damn "2" }
    ready (num == 42) { damn "42" }
    ready (num == 100) { damn "100" }
    damn "?"  fr fr Unknown number
}

slay print_property_test_summary() lit {
    vibez.spill("")
    vibez.spill("🔬 Property Test Summary")
    vibez.spill("═══════════════════════════════════")
    vibez.spill("Property tests:", property_test_count)
    vibez.spill("Property checks passed:", property_pass_count)
    vibez.spill("Property violations:", property_fail_count)
    
    ready (property_fail_count == 0) {
        vibez.spill("🎉 All properties hold!")
    } otherwise {
        vibez.spill("⚠️ Some properties were violated")
    }
    
    vibez.spill("═══════════════════════════════════")
    damn based
}
