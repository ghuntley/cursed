# CURSED Atomic Operations Module Tests (atomic_drip)
# Comprehensive test suite for hardware atomic operations using testz v2.0

yeet "testz"
yeet "atomic_drip"

# Test basic atomic i32 operations
slay test_atomic_i32_basic() {
    test_start("atomic_i32_basic")
    
    sus atomic *AtomicI32 = atomic_i32_new(42)
    sus value normie = atomic_load_i32(atomic)
    
    assert_eq_int(value, 42)
    
    # Test store
    atomic_store_i32(atomic, 100)
    sus stored normie = atomic_load_i32(atomic)
    assert_eq_int(stored, 100)
    
    vibez.spill("✅ Basic atomic i32 operations test passed")
}

# Test atomic i32 compare-and-swap operations
slay test_atomic_i32_cas() {
    test_start("atomic_i32_cas")
    
    sus atomic *AtomicI32 = atomic_i32_new(50)
    
    # Successful CAS
    sus success lit = atomic_cas_i32(atomic, 50, 75)
    assert_true(success)
    sus value normie = atomic_load_i32(atomic)
    assert_eq_int(value, 75)
    
    # Failed CAS (wrong expected value)
    sus failure lit = atomic_cas_i32(atomic, 50, 100)
    assert_false(failure)
    sus unchanged normie = atomic_load_i32(atomic)
    assert_eq_int(unchanged, 75)
    
    # Test weak CAS
    sus expected normie = 75
    sus weak_success lit = atomic_cas_weak_i32(atomic, &expected, 125)
    assert_true(weak_success)
    sus weak_result normie = atomic_load_i32(atomic)
    assert_eq_int(weak_result, 125)
    
    vibez.spill("✅ Atomic i32 CAS test passed")
}

# Test atomic i32 arithmetic operations
slay test_atomic_i32_arithmetic() {
    test_start("atomic_i32_arithmetic")
    
    sus atomic *AtomicI32 = atomic_i32_new(10)
    
    # Test add (returns old value)
    sus old_add normie = atomic_add_i32(atomic, 5)
    assert_eq_int(old_add, 10)
    sus after_add normie = atomic_load_i32(atomic)
    assert_eq_int(after_add, 15)
    
    # Test subtract (returns old value)
    sus old_sub normie = atomic_sub_i32(atomic, 3)
    assert_eq_int(old_sub, 15)
    sus after_sub normie = atomic_load_i32(atomic)
    assert_eq_int(after_sub, 12)
    
    # Test increment (returns new value)
    sus inc_result normie = atomic_increment_i32(atomic)
    assert_eq_int(inc_result, 13)
    sus after_inc normie = atomic_load_i32(atomic)
    assert_eq_int(after_inc, 13)
    
    # Test decrement (returns new value)
    sus dec_result normie = atomic_decrement_i32(atomic)
    assert_eq_int(dec_result, 12)
    sus after_dec normie = atomic_load_i32(atomic)
    assert_eq_int(after_dec, 12)
    
    vibez.spill("✅ Atomic i32 arithmetic test passed")
}

# Test atomic i32 bitwise operations
slay test_atomic_i32_bitwise() {
    test_start("atomic_i32_bitwise")
    
    sus atomic *AtomicI32 = atomic_i32_new(0xFF)  # 255 in binary: 11111111
    
    # Test AND operation
    sus old_and normie = atomic_and_i32(atomic, 0x0F)  # 15 in binary: 00001111
    assert_eq_int(old_and, 255)
    sus after_and normie = atomic_load_i32(atomic)
    assert_eq_int(after_and, 15)  # 255 & 15 = 15
    
    # Test OR operation
    sus old_or normie = atomic_or_i32(atomic, 0xF0)  # 240 in binary: 11110000
    assert_eq_int(old_or, 15)
    sus after_or normie = atomic_load_i32(atomic)
    assert_eq_int(after_or, 255)  # 15 | 240 = 255
    
    # Test XOR operation
    sus old_xor normie = atomic_xor_i32(atomic, 0xFF)  # 255 in binary: 11111111
    assert_eq_int(old_xor, 255)
    sus after_xor normie = atomic_load_i32(atomic)
    assert_eq_int(after_xor, 0)  # 255 ^ 255 = 0
    
    vibez.spill("✅ Atomic i32 bitwise operations test passed")
}

# Test atomic i32 swap operation
slay test_atomic_i32_swap() {
    test_start("atomic_i32_swap")
    
    sus atomic *AtomicI32 = atomic_i32_new(25)
    sus old normie = atomic_swap_i32(atomic, 60)
    
    assert_eq_int(old, 25)
    sus new normie = atomic_load_i32(atomic)
    assert_eq_int(new, 60)
    
    vibez.spill("✅ Atomic i32 swap test passed")
}

