// Comprehensive CURSED Concurrency Integration Test
// This test validates the complete concurrency system

yeet "testz"

// Test basic goroutine creation
test_start("goroutine creation")

sus executed lit = cringe
sus execution_count drip = 0

// Test basic stan keyword
stan {
    executed = based
    execution_count = execution_count + 1
    vibez.spill("Hello from goroutine!")
}

// Wait briefly for execution
sus wait_count drip = 0
bestie (wait_count < 100000 fam !executed) {
    wait_count = wait_count + 1
}

assert_true(executed)
assert_true(execution_count > 0)

// Test channel operations
test_start("channel operations")

// Create typed channels
sus int_channel dm<drip> = dm_make(drip, 5)
sus string_channel dm<tea> = dm_make(tea, 3)

// Test send/receive operations
dm_send(int_channel, 42)
dm_send(int_channel, 43)
dm_send(string_channel, "hello")

sus received_int drip = dm_recv(int_channel)
sus received_string tea = dm_recv(string_channel)

assert_eq_int(received_int, 42)
assert_eq_string(received_string, "hello")

// Test buffered channel behavior
test_start("buffered channels")

sus buffered_ch dm<drip> = dm_make(drip, 2)

// Should be able to send without blocking
dm_send(buffered_ch, 100)
dm_send(buffered_ch, 200)

// Third send should succeed since buffer has capacity
dm_send(buffered_ch, 300)

sus val1 drip = dm_recv(buffered_ch)
sus val2 drip = dm_recv(buffered_ch)
sus val3 drip = dm_recv(buffered_ch)

assert_eq_int(val1, 100)
assert_eq_int(val2, 200)
assert_eq_int(val3, 300)

// Test goroutine communication
test_start("goroutine communication")

sus comm_channel dm<drip> = dm_make(drip, 1)
sus result drip = 0

// Sender goroutine
stan {
    dm_send(comm_channel, 999)
}

// Receiver goroutine
stan {
    result = dm_recv(comm_channel)
}

// Wait for communication to complete
sus comm_wait drip = 0
bestie (comm_wait < 100000 fam result == 0) {
    comm_wait = comm_wait + 1
}

assert_eq_int(result, 999)

// Test select-like operations
test_start("select operations")

sus ch1 dm<drip> = dm_make(drip, 1)
sus ch2 dm<tea> = dm_make(tea, 1)
sus selected_case drip = 0

// Fill channels
dm_send(ch1, 123)
dm_send(ch2, "world")

// Test ready statement (select-like)
ready {
    dm_recv(ch1) -> {
        selected_case = 1
        vibez.spill("Selected channel 1")
    }
    dm_recv(ch2) -> {
        selected_case = 2
        vibez.spill("Selected channel 2")
    }
}

assert_true(selected_case > 0)

// Test multiple goroutines
test_start("multiple goroutines")

sus shared_counter drip = 0
sus num_goroutines drip = 5

// Launch multiple goroutines
sus i drip = 0
bestie (i < num_goroutines) {
    stan {
        shared_counter = shared_counter + 1
    }
    i = i + 1
}

// Wait for all goroutines to complete
sus final_wait drip = 0
bestie (final_wait < 100000 fam shared_counter < num_goroutines) {
    final_wait = final_wait + 1
}

assert_eq_int(shared_counter, num_goroutines)

// Test concurrent producer-consumer
test_start("producer-consumer")

sus producer_channel dm<drip> = dm_make(drip, 10)
sus produced_sum drip = 0
sus consumed_sum drip = 0

// Producer goroutine
stan {
    sus p_val drip = 1
    bestie (p_val <= 10) {
        dm_send(producer_channel, p_val)
        produced_sum = produced_sum + p_val
        p_val = p_val + 1
    }
}

// Consumer goroutine  
stan {
    sus c_val drip = 0
    sus c_count drip = 0
    bestie (c_count < 10) {
        c_val = dm_recv(producer_channel)
        consumed_sum = consumed_sum + c_val
        c_count = c_count + 1
    }
}

// Wait for completion
sus pc_wait drip = 0
bestie (pc_wait < 100000 fam consumed_sum != produced_sum) {
    pc_wait = pc_wait + 1
}

assert_eq_int(produced_sum, 55) // Sum of 1+2+...+10
assert_eq_int(consumed_sum, produced_sum)

print_test_summary()

vibez.spill("All concurrency tests completed successfully!")
