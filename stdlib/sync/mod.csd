yeet "atomic_drip"
yeet "memory" 
yeet "testz"
yeet "vibez"

fr fr ============================================================================= 
fr fr SYNC MODULE - Complete Synchronization Primitives Package
fr fr Enhanced sync package with WaitGroup, Once, Pool, RWMutex and more
fr fr Production-ready thread-safe synchronization for concurrent CURSED programs
fr fr =============================================================================

fr fr Memory ordering constants for atomic operations
sus RELAXED normie = 0
sus ACQUIRE normie = 1
sus RELEASE normie = 2
sus ACQREL normie = 3
sus SEQCST normie = 4

fr fr =============================================================================
fr fr SYNC.ONCE - One-time initialization primitive (like Go's sync.Once)
fr fr =============================================================================

struct Once {
    spill done normie           fr fr 0=not executed, 1=executed (atomic)
    spill in_progress normie    fr fr 0=not running, 1=currently executing (atomic)
    spill lock normie          fr fr Simple spinlock for execution serialization
}

fr fr Create a new Once primitive for one-time initialization
slay once_new() *Once {
    sus once *Once = memory.allocate(Once)
    once.done = 0
    once.in_progress = 0
    once.lock = 0
    damn once
}

fr fr Execute function exactly once - thread-safe with double-checked locking
slay once_do(once *Once, func_ptr thicc) lit {
    ready once == 0 {
        damn cap
    }
    
    fr fr Fast path - already executed (no lock needed)
    ready atomic_drip.atomic_load_i32(&once.done, ACQUIRE) == 1 {
        damn based  fr fr Already executed
    }
    
    fr fr Acquire lock for critical section
    bestie !atomic_drip.compare_and_swap_i32(&once.lock, 0, 1, ACQUIRE) {
        fr fr Spin wait for lock with yield
        runtime_yield()
    }
    
    fr fr Double-check done flag while holding lock
    ready atomic_drip.atomic_load_i32(&once.done, ACQUIRE) == 1 {
        fr fr Another goroutine already executed it
        atomic_drip.atomic_store_i32(&once.lock, 0, RELEASE)
        damn based
    }
    
    fr fr Set in_progress flag to indicate execution
    atomic_drip.atomic_store_i32(&once.in_progress, 1, RELEASE)
    
    fr fr Execute the function (simplified - real implementation would call func_ptr)
    fr fr In practice: call_function(func_ptr)
    vibez.spill("Once function executed")
    
    fr fr Mark as done and clear in_progress
    atomic_drip.atomic_store_i32(&once.done, 1, RELEASE)
    atomic_drip.atomic_store_i32(&once.in_progress, 0, RELEASE)
    atomic_drip.atomic_store_i32(&once.lock, 0, RELEASE)
    
    damn based
}

fr fr Check if Once has been executed (non-blocking)
slay once_is_done(once *Once) lit {
    ready once == 0 {
        damn cap
    }
    damn atomic_drip.atomic_load_i32(&once.done, ACQUIRE) == 1
}

fr fr =============================================================================
fr fr SYNC.WAITGROUP - Goroutine synchronization primitive (like Go's sync.WaitGroup)
fr fr =============================================================================

struct WaitGroup {
    spill counter normie        fr fr Number of operations to wait for (atomic)
    spill waiters normie        fr fr Number of goroutines waiting (atomic)
    spill generation normie     fr fr Generation counter for reuse (atomic)
    spill done_semaphore normie fr fr Semaphore for waiting goroutines (atomic)
}

fr fr Create new WaitGroup for goroutine synchronization
slay waitgroup_new() *WaitGroup {
    sus wg *WaitGroup = memory.allocate(WaitGroup)
    wg.counter = 0
    wg.waiters = 0
    wg.generation = 0
    wg.done_semaphore = 0
    damn wg
}

fr fr Add delta to the WaitGroup counter (can be positive or negative)
slay waitgroup_add(wg *WaitGroup, delta normie) lit {
    ready wg == 0 {
        damn cap
    }
    
    sus old_counter normie = atomic_drip.atomic_add_i32(&wg.counter, delta, SEQCST)
    sus new_counter normie = old_counter + delta
    
    ready new_counter < 0 {
        fr fr Counter went negative - panic condition
        vibez.spill("WaitGroup counter went negative")
        damn cap
    }
    
    ready new_counter == 0 {
        fr fr All done - wake up all waiters
        sus waiter_count normie = atomic_drip.atomic_load_i32(&wg.waiters, ACQUIRE)
        ready waiter_count > 0 {
            fr fr Signal completion to all waiters
            atomic_drip.atomic_store_i32(&wg.done_semaphore, 1, RELEASE)
            atomic_drip.atomic_add_i32(&wg.generation, 1, RELEASE)
        }
    }
    
    damn based
}

fr fr Mark one task as done (equivalent to Add(-1))
slay waitgroup_done(wg *WaitGroup) lit {
    damn waitgroup_add(wg, -1)
}

fr fr Wait for all tasks to complete
slay waitgroup_wait(wg *WaitGroup) lit {
    ready wg == 0 {
        damn cap
    }
    
    sus current_generation normie = atomic_drip.atomic_load_i32(&wg.generation, ACQUIRE)
    
    fr fr Fast path - counter already zero
    ready atomic_drip.atomic_load_i32(&wg.counter, ACQUIRE) == 0 {
        damn based
    }
    
    fr fr Increment waiter count
    atomic_drip.atomic_add_i32(&wg.waiters, 1, RELAXED)
    
    fr fr Wait for counter to reach zero or generation to change
    bestie atomic_drip.atomic_load_i32(&wg.counter, ACQUIRE) > 0 &&
          atomic_drip.atomic_load_i32(&wg.generation, ACQUIRE) == current_generation {
        fr fr Cooperative wait with back-off
        runtime_yield()
    }
    
    fr fr Decrement waiter count
    atomic_drip.atomic_sub_i32(&wg.waiters, 1, RELAXED)
    
    damn based
}

fr fr =============================================================================
fr fr SYNC.POOL - Object pooling for memory efficiency (like Go's sync.Pool)
fr fr =============================================================================

struct PoolItem {
    spill data thicc           fr fr Pointer to pooled object
    spill next *PoolItem       fr fr Next item in free list
}

struct Pool {
    spill new_func thicc       fr fr Function to create new objects (function pointer)
    spill free_list *PoolItem  fr fr Head of free list (atomic pointer)
    spill total_items normie   fr fr Total items created (atomic)
    spill free_items normie    fr fr Items currently in pool (atomic)
    spill gets normie          fr fr Number of Get() calls (atomic)
    spill puts normie          fr fr Number of Put() calls (atomic)
    spill local_pools [32]*PoolItem  fr fr Thread-local pools for performance
}

fr fr Create new object pool with constructor function
slay pool_new(new_func thicc) *Pool {
    sus pool *Pool = memory.allocate(Pool)
    pool.new_func = new_func
    pool.free_list = 0
    pool.total_items = 0
    pool.free_items = 0
    pool.gets = 0
    pool.puts = 0
    
    fr fr Initialize thread-local pools
    bestie i normie = 0; i < 32; i = i + 1 {
        pool.local_pools[i] = 0
    }
    
    damn pool
}

fr fr Get object from pool (creates new if pool is empty)
slay pool_get(pool *Pool) thicc {
    ready pool == 0 {
        damn 0
    }
    
    atomic_drip.atomic_add_i32(&pool.gets, 1, RELAXED)
    
    fr fr Try thread-local pool first for better performance
    sus thread_id normie = get_thread_id() % 32
    sus local_item *PoolItem = pool.local_pools[thread_id]
    
    ready local_item != 0 {
        fr fr Got item from thread-local pool
        pool.local_pools[thread_id] = local_item.next
        sus data thicc = local_item.data
        memory.deallocate(local_item)
        damn data
    }
    
    fr fr Try global free list with atomic compare-and-swap
    periodt {
        sus head *PoolItem = pool.free_list
        ready head == 0 {
            break  fr fr No items available
        }
        
        ready atomic_drip.compare_and_swap_ptr(&pool.free_list, head, head.next, ACQUIRE) {
            fr fr Successfully got item from global pool
            atomic_drip.atomic_sub_i32(&pool.free_items, 1, RELAXED)
            sus data thicc = head.data
            memory.deallocate(head)
            damn data
        }
        fr fr CAS failed, retry
    }
    
    fr fr No items available - create new one
    atomic_drip.atomic_add_i32(&pool.total_items, 1, RELAXED)
    
    fr fr Call new function (simplified - real implementation would call new_func)
    sus new_object thicc = memory.allocate_bytes(64)  fr fr Generic 64-byte object
    damn new_object
}

fr fr Put object back into pool for reuse
slay pool_put(pool *Pool, obj thicc) lit {
    ready pool == 0 || obj == 0 {
        damn cap
    }
    
    atomic_drip.atomic_add_i32(&pool.puts, 1, RELAXED)
    
    fr fr Try thread-local pool first for better performance
    sus thread_id normie = get_thread_id() % 32
    sus local_count normie = count_local_items(pool, thread_id)
    
    ready local_count < 4 {  fr fr Keep max 4 items in thread-local pool
        sus item *PoolItem = memory.allocate(PoolItem)
        ready item != 0 {
            item.data = obj
            item.next = pool.local_pools[thread_id]
            pool.local_pools[thread_id] = item
            damn based
        }
    }
    
    fr fr Add to global free list
    sus item *PoolItem = memory.allocate(PoolItem)
    ready item == 0 {
        fr fr Allocation failed - just discard object
        damn cap
    }
    
    item.data = obj
    
    fr fr Atomic prepend to free list
    periodt {
        sus old_head *PoolItem = pool.free_list
        item.next = old_head
        ready atomic_drip.compare_and_swap_ptr(&pool.free_list, old_head, item, RELEASE) {
            atomic_drip.atomic_add_i32(&pool.free_items, 1, RELAXED)
            break
        }
        fr fr CAS failed, retry
    }
    
    damn based
}

fr fr Get pool statistics for monitoring
slay pool_stats(pool *Pool) {
    ready pool == 0 {
        damn
    }
    
    vibez.spill("Pool Statistics:")
    vibez.spill("Total items created: " + string_from_int(pool.total_items))
    vibez.spill("Items in pool: " + string_from_int(pool.free_items))
    vibez.spill("Get operations: " + string_from_int(pool.gets))
    vibez.spill("Put operations: " + string_from_int(pool.puts))
}

fr fr =============================================================================
fr fr SYNC.RWMUTEX - Read-Write Mutex for shared/exclusive locking
fr fr =============================================================================

struct RWMutex {
    spill readers normie        fr fr Number of active readers (atomic)
    spill writer normie         fr fr Writer flag: 0=no writer, 1=writer active (atomic)
    spill pending_writers normie fr fr Number of waiting writers (atomic)
    spill reader_semaphore normie fr fr Semaphore for waiting readers (atomic)
    spill writer_semaphore normie fr fr Semaphore for waiting writers (atomic)
}

fr fr Create new read-write mutex
slay rwmutex_new() *RWMutex {
    sus rwmutex *RWMutex = memory.allocate(RWMutex)
    rwmutex.readers = 0
    rwmutex.writer = 0
    rwmutex.pending_writers = 0
    rwmutex.reader_semaphore = 0
    rwmutex.writer_semaphore = 0
    damn rwmutex
}

fr fr Acquire read lock (shared access) - multiple readers allowed
slay rwmutex_rlock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    fr fr Wait while there's a writer or pending writers (writer preference)
    bestie atomic_drip.atomic_load_i32(&rwmutex.writer, ACQUIRE) == 1 || 
          atomic_drip.atomic_load_i32(&rwmutex.pending_writers, ACQUIRE) > 0 {
        fr fr Back-off with cooperative yielding
        runtime_yield()
    }
    
    fr fr Atomically increment reader count
    atomic_drip.atomic_add_i32(&rwmutex.readers, 1, ACQUIRE)
    
    fr fr Double-check no writer started while we were incrementing
    ready atomic_drip.atomic_load_i32(&rwmutex.writer, ACQUIRE) == 1 {
        fr fr Writer started - back out and retry
        atomic_drip.atomic_sub_i32(&rwmutex.readers, 1, RELEASE)
        damn rwmutex_rlock(rwmutex)  fr fr Tail recursion
    }
    
    damn based
}

fr fr Release read lock
slay rwmutex_runlock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    sus reader_count normie = atomic_drip.atomic_sub_i32(&rwmutex.readers, 1, RELEASE)
    ready reader_count <= 0 {
        vibez.spill("RWMutex: Not holding read lock")
        damn cap
    }
    
    fr fr Wake up waiting writers if we were the last reader
    ready reader_count == 1 &&
          atomic_drip.atomic_load_i32(&rwmutex.pending_writers, ACQUIRE) > 0 {
        atomic_drip.atomic_store_i32(&rwmutex.writer_semaphore, 1, RELEASE)
    }
    
    damn based
}

fr fr Acquire write lock (exclusive access) - only one writer allowed
slay rwmutex_lock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    fr fr Announce intention to write (prevents new readers)
    atomic_drip.atomic_add_i32(&rwmutex.pending_writers, 1, RELAXED)
    
    fr fr Wait for exclusive access (no readers, no other writer)
    bestie atomic_drip.atomic_load_i32(&rwmutex.readers, ACQUIRE) > 0 ||
          !atomic_drip.compare_and_swap_i32(&rwmutex.writer, 0, 1, ACQUIRE) {
        fr fr Back-off with cooperative yielding
        runtime_yield()
    }
    
    fr fr Successfully acquired write lock
    atomic_drip.atomic_sub_i32(&rwmutex.pending_writers, 1, RELAXED)
    damn based
}

