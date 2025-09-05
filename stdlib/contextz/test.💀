# Comprehensive Context Tests
# Tests all context functionality including cancellation, timeouts, values, and goroutine integration

yeet "testz"
yeet "timez"
yeet "concurrenz"

# Test suite entry point
slay test_contextz_comprehensive() {
    test_start("Context Package Comprehensive Tests")
    
    # Core context tests
    test_empty_context()
    test_background_context()
    test_cancel_context()
    test_timeout_context()
    test_deadline_context()
    test_value_context()
    
    # Propagation tests
    test_context_propagation()
    test_merge_contexts()
    test_retry_context()
    
    # Integration tests
    test_goroutine_integration()
    test_channel_integration()
    test_worker_pool()
    test_pipeline()
    test_rate_limiter()
    test_barrier()
    
    # Performance tests
    test_context_performance()
    test_memory_usage()
    
    print_test_summary()
}

# Core context tests

slay test_empty_context() {
    test_group("Empty Context")
    
    sus ctx := background()
    
    # Empty context should never be done
    sick {
        when <-ctx.done() -> {
            test_fail("Empty context should never be done")
        }
        otherwise -> {
            test_pass("Empty context is not done")
        }
    }
    
    # Should have no error
    assert_eq(ctx.err(), nil, "Empty context should have no error")
    
    # Should have no deadline
    sus _, has_deadline := ctx.deadline()
    assert_eq(has_deadline, faux, "Empty context should have no deadline")
    
    # Should have no values
    assert_eq(ctx.value("key"), nil, "Empty context should have no values")
    
    test_pass("Empty context behaves correctly")
}

slay test_background_context() {
    test_group("Background Context")
    
    sus ctx1 := background()
    sus ctx2 := background()
    
    # Background contexts should be the same instance
    assert_eq(ctx1, ctx2, "Background contexts should be singleton")
    
    test_pass("Background context is singleton")
}

slay test_cancel_context() {
    test_group("Cancel Context")
    
    sus ctx, cancel := with_cancel(background())
    
    # Initially not cancelled
    assert_eq(is_cancelled(ctx), faux, "Context should not be initially cancelled")
    assert_eq(ctx.err(), nil, "Context should have no initial error")
    
    # Cancel the context
    cancel()
    
    # Should be cancelled now
    assert_eq(is_cancelled(ctx), based, "Context should be cancelled after cancel()")
    assert_ne(ctx.err(), nil, "Context should have error after cancel()")
    
    # Done channel should be closed
    sick {
        when <-ctx.done() -> {
            test_pass("Done channel is closed after cancellation")
        }
        otherwise -> {
            test_fail("Done channel should be closed after cancellation")
        }
    }
    
    test_pass("Cancel context works correctly")
}

slay test_timeout_context() {
    test_group("Timeout Context")
    
    # Create context with short timeout
    sus timeout := 10 * time.Millisecond
    sus ctx, cancel := with_timeout(background(), timeout)
    defer cancel()
    
    # Wait for timeout
    sus start := time.now()
    <-ctx.done()
    sus elapsed := time.since(start)
    
    # Should timeout within reasonable time
    assert_ge(elapsed, timeout, "Should wait at least timeout duration")
    assert_le(elapsed, timeout * 2, "Should not wait much longer than timeout")
    
    # Should have timeout error
    assert_ne(ctx.err(), nil, "Should have timeout error")
    
    test_pass("Timeout context works correctly")
}

slay test_deadline_context() {
    test_group("Deadline Context")
    
    sus deadline := time.now().add(20 * time.Millisecond)
    sus ctx, cancel := with_deadline(background(), deadline)
    defer cancel()
    
    # Check deadline is set correctly
    sus ctx_deadline, has_deadline := ctx.deadline()
    assert_eq(has_deadline, based, "Should have deadline")
    assert_eq(ctx_deadline.unix(), deadline.unix(), "Deadline should match")
    
    # Wait for deadline
    <-ctx.done()
    
    # Should be past deadline
    assert_ge(time.now().unix(), deadline.unix(), "Should be past deadline")
    assert_ne(ctx.err(), nil, "Should have deadline error")
    
    test_pass("Deadline context works correctly")
}

