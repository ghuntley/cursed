# CURSED Atomic Operations Syntax Validation
# Quick syntax check for the new atomic_drip module

yeet "atomic_drip"

# Test that basic atomic types can be created
slay test_atomic_types() {
    # Basic atomic types
    sus atomic_i32 *AtomicI32 = atomic_i32_new(42)
    sus atomic_i64 *AtomicI64 = atomic_i64_new(1000)
    sus atomic_flag *AtomicFlag = atomic_flag_new()
    sus atomic_counter *AtomicCounter = atomic_counter_new(0)
    sus atomic_ptr *AtomicPtr = atomic_ptr_new(0x1000.(*void))
    
    # Test basic operations compile
    sus value_i32 normie = atomic_load_i32(atomic_i32)
    sus value_i64 thicc = atomic_load_i64(atomic_i64)
    sus flag_set lit = atomic_flag_is_set(atomic_flag)
    sus counter_val normie = atomic_counter_get(atomic_counter)
    sus ptr_val *void = atomic_ptr_load(atomic_ptr)
    
    # Test arithmetic operations
    sus old_add normie = atomic_add_i32(atomic_i32, 5)
    sus old_inc normie = atomic_increment_i32(atomic_i32)
    sus cas_success lit = atomic_cas_i32(atomic_i32, 42, 50)
    
    # Test memory ordering
    memory_fence()
    acquire_fence()
    release_fence()
    compiler_fence()
    
    # Test synchronization primitives
    sus lock *Spinlock = spinlock_new()
    sus rw_lock *RwSpinlock = rw_spinlock_new()
    
    sus try_lock_success lit = spinlock_try_lock(lock)
    yo try_lock_success {
        spinlock_unlock(lock)
    }
    
    sus try_write_success lit = rw_spinlock_try_write_lock(rw_lock)
    yo try_write_success {
        rw_spinlock_write_unlock(rw_lock)
    }
}

# Test memory ordering constants
slay test_memory_ordering_constants() {
    sus relaxed normie = MEMORY_ORDER_RELAXED
    sus acquire normie = MEMORY_ORDER_ACQUIRE  
    sus release normie = MEMORY_ORDER_RELEASE
    sus acq_rel normie = MEMORY_ORDER_ACQ_REL
    sus seq_cst normie = MEMORY_ORDER_SEQ_CST
    
    # Test ordered operations
    sus atomic *AtomicI32 = atomic_i32_new(1)
    atomic_store_i32_ordered(atomic, 2, relaxed)
    sus value normie = atomic_load_i32_ordered(atomic, acquire)
    sus cas_ordered lit = atomic_cas_i32_ordered(atomic, 2, 3, seq_cst)
}

# Main syntax validation
slay main() {
    test_atomic_types()
    test_memory_ordering_constants()
    
    # If we get here, syntax is valid
}