# Test atomic i64 operations
slay test_atomic_i64_operations() {
    test_start("atomic_i64_operations")
    
    sus atomic *AtomicI64 = atomic_i64_new(1000000)
    
    # Test basic load/store
    sus value thicc = atomic_load_i64(atomic)
    assert_eq_int(value, 1000000)
    
    atomic_store_i64(atomic, 9876543210)
    sus stored thicc = atomic_load_i64(atomic)
    assert_eq_int(stored, 9876543210)
    
    # Test CAS
    sus success lit = atomic_cas_i64(atomic, 9876543210, 5000000000)
    assert_true(success)
    sus cas_result thicc = atomic_load_i64(atomic)
    assert_eq_int(cas_result, 5000000000)
    
    # Test arithmetic
    sus old_add thicc = atomic_add_i64(atomic, 1000000000)
    assert_eq_int(old_add, 5000000000)
    sus after_add thicc = atomic_load_i64(atomic)
    assert_eq_int(after_add, 6000000000)
    
    sus old_sub thicc = atomic_sub_i64(atomic, 500000000)
    assert_eq_int(old_sub, 6000000000)
    sus after_sub thicc = atomic_load_i64(atomic)
    assert_eq_int(after_sub, 5500000000)
    
    # Test increment/decrement
    sus inc_result thicc = atomic_increment_i64(atomic)
    assert_eq_int(inc_result, 5500000001)
    
    sus dec_result thicc = atomic_decrement_i64(atomic)
    assert_eq_int(dec_result, 5500000000)
    
    vibez.spill("✅ Atomic i64 operations test passed")
}

# Test atomic flag operations
slay test_atomic_flag() {
    test_start("atomic_flag")
    
    sus flag *AtomicFlag = atomic_flag_new()
    
    # Initial state should be clear
    assert_false(atomic_flag_is_set(flag))
    
    # First test-and-set should return false (was clear)
    sus initial lit = atomic_flag_test_and_set(flag)
    assert_false(initial)
    assert_true(atomic_flag_is_set(flag))
    
    # Second test-and-set should return true (was set)
    sus second lit = atomic_flag_test_and_set(flag)
    assert_true(second)
    assert_true(atomic_flag_is_set(flag))
    
    # Clear the flag
    atomic_flag_clear(flag)
    assert_false(atomic_flag_is_set(flag))
    
    # Test-and-set should return false again
    sus cleared lit = atomic_flag_test_and_set(flag)
    assert_false(cleared)
    assert_true(atomic_flag_is_set(flag))
    
    vibez.spill("✅ Atomic flag test passed")
}

# Test atomic counter operations
slay test_atomic_counter() {
    test_start("atomic_counter")
    
    sus counter *AtomicCounter = atomic_counter_new(100)
    
    # Test initial value
    sus initial normie = atomic_counter_get(counter)
    assert_eq_int(initial, 100)
    
    # Test increment (returns old value)
    sus old_inc normie = atomic_counter_increment(counter)
    assert_eq_int(old_inc, 100)
    sus after_inc normie = atomic_counter_get(counter)
    assert_eq_int(after_inc, 101)
    
    # Test decrement (returns old value)
    sus old_dec normie = atomic_counter_decrement(counter)
    assert_eq_int(old_dec, 101)
    sus after_dec normie = atomic_counter_get(counter)
    assert_eq_int(after_dec, 100)
    
    # Test add
    sus old_add normie = atomic_counter_add(counter, 50)
    assert_eq_int(old_add, 100)
    sus after_add normie = atomic_counter_get(counter)
    assert_eq_int(after_add, 150)
    
    # Test subtract
    sus old_sub normie = atomic_counter_sub(counter, 25)
    assert_eq_int(old_sub, 150)
    sus after_sub normie = atomic_counter_get(counter)
    assert_eq_int(after_sub, 125)
    
    # Test set (returns old value)
    sus old_set normie = atomic_counter_set(counter, 500)
    assert_eq_int(old_set, 125)
    sus set_value normie = atomic_counter_get(counter)
    assert_eq_int(set_value, 500)
    
    # Test reset (returns old value)
    sus old_reset normie = atomic_counter_reset(counter)
    assert_eq_int(old_reset, 500)
    sus reset_value normie = atomic_counter_get(counter)
    assert_eq_int(reset_value, 0)
    
    # Test CAS
    sus cas_success lit = atomic_counter_cas(counter, 0, 1000)
    assert_true(cas_success)
    sus cas_value normie = atomic_counter_get(counter)
    assert_eq_int(cas_value, 1000)
    
    vibez.spill("✅ Atomic counter test passed")
}