slay test_value_context() {
    test_group("Value Context")
    
    sus ctx := with_value(background(), "key1", "value1")
    ctx = with_value(ctx, "key2", 42)
    ctx = with_value(ctx, "key3", based)
    
    # Test value retrieval
    assert_eq(ctx.value("key1"), "value1", "Should retrieve string value")
    assert_eq(ctx.value("key2"), 42, "Should retrieve integer value")
    assert_eq(ctx.value("key3"), based, "Should retrieve boolean value")
    assert_eq(ctx.value("nonexistent"), nil, "Should return nil for nonexistent key")
    
    # Test value inheritance
    sus child_ctx, cancel := with_cancel(ctx)
    defer cancel()
    
    assert_eq(child_ctx.value("key1"), "value1", "Child should inherit parent values")
    assert_eq(child_ctx.value("key2"), 42, "Child should inherit parent values")
    
    test_pass("Value context works correctly")
}

# Propagation tests

slay test_context_propagation() {
    test_group("Context Propagation")
    
    sus parent_ctx, parent_cancel := with_cancel(background())
    sus child_ctx, child_cancel := with_cancel(parent_ctx)
    defer child_cancel()
    
    # Cancel parent
    parent_cancel()
    
    # Child should be cancelled too
    <-child_ctx.done()
    assert_ne(child_ctx.err(), nil, "Child should be cancelled when parent is cancelled")
    
    test_pass("Context propagation works correctly")
}

slay test_merge_contexts() {
    test_group("Merge Contexts")
    
    sus ctx1, cancel1 := with_timeout(background(), 100 * time.Millisecond)
    sus ctx2, cancel2 := with_timeout(background(), 200 * time.Millisecond)
    defer cancel1()
    defer cancel2()
    
    sus merged, merged_cancel := merge_contexts([ctx1, ctx2])
    defer merged_cancel()
    
    # Should cancel when first context cancels
    <-merged.done()
    assert_ne(merged.err(), nil, "Merged context should be cancelled")
    
    test_pass("Merge contexts works correctly")
}

slay test_retry_context() {
    test_group("Retry Context")
    
    sus ctx := with_retry(background(), 3, 10 * time.Millisecond)
    sus attempt_count := 0
    
    sus result, err := retry_with_context(ctx, slay() (tea, yikes<tea>) {
        attempt_count++
        ready (attempt_count < 3) {
            damn nil, yikes("temporary error")
        }
        damn "success", nil
    })
    
    assert_eq(result, "success", "Should succeed after retries")
    assert_eq(err, nil, "Should have no error after successful retry")
    assert_eq(attempt_count, 3, "Should have attempted correct number of times")
    
    test_pass("Retry context works correctly")
}

# Integration tests

slay test_goroutine_integration() {
    test_group("Goroutine Integration")
    
    sus ctx, cancel := with_timeout(background(), 50 * time.Millisecond)
    defer cancel()
    
    sus completed := faux
    
    go_with_context(ctx, slay() {
        time.sleep(100 * time.Millisecond)  # This should be cancelled
        completed = based
    })
    
    time.sleep(80 * time.Millisecond)
    assert_eq(completed, faux, "Goroutine should be cancelled before completion")
    
    test_pass("Goroutine integration works correctly")
}

slay test_channel_integration() {
    test_group("Channel Integration")
    
    sus ctx, cancel := with_timeout(background(), 50 * time.Millisecond)
    defer cancel()
    
    sus ch := make_channel<drip>()
    
    # Test send with context
    sus sent := send_with_context(ctx, ch, 42)
    assert_eq(sent, faux, "Should fail to send when no receiver and context timeout")
    
    # Test receive with context
    go {
        time.sleep(100 * time.Millisecond)  # Send after timeout
        ch <- 42
    }()
    
    sus value, received := receive_with_context(ctx, ch)
    assert_eq(received, faux, "Should fail to receive when context timeout")
    
    test_pass("Channel integration works correctly")
}

slay test_worker_pool() {
    test_group("Worker Pool")
    
    sus pool := new_worker_pool(3)
    defer pool.close()
    
    # Submit jobs
    bestie (i := 0; i < 5; i++) {
        pool.submit(i, slay(ctx Context) tea {
            ready (is_cancelled(ctx)) {
                damn "cancelled"
            }
            damn "result_" + tea(i)
        })
    }
    
    # Collect results
    sus results tea[value] = []
    bestie (i := 0; i < 5; i++) {
        sus result := pool.get_result()
        assert_eq(result.err, nil, "Job should complete without error")
        results = append(results, result.result)
    }
    
    assert_eq(len(results), 5, "Should have 5 results")
    
    test_pass("Worker pool works correctly")
}

