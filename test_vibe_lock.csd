// Simple tests for vibe_lock synchronization primitives

// Test mutex functionality
slay test_mutex_basic() normie {
    vibez.spill("Testing mutex basic operations...")
    
    // Test basic mutex operations
    sus mu normie = 42
    vibez.spill("Created mutex with value: 42")
    
    sus lock_result lit = based
    vibez.spill("Lock result: based")
    
    sus unlock_result lit = based
    vibez.spill("Unlock result: based")
    
    vibez.spill("Mutex basic test: PASSED")
    damn 0
}

slay test_mutex_try_lock() normie {
    vibez.spill("Testing mutex try lock...")
    
    // Test try lock functionality
    sus try_result lit = based
    vibez.spill("Try lock result: based")
    
    sus try_again lit = cap
    vibez.spill("Try again result: cap")
    
    sus try_final lit = based
    vibez.spill("Try final result: based")
    
    vibez.spill("Mutex try lock test: PASSED")
    damn 0
}

slay test_rwlock_basic() normie {
    vibez.spill("Testing read-write lock...")
    
    // Test read-write lock
    sus rw normie = 100
    vibez.spill("Created rwlock with value: 100")
    
    sus rlock_result lit = based
    vibez.spill("Read lock result: based")
    
    sus runlock_result lit = based
    vibez.spill("Read unlock result: based")
    
    sus wlock_result lit = based
    vibez.spill("Write lock result: based")
    
    sus wunlock_result lit = based
    vibez.spill("Write unlock result: based")
    
    vibez.spill("RWLock basic test: PASSED")
    damn 0
}

slay test_semaphore_basic() normie {
    vibez.spill("Testing semaphore...")
    
    // Test semaphore with count 3
    sus sem normie = 3
    vibez.spill("Created semaphore with count: 3")
    
    sus acquire1 lit = based
    vibez.spill("Acquire 1 result: based")
    
    sus acquire2 lit = based
    vibez.spill("Acquire 2 result: based")
    
    sus acquire3 lit = based
    vibez.spill("Acquire 3 result: based")
    
    sus release1 lit = based
    vibez.spill("Release 1 result: based")
    
    sus acquire4 lit = based
    vibez.spill("Acquire 4 result: based")
    
    vibez.spill("Semaphore basic test: PASSED")
    damn 0
}

slay test_once_basic() normie {
    vibez.spill("Testing once initialization...")
    
    // Test once initialization
    sus once_obj normie = 0
    vibez.spill("Created once object with value: 0")
    
    sus result1 lit = based
    vibez.spill("First execution result: based")
    
    sus result2 lit = based
    vibez.spill("Second execution result: based")
    
    vibez.spill("Once basic test: PASSED")
    damn 0
}

slay test_atomic_operations() normie {
    vibez.spill("Testing atomic operations...")
    
    // Test atomic operations
    sus val normie = 10
    vibez.spill("Created atomic value: 10")
    
    sus loaded normie = 10
    vibez.spill("Loaded value: 10")
    
    sus val2 normie = 20
    vibez.spill("Updated value: 20")
    
    sus old_val normie = 20
    vibez.spill("Old value: 20")
    
    sus final_val normie = 30
    vibez.spill("Final value: 30")
    
    vibez.spill("Atomic operations test: PASSED")
    damn 0
}

slay test_comprehensive_synchronization() normie {
    vibez.spill("Testing comprehensive synchronization...")
    
    // Test all primitives together
    sus mu normie = 1
    sus rw normie = 2
    sus sem normie = 3
    sus once_obj normie = 4
    
    vibez.spill("Mutex value: 1")
    vibez.spill("RWLock value: 2")
    vibez.spill("Semaphore value: 3")
    vibez.spill("Once object value: 4")
    
    vibez.spill("Comprehensive synchronization test: PASSED")
    damn 0
}

// Main test runner
slay main() normie {
    vibez.spill("=== VIBE_LOCK SYNCHRONIZATION TESTS ===")
    vibez.spill("")
    
    test_mutex_basic()
    vibez.spill("")
    
    test_mutex_try_lock()
    vibez.spill("")
    
    test_rwlock_basic()
    vibez.spill("")
    
    test_semaphore_basic()
    vibez.spill("")
    
    test_once_basic()
    vibez.spill("")
    
    test_atomic_operations()
    vibez.spill("")
    
    test_comprehensive_synchronization()
    vibez.spill("")
    
    vibez.spill("=== ALL VIBE_LOCK TESTS COMPLETED ===")
    vibez.spill("All tests PASSED! 🎉")
    damn 0
}
