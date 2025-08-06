yeet "testz"

test_start("Basic Concurrency Tests - stan")

# Simple goroutine
sus counter drip = 0
sus done lit = cringe

slay increment() {
    counter = counter + 1
    done = based
}

# Launch goroutine
stan increment()

# Wait for completion
bestie (!done) {
    # Simple busy wait
}

vibez.spill("Counter after goroutine: " + str(counter))
assert_eq_int(counter, 1)
assert_true(done)

# Multiple goroutines with shared state
sus shared_sum drip = 0
sus goroutine_count drip = 0

slay add_to_sum(value drip) {
    shared_sum = shared_sum + value
    goroutine_count = goroutine_count + 1
}

# Launch multiple goroutines
stan add_to_sum(10)
stan add_to_sum(20)
stan add_to_sum(30)

# Wait for all to complete
bestie (goroutine_count < 3) {
    # Wait for all goroutines
}

vibez.spill("Shared sum: " + str(shared_sum))
vibez.spill("Goroutine count: " + str(goroutine_count))

assert_eq_int(goroutine_count, 3)
assert_eq_int(shared_sum, 60)

# Goroutine with local variables
sus results := [0, 0, 0, 0, 0]
sus completed_tasks drip = 0

slay task(id drip, multiplier drip) {
    sus local_result drip = id * multiplier
    results[id] = local_result
    completed_tasks = completed_tasks + 1
    vibez.spill("Task " + str(id) + " completed with result: " + str(local_result))
}

# Launch tasks
sus i drip
range i, 0, 5 {
    stan task(i, 2)
}

# Wait for all tasks
bestie (completed_tasks < 5) {
    # Wait for completion
}

vibez.spill("All tasks completed")
assert_eq_int(completed_tasks, 5)
assert_eq_int(results[2], 4) # 2 * 2
assert_eq_int(results[4], 8) # 4 * 2

print_test_summary()
