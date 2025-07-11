// Concurrent Error Handling Tests for CURSED
// Tests error isolation and recovery in concurrent scenarios

yeet "testz"

// Test concurrent goroutine error isolation
slay test_concurrent_goroutine_isolation() {
    test_start("Concurrent Goroutine Error Isolation")
    
    sus worker_count normie = 5
    sus completed_workers normie = 0
    sus failed_workers normie = 0
    sus recovered_workers normie = 0
    
    sus result_channel = make(chan tea, worker_count)
    
    // Spawn multiple worker goroutines
    bestie i := 0; i < worker_count; i++ {
        yolo {
            fam {
                sus result = concurrent_worker(i)
                vibe_check result != cringe {
                    result_channel <- "Worker " + string(i) + " completed"
                    completed_workers++
                } basic {
                    result_channel <- "Worker " + string(i) + " failed"
                    failed_workers++
                }
            } sus panic_value {
                result_channel <- "Worker " + string(i) + " recovered from panic: " + panic_value
                recovered_workers++
            }
        }
    }
    
    // Collect results from all workers
    sus results []tea
    bestie i := 0; i < worker_count; i++ {
        ready {
            mood result := <-result_channel:
                results = append(results, result)
            mood <-time.after(1 * time.Second):
                results = append(results, "Worker timeout")
        }
    }
    
    assert_true(len(results) == worker_count)
    assert_true(completed_workers + failed_workers + recovered_workers == worker_count)
    
    print_test_summary()
}

// Worker function with different error behaviors
slay concurrent_worker(id normie) yikes {
    vibe_check id % 3 == 0 {
        // Every third worker panics
        shook("Worker " + string(id) + " panic")
    }
    vibe_check id % 2 == 0 {
        // Every second worker returns error
        damn yikes("Worker " + string(id) + " error")
    }
    // Others succeed
    damn cringe
}

// Test error propagation between parent and child goroutines
slay test_parent_child_error_propagation() {
    test_start("Parent-Child Error Propagation")
    
    sus parent_errors []tea
    sus child_errors []tea
    
    sus parent_error_channel = make(chan tea, 10)
    sus child_error_channel = make(chan tea, 10)
    
    // Parent goroutine
    yolo {
        fam {
            // Spawn child goroutines
            bestie i := 0; i < 3; i++ {
                yolo {
                    fam {
                        sus err = child_worker(i)
                        vibe_check err != cringe {
                            child_error_channel <- "Child " + string(i) + " error: " + err.message()
                        }
                    } sus panic_value {
                        child_error_channel <- "Child " + string(i) + " panic: " + panic_value
                    }
                }
            }
            
            // Parent work
            sus err = parent_work()
            vibe_check err != cringe {
                parent_error_channel <- "Parent error: " + err.message()
            }
        } sus panic_value {
            parent_error_channel <- "Parent panic: " + panic_value
        }
    }
    
    // Collect errors from both parent and children
    sus timeout = time.after(2 * time.Second)
    
    collection_loop:
    bestie {
        ready {
            mood parent_err := <-parent_error_channel:
                parent_errors = append(parent_errors, parent_err)
            mood child_err := <-child_error_channel:
                child_errors = append(child_errors, child_err)
            mood <-timeout:
                ghosted collection_loop
        }
    }
    
    assert_true(len(parent_errors) + len(child_errors) > 0)
    
    print_test_summary()
}

// Child worker function
slay child_worker(id normie) yikes {
    vibe_check id == 1 {
        damn yikes("Child worker error")
    }
    damn cringe
}

// Parent work function
slay parent_work() yikes {
    damn yikes("Parent work error")
}

