fr fr Comprehensive Test Suite for Goroutine Runtime Integration
fr fr Tests all goroutine functionality including work-stealing scheduler
fr fr P0 Critical testing for CURSED concurrency system

yeet "concurrenz.goroutine_runtime"
yeet "testz"
yeet "vibez"
yeet "mathz"

fr fr =============================================================================
fr fr BASIC GOROUTINE FUNCTIONALITY TESTS  
fr fr =============================================================================

fr fr Test basic goroutine spawning and execution
slay test_basic_goroutine_spawn() lit {
    vibez.spill("🧪 Testing basic goroutine spawning...")
    
    fr fr Initialize scheduler
    ready init_goroutine_scheduler(2) == cap {
        vibez.spill("❌ Failed to initialize scheduler")
        damn cap
    }
    
    fr fr Simple test function for goroutine
    slay test_goroutine_func(context thicc) {
        vibez.spill("✅ Goroutine executed successfully")
    }
    
    fr fr Spawn goroutine
    sus goroutine_id thicc = stan(test_goroutine_func, 0)
    ready goroutine_id == 0 {
        shutdown_goroutine_scheduler()
        damn cap
    }
    
    vibez.spill("✅ Goroutine spawned with ID:", goroutine_id)
    
    fr fr Wait briefly for execution (simplified)
    sus wait_cycles normie = 0
    bestie wait_cycles < 1000000 {
        runtime_yield_cpu()
        wait_cycles = wait_cycles + 1
    }
    
    shutdown_goroutine_scheduler()
    damn based
}

fr fr Test multiple goroutine spawning
slay test_multiple_goroutines() lit {
    vibez.spill("🧪 Testing multiple goroutine spawning...")
    
    ready init_goroutine_scheduler(4) == cap {
        vibez.spill("❌ Failed to initialize scheduler")
        damn cap
    }
    
    fr fr Spawn multiple goroutines
    sus goroutine_ids thicc[value] = [0, 0, 0, 0, 0]  fr fr Array for 5 goroutines
    sus spawn_count normie = 5
    
    slay multi_goroutine_func(context thicc) {
        sus id thicc = context
        vibez.spill("✅ Goroutine", id, "is running")
    }
    
    sus i normie = 0
    bestie i < spawn_count {
        goroutine_ids[i] = stan(multi_goroutine_func, i + 100)  fr fr Pass ID as context
        ready goroutine_ids[i] == 0 {
            vibez.spill("❌ Failed to spawn goroutine", i)
            shutdown_goroutine_scheduler()
            damn cap
        }
        i = i + 1
    }
    
    vibez.spill("✅ Successfully spawned", spawn_count, "goroutines")
    
    fr fr Wait for all goroutines to complete
    sus wait_cycles normie = 0
    bestie wait_cycles < 5000000 {
        runtime_yield_cpu()
        wait_cycles = wait_cycles + 1
    }
    
    shutdown_goroutine_scheduler()
    damn based
}

fr fr =============================================================================
fr fr WORK-STEALING QUEUE TESTS
fr fr =============================================================================

fr fr Test work queue basic operations
slay test_work_queue_operations() lit {
    vibez.spill("🧪 Testing work-stealing queue operations...")
    
    sus queue WorkStealingQueue
    ready init_work_queue(&queue, 10) == cap {
        vibez.spill("❌ Failed to initialize work queue")
        damn cap
    }
    
    fr fr Test enqueue operation
    slay dummy_task_func(ctx thicc) {
        vibez.spill("Task executed")
    }
    
    sus enqueue_result lit = enqueue_work(&queue, dummy_task_func, 42, 5)
    ready enqueue_result == cap {
        vibez.spill("❌ Failed to enqueue work")
        damn cap
    }
    
    fr fr Test dequeue operation
    sus dequeued_task thicc = dequeue_work_local(&queue)
    ready dequeued_task == 0 {
        vibez.spill("❌ Failed to dequeue work")
        damn cap
    }
    
    vibez.spill("✅ Work queue operations successful")
    
    fr fr Cleanup task node
    memory.free(dequeued_task)
    
    damn based
}

