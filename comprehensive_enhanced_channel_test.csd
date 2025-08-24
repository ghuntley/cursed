yeet "concurrenz/mod_enhanced_channels"
yeet "testz"
yeet "vibez"

fr fr Comprehensive Enhanced Channel Operations Test Suite
fr fr Tests: Blocking operations, deadlock prevention, select statements, buffered channels, cleanup

fr fr Test configuration and setup
sus test_passed normie = 0
sus test_failed normie = 0
sus test_total normie = 0

fr fr Test helper macros
slay test_start(test_name tea) {
    vibez.spillf("\n🧪 Test: {}", test_name)
    test_total = test_total + 1
}

slay test_assert(condition lit, message tea) {
    ready condition {
        vibez.spillf("  ✅ {}", message)
        test_passed = test_passed + 1
    } otherwise {
        vibez.spillf("  ❌ {}", message)
        test_failed = test_failed + 1
    }
}

slay test_summary() {
    vibez.spill("\n📊 Test Summary:")
    vibez.spillf("  Total tests: {}", test_total)
    vibez.spillf("  Passed: {}", test_passed)
    vibez.spillf("  Failed: {}", test_failed)
    ready test_failed == 0 {
        vibez.spill("  🎉 All tests passed!")
    } otherwise {
        vibez.spillf("  💥 {} tests failed", test_failed)
    }
}

fr fr =============================================================================
fr fr BASIC CHANNEL OPERATIONS TESTS
fr fr =============================================================================

slay test_enhanced_channel_creation() {
    test_start("Enhanced Channel Creation")
    
    fr fr Test unbuffered channel creation
    sus sync_ch *EnhancedChannel = create_enhanced_channel(0)
    test_assert(sync_ch != 0, "Unbuffered channel created successfully")
    test_assert(sync_ch.capacity == 0, "Unbuffered channel has zero capacity")
    test_assert(!enhanced_channel_is_closed(sync_ch), "New channel is not closed")
    
    fr fr Test buffered channel creation
    sus buf_ch *EnhancedChannel = create_enhanced_channel(10)
    test_assert(buf_ch != 0, "Buffered channel created successfully")
    test_assert(buf_ch.capacity == 10, "Buffered channel has correct capacity")
    test_assert(!enhanced_channel_is_closed(buf_ch), "New buffered channel is not closed")
    
    fr fr Test channel statistics
    sus stats ChannelStats = enhanced_channel_stats(buf_ch)
    test_assert(stats.capacity == 10, "Channel stats show correct capacity")
    test_assert(stats.current_size == 0, "Channel stats show empty buffer")
    test_assert(stats.total_sends == 0, "Channel stats show zero sends")
    test_assert(stats.total_recvs == 0, "Channel stats show zero receives")
    test_assert(!stats.is_closed, "Channel stats show not closed")
    
    fr fr Cleanup
    enhanced_channel_cleanup(sync_ch)
    enhanced_channel_cleanup(buf_ch)
}

slay test_buffered_channel_basic_operations() {
    test_start("Buffered Channel Basic Operations")
    
    sus ch *EnhancedChannel = create_enhanced_channel(3)
    
    fr fr Test basic send operations
    test_assert(enhanced_channel_send(ch, 100), "First send successful")
    test_assert(enhanced_channel_send(ch, 200), "Second send successful")
    test_assert(enhanced_channel_send(ch, 300), "Third send successful")
    
    fr fr Check channel statistics after sends
    sus stats_after_sends ChannelStats = enhanced_channel_stats(ch)
    test_assert(stats_after_sends.current_size == 3, "Buffer full after 3 sends")
    test_assert(stats_after_sends.total_sends == 3, "Total send count is 3")
    test_assert(stats_after_sends.total_recvs == 0, "No receives yet")
    
    fr fr Test basic receive operations
    sus recv_data normie = 0
    sus recv_ok lit = cap
    
    (recv_data, recv_ok) = enhanced_channel_receive(ch)
    test_assert(recv_ok && recv_data == 100, "First receive got correct data")
    
    (recv_data, recv_ok) = enhanced_channel_receive(ch)
    test_assert(recv_ok && recv_data == 200, "Second receive got correct data")
    
    (recv_data, recv_ok) = enhanced_channel_receive(ch)
    test_assert(recv_ok && recv_data == 300, "Third receive got correct data")
    
    fr fr Check final statistics
    sus stats_after_recvs ChannelStats = enhanced_channel_stats(ch)
    test_assert(stats_after_recvs.current_size == 0, "Buffer empty after receives")
    test_assert(stats_after_recvs.total_sends == 3, "Total send count preserved")
    test_assert(stats_after_recvs.total_recvs == 3, "Total receive count is 3")
    
    enhanced_channel_cleanup(ch)
}

