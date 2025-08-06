# Advanced Select Statement Test for CURSED Compiler
# Tests comprehensive select statement functionality with LLVM codegen

yeet "testz"
yeet "concurrenz"

slay test_basic_select_compilation() {
    test_start("Basic Select Compilation")
    
    sus ch1 dm<normie> = dm<normie>(1)
    sus ch2 dm<normie> = dm<normie>(1)
    
    # Send values to test channels
    dm_send(ch1, 42)
    dm_send(ch2, 84)
    
    sus result normie = 0
    sus source normie = 0
    
    # Basic select statement with mixed send/receive
    ready {
        mood val := dm_recv(ch1): {
            result = val
            source = 1
            vibez.spill("Received from ch1: ", val)
        }
        mood val := dm_recv(ch2): {
            result = val
            source = 2
            vibez.spill("Received from ch2: ", val)
        }
        basic: {
            result = -1
            vibez.spill("Default case executed")
        }
    }
    
    assert_true(result > 0)
    assert_true(source > 0)
    vibez.spill("Result: ", result, " Source: ", source)
    
    print_test_summary()
}

slay test_non_blocking_select() {
    test_start("Non-blocking Select")
    
    sus ch dm<normie> = dm<normie>(0)  # Unbuffered channel
    sus executed_default lit = cringe
    
    ready {
        mood val := dm_recv(ch): {
            vibez.spill("Should not execute - channel empty")
        }
        basic: {
            executed_default = based
            vibez.spill("Default case - channel not ready")
        }
    }
    
    assert_true(executed_default)
    print_test_summary()
}

slay test_select_with_send_operations() {
    test_start("Select with Send Operations")
    
    sus send_ch dm<normie> = dm<normie>(1)  # Buffered channel
    sus recv_ch dm<normie> = dm<normie>(0)  # Unbuffered channel
    
    sus operation tea = ""
    
    ready {
        mood dm_send(send_ch, 123): {
            operation = "sent"
            vibez.spill("Successfully sent to buffered channel")
        }
        mood dm_send(recv_ch, 456): {
            operation = "sent_unbuffered"
            vibez.spill("Sent to unbuffered channel (should not happen)")
        }
        basic: {
            operation = "default"
            vibez.spill("No channels ready for send")
        }
    }
    
    assert_eq_string(operation, "sent")
    print_test_summary()
}

slay test_select_with_goroutines() {
    test_start("Select with Goroutines")
    
    sus ch dm<normie> = dm<normie>(0)
    sus received normie = 0
    
    # Start goroutine that will send after a brief delay
    stan {
        yolo()  # Yield to ensure select starts first
        dm_send(ch, 999)
        vibez.spill("Goroutine sent value")
    }
    
    ready {
        mood val := dm_recv(ch): {
            received = val
            vibez.spill("Received from goroutine: ", val)
        }
        basic: {
            vibez.spill("No value received from goroutine")
        }
    }
    
    # Give goroutine time to execute
    yolo()
    
    # Try again if first select hit default
    vibes received == 0 {
        ready {
            mood val := dm_recv(ch): {
                received = val
                vibez.spill("Received on retry: ", val)
            }
            basic: {
                vibez.spill("Still no value available")
            }
        }
    }
    
    vibez.spill("Final received value: ", received)
    print_test_summary()
}

slay test_select_multiple_ready_channels() {
    test_start("Select Multiple Ready Channels")
    
    sus ch1 dm<normie> = dm<normie>(1)
    sus ch2 dm<normie> = dm<normie>(1) 
    sus ch3 dm<normie> = dm<normie>(1)
    
    # Fill all channels
    dm_send(ch1, 111)
    dm_send(ch2, 222)
    dm_send(ch3, 333)
    
    sus counts [3]normie = [0, 0, 0]
    
    # Run select multiple times to test fairness
    bestie i drip = 0; i < 6; i++ {
        ready {
            mood val := dm_recv(ch1): {
                counts[0]++
                dm_send(ch1, val)  # Put back for next iteration
            }
            mood val := dm_recv(ch2): {
                counts[1]++
                dm_send(ch2, val)  # Put back for next iteration
            }
            mood val := dm_recv(ch3): {
                counts[2]++
                dm_send(ch3, val)  # Put back for next iteration
            }
        }
    }
    
    vibez.spill("Channel selection counts: ", counts[0], " ", counts[1], " ", counts[2])
    
    # All channels should be selected at least once for good fairness
    assert_true(counts[0] > 0)
    assert_true(counts[1] > 0)
    assert_true(counts[2] > 0)
    
    print_test_summary()
}

