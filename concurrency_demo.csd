# CURSED Concurrency Demo
# Demonstrates stan/dm system with goroutines and channels

yeet "vibez"
yeet "timez"

# Basic goroutine demonstration
slay simple_worker(id normie, result dm<normie>) {
    vibez.spill("Worker", id, "starting")
    
    # Simulate some work
    timez.sleep(100) # 100ms
    
    sus computation = id * id
    dm_send(result, computation)
    
    vibez.spill("Worker", id, "completed, result:", computation)
}

# Demonstrate basic stan and dm operations
slay basic_concurrency_demo() {
    vibez.spill("=== Basic Concurrency Demo ===")
    
    # Create unbuffered channel for results
    sus result_ch dm<normie>
    
    # Spawn goroutines using stan
    stan simple_worker(1, result_ch)
    stan simple_worker(2, result_ch)
    stan simple_worker(3, result_ch)
    
    # Collect results from goroutines
    sus results []normie
    bestie i := 0; i < 3; i++ {
        sus result = dm_recv(result_ch)
        results = append(results, result)
        vibez.spill("Received result:", result)
    }
    
    vibez.spill("All results:", results)
    dm_close(result_ch)
}

# Producer-Consumer pattern
slay producer(data dm<normie>, count normie) {
    vibez.spill("Producer starting, will send", count, "items")
    
    bestie i := 0; i < count; i++ {
        sus value = i * 10
        dm_send(data, value)
        vibez.spill("Produced:", value)
        timez.sleep(50) # 50ms delay
    }
    
    dm_close(data)
    vibez.spill("Producer finished")
}

slay consumer(data dm<normie>, id normie) {
    vibez.spill("Consumer", id, "starting")
    
    bestie based {
        sus value, ok = dm_recv(data)
        ready (!ok) {
            ghosted # Channel closed
        }
        
        vibez.spill("Consumer", id, "received:", value)
        timez.sleep(30) # 30ms processing time
    }
    
    vibez.spill("Consumer", id, "finished")
}

slay producer_consumer_demo() {
    vibez.spill("\n=== Producer-Consumer Demo ===")
    
    # Create buffered channel
    sus data_ch dm<normie>[5]
    
    # Start producer and consumers
    stan producer(data_ch, 10)
    stan consumer(data_ch, 1)
    stan consumer(data_ch, 2)
    
    # Wait for completion
    timez.sleep(2000) # 2 seconds
}

# Fan-out/Fan-in pattern
slay worker_pool_task(jobs dm<normie>, results dm<normie>, worker_id normie) {
    vibez.spill("Worker", worker_id, "ready")
    
    bestie based {
        sus job, ok = dm_recv(jobs)
        ready (!ok) {
            ghosted # No more jobs
        }
        
        vibez.spill("Worker", worker_id, "processing job:", job)
        
        # Simulate work
        timez.sleep(100)
        sus result = job * job
        
        dm_send(results, result)
        vibez.spill("Worker", worker_id, "completed job", job, "->", result)
    }
    
    vibez.spill("Worker", worker_id, "shutting down")
}

slay worker_pool_demo() {
    vibez.spill("\n=== Worker Pool Demo ===")
    
    sus jobs dm<normie>[10]
    sus results dm<normie>[10]
    
    # Start worker pool
    sus num_workers normie = 3
    bestie i := 0; i < num_workers; i++ {
        stan worker_pool_task(jobs, results, i + 1)
    }
    
    # Send jobs
    sus num_jobs normie = 8
    bestie i := 0; i < num_jobs; i++ {
        dm_send(jobs, i + 1)
    }
    dm_close(jobs)
    
    # Collect results
    sus collected_results []normie
    bestie i := 0; i < num_jobs; i++ {
        sus result = dm_recv(results)
        collected_results = append(collected_results, result)
    }
    
    dm_close(results)
    vibez.spill("Worker pool results:", collected_results)
}

# Select statement demonstration with ready/mood/basic
slay select_demo() {
    vibez.spill("\n=== Select Statement Demo ===")
    
    sus ch1 dm<normie>[2]
    sus ch2 dm<tea>[2]
    sus timeout dm<lit>[1]
    
    # Start goroutines that send data
    stan {
        timez.sleep(200)
        dm_send(ch1, 42)
        vibez.spill("Sent to ch1")
    }
    
    stan {
        timez.sleep(300)
        dm_send(ch2, "hello")
        vibez.spill("Sent to ch2")
    }
    
    stan {
        timez.sleep(500)
        dm_send(timeout, based)
        vibez.spill("Timeout triggered")
    }
    
    # Demonstrate select with multiple channels
    sus received_count normie = 0
    bestie received_count < 3 {
        ready {
            mood value := dm_recv(ch1):
                vibez.spill("Selected from ch1:", value)
                received_count++
            mood message := dm_recv(ch2):
                vibez.spill("Selected from ch2:", message)
                received_count++
            mood dm_recv(timeout):
                vibez.spill("Select timed out")
                received_count++
            basic:
                vibez.spill("No channels ready, waiting...")
                timez.sleep(50)
        }
    }
    
    dm_close(ch1)
    dm_close(ch2)
    dm_close(timeout)
}

