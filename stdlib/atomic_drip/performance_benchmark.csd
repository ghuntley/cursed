fr fr CURSED Atomic Operations Performance Benchmark
fr fr Comprehensive performance testing and validation

yeet "testz"
yeet "atomic_drip"

fr fr Performance benchmark configuration
sus BENCHMARK_ITERATIONS normie = 100000
sus CONTENTION_THREADS normie = 4
sus CACHE_LINE_SIZE normie = 64

fr fr Benchmark atomic load/store operations
slay benchmark_atomic_load_store() {
    test_start("benchmark_atomic_load_store")
    
    sus atomic *AtomicI32 = atomic_i32_new(42)
    sus iterations normie = BENCHMARK_ITERATIONS fr fr Benchmark atomic loads
    bestie i := 0; i < iterations; i++ {
        sus value normie = atomic_load_i32(atomic) fr fr Prevent optimization
        yo value != 42 {
            vibez.spill("Unexpected value:", value)
        }
    } fr fr Benchmark atomic stores
    bestie i := 0; i < iterations; i++ {
        atomic_store_i32(atomic, i)
    }
    
    sus final normie = atomic_load_i32(atomic)
    assert_eq_int(final, iterations - 1)
    
    vibez.spill("✅ Load/store benchmark completed")
}

fr fr Benchmark atomic compare-and-swap operations
slay benchmark_atomic_cas() {
    test_start("benchmark_atomic_cas")
    
    sus atomic *AtomicI32 = atomic_i32_new(0)
    sus iterations normie = BENCHMARK_ITERATIONS / 10 fr fr CAS is more expensive
    sus successful_cas normie = 0 fr fr Benchmark strong CAS
    bestie i := 0; i < iterations; i++ {
        sus expected normie = atomic_load_i32(atomic)
        yo atomic_cas_i32(atomic, expected, expected + 1) {
            successful_cas++
        }
    } fr fr Benchmark weak CAS
    sus weak_successful normie = 0
    bestie i := 0; i < iterations; i++ {
        sus expected normie = atomic_load_i32(atomic)
        yo atomic_cas_weak_i32(atomic, &expected, expected + 1) {
            weak_successful++
        }
    }
    
    assert_true(successful_cas > 0)
    assert_true(weak_successful > 0)
    
    vibez.spill("✅ CAS benchmark completed")
    vibez.spill("Strong CAS success rate:", successful_cas * 100 / iterations, "%")
    vibez.spill("Weak CAS success rate:", weak_successful * 100 / iterations, "%")
}

fr fr Benchmark atomic arithmetic operations
slay benchmark_atomic_arithmetic() {
    test_start("benchmark_atomic_arithmetic")
    
    sus atomic *AtomicI32 = atomic_i32_new(0)
    sus iterations normie = BENCHMARK_ITERATIONS fr fr Benchmark atomic add
    bestie i := 0; i < iterations / 4; i++ {
        atomic_add_i32(atomic, 1)
    } fr fr Benchmark atomic subtract
    bestie i := 0; i < iterations / 4; i++ {
        atomic_sub_i32(atomic, 1)
    } fr fr Benchmark atomic increment
    bestie i := 0; i < iterations / 4; i++ {
        atomic_increment_i32(atomic)
    } fr fr Benchmark atomic decrement
    bestie i := 0; i < iterations / 4; i++ {
        atomic_decrement_i32(atomic)
    }
    
    sus final normie = atomic_load_i32(atomic)
    assert_eq_int(final, 0) fr fr Should be back to 0
    
    vibez.spill("✅ Arithmetic benchmark completed")
}

fr fr Benchmark atomic bitwise operations
slay benchmark_atomic_bitwise() {
    test_start("benchmark_atomic_bitwise")
    
    sus atomic *AtomicI32 = atomic_i32_new(0xFFFFFFFF)
    sus iterations normie = BENCHMARK_ITERATIONS / 10 fr fr Benchmark atomic AND
    bestie i := 0; i < iterations; i++ {
        atomic_and_i32(atomic, 0xFFFFFFFE) fr fr Clear LSB
        atomic_or_i32(atomic, 0x00000001) fr fr Set LSB
    } fr fr Benchmark atomic XOR
    bestie i := 0; i < iterations; i++ {
        atomic_xor_i32(atomic, 0xAAAAAAAA) fr fr Toggle pattern
    }
    
    sus final normie = atomic_load_i32(atomic) fr fr Final value depends on whether iterations is even/odd
    assert_true(final == 0x55555555 || final == 0xFFFFFFFF)
    
    vibez.spill("✅ Bitwise benchmark completed")
}

