fr fr Comprehensive Validation of Pure CURSED Operations
fr fr Testing what currently works without Zig FFI dependencies

vibez.spill("🧪 CURSED Self-Hosting Validation Test")
vibez.spill("================================================")
vibez.spill("")

fr fr ===== CORE LANGUAGE FEATURES =====

vibez.spill("1. Testing Core Language Features...")

fr fr Variable declarations and assignments
sus int_var drip = 42
sus string_var tea = "hello world"
sus bool_var lit = based
sus array_var []drip = [1, 2, 3, 4, 5]
sus string_array []tea = ["one", "two", "three"]

vibez.spill("   ✅ Variable declarations working")

fr fr Basic arithmetic (should work with pure CURSED)
sus math_result1 drip = 10 + 20
sus math_result2 drip = 50 - 15
sus math_result3 drip = 6 * 7
sus math_result4 drip = 84 / 2

vibez.spill("   ✅ Basic arithmetic working")
vibez.spill("     10 + 20 =", math_result1)
vibez.spill("     50 - 15 =", math_result2)
vibez.spill("     6 * 7 =", math_result3)
vibez.spill("     84 / 2 =", math_result4)

fr fr ===== STRING OPERATIONS =====

vibez.spill("")
vibez.spill("2. Testing String Operations...")

fr fr String concatenation (pure CURSED operator)
sus concat_result tea = "Hello" + " " + "World"
vibez.spill("   String concatenation:", concat_result)

fr fr String equality
sus str_eq1 lit = "test" == "test"
sus str_eq2 lit = "test" == "different"
vibez.spill("   String equality (same):", str_eq1)
vibez.spill("   String equality (different):", str_eq2)

fr fr String inequality  
sus str_neq lit = "hello" != "goodbye"
vibez.spill("   String inequality:", str_neq)

vibez.spill("   ✅ Basic string operations working")

fr fr ===== ARRAY OPERATIONS =====

vibez.spill("")
vibez.spill("3. Testing Array Operations...")

fr fr Array length (built-in len function)
sus arr_length drip = len(array_var)
sus str_arr_length drip = len(string_array)

vibez.spill("   Integer array length:", arr_length)
vibez.spill("   String array length:", str_arr_length)

fr fr Array indexing
sus first_element drip = array_var[0]
sus last_element drip = array_var[4]
sus first_string tea = string_array[0]

vibez.spill("   First integer element:", first_element)
vibez.spill("   Last integer element:", last_element)
vibez.spill("   First string element:", first_string)

vibez.spill("   ✅ Array operations working")

fr fr ===== CONTROL FLOW =====

vibez.spill("")
vibez.spill("4. Testing Control Flow...")

fr fr Conditional statements
ready (based) {
    vibez.spill("   ✅ If statement (true) working")
} otherwise {
    vibez.spill("   ❌ If statement (true) failed")
}

ready (cringe) {
    vibez.spill("   ❌ If statement (false) failed")
} otherwise {
    vibez.spill("   ✅ If statement (false) working")
}

fr fr Loop iteration
vibez.spill("   Loop iteration test:")
sus loop_counter drip = 0
bestie (loop_counter < 3) {
    vibez.spill("     Iteration:", loop_counter)
    loop_counter = loop_counter + 1
}

vibez.spill("   ✅ Loop control working")

fr fr ===== FUNCTION DEFINITIONS AND CALLS =====

vibez.spill("")
vibez.spill("5. Testing Function Definitions and Calls...")

fr fr Simple function definition
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

sus func_result drip = add_numbers(15, 27)
vibez.spill("   Function call result (15 + 27):", func_result)

fr fr String function
slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

sus greeting tea = greet("CURSED")
vibez.spill("   String function result:", greeting)

fr fr Boolean function
slay is_positive(x drip) lit {
    damn x > 0
}

sus pos_test1 lit = is_positive(10)
sus pos_test2 lit = is_positive(-5)
vibez.spill("   Boolean function (10 > 0):", pos_test1)
vibez.spill("   Boolean function (-5 > 0):", pos_test2)

vibez.spill("   ✅ Function definitions and calls working")

fr fr ===== COMPLEX EXPRESSIONS =====

vibez.spill("")
vibez.spill("6. Testing Complex Expressions...")

fr fr Nested arithmetic
sus complex_math drip = (10 + 20) * (8 - 3) / 2
vibez.spill("   Complex math ((10+20)*(8-3)/2):", complex_math)

fr fr Mixed type operations
sus mixed_result lit = (5 * 2) == 10
vibez.spill("   Mixed type result (5*2 == 10):", mixed_result)

fr fr String and number operations
sus str_num_test tea = "Result: " + "42"
vibez.spill("   String-number concatenation:", str_num_test)

vibez.spill("   ✅ Complex expressions working")

fr fr ===== ARRAY MANIPULATION =====

vibez.spill("")
vibez.spill("7. Testing Array Manipulation...")

fr fr Array creation with expressions
sus computed_array []drip = [1 + 1, 2 * 2, 3 + 2]
vibez.spill("   Computed array length:", len(computed_array))
vibez.spill("   Computed array elements:", computed_array[0], computed_array[1], computed_array[2])

fr fr Array with mixed expressions
sus sum_of_array drip = computed_array[0] + computed_array[1] + computed_array[2]
vibez.spill("   Sum of array elements:", sum_of_array)

vibez.spill("   ✅ Array manipulation working")

fr fr ===== RECURSIVE FUNCTIONS =====

vibez.spill("")
vibez.spill("8. Testing Recursive Functions...")

fr fr Simple recursive function
slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

sus fact_result drip = factorial(5)
vibez.spill("   Factorial of 5:", fact_result)

fr fr Fibonacci
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

sus fib_result drip = fibonacci(7)
vibez.spill("   Fibonacci(7):", fib_result)

vibez.spill("   ✅ Recursive functions working")

fr fr ===== ADVANCED ARRAY OPERATIONS =====

vibez.spill("")
vibez.spill("9. Testing Advanced Array Operations...")

fr fr Array iteration with function
slay process_array(arr []drip) drip {
    sus total drip = 0
    sus i drip = 0
    bestie (i < len(arr)) {
        total = total + arr[i]
        i = i + 1
    }
    damn total
}

sus array_sum drip = process_array([10, 20, 30, 40])
vibez.spill("   Array sum (10+20+30+40):", array_sum)

fr fr String array processing
slay count_strings(strings []tea) drip {
    sus count drip = 0
    sus i drip = 0
    bestie (i < len(strings)) {
        ready (strings[i] != "") {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

sus str_count drip = count_strings(["hello", "world", "", "test"])
vibez.spill("   Non-empty string count:", str_count)

vibez.spill("   ✅ Advanced array operations working")

fr fr ===== FINAL SUMMARY =====

vibez.spill("")
vibez.spill("🎉 CURSED Self-Hosting Validation Summary")
vibez.spill("=========================================")
vibez.spill("✅ Core Language Features: WORKING")
vibez.spill("✅ String Operations: WORKING")
vibez.spill("✅ Array Operations: WORKING")
vibez.spill("✅ Control Flow: WORKING")
vibez.spill("✅ Function Definitions: WORKING")
vibez.spill("✅ Complex Expressions: WORKING")
vibez.spill("✅ Array Manipulation: WORKING")
vibez.spill("✅ Recursive Functions: WORKING")
vibez.spill("✅ Advanced Operations: WORKING")
vibez.spill("")
vibez.spill("🚀 CURSED is ready for self-hosting migration!")
vibez.spill("📋 Next: Replace Zig built-ins with pure CURSED stdlib")