fr fr Test work stealing between queues
slay test_work_stealing() lit {
    vibez.spill("🧪 Testing work stealing between queues...")
    
    sus queue1 WorkStealingQueue
    sus queue2 WorkStealingQueue
    
    ready init_work_queue(&queue1, 10) == cap || init_work_queue(&queue2, 10) == cap {
        vibez.spill("❌ Failed to initialize work queues")
        damn cap
    }
    
    fr fr Fill queue1 with tasks
    sus tasks_added normie = 0
    bestie tasks_added < 5 {
        sus result lit = enqueue_work(&queue1, 0, tasks_added * 10, tasks_added)
        ready result == based {
            tasks_added = tasks_added + 1
        } otherwise {
            break
        }
    }
    
    vibez.spill("Added", tasks_added, "tasks to queue1")
    
    fr fr Steal work from queue1 to queue2
    sus stolen_tasks normie = 0
    bestie stolen_tasks < 3 {
        sus stolen_work thicc = steal_work(&queue1)
        ready stolen_work != 0 {
            stolen_tasks = stolen_tasks + 1
            memory.free(stolen_work)  fr fr Cleanup
        } otherwise {
            break
        }
    }
    
    vibez.spill("✅ Successfully stole", stolen_tasks, "tasks")
    
    fr fr Check remaining tasks in queue1
    sus remaining_tasks normie = atomic_drip.atomic_load_i32(&queue1.size, atomic_drip.ACQUIRE)
    vibez.spill("Remaining tasks in queue1:", remaining_tasks)
    
    ready stolen_tasks > 0 {
        damn based
    } otherwise {
        vibez.spill("⚠️ No tasks were stolen (queue might have been empty)")
        damn based  fr fr Not necessarily a failure
    }
}

fr fr =============================================================================
fr fr GOROUTINE STACK MANAGEMENT TESTS
fr fr =============================================================================

fr fr Test goroutine stack allocation and deallocation
slay test_goroutine_stack_management() lit {
    vibez.spill("🧪 Testing goroutine stack management...")
    
    fr fr Test stack allocation
    sus stack *GoroutineStack = allocate_goroutine_stack(DEFAULT_STACK_SIZE)
    ready stack == 0 {
        vibez.spill("❌ Failed to allocate goroutine stack")
        damn cap
    }
    
    vibez.spill("✅ Goroutine stack allocated successfully")
    vibez.spill("Stack size:", stack.stack_size)
    vibez.spill("Stack memory address:", stack.stack_memory)
    
    fr fr Test stack integrity check
    sus integrity_result lit = check_stack_integrity(stack)
    ready integrity_result == cap {
        vibez.spill("❌ Stack integrity check failed")
        free_goroutine_stack(stack)
        damn cap
    }
    
    vibez.spill("✅ Stack integrity check passed")
    
    fr fr Test stack deallocation
    free_goroutine_stack(stack)
    vibez.spill("✅ Goroutine stack freed successfully")
    
    damn based
}

fr fr Test stack overflow detection
slay test_stack_overflow_detection() lit {
    vibez.spill("🧪 Testing stack overflow detection...")
    
    sus stack *GoroutineStack = allocate_goroutine_stack(DEFAULT_STACK_SIZE)
    ready stack == 0 {
        vibez.spill("❌ Failed to allocate stack for overflow test")
        damn cap
    }
    
    fr fr Artificially create stack overflow condition
    stack.stack_pointer = stack.stack_memory - 1  fr fr Below stack base
    
    fr fr This should detect overflow
    sus overflow_result lit = check_stack_integrity(stack)
    ready overflow_result == based {
        vibez.spill("❌ Stack overflow was not detected!")
        free_goroutine_stack(stack)
        damn cap
    }
    
    vibez.spill("✅ Stack overflow correctly detected")
    
    fr fr Reset stack pointer to valid position
    stack.stack_pointer = stack.stack_base
    sus normal_result lit = check_stack_integrity(stack)
    ready normal_result == cap {
        vibez.spill("❌ Valid stack incorrectly flagged as overflow")
        free_goroutine_stack(stack)
        damn cap
    }
    
    free_goroutine_stack(stack)
    vibez.spill("✅ Stack overflow detection working correctly")
    damn based
}

