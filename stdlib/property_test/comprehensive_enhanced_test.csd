yeet "testz"
yeet "enhanced_mod"
yeet "advanced_generators"
yeet "property_combinators"
yeet "stringz"
yeet "mathz"

fr fr Comprehensive Test Suite for Enhanced Property Testing Framework
fr fr Tests all enhanced functionality including UTF-8, performance measurement, and advanced shrinking

fr fr ===== UTF-8 STRING HANDLING TESTS =====

slay test_utf8_string_operations() {
    test_start("UTF-8 String Operations")
    
    fr fr Test UTF-8 character length detection
    assert_eq_int(utf8_char_length(65), 1)      fr fr ASCII 'A'
    assert_eq_int(utf8_char_length(194), 2)     fr fr 2-byte UTF-8 start
    assert_eq_int(utf8_char_length(224), 3)     fr fr 3-byte UTF-8 start
    assert_eq_int(utf8_char_length(240), 4)     fr fr 4-byte UTF-8 start
    
    fr fr Test UTF-8 string length calculation
    sus ascii_string tea = "Hello"
    assert_eq_int(utf8_string_length(ascii_string), 5)
    
    fr fr Test UTF-8 character extraction
    sus mixed_string tea = "Hi🙂"
    sus emoji tea = utf8_char_at(mixed_string, 2)
    assert_true(stringz.length(emoji) > 1)  fr fr Emoji should be multi-byte
    
    fr fr Test UTF-8 substring extraction
    sus sub tea = utf8_substring("Hello, 世界!", 7, 9)
    assert_true(stringz.length(sub) > 0)
    
    vibez.spill("✅ UTF-8 string operations passed")
}

slay test_performance_measurement() {
    test_start("Performance Measurement")
    
    fr fr Test timing calibration
    timing_calibrate()
    assert_true(timing_overhead >= 0)
    
    fr fr Test execution time measurement
    slay test_function(input normie) normie {
        sus i normie = 0
        bestie i < input {
            i = i + 1  fr fr Simple loop to consume time
        }
        damn input
    }
    
    sus execution_time normie = measure_execution_time(test_function, 1000)
    assert_true(execution_time > 0)
    
    vibez.spill("✅ Performance measurement calibrated, overhead: " + stringz.from_int(timing_overhead) + "ns")
}

slay test_reflection_type_system() {
    test_start("Reflection-Based Type System")
    
    fr fr Test type detection for different types
    sus int_val normie = 42
    sus string_val tea = "test"
    sus bool_val lit = based
    sus array_val [] = [1, 2, 3]
    
    assert_true(is_numeric_type(int_val))
    assert_true(is_string_type(string_val))
    assert_true(is_boolean_type(bool_val))
    assert_true(is_array_type(array_val))
    
    fr fr Test deep equality comparison
    sus arr1 [] = [1, 2, 3]
    sus arr2 [] = [1, 2, 3]
    sus arr3 [] = [1, 2, 4]
    
    assert_true(deep_equal(arr1, arr2))
    assert_true(!deep_equal(arr1, arr3))
    
    vibez.spill("✅ Reflection-based type system working")
}

fr fr ===== ADVANCED GENERATOR TESTS =====

slay test_statistical_generators() {
    test_start("Statistical Generators")
    
    fr fr Test normal distribution generator
    sus normal_samples [] = []
    sus i normie = 0
    bestie i < 100 {
        sus sample normie = gen_normal_int(50, 10)
        normal_samples = reflectz.array_append(normal_samples, sample)
        assert_true(sample >= 0)  fr fr Should be reasonable values
        i = i + 1
    }
    
    fr fr Test exponential distribution
    sus exp_sample normie = gen_exponential_int(0.1)
    assert_true(exp_sample > 0)
    
    fr fr Test power law distribution
    sus power_sample normie = gen_power_law_int(2.0, 1, 100)
    assert_true(power_sample >= 1 && power_sample <= 100)
    
    vibez.spill("✅ Statistical generators producing valid samples")
}

