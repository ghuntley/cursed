yeet "testz"
yeet "errorz"

fr fr Test suite for CURSED Error Handling Module (errorz)

slay test_error_creation() {
    test_start("Error Creation")
    
    fr fr Test basic error creation
    sus err *ErrorInstance = create_error("Test error")
    assert_true(err != 0)
    assert_eq_string(error_message(err), "Test error")
    assert_eq_int(error_code(err), 0)
    assert_eq_int(error_severity(err), ERROR)
    
    fr fr Test error with code
    sus err_with_code *ErrorInstance = create_error_with_code("File not found", 404)
    assert_eq_string(error_message(err_with_code), "File not found")
    assert_eq_int(error_code(err_with_code), 404)
    
    fr fr Test detailed error
    sus detailed_err *ErrorInstance = create_detailed_error("Connection failed", 500, "Server timeout", CRITICAL)
    assert_eq_string(error_message(detailed_err), "Connection failed")
    assert_eq_int(error_code(detailed_err), 500)
    assert_eq_string(error_details(detailed_err), "Server timeout")
    assert_eq_int(error_severity(detailed_err), CRITICAL)
    
    vibez.spill("✅ Error creation tests passed")
}

slay test_error_categories() {
    test_start("Error Categories")
    
    fr fr Test memory error
    sus mem_err *ErrorInstance = create_memory_error("Out of memory")
    assert_true(is_memory_error(mem_err))
    assert_eq_int(error_category(mem_err), MEMORY_YIKES)
    assert_eq_int(error_code(mem_err), MEMORY_YIKES + 1)
    
    fr fr Test I/O error
    sus io_err *ErrorInstance = create_io_error("File not found", 404)
    assert_true(is_io_error(io_err))
    assert_eq_int(error_category(io_err), IO_YIKES)
    assert_eq_int(error_code(io_err), IO_YIKES + 404)
    
    fr fr Test network error
    sus net_err *ErrorInstance = create_network_error("Connection timeout", 408)
    assert_true(is_network_error(net_err))
    assert_eq_int(error_category(net_err), NETWORK_YIKES)
    
    fr fr Test parse error
    sus parse_err *ErrorInstance = create_parse_error("Syntax error", 42)
    assert_true(is_parse_error(parse_err))
    assert_eq_int(error_category(parse_err), PARSE_YIKES)
    assert_eq_int(parse_err.source_line, 42)
    
    fr fr Test type error
    sus type_err *ErrorInstance = create_type_error("Type mismatch")
    assert_true(is_type_error(type_err))
    assert_eq_int(error_category(type_err), TYPE_YIKES)
    
    fr fr Test security error
    sus sec_err *ErrorInstance = create_security_error("Access denied")
    assert_true(is_security_error(sec_err))
    assert_eq_int(error_category(sec_err), SECURITY_YIKES)
    assert_eq_int(error_severity(sec_err), CRITICAL)
    
    vibez.spill("✅ Error category tests passed")
}

slay test_error_wrapping() {
    test_start("Error Wrapping")
    
    fr fr Test basic wrapping
    sus original *ErrorInstance = create_error("Original error")
    sus wrapped *ErrorInstance = wrap_error(original, "Context")
    
    assert_eq_string(error_message(wrapped), "Context: Original error")
    assert_eq_int(error_code(wrapped), error_code(original))
    assert_true(wrapped.wrapped_error == original)
    
    fr fr Test unwrapping
    sus unwrapped *ErrorInstance = unwrap_error(wrapped)
    assert_true(unwrapped == original)
    assert_eq_string(error_message(unwrapped), "Original error")
    
    fr fr Test root error
    sus double_wrapped *ErrorInstance = wrap_error(wrapped, "Outer context")
    sus root *ErrorInstance = root_error(double_wrapped)
    assert_true(root == original)
    assert_eq_string(error_message(root), "Original error")
    
    fr fr Test wrapping null error
    sus null_wrapped *ErrorInstance = wrap_error(0, "Context for null")
    assert_true(null_wrapped != 0)
    assert_eq_string(error_message(null_wrapped), "Context for null")
    
    vibez.spill("✅ Error wrapping tests passed")
}

slay test_error_formatting() {
    test_start("Error Formatting")
    
    sus err *ErrorInstance = create_detailed_error("Test error", 500, "Additional details", CRITICAL)
    
    fr fr Test basic formatting
    sus formatted tea = format_error(err)
    assert_true(formatted != "")
    vibez.spill("Formatted error: " + formatted)
    
    fr fr Test formatting with stack
    sus formatted_with_stack tea = format_error_with_stack(err)
    assert_true(formatted_with_stack != "")
    
    fr fr Test null error formatting
    sus null_formatted tea = format_error(0)
    assert_eq_string(null_formatted, "no error")
    
    vibez.spill("✅ Error formatting tests passed")
}

