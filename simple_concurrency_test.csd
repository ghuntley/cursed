yeet "testz"
yeet "concurrenz"

slay test_basic_concurrency() {
    test_start("Basic concurrency test")
    
    # Test mutex
    sus m *Mutex = mutex_new()
    assert_false(m.locked)
    
    mutex_lock(m)
    assert_true(m.locked)
    
    mutex_unlock(m)
    assert_false(m.locked)
    
    # Test atomic
    sus a *Atomic = atomic_new(10)
    assert_eq_int(atomic_load(a), 10)
    
    atomic_store(a, 20)
    assert_eq_int(atomic_load(a), 20)
    
    # Test waitgroup
    sus wg *WaitGroup = waitgroup_new()
    assert_eq_int(wg.count, 0)
    
    waitgroup_add(wg, 1)
    assert_eq_int(wg.count, 1)
    
    waitgroup_done(wg)
    assert_eq_int(wg.count, 0)
    
    vibez.spill("✓ Basic concurrency primitives working")
}

slay test_channel_operations() {
    test_start("Channel operations test")
    
    sus ch chan normie = make(chan normie, 2)
    
    # Send values
    ch <- 42
    ch <- 100
    
    # Receive values
    sus val1 normie = <-ch
    sus val2 normie = <-ch
    
    assert_eq_int(val1, 42)
    assert_eq_int(val2, 100)
    
    vibez.spill("✓ Channel operations working")
}

slay main() {
    vibez.spill("=== Simple Concurrency Test ===")
    
    test_basic_concurrency()
    test_channel_operations()
    
    print_test_summary()
    
    vibez.spill("✓ Concurrency module working correctly!")
}
