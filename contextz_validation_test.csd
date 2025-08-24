# Context Package Validation Test
# Comprehensive test of the contextz stdlib package

yeet "stdlib/contextz/context"
yeet "stdlib/contextz/propagation" 
yeet "stdlib/contextz/integration"
yeet "stdlib/contextz/test"
yeet "testz"
yeet "timez"
yeet "concurrenz"

slay main() {
    vibez.spill("=== CURSED Context Package Validation ===")
    
    # Run comprehensive context tests
    test_contextz_comprehensive()
    
    # Additional integration validation
    test_context_integration_scenarios()
    
    vibez.spill("=== Context Package Validation Complete ===")
}

slay test_context_integration_scenarios() {
    test_start("Context Integration Scenarios")
    
    # Test 1: HTTP-like request processing with context
    test_http_request_simulation()
    
    # Test 2: Database transaction with timeout
    test_database_transaction_simulation()
    
    # Test 3: Microservice call chain with cancellation
    test_microservice_chain_simulation()
    
    # Test 4: Batch processing with context cancellation
    test_batch_processing_simulation()
    
    # Test 5: Real-world timeout scenarios
    test_real_world_timeout_scenarios()
    
    print_test_summary()
}

slay test_http_request_simulation() {
    test_group("HTTP Request Simulation")
    
    # Simulate HTTP request processing with context
    sus request_id := "req_123"
    sus user_id := "user_456"
    
    sus ctx := background()
    ctx = with_value(ctx, "request_id", request_id)
    ctx = with_value(ctx, "user_id", user_id)
    
    # Add request timeout
    ctx, cancel := with_timeout(ctx, 500 * time.Millisecond)
    defer cancel()
    
    # Simulate request processing pipeline
    sus success := process_http_request(ctx, "GET /api/users")
    
    assert_eq(success, based, "HTTP request should complete successfully")
    test_pass("HTTP request simulation completed")
}

slay process_http_request(ctx Context, request tea) lit {
    # Validate context values
    sus req_id := ctx.value("request_id").(tea)
    sus user_id := ctx.value("user_id").(tea)
    
    vibez.spill("Processing request", req_id, "for user", user_id)
    
    # Simulate database lookup
    ready (!simulate_database_query(ctx, "SELECT * FROM users")) {
        vibez.spill("Database query cancelled")
        damn faux
    }
    
    # Simulate external API call
    ready (!simulate_api_call(ctx, "GET /external/api")) {
        vibez.spill("API call cancelled")
        damn faux
    }
    
    # Simulate response preparation
    time.sleep(50 * time.Millisecond)
    
    ready (is_cancelled(ctx)) {
        vibez.spill("Request processing cancelled")
        damn faux
    }
    
    vibez.spill("Request processing completed")
    damn based
}

slay simulate_database_query(ctx Context, query tea) lit {
    vibez.spill("Executing query:", query)
    
    # Simulate database delay
    sus delay := 100 * time.Millisecond
    ready (!sleep_with_cancel(ctx, delay)) {
        vibez.spill("Database query timed out")
        damn faux
    }
    
    vibez.spill("Database query completed")
    damn based
}

slay simulate_api_call(ctx Context, endpoint tea) lit {
    vibez.spill("Calling API:", endpoint)
    
    # Simulate network delay
    sus delay := 150 * time.Millisecond
    ready (!sleep_with_cancel(ctx, delay)) {
        vibez.spill("API call timed out")
        damn faux
    }
    
    vibez.spill("API call completed")
    damn based
}

slay test_database_transaction_simulation() {
    test_group("Database Transaction Simulation")
    
    # Create context with transaction timeout
    sus ctx, cancel := with_timeout(background(), 200 * time.Millisecond)
    defer cancel()
    
    sus success := execute_transaction(ctx)
    
    # This should timeout since transaction takes 300ms but timeout is 200ms
    assert_eq(success, faux, "Transaction should timeout")
    test_pass("Database transaction timeout works correctly")
}

slay execute_transaction(ctx Context) lit {
    vibez.spill("Starting database transaction")
    
    # Begin transaction
    ready (is_cancelled(ctx)) {
        damn faux
    }
    
    # Execute multiple queries
    bestie (i := 0; i < 3; i++) {
        vibez.spill("Executing query", i + 1)
        
        # Each query takes 100ms (total 300ms)
        ready (!sleep_with_cancel(ctx, 100 * time.Millisecond)) {
            vibez.spill("Transaction cancelled during query", i + 1)
            damn faux
        }
    }
    
    vibez.spill("Transaction completed successfully")
    damn based
}

