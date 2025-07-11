# Test program to validate GC performance improvements
# This program creates allocation patterns similar to web server workloads

yeet "testz"
yeet "time"

# Simulate web server request handling
slay simulate_web_requests(num_requests normie) {
    vibez.spill("Starting web server simulation with", num_requests, "requests")
    
    sus total_pause_time drip = 0.0
    sus max_pause_time drip = 0.0
    sus pause_count normie = 0
    
    bestie i := 0; i < num_requests; i++ {
        # Simulate request processing allocation
        sus request_body tea = "Lorem ipsum dolor sit amet, consectetur adipiscing elit"
        sus headers := make_headers()
        sus response_data := process_request(request_body)
        
        # Force GC every 50 requests to measure pause times
        if (i % 50) == 0 {
            sus start_time := time.now()
            
            # Trigger garbage collection
            runtime.gc()
            
            sus end_time := time.now()
            sus pause_time := time.diff_ms(end_time, start_time)
            
            total_pause_time = total_pause_time + pause_time
            pause_count = pause_count + 1
            
            if pause_time > max_pause_time {
                max_pause_time = pause_time
            }
            
            vibez.spill("GC pause", i/50, ":", pause_time, "ms")
        }
    }
    
    sus avg_pause_time := total_pause_time / pause_count
    vibez.spill("Average GC pause time:", avg_pause_time, "ms")
    vibez.spill("Maximum GC pause time:", max_pause_time, "ms")
    
    # Validate performance targets
    if max_pause_time > 100.0 {
        vibez.spill("❌ FAIL: Max pause time exceeds 100ms target")
    } else {
        vibez.spill("✅ PASS: Max pause time meets <100ms target")
    }
    
    if avg_pause_time > 50.0 {
        vibez.spill("❌ FAIL: Average pause time exceeds 50ms target")
    } else {
        vibez.spill("✅ PASS: Average pause time meets <50ms target")
    }
}

# Create request headers map
slay make_headers() {
    sus headers := make(map[tea]tea)
    headers["Content-Type"] = "application/json"
    headers["User-Agent"] = "CURSED-Client/1.0"
    headers["Accept"] = "application/json, text/plain"
    headers["Authorization"] = "Bearer token123456789"
    damn headers
}

# Process request and generate response
slay process_request(body tea) tea {
    # Simulate JSON parsing and processing
    sus parsed_data := parse_json(body)
    sus processed_data := transform_data(parsed_data)
    sus response := generate_response(processed_data)
    damn response
}

# Simulate JSON parsing (creates temporary objects)
slay parse_json(json_str tea) {
    sus result := make(map[tea]interface{})
    
    # Simulate parsing by creating temporary objects
    bestie i := 0; i < 10; i++ {
        sus temp_key := "key_" + string(i)
        sus temp_value := "value_" + string(i)
        result[temp_key] = temp_value
    }
    
    damn result
}

# Transform data (creates intermediate objects)
slay transform_data(data interface{}) interface{} {
    sus transformed := make(map[tea]interface{})
    
    # Simulate data transformation
    bestie i := 0; i < 20; i++ {
        sus field_name := "field_" + string(i)
        sus field_value := "transformed_" + string(i)
        transformed[field_name] = field_value
    }
    
    damn transformed
}

# Generate response (creates output objects)
slay generate_response(data interface{}) tea {
    sus response := "{"
    
    # Simulate JSON generation
    bestie i := 0; i < 15; i++ {
        sus field := "\"response_field_" + string(i) + "\": \"response_value_" + string(i) + "\""
        if i > 0 {
            response = response + ", "
        }
        response = response + field
    }
    
    response = response + "}"
    damn response
}

# Test concurrent allocation patterns
slay test_concurrent_allocation() {
    vibez.spill("Testing concurrent allocation patterns...")
    
    sus channels := make(chan normie, 4)
    
    # Start 4 goroutines doing allocation
    yolo worker_goroutine(channels, 1)
    yolo worker_goroutine(channels, 2)
    yolo worker_goroutine(channels, 3)
    yolo worker_goroutine(channels, 4)
    
    # Wait for all workers to complete
    bestie i := 0; i < 4; i++ {
        sus worker_id := <-channels
        vibez.spill("Worker", worker_id, "completed")
    }
    
    vibez.spill("All concurrent workers completed")
}

# Worker goroutine that performs allocations
slay worker_goroutine(result_chan chan normie, worker_id normie) {
    vibez.spill("Worker", worker_id, "started")
    
    # Each worker performs different allocation patterns
    if worker_id == 1 {
        # Small frequent allocations
        bestie i := 0; i < 1000; i++ {
            sus data := make([]normie, 10)
            bestie j := 0; j < 10; j++ {
                data[j] = i + j
            }
        }
    } else if worker_id == 2 {
        # Medium allocations
        bestie i := 0; i < 500; i++ {
            sus data := make([]tea, 100)
            bestie j := 0; j < 100; j++ {
                data[j] = "worker2_data_" + string(j)
            }
        }
    } else if worker_id == 3 {
        # Large allocations
        bestie i := 0; i < 100; i++ {
            sus data := make([]normie, 1000)
            bestie j := 0; j < 1000; j++ {
                data[j] = i * 1000 + j
            }
        }
    } else {
        # Mixed allocation sizes
        bestie i := 0; i < 200; i++ {
            if (i % 3) == 0 {
                sus small := make([]normie, 5)
                small[0] = i
            } else if (i % 3) == 1 {
                sus medium := make([]tea, 50)
                medium[0] = "mixed_" + string(i)
            } else {
                sus large := make([]normie, 500)
                large[0] = i
            }
        }
    }
    
    result_chan <- worker_id
}

# Test memory pressure scenarios
slay test_memory_pressure() {
    vibez.spill("Testing memory pressure scenarios...")
    
    sus large_objects := make([][]normie, 100)
    
    # Allocate increasingly large objects
    bestie i := 0; i < 100; i++ {
        sus size := (i + 1) * 1000
        large_objects[i] = make([]normie, size)
        
        # Fill with data
        bestie j := 0; j < size; j++ {
            large_objects[i][j] = j
        }
        
        # Trigger GC every 10 large allocations
        if (i % 10) == 0 {
            sus start_time := time.now()
            runtime.gc()
            sus end_time := time.now()
            sus pause_time := time.diff_ms(end_time, start_time)
            
            vibez.spill("Memory pressure GC pause", i/10, ":", pause_time, "ms")
            
            if pause_time > 200.0 {
                vibez.spill("⚠️  WARNING: High memory pressure causing long GC pauses")
            }
        }
    }
    
    vibez.spill("Memory pressure test completed")
}

# Main test function
slay test_gc_performance() {
    vibez.spill("=== GC Performance Test Suite ===")
    
    test_start("GC Performance Validation")
    
    # Test 1: Web server workload
    vibez.spill("\n--- Test 1: Web Server Workload ---")
    simulate_web_requests(1000)
    
    # Test 2: Concurrent allocation
    vibez.spill("\n--- Test 2: Concurrent Allocation ---")
    test_concurrent_allocation()
    
    # Test 3: Memory pressure
    vibez.spill("\n--- Test 3: Memory Pressure ---")
    test_memory_pressure()
    
    vibez.spill("\n=== GC Performance Tests Complete ===")
    
    print_test_summary()
}

# Run the performance tests
test_gc_performance()
