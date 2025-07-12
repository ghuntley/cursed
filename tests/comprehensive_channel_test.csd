vibe main

yeet "testz"

// Test basic channel operations
slay test_basic_channel_operations() {
    test_start("Basic channel operations")
    
    // Create an unbuffered channel
    sus (sender, receiver) := channel()
    
    // Test send and receive
    yolo {
        sender.send(42)
    }
    
    sus value := receiver.recv()
    assert_eq_int(value, 42)
    
    print_test_summary()
}

// Test buffered channel operations
slay test_buffered_channel_operations() {
    test_start("Buffered channel operations")
    
    // Create a buffered channel with capacity 3
    sus (sender, receiver) := buffered_channel(3)
    
    // Send multiple values without blocking
    sender.send(1)
    sender.send(2)
    sender.send(3)
    
    // Receive all values
    sus val1 := receiver.recv()
    sus val2 := receiver.recv()
    sus val3 := receiver.recv()
    
    assert_eq_int(val1, 1)
    assert_eq_int(val2, 2)
    assert_eq_int(val3, 3)
    
    print_test_summary()
}

// Test channel closing
slay test_channel_closing() {
    test_start("Channel closing")
    
    sus (sender, receiver) := channel()
    
    // Send a value then close
    sender.send(100)
    sender.close()
    
    // Should still receive the buffered value
    sus value := receiver.recv()
    assert_eq_int(value, 100)
    
    // Next receive should indicate closed
    sus result := receiver.try_recv()
    assert_true(result.is_closed())
    
    print_test_summary()
}

// Test select statement with multiple channels
slay test_select_statement() {
    test_start("Select statement")
    
    sus (sender1, receiver1) := buffered_channel(1)
    sus (sender2, receiver2) := buffered_channel(1)
    
    // Send to channel 1
    sender1.send(42)
    
    // Use select to receive from either channel
    ready {
        case value := <-receiver1 -> {
            assert_eq_int(value, 42)
        }
        case value := <-receiver2 -> {
            assert_true(cap) // Should not reach here
        }
        default -> {
            assert_true(cap) // Should not reach here
        }
    }
    
    print_test_summary()
}

// Test channel buffering and overflow
slay test_channel_buffering() {
    test_start("Channel buffering and overflow")
    
    sus (sender, receiver) := buffered_channel(2)
    
    // Fill the buffer
    sender.send(1)
    sender.send(2)
    
    // Next send should block or fail with try_send
    sus result := sender.try_send(3)
    assert_true(result.would_block())
    
    // Receive one value to make space
    sus value := receiver.recv()
    assert_eq_int(value, 1)
    
    // Now should be able to send
    sus result2 := sender.try_send(3)
    assert_true(result2.is_ok())
    
    print_test_summary()
}

// Test various channel scenarios
slay test_channel_scenarios() {
    test_start("Various channel scenarios")
    
    // Test channel with timeout
    sus (sender, receiver) := channel()
    
    // Should timeout when no data available
    sus result := receiver.recv_timeout(Duration::from_millis(10))
    assert_true(result.would_block())
    
    // Test channel iteration
    sus (sender2, receiver2) := buffered_channel(3)
    sender2.send(1)
    sender2.send(2)
    sender2.send(3)
    sender2.close()
    
    sus total := 0
    bestie value := range receiver2 {
        total += value
    }
    assert_eq_int(total, 6)
    
    print_test_summary()
}

// Test concurrent channel operations
slay test_concurrent_channel_operations() {
    test_start("Concurrent channel operations")
    
    sus (sender, receiver) := buffered_channel(10)
    
    // Spawn multiple senders
    bestie i := 0; i < 5; i++ {
        yolo {
            sender.send(i)
        }
    }
    
    // Receive all values
    sus received := 0
    bestie i := 0; i < 5; i++ {
        sus value := receiver.recv()
        received += value
    }
    
    // Should have received sum of 0+1+2+3+4 = 10
    assert_eq_int(received, 10)
    
    print_test_summary()
}

// Test channel statistics and monitoring
slay test_channel_statistics() {
    test_start("Channel statistics")
    
    sus (sender, receiver) := buffered_channel(5)
    
    // Send some values
    sender.send(1)
    sender.send(2)
    
    // Check statistics
    sus stats := sender.channel().stats()
    assert_eq_int(stats.capacity, 5)
    assert_eq_int(stats.current_length, 2)
    assert_true(stats.is_buffered())
    
    print_test_summary()
}

// Main test runner
slay main() {
    test_basic_channel_operations()
    test_buffered_channel_operations()
    test_channel_closing()
    test_select_statement()
    test_channel_buffering()
    test_channel_scenarios()
    test_concurrent_channel_operations()
    test_channel_statistics()
    
    vibez.spill("All comprehensive channel tests completed!")
}