slay test_error_comparison() {
    test_start("Error Comparison")
    
    sus err1 *ErrorInstance = create_error_with_code("Test error", 404)
    sus err2 *ErrorInstance = create_error_with_code("Test error", 404)
    sus err3 *ErrorInstance = create_error_with_code("Different error", 404)
    sus err4 *ErrorInstance = create_error_with_code("Test error", 500)
    
    fr fr Test error equality
    assert_true(errors_equal(err1, err2))
    assert_false(errors_equal(err1, err3))
    assert_false(errors_equal(err1, err4))
    assert_true(errors_equal(0, 0))
    assert_false(errors_equal(err1, 0))
    
    fr fr Test code matching
    assert_true(error_matches_code(err1, 404))
    assert_false(error_matches_code(err1, 500))
    assert_false(error_matches_code(0, 404))
    
    fr fr Test category matching
    sus mem_err *ErrorInstance = create_memory_error("Memory error")
    assert_true(error_matches_category(mem_err, MEMORY_YIKES))
    assert_false(error_matches_category(mem_err, IO_YIKES))
    
    vibez.spill("✅ Error comparison tests passed")
}

slay test_error_collection() {
    test_start("Error Collection")
    
    sus collection *ErrorCollection = create_error_collection(5)
    assert_true(collection != 0)
    assert_false(has_errors(collection))
    assert_eq_int(collection.count, 0)
    
    fr fr Add errors to collection
    sus err1 *ErrorInstance = create_error("Error 1")
    sus err2 *ErrorInstance = create_error("Error 2")
    sus err3 *ErrorInstance = create_error("Error 3")
    
    assert_true(add_error(collection, err1))
    assert_true(add_error(collection, err2))
    assert_true(add_error(collection, err3))
    
    assert_true(has_errors(collection))
    assert_eq_int(collection.count, 3)
    
    fr fr Test combining errors
    sus combined *ErrorInstance = combine_errors(collection)
    assert_true(combined != 0)
    assert_eq_int(error_severity(combined), CRITICAL)
    
    sus combined_message tea = error_message(combined)
    assert_true(combined_message != "")
    vibez.spill("Combined error: " + combined_message)
    
    fr fr Test empty collection
    sus empty_collection *ErrorCollection = create_error_collection(5)
    sus empty_combined *ErrorInstance = combine_errors(empty_collection)
    assert_true(empty_combined == 0)
    
    vibez.spill("✅ Error collection tests passed")
}

slay test_panic_handling() {
    test_start("Panic Handling")
    
    fr fr Test panic triggering
    assert_false(has_active_panic())
    
    trigger_panic("Test panic")
    assert_true(has_active_panic())
    
    fr fr Test panic recovery
    sus recovered *PanicValue = recover_panic()
    assert_true(recovered != 0)
    assert_eq_string(recovered.message, "Test panic")
    assert_eq_int(recovered.severity, FATAL)
    assert_true(recovered.recovered)
    
    assert_false(has_active_panic())
    
    fr fr Test critical panic
    trigger_critical_panic("Critical test panic")
    sus critical_recovered *PanicValue = recover_panic()
    assert_true(critical_recovered != 0)
    assert_eq_int(critical_recovered.severity, CRITICAL)
    
    vibez.spill("✅ Panic handling tests passed")
}

slay test_error_statistics() {
    test_start("Error Statistics")
    
    fr fr Initialize and test empty stats
    initialize_error_stats()
    sus stats *ErrorStats = get_error_stats()
    assert_true(stats != 0)
    assert_eq_int(stats.total_errors, 0)
    
    fr fr Record some errors
    sus mem_err *ErrorInstance = create_memory_error("Memory error")
    sus io_err *ErrorInstance = create_io_error("I/O error", 404)
    sus critical_err *ErrorInstance = create_detailed_error("Critical error", 500, "", CRITICAL)
    
    record_error(mem_err)
    record_error(io_err)
    record_error(critical_err)
    
    fr fr Check updated stats
    sus updated_stats *ErrorStats = get_error_stats()
    assert_eq_int(updated_stats.total_errors, 3)
    assert_eq_int(updated_stats.errors_by_severity[ERROR], 2)
    assert_eq_int(updated_stats.errors_by_severity[CRITICAL], 1)
    
    print_error_stats()
    
    vibez.spill("✅ Error statistics tests passed")
}