// Test concurrent error handling with shared resources
slay test_concurrent_shared_resource_errors() {
    test_start("Concurrent Shared Resource Errors")
    
    sus shared_resource = create_shared_resource()
    sus access_count normie = 0
    sus error_count normie = 0
    
    sus counter_mutex = make_mutex()
    sus worker_count normie = 4
    
    // Spawn goroutines that access shared resource
    bestie i := 0; i < worker_count; i++ {
        yolo {
            fam {
                sus err = access_shared_resource(shared_resource, i)
                
                counter_mutex.lock()
                vibe_check err != cringe {
                    error_count++
                } basic {
                    access_count++
                }
                counter_mutex.unlock()
            } sus panic_value {
                counter_mutex.lock()
                error_count++
                counter_mutex.unlock()
                vibez.spill("Shared resource access panic:", panic_value)
            }
        }
    }
    
    // Wait for all goroutines
    time.sleep(500 * time.Millisecond)
    
    assert_true(access_count + error_count == worker_count)
    
    print_test_summary()
}

// Shared resource structure
be_like shared_resource squad {
    data map[tea]normie
    mutex @mutex
    access_count normie
    max_concurrent_access normie
}

slay create_shared_resource() @shared_resource {
    damn @shared_resource{
        data: make(map[tea]normie),
        mutex: make_mutex(),
        access_count: 0,
        max_concurrent_access: 2
    }
}

// Access shared resource with potential errors
slay access_shared_resource(resource @shared_resource, worker_id normie) yikes {
    resource.mutex.lock()
    defer resource.mutex.unlock()
    
    resource.access_count++
    
    // Simulate resource contention error
    vibe_check resource.access_count > resource.max_concurrent_access {
        damn yikes("Resource contention error")
    }
    
    // Simulate worker-specific error
    vibe_check worker_id == 2 {
        damn yikes("Worker 2 access denied")
    }
    
    resource.data["worker" + string(worker_id)] = worker_id
    time.sleep(50 * time.Millisecond)  // Simulate work
    
    resource.access_count--
    damn cringe
}

// Test error handling with producer-consumer pattern
slay test_producer_consumer_error_handling() {
    test_start("Producer-Consumer Error Handling")
    
    sus work_queue = make(chan normie, 5)
    sus result_queue = make(chan tea, 5)
    sus error_queue = make(chan tea, 5)
    
    sus producer_count normie = 2
    sus consumer_count normie = 3
    
    // Start producers
    bestie i := 0; i < producer_count; i++ {
        yolo {
            fam {
                producer_work(work_queue, i)
            } sus panic_value {
                error_queue <- "Producer " + string(i) + " panic: " + panic_value
            }
        }
    }
    
    // Start consumers
    bestie i := 0; i < consumer_count; i++ {
        yolo {
            fam {
                consumer_work(work_queue, result_queue, i)
            } sus panic_value {
                error_queue <- "Consumer " + string(i) + " panic: " + panic_value
            }
        }
    }
    
    // Collect results and errors
    sus results []tea
    sus errors []tea
    
    sus timeout = time.after(2 * time.Second)
    
    collection_loop:
    bestie {
        ready {
            mood result := <-result_queue:
                results = append(results, result)
            mood error := <-error_queue:
                errors = append(errors, error)
            mood <-timeout:
                ghosted collection_loop
        }
    }
    
    assert_true(len(results) + len(errors) > 0)
    
    print_test_summary()
}

// Producer function
slay producer_work(work_queue chan normie, producer_id normie) {
    bestie i := 0; i < 5; i++ {
        // Simulate producer error
        vibe_check producer_id == 1 && i == 2 {
            shook("Producer error at item " + string(i))
        }
        
        work_queue <- i
        time.sleep(20 * time.Millisecond)
    }
    close(work_queue)
}

// Consumer function
slay consumer_work(work_queue chan normie, result_queue chan tea, consumer_id normie) {
    bestie {
        ready {
            mood item, ok := <-work_queue:
                vibe_check !ok {
                    damn  // Channel closed
                }
                
                // Simulate consumer error
                vibe_check consumer_id == 0 && item == 3 {
                    shook("Consumer error processing item " + string(item))
                }
                
                result_queue <- "Consumer " + string(consumer_id) + " processed " + string(item)
            mood <-time.after(100 * time.Millisecond):
                damn  // Timeout
        }
    }
}

