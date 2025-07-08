# CURSED Atomic Operations Module Tests (atomic_drip)
# Comprehensive test suite using testz v2.0 framework

yeet "testz"
yeet "atomic_drip"

# Test atomic i32 creation and basic operations
slay test_atomic_i32_creation() {
    test_start("atomic_i32_creation")
    
    sus atomic *AtomicI32 = atomic_i32_new(42)
    sus value normie = atomic_load_i32(atomic)
    
    assert_eq_int(value, 42)
    vibez.spill("✅ Atomic i32 creation test passed")
}

# Test atomic i32 store and load operations
slay test_atomic_i32_store_load() {
    test_start("atomic_i32_store_load")
    
    sus atomic *AtomicI32 = atomic_i32_new(0)
    atomic_store_i32(atomic, 100)
    sus value normie = atomic_load_i32(atomic)
    
    assert_eq_int(value, 100)
    vibez.spill("✅ Atomic i32 store/load test passed")
}

# Test atomic i32 compare-and-swap operation
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
    
    vibez.spill("✅ Atomic i32 CAS test passed")
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

# Test atomic i32 add operation
slay test_atomic_i32_add() {
    test_start("atomic_i32_add")
    
    sus atomic *AtomicI32 = atomic_i32_new(10)
    sus old normie = atomic_add_i32(atomic, 5)
    
    assert_eq_int(old, 10)
    sus new normie = atomic_load_i32(atomic)
    assert_eq_int(new, 15)
    
    vibez.spill("✅ Atomic i32 add test passed")
}

# Test atomic i32 subtract operation
slay test_atomic_i32_sub() {
    test_start("atomic_i32_sub")
    
    sus atomic *AtomicI32 = atomic_i32_new(20)
    sus old normie = atomic_sub_i32(atomic, 8)
    
    assert_eq_int(old, 20)
    sus new normie = atomic_load_i32(atomic)
    assert_eq_int(new, 12)
    
    vibez.spill("✅ Atomic i32 subtract test passed")
}

# Test atomic i64 creation and basic operations
slay test_atomic_i64_creation() {
    test_start("atomic_i64_creation")
    
    sus atomic *AtomicI64 = atomic_i64_new(1000000)
    sus value thicc = atomic_load_i64(atomic)
    
    assert_eq_int(value, 1000000)
    vibez.spill("✅ Atomic i64 creation test passed")
}

# Test atomic i64 store and load operations
slay test_atomic_i64_store_load() {
    test_start("atomic_i64_store_load")
    
    sus atomic *AtomicI64 = atomic_i64_new(0)
    atomic_store_i64(atomic, 9876543210)
    sus value thicc = atomic_load_i64(atomic)
    
    assert_eq_int(value, 9876543210)
    vibez.spill("✅ Atomic i64 store/load test passed")
}

# Test atomic i64 compare-and-swap operation
slay test_atomic_i64_cas() {
    test_start("atomic_i64_cas")
    
    sus atomic *AtomicI64 = atomic_i64_new(5000000)
    
    # Successful CAS
    sus success lit = atomic_cas_i64(atomic, 5000000, 7500000)
    assert_true(success)
    sus value thicc = atomic_load_i64(atomic)
    assert_eq_int(value, 7500000)
    
    # Failed CAS (wrong expected value)
    sus failure lit = atomic_cas_i64(atomic, 5000000, 10000000)
    assert_false(failure)
    sus unchanged thicc = atomic_load_i64(atomic)
    assert_eq_int(unchanged, 7500000)
    
    vibez.spill("✅ Atomic i64 CAS test passed")
}

# Test atomic i64 add operation
slay test_atomic_i64_add() {
    test_start("atomic_i64_add")
    
    sus atomic *AtomicI64 = atomic_i64_new(1000000)
    sus old thicc = atomic_add_i64(atomic, 500000)
    
    assert_eq_int(old, 1000000)
    sus new thicc = atomic_load_i64(atomic)
    assert_eq_int(new, 1500000)
    
    vibez.spill("✅ Atomic i64 add test passed")
}

# Test atomic flag operations
slay test_atomic_flag() {
    test_start("atomic_flag")
    
    sus flag *AtomicFlag = atomic_flag_new()
    
    # Initial state should be clear
    sus initial lit = atomic_flag_test_and_set(flag)
    assert_false(initial)
    
    # Second test-and-set should return true (was set)
    sus second lit = atomic_flag_test_and_set(flag)
    assert_true(second)
    
    # Clear the flag
    atomic_flag_clear(flag)
    
    # Test-and-set should return false again
    sus cleared lit = atomic_flag_test_and_set(flag)
    assert_false(cleared)
    
    vibez.spill("✅ Atomic flag test passed")
}

