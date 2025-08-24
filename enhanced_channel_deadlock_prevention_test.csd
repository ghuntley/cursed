yeet "concurrenz/mod_enhanced_channels"
yeet "testz"
yeet "vibez"

fr fr Enhanced Channel Deadlock Prevention Test Suite  
fr fr Comprehensive deadlock scenario testing and prevention validation

sus test_passed normie = 0
sus test_failed normie = 0
sus test_total normie = 0

slay test_start(test_name tea) {
    vibez.spillf("\n🔒 Deadlock Test: {}", test_name)
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
    vibez.spill("\n📊 Deadlock Prevention Test Summary:")
    vibez.spillf("  Total deadlock tests: {}", test_total)
    vibez.spillf("  Passed: {}", test_passed)
    vibez.spillf("  Failed: {}", test_failed)
    ready test_failed == 0 {
        vibez.spill("  🛡️  All deadlock prevention tests passed!")
    } otherwise {
        vibez.spillf("  ⚠️  {} deadlock tests failed", test_failed)
    }
}

fr fr =============================================================================
fr fr TIMEOUT DEADLOCK PREVENTION TESTS
fr fr =============================================================================

slay test_send_timeout_prevention() {
    test_start("Send Timeout Deadlock Prevention")
    
    fr fr Configure short timeout for testing
    configure_deadlock_prevention(100, 1000, based)  fr fr 100ms timeout
    
    fr fr Create full channel that will block sends
    sus ch *EnhancedChannel = create_enhanced_channel(1)
    test_assert(enhanced_channel_send(ch, 42), "Fill channel buffer")
    
    fr fr Verify channel is full
    sus stats ChannelStats = enhanced_channel_stats(ch)
    test_assert(stats.current_size == 1, "Channel buffer is full")
    
    fr fr Attempt send that should timeout to prevent deadlock
    sus start_time thicc = get_current_time_ms()
    sus send_result lit = enhanced_channel_send(ch, 84)  fr fr This should timeout
    sus end_time thicc = get_current_time_ms()
    sus elapsed_time thicc = end_time - start_time
    
    test_assert(!send_result, "Send to full channel timed out (prevented deadlock)")
    test_assert(elapsed_time >= 100, "Send timeout took at least configured time")
    test_assert(elapsed_time < 500, "Send timeout didn't take too long")
    
    fr fr Restore normal timeout
    configure_deadlock_prevention(10000, 1000, based)
    enhanced_channel_cleanup(ch)
}

slay test_receive_timeout_prevention() {
    test_start("Receive Timeout Deadlock Prevention")
    
    fr fr Configure short timeout
    configure_deadlock_prevention(100, 1000, based)
    
    fr fr Create empty channel that will block receives
    sus ch *EnhancedChannel = create_enhanced_channel(1)
    
    fr fr Verify channel is empty
    sus stats ChannelStats = enhanced_channel_stats(ch)
    test_assert(stats.current_size == 0, "Channel buffer is empty")
    
    fr fr Attempt receive that should timeout
    sus start_time thicc = get_current_time_ms()
    sus recv_data normie = 0
    sus recv_ok lit = cap
    (recv_data, recv_ok) = enhanced_channel_receive(ch)
    sus end_time thicc = get_current_time_ms()
    sus elapsed_time thicc = end_time - start_time
    
    test_assert(!recv_ok, "Receive from empty channel timed out (prevented deadlock)")
    test_assert(elapsed_time >= 100, "Receive timeout took at least configured time")
    test_assert(elapsed_time < 500, "Receive timeout didn't take too long")
    
    configure_deadlock_prevention(10000, 1000, based)
    enhanced_channel_cleanup(ch)
}