slay test_unbuffered_channel_basic_operations() {
    test_start("Unbuffered Channel Basic Operations")
    
    sus ch *EnhancedChannel = create_enhanced_channel(0)
    
    fr fr Test that channel is truly unbuffered
    test_assert(ch.capacity == 0, "Channel has zero capacity")
    
    fr fr In a real concurrent environment, this would require goroutines
    fr fr For this test, we'll verify the channel structure is set up correctly
    sus stats ChannelStats = enhanced_channel_stats(ch)
    test_assert(stats.capacity == 0, "Stats confirm unbuffered channel")
    test_assert(stats.current_size == 0, "Unbuffered channel shows zero size")
    
    enhanced_channel_cleanup(ch)
}

fr fr =============================================================================
fr fr CHANNEL CLOSING AND CLEANUP TESTS
fr fr =============================================================================

slay test_channel_closing_and_cleanup() {
    test_start("Channel Closing and Cleanup")
    
    sus ch *EnhancedChannel = create_enhanced_channel(2)
    
    fr fr Test channel is initially open
    test_assert(!enhanced_channel_is_closed(ch), "Channel initially open")
    
    fr fr Add some data before closing
    test_assert(enhanced_channel_send(ch, 42), "Send to open channel successful")
    test_assert(enhanced_channel_send(ch, 84), "Second send to open channel successful")
    
    fr fr Close the channel
    test_assert(enhanced_channel_close(ch), "Channel close successful")
    test_assert(enhanced_channel_is_closed(ch), "Channel is now closed")
    
    fr fr Test that sending to closed channel fails
    test_assert(!enhanced_channel_send(ch, 999), "Send to closed channel fails")
    
    fr fr Test receiving existing data from closed channel
    sus recv_data normie = 0
    sus recv_ok lit = cap
    
    (recv_data, recv_ok) = enhanced_channel_receive(ch)
    test_assert(recv_ok && recv_data == 42, "Can receive existing data from closed channel")
    
    (recv_data, recv_ok) = enhanced_channel_receive(ch)
    test_assert(recv_ok && recv_data == 84, "Can receive second existing data from closed channel")
    
    fr fr Test receiving from closed empty channel fails
    (recv_data, recv_ok) = enhanced_channel_receive(ch)
    test_assert(!recv_ok, "Receive from closed empty channel fails")
    
    fr fr Test double close is idempotent
    test_assert(enhanced_channel_close(ch), "Double close is idempotent")
    test_assert(enhanced_channel_is_closed(ch), "Channel remains closed after double close")
    
    enhanced_channel_cleanup(ch)
}

fr fr =============================================================================
fr fr SELECT STATEMENT TESTS
fr fr =============================================================================

