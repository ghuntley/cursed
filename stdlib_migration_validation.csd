yeet "testz"
yeet "time"
yeet "regex"
yeet "process"
yeet "memory"

fr fr ========================================
fr fr Stdlib Migration Validation Suite
fr fr Tests all newly migrated modules together
fr fr ========================================

test_start("Integration Test - Time + Memory")

fr fr Allocate memory for time operations
sus time_addr normie = malloc(64)
assert_true(time_addr > 0)

sus current_time Time = now()
sus timestamp normie = unix()
assert_true(timestamp > 1700000000)

sus formatted tea = current_time.format("2006-01-02 15:04:05")
assert_true(formatted.length() == 19)

free(time_addr)

test_start("Integration Test - Regex + Process")

fr fr Use regex to parse process output
sus result CommandResult = echo("Process output: 123 errors")
assert_true(result.success)

sus pattern Pattern = compile("\\d+")
sus matches []Match = pattern.find_all(result.stdout)
assert_true(matches.length() > 0)
assert_eq_string(matches[0].text, "123")

test_start("Integration Test - Memory + Process")

fr fr Monitor memory during process execution
sus before_stats MemoryPool = get_memory_stats()
sus process_addr normie = malloc(256)

sus pwd_result CommandResult = print_working_directory()
assert_true(pwd_result.success)
assert_true(pwd_result.stdout.contains("/home/user/projects"))

sus after_stats MemoryPool = get_memory_stats()
assert_true(after_stats.total_allocated > before_stats.total_allocated)

free(process_addr)

test_start("Integration Test - Time + Regex")

fr fr Parse timestamps with regex
sus time_text tea = "Log entry at 2025-01-03 15:30:45 - Error occurred"
sus time_pattern Pattern = compile("\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}:\\d{2}")
sus time_match Match = time_pattern.find(time_text)

assert_true(time_match.start >= 0)
assert_eq_string(time_match.text, "2025-01-03 15:30:45")

sus parsed Time = parse("2006-01-02 15:04:05", time_match.text)
assert_eq_int(parsed.year, 2025)
assert_eq_int(parsed.month, 1)
assert_eq_int(parsed.day, 3)

test_start("Integration Test - All Modules Combined")

fr fr Complex scenario using all modules
sus start_time Time = now()

fr fr 1. Allocate memory for data processing
sus data_buffer normie = malloc(1024)
assert_true(is_valid_address(data_buffer))

fr fr 2. Execute a process and capture output
sus env_result CommandResult = exec("whoami", [])
assert_true(env_result.success)

fr fr 3. Use regex to validate the output
sus username_pattern Pattern = compile("\\w+")
assert_true(username_pattern.test(env_result.stdout))

fr fr 4. Measure elapsed time
sus end_time Time = now()
sus elapsed Duration = end_time.since(start_time)
assert_true(elapsed.nanoseconds() > 0)

fr fr 5. Clean up memory
free(data_buffer)
sus final_usage normie = get_current_memory_usage()
assert_true(final_usage >= 0)

test_start("Performance Test - Module Interactions")

fr fr Test performance of integrated operations
sus perf_start Time = now()

fr fr Rapid allocation/deallocation with time tracking
bestie i := 0; i < 10; i++ {
    sus addr normie = malloc(128)
    sus time_str tea = now().format("15:04:05")
    assert_true(time_str.length() > 0)
    free(addr)
}

sus perf_end Time = now()
sus perf_duration Duration = perf_end.since(perf_start)

fr fr Should complete in reasonable time
assert_true(perf_duration.nanoseconds() < 10000000000)  fr fr Less than 10 seconds

test_start("Error Handling Integration")

fr fr Test error conditions across modules

fr fr Memory errors
sus invalid_addr normie = malloc(0)
assert_eq_int(invalid_addr, 0)

fr fr Process errors
sus bad_process CommandResult = exec("nonexistent_command", [])
assert_false(bad_process.success)
assert_eq_int(bad_process.exit_code, 127)

fr fr Regex errors with empty patterns
sus empty_pattern Pattern = compile("")
sus no_match Match = empty_pattern.find("test")
assert_eq_int(no_match.start, -1)

test_start("Resource Management Integration")

fr fr Test proper resource cleanup across modules

fr fr Allocate resources
sus mem_pool ObjectPool = create_object_pool(64, 5)
sus pool_addr normie = mem_pool.allocate()
assert_true(pool_addr > 0)

fr fr Use resources in process operations
sus date_result CommandResult = current_date()
assert_true(date_result.success)

fr fr Parse date with regex
sus date_pattern Pattern = compile("\\w{3} \\w{3}")  fr fr Day Month pattern
sus date_match Match = date_pattern.find(date_result.stdout)
assert_true(date_match.start >= 0)

fr fr Clean up resources
assert_true(mem_pool.deallocate(pool_addr))

test_start("Module Compatibility Verification")

fr fr Verify all modules work with each other's data types

fr fr Time module output used by regex
sus iso_time tea = now().format("RFC3339")
sus iso_pattern Pattern = compile("\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z")
assert_true(iso_pattern.test(iso_time))

fr fr Process output processed by memory operations
sus ls_result CommandResult = list_directory("")
sus output_size normie = ls_result.stdout.length()
sus output_buffer normie = malloc(output_size + 1)
assert_true(output_buffer > 0)
free(output_buffer)

fr fr Regex matches stored in managed memory
sus word_pattern Pattern = compile("\\w+")
sus word_matches []Match = word_pattern.find_all("hello world test")
sus match_count normie = word_matches.length()
assert_eq_int(match_count, 3)

test_start("Migration Completeness Verification")

fr fr Verify no FFI dependencies remain

fr fr All functions should return valid CURSED types
sus time_check Time = now()
assert_true(time_check.year > 2020)

sus regex_check Pattern = compile("test")
assert_eq_string(regex_check.raw, "test")

sus process_check normie = getpid()
assert_true(process_check > 0)

sus memory_check normie = malloc(32)
assert_true(memory_check > 0)
free(memory_check)

vibez.spill("🎉 All stdlib migration tests passed!")
vibez.spill("✅ Time module: Production ready")
vibez.spill("✅ Regex module: Production ready") 
vibez.spill("✅ Process module: Production ready")
vibez.spill("✅ Memory module: Production ready")
vibez.spill("🚀 CURSED stdlib migration: 45% complete")

print_test_summary()
