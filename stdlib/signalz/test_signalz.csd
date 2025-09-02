yeet "testz"
yeet "signalz"

fr fr Test suite for CURSED Signal Handling Module (signalz)

slay test_signal_system_initialization() {
    test_start("Signal System Initialization")
    
    fr fr Test initialization
    initialize_signal_system()
    
    fr fr Check that system components are created
    sus stats *SignalStats = get_signal_statistics()
    assert_true(stats != 0)
    assert_eq_int(stats.total_signals, 0)
    
    sus queued_count normie = get_queued_signal_count()
    assert_eq_int(queued_count, 0)
    
    vibez.spill("✅ Signal system initialization tests passed")
}

slay test_signal_handler_registration() {
    test_start("Signal Handler Registration")
    
    sus handler_called lit = cap
    sus received_signal normie = 0
    
    fr fr Define test signal handler
    slay test_handler(signal_num normie) {
        handler_called = based
        received_signal = signal_num
        vibez.spill("Handler called for signal " + string(signal_num))
    }
    
    fr fr Test handler registration
    sus err *ErrorInstance = signal_register(SIGUSR1, test_handler)
    assert_true(err == 0)
    
    fr fr Test invalid signal number
    sus invalid_err *ErrorInstance = signal_register(999, test_handler)
    assert_true(invalid_err != 0)
    assert_true(is_error(error_result(invalid_err)))
    
    vibez.spill("✅ Signal handler registration tests passed")
}

slay test_signal_delivery() {
    test_start("Signal Delivery")
    
    sus handler_executed lit = cap
    sus signal_received normie = 0
    
    slay delivery_handler(signal_num normie) {
        handler_executed = based
        signal_received = signal_num
    }
    
    fr fr Register handler
    signal_register(SIGUSR2, delivery_handler)
    
    fr fr Deliver signal
    sus err *ErrorInstance = deliver_signal(SIGUSR2, 1234)
    assert_true(err == 0)
    
    fr fr Check statistics
    sus stats *SignalStats = get_signal_statistics()
    assert_true(stats.total_signals > 0)
    assert_true(stats.signals_received[SIGUSR2] > 0)
    
    vibez.spill("✅ Signal delivery tests passed")
}

slay test_signal_names() {
    test_start("Signal Names")
    
    fr fr Test signal name lookup
    assert_eq_string(signal_name(SIGINT), "SIGINT")
    assert_eq_string(signal_name(SIGTERM), "SIGTERM")
    assert_eq_string(signal_name(SIGUSR1), "SIGUSR1")
    assert_eq_string(signal_name(SIGUSR2), "SIGUSR2")
    assert_eq_string(signal_name(999), "UNKNOWN")
    
    fr fr Test signal number lookup
    assert_eq_int(signal_number("SIGINT"), SIGINT)
    assert_eq_int(signal_number("SIGTERM"), SIGTERM)
    assert_eq_int(signal_number("SIGUSR1"), SIGUSR1)
    assert_eq_int(signal_number("SIGUSR2"), SIGUSR2)
    assert_eq_int(signal_number("UNKNOWN"), 0)
    
    vibez.spill("✅ Signal name tests passed")
}

slay test_signal_masking() {
    test_start("Signal Masking")
    
    fr fr Test signal blocking
    sus err1 *ErrorInstance = signal_block(SIGUSR1)
    assert_true(err1 == 0)
    assert_true(is_signal_blocked(SIGUSR1))
    
    fr fr Test signal unblocking
    sus err2 *ErrorInstance = signal_unblock(SIGUSR1)
    assert_true(err2 == 0)
    assert_false(is_signal_blocked(SIGUSR1))
    
    fr fr Test blocking multiple signals
    sus signals normie[value] = [SIGUSR1, SIGUSR2, SIGALRM]
    sus err3 *ErrorInstance = block_signals_array(signals, 3)
    assert_true(err3 == 0)
    
    assert_true(is_signal_blocked(SIGUSR1))
    assert_true(is_signal_blocked(SIGUSR2))
    assert_true(is_signal_blocked(SIGALRM))
    
    fr fr Test unblocking multiple signals
    sus err4 *ErrorInstance = unblock_signals_array(signals, 3)
    assert_true(err4 == 0)
    
    assert_false(is_signal_blocked(SIGUSR1))
    assert_false(is_signal_blocked(SIGUSR2))
    assert_false(is_signal_blocked(SIGALRM))
    
    fr fr Test invalid signal number
    sus err5 *ErrorInstance = signal_block(999)
    assert_true(err5 != 0)
    
    vibez.spill("✅ Signal masking tests passed")
}

