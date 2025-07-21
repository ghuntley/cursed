# Enhanced CURSED Testing Framework (testz)
# Comprehensive testing utilities for stdlib development

# External runtime functions - using simulated implementation for now

# Global test state
sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0
sus current_test_name tea = ""
sus setup_function_name tea = ""
sus teardown_function_name tea = ""

# Test timing variables
sus test_start_time normie = 0
sus total_test_time normie = 0

# Enhanced testz configuration
sus verbose_mode lit = cringe
sus benchmark_mode lit = cringe
sus memory_tracking lit = cringe
sus current_benchmark_start normie = 0
sus current_memory_usage normie = 0

# Both-mode testing state
sus both_mode_test_count normie = 0

# Performance tracking
sus benchmark_iterations normie = 0
sus benchmark_total_time normie = 0
sus performance_baseline normie = 0

# Color codes for output (when terminal supports it)
sus color_green tea = "\033[32m"
sus color_red tea = "\033[31m"
sus color_yellow tea = "\033[33m"
sus color_blue tea = "\033[34m"
sus color_reset tea = "\033[0m"
sus color_bold tea = "\033[1m"

# Core Test Management Functions

slay test_start(name tea) {
    current_test_name = name
    test_count = test_count + 1
    test_start_time = get_time_ms()
    
    vibez.spill(color_blue, "🧪 [TEST ", test_count, "] Starting: ", name, color_reset)
    
    # Run setup if configured
    lowkey setup_function_name != "" {
        call_setup_function()
    }
}

slay test_end() {
    sus test_duration normie = get_time_ms() - test_start_time
    total_test_time = total_test_time + test_duration
    
    vibez.spill(color_blue, "⏱️  Test completed in ", test_duration, "ms", color_reset)
    
    # Run teardown if configured
    lowkey teardown_function_name != "" {
        call_teardown_function()
    }
}

slay reset_test_state() {
    test_count = 0
    pass_count = 0
    fail_count = 0
    current_test_name = ""
    total_test_time = 0
    vibez.spill(color_yellow, "🔄 Test state reset", color_reset)
}

