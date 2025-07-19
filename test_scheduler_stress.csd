yeet "testz"

test_start("Scheduler Stress Test")

sus completed_tasks drip = 0

// Spawn many short-lived goroutines
periodt i := 0; i < 10000; i++ {
    stan {
        // Short computation
        sus result drip = i * 2 + 1
        atomic_increment(&completed_tasks)
        
        lowkey i % 100 == 0 {
            yield_goroutine()
        }
    }
}

wait_for_all_goroutines()

assert_eq_int(completed_tasks, 10000)

print_test_summary()