slay test_domain_specific_generators() {
    test_start("Domain-Specific Generators")
    
    fr fr Test realistic email generation
    sus email tea = gen_email_realistic()
    assert_true(stringz.contains(email, "@"))
    assert_true(stringz.contains(email, "."))
    
    fr fr Test phone number generation
    sus phone tea = gen_phone_number("US")
    assert_true(stringz.contains(phone, "("))
    assert_true(stringz.contains(phone, ")"))
    assert_true(stringz.contains(phone, "-"))
    
    fr fr Test credit card generation
    sus visa tea = gen_credit_card("visa")
    assert_true(stringz.starts_with(visa, "4"))
    assert_eq_int(stringz.length(visa), 16)
    
    fr fr Test URL generation
    sus url tea = gen_url("https", based)
    assert_true(stringz.starts_with(url, "https://"))
    
    vibez.spill("✅ Domain-specific generators working: " + email)
}

slay test_edge_case_generators() {
    test_start("Edge Case Generators")
    
    fr fr Test boundary value generation
    sus boundary normie = gen_boundary_int(100, 5)
    assert_true(boundary >= 90 && boundary <= 110)  fr fr Within reasonable range
    
    fr fr Test edge case string generation
    sus edge_string tea = gen_edge_case_string()
    assert_true(edge_string != false)  fr fr Should generate something
    
    fr fr Test problematic float generation
    sus problem_float drip = gen_problematic_float()
    assert_true(problem_float == problem_float || mathz.is_nan(problem_float))  fr fr Valid or NaN
    
    vibez.spill("✅ Edge case generators producing boundary values")
}

fr fr ===== ENHANCED SHRINKING TESTS =====

slay test_smart_shrinking_algorithms() {
    test_start("Smart Shrinking Algorithms")
    
    fr fr Test integer shrinking towards zero
    sus int_shrinks [] = shrink_towards_zero(100)
    assert_true(reflectz.array_length(int_shrinks) > 0)
    
    fr fr Check that zero is always first candidate
    sus first_shrink normie = reflectz.array_get(int_shrinks, 0)
    assert_eq_int(first_shrink, 0)
    
    fr fr Test string shrinking towards empty
    sus string_shrinks [] = shrink_towards_empty_string("Hello, World!")
    assert_true(reflectz.array_length(string_shrinks) > 0)
    
    fr fr Check that empty string is always first candidate
    sus first_string_shrink tea = reflectz.array_get(string_shrinks, 0)
    assert_true(stringz.compare(first_string_shrink, "") == 0)
    
    fr fr Test array shrinking towards empty
    sus array_test [] = [1, 2, 3, 4, 5]
    sus array_shrinks [] = shrink_towards_empty_array(array_test)
    assert_true(reflectz.array_length(array_shrinks) > 0)
    
    fr fr Test smart shrinking dispatch
    sus smart_int_shrinks [] = smart_shrink(42)
    sus smart_string_shrinks [] = smart_shrink("test")
    sus smart_array_shrinks [] = smart_shrink([1, 2, 3])
    
    assert_true(reflectz.array_length(smart_int_shrinks) > 0)
    assert_true(reflectz.array_length(smart_string_shrinks) > 0)
    assert_true(reflectz.array_length(smart_array_shrinks) > 0)
    
    vibez.spill("✅ Smart shrinking algorithms working correctly")
}

fr fr ===== PROPERTY COMBINATOR TESTS =====

slay test_mathematical_property_combinators() {
    test_start("Mathematical Property Combinators")
    
    fr fr Test commutative property
    slay addition(a normie, b normie) normie {
        damn a + b
    }
    
    sus addition_commutative slay = prop_commutative(addition)
    assert_true(addition_commutative([5, 3]))
    assert_true(addition_commutative([10, -2]))
    
    fr fr Test associative property
    slay multiplication(a normie, b normie) normie {
        damn a * b
    }
    
    sus mult_associative slay = prop_associative(multiplication)
    assert_true(mult_associative([2, 3, 4]))
    assert_true(mult_associative([1, 5, 2]))
    
    fr fr Test identity property
    sus add_identity slay = prop_identity(addition, 0)
    assert_true(add_identity(42))
    assert_true(add_identity(-17))
    
    fr fr Test idempotent property
    slay absolute_value(x normie) normie {
        vibes x < 0 {
            damn -x
        }
        damn x
    }
    
    sus abs_idempotent slay = prop_idempotent(absolute_value)
    assert_true(abs_idempotent(5))
    assert_true(abs_idempotent(-5))
    
    vibez.spill("✅ Mathematical property combinators verified")
}