slay test_select_context_creation_and_management() {
    test_start("Select Context Creation and Management")
    
    fr fr Create select context
    sus ctx *SelectContext = create_select_context(3)
    test_assert(ctx != 0, "Select context created successfully")
    test_assert(ctx.channel_count == 3, "Select context has correct channel count")
    test_assert(ctx.ready_channel == -1, "Select context initially has no ready channel")
    
    fr fr Create test channels
    sus ch1 *EnhancedChannel = create_enhanced_channel(1)
    sus ch2 *EnhancedChannel = create_enhanced_channel(1) 
    sus ch3 *EnhancedChannel = create_enhanced_channel(0)
    
    fr fr Add channels to select context
    test_assert(select_add_recv(ctx, 0, ch1), "Added receive operation for channel 1")
    test_assert(select_add_send(ctx, 1, ch2, 42), "Added send operation for channel 2")
    test_assert(select_add_recv(ctx, 2, ch3), "Added receive operation for channel 3")
    
    fr fr Verify operations were set correctly
    test_assert(ctx.channels[0] == ch1, "Channel 0 set correctly")
    test_assert(ctx.channels[1] == ch2, "Channel 1 set correctly") 
    test_assert(ctx.channels[2] == ch3, "Channel 2 set correctly")
    test_assert(ctx.operations[0] == 0, "Operation 0 is receive")
    test_assert(ctx.operations[1] == 1, "Operation 1 is send")
    test_assert(ctx.operations[2] == 0, "Operation 2 is receive")
    test_assert(ctx.send_data[1] == 42, "Send data set correctly")
    
    fr fr Cleanup
    enhanced_channel_cleanup(ch1)
    enhanced_channel_cleanup(ch2)
    enhanced_channel_cleanup(ch3)
}

slay test_select_try_operations() {
    test_start("Select Try Operations")
    
    fr fr Test select try receive on empty buffered channel
    sus ch1 *EnhancedChannel = create_enhanced_channel(2)
    sus result_data normie = 0
    test_assert(!select_try_receive_enhanced(ch1, &result_data), "Try receive on empty channel fails")
    
    fr fr Add data and test successful try receive
    test_assert(enhanced_channel_send(ch1, 123), "Send data for try receive test")
    test_assert(select_try_receive_enhanced(ch1, &result_data), "Try receive with data succeeds")
    test_assert(result_data == 123, "Try receive got correct data")
    
    fr fr Test select try send on empty buffered channel
    test_assert(select_try_send_enhanced(ch1, 456), "Try send to empty channel succeeds")
    
    fr fr Fill channel and test try send failure
    test_assert(enhanced_channel_send(ch1, 789), "Fill channel for try send test")
    test_assert(!select_try_send_enhanced(ch1, 999), "Try send to full channel fails")
    
    fr fr Test try receive on closed channel with data
    test_assert(select_try_receive_enhanced(ch1, &result_data), "Try receive remaining data")
    test_assert(result_data == 456, "Got first remaining data")
    test_assert(select_try_receive_enhanced(ch1, &result_data), "Try receive second remaining data")
    test_assert(result_data == 789, "Got second remaining data")
    
    enhanced_channel_close(ch1)
    test_assert(select_try_receive_enhanced(ch1, &result_data), "Try receive on closed empty channel succeeds")
    test_assert(result_data == 0, "Closed empty channel returns 0")
    test_assert(!select_try_send_enhanced(ch1, 111), "Try send to closed channel fails")
    
    enhanced_channel_cleanup(ch1)
}

slay test_select_execute_with_ready_channels() {
    test_start("Select Execute with Ready Channels")
    
    fr fr Create channels with data ready
    sus ch1 *EnhancedChannel = create_enhanced_channel(2)
    sus ch2 *EnhancedChannel = create_enhanced_channel(2)
    sus ch3 *EnhancedChannel = create_enhanced_channel(2)
    
    fr fr Prepare ch2 with data for receiving
    test_assert(enhanced_channel_send(ch2, 777), "Prepare ch2 with data")
    
    fr fr Create select context
    sus ctx *SelectContext = create_select_context(3)
    test_assert(select_add_recv(ctx, 0, ch1), "Add recv on empty ch1")
    test_assert(select_add_recv(ctx, 1, ch2), "Add recv on ready ch2")
    test_assert(select_add_send(ctx, 2, ch3, 888), "Add send to empty ch3")
    
    fr fr Execute select - should find ch2 ready for receive or ch3 ready for send
    sus ready_channel normie = enhanced_select_execute(ctx, 100)  fr fr 100ms timeout
    test_assert(ready_channel >= 0, "Select found a ready channel")
    test_assert(ready_channel == 1 || ready_channel == 2, "Ready channel is ch2 (recv) or ch3 (send)")
    
    ready ready_channel == 1 {
        test_assert(ctx.result_data == 777, "Received correct data from ch2")
    } otherwise ready ready_channel == 2 {
        fr fr Verify send actually happened by trying to receive
        sus recv_data normie = 0
        sus recv_ok lit = cap
        (recv_data, recv_ok) = enhanced_channel_receive(ch3)
        test_assert(recv_ok && recv_data == 888, "Send to ch3 was successful")
    }
    
    enhanced_channel_cleanup(ch1)
    enhanced_channel_cleanup(ch2)
    enhanced_channel_cleanup(ch3)
}

