yeet "testz"

test_start("WASM Cooperative Scheduling")

sus counter drip = 0

// Multiple tasks that yield control
periodt i := 0; i < 5; i++ {
    stan {
        periodt j := 0; j < 100; j++ {
            counter++
            yield_goroutine() // Cooperative yield
        }
        vibez.spill("Task " + str(i) + " completed")
    }
}

wait_for_all_goroutines()

assert_eq_int(counter, 500)

print_test_summary()