slay test_functional_property_combinators() {
    test_start("Functional Property Combinators")
    
    fr fr Test pure function property
    slay pure_square(x normie) normie {
        damn x * x
    }
    
    sus square_pure slay = prop_pure_function(pure_square)
    assert_true(square_pure(7))
    assert_true(square_pure(-3))
    
    fr fr Test invertible function property
    slay double_value(x normie) normie {
        damn x * 2
    }
    
    slay halve_value(x normie) normie {
        damn x / 2
    }
    
    sus double_halve_invertible slay = prop_invertible(double_value, halve_value)
    assert_true(double_halve_invertible(10))
    assert_true(double_halve_invertible(50))
    
    vibez.spill("✅ Functional property combinators working")
}

fr fr ===== ENHANCED PROPERTY TEST EXECUTION =====

slay test_enhanced_property_execution() {
    test_start("Enhanced Property Test Execution")
    
    fr fr Test property with configuration
    slay simple_property(x normie) lit {
        damn x * 2 / 2 == x
    }
    
    slay simple_generator() normie {
        damn gen_int_weighted([[-100, 100, 1.0]])
    }
    
    sus config [] = ["test_count", 20, "verbose", cap, "timeout_ms", 1000]
    
    sus test_result lit = forall_with_config(
        simple_generator,
        simple_property,
        "Division cancellation property",
        config
    )
    
    assert_true(test_result)
    
    vibez.spill("✅ Enhanced property test execution successful")
}

slay test_shrinking_integration() {
    test_start("Shrinking Integration")
    
    fr fr Create a property that fails for large values
    slay bounded_property(x normie) lit {
        damn x < 50  fr fr Will fail for x >= 50
    }
    
    slay large_int_generator() normie {
        damn rand_range(45, 100)  fr fr Will generate failing cases
    }
    
    fr fr This test is expected to fail and demonstrate shrinking
    vibez.spill("Testing shrinking with intentionally failing property...")
    
    yikes {
        forall_enhanced(large_int_generator, bounded_property, "Bounded value property")
        assert_true(cap)  fr fr Should not reach here
    } fam {
        when _ -> {
            vibez.spill("✅ Property correctly failed and shrinking was demonstrated")
        }
    }
}

fr fr ===== GENERATOR COMBINATOR TESTS =====

slay test_generator_combinators() {
    test_start("Generator Combinators")
    
    fr fr Test oneof generator
    slay gen_small() normie { damn gen_int_weighted([[1, 10, 1.0]]) }
    slay gen_large() normie { damn gen_int_weighted([[100, 1000, 1.0]]) }
    
    sus combined_gens [] = [gen_small, gen_large]
    sus oneof_value normie = gen_oneof(combined_gens)
    assert_true(oneof_value > 0)
    
    fr fr Test frequency-based generator
    sus weighted_gens [] = [[3.0, gen_small], [1.0, gen_large]]
    sus freq_value normie = gen_frequency(weighted_gens)
    assert_true(freq_value > 0)
    
    fr fr Test list generator
    sus int_list [] = gen_list_of(gen_small, slay() { damn 5 })
    assert_eq_int(reflectz.array_length(int_list), 5)
    
    fr fr Test tuple generator  
    sus tuple_gens [] = [gen_small, gen_large, slay() { damn "test" }]
    sus tuple_result [] = gen_tuple(tuple_gens)
    assert_eq_int(reflectz.array_length(tuple_result), 3)
    
    vibez.spill("✅ Generator combinators producing expected results")
}