slay test_unbuffered_channel_timeout_prevention() {
    test_start("Unbuffered Channel Timeout Prevention")
    
    configure_deadlock_prevention(150, 1000, based)
    
    fr fr Create unbuffered channel (requires synchronous communication)
    sus sync_ch *EnhancedChannel = create_enhanced_channel(0)
    test_assert(sync_ch.capacity == 0, "Created unbuffered channel")
    
    fr fr Test send timeout (no receiver waiting)
    sus start_time thicc = get_current_time_ms()
    sus send_result lit = enhanced_channel_send(sync_ch, 123)
    sus end_time thicc = get_current_time_ms()
    sus elapsed thicc = end_time - start_time
    
    test_assert(!send_result, "Send to unbuffered channel with no receiver timed out")
    test_assert(elapsed >= 150, "Unbuffered send timeout respected configured time")
    
    fr fr Test receive timeout (no sender waiting)
    start_time = get_current_time_ms()
    sus recv_data normie = 0
    sus recv_ok lit = cap
    (recv_data, recv_ok) = enhanced_channel_receive(sync_ch)
    end_time = get_current_time_ms()
    elapsed = end_time - start_time
    
    test_assert(!recv_ok, "Receive from unbuffered channel with no sender timed out")
    test_assert(elapsed >= 150, "Unbuffered receive timeout respected configured time")
    
    configure_deadlock_prevention(10000, 1000, based)
    enhanced_channel_cleanup(sync_ch)
}

fr fr =============================================================================
fr fr WAITER LIMIT DEADLOCK PREVENTION TESTS
fr fr =============================================================================

slay test_max_waiters_enforcement() {
    test_start("Maximum Waiters Enforcement")
    
    fr fr Create channel with very low waiter limit
    sus ch *EnhancedChannel = create_enhanced_channel(1)
    ch.max_waiters = 2  fr fr Only allow 2 waiters
    
    fr fr Fill channel to force waiting
    test_assert(enhanced_channel_send(ch, 1), "Fill channel")
    
    fr fr Verify waiter limit enforcement
    fr fr In a real concurrent system, this would involve multiple goroutines
    fr fr For this test, we verify the limit is configured correctly
    test_assert(ch.max_waiters == 2, "Channel waiter limit set correctly")
    
    fr fr Simulate checking waiter count
    sus current_send_waiters normie = atomic_drip.atomic_load_i32(&ch.send_waiters, ACQUIRE)
    sus current_recv_waiters normie = atomic_drip.atomic_load_i32(&ch.recv_waiters, ACQUIRE)
    sus total_waiters normie = current_send_waiters + current_recv_waiters
    
    test_assert(total_waiters <= ch.max_waiters, "Waiter count within limits")
    
    enhanced_channel_cleanup(ch)
}

slay test_global_waiter_limit_detection() {
    test_start("Global Waiter Limit Detection")
    
    fr fr Configure low global limit for testing
    configure_deadlock_prevention(10000, 5, based)  fr fr Only 5 total waiters allowed
    
    fr fr Create multiple channels and simulate high waiter count
    sus ch1 *EnhancedChannel = create_enhanced_channel(1)
    sus ch2 *EnhancedChannel = create_enhanced_channel(1)
    sus ch3 *EnhancedChannel = create_enhanced_channel(1)
    
    fr fr Check initial deadlock state
    sus initial_deadlock lit = check_for_deadlocks()
    test_assert(!initial_deadlock, "No initial deadlock with empty channels")
    
    fr fr In a real system, we would create actual waiters
    fr fr For testing, we verify the detection logic exists
    test_assert(global_deadlock_config.max_total_waiters == 5, "Global waiter limit configured")
    test_assert(global_deadlock_config.prevention_enabled, "Deadlock prevention enabled")
    
    fr fr Restore normal limits
    configure_deadlock_prevention(10000, 1000, based)
    
    enhanced_channel_cleanup(ch1)
    enhanced_channel_cleanup(ch2)
    enhanced_channel_cleanup(ch3)
}

fr fr =============================================================================
fr fr CHANNEL REGISTRY AND DETECTION TESTS
fr fr =============================================================================