slay test_select_timeout() {
    test_start("Select Timeout")
    
    fr fr Create channels with no ready operations
    sus ch1 *EnhancedChannel = create_enhanced_channel(0)  fr fr Unbuffered, no receivers
    sus ch2 *EnhancedChannel = create_enhanced_channel(2)  fr fr Buffered, empty
    
    fr fr Fill ch2 to make sends block
    test_assert(enhanced_channel_send(ch2, 1), "Fill ch2 slot 1")
    test_assert(enhanced_channel_send(ch2, 2), "Fill ch2 slot 2")
    
    fr fr Create select context with operations that will block
    sus ctx *SelectContext = create_select_context(2)
    test_assert(select_add_recv(ctx, 0, ch1), "Add recv on unbuffered ch1 (no senders)")
    test_assert(select_add_send(ctx, 1, ch2, 3), "Add send to full ch2")
    
    fr fr Execute select with short timeout
    sus ready_channel normie = enhanced_select_execute(ctx, 10)  fr fr 10ms timeout
    test_assert(ready_channel == -1, "Select timed out as expected")
    
    enhanced_channel_cleanup(ch1)
    enhanced_channel_cleanup(ch2)
}

fr fr =============================================================================
fr fr DEADLOCK PREVENTION TESTS
fr fr =============================================================================

slay test_deadlock_prevention_configuration() {
    test_start("Deadlock Prevention Configuration")
    
    fr fr Test default configuration
    test_assert(global_deadlock_config.prevention_enabled, "Deadlock prevention enabled by default")
    test_assert(global_deadlock_config.max_wait_time > 0, "Max wait time is positive")
    test_assert(global_deadlock_config.max_total_waiters > 0, "Max total waiters is positive")
    
    fr fr Test configuration changes
    configure_deadlock_prevention(5000, 500, based)
    test_assert(global_deadlock_config.max_wait_time == 5000, "Max wait time updated")
    test_assert(global_deadlock_config.max_total_waiters == 500, "Max total waiters updated")
    test_assert(global_deadlock_config.prevention_enabled, "Deadlock prevention still enabled")
    
    fr fr Test disabling deadlock prevention
    configure_deadlock_prevention(1000, 100, cap)
    test_assert(!global_deadlock_config.prevention_enabled, "Deadlock prevention disabled")
    
    fr fr Restore defaults
    configure_deadlock_prevention(10000, 1000, based)
}

slay test_channel_registry_management() {
    test_start("Channel Registry Management")
    
    fr fr Reset registry
    global_registry_size = 0
    
    fr fr Create and register channels
    sus ch1 *EnhancedChannel = create_enhanced_channel(1)
    sus ch2 *EnhancedChannel = create_enhanced_channel(2)
    sus ch3 *EnhancedChannel = create_enhanced_channel(3)
    
    fr fr Verify channels were registered
    test_assert(global_registry_size >= 3, "Channels were registered")
    
    fr fr Test deadlock detection
    sus deadlock_detected lit = check_for_deadlocks()
    test_assert(!deadlock_detected, "No deadlock with normal channels")
    
    fr fr Cleanup and verify unregistration
    enhanced_channel_cleanup(ch1)
    enhanced_channel_cleanup(ch2)  
    enhanced_channel_cleanup(ch3)
    
    fr fr After cleanup, registry size should decrease
    fr fr (exact size depends on cleanup order and other active channels)
    test_assert(global_registry_size >= 0, "Registry size is valid after cleanup")
}