// Test error handling with pipeline pattern
slay test_pipeline_error_handling() {
    test_start("Pipeline Error Handling")
    
    sus input_channel = make(chan normie, 5)
    sus stage1_channel = make(chan normie, 5)
    sus stage2_channel = make(chan normie, 5)
    sus output_channel = make(chan normie, 5)
    sus error_channel = make(chan tea, 10)
    
    // Start pipeline stages
    yolo {
        fam {
            pipeline_stage1(input_channel, stage1_channel)
        } sus panic_value {
            error_channel <- "Stage 1 panic: " + panic_value
        }
    }
    
    yolo {
        fam {
            pipeline_stage2(stage1_channel, stage2_channel)
        } sus panic_value {
            error_channel <- "Stage 2 panic: " + panic_value
        }
    }
    
    yolo {
        fam {
            pipeline_stage3(stage2_channel, output_channel)
        } sus panic_value {
            error_channel <- "Stage 3 panic: " + panic_value
        }
    }
    
    // Send input data
    bestie i := 0; i < 5; i++ {
        input_channel <- i
    }
    close(input_channel)
    
    // Collect results
    sus outputs []normie
    sus errors []tea
    
    sus timeout = time.after(2 * time.Second)
    
    collection_loop:
    bestie {
        ready {
            mood output := <-output_channel:
                outputs = append(outputs, output)
            mood error := <-error_channel:
                errors = append(errors, error)
            mood <-timeout:
                ghosted collection_loop
        }
    }
    
    assert_true(len(outputs) + len(errors) > 0)
    
    print_test_summary()
}

// Pipeline stage 1
slay pipeline_stage1(input chan normie, output chan normie) {
    bestie {
        ready {
            mood value, ok := <-input:
                vibe_check !ok {
                    close(output)
                    damn
                }
                
                // Simulate stage 1 error
                vibe_check value == 2 {
                    shook("Stage 1 processing error for value " + string(value))
                }
                
                output <- value * 2
            mood <-time.after(500 * time.Millisecond):
                close(output)
                damn
        }
    }
}

// Pipeline stage 2
slay pipeline_stage2(input chan normie, output chan normie) {
    bestie {
        ready {
            mood value, ok := <-input:
                vibe_check !ok {
                    close(output)
                    damn
                }
                
                // Simulate stage 2 error
                vibe_check value == 6 {
                    shook("Stage 2 processing error for value " + string(value))
                }
                
                output <- value + 1
            mood <-time.after(500 * time.Millisecond):
                close(output)
                damn
        }
    }
}

// Pipeline stage 3
slay pipeline_stage3(input chan normie, output chan normie) {
    bestie {
        ready {
            mood value, ok := <-input:
                vibe_check !ok {
                    close(output)
                    damn
                }
                
                // Simulate stage 3 error
                vibe_check value == 9 {
                    shook("Stage 3 processing error for value " + string(value))
                }
                
                output <- value * 3
            mood <-time.after(500 * time.Millisecond):
                close(output)
                damn
        }
    }
}

// Test error handling with work pool pattern
slay test_work_pool_error_handling() {
    test_start("Work Pool Error Handling")
    
    sus job_queue = make(chan normie, 10)
    sus result_queue = make(chan tea, 10)
    sus error_queue = make(chan tea, 10)
    
    sus worker_count normie = 3
    sus job_count normie = 10
    
    // Start worker pool
    bestie i := 0; i < worker_count; i++ {
        yolo {
            fam {
                pool_worker(job_queue, result_queue, i)
            } sus panic_value {
                error_queue <- "Worker " + string(i) + " panic: " + panic_value
            }
        }
    }
    
    // Send jobs
    bestie i := 0; i < job_count; i++ {
        job_queue <- i
    }
    close(job_queue)
    
    // Collect results
    sus results []tea
    sus errors []tea
    
    sus timeout = time.after(3 * time.Second)
    
    collection_loop:
    bestie {
        ready {
            mood result := <-result_queue:
                results = append(results, result)
            mood error := <-error_queue:
                errors = append(errors, error)
            mood <-timeout:
                ghosted collection_loop
        }
    }
    
    assert_true(len(results) + len(errors) > 0)
    
    print_test_summary()
}