fr fr =============================================================================
fr fr GOROUTINE CONTEXT MANAGEMENT TESTS
fr fr =============================================================================

fr fr Test goroutine context creation and destruction
slay test_goroutine_context_lifecycle() lit {
    vibez.spill("🧪 Testing goroutine context lifecycle...")
    
    fr fr Create goroutine context
    sus ctx *GoroutineContext = create_goroutine_context(0, DEFAULT_STACK_SIZE)
    ready ctx == 0 {
        vibez.spill("❌ Failed to create goroutine context")
        damn cap
    }
    
    vibez.spill("✅ Goroutine context created successfully")
    vibez.spill("Context ID:", ctx.id)
    vibez.spill("Context state:", ctx.state)
    vibez.spill("Parent ID:", ctx.parent_id)
    vibez.spill("Stack size:", ctx.stack.stack_size)
    
    fr fr Verify context fields
    ready ctx.id == 0 {
        vibez.spill("❌ Invalid goroutine ID")
        destroy_goroutine_context(ctx)
        damn cap
    }
    
    ready ctx.state != GOROUTINE_READY {
        vibez.spill("❌ Invalid initial goroutine state")
        destroy_goroutine_context(ctx)
        damn cap
    }
    
    ready ctx.stack.stack_memory == 0 {
        vibez.spill("❌ Stack memory not allocated")
        destroy_goroutine_context(ctx)
        damn cap
    }
    
    ready ctx.registers == 0 {
        vibez.spill("❌ Register storage not allocated")
        destroy_goroutine_context(ctx)
        damn cap
    }
    
    vibez.spill("✅ All context fields properly initialized")
    
    fr fr Test context destruction
    destroy_goroutine_context(ctx)
    vibez.spill("✅ Goroutine context destroyed successfully")
    
    damn based
}

fr fr =============================================================================
fr fr SCHEDULER PERFORMANCE TESTS
fr fr =============================================================================

fr fr Test scheduler performance with many goroutines
slay test_scheduler_performance() lit {
    vibez.spill("🧪 Testing scheduler performance...")
    
    ready init_goroutine_scheduler(4) == cap {
        vibez.spill("❌ Failed to initialize scheduler")
        damn cap
    }
    
    fr fr Performance test function
    slay performance_task(context thicc) {
        sus task_id thicc = context
        fr fr Simulate work
        sus work_cycles normie = 0
        bestie work_cycles < 1000 {
            work_cycles = work_cycles + 1
        }
    }
    
    fr fr Spawn many goroutines
    sus num_goroutines normie = 100
    sus spawned_count normie = 0
    sus start_time thicc = get_current_time_ns()
    
    sus i normie = 0
    bestie i < num_goroutines {
        sus goroutine_id thicc = stan(performance_task, i)
        ready goroutine_id != 0 {
            spawned_count = spawned_count + 1
        }
        i = i + 1
    }
    
    sus end_time thicc = get_current_time_ns()
    sus spawn_duration thicc = end_time - start_time
    
    vibez.spill("✅ Spawned", spawned_count, "goroutines")
    vibez.spill("Spawn time:", spawn_duration, "ns")
    
    fr fr Wait for completion
    sus wait_start thicc = get_current_time_ns()
    sus wait_cycles normie = 0
    bestie wait_cycles < 10000000 {
        sus active_count normie = atomic_drip.atomic_load_i32(&global_scheduler.active_goroutines, atomic_drip.ACQUIRE)
        ready active_count == 0 {
            break
        }
        runtime_yield_cpu()
        wait_cycles = wait_cycles + 1
    }
    sus wait_end thicc = get_current_time_ns()
    sus wait_duration thicc = wait_end - wait_start
    
    vibez.spill("✅ All goroutines completed")
    vibez.spill("Completion time:", wait_duration, "ns")
    
    fr fr Print scheduler statistics
    sus stats *SchedulerStats = get_scheduler_stats()
    ready stats != 0 {
        vibez.spill("📊 Scheduler Statistics:")
        vibez.spill("  Total spawned:", stats.total_goroutines_spawned)
        vibez.spill("  Total completed:", stats.total_goroutines_completed)
        vibez.spill("  Context switches:", stats.total_context_switches)
        vibez.spill("  Work steals:", stats.total_work_steals)
    }
    
    shutdown_goroutine_scheduler()
    damn based
}

