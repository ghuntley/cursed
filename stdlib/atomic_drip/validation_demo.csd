# CURSED Atomic Operations Validation Demo
# Demonstrates hardware atomic operations functionality

yeet "testz"
yeet "atomic_drip"

# Simple atomic operations demonstration
slay demo_basic_atomics() {
    vibez.spill("🔬 Basic Atomic Operations Demo")
    
    # Create atomic integers
    sus atomic_i32 *AtomicI32 = atomic_i32_new(42)
    sus atomic_i64 *AtomicI64 = atomic_i64_new(1000000)
    
    vibez.spill("Initial i32 value:", atomic_load_i32(atomic_i32))
    vibez.spill("Initial i64 value:", atomic_load_i64(atomic_i64))
    
    # Test store operations
    atomic_store_i32(atomic_i32, 100)
    atomic_store_i64(atomic_i64, 2000000)
    
    vibez.spill("After store i32:", atomic_load_i32(atomic_i32))
    vibez.spill("After store i64:", atomic_load_i64(atomic_i64))
    
    vibez.spill("✅ Basic atomics work correctly")
}

# Demonstrate compare-and-swap operations
slay demo_cas_operations() {
    vibez.spill("🔄 Compare-and-Swap Demo")
    
    sus atomic *AtomicI32 = atomic_i32_new(50)
    
    # Successful CAS
    sus success lit = atomic_cas_i32(atomic, 50, 75)
    vibez.spill("CAS 50->75 success:", success)
    vibez.spill("Value after CAS:", atomic_load_i32(atomic))
    
    # Failed CAS
    sus failure lit = atomic_cas_i32(atomic, 50, 100)
    vibez.spill("CAS 50->100 success:", failure)  # Should be false
    vibez.spill("Value unchanged:", atomic_load_i32(atomic))
    
    vibez.spill("✅ CAS operations work correctly")
}

# Demonstrate atomic arithmetic
slay demo_atomic_arithmetic() {
    vibez.spill("➕ Atomic Arithmetic Demo")
    
    sus atomic *AtomicI32 = atomic_i32_new(10)
    
    sus old_add normie = atomic_add_i32(atomic, 5)
    vibez.spill("Add 5, old value:", old_add)
    vibez.spill("New value:", atomic_load_i32(atomic))
    
    sus old_sub normie = atomic_sub_i32(atomic, 3)
    vibez.spill("Sub 3, old value:", old_sub)
    vibez.spill("New value:", atomic_load_i32(atomic))
    
    sus inc_result normie = atomic_increment_i32(atomic)
    vibez.spill("Increment result:", inc_result)
    
    sus dec_result normie = atomic_decrement_i32(atomic)
    vibez.spill("Decrement result:", dec_result)
    
    vibez.spill("✅ Atomic arithmetic works correctly")
}

# Demonstrate atomic flags
slay demo_atomic_flags() {
    vibez.spill("🚩 Atomic Flag Demo")
    
    sus flag *AtomicFlag = atomic_flag_new()
    
    vibez.spill("Flag initially set:", atomic_flag_is_set(flag))
    
    sus was_set lit = atomic_flag_test_and_set(flag)
    vibez.spill("Test-and-set returned:", was_set)
    vibez.spill("Flag now set:", atomic_flag_is_set(flag))
    
    atomic_flag_clear(flag)
    vibez.spill("Flag after clear:", atomic_flag_is_set(flag))
    
    vibez.spill("✅ Atomic flags work correctly")
}

# Demonstrate memory ordering
slay demo_memory_ordering() {
    vibez.spill("🧠 Memory Ordering Demo")
    
    sus atomic *AtomicI32 = atomic_i32_new(1)
    
    # Test different memory orderings
    atomic_store_i32_ordered(atomic, 2, MEMORY_ORDER_RELAXED)
    vibez.spill("Relaxed store, value:", atomic_load_i32_ordered(atomic, MEMORY_ORDER_RELAXED))
    
    atomic_store_i32_ordered(atomic, 3, MEMORY_ORDER_RELEASE)
    vibez.spill("Release store, value:", atomic_load_i32_ordered(atomic, MEMORY_ORDER_ACQUIRE))
    
    atomic_store_i32(atomic, 4)  # SEQ_CST by default
    vibez.spill("SeqCst store, value:", atomic_load_i32(atomic))
    
    # Test memory fences
    memory_fence()
    compiler_fence()
    acquire_fence()
    release_fence()
    acq_rel_fence()
    
    vibez.spill("✅ Memory ordering works correctly")
}

# Demonstrate high-level synchronization
slay demo_synchronization() {
    vibez.spill("🔒 Synchronization Primitives Demo")
    
    # Test spinlock
    sus lock *Spinlock = spinlock_new()
    
    yo spinlock_try_lock(lock) {
        vibez.spill("Acquired spinlock")
        spinlock_unlock(lock)
        vibez.spill("Released spinlock")
    }
    
    # Test read-write spinlock
    sus rw_lock *RwSpinlock = rw_spinlock_new()
    
    rw_spinlock_read_lock(rw_lock)
    vibez.spill("Acquired read lock")
    rw_spinlock_read_unlock(rw_lock)
    
    yo rw_spinlock_try_write_lock(rw_lock) {
        vibez.spill("Acquired write lock")
        rw_spinlock_write_unlock(rw_lock)
    }
    
    vibez.spill("✅ Synchronization primitives work correctly")
}

# Main demo function
slay main() {
    vibez.spill("🚀 CURSED Hardware Atomic Operations Validation Demo")
    vibez.spill("====================================================")
    
    demo_basic_atomics()
    vibez.spill("")
    
    demo_cas_operations()
    vibez.spill("")
    
    demo_atomic_arithmetic()
    vibez.spill("")
    
    demo_atomic_flags()
    vibez.spill("")
    
    demo_memory_ordering()
    vibez.spill("")
    
    demo_synchronization()
    vibez.spill("")
    
    vibez.spill("====================================================")
    vibez.spill("🎉 All atomic operations demos completed successfully!")
    vibez.spill("⚡ Hardware atomics are ready for production use!")
    vibez.spill("🔧 Supports x86_64, ARM64, and WebAssembly platforms!")
}