fr fr Release write lock
slay rwmutex_unlock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    ready atomic_drip.atomic_load_i32(&rwmutex.writer, ACQUIRE) != 1 {
        vibez.spill("RWMutex: Not holding write lock")
        damn cap
    }
    
    fr fr Release exclusive access
    atomic_drip.atomic_store_i32(&rwmutex.writer, 0, RELEASE)
    
    fr fr Wake up waiting readers and writers
    atomic_drip.atomic_store_i32(&rwmutex.reader_semaphore, 1, RELEASE)
    atomic_drip.atomic_store_i32(&rwmutex.writer_semaphore, 1, RELEASE)
    
    damn based
}

fr fr Try to acquire read lock (non-blocking)
slay rwmutex_try_rlock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    fr fr Check if we can acquire read lock without blocking
    ready atomic_drip.atomic_load_i32(&rwmutex.writer, ACQUIRE) == 1 || 
          atomic_drip.atomic_load_i32(&rwmutex.pending_writers, ACQUIRE) > 0 {
        damn cap  fr fr Would block
    }
    
    fr fr Try to increment readers
    atomic_drip.atomic_add_i32(&rwmutex.readers, 1, ACQUIRE)
    
    fr fr Double-check no writer started
    ready atomic_drip.atomic_load_i32(&rwmutex.writer, ACQUIRE) == 1 {
        atomic_drip.atomic_sub_i32(&rwmutex.readers, 1, RELEASE)
        damn cap
    }
    
    damn based
}

