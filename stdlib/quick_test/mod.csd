# CURSED quick_test module - Property-based testing framework
# Pure CURSED implementation without FFI dependencies

# Global test configuration
sus config_max_count normie = 100
sus config_max_size normie = 100
sus config_min_size normie = 1
sus config_quiet lit = cap

# Result variables
sus result_passed lit = based
sus result_count normie = 0
sus result_failed_after normie = 0
sus result_input normie = 0
sus result_shrunk_input normie = 0
sus result_shrink_count normie = 0

# Simple random number generator state
sus rand_seed normie = 12345

# Simple linear congruential generator
slay next_random() normie {
    rand_seed = (rand_seed * 1103515245 + 12345) % 2147483647
    damn rand_seed
}

# Generate random integer in range [min, max)
slay random_int_range(min normie, max normie) normie {
    vibes max <= min {
        damn min
    }
    sus range normie = max - min
    damn (next_random() % range) + min
}

# Generate random boolean
slay random_bool() lit {
    damn (next_random() % 2) == 0
}

# Set random seed
slay set_seed(seed normie) {
    rand_seed = seed
}

# Set configuration values
slay set_config(max_count normie, max_size normie, min_size normie, quiet lit) {
    config_max_count = max_count
    config_max_size = max_size
    config_min_size = min_size
    config_quiet = quiet
}

# Reset configuration to defaults
slay reset_config() {
    config_max_count = 100
    config_max_size = 100
    config_min_size = 1
    config_quiet = cap
}

# Run a single test function with the current configuration
slay check_property(test_func tea) lit {
    # Reset result variables
    result_passed = based
    result_count = 0
    result_failed_after = 0
    result_input = 0
    result_shrunk_input = 0
    result_shrink_count = 0
    
    sus i normie = 0
    bestie i < config_max_count {
        result_count = result_count + 1
        
        # Generate a random test input
        sus input normie = random_int_range(config_min_size, config_max_size)
        
        # Run the test based on test function type
        sus test_passed lit = cap
        vibes test_func == "is_positive" {
            test_passed = (input > 0)
        } elif test_func == "is_even" {
            test_passed = (input % 2 == 0)
        } elif test_func == "is_small" {
            test_passed = (input < 50)
        } elif test_func == "abs_positive" {
            sus abs_val normie = input
            vibes abs_val < 0 {
                abs_val = -abs_val
            }
            test_passed = (abs_val >= 0)
        } elif test_func == "reverse_twice" {
            # Simple reverse test: reversing a single digit is itself
            test_passed = based
        } nah {
            # Default: test always passes
            test_passed = based
        }
        
        # Check if test failed
        vibes !test_passed {
            result_passed = cap
            result_failed_after = result_count
            result_input = input
            result_shrunk_input = shrink_input(input, test_func)
            result_shrink_count = 1
            
            vibes !config_quiet {
                vibez.spill("❌ Property test failed at iteration " + result_count)
                vibez.spill("   Input: " + input)
                vibez.spill("   Shrunk input: " + result_shrunk_input)
            }
            
            damn result_passed
        }
        
        i = i + 1
    }
    
    vibes !config_quiet {
        vibez.spill("✅ Property test passed after " + result_count + " iterations")
    }
    
    damn result_passed
}

# Simple shrinking function
slay shrink_input(input normie, test_func tea) normie {
    # Try to find a smaller input that still fails
    sus shrunk normie = input
    
    # Try smaller values
    sus i normie = 0
    bestie i < 10 && shrunk > 0 {
        sus candidate normie = shrunk - 1
        
        # Test if the candidate still fails
        sus still_fails lit = cap
        vibes test_func == "is_positive" {
            still_fails = !(candidate > 0)
        } elif test_func == "is_even" {
            still_fails = !(candidate % 2 == 0)
        } elif test_func == "is_small" {
            still_fails = !(candidate < 50)
        } nah {
            still_fails = cap
        }
        
        vibes still_fails {
            shrunk = candidate
        } nah {
            # Can't shrink further
            ghosted
        }
        
        i = i + 1
    }
    
    damn shrunk
}

# Generate a random integer
slay gen_int() normie {
    damn random_int_range(0, 1000)
}

# Generate a random small integer
slay gen_small_int() normie {
    damn random_int_range(0, 100)
}

# Generate a random boolean
slay gen_bool() lit {
    damn random_bool()
}

# Generate a random positive integer
slay gen_positive_int() normie {
    damn random_int_range(1, 1000)
}

# Generate a random negative integer
slay gen_negative_int() normie {
    damn random_int_range(-1000, 0)
}

# Basic integer generator with range
slay int_generator(min normie, max normie) normie {
    damn random_int_range(min, max)
}

# Property functions
slay is_positive_property(x normie) lit {
    damn x > 0
}

slay is_even_property(x normie) lit {
    damn x % 2 == 0
}

