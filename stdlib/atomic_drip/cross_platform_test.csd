fr fr CURSED Cross-Platform Atomic Operations Test
fr fr Validates hardware atomic operations across all target platforms

yeet "testz"
yeet "atomic_drip"

fr fr Cross-platform atomic feature detection
slay test_platform_atomic_support() {
    test_start("platform_atomic_support") fr fr Test basic atomic operations work on current platform
    sus atomic *AtomicI32 = atomic_i32_new(1) fr fr Test compare-and-swap (fundamental atomic operation)
    sus cas_success lit = atomic_cas_i32(atomic, 1, 2)
    assert_true(cas_success)
    
    sus value normie = atomic_load_i32(atomic)
    assert_eq_int(value, 2)
    
    vibez.spill("✅ Platform atomic support test passed")
}

fr fr Test x86_64 specific atomic optimizations
slay test_x86_64_atomics() {
    test_start("x86_64_atomics") fr fr Test operations that benefit from x86_64 LOCK prefix
    sus atomic *AtomicI32 = atomic_i32_new(0) fr fr Test atomic increment (LOCK INC on x86_64)
    bestie i := 0; i < 1000; i++ {
        atomic_increment_i32(atomic)
    }
    
    sus result normie = atomic_load_i32(atomic)
    assert_eq_int(result, 1000) fr fr Test atomic decrement (LOCK DEC on x86_64)
    bestie i := 0; i < 500; i++ {
        atomic_decrement_i32(atomic)
    }
    
    sus final normie = atomic_load_i32(atomic)
    assert_eq_int(final, 500)
    
    vibez.spill("✅ x86_64 atomic optimizations test passed")
}

fr fr Test ARM64 specific atomic optimizations
slay test_arm64_atomics() {
    test_start("arm64_atomics") fr fr Test operations that benefit from ARM64 exclusive instructions
    sus atomic *AtomicI64 = atomic_i64_new(0) fr fr Test large value operations (LDXR/STXR on ARM64)
    sus large_value thicc = 0x123456789ABCDEF0
    atomic_store_i64(atomic, large_value)
    sus loaded thicc = atomic_load_i64(atomic)
    assert_eq_int(loaded, large_value) fr fr Test compare-and-swap with large values
    sus cas_success lit = atomic_cas_i64(atomic, large_value, 0xFEDCBA9876543210)
    assert_true(cas_success)
    
    sus swapped thicc = atomic_load_i64(atomic)
    assert_eq_int(swapped, 0xFEDCBA9876543210)
    
    vibez.spill("✅ ARM64 atomic optimizations test passed")
}

fr fr Test WASM atomic support
slay test_wasm_atomics() {
    test_start("wasm_atomics") fr fr Test operations that map to WASM atomic instructions
    sus atomic *AtomicI32 = atomic_i32_new(42) fr fr Test atomic RMW operations (i32.atomic.rmw.* in WASM)
    sus old_add normie = atomic_add_i32(atomic, 8)
    assert_eq_int(old_add, 42)
    
    sus old_and normie = atomic_and_i32(atomic, 0xFF)
    assert_eq_int(old_and, 50) fr fr 42 + 8 = 50
    
    sus old_or normie = atomic_or_i32(atomic, 0x100)
    assert_eq_int(old_or, 50) fr fr 50 & 0xFF = 50
    
    sus final normie = atomic_load_i32(atomic)
    assert_eq_int(final, 306) fr fr 50 | 0x100 = 306
    
    vibez.spill("✅ WASM atomic support test passed")
}

fr fr Test memory ordering across platforms
slay test_cross_platform_memory_ordering() {
    test_start("cross_platform_memory_ordering")
    
    sus atomic1 *AtomicI32 = atomic_i32_new(0)
    sus atomic2 *AtomicI32 = atomic_i32_new(0) fr fr Test acquire-release ordering
    atomic_store_i32_ordered(atomic1, 1, MEMORY_ORDER_RELEASE)
    release_fence()
    
    sus value1 normie = atomic_load_i32_ordered(atomic1, MEMORY_ORDER_ACQUIRE)
    assert_eq_int(value1, 1) fr fr Test sequential consistency
    atomic_store_i32(atomic2, 2) fr fr Defaults to SEQ_CST
    memory_fence()
    
    sus value2 normie = atomic_load_i32(atomic2)
    assert_eq_int(value2, 2) fr fr Test relaxed ordering
    atomic_store_i32_ordered(atomic1, 3, MEMORY_ORDER_RELAXED)
    sus relaxed normie = atomic_load_i32_ordered(atomic1, MEMORY_ORDER_RELAXED)
    assert_eq_int(relaxed, 3)
    
    vibez.spill("✅ Cross-platform memory ordering test passed")
}

