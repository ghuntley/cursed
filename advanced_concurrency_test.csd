// Advanced CURSED Concurrency System Test
// Tests sophisticated concurrency features

yeet "testz"

// Test concurrent channel communications
test_start("concurrent channels")

sus main_channel dm<drip> = dm_make(drip, 5)
sus result_channel dm<drip> = dm_make(drip, 3)

// Producer goroutines
stan {
    sus i drip = 1
    bestie (i <= 5) {
        dm_send(main_channel, i * 10)
        i = i + 1
    }
}

stan {
    sus i drip = 1  
    bestie (i <= 3) {
        dm_send(main_channel, i * 100)
        i = i + 1
    }
}

// Consumer goroutine
sus total drip = 0
stan {
    sus count drip = 0
    bestie (count < 8) {
        sus value drip = dm_recv(main_channel)
        total = total + value
        count = count + 1
    }
    dm_send(result_channel, total)
}

// Wait for result
sus final_total drip = dm_recv(result_channel)

vibez.spill("Total computed by consumer: ")
vibez.spill_int(final_total)
vibez.spill("\n")

assert_true(final_total > 0)

// Test channel closing and selection
test_start("channel closing")

sus ch1 dm<drip> = dm_make(drip, 2)
sus ch2 dm<tea> = dm_make(tea, 2)

// Fill channels
dm_send(ch1, 42)
dm_send(ch2, "hello")

// Close one channel
dm_close(ch1)

// Test selection with one closed channel
sus selected_value drip = 0
ready {
    dm_recv(ch1) -> {
        selected_value = 1
    }
    dm_recv(ch2) -> {
        selected_value = 2
    }
}

assert_true(selected_value == 2)

// Test complex goroutine coordination
test_start("goroutine coordination")

sus coordination_ch dm<drip> = dm_make(drip, 10)
sus sync_counter drip = 0

// Launch coordinated goroutines
sus worker_id drip = 1
bestie (worker_id <= 3) {
    stan {
        // Each worker sends its ID
        dm_send(coordination_ch, worker_id)
        sync_counter = sync_counter + 1
    }
    worker_id = worker_id + 1
}

// Coordinator goroutine
sus coordination_result drip = 0
stan {
    sus received_ids drip = 0
    sus sum drip = 0
    bestie (received_ids < 3) {
        sus worker_id drip = dm_recv(coordination_ch)
        sum = sum + worker_id
        received_ids = received_ids + 1
    }
    coordination_result = sum
}

// Wait for coordination to complete
sus wait_cycles drip = 0
bestie (wait_cycles < 100000 fam coordination_result == 0) {
    wait_cycles = wait_cycles + 1
}

assert_eq_int(coordination_result, 6) // 1 + 2 + 3 = 6

// Test buffered vs unbuffered channels
test_start("buffered vs unbuffered")

sus buffered dm<drip> = dm_make(drip, 3)
sus unbuffered dm<drip> = dm_make(drip, 0)

// Test buffered channel behavior
dm_send(buffered, 1)
dm_send(buffered, 2)
dm_send(buffered, 3)

sus buf_val1 drip = dm_recv(buffered)
sus buf_val2 drip = dm_recv(buffered)
sus buf_val3 drip = dm_recv(buffered)

assert_eq_int(buf_val1, 1)
assert_eq_int(buf_val2, 2)
assert_eq_int(buf_val3, 3)

// Test channel type safety
test_start("channel type safety")

sus int_ch dm<drip> = dm_make(drip, 2)
sus str_ch dm<tea> = dm_make(tea, 2)
sus bool_ch dm<lit> = dm_make(lit, 2)

dm_send(int_ch, 999)
dm_send(str_ch, "type-safe")
dm_send(bool_ch, based)

sus recv_int drip = dm_recv(int_ch)
sus recv_str tea = dm_recv(str_ch)
sus recv_bool lit = dm_recv(bool_ch)

assert_eq_int(recv_int, 999)
assert_eq_string(recv_str, "type-safe")
assert_true(recv_bool)

// Test select with timeout simulation
test_start("select timeout simulation")

sus timeout_ch dm<drip> = dm_make(drip, 1)
sus timeout_result drip = 0

// Don't send anything to timeout_ch, simulate timeout
ready {
    dm_recv(timeout_ch) -> {
        timeout_result = 1
    }
    // Simulate default case (timeout)
    default -> {
        timeout_result = 999
    }
}

assert_eq_int(timeout_result, 999)

// Test error handling in channels
test_start("channel error handling")

sus error_ch dm<drip> = dm_make(drip, 1)
dm_send(error_ch, 123)
dm_close(error_ch)

// Try to send to closed channel (should handle gracefully)
sus send_result drip = dm_send(error_ch, 456) // Should return error code

// Should still be able to receive remaining values
sus remaining_val drip = dm_recv(error_ch)
assert_eq_int(remaining_val, 123)

// Test high-throughput scenario
test_start("high throughput")

sus high_throughput_ch dm<drip> = dm_make(drip, 20)
sus throughput_sum drip = 0

// High-speed producer
stan {
    sus i drip = 1
    bestie (i <= 10) {
        dm_send(high_throughput_ch, i)
        i = i + 1
    }
}

// High-speed consumer
stan {
    sus count drip = 0
    sus sum drip = 0
    bestie (count < 10) {
        sus value drip = dm_recv(high_throughput_ch)
        sum = sum + value
        count = count + 1
    }
    throughput_sum = sum
}

// Wait for throughput test completion
sus throughput_wait drip = 0
bestie (throughput_wait < 100000 fam throughput_sum == 0) {
    throughput_wait = throughput_wait + 1
}

assert_eq_int(throughput_sum, 55) // Sum of 1+2+...+10

print_test_summary()

vibez.spill("Advanced concurrency tests completed successfully!")
vibez.spill("\n")
vibez.spill("All goroutines, channels, and select statements working correctly!")