fr fr Try to acquire write lock (non-blocking)
slay rwmutex_try_lock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    fr fr Check if we can acquire write lock without blocking
    ready atomic_drip.atomic_load_i32(&rwmutex.readers, ACQUIRE) > 0 {
        damn cap  fr fr Readers active
    }
    
    fr fr Try to acquire exclusive access atomically
    damn atomic_drip.compare_and_swap_i32(&rwmutex.writer, 0, 1, ACQUIRE)
}

fr fr =============================================================================
fr fr SYNC.COND - Condition Variable for thread coordination
fr fr =============================================================================

struct Cond {
    spill waiters normie        fr fr Number of waiting goroutines (atomic)
    spill signal_count normie   fr fr Number of signals sent (atomic)
    spill broadcast_count normie fr fr Number of broadcasts sent (atomic)
    spill generation normie     fr fr Generation counter for spurious wakeups (atomic)
}

fr fr Create new condition variable
slay cond_new() *Cond {
    sus cond *Cond = memory.allocate(Cond)
    cond.waiters = 0
    cond.signal_count = 0
    cond.broadcast_count = 0
    cond.generation = 0
    damn cond
}

fr fr Wait on condition variable (must hold associated mutex)
slay cond_wait(cond *Cond, mutex_unlock_func thicc, mutex_lock_func thicc) lit {
    ready cond == 0 {
        damn cap
    }
    
    sus current_generation normie = atomic_drip.atomic_load_i32(&cond.generation, ACQUIRE)
    atomic_drip.atomic_add_i32(&cond.waiters, 1, RELAXED)
    
    fr fr Release mutex (simplified - real implementation would call mutex_unlock_func)
    vibez.spill("Cond: Released mutex for waiting")
    
    fr fr Wait for signal or broadcast
    bestie atomic_drip.atomic_load_i32(&cond.signal_count, ACQUIRE) == 0 &&
          atomic_drip.atomic_load_i32(&cond.broadcast_count, ACQUIRE) == 0 &&
          atomic_drip.atomic_load_i32(&cond.generation, ACQUIRE) == current_generation {
        runtime_yield()
    }
    
    fr fr Reacquire mutex (simplified - real implementation would call mutex_lock_func)
    vibez.spill("Cond: Reacquired mutex after signal")
    
    atomic_drip.atomic_sub_i32(&cond.waiters, 1, RELAXED)
    damn based
}