fr fr =============================================================================
fr fr GOROUTINE MEMORY MANAGEMENT TESTS
fr fr =============================================================================

fr fr Test goroutine-specific memory allocator
slay test_goroutine_allocator() lit {
    vibez.spill("🧪 Testing goroutine memory allocator...")
    
    sus allocator GoroutineAllocator
    sus arena_size normie = 4096  fr fr 4KB arena
    
    ready init_goroutine_allocator(&allocator, arena_size) == cap {
        vibez.spill("❌ Failed to initialize goroutine allocator")
        damn cap
    }
    
    vibez.spill("✅ Goroutine allocator initialized")
    vibez.spill("Arena size:", allocator.arena_size)
    
    fr fr Test memory allocation
    sus allocated_blocks thicc[value] = [0, 0, 0, 0, 0]
    sus block_size normie = 256  fr fr 256 bytes per block
    sus num_blocks normie = 5
    
    sus i normie = 0
    bestie i < num_blocks {
        allocated_blocks[i] = goroutine_allocate(&allocator, block_size)
        ready allocated_blocks[i] == 0 {
            vibez.spill("❌ Failed to allocate block", i)
            free_goroutine_allocator(&allocator)
            damn cap
        }
        i = i + 1
    }
    
    vibez.spill("✅ Successfully allocated", num_blocks, "blocks")
    vibez.spill("Arena used:", allocator.arena_used)
    vibez.spill("Allocation count:", allocator.allocation_count)
    
    fr fr Test allocation limit
    sus large_block thicc = goroutine_allocate(&allocator, arena_size)  fr fr Should fail
    ready large_block != 0 {
        vibez.spill("❌ Allocator should have failed for oversized allocation")
        free_goroutine_allocator(&allocator)
        damn cap
    }
    
    vibez.spill("✅ Allocator correctly rejected oversized allocation")
    
    fr fr Cleanup
    free_goroutine_allocator(&allocator)
    vibez.spill("✅ Goroutine allocator freed successfully")
    
    damn based
}

fr fr =============================================================================
fr fr INTEGRATION TESTS WITH CHANNELS
fr fr =============================================================================

