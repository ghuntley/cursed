yeet "testz"

test_start("PAL Validation Test")

# Basic Platform Detection Test
test_start("Platform Detection")
sus platform tea = get_platform_info()
assert_true(platform.arch != "")
assert_true(platform.os != "")
vibez.spill("Detected: " + platform.arch + "/" + platform.os)

# Basic Memory Management Test  
test_start("Memory Management")
sus mem1 drip = allocate_memory(1024)
sus mem2 drip = allocate_memory(4096)
assert_true(mem1 != 0)
assert_true(mem2 != 0)
assert_true(mem1 != mem2)
deallocate_memory(mem1, 1024)
deallocate_memory(mem2, 4096)
vibez.spill("Memory allocation/deallocation: PASS")

# Basic Scheduler Test
test_start("Scheduler Functionality")
sus counter drip = 0
periodt i := 0; i < 10; i++ {
    stan {
        counter = counter + 1
    }
}
wait_for_all_goroutines()
assert_eq_int(counter, 10)
vibez.spill("Scheduler: PASS")

# Hardware Feature Detection Test
test_start("Hardware Features")
sus has_simd lit = platform_supports_simd()
sus has_crypto lit = platform_supports_crypto()
vibez.spill("SIMD: " + str(has_simd))
vibez.spill("Crypto: " + str(has_crypto))

print_test_summary()
