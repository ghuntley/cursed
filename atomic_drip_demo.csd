# CURSED Atomic Operations Demo
# Demonstrates the atomic_drip module functionality

yeet "atomic_drip"

slay main() {
    vibez.spill("🧪 CURSED Atomic Operations Demo")
    vibez.spill("=================================")
    
    # Demo 1: Atomic Counter
    vibez.spill("\n🔢 Atomic Counter Demo:")
    sus counter *AtomicCounter = atomic_counter_new(0)
    
    vibez.spill("Initial counter value:", atomic_counter_get(counter))
    
    # Increment counter multiple times
    bestie i := 0; i < 5; i++ {
        sus old normie = atomic_counter_increment(counter)
        vibez.spill("Incremented counter from", old, "to", atomic_counter_get(counter))
    }
    
    # Demo 2: Atomic i32 Operations
    vibez.spill("\n🔧 Atomic i32 Operations Demo:")
    sus atomic *AtomicI32 = atomic_i32_new(100)
    
    vibez.spill("Initial value:", atomic_load_i32(atomic))
    
    # Atomic add
    sus old_add normie = atomic_add_i32(atomic, 25)
    vibez.spill("Added 25, old value:", old_add, "new value:", atomic_load_i32(atomic))
    
    # Atomic subtract
    sus old_sub normie = atomic_sub_i32(atomic, 15)
    vibez.spill("Subtracted 15, old value:", old_sub, "new value:", atomic_load_i32(atomic))
    
    # Demo 3: Compare-and-Swap
    vibez.spill("\n🔄 Compare-and-Swap Demo:")
    sus cas_atomic *AtomicI32 = atomic_i32_new(50)
    
    vibez.spill("Initial value:", atomic_load_i32(cas_atomic))
    
    # Successful CAS
    sus success lit = atomic_cas_i32(cas_atomic, 50, 75)
    vibez.spill("CAS(50 -> 75):", success, "value:", atomic_load_i32(cas_atomic))
    
    # Failed CAS
    sus failure lit = atomic_cas_i32(cas_atomic, 50, 100)
    vibez.spill("CAS(50 -> 100):", failure, "value:", atomic_load_i32(cas_atomic))
    
    # Demo 4: Atomic Flag
    vibez.spill("\n🚩 Atomic Flag Demo:")
    sus flag *AtomicFlag = atomic_flag_new()
    
    sus initial lit = atomic_flag_test_and_set(flag)
    vibez.spill("Initial flag test-and-set:", initial)
    
    sus second lit = atomic_flag_test_and_set(flag)
    vibez.spill("Second flag test-and-set:", second)
    
    atomic_flag_clear(flag)
    vibez.spill("Flag cleared")
    
    sus after_clear lit = atomic_flag_test_and_set(flag)
    vibez.spill("After clear test-and-set:", after_clear)
    
    # Demo 5: Atomic Swap
    vibez.spill("\n🔀 Atomic Swap Demo:")
    sus swap_atomic *AtomicI32 = atomic_i32_new(200)
    
    vibez.spill("Initial value:", atomic_load_i32(swap_atomic))
    
    sus old_swap normie = atomic_swap_i32(swap_atomic, 300)
    vibez.spill("Swapped to 300, old value:", old_swap, "new value:", atomic_load_i32(swap_atomic))
    
    # Demo 6: Memory Fence
    vibez.spill("\n🛡️ Memory Fence Demo:")
    vibez.spill("Calling memory_fence()...")
    memory_fence()
    vibez.spill("Memory fence completed")
    
    vibez.spill("\n🎉 Atomic operations demo completed successfully!")
}
