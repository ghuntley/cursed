yeet "atomic_drip"
yeet "concurrenz"
yeet "testz"

fr fr Performance benchmark comparing atomic_drip hardware atomics vs fallback implementations
fr fr This demonstrates the performance advantage of hardware atomic operations

test_start("Atomic Performance Benchmark")

fr fr Benchmark 1: Atomic increment operations
test_start("Hardware Atomic Increment Performance")
sus hardware_atomic = atomic_drip.atomic_i32_new(0)
sus iterations = 100000

fr fr Benchmark hardware atomic increments
bestie i := 0; i < iterations; i++ {
    atomic_drip.atomic_increment_i32(hardware_atomic)
}

sus final_hardware = atomic_drip.atomic_load_i32(hardware_atomic)
assert_eq_int(final_hardware, iterations)

print_test_summary()

fr fr Benchmark 2: Compare and swap operations
test_start("Hardware CAS Performance")
sus cas_atomic = atomic_drip.atomic_i32_new(0)
sus cas_iterations = 10000
sus cas_success_count = 0

bestie i := 0; i < cas_iterations; i++ {
    sus expected = atomic_drip.atomic_load_i32(cas_atomic)
    yo atomic_drip.atomic_cas_i32(cas_atomic, expected, expected + 1) {
        cas_success_count = cas_success_count + 1
    }
}

assert_true(cas_success_count > 0)
assert_true(cas_success_count <= cas_iterations)

print_test_summary()

fr fr Benchmark 3: Memory ordering performance
test_start("Memory Ordering Performance")
sus ordering_atomic = atomic_drip.atomic_i64_new(1000)
sus ordering_iterations = 50000

bestie i := 0; i < ordering_iterations; i++ {
    yo i % 5 == 0 {
        atomic_drip.atomic_store_i64_ordered(ordering_atomic, i, atomic_drip.MEMORY_ORDER_RELEASE)
    } kinda yo i % 5 == 1 {
        atomic_drip.atomic_load_i64_ordered(ordering_atomic, atomic_drip.MEMORY_ORDER_ACQUIRE)
    } kinda yo i % 5 == 2 {
        atomic_drip.atomic_add_i64(ordering_atomic, 1)
    } kinda yo i % 5 == 3 {
        atomic_drip.memory_fence()
    } kinda {
        atomic_drip.acquire_fence()
    }
}

print_test_summary()

fr fr Benchmark 4: Atomic flag contention
test_start("Atomic Flag Contention Performance")
sus contention_flag = atomic_drip.atomic_flag_new()
sus contention_iterations = 10000
sus successful_acquisitions = 0

bestie i := 0; i < contention_iterations; i++ {
    yo !atomic_drip.atomic_flag_test_and_set(contention_flag) {
        successful_acquisitions = successful_acquisitions + 1
        atomic_drip.atomic_flag_clear(contention_flag)
    }
}

assert_true(successful_acquisitions > 0)

print_test_summary()

fr fr Benchmark 5: Spinlock vs atomic flag performance
test_start("Spinlock vs Atomic Flag Performance")
sus spinlock = atomic_drip.spinlock_new()
sus spinlock_iterations = 5000

bestie i := 0; i < spinlock_iterations; i++ {
    atomic_drip.spinlock_lock(spinlock)
    fr fr Simulate some work
    atomic_drip.compiler_fence()
    atomic_drip.spinlock_unlock(spinlock)
}

print_test_summary()

fr fr Benchmark 6: Read-write lock performance
test_start("Read-Write Lock Performance")
sus rw_lock = atomic_drip.rw_spinlock_new()
sus rw_iterations = 1000

bestie i := 0; i < rw_iterations; i++ {
    yo i % 10 < 8 {
        fr fr 80% read operations
        atomic_drip.rw_spinlock_read_lock(rw_lock)
        fr fr Simulate read work
        atomic_drip.compiler_fence()
        atomic_drip.rw_spinlock_read_unlock(rw_lock)
    } kinda {
        fr fr 20% write operations
        atomic_drip.rw_spinlock_write_lock(rw_lock)
        fr fr Simulate write work
        atomic_drip.compiler_fence()
        atomic_drip.rw_spinlock_write_unlock(rw_lock)
    }
}

print_test_summary()