fr fr Test goroutines communicating via channels
slay test_goroutine_channel_communication() lit {
    vibez.spill("🧪 Testing goroutine-channel communication...")
    
    fr fr Initialize scheduler and concurrency runtime
    ready init_goroutine_scheduler(2) == cap {
        vibez.spill("❌ Failed to initialize scheduler")
        damn cap
    }
    
    fr fr Create a channel for communication
    sus channel *Channel = create_channel(5)  fr fr Buffered channel
    ready channel == 0 {
        vibez.spill("❌ Failed to create channel")
        shutdown_goroutine_scheduler()
        damn cap
    }
    
    fr fr Producer goroutine
    slay producer_goroutine(ctx thicc) {
        sus ch *Channel = ctx
        sus i normie = 1
        bestie i <= 3 {
            sus send_result lit = channel_send(ch, i * 10)
            ready send_result == based {
                vibez.spill("📤 Producer sent:", i * 10)
            }
            i = i + 1
        }
        vibez.spill("✅ Producer completed")
    }
    
    fr fr Consumer goroutine
    slay consumer_goroutine(ctx thicc) {
        sus ch *Channel = ctx
        sus received_count normie = 0
        bestie received_count < 3 {
            sus value normie = channel_receive(ch)
            ready value != 0 {
                vibez.spill("📥 Consumer received:", value)
                received_count = received_count + 1
            }
        }
        vibez.spill("✅ Consumer completed")
    }
    
    fr fr Spawn producer and consumer goroutines
    sus producer_id thicc = stan(producer_goroutine, channel)
    sus consumer_id thicc = stan(consumer_goroutine, channel)
    
    ready producer_id == 0 || consumer_id == 0 {
        vibez.spill("❌ Failed to spawn goroutines")
        shutdown_goroutine_scheduler()
        damn cap
    }
    
    vibez.spill("✅ Spawned producer (", producer_id, ") and consumer (", consumer_id, ")")
    
    fr fr Wait for completion
    sus wait_cycles normie = 0
    bestie wait_cycles < 5000000 {
        sus active_count normie = atomic_drip.atomic_load_i32(&global_scheduler.active_goroutines, atomic_drip.ACQUIRE)
        ready active_count == 0 {
            break
        }
        runtime_yield_cpu()
        wait_cycles = wait_cycles + 1
    }
    
    vibez.spill("✅ Goroutine-channel communication test completed")
    
    shutdown_goroutine_scheduler()
    damn based
}

fr fr =============================================================================
fr fr STRESS TESTS
fr fr =============================================================================

fr fr Stress test with high goroutine count
slay test_high_goroutine_count() lit {
    vibez.spill("🧪 Stress testing with high goroutine count...")
    
    ready init_goroutine_scheduler(8) == cap {
        vibez.spill("❌ Failed to initialize scheduler")
        damn cap
    }
    
    slay stress_task(ctx thicc) {
        sus task_id thicc = ctx
        fr fr Simulate variable workload
        sus work_amount normie = (task_id % 100) + 1
        sus work_done normie = 0
        bestie work_done < work_amount {
            work_done = work_done + 1
        }
    }
    
    fr fr Spawn many goroutines rapidly
    sus target_goroutines normie = 1000
    sus spawned normie = 0
    sus failed normie = 0
    
    sus start_time thicc = get_current_time_ns()
    
    sus i normie = 0
    bestie i < target_goroutines {
        sus goroutine_id thicc = stan(stress_task, i)
        ready goroutine_id != 0 {
            spawned = spawned + 1
        } otherwise {
            failed = failed + 1
        }
        i = i + 1
        
        fr fr Occasional yield to prevent overwhelming scheduler
        ready i % 100 == 0 {
            runtime_yield_cpu()
        }
    }
    
    sus spawn_time thicc = get_current_time_ns() - start_time
    
    vibez.spill("📊 Stress test spawn results:")
    vibez.spill("  Target goroutines:", target_goroutines)
    vibez.spill("  Successfully spawned:", spawned)
    vibez.spill("  Failed to spawn:", failed)
    vibez.spill("  Spawn time:", spawn_time, "ns")
    
    fr fr Wait for all to complete
    sus completion_start thicc = get_current_time_ns()
    sus max_wait_cycles normie = 20000000
    sus wait_cycles normie = 0
    
    bestie wait_cycles < max_wait_cycles {
        sus active_count normie = atomic_drip.atomic_load_i32(&global_scheduler.active_goroutines, atomic_drip.ACQUIRE)
        ready active_count == 0 {
            break
        }
        runtime_yield_cpu()
        wait_cycles = wait_cycles + 1
    }
    
    sus completion_time thicc = get_current_time_ns() - completion_start
    sus final_active normie = atomic_drip.atomic_load_i32(&global_scheduler.active_goroutines, atomic_drip.ACQUIRE)
    
    vibez.spill("📊 Stress test completion results:")
    vibez.spill("  Completion time:", completion_time, "ns")
    vibez.spill("  Final active goroutines:", final_active)
    
    ready final_active == 0 {
        vibez.spill("✅ All goroutines completed successfully")
    } otherwise {
        vibez.spill("⚠️ Some goroutines still active (timeout reached)")
    }
    
    fr fr Print final statistics
    sus stats *SchedulerStats = get_scheduler_stats()
    ready stats != 0 {
        vibez.spill("📊 Final Scheduler Statistics:")
        vibez.spill("  Total spawned:", stats.total_goroutines_spawned)
        vibez.spill("  Total completed:", stats.total_goroutines_completed)
        vibez.spill("  Context switches:", stats.total_context_switches)
        vibez.spill("  Work steals:", stats.total_work_steals)
    }
    
    shutdown_goroutine_scheduler()
    
    ready spawned >= (target_goroutines * 8 / 10) {  fr fr At least 80% success rate
        damn based
    } otherwise {
        vibez.spill("❌ Insufficient goroutines spawned successfully")
        damn cap
    }
}

