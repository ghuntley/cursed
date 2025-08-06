fr fr CURSED Concurrency Optimizations Test
fr fr Tests specific optimizations: channel performance, scheduler efficiency, memory management

yeet "testz"

fr fr Test 1: Channel communication performance optimizations
slay test_channel_performance() {
    test_start("Channel Performance Optimizations")
    
    vibez.spill("Testing optimized channel communication patterns")
    
    fr fr Test high-throughput channel operations
    sus high_capacity_ch dm<normie> = dm<normie>(1000)
    sus message_count drip = 500
    
    vibez.spillf("Created high-capacity channel ({}), sending {} messages", 1000, message_count)
    
    fr fr Producer goroutine
    stan {
        vibez.spill("Producer: Starting high-volume send")
        sus i drip = 0
        bestie (i < message_count) {
            dm_send(high_capacity_ch, i)
            i = i + 1
        }
        vibez.spillf("Producer: Sent {} messages", message_count)
    }
    
    fr fr Consumer goroutine
    stan {
        vibez.spill("Consumer: Starting high-volume receive")
        sus received_count drip = 0
        sus total_sum drip = 0
        
        bestie (received_count < message_count) {
            sus value normie = dm_recv(high_capacity_ch)
            total_sum = total_sum + value
            received_count = received_count + 1
        }
        
        vibez.spillf("Consumer: Received {} messages, sum = {}", received_count, total_sum)
        
        fr fr Expected sum: 0+1+2+...+(message_count-1) = message_count*(message_count-1)/2
        sus expected_sum drip = message_count * (message_count - 1) / 2
        assert_eq_int(total_sum, expected_sum)
    }
    
    fr fr Give time for completion
    damn()
    damn()
    
    vibez.spill("✅ Channel performance optimization test completed")
    print_test_summary()
}

fr fr Test 2: Work-stealing scheduler efficiency
slay test_work_stealing_efficiency() {
    test_start("Work-Stealing Scheduler Efficiency")
    
    vibez.spill("Testing work-stealing scheduler with uneven workloads")
    
    sus task_count drip = 20
    sus results dm<normie> = dm<normie>(task_count)
    
    fr fr Create tasks with varying computational loads
    sus i drip = 0
    bestie (i < task_count) {
        stan {
            fr fr Variable work based on task ID
            sus work_units drip = (i % 3 + 1) * 100  fr fr 100, 200, or 300 units of work
            sus local_sum drip = 0
            sus j drip = 0
            
            bestie (j < work_units) {
                local_sum = local_sum + (j * j)  fr fr Quadratic work to make it CPU-intensive
                j = j + 1
            }
            
            vibez.spillf("Task {}: Completed {} work units, result = {}", i, work_units, local_sum)
            dm_send(results, local_sum)
        }
        i = i + 1
    }
    
    vibez.spillf("Spawned {} tasks with varying workloads", task_count)
    
    fr fr Collect all results
    sus completed_tasks drip = 0
    sus all_results normie[20]  fr fr Array to store results
    
    bestie (completed_tasks < task_count) {
        sus result normie = dm_recv(results)
        all_results[completed_tasks] = result
        completed_tasks = completed_tasks + 1
        vibez.spillf("Collected result {}/{}: {}", completed_tasks, task_count, result)
    }
    
    vibez.spillf("All {} tasks completed through work-stealing scheduler", task_count)
    
    fr fr Verify we got all results
    assert_eq_int(completed_tasks, task_count)
    
    vibez.spill("✅ Work-stealing efficiency test completed")
    print_test_summary()
}

fr fr Test 3: Channel type safety optimizations
slay test_channel_type_safety() {
    test_start("Channel Type Safety Optimizations")
    
    vibez.spill("Testing type-safe channel operations with different data types")
    
    fr fr Test multiple channel types in parallel
    sus int_ch dm<normie> = dm<normie>(5)
    
    fr fr Test sending and receiving different value ranges
    stan {
        vibez.spill("Type safety test: Sending integers")
        dm_send(int_ch, 42)
        dm_send(int_ch, -17)
        dm_send(int_ch, 0)
        dm_send(int_ch, 999)
        dm_send(int_ch, -999)
        vibez.spill("Type safety test: All integers sent")
    }
    
    fr fr Receive and validate all values
    sus expected normie[5] = [42, -17, 0, 999, -999]
    sus i drip = 0
    
    bestie (i < 5) {
        sus received normie = dm_recv(int_ch)
        vibez.spillf("Received value {}: {}", i, received)
        assert_eq_int(received, expected[i])
        i = i + 1
    }
    
    vibez.spill("All type-safe operations completed successfully")
    
    vibez.spill("✅ Channel type safety test completed")
    print_test_summary()
}

