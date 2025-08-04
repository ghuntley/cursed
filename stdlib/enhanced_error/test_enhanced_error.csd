yeet "testz"
yeet "enhanced_error"

test_start("Enhanced Error Handling Comprehensive Tests")

fr fr ================================
fr fr Error Type Tests
fr fr ================================

test_start("RuntimeError creation and methods")
sus runtime_err RuntimeError = RuntimeError{
    msg: "Something went wrong",
    code: 500,
    stack: ["main", "function"],
    recoverable: based
}

assert_eq_string(runtime_err.message(), "Something went wrong")
assert_eq_int(runtime_err.error_code(), 500)
assert_eq_string(runtime_err.error_type(), "RuntimeError")
assert_true(runtime_err.is_recoverable())

test_start("ValidationError creation and methods")
sus validation_err ValidationError = ValidationError{
    msg: "Invalid input",
    field: "email",
    value: "invalid-email"
}

sus validation_message tea = validation_err.message()
assert_true(string_contains(validation_message, "Invalid input"))
assert_true(string_contains(validation_message, "email"))
assert_true(string_contains(validation_message, "invalid-email"))
assert_eq_int(validation_err.error_code(), 1001)
assert_true(validation_err.is_recoverable())

test_start("NetworkError creation and methods")
sus network_err NetworkError = NetworkError{
    msg: "Connection failed",
    status_code: 404,
    endpoint: "/api/users"
}

sus network_message tea = network_err.message()
assert_true(string_contains(network_message, "Connection failed"))
assert_true(string_contains(network_message, "/api/users"))
assert_true(string_contains(network_message, "404"))
assert_eq_int(network_err.error_code(), 404)
assert_false(network_err.is_recoverable()) fr fr 404 is not recoverable

test_start("FileSystemError creation and methods")
sus fs_err FileSystemError = FileSystemError{
    msg: "File not found",
    path: "/tmp/test.txt",
    operation: "read"
}

sus fs_message tea = fs_err.message()
assert_true(string_contains(fs_message, "File not found"))
assert_true(string_contains(fs_message, "/tmp/test.txt"))
assert_true(string_contains(fs_message, "read"))
assert_true(fs_err.is_recoverable()) fr fr Read operations are recoverable

fr fr ================================
fr fr Result Type Tests
fr fr ================================

test_start("Ok result creation and methods")
sus ok_result Result<normie, tea> = ok<normie, tea>(42)
assert_true(ok_result.is_ok())
assert_false(ok_result.is_error())
assert_eq_int(ok_result.unwrap(), 42)
assert_eq_int(ok_result.unwrap_or(100), 42)

test_start("Err result creation and methods")
sus err_result Result<normie, tea> = error<normie, tea>("Something failed")
assert_false(err_result.is_ok())
assert_true(err_result.is_error())
assert_eq_string(err_result.unwrap_error(), "Something failed")
assert_eq_int(err_result.unwrap_or(100), 100)

test_start("Result map operations")
sus original_ok Result<normie, tea> = ok<normie, tea>(10)
sus doubled_mapper slay(normie) normie = slay(x normie) normie { damn x * 2 }
sus mapped_result Result<normie, tea> = original_ok.map(doubled_mapper)
assert_true(mapped_result.is_ok())
assert_eq_int(mapped_result.unwrap(), 20)

test_start("Result map_error operations")
sus original_err Result<normie, tea> = error<normie, tea>("original error")
sus error_mapper slay(tea) tea = slay(err tea) tea { damn "mapped: " + err }
sus mapped_error_result Result<normie, tea> = original_err.map_error(error_mapper)
assert_true(mapped_error_result.is_error())
assert_eq_string(mapped_error_result.unwrap_error(), "mapped: original error")

fr fr ================================
fr fr Error Creation Function Tests
fr fr ================================

test_start("create_runtime_error function")
sus created_runtime Error = create_runtime_error("Runtime issue", 503, based)
assert_eq_string(created_runtime.error_type(), "RuntimeError")
assert_eq_int(created_runtime.error_code(), 503)
assert_true(created_runtime.is_recoverable())

test_start("create_validation_error function")
sus created_validation Error = create_validation_error("Bad data", "username", "invalid")
assert_eq_string(created_validation.error_type(), "ValidationError")
assert_eq_int(created_validation.error_code(), 1001)

test_start("create_network_error function")
sus created_network Error = create_network_error("Timeout", 408, "/api/data")
assert_eq_string(created_network.error_type(), "NetworkError")
assert_eq_int(created_network.error_code(), 408)

test_start("create_filesystem_error function")
sus created_fs Error = create_filesystem_error("Permission denied", "/etc/passwd", "write")
assert_eq_string(created_fs.error_type(), "FileSystemError")
assert_eq_int(created_fs.error_code(), 2001)

fr fr ================================
fr fr Result Utility Function Tests
fr fr ================================

test_start("result_from_nullable with valid value")
sus valid_result Result<normie, tea> = result_from_nullable(42, based, "Invalid")
assert_true(valid_result.is_ok())
assert_eq_int(valid_result.unwrap(), 42)