# Pipeline pattern demonstration
slay stage1_parser(input dm<tea>, output dm<normie>) {
    vibez.spill("Stage 1 (Parser) starting")
    
    bestie based {
        sus text, ok = dm_recv(input)
        ready (!ok) {
            ghosted
        }
        
        # Parse text to number (simplified)
        sus length = len(text)
        dm_send(output, length)
        vibez.spill("Parsed:", text, "-> length:", length)
    }
    
    dm_close(output)
    vibez.spill("Stage 1 finished")
}

slay stage2_transformer(input dm<normie>, output dm<normie>) {
    vibez.spill("Stage 2 (Transformer) starting")
    
    bestie based {
        sus number, ok = dm_recv(input)
        ready (!ok) {
            ghosted
        }
        
        sus transformed = number * 2
        dm_send(output, transformed)
        vibez.spill("Transformed:", number, "->", transformed)
    }
    
    dm_close(output)
    vibez.spill("Stage 2 finished")
}

slay stage3_formatter(input dm<normie>, output dm<tea>) {
    vibez.spill("Stage 3 (Formatter) starting")
    
    bestie based {
        sus number, ok = dm_recv(input)
        ready (!ok) {
            ghosted
        }
        
        sus formatted = "Result: " + string(number)
        dm_send(output, formatted)
        vibez.spill("Formatted:", number, "->", formatted)
    }
    
    dm_close(output)
    vibez.spill("Stage 3 finished")
}

slay pipeline_demo() {
    vibez.spill("\n=== Pipeline Demo ===")
    
    # Create pipeline channels
    sus stage1_out dm<normie>[2]
    sus stage2_out dm<normie>[2]
    sus final_out dm<tea>[2]
    
    # Create input channel and start pipeline
    sus input dm<tea>[2]
    stan stage1_parser(input, stage1_out)
    stan stage2_transformer(stage1_out, stage2_out)
    stan stage3_formatter(stage2_out, final_out)
    
    # Send data through pipeline
    sus test_data []tea = ["hello", "world", "cursed", "language"]
    bestie _, text := flex test_data {
        dm_send(input, text)
    }
    dm_close(input)
    
    # Collect final results
    sus results []tea
    bestie based {
        sus result, ok = dm_recv(final_out)
        ready (!ok) {
            ghosted
        }
        results = append(results, result)
    }
    
    vibez.spill("Pipeline results:", results)
}

# Concurrent error handling with goroutines
slay risky_worker(id normie, ch dm<normie>) {
    fam {
        ready (id == 2) {
            yikes("Worker 2 intentionally fails") shook
        }
        
        sus result = id * 100
        dm_send(ch, result)
        vibez.spill("Worker", id, "succeeded with result:", result)
    } sus error {
        vibez.spill("Worker", id, "failed with error:", error.message())
        dm_send(ch, -1) # Send error indicator
    }
}

slay concurrent_error_demo() {
    vibez.spill("\n=== Concurrent Error Handling Demo ===")
    
    sus results dm<normie>[5]
    
    # Start workers, one will fail
    bestie i := 1; i <= 3; i++ {
        stan risky_worker(i, results)
    }
    
    # Collect results
    bestie i := 0; i < 3; i++ {
        sus result = dm_recv(results)
        ready (result == -1) {
            vibez.spill("Received error indicator from worker")
        } basic {
            vibez.spill("Received successful result:", result)
        }
    }
    
    dm_close(results)
}

# Main demonstration function
slay main() {
    vibez.spill("Starting CURSED Concurrency Demonstrations")
    
    basic_concurrency_demo()
    producer_consumer_demo()
    worker_pool_demo()
    select_demo()
    pipeline_demo()
    concurrent_error_demo()
    
    vibez.spill("\n=== All Concurrency Demos Complete ===")
    vibez.spill("Note: Some operations may still be completing in background goroutines")
}

# Run the demo
main()

# Give final goroutines time to complete
timez.sleep(1000)
vibez.spill("Final cleanup complete")