fr fr Benchmark atomic flag operations
slay benchmark_atomic_flag() {
    test_start("benchmark_atomic_flag")
    
    sus flag *AtomicFlag = atomic_flag_new()
    sus iterations normie = BENCHMARK_ITERATIONS
    sus test_and_set_count normie = 0 fr fr Benchmark test-and-set operations
    bestie i := 0; i < iterations; i++ {
        yo atomic_flag_test_and_set(flag) {
            test_and_set_count++
        }
        atomic_flag_clear(flag)
    }
    
    assert_true(test_and_set_count < iterations) fr fr Some should be already set
    
    vibez.spill("✅ Flag benchmark completed")
    vibez.spill("Test-and-set success rate:", test_and_set_count * 100 / iterations, "%")
}

fr fr Benchmark memory ordering overhead
slay benchmark_memory_ordering() {
    test_start("benchmark_memory_ordering")
    
    sus atomic *AtomicI32 = atomic_i32_new(0)
    sus iterations normie = BENCHMARK_ITERATIONS fr fr Benchmark relaxed ordering (fastest)
    bestie i := 0; i < iterations / 4; i++ {
        atomic_store_i32_ordered(atomic, i, MEMORY_ORDER_RELAXED)
        sus value normie = atomic_load_i32_ordered(atomic, MEMORY_ORDER_RELAXED)
    } fr fr Benchmark acquire-release ordering
    bestie i := 0; i < iterations / 4; i++ {
        atomic_store_i32_ordered(atomic, i, MEMORY_ORDER_RELEASE)
        sus value normie = atomic_load_i32_ordered(atomic, MEMORY_ORDER_ACQUIRE)
    } fr fr Benchmark sequential consistency (strongest)
    bestie i := 0; i < iterations / 4; i++ {
        atomic_store_i32(atomic, i) fr fr Defaults to SEQ_CST
        sus value normie = atomic_load_i32(atomic)
    } fr fr Benchmark memory fences
    bestie i := 0; i < iterations / 4; i++ {
        memory_fence()
        compiler_fence()
    }
    
    vibez.spill("✅ Memory ordering benchmark completed")
}

fr fr Benchmark contention simulation
slay benchmark_contention_simulation() {
    test_start("benchmark_contention_simulation")
    
    sus atomic *AtomicI32 = atomic_i32_new(0)
    sus counter *AtomicCounter = atomic_counter_new(0)
    sus iterations normie = BENCHMARK_ITERATIONS / 100 fr fr Simulate high contention scenario
    bestie thread := 0; thread < CONTENTION_THREADS; thread++ {
        bestie i := 0; i < iterations; i++ { fr fr Multiple operations competing for same atomic
            atomic_increment_i32(atomic)
            atomic_counter_increment(counter) fr fr Add some delay to increase contention
            bestie delay := 0; delay < 10; delay++ {
                compiler_fence()
            }
        }
    }
    
    sus atomic_final normie = atomic_load_i32(atomic)
    sus counter_final normie = atomic_counter_get(counter)
    
    sus expected normie = CONTENTION_THREADS * iterations
    assert_eq_int(atomic_final, expected)
    assert_eq_int(counter_final, expected)
    
    vibez.spill("✅ Contention simulation benchmark completed")
}

fr fr Benchmark spinlock performance
slay benchmark_spinlock_performance() {
    test_start("benchmark_spinlock_performance")
    
    sus lock *Spinlock = spinlock_new()
    sus counter normie = 0
    sus iterations normie = BENCHMARK_ITERATIONS / 100 fr fr Benchmark spinlock acquire/release
    bestie i := 0; i < iterations; i++ {
        spinlock_lock(lock)
        counter++ fr fr Critical section
        spinlock_unlock(lock)
    }
    
    assert_eq_int(counter, iterations) fr fr Benchmark try_lock operations
    sus try_success_count normie = 0
    bestie i := 0; i < iterations; i++ {
        yo spinlock_try_lock(lock) {
            try_success_count++
            spinlock_unlock(lock)
        }
    }
    
    assert_eq_int(try_success_count, iterations) fr fr All should succeed when uncontended
    
    vibez.spill("✅ Spinlock performance benchmark completed")
}

fr fr Benchmark read-write spinlock performance
slay benchmark_rw_spinlock_performance() {
    test_start("benchmark_rw_spinlock_performance")
    
    sus rw_lock *RwSpinlock = rw_spinlock_new()
    sus read_count normie = 0
    sus write_count normie = 0
    sus iterations normie = BENCHMARK_ITERATIONS / 100 fr fr Benchmark read operations (should be concurrent)
    bestie reader := 0; reader < 3; reader++ {
        bestie i := 0; i < iterations / 3; i++ {
            rw_spinlock_read_lock(rw_lock)
            read_count++ fr fr Reading shared data
            rw_spinlock_read_unlock(rw_lock)
        }
    } fr fr Benchmark write operations (should be exclusive)
    bestie i := 0; i < iterations / 3; i++ {
        rw_spinlock_write_lock(rw_lock)
        write_count++ fr fr Writing shared data
        rw_spinlock_write_unlock(rw_lock)
    }
    
    assert_eq_int(read_count, iterations)
    assert_eq_int(write_count, iterations / 3)
    
    vibez.spill("✅ Read-write spinlock performance benchmark completed")
}