slay test_select_variable_binding() {
    test_start("Select Variable Binding")
    
    sus ch dm<normie> = dm<normie>(1)
    dm_send(ch, 777)
    
    sus bound_value normie = 0
    sus binding_worked lit = cringe
    
    ready {
        mood received_val := dm_recv(ch): {
            bound_value = received_val
            binding_worked = based
            vibez.spill("Bound variable value: ", received_val)
        }
        basic: {
            vibez.spill("Channel not ready")
        }
    }
    
    assert_true(binding_worked)
    assert_eq_int(bound_value, 777)
    
    print_test_summary()
}

slay test_select_error_handling() {
    test_start("Select Error Handling")
    
    sus ch dm<normie> = dm<normie>(1)
    
    # Close channel after putting a value
    dm_send(ch, 555)
    dm_close(ch)
    
    sus received_from_closed lit = cringe
    sus value_from_closed normie = 0
    
    ready {
        mood val := dm_recv(ch): {
            received_from_closed = based
            value_from_closed = val
            vibez.spill("Received from closed channel: ", val)
        }
        basic: {
            vibez.spill("Closed channel not ready")
        }
    }
    
    assert_true(received_from_closed)
    assert_eq_int(value_from_closed, 555)
    
    # Try to receive again - should get zero value
    sus second_receive normie = 0
    sus got_zero_value lit = cringe
    
    ready {
        mood val := dm_recv(ch): {
            second_receive = val
            vibes val == 0 {
                got_zero_value = based
            }
            vibez.spill("Second receive: ", val)
        }
        basic: {
            vibez.spill("No second receive possible")
        }
    }
    
    assert_eq_int(second_receive, 0)
    assert_true(got_zero_value)
    
    print_test_summary()
}

slay test_complex_select_workflow() {
    test_start("Complex Select Workflow")
    
    sus work_ch dm<normie> = dm<normie>(2)
    sus result_ch dm<normie> = dm<normie>(2)
    sus control_ch dm<normie> = dm<normie>(1)
    
    # Worker goroutine
    stan {
        bestie i drip = 0; i < 3; i++ {
            ready {
                mood work := dm_recv(work_ch): {
                    sus result normie = work * 2  # Double the work
                    dm_send(result_ch, result)
                    vibez.spill("Processed work: ", work, " -> ", result)
                }
                mood dm_recv(control_ch): {
                    vibez.spill("Worker stopping")
                    damn  # Exit goroutine
                }
            }
        }
    }
    
    # Send work items
    dm_send(work_ch, 10)
    dm_send(work_ch, 20)
    
    sus results [2]normie = [0, 0]
    sus result_count normie = 0
    
    # Collect results
    bestie result_count < 2 {
        ready {
            mood result := dm_recv(result_ch): {
                results[result_count] = result
                result_count++
                vibez.spill("Got result: ", result)
            }
            basic: {
                yolo()  # Yield to let worker process
            }
        }
    }
    
    # Signal worker to stop
    dm_send(control_ch, 1)
    
    # Verify results
    assert_eq_int(result_count, 2)
    assert_eq_int(results[0], 20)  # 10 * 2
    assert_eq_int(results[1], 40)  # 20 * 2
    
    vibez.spill("All work completed successfully")
    print_test_summary()
}

slay main() {
    vibez.spill("=== Advanced Select Statement Tests ===")
    
    test_basic_select_compilation()
    test_non_blocking_select()
    test_select_with_send_operations()
    test_select_with_goroutines()
    test_select_multiple_ready_channels()
    test_select_variable_binding()
    test_select_error_handling()
    test_complex_select_workflow()
    
    vibez.spill("=== All Advanced Select Tests Complete ===")
}