slay test_circuit_breaker() {
    test_start("Circuit Breaker")
    
    sus cb *CircuitBreaker = create_circuit_breaker(3, 1000)
    assert_true(cb != 0)
    assert_eq_int(cb.state, CIRCUIT_CLOSED)
    assert_eq_int(cb.failure_threshold, 3)
    
    fr fr Define a failing operation
    slay failing_operation() *ErrorInstance {
        damn create_error("Operation failed")
    }
    
    fr fr Define a successful operation
    slay successful_operation() *ErrorInstance {
        damn 0  fr fr No error = success
    }
    
    fr fr Test circuit breaker with failures
    sus result1 *ErrorInstance = circuit_breaker_call(cb, failing_operation)
    assert_true(result1 != 0)
    assert_eq_int(cb.failure_count, 1)
    assert_eq_int(cb.state, CIRCUIT_CLOSED)
    
    sus result2 *ErrorInstance = circuit_breaker_call(cb, failing_operation)
    sus result3 *ErrorInstance = circuit_breaker_call(cb, failing_operation)
    assert_eq_int(cb.failure_count, 3)
    assert_eq_int(cb.state, CIRCUIT_OPEN)
    
    fr fr Test circuit breaker when open
    sus result4 *ErrorInstance = circuit_breaker_call(cb, successful_operation)
    assert_true(result4 != 0)
    assert_eq_string(error_message(result4), "Circuit breaker is open")
    
    fr fr Test success resets circuit breaker
    cb.state = CIRCUIT_CLOSED
    cb.failure_count = 0
    sus success_result *ErrorInstance = circuit_breaker_call(cb, successful_operation)
    assert_true(success_result == 0)
    assert_eq_int(cb.failure_count, 0)
    assert_eq_int(cb.state, CIRCUIT_CLOSED)
    
    vibez.spill("✅ Circuit breaker tests passed")
}

slay test_result_types() {
    test_start("Result Types")
    
    fr fr Test successful result
    sus ok_res *Result = ok_result(42)
    assert_true(is_ok(ok_res))
    assert_false(is_error(ok_res))
    assert_eq_int(unwrap_result(ok_res), 42)
    
    fr fr Test error result
    sus err *ErrorInstance = create_error("Test error")
    sus err_res *Result = error_result(err)
    assert_false(is_ok(err_res))
    assert_true(is_error(err_res))
    
    sus unwrapped_err *ErrorInstance = unwrap_error_from_result(err_res)
    assert_true(unwrapped_err == err)
    assert_eq_string(error_message(unwrapped_err), "Test error")
    
    vibez.spill("✅ Result type tests passed")
}

slay test_validation_helpers() {
    test_start("Validation Helpers")
    
    fr fr Test null pointer check
    sus null_err *ErrorInstance = check_null(0, "test_pointer")
    assert_true(null_err != 0)
    assert_true(is_error(error_result(null_err)))
    
    sus valid_ptr normie = 42
    sus valid_err *ErrorInstance = check_null(&valid_ptr, "valid_pointer")
    assert_true(valid_err == 0)
    
    fr fr Test bounds checking
    sus bounds_err *ErrorInstance = check_bounds(10, 5)
    assert_true(bounds_err != 0)
    
    sus valid_bounds_err *ErrorInstance = check_bounds(3, 5)
    assert_true(valid_bounds_err == 0)
    
    fr fr Test positive value check
    sus positive_err *ErrorInstance = check_positive(-5, "test_value")
    assert_true(positive_err != 0)
    
    sus valid_positive_err *ErrorInstance = check_positive(10, "test_value")
    assert_true(valid_positive_err == 0)
    
    fr fr Test string validation
    sus empty_string_err *ErrorInstance = validate_string_not_empty("", "test_string")
    assert_true(empty_string_err != 0)
    
    sus valid_string_err *ErrorInstance = validate_string_not_empty("valid", "test_string")
    assert_true(valid_string_err == 0)
    
    vibez.spill("✅ Validation helper tests passed")
}

slay run_all_errorz_tests() {
    vibez.spill("🚀 Starting CURSED Error Handling (errorz) Tests")
    
    test_error_creation()
    test_error_categories()
    test_error_wrapping()
    test_error_formatting()
    test_error_comparison()
    test_error_collection()
    test_panic_handling()
    test_error_statistics()
    test_circuit_breaker()
    test_result_types()
    test_validation_helpers()
    
    print_test_summary()
    vibez.spill("✅ All errorz tests completed!")
}

fr fr Run tests when this file is executed
run_all_errorz_tests()
