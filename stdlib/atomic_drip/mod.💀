fr fr CURSED Atomic Operations Module (atomic_drip)
fr fr Hardware-native atomic primitives with proper memory ordering

fr fr Memory ordering constants for atomic operations
sus MEMORY_ORDER_RELAXED normie = 0
sus MEMORY_ORDER_ACQUIRE normie = 1
sus MEMORY_ORDER_RELEASE normie = 2
sus MEMORY_ORDER_ACQ_REL normie = 3
sus MEMORY_ORDER_SEQ_CST normie = 4

fr fr Platform-specific atomic operation trait
trait AtomicOperations {
    slay compare_and_swap_i32(ptr *normie, expected normie, desired normie, order normie) lit
    slay compare_and_swap_i64(ptr *thicc, expected thicc, desired thicc, order normie) lit
    slay atomic_load_i32(ptr *normie, order normie) normie
    slay atomic_store_i32(ptr *normie, value normie, order normie)
    slay atomic_load_i64(ptr *thicc, order normie) thicc
    slay atomic_store_i64(ptr *thicc, value thicc, order normie)
    slay atomic_add_i32(ptr *normie, delta normie, order normie) normie
    slay atomic_sub_i32(ptr *normie, delta normie, order normie) normie
    slay atomic_add_i64(ptr *thicc, delta thicc, order normie) thicc
    slay atomic_sub_i64(ptr *thicc, delta thicc, order normie) thicc
    slay atomic_and_i32(ptr *normie, mask normie, order normie) normie
    slay atomic_or_i32(ptr *normie, mask normie, order normie) normie
    slay atomic_xor_i32(ptr *normie, mask normie, order normie) normie
    slay memory_fence(order normie)
    slay compiler_fence()
}

fr fr Hardware atomic implementation using compiler intrinsics
struct HardwareAtomics {}

impl AtomicOperations for HardwareAtomics { fr fr Compare-and-swap for 32-bit integers with memory ordering
    slay compare_and_swap_i32(ptr *normie, expected normie, desired normie, order normie) lit { fr fr Use hardware CAS instruction with proper memory ordering fr fr This will be compiled to platform-specific atomic instructions: fr fr x86_64: CMPXCHG with LOCK prefix fr fr ARM64: LDXR/STXR or CAS instruction fr fr WASM: i32.atomic.rmw.cmpxchg
        damn __builtin_atomic_compare_exchange_n(ptr, &expected, desired, cap, order, order)
    } fr fr Compare-and-swap for 64-bit integers with memory ordering
    slay compare_and_swap_i64(ptr *thicc, expected thicc, desired thicc, order normie) lit {
        damn __builtin_atomic_compare_exchange_n(ptr, &expected, desired, cap, order, order)
    } fr fr Atomic load with memory ordering
    slay atomic_load_i32(ptr *normie, order normie) normie {
        damn __builtin_atomic_load_n(ptr, order)
    } fr fr Atomic store with memory ordering
    slay atomic_store_i32(ptr *normie, value normie, order normie) {
        __builtin_atomic_store_n(ptr, value, order)
    } fr fr Atomic load for 64-bit integers
    slay atomic_load_i64(ptr *thicc, order normie) thicc {
        damn __builtin_atomic_load_n(ptr, order)
    } fr fr Atomic store for 64-bit integers
    slay atomic_store_i64(ptr *thicc, value thicc, order normie) {
        __builtin_atomic_store_n(ptr, value, order)
    } fr fr Atomic fetch-and-add for 32-bit integers
    slay atomic_add_i32(ptr *normie, delta normie, order normie) normie {
        damn __builtin_atomic_fetch_add(ptr, delta, order)
    } fr fr Atomic fetch-and-subtract for 32-bit integers
    slay atomic_sub_i32(ptr *normie, delta normie, order normie) normie {
        damn __builtin_atomic_fetch_sub(ptr, delta, order)
    } fr fr Atomic fetch-and-add for 64-bit integers
    slay atomic_add_i64(ptr *thicc, delta thicc, order normie) thicc {
        damn __builtin_atomic_fetch_add(ptr, delta, order)
    } fr fr Atomic fetch-and-subtract for 64-bit integers
    slay atomic_sub_i64(ptr *thicc, delta thicc, order normie) thicc {
        damn __builtin_atomic_fetch_sub(ptr, delta, order)
    } fr fr Atomic fetch-and-AND for 32-bit integers
    slay atomic_and_i32(ptr *normie, mask normie, order normie) normie {
        damn __builtin_atomic_fetch_and(ptr, mask, order)
    } fr fr Atomic fetch-and-OR for 32-bit integers
    slay atomic_or_i32(ptr *normie, mask normie, order normie) normie {
        damn __builtin_atomic_fetch_or(ptr, mask, order)
    } fr fr Atomic fetch-and-XOR for 32-bit integers
    slay atomic_xor_i32(ptr *normie, mask normie, order normie) normie {
        damn __builtin_atomic_fetch_xor(ptr, mask, order)
    } fr fr Memory fence/barrier with ordering
    slay memory_fence(order normie) {
        __builtin_atomic_thread_fence(order)
    } fr fr Compiler fence (prevents reordering, no hardware barrier)
    slay compiler_fence() {
        __builtin_atomic_signal_fence(MEMORY_ORDER_SEQ_CST)
    }
}