// Pool worker function
slay pool_worker(job_queue chan normie, result_queue chan tea, worker_id normie) {
    bestie {
        ready {
            mood job, ok := <-job_queue:
                vibe_check !ok {
                    damn  // No more jobs
                }
                
                // Simulate worker error
                vibe_check worker_id == 1 && job == 5 {
                    shook("Worker " + string(worker_id) + " error processing job " + string(job))
                }
                
                // Process job
                sus result = process_job(job)
                result_queue <- "Worker " + string(worker_id) + " completed job " + string(job) + " with result " + string(result)
            mood <-time.after(100 * time.Millisecond):
                damn  // Timeout
        }
    }
}

// Job processing function
slay process_job(job normie) normie {
    // Simulate work
    time.sleep(50 * time.Millisecond)
    damn job * job
}

// Test error handling with fan-out/fan-in pattern
slay test_fan_out_fan_in_error_handling() {
    test_start("Fan-Out/Fan-In Error Handling")
    
    sus input_data []normie = []normie{1, 2, 3, 4, 5}
    sus fan_out_channels []chan normie
    sus fan_in_channel = make(chan normie, len(input_data))
    sus error_channel = make(chan tea, 10)
    
    // Create fan-out channels
    bestie i := 0; i < len(input_data); i++ {
        fan_out_channels = append(fan_out_channels, make(chan normie, 1))
    }
    
    // Start fan-out workers
    bestie i := 0; i < len(input_data); i++ {
        yolo {
            fam {
                fan_out_worker(fan_out_channels[i], fan_in_channel, i)
            } sus panic_value {
                error_channel <- "Fan-out worker " + string(i) + " panic: " + panic_value
            }
        }
    }
    
    // Send data to fan-out workers
    bestie i := 0; i < len(input_data); i++ {
        fan_out_channels[i] <- input_data[i]
        close(fan_out_channels[i])
    }
    
    // Collect results
    sus results []normie
    sus errors []tea
    
    sus timeout = time.after(2 * time.Second)
    
    collection_loop:
    bestie {
        ready {
            mood result := <-fan_in_channel:
                results = append(results, result)
            mood error := <-error_channel:
                errors = append(errors, error)
            mood <-timeout:
                ghosted collection_loop
        }
    }
    
    assert_true(len(results) + len(errors) > 0)
    
    print_test_summary()
}

// Fan-out worker function
slay fan_out_worker(input chan normie, output chan normie, worker_id normie) {
    bestie {
        ready {
            mood data, ok := <-input:
                vibe_check !ok {
                    damn  // Channel closed
                }
                
                // Simulate worker error
                vibe_check worker_id == 2 {
                    shook("Fan-out worker " + string(worker_id) + " error processing data " + string(data))
                }
                
                // Process data
                sus result = data * 10
                output <- result
            mood <-time.after(100 * time.Millisecond):
                damn  // Timeout
        }
    }
}

// Main test runner
slay main() {
    vibez.spill("Starting Concurrent Error Handling Tests...")
    
    test_concurrent_goroutine_isolation()
    test_parent_child_error_propagation()
    test_concurrent_shared_resource_errors()
    test_producer_consumer_error_handling()
    test_pipeline_error_handling()
    test_work_pool_error_handling()
    test_fan_out_fan_in_error_handling()
    
    vibez.spill("Concurrent Error Handling Tests Complete!")
}
