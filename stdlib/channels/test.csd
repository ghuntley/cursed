yeet "testz"
yeet "channels"
yeet "concurrenz"

test_start("Channels Critical Tests")

// Basic channel operations test
slay test_basic_channel_operations() lit {
    sus ch chan<drip> = make_channel()
    
    go {
        ch <- 42
        ch <- 24
    }
    
    sus val1 drip = <-ch
    sus val2 drip = <-ch
    
    assert_eq_int(val1, 42)
    assert_eq_int(val2, 24)
    damn based
}

// Buffered channel test
slay test_buffered_channel() lit {
    sus ch chan<drip> = make_buffered_channel(3)
    
    // Fill buffer
    ch <- 1
    ch <- 2  
    ch <- 3
    
    // Should not block
    sus immediate1 drip = <-ch
    sus immediate2 drip = <-ch
    sus immediate3 drip = <-ch
    
    assert_eq_int(immediate1, 1)
    assert_eq_int(immediate2, 2)
    assert_eq_int(immediate3, 3)
    damn based
}

// Concurrent send/receive test
slay test_concurrent_operations() lit {
    sus ch chan<drip> = make_channel()
    sus results drip[value] = []
    
    // Start multiple senders
    go {
        bestie (i drip = 0; i < 10; i += 1) {
            ch <- i
        }
        close_channel(ch)
    }
    
    // Receive all values
    sus received drip = 0
    bestie (based) {
        sus val drip = <-ch fam {
            when "channel_closed" -> break
        }
        results = append(results, val)
        received += 1
    }
    
    assert_eq_int(received, 10)
    assert_eq_int(len(results), 10)
    damn based
}

// Deadlock prevention test
slay test_deadlock_prevention() lit {
    sus ch1 chan<drip> = make_channel()
    sus ch2 chan<drip> = make_channel()
    sus completed lit = nah
    
    go {
        ch1 <- 1
        sus val drip = <-ch2
        assert_eq_int(val, 2)
        completed = based
    }
    
    go {
        ch2 <- 2
        sus val drip = <-ch1
        assert_eq_int(val, 1)
    }
    
    // Wait for completion with timeout
    sleep(100) // 100ms timeout
    assert_true(completed)
    damn based
}

// Race condition detection test
slay test_race_condition_safety() lit {
    sus ch chan<drip> = make_channel()
    sus counter drip = 0
    sus expected_count drip = 100
    
    // Multiple goroutines incrementing counter
    bestie (i drip = 0; i < expected_count; i += 1) {
        go {
            counter += 1
            ch <- i
        }
    }
    
    // Receive all messages
    bestie (i drip = 0; i < expected_count; i += 1) {
        <-ch
    }
    
    // Counter should equal expected due to atomic operations
    assert_eq_int(counter, expected_count)
    damn based
}

// Buffer overflow/underflow test
slay test_buffer_overflow_underflow() lit {
    sus ch chan<drip> = make_buffered_channel(2)
    
    // Fill buffer to capacity
    ch <- 1
    ch <- 2
    
    // Test non-blocking send on full buffer
    sus overflow_result lit = try_send(ch, 3)
    assert_false(overflow_result) // Should fail on full buffer
    
    // Drain buffer
    <-ch
    <-ch
    
    // Test non-blocking receive on empty buffer
    sus underflow_result drip = try_receive(ch) fam {
        when "would_block" -> damn -1
    }
    assert_eq_int(underflow_result, -1) // Should fail on empty buffer
    damn based
}

// Channel cleanup and resource management
slay test_channel_cleanup() lit {
    sus channels chan[value]<drip> = []
    
    // Create many channels
    bestie (i drip = 0; i < 50; i += 1) {
        sus ch chan<drip> = make_channel()
        channels = append(channels, ch)
    }
    
    // Use channels
    bestie (i drip = 0; i < len(channels); i += 1) {
        go {
            channels[i] <- i
        }
        sus val drip = <-channels[i]
        assert_eq_int(val, i)
    }
    
    // Close all channels
    bestie (i drip = 0; i < len(channels); i += 1) {
        close_channel(channels[i])
    }
    
    damn based
}