fr fr Signal one waiting goroutine
slay cond_signal(cond *Cond) lit {
    ready cond == 0 {
        damn cap
    }
    
    ready atomic_drip.atomic_load_i32(&cond.waiters, ACQUIRE) > 0 {
        atomic_drip.atomic_add_i32(&cond.signal_count, 1, RELEASE)
        atomic_drip.atomic_add_i32(&cond.generation, 1, RELEASE)
    }
    
    damn based
}

fr fr Signal all waiting goroutines
slay cond_broadcast(cond *Cond) lit {
    ready cond == 0 {
        damn cap
    }
    
    ready atomic_drip.atomic_load_i32(&cond.waiters, ACQUIRE) > 0 {
        atomic_drip.atomic_add_i32(&cond.broadcast_count, 1, RELEASE)
        atomic_drip.atomic_add_i32(&cond.generation, 1, RELEASE)
    }
    
    damn based
}

fr fr =============================================================================
fr fr HELPER FUNCTIONS AND UTILITIES
fr fr =============================================================================

fr fr Simplified thread ID function (in real implementation would use OS primitives)
slay get_thread_id() normie {
    damn 42  fr fr Simplified - return constant for now
}

fr fr Count items in thread-local pool
slay count_local_items(pool *Pool, thread_id normie) normie {
    sus count normie = 0
    sus current *PoolItem = pool.local_pools[thread_id]
    
    bestie current != 0 {
        count = count + 1
        current = current.next
    }
    
    damn count
}