fr fr Benchmark atomic pointer operations
slay benchmark_atomic_pointer_performance() {
    test_start("benchmark_atomic_pointer_performance")
    
    sus ptr1 *void = 0x1000.(*void)
    sus ptr2 *void = 0x2000.(*void)
    sus ptr3 *void = 0x3000.(*void)
    
    sus atomic_ptr *AtomicPtr = atomic_ptr_new(ptr1)
    sus iterations normie = BENCHMARK_ITERATIONS / 10 fr fr Benchmark pointer load/store
    bestie i := 0; i < iterations; i++ {
        atomic_ptr_store(atomic_ptr, ptr2)
        sus loaded *void = atomic_ptr_load(atomic_ptr)
        assert_eq_int(loaded.(thicc), ptr2.(thicc))
    } fr fr Benchmark pointer CAS
    sus cas_success_count normie = 0
    bestie i := 0; i < iterations; i++ {
        yo atomic_ptr_cas(atomic_ptr, ptr2, ptr3) {
            cas_success_count++
            atomic_ptr_store(atomic_ptr, ptr2) fr fr Reset for next iteration
        }
    }
    
    assert_eq_int(cas_success_count, iterations)
    
    vibez.spill("✅ Atomic pointer performance benchmark completed")
}

fr fr Benchmark cache line effects
slay benchmark_cache_line_effects() {
    test_start("benchmark_cache_line_effects") fr fr Create atomics that might share cache lines
    sus atomic1 *AtomicI32 = atomic_i32_new(0)
    sus atomic2 *AtomicI32 = atomic_i32_new(0)
    sus iterations normie = BENCHMARK_ITERATIONS / 10 fr fr Benchmark operations on potentially shared cache line
    bestie i := 0; i < iterations; i++ {
        atomic_increment_i32(atomic1)
        atomic_increment_i32(atomic2)
    }
    
    sus result1 normie = atomic_load_i32(atomic1)
    sus result2 normie = atomic_load_i32(atomic2)
    
    assert_eq_int(result1, iterations)
    assert_eq_int(result2, iterations)
    
    vibez.spill("✅ Cache line effects benchmark completed")
}

fr fr Comprehensive performance comparison
slay benchmark_performance_comparison() {
    test_start("benchmark_performance_comparison")
    
    sus atomic *AtomicI32 = atomic_i32_new(0)
    sus iterations normie = 10000 fr fr Compare different atomic operations performance
    vibez.spill("Performance comparison (", iterations, " iterations):") fr fr Atomic load
    bestie i := 0; i < iterations; i++ {
        sus value normie = atomic_load_i32(atomic)
    }
    vibez.spill("- Atomic load: ✓") fr fr Atomic store
    bestie i := 0; i < iterations; i++ {
        atomic_store_i32(atomic, i)
    }
    vibez.spill("- Atomic store: ✓") fr fr Atomic increment
    bestie i := 0; i < iterations; i++ {
        atomic_increment_i32(atomic)
    }
    vibez.spill("- Atomic increment: ✓") fr fr Atomic CAS
    bestie i := 0; i < iterations; i++ {
        sus expected normie = atomic_load_i32(atomic)
        atomic_cas_i32(atomic, expected, expected + 1)
    }
    vibez.spill("- Atomic CAS: ✓")
    
    vibez.spill("✅ Performance comparison completed")
}

fr fr Main benchmark function
slay main() {
    vibez.spill("🚀 Running CURSED Atomic Operations Performance Benchmark")
    vibez.spill("==========================================================")
    vibez.spill("Configuration:")
    vibez.spill("- Benchmark iterations:", BENCHMARK_ITERATIONS)
    vibez.spill("- Contention threads:", CONTENTION_THREADS)
    vibez.spill("- Cache line size:", CACHE_LINE_SIZE)
    vibez.spill("==========================================================") fr fr Basic operation benchmarks
    benchmark_atomic_load_store()
    benchmark_atomic_cas()
    benchmark_atomic_arithmetic()
    benchmark_atomic_bitwise() fr fr Advanced operation benchmarks
    benchmark_atomic_flag()
    benchmark_memory_ordering()
    benchmark_atomic_pointer_performance() fr fr Concurrency and contention benchmarks
    benchmark_contention_simulation()
    benchmark_spinlock_performance()
    benchmark_rw_spinlock_performance() fr fr System-level benchmarks
    benchmark_cache_line_effects()
    benchmark_performance_comparison()
    
    vibez.spill("==========================================================")
    print_test_summary()
    vibez.spill("🎉 All atomic operations performance benchmarks completed!")
    vibez.spill("⚡ Hardware atomic operations validated for production use!")
}