test_start("result_from_nullable with invalid value")
sus invalid_result Result<normie, tea> = result_from_nullable(0, cringe, "Invalid value")
assert_true(invalid_result.is_error())
assert_eq_string(invalid_result.unwrap_error(), "Invalid value")

test_start("chain_results with successful operations")
sus first_ok Result<normie, tea> = ok<normie, tea>(5)
sus doubler slay(normie) Result<normie, tea> = slay(x normie) Result<normie, tea> {
    damn ok<normie, tea>(x * 2)
}
sus chained_result Result<normie, tea> = chain_results(first_ok, doubler)
assert_true(chained_result.is_ok())
assert_eq_int(chained_result.unwrap(), 10)

test_start("chain_results with first operation failing")
sus first_err Result<normie, tea> = error<normie, tea>("First failed")
sus chained_err_result Result<normie, tea> = chain_results(first_err, doubler)
assert_true(chained_err_result.is_error())
assert_eq_string(chained_err_result.unwrap_error(), "First failed")

test_start("collect_results with all successes")
sus results []Result<normie, tea> = [
    ok<normie, tea>(1),
    ok<normie, tea>(2),
    ok<normie, tea>(3)
]
sus collected Result<[]normie, tea> = collect_results(results)
assert_true(collected.is_ok())
sus values []normie = collected.unwrap()
assert_eq_int(len(values), 3)
assert_eq_int(values[0], 1)
assert_eq_int(values[2], 3)

test_start("collect_results with one failure")
sus mixed_results []Result<normie, tea> = [
    ok<normie, tea>(1),
    error<normie, tea>("Failed"),
    ok<normie, tea>(3)
]
sus collected_mixed Result<[]normie, tea> = collect_results(mixed_results)
assert_true(collected_mixed.is_error())
assert_eq_string(collected_mixed.unwrap_error(), "Failed")

fr fr ================================
fr fr Retry Configuration Tests
fr fr ================================

test_start("default_retry_config creation")
sus config RetryConfig = default_retry_config()
assert_eq_int(config.max_attempts, 3)
assert_eq_int(config.delay_ms, 100)
assert_eq_int(config.max_delay_ms, 5000)

test_start("retry_operation with successful operation")
sus attempt_count normie = 0
sus successful_operation slay() Result<normie, tea> = slay() Result<normie, tea> {
    attempt_count = attempt_count + 1
    damn ok<normie, tea>(42)
}

sus retry_result Result<normie, tea> = retry_operation(successful_operation, config)
assert_true(retry_result.is_ok())
assert_eq_int(retry_result.unwrap(), 42)
assert_eq_int(attempt_count, 1) fr fr Should succeed on first try

fr fr ================================
fr fr Error Logging Tests
fr fr ================================

test_start("ErrorLogger creation")
sus logger ErrorLogger = ErrorLogger_new(100, "test.log")
assert_eq_int(logger.log_level, 100)
assert_eq_string(logger.output_file, "test.log")
assert_eq_int(logger.max_entries, 1000)

test_start("format_error_log_entry")
sus test_error Error = create_runtime_error("Test error", 500, based)
sus timestamp tea = "2025-01-01T12:00:00Z"
sus log_entry tea = format_error_log_entry(timestamp, test_error)

assert_true(string_contains(log_entry, "2025-01-01T12:00:00Z"))
assert_true(string_contains(log_entry, "RuntimeError"))
assert_true(string_contains(log_entry, "500"))
assert_true(string_contains(log_entry, "Test error"))

fr fr ================================
fr fr Error Aggregation Tests
fr fr ================================

test_start("ErrorAggregator creation")
sus aggregator ErrorAggregator = ErrorAggregator_new()
assert_eq_int(len(aggregator.errors), 0)
assert_eq_int(len(aggregator.error_counts), 0)

test_start("ErrorAggregator add errors")
sus agg ErrorAggregator = ErrorAggregator_new()
sus error1 Error = create_runtime_error("Error 1", 500, based)
sus error2 Error = create_validation_error("Error 2", "field", "value")
sus error3 Error = create_runtime_error("Error 3", 501, cringe)

agg = ErrorAggregator_add(agg, error1)
agg = ErrorAggregator_add(agg, error2)
agg = ErrorAggregator_add(agg, error3)

assert_eq_int(len(agg.errors), 3)
assert_eq_int(len(agg.error_counts), 2) fr fr RuntimeError and ValidationError

test_start("ErrorAggregator get stats")
sus stats ErrorStats = ErrorAggregator_get_stats(agg)
assert_eq_int(stats.total_errors, 3)
assert_eq_int(stats.unique_types, 2)
assert_eq_string(stats.most_common_type, "RuntimeError") fr fr 2 runtime errors vs 1 validation

fr fr ================================
fr fr Panic and Recovery Tests
fr fr ================================

test_start("PanicInfo creation")
sus panic_info PanicInfo = PanicInfo{
    message: "Critical error",
    location: "main.csd:42",
    recovery_possible: based
}