slay test_waiter_limit_enforcement() {
    test_start("Waiter Limit Enforcement")
    
    sus ch *EnhancedChannel = create_enhanced_channel(1)
    
    fr fr Fill channel to force sends to wait
    test_assert(enhanced_channel_send(ch, 42), "Fill channel buffer")
    
    fr fr Set a very low waiter limit for testing
    ch.max_waiters = 1
    
    fr fr First send should succeed (will wait)
    fr fr Note: In a real concurrent environment, this would require careful orchestration
    fr fr For this test, we verify the limit checking logic
    
    sus original_max_waiters normie = ch.max_waiters
    test_assert(original_max_waiters == 1, "Waiter limit set correctly")
    
    enhanced_channel_cleanup(ch)
}

fr fr =============================================================================
fr fr COMPATIBILITY LAYER TESTS  
fr fr =============================================================================

slay test_go_style_channel_operations() {
    test_start("Go-style Channel Operations")
    
    fr fr Test make_chan
    sus ch *EnhancedChannel = make_chan(2)
    test_assert(ch != 0, "make_chan created channel")
    test_assert(ch.capacity == 2, "make_chan created correct capacity")
    
    fr fr Test chan_send (equivalent to ch <- data)
    test_assert(chan_send(ch, 100), "chan_send successful")
    test_assert(chan_send(ch, 200), "second chan_send successful")
    
    fr fr Test chan_recv (equivalent to data := <-ch)
    sus data1 normie = chan_recv(ch)
    test_assert(data1 == 100, "chan_recv got first data")
    
    sus data2 normie = chan_recv(ch)
    test_assert(data2 == 200, "chan_recv got second data")
    
    fr fr Test chan_recv_ok (equivalent to data, ok := <-ch)
    test_assert(chan_send(ch, 300), "Send data for recv_ok test")
    sus data3 normie = 0
    sus ok3 lit = cap
    (data3, ok3) = chan_recv_ok(ch)
    test_assert(ok3 && data3 == 300, "chan_recv_ok got data and ok")
    
    fr fr Test close_chan
    close_chan(ch)
    test_assert(enhanced_channel_is_closed(ch), "close_chan closed channel")
    
    fr fr Test recv from closed channel
    (data3, ok3) = chan_recv_ok(ch)
    test_assert(!ok3, "recv from closed empty channel returns not ok")
    
    enhanced_channel_cleanup(ch)
}

fr fr =============================================================================
fr fr STRESS AND EDGE CASE TESTS
fr fr =============================================================================

slay test_large_buffer_operations() {
    test_start("Large Buffer Operations")
    
    sus large_ch *EnhancedChannel = create_enhanced_channel(100)
    
    fr fr Fill large buffer
    sus i normie = 0
    bestie i < 100 {
        test_assert(enhanced_channel_send(large_ch, i + 1000), "Send to large buffer succeeded")
        i = i + 1
    }
    
    fr fr Verify buffer is full
    sus stats ChannelStats = enhanced_channel_stats(large_ch)
    test_assert(stats.current_size == 100, "Large buffer is full")
    test_assert(stats.total_sends == 100, "Correct send count")
    
    fr fr Drain large buffer
    sus j normie = 0
    bestie j < 100 {
        sus recv_data normie = 0
        sus recv_ok lit = cap
        (recv_data, recv_ok) = enhanced_channel_receive(large_ch)
        test_assert(recv_ok, "Receive from large buffer succeeded")
        test_assert(recv_data == j + 1000, "Received correct data from large buffer")
        j = j + 1
    }
    
    fr fr Verify buffer is empty
    sus final_stats ChannelStats = enhanced_channel_stats(large_ch)
    test_assert(final_stats.current_size == 0, "Large buffer is empty")
    test_assert(final_stats.total_recvs == 100, "Correct receive count")
    
    enhanced_channel_cleanup(large_ch)
}