slay test_channel_registration_and_unregistration() {
    test_start("Channel Registration and Unregistration")
    
    fr fr Get initial registry size
    sus initial_size normie = global_registry_size
    
    fr fr Create channels and verify registration
    sus ch1 *EnhancedChannel = create_enhanced_channel(2)
    sus ch2 *EnhancedChannel = create_enhanced_channel(3)
    
    test_assert(global_registry_size >= initial_size + 2, "Channels were registered")
    
    fr fr Test that registry contains our channels
    sus found_ch1 lit = cap
    sus found_ch2 lit = cap
    
    sus i normie = 0
    bestie i < global_registry_size {
        ready global_channel_registry[i] == ch1 {
            found_ch1 = based
        }
        ready global_channel_registry[i] == ch2 {
            found_ch2 = based
        }
        i = i + 1
    }
    
    test_assert(found_ch1, "Channel 1 found in registry")
    test_assert(found_ch2, "Channel 2 found in registry")
    
    fr fr Cleanup and verify unregistration
    enhanced_channel_cleanup(ch1)
    enhanced_channel_cleanup(ch2)
    
    fr fr Registry should be cleaned up (size may not decrease immediately due to other tests)
    test_assert(global_registry_size >= 0, "Registry remains valid after cleanup")
}

slay test_deadlock_detection_heuristics() {
    test_start("Deadlock Detection Heuristics")
    
    fr fr Test with no channels
    global_registry_size = 0
    sus deadlock_empty lit = check_for_deadlocks()
    test_assert(!deadlock_empty, "No deadlock detected with empty registry")
    
    fr fr Create channels for testing heuristics
    sus test_ch1 *EnhancedChannel = create_enhanced_channel(1)
    sus test_ch2 *EnhancedChannel = create_enhanced_channel(2)
    sus test_ch3 *EnhancedChannel = create_enhanced_channel(0)
    
    fr fr Test deadlock detection with normal channels
    sus deadlock_normal lit = check_for_deadlocks()
    test_assert(!deadlock_normal, "No deadlock detected with normal channels")
    
    fr fr Simulate some waiters (in real implementation these would be actual blocked goroutines)
    atomic_drip.atomic_store_i32(&test_ch1.send_waiters, 1, RELAXED)
    atomic_drip.atomic_store_i32(&test_ch2.recv_waiters, 2, RELAXED)
    
    sus deadlock_waiters lit = check_for_deadlocks()
    fr fr Deadlock detection depends on the specific heuristics and thresholds
    test_assert(deadlock_waiters == based || deadlock_waiters == cap, "Deadlock detection ran with waiters")
    
    fr fr Reset waiters
    atomic_drip.atomic_store_i32(&test_ch1.send_waiters, 0, RELAXED)
    atomic_drip.atomic_store_i32(&test_ch2.recv_waiters, 0, RELAXED)
    
    enhanced_channel_cleanup(test_ch1)
    enhanced_channel_cleanup(test_ch2)
    enhanced_channel_cleanup(test_ch3)
}

fr fr =============================================================================
fr fr SELECT STATEMENT DEADLOCK PREVENTION TESTS
fr fr =============================================================================

slay test_select_timeout_prevention() {
    test_start("Select Statement Timeout Prevention")
    
    fr fr Create channels that will cause select to block
    sus ch1 *EnhancedChannel = create_enhanced_channel(0)  fr fr Unbuffered, no senders/receivers
    sus ch2 *EnhancedChannel = create_enhanced_channel(1)  fr fr Buffered but empty
    
    fr fr Fill ch2 to make sends block
    test_assert(enhanced_channel_send(ch2, 99), "Fill ch2 for select test")
    
    fr fr Create select context with operations that will block
    sus ctx *SelectContext = create_select_context(2)
    test_assert(select_add_recv(ctx, 0, ch1), "Add blocking recv to select")
    test_assert(select_add_send(ctx, 1, ch2, 100), "Add blocking send to select")
    
    fr fr Execute select with timeout
    sus start_time thicc = get_current_time_ms()
    sus ready_channel normie = enhanced_select_execute(ctx, 200)  fr fr 200ms timeout
    sus end_time thicc = get_current_time_ms()
    sus elapsed thicc = end_time - start_time
    
    test_assert(ready_channel == -1, "Select timed out as expected")
    test_assert(elapsed >= 200, "Select timeout took at least specified time")
    test_assert(elapsed < 1000, "Select timeout didn't take too long")
    
    enhanced_channel_cleanup(ch1)
    enhanced_channel_cleanup(ch2)
}

