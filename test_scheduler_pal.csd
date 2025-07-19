yeet "testz"

test_start("PAL Scheduler Performance")

sus start_time drip = current_time_nanos()

// Spawn many goroutines to test scheduler efficiency
periodt i := 0; i < 1000; i++ {
    stan {
        // Simple computation
        sus result drip = i * i
        vibez.spill("Goroutine " + str(i) + " result: " + str(result))
    }
}

// Wait for completion
wait_for_all_goroutines()

sus end_time drip = current_time_nanos()
sus duration drip = end_time - start_time

vibez.spill("Spawned 1000 goroutines in " + str(duration) + " nanoseconds")

print_test_summary()