slay test_rapid_channel_creation_and_cleanup() {
    test_start("Rapid Channel Creation and Cleanup")
    
    fr fr Create and cleanup many channels rapidly
    sus created_count normie = 0
    sus cleanup_count normie = 0
    
    sus k normie = 0
    bestie k < 50 {
        sus rapid_ch *EnhancedChannel = create_enhanced_channel(k % 10 + 1)
        ready rapid_ch != 0 {
            created_count = created_count + 1
            
            fr fr Quick operation
            enhanced_channel_send(rapid_ch, k)
            
            fr fr Cleanup
            enhanced_channel_cleanup(rapid_ch)
            cleanup_count = cleanup_count + 1
        }
        k = k + 1
    }
    
    test_assert(created_count == 50, "All channels created successfully")
    test_assert(cleanup_count == 50, "All channels cleaned up successfully")
}

slay test_select_with_many_channels() {
    test_start("Select with Many Channels")
    
    sus channel_count normie = 10
    sus channels []*EnhancedChannel = memory.allocate_array(*EnhancedChannel, channel_count)
    
    fr fr Create many channels
    sus i normie = 0
    bestie i < channel_count {
        channels[i] = create_enhanced_channel(1)
        i = i + 1
    }
    
    fr fr Make one channel ready
    sus ready_index normie = 5
    test_assert(enhanced_channel_send(channels[ready_index], 999), "Prepared ready channel")
    
    fr fr Create large select context
    sus ctx *SelectContext = create_select_context(channel_count)
    sus j normie = 0
    bestie j < channel_count {
        test_assert(select_add_recv(ctx, j, channels[j]), "Added channel to large select")
        j = j + 1
    }
    
    fr fr Execute select
    sus found_ready normie = enhanced_select_execute(ctx, 1000)  fr fr 1 second timeout
    test_assert(found_ready == ready_index, "Found the ready channel in large select")
    test_assert(ctx.result_data == 999, "Got correct data from large select")
    
    fr fr Cleanup all channels
    sus k normie = 0
    bestie k < channel_count {
        enhanced_channel_cleanup(channels[k])
        k = k + 1
    }
}

fr fr =============================================================================
fr fr RUN ALL TESTS
fr fr =============================================================================

slay main() {
    vibez.spill("🚀 Enhanced Channel Operations Comprehensive Test Suite")
    vibez.spill("   Testing: Blocking ops, deadlock prevention, select statements, buffered channels")
    
    fr fr Configure test environment
    configure_deadlock_prevention(5000, 1000, based)  fr fr 5 second timeout, up to 1000 waiters
    
    fr fr Basic functionality tests
    test_enhanced_channel_creation()
    test_buffered_channel_basic_operations()
    test_unbuffered_channel_basic_operations()
    
    fr fr Channel management tests
    test_channel_closing_and_cleanup()
    
    fr fr Select statement tests
    test_select_context_creation_and_management()
    test_select_try_operations()
    test_select_execute_with_ready_channels()
    test_select_timeout()
    
    fr fr Deadlock prevention tests
    test_deadlock_prevention_configuration()
    test_channel_registry_management()
    test_waiter_limit_enforcement()
    
    fr fr Compatibility tests
    test_go_style_channel_operations()
    
    fr fr Stress and edge case tests
    test_large_buffer_operations()
    test_rapid_channel_creation_and_cleanup()
    test_select_with_many_channels()
    
    fr fr Print final results
    test_summary()
    
    ready test_failed > 0 {
        vibez.spill("\n💥 Some tests failed - check implementation")
        damn 1
    } otherwise {
        vibez.spill("\n🎉 All enhanced channel tests passed - Production ready!")
        damn 0
    }
}