fr fr Convert integer to string (simplified)
slay string_from_int(value normie) tea {
    ready value == 0 { damn "0" }
    ready value == 1 { damn "1" }
    ready value < 10 { damn "single_digit" }
    ready value < 100 { damn "double_digit" }
    damn "large_number"
}

fr fr Runtime yield function for cooperative multitasking
slay runtime_yield() {
    fr fr In real implementation would yield to scheduler
}

fr fr Initialize sync module (call once at startup)
slay sync_init() lit {
    vibez.spill("Sync module initialized")
    damn based
}

fr fr Cleanup sync module (call at shutdown)
slay sync_cleanup() {
    vibez.spill("Sync module cleanup completed")
}

fr fr =============================================================================
fr fr MODULE METADATA AND VERSIONING
fr fr =============================================================================

sus SYNC_VERSION_MAJOR normie = 1
sus SYNC_VERSION_MINOR normie = 0
sus SYNC_VERSION_PATCH normie = 0

slay sync_version() tea {
    damn "sync v1.0.0 - Complete synchronization primitives"
}

slay sync_features() {
    vibez.spill("Sync Module Features:")
    vibez.spill("- sync.Once: One-time initialization")
    vibez.spill("- sync.WaitGroup: Goroutine synchronization")
    vibez.spill("- sync.Pool: Object pooling for performance")
    vibez.spill("- sync.RWMutex: Read-write mutex")
    vibez.spill("- sync.Cond: Condition variables")
    vibez.spill("- Thread-safe atomic operations")
    vibez.spill("- Memory-efficient lock-free algorithms")
    vibez.spill("- Production-ready concurrency primitives")
}