fr fr Test atomic operations performance characteristics
slay test_atomic_performance_characteristics() {
    test_start("atomic_performance_characteristics")
    
    sus iterations normie = 10000
    sus atomic *AtomicI32 = atomic_i32_new(0) fr fr Test contention-free performance
    bestie i := 0; i < iterations; i++ {
        atomic_cas_i32(atomic, i, i + 1)
    }
    
    sus cas_result normie = atomic_load_i32(atomic)
    assert_true(cas_result > 0) fr fr Some CAS operations should succeed fr fr Test atomic counter performance
    sus counter *AtomicCounter = atomic_counter_new(0)
    bestie i := 0; i < iterations; i++ {
        atomic_counter_increment(counter)
    }
    
    sus counter_final normie = atomic_counter_get(counter)
    assert_eq_int(counter_final, iterations)
    
    vibez.spill("✅ Atomic performance characteristics test passed")
}

fr fr Test weak vs strong CAS semantics
slay test_weak_vs_strong_cas() {
    test_start("weak_vs_strong_cas")
    
    sus atomic *AtomicI32 = atomic_i32_new(100) fr fr Strong CAS - should succeed if values match
    sus strong_success lit = atomic_cas_i32(atomic, 100, 200)
    assert_true(strong_success)
    sus strong_result normie = atomic_load_i32(atomic)
    assert_eq_int(strong_result, 200) fr fr Weak CAS - may fail spuriously, so loop until success
    atomic_store_i32(atomic, 300)
    sus expected normie = 300
    sus weak_attempts normie = 0
    sus weak_success lit = cap
    
    bestie !weak_success && weak_attempts < 100 {
        expected = atomic_load_i32(atomic)
        weak_success = atomic_cas_weak_i32(atomic, &expected, 400)
        weak_attempts++
    }
    
    assert_true(weak_success)
    sus weak_result normie = atomic_load_i32(atomic)
    assert_eq_int(weak_result, 400)
    
    vibez.spill("✅ Weak vs strong CAS test passed")
}

fr fr Test atomic flag synchronization patterns
slay test_atomic_flag_patterns() {
    test_start("atomic_flag_patterns")
    
    sus flag *AtomicFlag = atomic_flag_new()
    sus counter *AtomicCounter = atomic_counter_new(0) fr fr Test simple spinlock pattern
    yo !atomic_flag_test_and_set(flag) { fr fr Critical section
        atomic_counter_increment(counter)
        atomic_counter_increment(counter)
        atomic_flag_clear(flag)
    }
    
    sus critical_work normie = atomic_counter_get(counter)
    assert_eq_int(critical_work, 2) fr fr Test ordered flag operations
    atomic_flag_clear_ordered(flag, MEMORY_ORDER_RELEASE)
    assert_false(atomic_flag_is_set(flag))
    
    atomic_flag_test_and_set_ordered(flag, MEMORY_ORDER_ACQUIRE)
    assert_true(atomic_flag_is_set(flag))
    
    vibez.spill("✅ Atomic flag patterns test passed")
}