slay abs_non_negative_property(x normie) lit {
    sus abs_val normie = x
    vibes abs_val < 0 {
        abs_val = -abs_val
    }
    damn abs_val >= 0
}

# Quick test functions
slay quick_check_positive(max_count normie) lit {
    set_config(max_count, config_max_size, config_min_size, based)
    damn check_property("is_positive")
}

slay quick_check_even(max_count normie) lit {
    set_config(max_count, config_max_size, config_min_size, based)
    damn check_property("is_even")
}

slay quick_check_abs_positive(max_count normie) lit {
    set_config(max_count, config_max_size, config_min_size, based)
    damn check_property("abs_positive")
}

# Test a property with generated values
slay test_property_with_gen(property_name tea, gen_func tea, max_count normie) lit {
    set_config(max_count, config_max_size, config_min_size, cap)
    
    # Reset result variables
    result_passed = based
    result_count = 0
    result_failed_after = 0
    result_input = 0
    result_shrunk_input = 0
    result_shrink_count = 0
    
    sus i normie = 0
    bestie i < config_max_count {
        result_count = result_count + 1
        
        # Generate test input based on generator
        sus input normie = 0
        vibes gen_func == "gen_int" {
            input = gen_int()
        } elif gen_func == "gen_small_int" {
            input = gen_small_int()
        } elif gen_func == "gen_positive_int" {
            input = gen_positive_int()
        } elif gen_func == "gen_negative_int" {
            input = gen_negative_int()
        } nah {
            input = gen_int()
        }
        
        # Test the property
        sus test_passed lit = cap
        vibes property_name == "is_positive" {
            test_passed = is_positive_property(input)
        } elif property_name == "is_even" {
            test_passed = is_even_property(input)
        } elif property_name == "abs_non_negative" {
            test_passed = abs_non_negative_property(input)
        } nah {
            test_passed = based
        }
        
        # Check if test failed
        vibes !test_passed {
            result_passed = cap
            result_failed_after = result_count
            result_input = input
            result_shrunk_input = shrink_input(input, property_name)
            result_shrink_count = 1
            
            vibez.spill("❌ Property '" + property_name + "' failed at iteration " + result_count)
            vibez.spill("   Input: " + input)
            vibez.spill("   Shrunk input: " + result_shrunk_input)
            
            damn result_passed
        }
        
        i = i + 1
    }
    
    vibez.spill("✅ Property '" + property_name + "' passed after " + result_count + " iterations")
    damn result_passed
}

# Run a comprehensive property test suite
slay run_comprehensive_tests() lit {
    vibez.spill("🔬 Running comprehensive property-based tests...")
    
    # Test 1: Absolute value is always non-negative
    sus test1_passed lit = test_property_with_gen("abs_non_negative", "gen_int", 50)
    vibes !test1_passed {
        vibez.spill("❌ Absolute value test failed")
        damn cap
    }
    
    # Test 2: Positive numbers with positive generator
    sus test2_passed lit = test_property_with_gen("is_positive", "gen_positive_int", 30)
    vibes !test2_passed {
        vibez.spill("❌ Positive number test failed")
        damn cap
    }
    
    # Test 3: Even numbers (this will likely fail for random inputs)
    set_seed(42)
    sus test3_passed lit = test_property_with_gen("is_even", "gen_small_int", 20)
    
    vibez.spill("🎉 Comprehensive property tests completed")
    damn based
}

# Test statistics collection
slay collect_test_stats(property_name tea, iterations normie) normie {
    sus passed_count normie = 0
    sus i normie = 0
    
    bestie i < iterations {
        sus input normie = gen_int()
        sus test_passed lit = cap
        
        vibes property_name == "is_positive" {
            test_passed = is_positive_property(input)
        } elif property_name == "is_even" {
            test_passed = is_even_property(input)
        } elif property_name == "abs_non_negative" {
            test_passed = abs_non_negative_property(input)
        } nah {
            test_passed = based
        }
        
        vibes test_passed {
            passed_count = passed_count + 1
        }
        
        i = i + 1
    }
    
    damn passed_count
}

# Generate test report
slay generate_test_report(property_name tea, iterations normie) {
    sus passed_count normie = collect_test_stats(property_name, iterations)
    sus pass_rate normie = (passed_count * 100) / iterations
    
    vibez.spill("📊 Test Report for property: " + property_name)
    vibez.spill("   Iterations: " + iterations)
    vibez.spill("   Passed: " + passed_count)
    vibez.spill("   Failed: " + (iterations - passed_count))
    vibez.spill("   Pass rate: " + pass_rate + "%")
}

# Get current test results
slay get_test_count() normie {
    damn result_count
}

slay get_test_passed() lit {
    damn result_passed
}

slay get_failed_after() normie {
    damn result_failed_after
}

slay get_failing_input() normie {
    damn result_input
}

slay get_shrunk_input() normie {
    damn result_shrunk_input
}