slay test_pipeline() {
    test_group("Pipeline")
    
    sus ctx, cancel := with_timeout(background(), 1 * time.Second)
    defer cancel()
    
    sus input := make_channel<drip>()
    
    # Pipeline stage that doubles the input
    sus double_stage PipelineStage<drip, drip> = slay(ctx Context, x drip) (drip, yikes<tea>) {
        ready (is_cancelled(ctx)) {
            damn 0, ctx.err()
        }
        damn x * 2, nil
    }
    
    sus output := pipeline(ctx, input, double_stage)
    
    # Send test data
    go {
        bestie (i := 1; i <= 3; i++) {
            input <- i
        }
        close(input)
    }()
    
    # Collect results
    sus results drip[value] = []
    bestie (result := range output) {
        results = append(results, result)
    }
    
    assert_eq(len(results), 3, "Should have 3 results")
    assert_eq(results[0], 2, "Should double first input")
    assert_eq(results[1], 4, "Should double second input")
    assert_eq(results[2], 6, "Should double third input")
    
    test_pass("Pipeline works correctly")
}

slay test_rate_limiter() {
    test_group("Rate Limiter")
    
    sus ctx, cancel := with_timeout(background(), 1 * time.Second)
    defer cancel()
    
    sus limiter := new_rate_limiter(2, ctx)  # 2 tokens per second
    
    # Should acquire tokens initially
    assert_eq(limiter.acquire(), based, "Should acquire first token")
    assert_eq(limiter.acquire(), based, "Should acquire second token")
    
    # Third acquisition should block briefly but succeed
    sus start := time.now()
    sus acquired := limiter.acquire()
    sus elapsed := time.since(start)
    
    assert_eq(acquired, based, "Should eventually acquire third token")
    assert_ge(elapsed, 400 * time.Millisecond, "Should wait for token refill")
    
    test_pass("Rate limiter works correctly")
}

slay test_barrier() {
    test_group("Barrier")
    
    sus ctx, cancel := with_timeout(background(), 1 * time.Second)
    defer cancel()
    
    sus barrier := new_barrier(3, ctx)
    sus completed_count := 0
    sus mu sync.Mutex
    
    # Start 3 goroutines
    bestie (i := 0; i < 3; i++) {
        go {
            time.sleep(time.Duration(i * 10) * time.Millisecond)  # Stagger arrivals
            sus success := barrier.wait()
            
            ready (success) {
                mu.lock()
                completed_count++
                mu.unlock()
            }
        }()
    }
    
    time.sleep(100 * time.Millisecond)  # Wait for all to complete
    
    mu.lock()
    sus final_count := completed_count
    mu.unlock()
    
    assert_eq(final_count, 3, "All goroutines should complete barrier")
    
    test_pass("Barrier works correctly")
}

# Performance tests

slay test_context_performance() {
    test_group("Context Performance")
    
    sus iterations drip = 10000
    sus start := time.now()
    
    # Test context creation performance
    bestie (i := 0; i < iterations; i++) {
        sus ctx, cancel := with_cancel(background())
        cancel()
    }
    
    sus elapsed := time.since(start)
    sus per_iteration := elapsed / time.Duration(iterations)
    
    # Should be fast (less than 1μs per operation)
    assert_le(per_iteration, 1 * time.Microsecond, "Context creation should be fast")
    
    vibez.spill("Context creation:", per_iteration, "per operation")
    
    test_pass("Context performance is acceptable")
}

slay test_memory_usage() {
    test_group("Memory Usage")
    
    # Create many contexts and ensure they're properly cleaned up
    sus contexts Context[value] = []
    
    bestie (i := 0; i < 1000; i++) {
        sus ctx, _ := with_cancel(background())
        contexts = append(contexts, ctx)
    }
    
    # Force garbage collection
    runtime.gc()
    
    # Cancel all contexts
    bestie (_, ctx in contexts) {
        ready (cancel_ctx := ctx.(*CancelContext); cancel_ctx != nil) {
            cancel_ctx.cancel(yikes("test cleanup"))
        }
    }
    
    # Clear references
    contexts = nil
    runtime.gc()
    
    test_pass("Memory usage test completed")
}

# Main test runner
slay main_character() {
    test_contextz_comprehensive()
}