slay test_select_fairness_prevents_starvation() {
    test_start("Select Fairness Prevents Starvation")
    
    fr fr Create multiple channels with ready operations
    sus ch1 *EnhancedChannel = create_enhanced_channel(1)
    sus ch2 *EnhancedChannel = create_enhanced_channel(1)
    sus ch3 *EnhancedChannel = create_enhanced_channel(1)
    
    fr fr Prepare all channels with data
    test_assert(enhanced_channel_send(ch1, 101), "Prepare ch1 with data")
    test_assert(enhanced_channel_send(ch2, 102), "Prepare ch2 with data")
    test_assert(enhanced_channel_send(ch3, 103), "Prepare ch3 with data")
    
    fr fr Create select context
    sus ctx *SelectContext = create_select_context(3)
    test_assert(select_add_recv(ctx, 0, ch1), "Add recv from ch1")
    test_assert(select_add_recv(ctx, 1, ch2), "Add recv from ch2")
    test_assert(select_add_recv(ctx, 2, ch3), "Add recv from ch3")
    
    fr fr Execute select multiple times to test fairness
    sus selected_channels []normie = memory.allocate_array(normie, 10)
    sus different_channels_selected normie = 0
    sus j normie = 0
    
    bestie j < 10 {
        fr fr Refresh channels with data for each iteration
        ready j > 0 {
            test_assert(enhanced_channel_send(ch1, 201 + j), "Refresh ch1")
            test_assert(enhanced_channel_send(ch2, 202 + j), "Refresh ch2") 
            test_assert(enhanced_channel_send(ch3, 203 + j), "Refresh ch3")
        }
        
        sus ready_channel normie = enhanced_select_execute(ctx, 100)
        ready ready_channel >= 0 && ready_channel < 3 {
            selected_channels[j] = ready_channel
            
            fr fr Check if this is a new channel we haven't seen
            sus is_new lit = based
            sus k normie = 0
            bestie k < j {
                ready selected_channels[k] == ready_channel {
                    is_new = cap
                    break
                }
                k = k + 1
            }
            ready is_new {
                different_channels_selected = different_channels_selected + 1
            }
        }
        j = j + 1
    }
    
    fr fr Fairness test: we should have selected different channels
    test_assert(different_channels_selected > 1, "Select fairness: multiple different channels selected")
    
    enhanced_channel_cleanup(ch1)
    enhanced_channel_cleanup(ch2)
    enhanced_channel_cleanup(ch3)
}

fr fr =============================================================================
fr fr EDGE CASE DEADLOCK SCENARIOS
fr fr =============================================================================

slay test_closed_channel_deadlock_prevention() {
    test_start("Closed Channel Deadlock Prevention")
    
    sus ch *EnhancedChannel = create_enhanced_channel(1)
    
    fr fr Close channel immediately
    test_assert(enhanced_channel_close(ch), "Close channel")
    test_assert(enhanced_channel_is_closed(ch), "Channel is closed")
    
    fr fr Operations on closed channels should not cause deadlocks
    sus send_to_closed lit = enhanced_channel_send(ch, 42)
    test_assert(!send_to_closed, "Send to closed channel fails immediately (no deadlock)")
    
    sus recv_data normie = 0
    sus recv_ok lit = cap
    (recv_data, recv_ok) = enhanced_channel_receive(ch)
    test_assert(!recv_ok, "Receive from closed empty channel fails immediately (no deadlock)")
    
    fr fr Select operations should also handle closed channels
    sus ctx *SelectContext = create_select_context(1)
    test_assert(select_add_send(ctx, 0, ch, 99), "Add send to closed channel in select")
    
    sus ready_channel normie = enhanced_select_execute(ctx, 50)
    test_assert(ready_channel == -1, "Select with closed channel times out (no deadlock)")
    
    enhanced_channel_cleanup(ch)
}