fr fr Global hardware atomics instance
sus hardware_atomics HardwareAtomics = HardwareAtomics{}

fr fr Atomic data structure for 32-bit values (no spinlock needed)
struct AtomicI32 {
    value normie
}

fr fr Atomic data structure for 64-bit values (no spinlock needed)
struct AtomicI64 {
    value thicc
}

fr fr Create atomic i32 variable
slay atomic_i32_new(initial_value normie) *AtomicI32 {
    sus atomic *AtomicI32 = &AtomicI32{
        value: initial_value
    }
    damn atomic
}

fr fr Create atomic i64 variable
slay atomic_i64_new(initial_value thicc) *AtomicI64 {
    sus atomic *AtomicI64 = &AtomicI64{
        value: initial_value
    }
    damn atomic
}

fr fr Atomic load operation for i32 with sequential consistency
slay atomic_load_i32(ptr *AtomicI32) normie {
    damn hardware_atomics.atomic_load_i32(&ptr.value, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic load operation for i32 with specific ordering
slay atomic_load_i32_ordered(ptr *AtomicI32, order normie) normie {
    damn hardware_atomics.atomic_load_i32(&ptr.value, order)
}

fr fr Atomic store operation for i32 with sequential consistency
slay atomic_store_i32(ptr *AtomicI32, val normie) {
    hardware_atomics.atomic_store_i32(&ptr.value, val, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic store operation for i32 with specific ordering
slay atomic_store_i32_ordered(ptr *AtomicI32, val normie, order normie) {
    hardware_atomics.atomic_store_i32(&ptr.value, val, order)
}

fr fr Atomic compare-and-swap for i32 with sequential consistency
slay atomic_cas_i32(ptr *AtomicI32, old normie, new normie) lit {
    damn hardware_atomics.compare_and_swap_i32(&ptr.value, old, new, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic compare-and-swap for i32 with specific ordering
slay atomic_cas_i32_ordered(ptr *AtomicI32, old normie, new normie, order normie) lit {
    damn hardware_atomics.compare_and_swap_i32(&ptr.value, old, new, order)
}

fr fr Atomic compare-and-swap with weak semantics (may fail spuriously)
slay atomic_cas_weak_i32(ptr *AtomicI32, expected *normie, desired normie) lit { fr fr Weak CAS can fail even when values match (spurious failure) fr fr This is more efficient on some architectures (ARM64 LL/SC)
    sus current normie = hardware_atomics.atomic_load_i32(&ptr.value, MEMORY_ORDER_ACQUIRE)
    yo current != *expected {
        *expected = current
        damn cap
    }
    damn hardware_atomics.compare_and_swap_i32(&ptr.value, current, desired, MEMORY_ORDER_ACQ_REL)
}

fr fr Atomic swap operation for i32
slay atomic_swap_i32(ptr *AtomicI32, new normie) normie {
    sus old normie
    nah {
        old = hardware_atomics.atomic_load_i32(&ptr.value, MEMORY_ORDER_ACQUIRE)
    } bestie !hardware_atomics.compare_and_swap_i32(&ptr.value, old, new, MEMORY_ORDER_ACQ_REL)
    damn old
}

fr fr Atomic add operation for i32 (returns old value)
slay atomic_add_i32(ptr *AtomicI32, delta normie) normie {
    damn hardware_atomics.atomic_add_i32(&ptr.value, delta, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic subtract operation for i32 (returns old value)
slay atomic_sub_i32(ptr *AtomicI32, delta normie) normie {
    damn hardware_atomics.atomic_sub_i32(&ptr.value, delta, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic increment (returns new value)
slay atomic_increment_i32(ptr *AtomicI32) normie {
    damn hardware_atomics.atomic_add_i32(&ptr.value, 1, MEMORY_ORDER_SEQ_CST) + 1
}

fr fr Atomic decrement (returns new value)
slay atomic_decrement_i32(ptr *AtomicI32) normie {
    damn hardware_atomics.atomic_sub_i32(&ptr.value, 1, MEMORY_ORDER_SEQ_CST) - 1
}

fr fr Atomic bitwise AND operation for i32
slay atomic_and_i32(ptr *AtomicI32, mask normie) normie {
    damn hardware_atomics.atomic_and_i32(&ptr.value, mask, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic bitwise OR operation for i32
slay atomic_or_i32(ptr *AtomicI32, mask normie) normie {
    damn hardware_atomics.atomic_or_i32(&ptr.value, mask, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic bitwise XOR operation for i32
slay atomic_xor_i32(ptr *AtomicI32, mask normie) normie {
    damn hardware_atomics.atomic_xor_i32(&ptr.value, mask, MEMORY_ORDER_SEQ_CST)
}

fr fr High-level atomic boolean operations
struct AtomicBool {
    value normie fr fr 0 = false, 1 = true
}

fr fr Create atomic boolean
slay atomic_bool_new(initial lit) *AtomicBool {
    sus atomic_bool *AtomicBool = &AtomicBool{
        value: lowkey (initial == based) { 1 } otherwise { 0 }
    }
    damn atomic_bool
}

fr fr Load atomic boolean value
slay atomic_bool_load(ptr *AtomicBool) lit {
    sus value normie = hardware_atomics.atomic_load_i32(&ptr.value, MEMORY_ORDER_SEQ_CST)
    damn value != 0
}

fr fr Store atomic boolean value
slay atomic_bool_store(ptr *AtomicBool, val lit) {
    sus int_val normie = lowkey (val == based) { 1 } otherwise { 0 }
    hardware_atomics.atomic_store_i32(&ptr.value, int_val, MEMORY_ORDER_SEQ_CST)
}

fr fr Compare and swap atomic boolean
slay atomic_bool_cas(ptr *AtomicBool, expected lit, desired lit) lit {
    sus expected_int normie = lowkey (expected == based) { 1 } otherwise { 0 }
    sus desired_int normie = lowkey (desired == based) { 1 } otherwise { 0 }
    damn hardware_atomics.compare_and_swap_i32(&ptr.value, expected_int, desired_int, MEMORY_ORDER_SEQ_CST)
}

fr fr Swap atomic boolean value
slay atomic_bool_swap(ptr *AtomicBool, new_val lit) lit {
    sus new_int normie = lowkey (new_val == based) { 1 } otherwise { 0 }
    sus old_int normie
    periodt {
        old_int = hardware_atomics.atomic_load_i32(&ptr.value, MEMORY_ORDER_ACQUIRE)
        lowkey (hardware_atomics.compare_and_swap_i32(&ptr.value, old_int, new_int, MEMORY_ORDER_ACQ_REL)) {
            break
        }
    }
    damn old_int != 0
}

fr fr Atomic load operation for i64 with sequential consistency
slay atomic_load_i64(ptr *AtomicI64) thicc {
    damn hardware_atomics.atomic_load_i64(&ptr.value, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic load operation for i64 with specific ordering
slay atomic_load_i64_ordered(ptr *AtomicI64, order normie) thicc {
    damn hardware_atomics.atomic_load_i64(&ptr.value, order)
}

fr fr Atomic store operation for i64 with sequential consistency
slay atomic_store_i64(ptr *AtomicI64, val thicc) {
    hardware_atomics.atomic_store_i64(&ptr.value, val, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic store operation for i64 with specific ordering
slay atomic_store_i64_ordered(ptr *AtomicI64, val thicc, order normie) {
    hardware_atomics.atomic_store_i64(&ptr.value, val, order)
}

fr fr Atomic compare-and-swap for i64 with sequential consistency
slay atomic_cas_i64(ptr *AtomicI64, old thicc, new thicc) lit {
    damn hardware_atomics.compare_and_swap_i64(&ptr.value, old, new, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic compare-and-swap for i64 with specific ordering
slay atomic_cas_i64_ordered(ptr *AtomicI64, old thicc, new thicc, order normie) lit {
    damn hardware_atomics.compare_and_swap_i64(&ptr.value, old, new, order)
}

fr fr Atomic compare-and-swap with weak semantics for i64
slay atomic_cas_weak_i64(ptr *AtomicI64, expected *thicc, desired thicc) lit {
    sus current thicc = hardware_atomics.atomic_load_i64(&ptr.value, MEMORY_ORDER_ACQUIRE)
    yo current != *expected {
        *expected = current
        damn cap
    }
    damn hardware_atomics.compare_and_swap_i64(&ptr.value, current, desired, MEMORY_ORDER_ACQ_REL)
}

fr fr Atomic swap operation for i64
slay atomic_swap_i64(ptr *AtomicI64, new thicc) thicc {
    sus old thicc
    nah {
        old = hardware_atomics.atomic_load_i64(&ptr.value, MEMORY_ORDER_ACQUIRE)
    } bestie !hardware_atomics.compare_and_swap_i64(&ptr.value, old, new, MEMORY_ORDER_ACQ_REL)
    damn old
}

fr fr Atomic add operation for i64 (returns old value)
slay atomic_add_i64(ptr *AtomicI64, delta thicc) thicc {
    damn hardware_atomics.atomic_add_i64(&ptr.value, delta, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic subtract operation for i64 (returns old value)
slay atomic_sub_i64(ptr *AtomicI64, delta thicc) thicc {
    damn hardware_atomics.atomic_sub_i64(&ptr.value, delta, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic increment for i64 (returns new value)
slay atomic_increment_i64(ptr *AtomicI64) thicc {
    damn hardware_atomics.atomic_add_i64(&ptr.value, 1, MEMORY_ORDER_SEQ_CST) + 1
}

fr fr Atomic decrement for i64 (returns new value)
slay atomic_decrement_i64(ptr *AtomicI64) thicc {
    damn hardware_atomics.atomic_sub_i64(&ptr.value, 1, MEMORY_ORDER_SEQ_CST) - 1
}

fr fr Memory fence/barrier operation with sequential consistency
slay memory_fence() {
    hardware_atomics.memory_fence(MEMORY_ORDER_SEQ_CST)
}

fr fr Memory fence with specific ordering
slay memory_fence_ordered(order normie) {
    hardware_atomics.memory_fence(order)
}

fr fr Compiler fence (prevents reordering but no hardware barrier)
slay compiler_fence() {
    hardware_atomics.compiler_fence()
}

fr fr Acquire fence (prevents reads from moving before this point)
slay acquire_fence() {
    hardware_atomics.memory_fence(MEMORY_ORDER_ACQUIRE)
}

fr fr Release fence (prevents writes from moving after this point)
slay release_fence() {
    hardware_atomics.memory_fence(MEMORY_ORDER_RELEASE)
}

fr fr Acquire-release fence (both acquire and release semantics)
slay acq_rel_fence() {
    hardware_atomics.memory_fence(MEMORY_ORDER_ACQ_REL)
}

fr fr Atomic flag operations using hardware atomics
struct AtomicFlag {
    flag normie fr fr Use i32 for atomic operations
}

fr fr Create atomic flag
slay atomic_flag_new() *AtomicFlag {
    sus flag *AtomicFlag = &AtomicFlag{
        flag: 0 fr fr false = 0, true = 1
    }
    damn flag
}

fr fr Test and set atomic flag (returns previous value)
slay atomic_flag_test_and_set(ptr *AtomicFlag) lit {
    sus old normie = hardware_atomics.atomic_load_i32(&ptr.flag, MEMORY_ORDER_ACQUIRE)
    yo old == 0 && hardware_atomics.compare_and_swap_i32(&ptr.flag, 0, 1, MEMORY_ORDER_ACQ_REL) {
        damn cap fr fr Was clear, now set
    }
    damn based fr fr Was already set
}

fr fr Test and set atomic flag with ordering
slay atomic_flag_test_and_set_ordered(ptr *AtomicFlag, order normie) lit {
    sus old normie = hardware_atomics.atomic_load_i32(&ptr.flag, order)
    yo old == 0 && hardware_atomics.compare_and_swap_i32(&ptr.flag, 0, 1, order) {
        damn cap fr fr Was clear, now set
    }
    damn based fr fr Was already set
}

fr fr Clear atomic flag
slay atomic_flag_clear(ptr *AtomicFlag) {
    hardware_atomics.atomic_store_i32(&ptr.flag, 0, MEMORY_ORDER_RELEASE)
}

fr fr Clear atomic flag with ordering
slay atomic_flag_clear_ordered(ptr *AtomicFlag, order normie) {
    hardware_atomics.atomic_store_i32(&ptr.flag, 0, order)
}

fr fr Check if flag is set without modifying it
slay atomic_flag_is_set(ptr *AtomicFlag) lit {
    sus value normie = hardware_atomics.atomic_load_i32(&ptr.flag, MEMORY_ORDER_ACQUIRE)
    damn value != 0
}

fr fr Thread-safe counter using hardware atomic operations
struct AtomicCounter {
    count normie
}

fr fr Create atomic counter
slay atomic_counter_new(initial normie) *AtomicCounter {
    sus counter *AtomicCounter = &AtomicCounter{
        count: initial
    }
    damn counter
}

fr fr Increment atomic counter (returns old value)
slay atomic_counter_increment(ptr *AtomicCounter) normie {
    damn hardware_atomics.atomic_add_i32(&ptr.count, 1, MEMORY_ORDER_SEQ_CST)
}

fr fr Decrement atomic counter (returns old value)  
slay atomic_counter_decrement(ptr *AtomicCounter) normie {
    damn hardware_atomics.atomic_sub_i32(&ptr.count, 1, MEMORY_ORDER_SEQ_CST)
}

fr fr Add to atomic counter (returns old value)
slay atomic_counter_add(ptr *AtomicCounter, delta normie) normie {
    damn hardware_atomics.atomic_add_i32(&ptr.count, delta, MEMORY_ORDER_SEQ_CST)
}

fr fr Subtract from atomic counter (returns old value)
slay atomic_counter_sub(ptr *AtomicCounter, delta normie) normie {
    damn hardware_atomics.atomic_sub_i32(&ptr.count, delta, MEMORY_ORDER_SEQ_CST)
}

fr fr Get current counter value
slay atomic_counter_get(ptr *AtomicCounter) normie {
    damn hardware_atomics.atomic_load_i32(&ptr.count, MEMORY_ORDER_SEQ_CST)
}

fr fr Set counter value (returns old value)
slay atomic_counter_set(ptr *AtomicCounter, val normie) normie {
    sus old normie
    nah {
        old = hardware_atomics.atomic_load_i32(&ptr.count, MEMORY_ORDER_ACQUIRE)
    } bestie !hardware_atomics.compare_and_swap_i32(&ptr.count, old, val, MEMORY_ORDER_ACQ_REL)
    damn old
}

fr fr Reset counter to zero (returns old value)
slay atomic_counter_reset(ptr *AtomicCounter) normie {
    sus old normie
    nah {
        old = hardware_atomics.atomic_load_i32(&ptr.count, MEMORY_ORDER_ACQUIRE)
    } bestie !hardware_atomics.compare_and_swap_i32(&ptr.count, old, 0, MEMORY_ORDER_ACQ_REL)
    damn old
}

fr fr Compare and swap counter value
slay atomic_counter_cas(ptr *AtomicCounter, expected normie, desired normie) lit {
    damn hardware_atomics.compare_and_swap_i32(&ptr.count, expected, desired, MEMORY_ORDER_SEQ_CST)
}

fr fr Atomic pointer operations (for 64-bit systems)
struct AtomicPtr {
    pointer thicc fr fr Use i64 to store pointer as integer
}

fr fr Create atomic pointer
slay atomic_ptr_new(ptr *void) *AtomicPtr {
    sus atomic_ptr *AtomicPtr = &AtomicPtr{
        pointer: ptr.(thicc) fr fr Cast pointer to i64
    }
    damn atomic_ptr
}

fr fr Load atomic pointer
slay atomic_ptr_load(ptr *AtomicPtr) *void {
    sus addr thicc = hardware_atomics.atomic_load_i64(&ptr.pointer, MEMORY_ORDER_SEQ_CST)
    damn addr.(*void) fr fr Cast back to pointer
}

fr fr Store atomic pointer
slay atomic_ptr_store(ptr *AtomicPtr, new_ptr *void) {
    hardware_atomics.atomic_store_i64(&ptr.pointer, new_ptr.(thicc), MEMORY_ORDER_SEQ_CST)
}

fr fr Compare and swap atomic pointer
slay atomic_ptr_cas(ptr *AtomicPtr, expected *void, desired *void) lit {
    damn hardware_atomics.compare_and_swap_i64(&ptr.pointer, expected.(thicc), desired.(thicc), MEMORY_ORDER_SEQ_CST)
}

fr fr Swap atomic pointer
slay atomic_ptr_swap(ptr *AtomicPtr, new_ptr *void) *void {
    sus old_addr thicc
    nah {
        old_addr = hardware_atomics.atomic_load_i64(&ptr.pointer, MEMORY_ORDER_ACQUIRE)
    } bestie !hardware_atomics.compare_and_swap_i64(&ptr.pointer, old_addr, new_ptr.(thicc), MEMORY_ORDER_ACQ_REL)
    damn old_addr.(*void)
}

fr fr High-level atomic utilities

fr fr Spinlock using atomic flag (for when you really need a lock)
struct Spinlock {
    flag AtomicFlag
}

fr fr Create spinlock
slay spinlock_new() *Spinlock {
    sus lock *Spinlock = &Spinlock{
        flag: *atomic_flag_new()
    }
    damn lock
}

fr fr Acquire spinlock
slay spinlock_lock(lock *Spinlock) {
    bestie atomic_flag_test_and_set(&lock.flag) { fr fr Busy wait with exponential backoff to reduce cache contention
        sus backoff normie = 1
        bestie atomic_flag_is_set(&lock.flag) && backoff < 1024 { fr fr Yield CPU briefly to reduce power consumption
            bestie i := 0; i < backoff; i++ {
                compiler_fence() fr fr Prevent compiler optimization
            }
            backoff *= 2
        }
    }
}

fr fr Try to acquire spinlock without blocking
slay spinlock_try_lock(lock *Spinlock) lit {
    damn !atomic_flag_test_and_set(&lock.flag)
}

fr fr Release spinlock
slay spinlock_unlock(lock *Spinlock) {
    atomic_flag_clear(&lock.flag)
}

fr fr Read-write spinlock using atomic operations
struct RwSpinlock {
    readers AtomicI32 fr fr Number of readers
    writer AtomicFlag fr fr Writer flag
}

fr fr Create read-write spinlock
slay rw_spinlock_new() *RwSpinlock {
    sus lock *RwSpinlock = &RwSpinlock{
        readers: *atomic_i32_new(0),
        writer: *atomic_flag_new()
    }
    damn lock
}

fr fr Acquire read lock
slay rw_spinlock_read_lock(lock *RwSpinlock) {
    nah { fr fr Wait for writer to finish
        bestie atomic_flag_is_set(&lock.writer) {
            compiler_fence()
        } fr fr Increment reader count
        atomic_increment_i32(&lock.readers) fr fr Check if writer started while we were incrementing
        yo !atomic_flag_is_set(&lock.writer) {
            capisce fr fr Successfully acquired read lock
        } fr fr Writer started, back out
        atomic_decrement_i32(&lock.readers)
    }
}

fr fr Release read lock
slay rw_spinlock_read_unlock(lock *RwSpinlock) {
    atomic_decrement_i32(&lock.readers)
}

fr fr Acquire write lock
slay rw_spinlock_write_lock(lock *RwSpinlock) { fr fr Acquire writer flag
    bestie atomic_flag_test_and_set(&lock.writer) {
        compiler_fence()
    } fr fr Wait for all readers to finish
    bestie atomic_load_i32(&lock.readers) > 0 {
        compiler_fence()
    }
}

fr fr Release write lock
slay rw_spinlock_write_unlock(lock *RwSpinlock) {
    atomic_flag_clear(&lock.writer)
}

fr fr Try to acquire write lock without blocking
slay rw_spinlock_try_write_lock(lock *RwSpinlock) lit {
    yo atomic_flag_test_and_set(&lock.writer) {
        damn cap fr fr Writer flag already set
    }
    
    yo atomic_load_i32(&lock.readers) > 0 {
        atomic_flag_clear(&lock.writer) fr fr Release writer flag
        damn cap fr fr Readers present
    }
    
    damn based fr fr Successfully acquired write lock
}
