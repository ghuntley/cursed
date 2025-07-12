# CURSED quick_test module tests
# Comprehensive tests for property-based testing framework

yeet "testz"
yeet "quick_test"

# Test basic configuration
test_start("reset_config test")
reset_config()
assert_eq_int(config_max_count, 100)
assert_eq_int(config_max_size, 100)
assert_eq_int(config_min_size, 1)
assert_false(config_quiet)
test_end()

# Test set configuration
test_start("set_config test")
set_config(50, 200, 10, based)
assert_eq_int(config_max_count, 50)
assert_eq_int(config_max_size, 200)
assert_eq_int(config_min_size, 10)
assert_true(config_quiet)
test_end()

# Test random number generation
test_start("random number generation")
set_seed(42)
sus rand1 normie = next_random()
sus rand2 normie = next_random()
assert_ne_int(rand1, rand2)
assert_gt_int(rand1, 0)
assert_gt_int(rand2, 0)
test_end()

# Test random integer range generation
test_start("random_int_range")
set_seed(123)
sus val1 normie = random_int_range(10, 20)
sus val2 normie = random_int_range(10, 20)
assert_true(val1 >= 10)
assert_true(val1 < 20)
assert_true(val2 >= 10)
assert_true(val2 < 20)
test_end()

# Test random boolean generation
test_start("random_bool")
set_seed(456)
sus bool1 lit = random_bool()
sus bool2 lit = random_bool()
assert_true(bool1 == based || bool1 == cap)
assert_true(bool2 == based || bool2 == cap)
test_end()

# Test generators
test_start("gen_int generator")
set_seed(789)
sus int_val normie = gen_int()
assert_true(int_val >= 0)
assert_true(int_val < 1000)
test_end()

test_start("gen_small_int generator")
sus small_int normie = gen_small_int()
assert_true(small_int >= 0)
assert_true(small_int < 100)
test_end()

test_start("gen_positive_int generator")
sus pos_int normie = gen_positive_int()
assert_true(pos_int >= 1)
assert_true(pos_int < 1000)
test_end()

test_start("gen_negative_int generator")
sus neg_int normie = gen_negative_int()
assert_true(neg_int < 0)
assert_true(neg_int >= -1000)
test_end()

# Test property functions
test_start("is_positive_property")
assert_true(is_positive_property(5))
assert_false(is_positive_property(-5))
assert_false(is_positive_property(0))
test_end()

test_start("is_even_property")
assert_true(is_even_property(4))
assert_false(is_even_property(3))
assert_true(is_even_property(0))
assert_false(is_even_property(1))
test_end()

test_start("abs_non_negative_property")
assert_true(abs_non_negative_property(5))
assert_true(abs_non_negative_property(-5))
assert_true(abs_non_negative_property(0))
assert_true(abs_non_negative_property(100))
assert_true(abs_non_negative_property(-100))
test_end()

# Test shrinking function
test_start("shrink_input for positive property")
sus shrunk1 normie = shrink_input(10, "is_positive")
assert_true(shrunk1 <= 10)
sus shrunk2 normie = shrink_input(-5, "is_positive")
assert_true(shrunk2 <= -5)
test_end()

# Test basic property checking
test_start("check_property for abs_positive")
set_seed(999)
set_config(10, 100, 1, based)  # quiet mode
sus result1 lit = check_property("abs_positive")
assert_true(result1)
assert_eq_int(get_test_count(), 10)
test_end()

# Test property that should fail
test_start("check_property for is_positive (should fail)")
set_seed(1000)
set_config(20, 50, -50, based)  # quiet mode with negative range
sus result2 lit = check_property("is_positive")
assert_false(result2)
assert_true(get_failed_after() > 0)
test_end()

# Test property that should pass
test_start("check_property for is_positive with positive range")
set_config(15, 100, 1, based)  # quiet mode with positive range
sus result3 lit = check_property("is_positive")
assert_true(result3)
assert_eq_int(get_test_count(), 15)
test_end()

# Test quick check functions
test_start("quick_check_abs_positive")
sus abs_result lit = quick_check_abs_positive(25)
assert_true(abs_result)
test_end()

# Test property with generator
test_start("test_property_with_gen")
set_seed(1234)
sus gen_result lit = test_property_with_gen("abs_non_negative", "gen_int", 20)
assert_true(gen_result)
test_end()

# Test property with generator that should fail
test_start("test_property_with_gen (should fail)")
set_seed(5678)
sus gen_result2 lit = test_property_with_gen("is_positive", "gen_int", 15)
assert_false(gen_result2)
test_end()

# Test property with positive generator
test_start("test_property_with_gen (positive generator)")
sus gen_result3 lit = test_property_with_gen("is_positive", "gen_positive_int", 10)
assert_true(gen_result3)
test_end()

# Test statistics collection
test_start("collect_test_stats")
set_seed(9999)
sus stats1 normie = collect_test_stats("abs_non_negative", 50)
assert_eq_int(stats1, 50)  # Should all pass
sus stats2 normie = collect_test_stats("is_positive", 50)
assert_true(stats2 < 50)  # Some should fail
test_end()

# Test integer generator with range
test_start("int_generator")
set_seed(2468)
sus gen_val1 normie = int_generator(20, 30)
assert_true(gen_val1 >= 20)
assert_true(gen_val1 < 30)
sus gen_val2 normie = int_generator(100, 200)
assert_true(gen_val2 >= 100)
assert_true(gen_val2 < 200)
test_end()

# Test edge cases
test_start("edge cases - same min/max")
sus edge_val normie = random_int_range(50, 50)
assert_eq_int(edge_val, 50)
test_end()

test_start("edge cases - single value range")
sus single_val normie = random_int_range(10, 11)
assert_eq_int(single_val, 10)
test_end()

# Test boolean generator
test_start("gen_bool")
set_seed(1357)
sus bool_val lit = gen_bool()
assert_true(bool_val == based || bool_val == cap)
test_end()

# Test comprehensive test runner
test_start("run_comprehensive_tests")
set_seed(24680)
sus comp_result lit = run_comprehensive_tests()
assert_true(comp_result)
test_end()

# Test result accessor functions
test_start("result accessor functions")
set_seed(11111)
set_config(5, 100, 1, based)
check_property("abs_positive")
assert_eq_int(get_test_count(), 5)
assert_true(get_test_passed())
test_end()

# Test report generation (just test it runs without errors)
test_start("generate_test_report")
set_seed(13579)
generate_test_report("abs_non_negative", 10)
generate_test_report("is_positive", 20)
generate_test_report("is_even", 15)
assert_true(based)
test_end()

# Test with different seed values
test_start("different seed values")
set_seed(12345)
sus seed_val1 normie = next_random()
set_seed(54321)
sus seed_val2 normie = next_random()
assert_ne_int(seed_val1, seed_val2)
test_end()

# Test shrinking with different properties
test_start("shrinking with is_even")
sus shrunk_even normie = shrink_input(10, "is_even")
assert_true(shrunk_even <= 10)
sus shrunk_even2 normie = shrink_input(7, "is_even")
assert_true(shrunk_even2 <= 7)
test_end()

print_test_summary()