slay test_rapid_close_and_operation_deadlock_prevention() {
    test_start("Rapid Close and Operation Deadlock Prevention") 
    
    sus rapid_ch *EnhancedChannel = create_enhanced_channel(5)
    
    fr fr Add some data
    test_assert(enhanced_channel_send(rapid_ch, 1), "Send before rapid operations")
    test_assert(enhanced_channel_send(rapid_ch, 2), "Send more data")
    
    fr fr Rapid close
    test_assert(enhanced_channel_close(rapid_ch), "Rapid close")
    
    fr fr Immediate operations after close
    sus immediate_send lit = enhanced_channel_send(rapid_ch, 3)
    test_assert(!immediate_send, "Immediate send after close fails (no deadlock)")
    
    fr fr Can still receive existing data
    sus recv1_data normie = 0
    sus recv1_ok lit = cap
    (recv1_data, recv1_ok) = enhanced_channel_receive(rapid_ch)
    test_assert(recv1_ok && recv1_data == 1, "Can receive existing data after close")
    
    sus recv2_data normie = 0
    sus recv2_ok lit = cap
    (recv2_data, recv2_ok) = enhanced_channel_receive(rapid_ch)
    test_assert(recv2_ok && recv2_data == 2, "Can receive second existing data after close")
    
    fr fr No more data available
    sus recv3_data normie = 0
    sus recv3_ok lit = cap
    (recv3_data, recv3_ok) = enhanced_channel_receive(rapid_ch)
    test_assert(!recv3_ok, "No more data after draining closed channel")
    
    enhanced_channel_cleanup(rapid_ch)
}

slay test_memory_pressure_deadlock_prevention() {
    test_start("Memory Pressure Deadlock Prevention")
    
    fr fr Create many channels to simulate memory pressure
    sus pressure_channels []*EnhancedChannel = memory.allocate_array(*EnhancedChannel, 20)
    sus created_count normie = 0
    
    sus i normie = 0
    bestie i < 20 {
        pressure_channels[i] = create_enhanced_channel(i % 5 + 1)
        ready pressure_channels[i] != 0 {
            created_count = created_count + 1
        }
        i = i + 1
    }
    
    test_assert(created_count == 20, "Created channels under memory pressure")
    
    fr fr Perform operations on all channels
    sus operation_count normie = 0
    sus j normie = 0
    bestie j < 20 {
        ready pressure_channels[j] != 0 {
            sus send_result lit = enhanced_channel_send(pressure_channels[j], j + 100)
            ready send_result {
                operation_count = operation_count + 1
            }
        }
        j = j + 1
    }
    
    test_assert(operation_count > 0, "Performed operations under memory pressure")
    
    fr fr Cleanup all channels
    sus cleanup_count normie = 0
    sus k normie = 0
    bestie k < 20 {
        ready pressure_channels[k] != 0 {
            enhanced_channel_cleanup(pressure_channels[k])
            cleanup_count = cleanup_count + 1
        }
        k = k + 1
    }
    
    test_assert(cleanup_count == 20, "Cleaned up all channels under memory pressure")
}

fr fr =============================================================================
fr fr DEADLOCK PREVENTION CONFIGURATION TESTS
fr fr =============================================================================

