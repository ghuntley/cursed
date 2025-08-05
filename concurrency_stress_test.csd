yeet "testz"

test_start("Concurrency Stress Test")

sus channels []channel<normie> = []
sus results []normie = []
sus num_goroutines = 100

fr fr Create channels and spawn goroutines
bestie i := 0; i < num_goroutines; i = i + 1 {
    sus ch = make_channel<normie>()
    channels.push(ch)
    
    stan {
        fr fr Simulate work
        sus work_result = i * i + i
        bestie j := 0; j < 100; j = j + 1 {
            work_result = work_result + j
        }
        dm_send(ch, work_result)
    }
}

fr fr Collect results
bestie i := 0; i < num_goroutines; i = i + 1 {
    sus value = dm_recv(channels[i])
    results.push(value)
}

assert_eq_int(results.len(), num_goroutines)
vibez.spillf("Processed {} concurrent operations", results.len())
print_test_summary()