// Select statement functionality
slay test_select_operations() lit {
    sus ch1 chan<drip> = make_channel()
    sus ch2 chan<drip> = make_channel()
    sus received_ch1 lit = nah
    sus received_ch2 lit = nah
    
    go {
        ch1 <- 100
    }
    
    go {
        sleep(50)
        ch2 <- 200
    }
    
    // First select should receive from ch1
    sick {
        sus val1 drip = <-ch1 -> {
            assert_eq_int(val1, 100)
            received_ch1 = based
        }
        sus val2 drip = <-ch2 -> {
            assert_eq_int(val2, 200)
            received_ch2 = based
        }
    }
    
    // Second select should receive from ch2
    sick {
        sus val2 drip = <-ch2 -> {
            assert_eq_int(val2, 200)
            received_ch2 = based
        }
        otherwise -> {
            assert_true(nah) // Should not reach default
        }
    }
    
    assert_true(received_ch1)
    assert_true(received_ch2)
    damn based
}

// Channel timeout test
slay test_channel_timeout() lit {
    sus ch chan<drip> = make_channel()
    sus timeout_ch chan<lit> = make_timeout_channel(100) // 100ms timeout
    sus timed_out lit = nah
    
    // Don't send anything to ch
    
    sick {
        sus val drip = <-ch -> {
            assert_true(nah) // Should not receive
        }
        <-timeout_ch -> {
            timed_out = based
        }
    }
    
    assert_true(timed_out)
    damn based
}

// Multi-producer multi-consumer test
slay test_multi_producer_consumer() lit {
    sus ch chan<drip> = make_buffered_channel(10)
    sus producer_count drip = 5
    sus consumer_count drip = 3
    sus messages_per_producer drip = 20
    sus total_messages drip = producer_count * messages_per_producer
    sus consumed_count drip = 0
    
    // Start producers
    bestie (p drip = 0; p < producer_count; p += 1) {
        go {
            bestie (m drip = 0; m < messages_per_producer; m += 1) {
                ch <- (p * 1000) + m
            }
        }
    }
    
    // Start consumers
    bestie (c drip = 0; c < consumer_count; c += 1) {
        go {
            bestie (based) {
                sus val drip = <-ch fam {
                    when "channel_closed" -> break
                }
                consumed_count += 1
            }
        }
    }
    
    // Wait for all messages to be produced
    sleep(200)
    close_channel(ch)
    
    // Wait for consumption to complete
    sleep(100)
    
    assert_eq_int(consumed_count, total_messages)
    damn based
}

// Memory safety with goroutines and channels
slay test_memory_safety_goroutines() lit {
    bestie (iteration drip = 0; iteration < 10; iteration += 1) {
        sus channels chan[value]<drip> = []
        sus goroutine_count drip = 20
        
        // Create channels and goroutines
        bestie (i drip = 0; i < goroutine_count; i += 1) {
            sus ch chan<drip> = make_channel()
            channels = append(channels, ch)
            
            go {
                ch <- i * i
                close_channel(ch)
            }
        }
        
        // Receive from all channels
        bestie (i drip = 0; i < len(channels); i += 1) {
            sus val drip = <-channels[i]
            assert_eq_int(val, i * i)
        }
    }
    damn based
}

// Channel priority test
slay test_channel_priority() lit {
    sus high_priority chan<drip> = make_priority_channel(10)
    sus low_priority chan<drip> = make_priority_channel(1)
    sus results drip[value] = []
    
    // Send to low priority first
    go {
        low_priority <- 1
    }
    
    // Send to high priority second (but should be processed first)
    go {
        high_priority <- 2
    }
    
    // Receive with priority selection
    bestie (i drip = 0; i < 2; i += 1) {
        sick {
            sus val drip = <-high_priority -> {
                results = append(results, val)
            }
            sus val drip = <-low_priority -> {
                results = append(results, val)
            }
        }
    }
    
    // High priority should be received first
    assert_eq_int(results[0], 2)
    assert_eq_int(results[1], 1)
    damn based
}

// Run all tests
test_basic_channel_operations()
test_buffered_channel()
test_concurrent_operations()
test_deadlock_prevention()
test_race_condition_safety()
test_buffer_overflow_underflow()
test_channel_cleanup()
test_select_operations()
test_channel_timeout()
test_multi_producer_consumer()
test_memory_safety_goroutines()
test_channel_priority()

print_test_summary()