# Test atomic counter operations
slay test_atomic_counter() {
    test_start("atomic_counter")
    
    sus counter *AtomicCounter = atomic_counter_new(100)
    
    # Test initial value
    sus initial normie = atomic_counter_get(counter)
    assert_eq_int(initial, 100)
    
    # Test increment
    sus old_inc normie = atomic_counter_increment(counter)
    assert_eq_int(old_inc, 100)
    sus after_inc normie = atomic_counter_get(counter)
    assert_eq_int(after_inc, 101)
    
    # Test decrement
    sus old_dec normie = atomic_counter_decrement(counter)
    assert_eq_int(old_dec, 101)
    sus after_dec normie = atomic_counter_get(counter)
    assert_eq_int(after_dec, 100)
    
    # Test set
    atomic_counter_set(counter, 500)
    sus set_value normie = atomic_counter_get(counter)
    assert_eq_int(set_value, 500)
    
    # Test reset
    atomic_counter_reset(counter)
    sus reset_value normie = atomic_counter_get(counter)
    assert_eq_int(reset_value, 0)
    
    vibez.spill("✅ Atomic counter test passed")
}

# Test memory fence operation
slay test_memory_fence() {
    test_start("memory_fence")
    
    # Memory fence is a no-op in simulation, but test it doesn't crash
    memory_fence()
    
    # Test with atomic operations around fence
    sus atomic *AtomicI32 = atomic_i32_new(42)
    memory_fence()
    sus value normie = atomic_load_i32(atomic)
    memory_fence()
    
    assert_eq_int(value, 42)
    vibez.spill("✅ Memory fence test passed")
}

# Test atomic operations with multiple values
slay test_atomic_multiple_operations() {
    test_start("atomic_multiple_operations")
    
    sus atomic *AtomicI32 = atomic_i32_new(0)
    
    # Perform multiple operations
    bestie i := 0; i < 5; i++ {
        atomic_add_i32(atomic, 10)
    }
    
    sus final normie = atomic_load_i32(atomic)
    assert_eq_int(final, 50)
    
    # Subtract back
    bestie i := 0; i < 2; i++ {
        atomic_sub_i32(atomic, 15)
    }
    
    sus result normie = atomic_load_i32(atomic)
    assert_eq_int(result, 20)
    
    vibez.spill("✅ Atomic multiple operations test passed")
}

# Test atomic operations thread safety simulation
slay test_atomic_thread_safety() {
    test_start("atomic_thread_safety")
    
    sus atomic *AtomicI32 = atomic_i32_new(0)
    
    # Simulate concurrent operations
    bestie i := 0; i < 10; i++ {
        atomic_add_i32(atomic, 1)
        sus current normie = atomic_load_i32(atomic)
        atomic_sub_i32(atomic, 1)
    }
    
    sus final normie = atomic_load_i32(atomic)
    assert_eq_int(final, 0)
    
    vibez.spill("✅ Atomic thread safety simulation test passed")
}

# Test atomic memory ordering constants
slay test_memory_ordering() {
    test_start("memory_ordering")
    
    # Test memory ordering constants
    assert_eq_int(MEMORY_ORDER_RELAXED, 0)
    assert_eq_int(MEMORY_ORDER_ACQUIRE, 1)
    assert_eq_int(MEMORY_ORDER_RELEASE, 2)
    assert_eq_int(MEMORY_ORDER_ACQ_REL, 3)
    assert_eq_int(MEMORY_ORDER_SEQ_CST, 4)
    
    vibez.spill("✅ Memory ordering constants test passed")
}

# Main test function
slay main() {
    vibez.spill("🧪 Running CURSED Atomic Operations Module Tests")
    vibez.spill("================================================")
    
    # Run all atomic i32 tests
    test_atomic_i32_creation()
    test_atomic_i32_store_load()
    test_atomic_i32_cas()
    test_atomic_i32_swap()
    test_atomic_i32_add()
    test_atomic_i32_sub()
    
    # Run all atomic i64 tests
    test_atomic_i64_creation()
    test_atomic_i64_store_load()
    test_atomic_i64_cas()
    test_atomic_i64_add()
    
    # Run atomic flag tests
    test_atomic_flag()
    
    # Run atomic counter tests
    test_atomic_counter()
    
    # Run memory fence tests
    test_memory_fence()
    
    # Run advanced tests
    test_atomic_multiple_operations()
    test_atomic_thread_safety()
    test_memory_ordering()
    
    vibez.spill("================================================")
    print_test_summary()
    vibez.spill("🎉 All atomic operations tests completed!")
}