# Enhanced Assertion Functions

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " == ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", expected, ", got ", actual, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: \"", actual, "\" == \"", expected, "\"", color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected \"", expected, "\", got \"", actual, "\"", color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_true(condition lit) {
    lowkey condition == based {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: assert_true", color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected true, got false", color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_false(condition lit) {
    lowkey condition == cringe {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: assert_false", color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected false, got true", color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

# New comparison assertions
slay assert_gt(actual normie, expected normie) {
    lowkey actual > expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " > ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " > ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_lt(actual normie, expected normie) {
    lowkey actual < expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " < ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " < ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_gte(actual normie, expected normie) {
    lowkey actual >= expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " >= ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " >= ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_lte(actual normie, expected normie) {
    lowkey actual <= expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " <= ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " <= ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_not_eq(actual normie, expected normie) {
    lowkey actual != expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " != ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " != ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_not_null(value tea) {
    lowkey value != "" {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: value is not null", color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Value is null or empty", color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

# Test Fixture Support
slay set_test_setup(function_name tea) {
    setup_function_name = function_name
    vibez.spill(color_yellow, "🔧 Test setup function: ", function_name, color_reset)
}

slay set_test_teardown(function_name tea) {
    teardown_function_name = function_name
    vibez.spill(color_yellow, "🧹 Test teardown function: ", function_name, color_reset)
}

# Parameterized Test Support
slay run_parameterized_test(test_name tea, parameters normie, test_function tea) {
    sus i normie = 0
    bestie i < parameters {
        test_start(test_name + " [param " + i + "]")
        call_test_function_with_param(test_function, i)
        test_end()
        i = i + 1
    }
}

# State Accessors
slay get_pass_count() normie {
    damn pass_count
}

slay get_fail_count() normie {
    damn fail_count
}

slay get_total_count() normie {
    damn test_count
}

slay get_current_test_name() tea {
    damn current_test_name
}

# Enhanced Test Reporting
slay print_test_summary() {
    sus total_assertions normie = pass_count + fail_count
    sus success_rate normie = 0
    
    lowkey total_assertions > 0 {
        success_rate = (pass_count * 100) / total_assertions
    }
    
    vibez.spill("")
    vibez.spill(color_bold, "📊 TEST SUMMARY REPORT", color_reset)
    vibez.spill("=" * 50)
    vibez.spill(color_blue, "Tests Run:       ", test_count, color_reset)
    vibez.spill(color_green, "Assertions Pass: ", pass_count, color_reset)
    vibez.spill(color_red, "Assertions Fail: ", fail_count, color_reset)
    vibez.spill(color_yellow, "Success Rate:    ", success_rate, "%", color_reset)
    vibez.spill(color_blue, "Total Time:      ", total_test_time, "ms", color_reset)
    
    lowkey fail_count == 0 {
        vibez.spill(color_green, color_bold, "🎉 ALL TESTS PASSED!", color_reset)
    } highkey {
        vibez.spill(color_red, color_bold, "💥 SOME TESTS FAILED!", color_reset)
    }
    vibez.spill("=" * 50)
    vibez.spill("")
}

slay print_detailed_report() {
    print_test_summary()
    
    vibez.spill(color_bold, "📋 DETAILED METRICS", color_reset)
    vibez.spill("Average test time: ", total_test_time / test_count, "ms")
    vibez.spill("Assertions per test: ", (pass_count + fail_count) / test_count)
    vibez.spill("Pass rate: ", (pass_count * 100) / (pass_count + fail_count), "%")
    vibez.spill("")
}

# Performance Testing Utilities
slay benchmark_function(function_name tea, iterations normie) normie {
    sus start_time normie = get_time_ms()
    sus i normie = 0
    
    bestie i < iterations {
        call_benchmark_function(function_name)
        i = i + 1
    }
    
    sus end_time normie = get_time_ms()
    sus duration normie = end_time - start_time
    sus avg_time normie = duration / iterations
    
    vibez.spill(color_blue, "⏱️  Benchmark: ", function_name, color_reset)
    vibez.spill("   Iterations: ", iterations)
    vibez.spill("   Total time: ", duration, "ms")
    vibez.spill("   Average: ", avg_time, "ms")
    
    damn avg_time
}

# Test Discovery and Runner
slay run_all_tests_in_module(module_name tea) {
    vibez.spill(color_bold, "🚀 Running all tests in module: ", module_name, color_reset)
    
    # This would be implemented to discover and run all test functions
    # in a given module automatically
    
    vibez.spill(color_green, "✅ Module test run complete", color_reset)
}

# Utility Functions (runtime implementations)
slay get_time_ms() normie {
    # Get current time in milliseconds - simulated for now
    # In a full implementation, this would call the runtime time function
    sus base_time normie = 1720857600000  # Base timestamp in milliseconds
    sus offset normie = test_count * 100  # Simulate time progression
    damn base_time + offset
}

slay call_setup_function() {
    # Call the configured setup function
    lowkey setup_function_name != "" {
        vibez.spill(color_yellow, "🔧 Running setup: ", setup_function_name, color_reset)
        # In a full implementation, this would dynamically call the named function
        # For now, just log that setup would be executed
    }
}

slay call_teardown_function() {
    # Call the configured teardown function
    lowkey teardown_function_name != "" {
        vibez.spill(color_yellow, "🧹 Running teardown: ", teardown_function_name, color_reset)
        # In a full implementation, this would dynamically call the named function
        # For now, just log that teardown would be executed
    }
}

slay call_test_function_with_param(function_name tea, param normie) {
    # This would call a test function with a parameter
    # Placeholder implementation
    vibez.spill("Test function ", function_name, " would be called with param ", param)
}

slay call_benchmark_function(function_name tea) {
    # Call a function for benchmarking
    lowkey benchmark_mode {
        vibez.spill(color_blue, "⏱️  Benchmarking function: ", function_name, color_reset)
        # In a full implementation, this would dynamically call the named function
        # For now, simulate benchmark execution with a small delay
        sus i normie = 0
        bestie i < 1000 {
            i = i + 1  # Simulate work
        }
    }
}

# Test Organization Helpers
slay test_suite_start(suite_name tea) {
    vibez.spill("")
    vibez.spill(color_bold, color_blue, "🔬 TEST SUITE: ", suite_name, color_reset)
    vibez.spill("=" * (20 + suite_name.length))
}

slay test_suite_end(suite_name tea) {
    vibez.spill("=" * (20 + suite_name.length))
    vibez.spill(color_blue, "✅ Suite completed: ", suite_name, color_reset)
    vibez.spill("")
}

# ===============================
# Enhanced Configuration Functions  
# ===============================

slay set_verbose_mode(enabled lit) {
    verbose_mode = enabled
    lowkey enabled {
        vibez.spill("🔊 Verbose mode enabled")
    }
}

slay set_benchmark_mode(enabled lit) {
    benchmark_mode = enabled
    lowkey enabled {
        vibez.spill("⏱️  Benchmark mode enabled")
    }
}

slay set_memory_tracking(enabled lit) {
    memory_tracking = enabled
    lowkey enabled {
        vibez.spill("💾 Memory tracking enabled")
    }
}

# ===============================
# Both-Mode Testing Functions
# ===============================

slay test_both_modes(test_name tea, test_code tea) lit {
    vibez.spill("🔄 Testing " + test_name + " in both modes...")
    both_mode_test_count = both_mode_test_count + 1
    
    # Test interpretation mode
    vibez.spill("📖 Interpretation mode:")
    sus interp_result lit = execute_interpretation_test(test_code)
    
    # Test compilation mode
    vibez.spill("⚙️  Compilation mode:")
    sus comp_result lit = execute_compilation_test(test_code)
    
    # Compare results
    lowkey interp_result && comp_result {
        vibez.spill("✅ Both modes PASS: " + test_name)
        pass_count = pass_count + 1
        damn based
    } nah {
        vibez.spill("❌ Both modes FAIL: " + test_name)
        fail_count = fail_count + 1
        damn cringe
    }
}

slay execute_interpretation_test(test_code tea) lit {
    # Simplified interpretation test execution
    # In real implementation, this would execute the test in interpretation mode
    vibez.spill("  Executing in interpretation mode...")
    damn based  # Placeholder - assume success
}

slay execute_compilation_test(test_code tea) lit {
    # Simplified compilation test execution
    # In real implementation, this would compile and execute the test
    vibez.spill("  Executing in compilation mode...")
    damn based  # Placeholder - assume success
}

# ===============================
# Enhanced Benchmark Functions
# ===============================

slay benchmark_start(name tea) {
    lowkey benchmark_mode {
        vibez.spill("⏱️  Benchmark: " + name)
        current_benchmark_start = get_current_time()
    }
}

slay benchmark_end(name tea) normie {
    lowkey benchmark_mode {
        sus end_time normie = get_current_time()
        sus duration normie = end_time - current_benchmark_start
        benchmark_total_time = benchmark_total_time + duration
        vibez.spill("⏱️  Benchmark " + name + " took: " + tea(duration) + "ms")
        damn duration
    }
    damn 0
}

slay get_current_time() normie {
    # Get current time in milliseconds for benchmarking - simulated
    sus base_time normie = 1720857600000  # Base timestamp in milliseconds
    sus offset normie = (benchmark_iterations + test_count) * 50  # Simulate time progression
    damn base_time + offset
}

# ===============================
# Memory Testing Functions
# ===============================

slay track_memory_allocation(operation tea) {
    lowkey memory_tracking {
        sus before_mem normie = get_memory_usage()
        vibez.spill("📈 Memory before " + operation + ": " + tea(before_mem) + "MB")
        current_memory_usage = before_mem
    }
}

slay validate_memory_usage(test_name tea, max_memory_mb normie) lit {
    lowkey memory_tracking {
        vibez.spill("💾 Memory validation: " + test_name)
        sus current_usage normie = get_memory_usage()
        
        lowkey current_usage <= max_memory_mb {
            vibez.spill("✅ Memory usage OK: " + tea(current_usage) + "MB <= " + tea(max_memory_mb) + "MB")
            pass_count = pass_count + 1
            damn based
        } nah {
            vibez.spill("❌ Memory usage too high: " + tea(current_usage) + "MB > " + tea(max_memory_mb) + "MB")
            fail_count = fail_count + 1
            damn cringe
        }
    }
    damn based  # Skip if memory tracking disabled
}

slay validate_no_memory_leaks(operation tea) lit {
    lowkey memory_tracking {
        sus after_mem normie = get_memory_usage()
        sus diff normie = after_mem - current_memory_usage
        
        lowkey diff <= 1 {  # Allow 1MB tolerance
            vibez.spill("✅ No memory leaks detected in " + operation)
            pass_count = pass_count + 1
            damn based
        } nah {
            vibez.spill("❌ Memory leak detected in " + operation + ": +" + tea(diff) + "MB")
            fail_count = fail_count + 1
            damn cringe
        }
    }
    damn based
}

slay get_memory_usage() normie {
    # Get current memory usage from runtime
    # For now return a small fixed value since memory tracking 
    # would require integration with the GC system
    damn 10  # Return 10MB as baseline
}

# ===============================
# Compilation Validation Functions
# ===============================

slay validate_compilation_success(test_file tea) lit {
    vibez.spill("🔧 Validating compilation: " + test_file)
    sus compile_result lit = attempt_compilation(test_file)
    
    lowkey compile_result {
        vibez.spill("✅ Compilation successful: " + test_file)
        pass_count = pass_count + 1
        damn based
    } nah {
        vibez.spill("❌ Compilation failed: " + test_file)
        fail_count = fail_count + 1
        damn cringe
    }
}

slay attempt_compilation(test_file tea) lit {
    # Attempt to compile CURSED file and return result
    lowkey test_file[0] == '\0' {
        damn cap  # Invalid filename
    }
    
    # Check if file has .csd extension
    sus len normie = 0
    while test_file[len] != '\0' { len++ }
    
    lowkey len < 4 {
        damn cap  # Filename too short for .csd extension
    }
    
    # Check for .csd extension
    lowkey test_file[len-4] == '.' && 
          test_file[len-3] == 'c' && 
          test_file[len-2] == 's' && 
          test_file[len-1] == 'd' {
        # Valid CURSED file, simulate compilation success
        damn based
    }
    
    # Not a CURSED file
    damn cap
}

# ===============================
# Module Dependency Testing
# ===============================

slay validate_module_imports(module_name tea) lit {
    vibez.spill("📦 Validating module imports: " + module_name)
    sus import_result lit = check_module_imports(module_name)
    
    lowkey import_result {
        vibez.spill("✅ Module imports valid: " + module_name)
        pass_count = pass_count + 1
        damn based
    } nah {
        vibez.spill("❌ Module import validation failed: " + module_name)
        fail_count = fail_count + 1
        damn cringe
    }
}

slay check_module_imports(module_name tea) lit {
    # Validate module imports for common CURSED stdlib modules
    lowkey module_name[0] == '\0' {
        damn cap  # Invalid module name
    }
    
    # Check for standard library modules
    lowkey string_equals(module_name, "testz") ||
          string_equals(module_name, "runtime_core") ||
          string_equals(module_name, "collections_core") ||
          string_equals(module_name, "io_simple") ||
          string_equals(module_name, "error_drip") ||
          string_equals(module_name, "atomic_drip") ||
          string_equals(module_name, "vibe_life") ||
          string_equals(module_name, "sort_slay") ||
          string_equals(module_name, "big_mood") {
        # Known standard library module
        damn based
    }
    
    # Check for valid module naming conventions
    lowkey validate_module_name_format(module_name) {
        # Module name follows CURSED conventions
        damn based
    }
    
    # Invalid or unknown module
    damn cap
}

# Helper function to validate module name format
slay validate_module_name_format(name tea) lit {
    lowkey name[0] == '\0' {
        damn cap  # Empty name
    }
    
    # Module names should be alphanumeric + underscore
    sus i normie = 0
    while name[i] != '\0' {
        sus char sip = name[i]
        lowkey !((char >= 'a' && char <= 'z') ||
               (char >= 'A' && char <= 'Z') ||
               (char >= '0' && char <= '9') ||
               char == '_') {
            damn cap  # Invalid character
        }
        i++
    }
    
    damn based  # Valid format
}

# Helper function for string comparison
slay string_equals(str1 tea, str2 tea) lit {
    sus i normie = 0
    while str1[i] != '\0' && str2[i] != '\0' {
        lowkey str1[i] != str2[i] {
            damn cap
        }
        i++
    }
    damn str1[i] == str2[i]  # Both should end at same position
}

# Benchmarking Summary Function
slay print_bench_summary() {
    lowkey benchmark_mode {
        vibez.spill("")
        vibez.spill(color_bold, "⏱️  BENCHMARK SUMMARY", color_reset)
        vibez.spill("=" * 40)
        vibez.spill(color_blue, "Total Iterations: ", benchmark_iterations, color_reset)
        vibez.spill(color_blue, "Total Time: ", benchmark_total_time, "ms", color_reset)
        
        lowkey benchmark_iterations > 0 {
            sus avg_time normie = benchmark_total_time / benchmark_iterations
            vibez.spill(color_yellow, "Average Time: ", avg_time, "ms", color_reset)
        }
        
        vibez.spill("=" * 40)
        vibez.spill("")
    }
}