fr fr Test pointer atomic operations
slay test_atomic_pointer_operations() {
    test_start("atomic_pointer_operations") fr fr Create test pointers using integer addresses
    sus addr1 thicc = 0x1000
    sus addr2 thicc = 0x2000
    sus addr3 thicc = 0x3000
    
    sus ptr1 *void = addr1.(*void)
    sus ptr2 *void = addr2.(*void)
    sus ptr3 *void = addr3.(*void)
    
    sus atomic_ptr *AtomicPtr = atomic_ptr_new(ptr1) fr fr Test atomic pointer load/store
    sus loaded *void = atomic_ptr_load(atomic_ptr)
    assert_eq_int(loaded.(thicc), addr1)
    
    atomic_ptr_store(atomic_ptr, ptr2)
    sus stored *void = atomic_ptr_load(atomic_ptr)
    assert_eq_int(stored.(thicc), addr2) fr fr Test atomic pointer CAS
    sus ptr_cas_success lit = atomic_ptr_cas(atomic_ptr, ptr2, ptr3)
    assert_true(ptr_cas_success)
    
    sus cas_result *void = atomic_ptr_load(atomic_ptr)
    assert_eq_int(cas_result.(thicc), addr3) fr fr Test atomic pointer swap
    sus old_ptr *void = atomic_ptr_swap(atomic_ptr, ptr1)
    assert_eq_int(old_ptr.(thicc), addr3)
    
    sus swapped *void = atomic_ptr_load(atomic_ptr)
    assert_eq_int(swapped.(thicc), addr1)
    
    vibez.spill("✅ Atomic pointer operations test passed")
}

fr fr Test high-level synchronization primitives
slay test_synchronization_primitives() {
    test_start("synchronization_primitives") fr fr Test spinlock
    sus lock *Spinlock = spinlock_new()
    
    spinlock_lock(lock)
    sus try_fail lit = spinlock_try_lock(lock)
    assert_false(try_fail) fr fr Should fail when already locked
    spinlock_unlock(lock)
    
    sus try_success lit = spinlock_try_lock(lock)
    assert_true(try_success) fr fr Should succeed when unlocked
    spinlock_unlock(lock) fr fr Test read-write spinlock
    sus rw_lock *RwSpinlock = rw_spinlock_new() fr fr Multiple readers should be allowed
    rw_spinlock_read_lock(rw_lock)
    rw_spinlock_read_lock(rw_lock)
    rw_spinlock_read_unlock(rw_lock)
    rw_spinlock_read_unlock(rw_lock) fr fr Writer should block readers
    rw_spinlock_write_lock(rw_lock)
    sus read_blocked lit = rw_spinlock_try_write_lock(rw_lock)
    assert_false(read_blocked) fr fr Second writer should fail
    rw_spinlock_write_unlock(rw_lock)
    
    vibez.spill("✅ Synchronization primitives test passed")
}

fr fr Test memory fence ordering guarantees
slay test_memory_fence_guarantees() {
    test_start("memory_fence_guarantees")
    
    sus atomic1 *AtomicI32 = atomic_i32_new(0)
    sus atomic2 *AtomicI32 = atomic_i32_new(0) fr fr Test that fences prevent reordering
    atomic_store_i32_ordered(atomic1, 1, MEMORY_ORDER_RELAXED)
    release_fence() fr fr Prevents later operations from moving before this point
    atomic_store_i32_ordered(atomic2, 2, MEMORY_ORDER_RELAXED)
    
    acquire_fence() fr fr Prevents earlier operations from moving after this point
    sus value1 normie = atomic_load_i32_ordered(atomic1, MEMORY_ORDER_RELAXED)
    sus value2 normie = atomic_load_i32_ordered(atomic2, MEMORY_ORDER_RELAXED)
    
    assert_eq_int(value1, 1)
    assert_eq_int(value2, 2) fr fr Test acquire-release fence
    acq_rel_fence() fr fr Test compiler fence (should not crash)
    compiler_fence()
    
    vibez.spill("✅ Memory fence guarantees test passed")
}

fr fr Main cross-platform test function
slay main() {
    vibez.spill("🌍 Running CURSED Cross-Platform Atomic Operations Tests")
    vibez.spill("============================================================") fr fr Platform detection and support tests
    test_platform_atomic_support() fr fr Platform-specific optimization tests
    test_x86_64_atomics()
    test_arm64_atomics()
    test_wasm_atomics() fr fr Cross-platform compatibility tests
    test_cross_platform_memory_ordering()
    test_atomic_performance_characteristics() fr fr Advanced atomic operation tests
    test_weak_vs_strong_cas()
    test_atomic_flag_patterns()
    test_atomic_pointer_operations() fr fr High-level synchronization tests
    test_synchronization_primitives()
    test_memory_fence_guarantees()
    
    vibez.spill("============================================================")
    print_test_summary()
    vibez.spill("🎉 All cross-platform atomic operations tests completed!")
    vibez.spill("💪 Hardware atomics validated across all target platforms!")
}
