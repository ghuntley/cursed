yeet "testz"
yeet "goroutine_core"

# Test Goroutine Core System
test_start("Goroutine Core System Tests")

# Test 1: Scheduler initialization
sus init_result lit = init_goroutine_scheduler()
assert_true(init_result)
print_test_status("Scheduler initialization", init_result)

# Test 2: Goroutine spawning
sus goroutine_id normie = spawn_goroutine("test_function")
assert_true(goroutine_id > 0)
assert_true(goroutine_exists(goroutine_id))
print_test_status("Goroutine spawning", goroutine_id > 0)

# Test 3: Goroutine state management
sus initial_state normie = get_goroutine_state(goroutine_id)
assert_eq_int(initial_state, GOROUTINE_RUNNABLE)

sus state_change_result lit = set_goroutine_state(goroutine_id, GOROUTINE_RUNNING)
assert_true(state_change_result)
sus new_state normie = get_goroutine_state(goroutine_id)
assert_eq_int(new_state, GOROUTINE_RUNNING)
print_test_status("State management", based)

# Test 4: Goroutine execution
sus exec_result lit = execute_goroutine(goroutine_id)
assert_true(exec_result)
sus final_state normie = get_goroutine_state(goroutine_id)
assert_eq_int(final_state, GOROUTINE_DONE)
print_test_status("Goroutine execution", exec_result)

# Test 5: Multiple goroutine spawning
reset_scheduler()
init_goroutine_scheduler()

sus id1 normie = spawn_goroutine("func1")
sus id2 normie = spawn_goroutine("func2")
sus id3 normie = spawn_goroutine("func3")

assert_true(id1 > 0 && id2 > 0 && id3 > 0)
assert_true(id1 != id2 && id2 != id3 && id1 != id3)
print_test_status("Multiple goroutine spawning", based)

# Test 6: Scheduler queue management
sus stats_before map[tea]normie = get_scheduler_stats()
assert_true(stats_before["runnable_count"] >= 3)

sus run_result lit = run_next_goroutine()
assert_true(run_result)
sus stats_after map[tea]normie = get_scheduler_stats()
assert_true(stats_after["total_runs"] > stats_before["total_runs"])
print_test_status("Scheduler queue management", based)

# Test 7: Panic handling
sus panic_id normie = spawn_goroutine("panic_function")
sus panic_exec_result lit = execute_goroutine(panic_id)
assert_true(!panic_exec_result)  # Should fail due to panic
sus panic_state normie = get_goroutine_state(panic_id)
assert_eq_int(panic_state, GOROUTINE_DONE)  # Should be cleaned up
print_test_status("Panic handling", based)

# Test 8: Current goroutine tracking
reset_scheduler()
init_goroutine_scheduler()
sus current_before normie = current_goroutine_id()
assert_eq_int(current_before, 0)  # No current goroutine

sus track_id normie = spawn_goroutine("track_function")
execute_goroutine(track_id)
# After execution, should return to no current goroutine
sus current_after normie = current_goroutine_id()
assert_eq_int(current_after, 0)
print_test_status("Current goroutine tracking", based)

# Test 9: Goroutine cleanup
sus cleanup_id normie = spawn_goroutine("cleanup_function")
execute_goroutine(cleanup_id)
sus cleanup_result lit = cleanup_goroutine(cleanup_id)
assert_true(cleanup_result)
sus cleanup_state normie = get_goroutine_state(cleanup_id)
assert_eq_int(cleanup_state, GOROUTINE_DONE)
print_test_status("Goroutine cleanup", based)

# Test 10: Scheduler statistics
reset_scheduler()
init_goroutine_scheduler()

# Create several goroutines in different states
sus stats_id1 normie = spawn_goroutine("stats_func1")
sus stats_id2 normie = spawn_goroutine("stats_func2")
set_goroutine_state(stats_id1, GOROUTINE_BLOCKED)

sus detailed_stats map[tea]normie = get_scheduler_stats()
assert_true(detailed_stats["total_goroutines"] >= 2)
assert_true(detailed_stats["blocked_count"] >= 1)
assert_true(detailed_stats["runnable_count_by_state"] >= 1)
print_test_status("Scheduler statistics", based)

# Test 11: Health check
sus health_result lit = scheduler_health_check()
assert_true(health_result)
print_test_status("Health check", health_result)

# Test 12: Cooperative yielding
sus yield_id normie = spawn_goroutine("yield_function")
set_goroutine_state(yield_id, GOROUTINE_RUNNING)
sus prev_current normie = global_scheduler.current_id
global_scheduler.current_id = yield_id

sus yield_result lit = yield_goroutine()
assert_true(yield_result)
print_test_status("Cooperative yielding", yield_result)

# Test 13: Maximum goroutine limit
reset_scheduler()
init_goroutine_scheduler()

# Set next_id to near maximum to test limit
global_scheduler.next_id = MAX_GOROUTINES - 1
sus valid_id normie = spawn_goroutine("valid_function")
assert_true(valid_id > 0)

sus invalid_id normie = spawn_goroutine("invalid_function")
assert_eq_int(invalid_id, -1)  # Should fail due to limit
print_test_status("Maximum goroutine limit", based)

# Test 14: Function execution simulation
reset_scheduler()
init_goroutine_scheduler()

sus normal_id normie = spawn_goroutine("normal_function")
sus normal_exec lit = execute_function("normal_function")
assert_true(normal_exec)

sus panic_exec lit = execute_function("panic_function")
assert_true(!panic_exec)  # Should fail
print_test_status("Function execution simulation", based)

# Test 15: Panic recovery (basic test)
reset_scheduler()
init_goroutine_scheduler()

# Test recovery with no panic
sus no_panic_recovery tea = recover_goroutine_panic()
assert_eq_string(no_panic_recovery, "")

# Test setting up a panic state manually for recovery test
sus recovery_id normie = spawn_goroutine("recovery_function")
handle_goroutine_panic(recovery_id, "test_panic")
global_scheduler.current_id = recovery_id
set_goroutine_state(recovery_id, GOROUTINE_PANIC)

sus panic_recovery tea = recover_goroutine_panic()
assert_eq_string(panic_recovery, "test_panic")
print_test_status("Panic recovery", based)

print_test_summary()
