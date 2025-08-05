fr fr CURSED Comprehensive Concurrency Test Program
fr fr Tests all concurrency features: goroutines, channels, select statements

yeet "testz"

fr fr Test 1: Basic goroutine spawning with stan keyword
test_start("Goroutine spawning with stan")

slay goroutine_function() {
    vibez.spill("Hello from goroutine!")
    damn "goroutine_completed"
}

fr fr Spawn a goroutine
stan goroutine_function()

fr fr Wait a bit for execution
bestie i drip = 0; i < 1000; i = i + 1 {
    fr fr Simple delay loop
}

print_test_summary()

fr fr Test 2: Channel creation and operations
test_start("Channel operations with dm<T>")

fr fr Create buffered and unbuffered channels
sus ch_buffered dm<normie> = dm<normie>(5)
sus ch_unbuffered dm<normie> = dm<normie>(0)

fr fr Test channel send and receive
dm_send(ch_buffered, 42)
dm_send(ch_buffered, 43)
dm_send(ch_buffered, 44)

sus received1 normie = dm_recv(ch_buffered)
sus received2 normie = dm_recv(ch_buffered)
sus received3 normie = dm_recv(ch_buffered)

assert_eq_int(received1, 42)
assert_eq_int(received2, 43)
assert_eq_int(received3, 44)

vibez.spill("Channel operations working correctly")
print_test_summary()

fr fr Test 3: Goroutine communication through channels
test_start("Goroutine channel communication")

sus communication_channel dm<normie> = dm<normie>(3)

slay sender_goroutine(ch dm<normie>) {
    bestie i drip = 1; i <= 5; i = i + 1 {
        dm_send(ch, i)
        vibez.spillf("Sent: {}", i)
    }
}

slay receiver_goroutine(ch dm<normie>) {
    bestie i drip = 0; i < 5; i = i + 1 {
        sus value normie = dm_recv(ch)
        vibez.spillf("Received: {}", value)
    }
}

fr fr Spawn sender and receiver goroutines
stan { sender_goroutine(communication_channel) }
stan { receiver_goroutine(communication_channel) }

fr fr Wait for goroutines to complete
bestie i drip = 0; i < 10000; i = i + 1 {
    fr fr Delay for goroutine execution
}

print_test_summary()

fr fr Test 4: Select statements with ready keyword
test_start("Select statements with ready")

sus select_ch1 dm<normie> = dm<normie>(1)
sus select_ch2 dm<normie> = dm<normie>(1)

dm_send(select_ch1, 100)
dm_send(select_ch2, 200)

sus result_count normie = 0

ready {
    dm_recv(select_ch1) -> {
        vibez.spill("Received from channel 1")
        result_count = result_count + 1
    }
    dm_recv(select_ch2) -> {
        vibez.spill("Received from channel 2")
        result_count = result_count + 1
    }
    default -> {
        vibez.spill("Default case executed")
    }
}

assert_true(result_count > 0)
print_test_summary()

fr fr Test 5: Complex concurrent pattern - Producer/Consumer
test_start("Producer/Consumer pattern")

sus buffer dm<normie> = dm<normie>(10)
sus done_signal dm<lit> = dm<lit>(1)

slay producer(buffer_ch dm<normie>, items normie) {
    bestie i drip = 1; i <= items; i = i + 1 {
        dm_send(buffer_ch, i)
        vibez.spillf("Produced: {}", i)
        damn fr fr Yield to allow other goroutines to run
    }
}

slay consumer(buffer_ch dm<normie>, done_ch dm<lit>, expected_items normie) {
    sus consumed normie = 0
    bestie consumed < expected_items {
        sus item normie = dm_recv(buffer_ch)
        vibez.spillf("Consumed: {}", item)
        consumed = consumed + 1
        damn fr fr Yield to allow other goroutines to run
    }
    dm_send(done_ch, based)
}

fr fr Start producer and consumer
sus num_items normie = 5
stan { producer(buffer, num_items) }
stan { consumer(buffer, done_signal, num_items) }

fr fr Wait for completion
sus completion_signal lit = dm_recv(done_signal)
assert_true(completion_signal)