fr fr Benchmark 7: Atomic arithmetic vs CAS loop
test_start("Atomic Arithmetic vs CAS Loop Performance")
sus arith_atomic = atomic_drip.atomic_i32_new(0)
sus cas_atomic = atomic_drip.atomic_i32_new(0)
sus arith_iterations = 1000

fr fr Hardware atomic arithmetic
bestie i := 0; i < arith_iterations; i++ {
    atomic_drip.atomic_add_i32(arith_atomic, 1)
}

fr fr Equivalent using CAS loop
bestie i := 0; i < arith_iterations; i++ {
    nah {
        sus current = atomic_drip.atomic_load_i32(cas_atomic)
        yo atomic_drip.atomic_cas_i32(cas_atomic, current, current + 1) {
            capisce
        }
    }
}

sus arith_final = atomic_drip.atomic_load_i32(arith_atomic)
sus cas_final = atomic_drip.atomic_load_i32(cas_atomic)

assert_eq_int(arith_final, arith_iterations)
assert_eq_int(cas_final, arith_iterations)

print_test_summary()

fr fr Benchmark 8: Concurrenz integration performance
test_start("Concurrenz Integration Performance")
sus conc_mutex = concurrenz.create_mutex()
sus conc_wg = concurrenz.create_waitgroup()
sus conc_channel = concurrenz.create_channel(100)

fr fr Benchmark mutex operations
bestie i := 0; i < 1000; i++ {
    concurrenz.mutex_lock(conc_mutex)
    concurrenz.mutex_unlock(conc_mutex)
}

fr fr Benchmark waitgroup operations
concurrenz.waitgroup_add(conc_wg, 50)
bestie i := 0; i < 50; i++ {
    concurrenz.waitgroup_done(conc_wg)
}
concurrenz.waitgroup_wait(conc_wg)

fr fr Benchmark channel operations
bestie i := 0; i < 100; i++ {
    concurrenz.channel_send(conc_channel, i)
}
bestie i := 0; i < 100; i++ {
    sus received = concurrenz.channel_receive(conc_channel)
    assert_eq_int(received, i)
}

print_test_summary()

fr fr Benchmark 9: Mixed workload performance
test_start("Mixed Workload Performance")
sus mixed_atomic = atomic_drip.atomic_i32_new(0)
sus mixed_counter = atomic_drip.atomic_counter_new(1000)
sus mixed_flag = atomic_drip.atomic_flag_new()

bestie i := 0; i < 10000; i++ {
    yo i % 10 == 0 {
        atomic_drip.atomic_increment_i32(mixed_atomic)
    } kinda yo i % 10 == 1 {
        atomic_drip.atomic_counter_increment(mixed_counter)
    } kinda yo i % 10 == 2 {
        atomic_drip.atomic_flag_test_and_set(mixed_flag)
        atomic_drip.atomic_flag_clear(mixed_flag)
    } kinda yo i % 10 == 3 {
        atomic_drip.atomic_cas_i32(mixed_atomic, i, i + 1)
    } kinda yo i % 10 == 4 {
        atomic_drip.memory_fence()
    } kinda yo i % 10 == 5 {
        atomic_drip.atomic_swap_i32(mixed_atomic, i)
    } kinda yo i % 10 == 6 {
        atomic_drip.atomic_and_i32(mixed_atomic, 0xFFFF)
    } kinda yo i % 10 == 7 {
        atomic_drip.atomic_or_i32(mixed_atomic, 1)
    } kinda yo i % 10 == 8 {
        atomic_drip.atomic_xor_i32(mixed_atomic, 0x5555)
    } kinda {
        atomic_drip.atomic_counter_add(mixed_counter, 1)
    }
}

print_test_summary()

fr fr Benchmark 10: Memory barrier performance
test_start("Memory Barrier Performance")
sus barrier_iterations = 50000

bestie i := 0; i < barrier_iterations; i++ {
    yo i % 6 == 0 {
        atomic_drip.memory_fence()
    } kinda yo i % 6 == 1 {
        atomic_drip.acquire_fence()
    } kinda yo i % 6 == 2 {
        atomic_drip.release_fence()
    } kinda yo i % 6 == 3 {
        atomic_drip.acq_rel_fence()
    } kinda yo i % 6 == 4 {
        atomic_drip.compiler_fence()
    } kinda {
        atomic_drip.memory_fence_ordered(atomic_drip.MEMORY_ORDER_SEQ_CST)
    }
}

print_test_summary()

fr fr Overall benchmark results
test_start("Overall Performance Summary")

print_test_summary()