slay test_microservice_chain_simulation() {
    test_group("Microservice Chain Simulation")
    
    # Create root context for entire request
    sus root_ctx := with_value(background(), "trace_id", "trace_789")
    root_ctx, cancel := with_timeout(root_ctx, 1 * time.Second)
    defer cancel()
    
    # Call service chain
    sus result := call_service_a(root_ctx)
    
    assert_ne(result, nil, "Service chain should return result")
    test_pass("Microservice chain simulation completed")
}

slay call_service_a(ctx Context) tea {
    vibez.spill("Service A: Processing request")
    
    # Add service-specific timeout
    sus service_ctx, cancel := with_timeout(ctx, 800 * time.Millisecond)
    defer cancel()
    
    # Call downstream service
    sus result := call_service_b(service_ctx)
    ready (result == nil) {
        damn nil
    }
    
    # Process result
    time.sleep(50 * time.Millisecond)
    
    damn "service_a_" + result.(tea)
}

slay call_service_b(ctx Context) tea {
    vibez.spill("Service B: Processing request")
    
    # Add service-specific timeout
    sus service_ctx, cancel := with_timeout(ctx, 600 * time.Millisecond)
    defer cancel()
    
    # Call downstream service
    sus result := call_service_c(service_ctx)
    ready (result == nil) {
        damn nil
    }
    
    # Process result
    time.sleep(50 * time.Millisecond)
    
    damn "service_b_" + result.(tea)
}

slay call_service_c(ctx Context) tea {
    vibez.spill("Service C: Processing request")
    
    # Final service - do actual work
    ready (!sleep_with_cancel(ctx, 100 * time.Millisecond)) {
        damn nil
    }
    
    damn "service_c_result"
}

slay test_batch_processing_simulation() {
    test_group("Batch Processing Simulation")
    
    # Create context for batch processing with timeout
    sus ctx, cancel := with_timeout(background(), 2 * time.Second)
    defer cancel()
    
    # Process batch of items
    sus items []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus processed_count := process_batch(ctx, items)
    
    # Should process all items within timeout
    assert_eq(processed_count, 10, "Should process all items")
    test_pass("Batch processing simulation completed")
}

slay process_batch(ctx Context, items []drip) drip {
    vibez.spill("Starting batch processing of", len(items), "items")
    
    sus processed drip = 0
    
    bestie (_, item in items) {
        ready (is_cancelled(ctx)) {
            vibez.spill("Batch processing cancelled after", processed, "items")
            break
        }
        
        # Process each item
        ready (process_item(ctx, item)) {
            processed++
        }
    }
    
    vibez.spill("Batch processing completed:", processed, "items processed")
    damn processed
}

slay process_item(ctx Context, item drip) lit {
    # Each item takes 100ms to process
    ready (!sleep_with_cancel(ctx, 100 * time.Millisecond)) {
        damn faux
    }
    
    vibez.spill("Processed item:", item)
    damn based
}

slay test_real_world_timeout_scenarios() {
    test_group("Real World Timeout Scenarios")
    
    # Test scenario 1: Quick operation should complete
    test_quick_operation()
    
    # Test scenario 2: Slow operation should timeout
    test_slow_operation()
    
    # Test scenario 3: Manual cancellation
    test_manual_cancellation()
    
    test_pass("Real world timeout scenarios completed")
}

slay test_quick_operation() {
    vibez.spill("Testing quick operation...")
    
    sus result, completed := with_timeout_func(500 * time.Millisecond, slay() tea {
        time.sleep(100 * time.Millisecond)
        damn "quick_result"
    })
    
    assert_eq(completed, based, "Quick operation should complete")
    assert_eq(result, "quick_result", "Should return correct result")
}

slay test_slow_operation() {
    vibez.spill("Testing slow operation...")
    
    sus result, completed := with_timeout_func(200 * time.Millisecond, slay() tea {
        time.sleep(500 * time.Millisecond)
        damn "slow_result"
    })
    
    assert_eq(completed, faux, "Slow operation should timeout")
    assert_eq(result, nil, "Should return nil on timeout")
}

slay test_manual_cancellation() {
    vibez.spill("Testing manual cancellation...")
    
    sus ctx, cancel := with_timeout(background(), 1 * time.Second)
    sus result_chan := make_channel<tea>()
    
    # Start long operation
    go {
        time.sleep(2 * time.Second)  # This would timeout
        result_chan <- "completed"
    }()
    
    # Cancel after short delay
    go {
        time.sleep(100 * time.Millisecond)
        cancel()
    }()
    
    # Wait for either completion or cancellation
    sick {
        when result := <-result_chan -> {
            test_fail("Operation should have been cancelled")
        }
        when <-ctx.done() -> {
            assert_ne(ctx.err(), nil, "Should have cancellation error")
            vibez.spill("Operation cancelled as expected")
        }
    }
}