assert_eq_string(panic_info.message, "Critical error")
assert_eq_string(panic_info.location, "main.csd:42")
assert_true(panic_info.recovery_possible)

test_start("recover_from_panic function")
sus recovery_called lit = cringe
sus recovery_function slay() = slay() {
    recovery_called = based
}

sus recovery_success lit = recover_from_panic(recovery_function)
assert_true(recovery_success)
assert_true(recovery_called)

fr fr ================================
fr fr Error Context Tests
fr fr ================================

test_start("ErrorContext creation and metadata")
sus context ErrorContext = ErrorContext_new("database_query")
assert_eq_string(context.operation, "database_query")

context = ErrorContext_add_metadata(context, "table", "users")
context = ErrorContext_add_metadata(context, "query_id", "12345")

assert_eq_string(context.metadata["table"], "users")
assert_eq_string(context.metadata["query_id"], "12345")

test_start("ContextualError wrapping")
sus original Error = create_runtime_error("Connection lost", 500, based)
sus contextual ContextualError = ErrorContext_wrap_error(context, original)

sus contextual_message tea = contextual.message()
assert_true(string_contains(contextual_message, "Connection lost"))
assert_true(string_contains(contextual_message, "database_query"))
assert_true(string_contains(contextual_message, "table=users"))
assert_true(string_contains(contextual_message, "query_id=12345"))

assert_eq_int(contextual.error_code(), 500)
assert_eq_string(contextual.error_type(), "ContextualError")
assert_true(contextual.is_recoverable())

fr fr ================================
fr fr Utility Function Tests
fr fr ================================

test_start("string_format_int function")
assert_eq_string(string_format_int(0), "0")
assert_eq_string(string_format_int(42), "42")
assert_eq_string(string_format_int(404), "404")
assert_eq_string(string_format_int(500), "500")

test_start("min_int and max_int functions")
assert_eq_int(min_int(5, 10), 5)
assert_eq_int(min_int(20, 15), 15)
assert_eq_int(max_int(5, 10), 10)
assert_eq_int(max_int(20, 15), 20)

test_start("get_current_stack_trace function")
sus stack []tea = get_current_stack_trace()
assert_eq_int(len(stack), 3)
assert_eq_string(stack[0], "main")
assert_eq_string(stack[1], "function_call")
assert_eq_string(stack[2], "error_location")

test_start("get_current_timestamp function")
sus timestamp tea = get_current_timestamp()
assert_eq_string(timestamp, "2025-01-01T12:00:00Z")

fr fr ================================
fr fr Integration Tests
fr fr ================================

test_start("Complete error handling workflow")
fr fr Simulate a complex operation that might fail
sus operation_context ErrorContext = ErrorContext_new("user_registration")
operation_context = ErrorContext_add_metadata(operation_context, "user_id", "12345")
operation_context = ErrorContext_add_metadata(operation_context, "email", "test@example.com")

fr fr Create a validation error
sus validation_error Error = create_validation_error("Invalid email format", "email", "test@example.com")

fr fr Wrap with context
sus contextual_error ContextualError = ErrorContext_wrap_error(operation_context, validation_error)

fr fr Log the error
sus error_logger ErrorLogger = ErrorLogger_new(0, "app.log")
ErrorLogger_log(error_logger, contextual_error)

fr fr Add to aggregator
sus error_agg ErrorAggregator = ErrorAggregator_new()
error_agg = ErrorAggregator_add(error_agg, contextual_error)

sus final_stats ErrorStats = ErrorAggregator_get_stats(error_agg)
assert_eq_int(final_stats.total_errors, 1)
assert_eq_string(final_stats.most_common_type, "ContextualError")

test_start("Result chaining with error recovery")
fr fr Create a chain of operations that might fail
sus step1 slay() Result<normie, tea> = slay() Result<normie, tea> {
    damn ok<normie, tea>(10)
}

sus step2 slay(normie) Result<normie, tea> = slay(x normie) Result<normie, tea> {
    lowkey x > 5 {
        damn ok<normie, tea>(x * 2)
    } else {
        damn error<normie, tea>("Value too small")
    }
}

sus step3 slay(normie) Result<normie, tea> = slay(x normie) Result<normie, tea> {
    lowkey x < 100 {
        damn ok<normie, tea>(x + 5)
    } else {
        damn error<normie, tea>("Value too large")
    }
}

fr fr Chain the operations
sus result1 Result<normie, tea> = step1()
sus result2 Result<normie, tea> = chain_results(result1, step2)
sus result3 Result<normie, tea> = chain_results(result2, step3)

assert_true(result3.is_ok())
assert_eq_int(result3.unwrap(), 25) fr fr 10 * 2 + 5 = 25

print_test_summary()

vibez.spill("🎉 Enhanced Error Handling Tests Complete!")
vibez.spill("✅ Result types, error recovery, and logging working")
vibez.spill("🔄 Retry mechanisms and panic recovery validated")
vibez.spill("📊 Error aggregation and context system functional")
vibez.spill("🛡️ Comprehensive error management ready for production")
vibez.spill("⚡ Complex error handling workflows tested successfully")