# Test memory ordering and fences
slay test_memory_ordering() {
    test_start("memory_ordering")
    
    # Test memory ordering constants
    assert_eq_int(MEMORY_ORDER_RELAXED, 0)
    assert_eq_int(MEMORY_ORDER_ACQUIRE, 1)
    assert_eq_int(MEMORY_ORDER_RELEASE, 2)
    assert_eq_int(MEMORY_ORDER_ACQ_REL, 3)
    assert_eq_int(MEMORY_ORDER_SEQ_CST, 4)
    
    # Test different fence types (should not crash)
    memory_fence()
    memory_fence_ordered(MEMORY_ORDER_SEQ_CST)
    compiler_fence()
    acquire_fence()
    release_fence()
    acq_rel_fence()
    
    # Test ordered operations
    sus atomic *AtomicI32 = atomic_i32_new(42)
    
    atomic_store_i32_ordered(atomic, 100, MEMORY_ORDER_RELEASE)
    sus value normie = atomic_load_i32_ordered(atomic, MEMORY_ORDER_ACQUIRE)
    assert_eq_int(value, 100)
    
    sus cas_success lit = atomic_cas_i32_ordered(atomic, 100, 200, MEMORY_ORDER_ACQ_REL)
    assert_true(cas_success)
    sus final normie = atomic_load_i32_ordered(atomic, MEMORY_ORDER_SEQ_CST)
    assert_eq_int(final, 200)
    
    vibez.spill("✅ Memory ordering test passed")
}

# Test atomic pointer operations
slay test_atomic_pointer() {
    test_start("atomic_pointer")
    
    # Create some test pointers (using addresses as integers)
    sus ptr1 *void = 0x1000.(*void)
    sus ptr2 *void = 0x2000.(*void)
    sus ptr3 *void = 0x3000.(*void)
    
    sus atomic_ptr *AtomicPtr = atomic_ptr_new(ptr1)
    
    # Test load
    sus loaded *void = atomic_ptr_load(atomic_ptr)
    assert_eq_int(loaded.(thicc), ptr1.(thicc))
    
    # Test store
    atomic_ptr_store(atomic_ptr, ptr2)
    sus stored *void = atomic_ptr_load(atomic_ptr)
    assert_eq_int(stored.(thicc), ptr2.(thicc))
    
    # Test CAS
    sus cas_success lit = atomic_ptr_cas(atomic_ptr, ptr2, ptr3)
    assert_true(cas_success)
    sus cas_result *void = atomic_ptr_load(atomic_ptr)
    assert_eq_int(cas_result.(thicc), ptr3.(thicc))
    
    # Test swap
    sus old_ptr *void = atomic_ptr_swap(atomic_ptr, ptr1)
    assert_eq_int(old_ptr.(thicc), ptr3.(thicc))
    sus new_ptr *void = atomic_ptr_load(atomic_ptr)
    assert_eq_int(new_ptr.(thicc), ptr1.(thicc))
    
    vibez.spill("✅ Atomic pointer test passed")
}

# Test spinlock operations
slay test_spinlock() {
    test_start("spinlock")
    
    sus lock *Spinlock = spinlock_new()
    
    # Test try_lock on unlocked spinlock
    sus try_success lit = spinlock_try_lock(lock)
    assert_true(try_success)
    
    # Test try_lock on locked spinlock
    sus try_fail lit = spinlock_try_lock(lock)
    assert_false(try_fail)
    
    # Unlock and test again
    spinlock_unlock(lock)
    sus try_again lit = spinlock_try_lock(lock)
    assert_true(try_again)
    
    # Test normal lock/unlock
    spinlock_unlock(lock)
    spinlock_lock(lock)  # Should succeed
    spinlock_unlock(lock)
    
    vibez.spill("✅ Spinlock test passed")
}

# Test read-write spinlock operations
slay test_rw_spinlock() {
    test_start("rw_spinlock")
    
    sus rw_lock *RwSpinlock = rw_spinlock_new()
    
    # Test read lock
    rw_spinlock_read_lock(rw_lock)
    rw_spinlock_read_lock(rw_lock)  # Multiple readers allowed
    rw_spinlock_read_unlock(rw_lock)
    rw_spinlock_read_unlock(rw_lock)
    
    # Test write lock
    sus write_try lit = rw_spinlock_try_write_lock(rw_lock)
    assert_true(write_try)
    
    # Try write lock while write locked (should fail)
    sus write_fail lit = rw_spinlock_try_write_lock(rw_lock)
    assert_false(write_fail)
    
    # Unlock write
    rw_spinlock_write_unlock(rw_lock)
    
    # Test read lock after write unlock
    rw_spinlock_read_lock(rw_lock)
    
    # Try write lock while read locked (should fail)
    sus write_blocked lit = rw_spinlock_try_write_lock(rw_lock)
    assert_false(write_blocked)
    
    # Unlock read
    rw_spinlock_read_unlock(rw_lock)
    
    # Now write lock should succeed
    sus write_success lit = rw_spinlock_try_write_lock(rw_lock)
    assert_true(write_success)
    rw_spinlock_write_unlock(rw_lock)
    
    vibez.spill("✅ Read-write spinlock test passed")
}