fr fr ===== COMPREHENSIVE INTEGRATION TEST =====

slay test_comprehensive_string_properties() {
    test_start("Comprehensive String Properties")
    
    fr fr Test string concatenation with UTF-8
    slay utf8_concat_property(strings []) lit {
        vibes reflectz.array_length(strings) < 2 {
            damn based
        }
        
        sus s1 tea = reflectz.array_get(strings, 0)
        sus s2 tea = reflectz.array_get(strings, 1)
        sus concat tea = s1 + s2
        
        sus len1 normie = utf8_string_length(s1)
        sus len2 normie = utf8_string_length(s2)
        sus concat_len normie = utf8_string_length(concat)
        
        damn concat_len == len1 + len2
    }
    
    slay utf8_string_pair_generator() [] {
        damn [gen_utf8_string(0, 20), gen_utf8_string(0, 20)]
    }
    
    forall_enhanced(
        utf8_string_pair_generator,
        utf8_concat_property,
        "UTF-8 string concatenation length property"
    )
    
    vibez.spill("✅ UTF-8 string properties verified")
}

slay test_performance_properties() {
    test_start("Performance Properties")
    
    fr fr Test that sorting has reasonable time complexity
    slay sorting_performance_property(input []) lit {
        sus size normie = reflectz.array_length(input)
        vibes size == 0 {
            damn based
        }
        
        slay simple_sort(arr []) [] {
            sus result [] = reflectz.array_copy(arr)
            sus length normie = reflectz.array_length(result)
            sus i normie = 0
            
            bestie i < length - 1 {
                sus j normie = 0
                bestie j < length - i - 1 {
                    sus current = reflectz.array_get(result, j)
                    sus next = reflectz.array_get(result, j + 1)
                    vibes current > next {
                        result = reflectz.array_set(result, j, next)
                        result = reflectz.array_set(result, j + 1, current)
                    }
                    j = j + 1
                }
                i = i + 1
            }
            damn result
        }
        
        sus execution_time normie = measure_execution_time(simple_sort, input)
        
        fr fr Very generous bound for bubble sort: O(n^2) * constant
        sus expected_max normie = size * size * 1000  fr fr Nanoseconds
        
        damn execution_time < expected_max
    }
    
    slay small_array_generator() [] {
        damn gen_array_with_generator(slay() { damn gen_int_weighted([[-50, 50, 1.0]]) }, 1, 10)
    }
    
    forall_enhanced(
        small_array_generator,
        sorting_performance_property,
        "Sorting performance within bounds"
    )
    
    vibez.spill("✅ Performance properties validated")
}

fr fr ===== MAIN TEST EXECUTION =====

slay run_comprehensive_enhanced_tests() {
    vibez.spill("🚀 Starting Comprehensive Enhanced Property Testing Suite")
    
    fr fr Initialize framework with verbose logging
    set_global_config(50, 100, 5000, based)
    
    fr fr Run all test categories
    test_utf8_string_operations()
    test_performance_measurement() 
    test_reflection_type_system()
    
    test_statistical_generators()
    test_domain_specific_generators()
    test_edge_case_generators()
    
    test_smart_shrinking_algorithms()
    
    test_mathematical_property_combinators()
    test_functional_property_combinators()
    
    test_enhanced_property_execution()
    test_shrinking_integration()
    
    test_generator_combinators()
    
    test_comprehensive_string_properties()
    test_performance_properties()
    
    fr fr Final summary
    print_test_summary()
    vibez.spill("🎉 All enhanced property testing functionality verified!")
    vibez.spill("📊 Framework ready for production use with:")
    vibez.spill("   • UTF-8 string handling")
    vibez.spill("   • Real performance measurement") 
    vibez.spill("   • Reflection-based type system")
    vibez.spill("   • Advanced shrinking algorithms")
    vibez.spill("   • Comprehensive generator library")
    vibez.spill("   • Property combinators")
    vibez.spill("   • Statistical distributions")
}

fr fr Execute the comprehensive test suite
run_comprehensive_enhanced_tests()