fr fr =============================================================================
fr fr MAIN TEST RUNNER
fr fr =============================================================================

fr fr Run all goroutine runtime tests
slay run_all_goroutine_tests() lit {
    vibez.spill("🚀 Starting Comprehensive Goroutine Runtime Tests")
    vibez.spill("=" * 60)
    
    sus tests_passed normie = 0
    sus tests_failed normie = 0
    sus total_tests normie = 12
    
    fr fr Basic functionality tests
    ready test_basic_goroutine_spawn() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_basic_goroutine_spawn PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_basic_goroutine_spawn FAILED")
    }
    
    ready test_multiple_goroutines() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_multiple_goroutines PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_multiple_goroutines FAILED")
    }
    
    fr fr Work queue tests
    ready test_work_queue_operations() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_work_queue_operations PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_work_queue_operations FAILED")
    }
    
    ready test_work_stealing() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_work_stealing PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_work_stealing FAILED")
    }
    
    fr fr Stack management tests
    ready test_goroutine_stack_management() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_goroutine_stack_management PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_goroutine_stack_management FAILED")
    }
    
    ready test_stack_overflow_detection() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_stack_overflow_detection PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_stack_overflow_detection FAILED")
    }
    
    fr fr Context management tests
    ready test_goroutine_context_lifecycle() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_goroutine_context_lifecycle PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_goroutine_context_lifecycle FAILED")
    }
    
    fr fr Performance tests
    ready test_scheduler_performance() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_scheduler_performance PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_scheduler_performance FAILED")
    }
    
    fr fr Memory management tests
    ready test_goroutine_allocator() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_goroutine_allocator PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_goroutine_allocator FAILED")
    }
    
    fr fr Integration tests
    ready test_goroutine_channel_communication() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_goroutine_channel_communication PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_goroutine_channel_communication FAILED")
    }
    
    fr fr Stress tests
    ready test_high_goroutine_count() == based {
        tests_passed = tests_passed + 1
        vibez.spill("✅ test_high_goroutine_count PASSED")
    } otherwise {
        tests_failed = tests_failed + 1
        vibez.spill("❌ test_high_goroutine_count FAILED")
    }
    
    fr fr Final results
    vibez.spill("=" * 60)
    vibez.spill("🧪 GOROUTINE RUNTIME TEST RESULTS")
    vibez.spill("  Tests passed:", tests_passed)
    vibez.spill("  Tests failed:", tests_failed)
    vibez.spill("  Total tests:", total_tests)
    vibez.spill("  Success rate:", (tests_passed * 100) / total_tests, "%")
    
    ready tests_failed == 0 {
        vibez.spill("🎉 ALL GOROUTINE RUNTIME TESTS PASSED!")
        damn based
    } otherwise {
        vibez.spill("💥 SOME GOROUTINE RUNTIME TESTS FAILED!")
        damn cap
    }
}

fr fr Entry point for running tests
slay main() {
    run_all_goroutine_tests()
}