# Test concurrent operations simulation
slay test_concurrent_simulation() {
    test_start("concurrent_simulation")
    
    sus atomic *AtomicI32 = atomic_i32_new(0)
    
    # Simulate concurrent increments
    bestie i := 0; i < 100; i++ {
        atomic_increment_i32(atomic)
    }
    
    sus final normie = atomic_load_i32(atomic)
    assert_eq_int(final, 100)
    
    # Simulate concurrent decrements
    bestie i := 0; i < 50; i++ {
        atomic_decrement_i32(atomic)
    }
    
    sus result normie = atomic_load_i32(atomic)
    assert_eq_int(result, 50)
    
    # Simulate mixed operations
    bestie i := 0; i < 25; i++ {
        atomic_add_i32(atomic, 2)
        atomic_sub_i32(atomic, 1)
    }
    
    sus mixed_result normie = atomic_load_i32(atomic)
    assert_eq_int(mixed_result, 75)  # 50 + 25 = 75
    
    vibez.spill("✅ Concurrent simulation test passed")
}

# Test atomic operations under contention simulation
slay test_contention_simulation() {
    test_start("contention_simulation")
    
    sus flag *AtomicFlag = atomic_flag_new()
    sus counter *AtomicCounter = atomic_counter_new(0)
    
    # Simulate multiple threads trying to acquire flag
    bestie i := 0; i < 10; i++ {
        yo atomic_flag_test_and_set(flag) {
            # Flag was already set, continue
            simp
        } kinda {
            # Got the flag, do work
            atomic_counter_increment(counter)
            atomic_flag_clear(flag)
        }
    }
    
    # At least one thread should have gotten the flag
    sus work_done normie = atomic_counter_get(counter)
    assert_true(work_done > 0)
    
    vibez.spill("✅ Contention simulation test passed")
}

# Performance benchmark for atomic operations
slay test_atomic_performance() {
    test_start("atomic_performance")
    
    sus iterations normie = 10000
    sus atomic *AtomicI32 = atomic_i32_new(0)
    
    # Benchmark atomic increments
    bestie i := 0; i < iterations; i++ {
        atomic_increment_i32(atomic)
    }
    
    sus final normie = atomic_load_i32(atomic)
    assert_eq_int(final, iterations)
    
    # Benchmark CAS operations
    sus cas_count normie = 0
    bestie i := 0; i < 1000; i++ {
        sus expected normie = atomic_load_i32(atomic)
        yo atomic_cas_i32(atomic, expected, expected + 1) {
            cas_count++
        }
    }
    
    assert_true(cas_count > 0)
    
    vibez.spill("✅ Atomic performance test passed")
}

# Test weak CAS spurious failures
slay test_weak_cas() {
    test_start("weak_cas")
    
    sus atomic *AtomicI32 = atomic_i32_new(100)
    sus expected normie = 100
    
    # Weak CAS may fail spuriously, so retry in a loop
    sus success lit = cap
    sus attempts normie = 0
    bestie !success && attempts < 100 {
        expected = atomic_load_i32(atomic)
        success = atomic_cas_weak_i32(atomic, &expected, 200)
        attempts++
    }
    
    assert_true(success)
    sus final normie = atomic_load_i32(atomic)
    assert_eq_int(final, 200)
    
    vibez.spill("✅ Weak CAS test passed")
}

# Main test function
slay main() {
    vibez.spill("🧪 Running CURSED Hardware Atomic Operations Module Tests")
    vibez.spill("===========================================================")
    
    # Basic atomic operations tests
    test_atomic_i32_basic()
    test_atomic_i32_cas()
    test_atomic_i32_arithmetic()
    test_atomic_i32_bitwise()
    test_atomic_i32_swap()
    
    # 64-bit atomic operations
    test_atomic_i64_operations()
    
    # Atomic flag operations
    test_atomic_flag()
    
    # Atomic counter operations
    test_atomic_counter()
    
    # Memory ordering and fences
    test_memory_ordering()
    
    # Atomic pointer operations
    test_atomic_pointer()
    
    # High-level synchronization primitives
    test_spinlock()
    test_rw_spinlock()
    
    # Concurrency simulation tests
    test_concurrent_simulation()
    test_contention_simulation()
    
    # Performance and advanced tests
    test_atomic_performance()
    test_weak_cas()
    
    vibez.spill("===========================================================")
    print_test_summary()
    vibez.spill("🎉 All hardware atomic operations tests completed!")
}
