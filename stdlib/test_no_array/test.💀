yeet "testz"
yeet "test_no_array"

test_start("TEST_NO_ARRAY Non-Array Operations Validation")

// Test basic non-array data types
sus single_int drip = 42
assert_eq_int(single_int, 42)

sus single_string tea = "hello"
assert_eq_string(single_string, "hello")

sus single_bool lit = based
assert_true(single_bool)

// Test simple arithmetic without arrays
sus num1 drip = 10
sus num2 drip = 20
sus sum drip = num1 + num2
assert_eq_int(sum, 30)

sus product drip = num1 * num2
assert_eq_int(product, 200)

// Test string operations without arrays
sus str1 tea = "Hello"
sus str2 tea = "World"
sus concatenated tea = str1 + " " + str2
assert_eq_string(concatenated, "Hello World")

sus str_len drip = len(str1)
assert_eq_int(str_len, 5)

// Test boolean logic operations
sus bool1 lit = based
sus bool2 lit = nocap
sus and_result lit = bool1 && bool2
assert_false(and_result)

sus or_result lit = bool1 || bool2
assert_true(or_result)

// Test variable assignment and modification
sus mutable_var drip = 100
mutable_var = mutable_var + 50
assert_eq_int(mutable_var, 150)

// Test conditional operations without arrays
sus condition_test drip = 0
ready (single_int > 40) {
    condition_test = 1
}
assert_eq_int(condition_test, 1)

// Test function calls without arrays
slay simple_function(x drip) drip {
    damn x * 2
}

sus function_result drip = simple_function(25)
assert_eq_int(function_result, 50)

// Test loop operations without arrays
sus loop_sum drip = 0
bestie (sus i drip = 1; i <= 5; i++) {
    loop_sum = loop_sum + i
}
assert_eq_int(loop_sum, 15) // 1+2+3+4+5

// Test nested conditional logic
sus nested_test drip = 0
ready (single_int > 30) {
    ready (single_string == "hello") {
        nested_test = 999
    }
}
assert_eq_int(nested_test, 999)

// Test simple data structure operations
sus struct_value squad {
    field1 drip
    field2 tea
}

sus test_struct struct_value = {field1: 123, field2: "test"}
assert_eq_int(test_struct.field1, 123)
assert_eq_string(test_struct.field2, "test")

// Test basic mathematical operations
sus math_result drip = (10 + 5) * 2 - 3
assert_eq_int(math_result, 27)

sus division_result drip = 100 / 4
assert_eq_int(division_result, 25)

// Test string manipulation without arrays
sus upper_case tea = to_upper(str1)
assert_eq_string(upper_case, "HELLO")

sus trimmed tea = trim("  spaces  ")
assert_eq_string(trimmed, "spaces")

// Test simple error handling
sus error_test drip = 0
ready (single_int != 0) {
    error_test = single_int / 2
} otherwise {
    error_test = -1
}
assert_eq_int(error_test, 21)

// Test performance of non-array operations
sus perf_start drip = get_nanoseconds()
sus perf_result drip = 0
bestie (sus i drip = 0; i < 10000; i++) {
    perf_result = perf_result + (i % 100)
}
sus perf_end drip = get_nanoseconds()
sus perf_duration drip = perf_end - perf_start
assert_true(perf_result > 0)
assert_true(perf_duration < 50000000) // Less than 50ms

print_test_summary()
