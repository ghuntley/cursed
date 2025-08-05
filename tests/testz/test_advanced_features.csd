fr fr Advanced Features Test
fr fr Tests advanced CURSED language features

yeet "testz"

fr fr ================================
fr fr Function Testing
fr fr ================================

slay add_numbers(a normie, b normie) normie {
    damn a + b
}

slay multiply_numbers(a normie, b normie) normie {
    damn a * b
}

slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    } highkey {
        damn n * factorial(n - 1)
    }
}

slay test_function_calls() {
    testz.test_start("test_function_calls")
    
    sus result1 normie = add_numbers(5, 3)
    sus result2 normie = multiply_numbers(4, 6)
    sus result3 normie = factorial(5)
    
    testz.assert_eq(result1, 8)
    testz.assert_eq(result2, 24)
    testz.assert_eq(result3, 120)
}

slay test_recursive_functions() {
    testz.test_start("test_recursive_functions")
    
    testz.assert_eq(factorial(0), 1)
    testz.assert_eq(factorial(1), 1)
    testz.assert_eq(factorial(2), 2)
    testz.assert_eq(factorial(3), 6)
    testz.assert_eq(factorial(4), 24)
}

fr fr ================================
fr fr Fibonacci Function for Testing
fr fr ================================

slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    } highkey {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

slay test_fibonacci_sequence() {
    testz.test_start("test_fibonacci_sequence")
    
    testz.assert_eq(fibonacci(0), 0)
    testz.assert_eq(fibonacci(1), 1)
    testz.assert_eq(fibonacci(2), 1)
    testz.assert_eq(fibonacci(3), 2)
    testz.assert_eq(fibonacci(4), 3)
    testz.assert_eq(fibonacci(5), 5)
    testz.assert_eq(fibonacci(6), 8)
}

fr fr ================================
fr fr Control Flow Tests
fr fr ================================

slay test_if_statements() {
    testz.test_start("test_if_statements")
    
    sus x normie = 10
    sus result tea = ""
    
    lowkey x > 5 {
        result = "large"
    } highkey {
        result = "small"
    }
    
    testz.assert_eq_string(result, "large")
    
    sus y normie = 3
    sus category tea = ""
    
    lowkey y < 5 {
        category = "small"
    } highkey lowkey y < 10 {
        category = "medium"
    } highkey {
        category = "large"
    }
    
    testz.assert_eq_string(category, "small")
}

slay test_while_loops() {
    testz.test_start("test_while_loops")
    
    sus count normie = 0
    sus sum normie = 0
    
    periodt count < 5 {
        sum = sum + count
        count = count + 1
    }
    
    testz.assert_eq(count, 5)
    testz.assert_eq(sum, 10)  fr fr 0 + 1 + 2 + 3 + 4 = 10
}

slay test_break_continue() {
    testz.test_start("test_break_continue")
    
    sus sum normie = 0
    sus i normie = 0
    
    periodt i < 10 {
        lowkey i == 5 {
            i = i + 1
            simp
        }
        lowkey i == 8 {
            ghosted
        }
        sum = sum + i
        i = i + 1
    }
    
    fr fr Should sum 0+1+2+3+4+6+7 = 23 (skipping 5, breaking at 8)
    testz.assert_eq(sum, 23)
}

fr fr ================================
fr fr Type System Tests
fr fr ================================

slay test_type_assertions() {
    testz.test_start("test_type_assertions")
    
    sus number normie = 42
    sus float_val meal = 3.14
    sus char_val sip = 'A'
    
    fr fr Test type assertions
    sus small_int smol = number.(smol)
    sus large_int thicc = number.(thicc)
    sus converted_float meal = number.(meal)
    sus converted_int normie = char_val.(normie)
    
    testz.assert_eq(small_int, 42)
    testz.assert_eq(large_int, 42)
    testz.assert_eq_float(converted_float, 42.0)
    testz.assert_eq(converted_int, 65)  fr fr ASCII code for 'A'
}

slay test_mixed_arithmetic() {
    testz.test_start("test_mixed_arithmetic")
    
    sus int_val normie = 5
    sus float_val meal = 3.14
    
    fr fr Mixed arithmetic should work
    sus result1 meal = int_val + float_val
    sus result2 meal = int_val * float_val
    
    testz.assert_eq_float(result1, 8.14)
    testz.assert_eq_float(result2, 15.7)
}

fr fr ================================
fr fr String Operations Tests
fr fr ================================

slay test_string_operations() {
    testz.test_start("test_string_operations")
    
    sus hello tea = "Hello"
    sus world tea = "World"
    sus space tea = " "
    
    fr fr String concatenation
    sus greeting tea = hello + space + world
    testz.assert_eq_string(greeting, "Hello World")
    
    fr fr String length
    testz.assert_eq(hello.length, 5)
    testz.assert_eq(world.length, 5)
    testz.assert_eq(greeting.length, 11)
}

slay test_string_comparison() {
    testz.test_start("test_string_comparison")
    
    sus str1 tea = "hello"
    sus str2 tea = "hello"
    sus str3 tea = "world"
    
    testz.assert_eq_string(str1, str2)
    testz.assert_ne_string(str1, str3)
    testz.assert_true(str1 == str2)
    testz.assert_false(str1 == str3)
}

fr fr ================================
fr fr Character Operations Tests
fr fr ================================

slay test_character_operations() {
    testz.test_start("test_character_operations")
    
    sus char_a sip = 'A'
    sus char_b sip = 'B'
    sus char_1 sip = '1'
    
    fr fr Character to integer conversion
    sus ascii_a normie = char_a.(normie)
    sus ascii_b normie = char_b.(normie)
    sus ascii_1 normie = char_1.(normie)
    
    testz.assert_eq(ascii_a, 65)
    testz.assert_eq(ascii_b, 66)
    testz.assert_eq(ascii_1, 49)
    
    fr fr Character comparison
    testz.assert_true(char_a < char_b)
    testz.assert_false(char_b < char_a)
}

fr fr ================================
fr fr Tuple Tests
fr fr ================================

slay test_tuple_operations() {
    testz.test_start("test_tuple_operations")
    
    sus tuple1 (normie, tea, lit) = (42, "hello", based)
    sus tuple2 (normie, normie, normie) = (1, 2, 3)
    
    fr fr Tuple element access
    testz.assert_eq(tuple1.0, 42)
    testz.assert_eq_string(tuple1.1, "hello")
    testz.assert_eq_bool(tuple1.2, based)
    
    testz.assert_eq(tuple2.0, 1)
    testz.assert_eq(tuple2.1, 2)
    testz.assert_eq(tuple2.2, 3)
}

slay test_tuple_destructuring() {
    testz.test_start("test_tuple_destructuring")
    
    sus data (normie, tea, lit) = (100, "test", cap)
    
    fr fr Destructuring assignment
    sus (num, str, flag) = data
    
    testz.assert_eq(num, 100)
    testz.assert_eq_string(str, "test")
    testz.assert_eq_bool(flag, cap)
}

fr fr ================================
fr fr Short Variable Declaration Tests
fr fr ================================

slay test_short_declarations() {
    testz.test_start("test_short_declarations")
    
    x := 42
    y := 3.14
    z := "hello"
    flag := based
    
    testz.assert_eq(x, 42)
    testz.assert_eq_float(y, 3.14)
    testz.assert_eq_string(z, "hello")
    testz.assert_eq_bool(flag, based)
}

slay test_multiple_short_declarations() {
    testz.test_start("test_multiple_short_declarations")
    
    (a, b, c) := (1, 2, 3)
    (x, y) := (10, 20)
    
    testz.assert_eq(a, 1)
    testz.assert_eq(b, 2)
    testz.assert_eq(c, 3)
    testz.assert_eq(x, 10)
    testz.assert_eq(y, 20)
}

fr fr ================================
fr fr Test Runner
fr fr ================================

slay main() {
    vibez.spill("Running Advanced Features Test Suite")
    vibez.spill("====================================")
    
    fr fr Run all test functions
    test_function_calls()
    test_recursive_functions()
    test_fibonacci_sequence()
    test_if_statements()
    test_while_loops()
    test_break_continue()
    test_type_assertions()
    test_mixed_arithmetic()
    test_string_operations()
    test_string_comparison()
    test_character_operations()
    test_tuple_operations()
    test_tuple_destructuring()
    test_short_declarations()
    test_multiple_short_declarations()
    
    fr fr Print summary
    testz.print_test_summary()
    
    fr fr Return appropriate exit code
    lowkey testz.test_failed > 0 {
        damn 1
    } highkey {
        damn 0
    }
}