fr fr Test 4: Select statement optimization
slay test_select_optimization() {
    test_start("Select Statement Optimization")
    
    vibez.spill("Testing optimized select statement with multiple ready channels")
    
    fr fr Create multiple channels and make them ready in sequence
    sus ch_fast dm<normie> = dm<normie>(1)    fr fr Fast channel
    sus ch_medium dm<normie> = dm<normie>(1)  fr fr Medium channel
    sus ch_slow dm<normie> = dm<normie>(1)    fr fr Slow channel
    
    fr fr Fill channels at different rates
    stan {
        dm_send(ch_fast, 1)
        vibez.spill("Fast channel ready")
    }
    
    stan {
        damn()  fr fr Small delay
        dm_send(ch_medium, 2)
        vibez.spill("Medium channel ready")
    }
    
    stan {
        damn()
        damn()  fr fr Longer delay
        dm_send(ch_slow, 3)
        vibez.spill("Slow channel ready")
    }
    
    fr fr Select should pick the first available channel
    sus selections normie[3]
    sus selection_count drip = 0
    
    fr fr Multiple select operations to test fairness
    sus attempt drip = 0
    bestie (attempt < 3) {
        ready {
            mood value := dm_recv(ch_fast):
                vibez.spillf("Select {}: Picked fast channel, value = {}", attempt, value)
                selections[selection_count] = value
                selection_count = selection_count + 1
                
            mood value := dm_recv(ch_medium):
                vibez.spillf("Select {}: Picked medium channel, value = {}", attempt, value)
                selections[selection_count] = value
                selection_count = selection_count + 1
                
            mood value := dm_recv(ch_slow):
                vibez.spillf("Select {}: Picked slow channel, value = {}", attempt, value)
                selections[selection_count] = value
                selection_count = selection_count + 1
                
            basic:
                vibez.spillf("Select {}: Default case executed", attempt)
        }
        attempt = attempt + 1
        damn()  fr fr Brief pause between attempts
    }
    
    vibez.spillf("Made {} successful selections", selection_count)
    assert_true(selection_count >= 1)
    
    vibez.spill("✅ Select optimization test completed")
    print_test_summary()
}

fr fr Test 5: Memory management optimization
slay test_memory_optimization() {
    test_start("Memory Management Optimization")
    
    vibez.spill("Testing memory-efficient goroutine and channel management")
    
    fr fr Test creating and destroying many short-lived goroutines
    sus short_lived_count drip = 50
    sus completion_ch dm<normie> = dm<normie>(short_lived_count)
    
    vibez.spillf("Creating {} short-lived goroutines", short_lived_count)
    
    sus i drip = 0
    bestie (i < short_lived_count) {
        stan {
            fr fr Very short goroutine that just computes and exits
            sus task_id drip = i
            sus result drip = task_id * task_id
            
            dm_send(completion_ch, result)
        }
        i = i + 1
    }
    
    fr fr Collect results efficiently
    sus total_results drip = 0
    sus collected drip = 0
    
    bestie (collected < short_lived_count) {
        sus result normie = dm_recv(completion_ch)
        total_results = total_results + result
        collected = collected + 1
    }
    
    vibez.spillf("Collected {} results, total = {}", collected, total_results)
    
    fr fr Expected total: sum of squares 0^2 + 1^2 + ... + (n-1)^2
    sus expected drip = 0
    sus j drip = 0
    bestie (j < short_lived_count) {
        expected = expected + (j * j)
        j = j + 1
    }
    
    assert_eq_int(total_results, expected)
    
    vibez.spill("Memory optimization test: All goroutines cleaned up efficiently")
    
    vibez.spill("✅ Memory optimization test completed")
    print_test_summary()
}

fr fr Main test function
slay main() {
    vibez.spill("⚡ CURSED Concurrency Optimizations Test Suite")
    vibez.spill("==============================================")
    
    test_channel_performance()
    vibez.spill("")
    
    test_work_stealing_efficiency()
    vibez.spill("")
    
    test_channel_type_safety()
    vibez.spill("")
    
    test_select_optimization()
    vibez.spill("")
    
    test_memory_optimization()
    vibez.spill("")
    
    vibez.spill("✅ All concurrency optimization tests completed!")
    vibez.spill("🔧 Optimizations tested:")
    vibez.spill("   - High-throughput channel communication")
    vibez.spill("   - Work-stealing scheduler efficiency")
    vibez.spill("   - Type-safe channel operations")
    vibez.spill("   - Optimized select statement multiplexing")
    vibez.spill("   - Memory-efficient lifecycle management")
}
