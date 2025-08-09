yeet "timez"
yeet "testz"

fr fr ===== TIMEZ SLEEP FUNCTIONALITY TESTS =====

test_start("Basic Sleep Functions")

fr fr Test sleep function (should not busy-wait)
sleep(100)  # Sleep for 100ms
sleep_seconds(1)  # Sleep for 1 second  
sleep_minutes(1)  # Sleep for 1 minute (simulated)

# Test micro/nanosleep
usleep(500000)  # 500ms in microseconds
nanosleep(100000000)  # 100ms in nanoseconds

test_start("Sleep with Interruption")

# Test interrupt functionality
interrupt_sleep()
assert_false(is_sleeping())

# Test remaining sleep time
sus remaining drip = get_sleep_remaining()
assert_eq_int(remaining, 0)

test_start("High-Resolution Timing")

sus precise_time drip = precise_timestamp()
assert_true(precise_time > 0)

sus nano_time drip = timestamp_nanoseconds()
assert_true(nano_time > precise_time)

test_start("Benchmarking Functions")

sus start_time drip = benchmark_start()
sleep(50)  # Simulate some work
sus elapsed_microseconds drip = benchmark_end(start_time)
sus elapsed_ms drip = benchmark_ms(start_time)
sus elapsed_seconds drip = benchmark_seconds(start_time)

assert_true(elapsed_microseconds > 0)
assert_true(elapsed_ms >= 0)
assert_true(elapsed_seconds >= 0)

test_start("Rate Limiting and Throttling")

sus last_call drip = precise_timestamp() / 1000
sleep(10)  # Simulate some time passing

sus delay_needed drip = delay_between_calls(last_call, 100)
assert_true(delay_needed >= 0)

sus interval drip = rate_limit(10)  # 10 calls per second
assert_eq_int(interval, 100)  # Should be 100ms between calls

test_start("Timeout Operations")

assert_true(timeout_operation(1000))  # Start 1 second timeout
assert_false(is_timeout_expired(1000))  # Should not be expired immediately

# Test wait with timeout (condition already false, should timeout)
sus timeout_result lit = wait_with_timeout(cringe, 50, 10)
assert_false(timeout_result)  # Should timeout

test_start("Periodic Operations")

# Test periodic timer with few iterations
periodic_timer(10, 3)  # 10ms interval, 3 iterations

# Test scheduled repeated execution
schedule_repeated(5, 10, 2)  # 5ms initial delay, 10ms interval, 2 repetitions

test_start("Performance Monitoring")

sus db_time drip = measure_execution_time("database_query")
assert_true(db_time > 0)

sus file_time drip = measure_execution_time("file_io")
assert_true(file_time > 0)

sus network_time drip = measure_execution_time("network_request")
assert_true(network_time > 0)

# Run performance test with multiple iterations
performance_test("database_query", 3)

test_start("Sleep Until Target Time")

sus current_time drip = current_timestamp()
sus future_time drip = current_time + 1  # 1 second in future
sleep_until(future_time)

# Test past time (should not sleep)
sus past_time drip = current_time - 1
sleep_until(past_time)

test_start("Sleep with Callback")

# Test sleep with periodic progress callbacks
sleep_with_callback(100, 25)  # 100ms total, 25ms callback interval

test_start("Throttle Function Calls")

sus last_call_time drip = 0
sus new_call_time drip = throttle_call(last_call_time, 50)
assert_true(new_call_time > last_call_time)

print_test_summary()