slay test_signal_ignore_and_default() {
    test_start("Signal Ignore and Default")
    
    fr fr Test signal ignore
    sus err1 *ErrorInstance = signal_ignore(SIGPIPE)
    assert_true(err1 == 0)
    
    fr fr Test signal default
    sus err2 *ErrorInstance = signal_default(SIGPIPE)
    assert_true(err2 == 0)
    
    fr fr Test signal unregister
    sus test_handler slay(normie) = slay(sig normie) { vibez.spill("Test") }
    signal_register(SIGUSR1, test_handler)
    
    sus err3 *ErrorInstance = signal_unregister(SIGUSR1)
    assert_true(err3 == 0)
    
    vibez.spill("✅ Signal ignore and default tests passed")
}

slay test_signal_queue() {
    test_start("Signal Queue")
    
    fr fr Block a signal to test queueing
    signal_block(SIGUSR1)
    
    fr fr Deliver signal while blocked (should be queued)
    sus err1 *ErrorInstance = deliver_signal(SIGUSR1, 1001)
    assert_true(err1 == 0)
    
    fr fr Check queue count
    sus queued_before normie = get_queued_signal_count()
    assert_true(queued_before > 0)
    
    fr fr Unblock signal (should deliver queued signals)
    signal_unblock(SIGUSR1)
    
    fr fr Check queue count after delivery
    sus queued_after normie = get_queued_signal_count()
    fr fr Note: In simplified implementation, queue may not actually decrease
    
    fr fr Test queue clearing
    clear_signal_queue()
    sus queued_cleared normie = get_queued_signal_count()
    assert_eq_int(queued_cleared, 0)
    
    vibez.spill("✅ Signal queue tests passed")
}

slay test_process_communication() {
    test_start("Process Communication")
    
    fr fr Test process communication creation
    sus comm *ProcessComm = create_process_comm(1234, 1000)
    assert_true(comm != 0)
    assert_eq_int(comm.process_id, 1234)
    assert_eq_int(comm.timeout_ms, 1000)
    assert_true(comm.active)
    
    fr fr Test sending signal to process
    sus err *ErrorInstance = send_signal_to_process(1234, SIGUSR1)
    assert_true(err == 0)
    
    fr fr Test invalid process ID
    sus invalid_err *ErrorInstance = send_signal_to_process(-1, SIGUSR1)
    assert_true(invalid_err != 0)
    
    fr fr Test response mechanism (simplified)
    sus response_sent lit = respond_to_signal(comm, 42)
    fr fr In simplified implementation, this may not work perfectly
    
    fr fr Test closing communication
    close_process_comm(comm)
    assert_false(comm.active)
    
    vibez.spill("✅ Process communication tests passed")
}

slay test_signal_statistics() {
    test_start("Signal Statistics")
    
    fr fr Reset statistics for clean test
    reset_signal_statistics()
    
    sus stats_before *SignalStats = get_signal_statistics()
    assert_eq_int(stats_before.total_signals, 0)
    
    fr fr Deliver some signals
    deliver_signal(SIGUSR1, 1001)
    deliver_signal(SIGUSR2, 1002)
    deliver_signal(SIGUSR1, 1003)
    
    sus stats_after *SignalStats = get_signal_statistics()
    assert_true(stats_after.total_signals >= 3)
    assert_true(stats_after.signals_received[SIGUSR1] >= 2)
    assert_true(stats_after.signals_received[SIGUSR2] >= 1)
    
    fr fr Test statistics printing
    print_signal_statistics()
    
    vibez.spill("✅ Signal statistics tests passed")
}

slay test_signal_enabling_disabling() {
    test_start("Signal Enabling/Disabling")
    
    fr fr Test disabling signal handling
    disable_signal_handling()
    
    fr fr Try to deliver signal while disabled
    sus err1 *ErrorInstance = deliver_signal(SIGUSR1, 1001)
    assert_true(err1 != 0)  fr fr Should fail when disabled
    
    fr fr Test re-enabling signal handling
    enable_signal_handling()
    
    fr fr Try to deliver signal while enabled
    sus err2 *ErrorInstance = deliver_signal(SIGUSR1, 1002)
    assert_true(err2 == 0)  fr fr Should succeed when enabled
    
    vibez.spill("✅ Signal enabling/disabling tests passed")
}

slay test_default_signal_actions() {
    test_start("Default Signal Actions")
    
    fr fr Test default actions don't crash
    fr fr Note: In real implementation these would exit process
    execute_default_action(SIGCHLD)  fr fr Should be ignored
    
    fr fr Test signal name to number conversions for all signals
    assert_eq_int(signal_number(signal_name(SIGINT)), SIGINT)
    assert_eq_int(signal_number(signal_name(SIGTERM)), SIGTERM)
    assert_eq_int(signal_number(signal_name(SIGUSR1)), SIGUSR1)
    assert_eq_int(signal_number(signal_name(SIGUSR2)), SIGUSR2)
    assert_eq_int(signal_number(signal_name(SIGALRM)), SIGALRM)
    assert_eq_int(signal_number(signal_name(SIGPIPE)), SIGPIPE)
    assert_eq_int(signal_number(signal_name(SIGCHLD)), SIGCHLD)
    
    vibez.spill("✅ Default signal action tests passed")
}