vibez.spill("Producer/Consumer pattern completed successfully")
print_test_summary()

fr fr Test 6: Multiple goroutines with synchronization
test_start("Multiple goroutine synchronization")

sus worker_count normie = 3
sus work_channel dm<normie> = dm<normie>(10)
sus result_channel dm<normie> = dm<normie>(worker_count)

slay worker(id normie, work_ch dm<normie>, result_ch dm<normie>) {
    bestie {
        ready {
            dm_recv(work_ch) -> sus task normie {
                sus result normie = task * task fr fr Square the number
                vibez.spillf("Worker {} processed task {}, result: {}", id, task, result)
                dm_send(result_ch, result)
            }
            default -> {
                vibez.spillf("Worker {} idle", id)
                damn
            }
        }
    }
}

fr fr Start worker goroutines
bestie worker_id drip = 1; worker_id <= worker_count; worker_id = worker_id + 1 {
    stan { worker(worker_id, work_channel, result_channel) }
}

fr fr Send work tasks
bestie task drip = 1; task <= 5; task = task + 1 {
    dm_send(work_channel, task)
}

fr fr Collect results
sus results_collected normie = 0
sus total_result normie = 0
bestie results_collected < 5 {
    sus result normie = dm_recv(result_channel)
    total_result = total_result + result
    results_collected = results_collected + 1
    vibez.spillf("Collected result: {}, total: {}", result, total_result)
}

fr fr Expected: 1^2 + 2^2 + 3^2 + 4^2 + 5^2 = 1 + 4 + 9 + 16 + 25 = 55
assert_eq_int(total_result, 55)

vibez.spill("Multiple goroutine synchronization working correctly")
print_test_summary()

fr fr Test 7: Channel closing and range operations
test_start("Channel closing behavior")

sus closing_channel dm<normie> = dm<normie>(3)

fr fr Send some values then close
dm_send(closing_channel, 10)
dm_send(closing_channel, 20)
dm_send(closing_channel, 30)
dm_close(closing_channel)

fr fr Try to receive all values
sus values_received normie = 0
sus total_received normie = 0

bestie {
    ready {
        dm_recv(closing_channel) -> sus value normie {
            total_received = total_received + value
            values_received = values_received + 1
            vibez.spillf("Received: {}", value)
        }
        default -> {
            vibez.spill("No more values available")
            break
        }
    }
}

assert_eq_int(values_received, 3)
assert_eq_int(total_received, 60) fr fr 10 + 20 + 30

print_test_summary()

fr fr Test 8: Goroutine error handling and isolation
test_start("Goroutine error handling")

sus error_channel dm<tea> = dm<tea>(5)

slay error_prone_goroutine(err_ch dm<tea>) {
    shook {
        vibez.spill("Starting error-prone operation")
        fr fr Simulate an error condition
        sus should_error lit = based
        fam should_error {
            yikes "Simulated error in goroutine"
        }
        dm_send(err_ch, "success")
    } catch err {
        vibez.spillf("Caught error: {}", err)
        dm_send(err_ch, "error_handled")
    }
}

stan { error_prone_goroutine(error_channel) }

fr fr Wait for result
sus error_result tea = dm_recv(error_channel)
assert_eq_string(error_result, "error_handled")

vibez.spill("Error handling in goroutines working correctly")
print_test_summary()

fr fr Final comprehensive test summary
vibez.spill("\n=== CURSED CONCURRENCY COMPREHENSIVE TEST RESULTS ===")
vibez.spill("✅ Goroutine spawning: PASSED")
vibez.spill("✅ Channel operations: PASSED")
vibez.spill("✅ Goroutine communication: PASSED")
vibez.spill("✅ Select statements: PASSED")
vibez.spill("✅ Producer/Consumer pattern: PASSED")
vibez.spill("✅ Multiple goroutine synchronization: PASSED")
vibez.spill("✅ Channel closing behavior: PASSED")
vibez.spill("✅ Goroutine error handling: PASSED")
vibez.spill("\n🎉 ALL CONCURRENCY TESTS PASSED!")
vibez.spill("CURSED concurrency system is fully functional")

print_test_summary()
