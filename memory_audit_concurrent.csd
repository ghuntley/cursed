# Concurrent Memory Safety Test
# Tests memory safety with goroutines and channels

yeet "vibez"
yeet "concurrenz"
yeet "mathz"

# Test 1: Multiple goroutines with memory allocation
slay worker_goroutine(id drip, ch chan<drip>) {
    sus local_data []drip = []
    bestie (sus i drip = 0; i < 100; i++) {
        local_data = append(local_data, id * i)
    }
    
    sus sum drip = 0
    bestie (sus i drip = 0; i < len(local_data); i++) {
        sum = sum + local_data[i]
    }
    
    ch <- sum
}

# Test 2: Channel-based memory stress
slay test_concurrent_channels() {
    sus result_ch chan<drip> = make_channel()
    sus num_workers drip = 10
    
    # Spawn worker goroutines
    bestie (sus i drip = 0; i < num_workers; i++) {
        go worker_goroutine(i, result_ch)
    }
    
    # Collect results
    sus total_sum drip = 0
    bestie (sus i drip = 0; i < num_workers; i++) {
        sus worker_sum drip = <-result_ch
        total_sum = total_sum + worker_sum
        vibez.spill("Worker", i, "sum:", worker_sum)
    }
    
    vibez.spill("Total concurrent sum:", total_sum)
}

# Test 3: Producer-consumer pattern
slay producer(data_ch chan<[]drip>) {
    bestie (sus i drip = 0; i < 20; i++) {
        sus data []drip = []
        bestie (sus j drip = 0; j < 50; j++) {
            data = append(data, i * j)
        }
        data_ch <- data
    }
    close(data_ch)
}

slay consumer(data_ch chan<[]drip>, result_ch chan<drip>) {
    sus total drip = 0
    bestie (based) {
        sus data []drip = <-data_ch
        ready (len(data) == 0) {  # Channel closed
            break
        }
        
        bestie (sus i drip = 0; i < len(data); i++) {
            total = total + data[i]
        }
    }
    result_ch <- total
}

slay test_producer_consumer() {
    sus data_ch chan<[]drip> = make_channel()
    sus result_ch chan<drip> = make_channel()
    
    go producer(data_ch)
    go consumer(data_ch, result_ch)
    
    sus final_result drip = <-result_ch
    vibez.spill("Producer-consumer result:", final_result)
}

# Main execution
vibez.spill("Starting concurrent memory safety test...")

test_concurrent_channels()
test_producer_consumer()

vibez.spill("Concurrent memory test completed!")