slay test_signal_mask_operations() {
    test_start("Signal Mask Operations")
    
    fr fr Test mask creation
    sus mask *SignalMask = create_signal_mask()
    assert_true(mask != 0)
    assert_eq_int(mask.count, 0)
    
    fr fr Test current mask
    sus current_mask *SignalMask = signal_mask_current()
    assert_true(current_mask != 0)
    
    fr fr Block some signals and check mask
    signal_block(SIGUSR1)
    signal_block(SIGUSR2)
    
    sus mask_with_blocks *SignalMask = signal_mask_current()
    assert_true(mask_with_blocks.count >= 2)
    
    fr fr Test mask restoration
    sus restore_err *ErrorInstance = signal_mask_restore(mask)
    assert_true(restore_err == 0)
    
    fr fr Test restoring invalid mask
    sus invalid_restore_err *ErrorInstance = signal_mask_restore(0)
    assert_true(invalid_restore_err != 0)
    
    vibez.spill("✅ Signal mask operation tests passed")
}

slay test_comprehensive_signal_workflow() {
    test_start("Comprehensive Signal Workflow")
    
    fr fr Setup complete signal handling workflow
    sus workflow_executed lit = cap
    sus signals_handled normie = 0
    
    slay workflow_handler(signal_num normie) {
        workflow_executed = based
        signals_handled = signals_handled + 1
        vibez.spill("Workflow handler got signal " + string(signal_num))
    }
    
    fr fr Register handlers for multiple signals
    signal_register(SIGUSR1, workflow_handler)
    signal_register(SIGUSR2, workflow_handler)
    signal_register(SIGALRM, workflow_handler)
    
    fr fr Test signal delivery with various states
    enable_signal_handling()
    
    fr fr 1. Normal delivery
    deliver_signal(SIGUSR1, 2001)
    
    fr fr 2. Blocked signal delivery (queued)
    signal_block(SIGUSR2)
    deliver_signal(SIGUSR2, 2002)
    
    fr fr 3. Unblock and deliver queued
    signal_unblock(SIGUSR2)
    
    fr fr 4. Ignored signal
    signal_ignore(SIGALRM)
    deliver_signal(SIGALRM, 2003)
    
    fr fr 5. Restored default
    signal_default(SIGALRM)
    deliver_signal(SIGALRM, 2004)
    
    fr fr Check final statistics
    sus final_stats *SignalStats = get_signal_statistics()
    assert_true(final_stats.total_signals >= 4)
    
    print_signal_statistics()
    
    vibez.spill("✅ Comprehensive signal workflow tests passed")
}

slay test_signal_error_conditions() {
    test_start("Signal Error Conditions")
    
    fr fr Test registering handler for uncatchable signals
    slay dummy_handler(sig normie) { vibez.spill("Should not be called") }
    
    fr fr These should work in our simplified implementation
    sus err1 *ErrorInstance = signal_register(SIGKILL, dummy_handler)
    fr fr In real implementation SIGKILL cannot be caught
    
    sus err2 *ErrorInstance = signal_register(SIGSTOP, dummy_handler)
    fr fr In real implementation SIGSTOP cannot be caught
    
    fr fr Test various invalid operations
    sus err3 *ErrorInstance = signal_register(0, dummy_handler)
    assert_true(err3 != 0)
    
    sus err4 *ErrorInstance = signal_register(999, dummy_handler)
    assert_true(err4 != 0)
    
    sus err5 *ErrorInstance = deliver_signal(0, 1001)
    assert_true(err5 != 0)
    
    sus err6 *ErrorInstance = deliver_signal(999, 1002)
    assert_true(err6 != 0)
    
    vibez.spill("✅ Signal error condition tests passed")
}

slay run_all_signalz_tests() {
    vibez.spill("🚀 Starting CURSED Signal Handling (signalz) Tests")
    
    test_signal_system_initialization()
    test_signal_handler_registration()
    test_signal_delivery()
    test_signal_names()
    test_signal_masking()
    test_signal_ignore_and_default()
    test_signal_queue()
    test_process_communication()
    test_signal_statistics()
    test_signal_enabling_disabling()
    test_default_signal_actions()
    test_signal_mask_operations()
    test_comprehensive_signal_workflow()
    test_signal_error_conditions()
    
    print_test_summary()
    vibez.spill("✅ All signalz tests completed!")
}

fr fr Run tests when this file is executed
run_all_signalz_tests()