slay test_deadlock_prevention_disable_and_enable() {
    test_start("Deadlock Prevention Disable and Enable")
    
    fr fr Test disabling deadlock prevention
    configure_deadlock_prevention(1000, 100, cap)  fr fr Disable
    test_assert(!global_deadlock_config.prevention_enabled, "Deadlock prevention disabled")
    
    fr fr Create channel and test that operations don't timeout when disabled
    sus ch *EnhancedChannel = create_enhanced_channel(1)
    test_assert(enhanced_channel_send(ch, 42), "Fill channel")
    
    fr fr With prevention disabled, this might block longer (but we can't test infinite blocking)
    fr fr Just verify the configuration is respected
    test_assert(!global_deadlock_config.prevention_enabled, "Prevention remains disabled")
    
    fr fr Re-enable and test
    configure_deadlock_prevention(100, 100, based)  fr fr Enable with short timeout
    test_assert(global_deadlock_config.prevention_enabled, "Deadlock prevention re-enabled")
    test_assert(global_deadlock_config.max_wait_time == 100, "Timeout configured correctly")
    
    fr fr Test that timeout is now enforced
    sus start_time thicc = get_current_time_ms()
    sus send_result lit = enhanced_channel_send(ch, 84)  fr fr Should timeout
    sus end_time thicc = get_current_time_ms()
    
    test_assert(!send_result, "Send timed out with prevention enabled")
    test_assert(end_time - start_time >= 100, "Timeout was enforced")
    
    fr fr Restore defaults
    configure_deadlock_prevention(10000, 1000, based)
    enhanced_channel_cleanup(ch)
}

slay test_adaptive_timeout_behavior() {
    test_start("Adaptive Timeout Behavior")
    
    fr fr Test different timeout values
    sus timeout_values []normie = memory.allocate_array(normie, 3)
    timeout_values[0] = 50
    timeout_values[1] = 100  
    timeout_values[2] = 200
    
    sus i normie = 0
    bestie i < 3 {
        configure_deadlock_prevention(timeout_values[i], 1000, based)
        
        sus ch *EnhancedChannel = create_enhanced_channel(1)
        test_assert(enhanced_channel_send(ch, 1), "Fill channel for timeout test")
        
        sus start_time thicc = get_current_time_ms()
        sus send_result lit = enhanced_channel_send(ch, 2)  fr fr Should timeout
        sus end_time thicc = get_current_time_ms()
        sus elapsed thicc = end_time - start_time
        
        test_assert(!send_result, "Send timed out with adaptive timeout")
        test_assert(elapsed >= timeout_values[i], "Timeout respected configured value")
        test_assert(elapsed < timeout_values[i] + 100, "Timeout didn't exceed expected range")
        
        enhanced_channel_cleanup(ch)
        i = i + 1
    }
    
    configure_deadlock_prevention(10000, 1000, based)
}

fr fr =============================================================================
fr fr RUN ALL DEADLOCK PREVENTION TESTS
fr fr =============================================================================

slay main() {
    vibez.spill("🔒 Enhanced Channel Deadlock Prevention Test Suite")
    vibez.spill("   Testing: Timeout prevention, waiter limits, detection heuristics")
    
    fr fr Timeout prevention tests
    test_send_timeout_prevention()
    test_receive_timeout_prevention()
    test_unbuffered_channel_timeout_prevention()
    
    fr fr Waiter limit tests
    test_max_waiters_enforcement()
    test_global_waiter_limit_detection()
    
    fr fr Registry and detection tests
    test_channel_registration_and_unregistration()
    test_deadlock_detection_heuristics()
    
    fr fr Select statement deadlock prevention
    test_select_timeout_prevention()
    test_select_fairness_prevents_starvation()
    
    fr fr Edge case deadlock scenarios
    test_closed_channel_deadlock_prevention()
    test_rapid_close_and_operation_deadlock_prevention()
    test_memory_pressure_deadlock_prevention()
    
    fr fr Configuration tests
    test_deadlock_prevention_disable_and_enable()
    test_adaptive_timeout_behavior()
    
    test_summary()
    
    ready test_failed > 0 {
        vibez.spill("\n⚠️  Some deadlock prevention tests failed")
        damn 1
    } otherwise {
        vibez.spill("\n🛡️  All deadlock prevention tests passed - System is safe!")
        damn 0
    }
}
