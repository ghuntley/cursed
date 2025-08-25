fr fr sync - Synchronization Primitives Module
fr fr Pure CURSED synchronization for concurrent programming
fr fr Mutexes, semaphores, condition variables, and atomic operations

yeet "core"
yeet "concurrenz"

fr fr Synchronization primitive states
fact UNLOCKED normie = 0
fact LOCKED normie = 1
fact MUTEX_FREE normie = 0
fact MUTEX_BUSY normie = 1

fr fr Atomic operation results  
fact ATOMIC_SUCCESS normie = 0
fact ATOMIC_FAILED normie = 1

fr fr ===== MUTEX IMPLEMENTATION =====

struct Mutex {
    state normie,           # 0 = unlocked, 1 = locked
    owner_id normie,        # ID of owning goroutine
    wait_count normie,      # Number of waiting goroutines
    is_initialized lit      # Whether mutex is ready
}

struct RWMutex {
    readers normie,         # Number of active readers
    writer_waiting lit,     # Whether a writer is waiting
    writer_active lit,      # Whether a writer is active
    read_wait_count normie, # Readers waiting for writer to finish
    write_wait_count normie # Writers waiting
}

fr fr Create new mutex
slay new_mutex() Mutex {
    sus mutex Mutex = Mutex{
        state: UNLOCKED,
        owner_id: 0, 
        wait_count: 0,
        is_initialized: based
    }
    damn mutex
}

fr fr Lock mutex (blocking)
slay mutex_lock(m *Mutex) {
    check m.is_initialized != based {
        damn
    }
    
    # Simulate atomic compare-and-swap
    bestie m.state == LOCKED {
        m.wait_count = m.wait_count + 1
        # Yield to other goroutines
        concurrenz.yield()
    }
    
    # Acquire lock
    m.state = LOCKED
    m.owner_id = concurrenz.current_goroutine_id()
}

fr fr Try to lock mutex (non-blocking)
slay mutex_try_lock(m *Mutex) lit {
    check m.is_initialized != based {
        damn cap
    }
    
    check m.state == UNLOCKED {
        m.state = LOCKED
        m.owner_id = concurrenz.current_goroutine_id()
        damn based
    }
    
    damn cap
}

fr fr Unlock mutex
slay mutex_unlock(m *Mutex) {
    check m.is_initialized != based {
        damn
    }
    
    check m.state == LOCKED && m.owner_id == concurrenz.current_goroutine_id() {
        m.state = UNLOCKED
        m.owner_id = 0
        
        # Notify waiting goroutines
        check m.wait_count > 0 {
            m.wait_count = m.wait_count - 1
            concurrenz.wake_goroutine()
        }
    }
}

fr fr ===== READ-WRITE MUTEX IMPLEMENTATION =====

slay new_rwmutex() RWMutex {
    sus rwmutex RWMutex = RWMutex{
        readers: 0,
        writer_waiting: cap,
        writer_active: cap,
        read_wait_count: 0,
        write_wait_count: 0
    }
    damn rwmutex
}

fr fr Acquire read lock
slay rwmutex_rlock(rw *RWMutex) {
    bestie rw.writer_active || rw.writer_waiting {
        rw.read_wait_count = rw.read_wait_count + 1
        concurrenz.yield()
    }
    
    rw.readers = rw.readers + 1
}

fr fr Release read lock
slay rwmutex_runlock(rw *RWMutex) {
    check rw.readers > 0 {
        rw.readers = rw.readers - 1
        
        # Wake up waiting writer if no more readers
        check rw.readers == 0 && rw.write_wait_count > 0 {
            concurrenz.wake_goroutine()
        }
    }
}

fr fr Acquire write lock
slay rwmutex_lock(rw *RWMutex) {
    rw.writer_waiting = based
    
    bestie rw.readers > 0 || rw.writer_active {
        rw.write_wait_count = rw.write_wait_count + 1
        concurrenz.yield()
    }
    
    rw.writer_waiting = cap
    rw.writer_active = based
}

fr fr Release write lock  
slay rwmutex_unlock(rw *RWMutex) {
    check rw.writer_active {
        rw.writer_active = cap
        
        # Wake up all waiting readers or one writer
        check rw.read_wait_count > 0 {
            bestie rw.read_wait_count > 0 {
                rw.read_wait_count = rw.read_wait_count - 1
                concurrenz.wake_goroutine()
            }
        } elseif rw.write_wait_count > 0 {
            rw.write_wait_count = rw.write_wait_count - 1
            concurrenz.wake_goroutine()
        }
    }
}

fr fr ===== SEMAPHORE IMPLEMENTATION =====

struct Semaphore {
    count normie,           # Available permits
    max_count normie,       # Maximum permits
    wait_queue_size normie  # Number of waiting goroutines
}

slay new_semaphore(initial_count normie) Semaphore {
    sus sem Semaphore = Semaphore{
        count: initial_count,
        max_count: initial_count,
        wait_queue_size: 0
    }
    damn sem
}

fr fr Acquire semaphore permit
slay semaphore_acquire(sem *Semaphore) {
    bestie sem.count <= 0 {
        sem.wait_queue_size = sem.wait_queue_size + 1
        concurrenz.yield()
    }
    
    sem.count = sem.count - 1
}

fr fr Try to acquire permit (non-blocking)
slay semaphore_try_acquire(sem *Semaphore) lit {
    check sem.count > 0 {
        sem.count = sem.count - 1
        damn based
    }
    damn cap
}

fr fr Release semaphore permit
slay semaphore_release(sem *Semaphore) {
    check sem.count < sem.max_count {
        sem.count = sem.count + 1
        
        check sem.wait_queue_size > 0 {
            sem.wait_queue_size = sem.wait_queue_size - 1
            concurrenz.wake_goroutine()
        }
    }
}

fr fr ===== CONDITION VARIABLE IMPLEMENTATION =====

struct Condition {
    wait_count normie,      # Number of waiting goroutines
    signal_count normie,    # Number of pending signals
    broadcast_pending lit   # Whether broadcast is pending
}

slay new_condition() Condition {
    sus cond Condition = Condition{
        wait_count: 0,
        signal_count: 0,
        broadcast_pending: cap
    }
    damn cond
}

fr fr Wait on condition variable (must hold mutex)
slay condition_wait(cond *Condition, mutex *Mutex) {
    mutex_unlock(mutex)
    
    cond.wait_count = cond.wait_count + 1
    
    bestie cond.signal_count == 0 && !cond.broadcast_pending {
        concurrenz.yield()
    }
    
    check cond.signal_count > 0 {
        cond.signal_count = cond.signal_count - 1
    }
    
    check cond.broadcast_pending {
        cond.broadcast_pending = cap  # Clear broadcast for this waiter
    }
    
    cond.wait_count = cond.wait_count - 1
    mutex_lock(mutex)
}

fr fr Signal one waiting goroutine
slay condition_signal(cond *Condition) {
    check cond.wait_count > 0 {
        cond.signal_count = cond.signal_count + 1
        concurrenz.wake_goroutine()
    }
}

fr fr Signal all waiting goroutines
slay condition_broadcast(cond *Condition) {
    check cond.wait_count > 0 {
        cond.broadcast_pending = based
        
        sus i normie = 0
        bestie i < cond.wait_count {
            concurrenz.wake_goroutine()
            i = i + 1
        }
    }
}

fr fr ===== ATOMIC OPERATIONS =====

struct AtomicInt {
    value normie,
    lock Mutex
}

struct AtomicBool {
    value lit,
    lock Mutex  
}

slay new_atomic_int(initial normie) AtomicInt {
    sus atomic AtomicInt = AtomicInt{
        value: initial,
        lock: new_mutex()
    }
    damn atomic
}

slay new_atomic_bool(initial lit) AtomicBool {
    sus atomic AtomicBool = AtomicBool{
        value: initial,
        lock: new_mutex()
    }
    damn atomic
}

fr fr Atomic integer operations
slay atomic_load_int(atomic *AtomicInt) normie {
    mutex_lock(&atomic.lock)
    sus result normie = atomic.value
    mutex_unlock(&atomic.lock)
    damn result
}

slay atomic_store_int(atomic *AtomicInt, value normie) {
    mutex_lock(&atomic.lock)
    atomic.value = value
    mutex_unlock(&atomic.lock)
}

slay atomic_add_int(atomic *AtomicInt, delta normie) normie {
    mutex_lock(&atomic.lock)
    atomic.value = atomic.value + delta
    sus result normie = atomic.value
    mutex_unlock(&atomic.lock)
    damn result
}

slay atomic_compare_swap_int(atomic *AtomicInt, expected normie, new_value normie) lit {
    mutex_lock(&atomic.lock)
    check atomic.value == expected {
        atomic.value = new_value
        mutex_unlock(&atomic.lock)
        damn based
    }
    mutex_unlock(&atomic.lock)
    damn cap
}

fr fr Atomic boolean operations
slay atomic_load_bool(atomic *AtomicBool) lit {
    mutex_lock(&atomic.lock)
    sus result lit = atomic.value
    mutex_unlock(&atomic.lock)
    damn result
}

slay atomic_store_bool(atomic *AtomicBool, value lit) {
    mutex_lock(&atomic.lock)
    atomic.value = value
    mutex_unlock(&atomic.lock)
}

slay atomic_swap_bool(atomic *AtomicBool, new_value lit) lit {
    mutex_lock(&atomic.lock)
    sus old_value lit = atomic.value
    atomic.value = new_value
    mutex_unlock(&atomic.lock)
    damn old_value
}

fr fr ===== WAITGROUP IMPLEMENTATION =====

struct WaitGroup {
    count normie,
    mutex Mutex,
    cond Condition
}

slay new_waitgroup() WaitGroup {
    sus wg WaitGroup = WaitGroup{
        count: 0,
        mutex: new_mutex(),
        cond: new_condition()
    }
    damn wg
}

slay waitgroup_add(wg *WaitGroup, delta normie) {
    mutex_lock(&wg.mutex)
    wg.count = wg.count + delta
    
    check wg.count == 0 {
        condition_broadcast(&wg.cond)
    }
    
    mutex_unlock(&wg.mutex)
}

slay waitgroup_done(wg *WaitGroup) {
    waitgroup_add(wg, -1)
}

slay waitgroup_wait(wg *WaitGroup) {
    mutex_lock(&wg.mutex)
    
    bestie wg.count > 0 {
        condition_wait(&wg.cond, &wg.mutex)
    }
    
    mutex_unlock(&wg.mutex)
}

fr fr ===== ONCE IMPLEMENTATION =====

struct Once {
    done lit,
    mutex Mutex
}

slay new_once() Once {
    sus once Once = Once{
        done: cap,
        mutex: new_mutex()
    }
    damn once
}

slay once_do(once *Once, f func()) {
    check once.done {
        damn  # Already executed
    }
    
    mutex_lock(&once.mutex)
    
    check !once.done {
        f()
        once.done = based
    }
    
    mutex_unlock(&once.mutex)
}

fr fr ===== SYNCHRONIZATION UTILITIES =====

slay yield() {
    concurrenz.yield()
}

slay sleep_ms(milliseconds normie) {
    core.sleep_millis(milliseconds)
}

fr fr Create a synchronized map (thread-safe)
struct SyncMap {
    data map<tea, tea>,
    mutex RWMutex
}

slay new_sync_map() SyncMap {
    sus sm SyncMap = SyncMap{
        data: {},
        mutex: new_rwmutex()
    }
    damn sm
}

slay sync_map_set(sm *SyncMap, key tea, value tea) {
    rwmutex_lock(&sm.mutex)
    sm.data.set(key, value)
    rwmutex_unlock(&sm.mutex)
}

slay sync_map_get(sm *SyncMap, key tea) tea {
    rwmutex_rlock(&sm.mutex)
    sus result tea = sm.data.get(key)
    rwmutex_runlock(&sm.mutex)
    damn result
}

slay sync_map_has(sm *SyncMap, key tea) lit {
    rwmutex_rlock(&sm.mutex)
    sus result lit = sm.data.has_key(key)
    rwmutex_runlock(&sm.mutex)
    damn result
}

slay sync_map_remove(sm *SyncMap, key tea) {
    rwmutex_lock(&sm.mutex)
    sm.data.remove(key)
    rwmutex_unlock(&sm.mutex)
}

fr fr ===== MODULE INITIALIZATION =====

slay init_sync() {
    vibez.spill("sync module initialized")
}

slay get_sync_info() tea {
    damn "sync v1.0 - Synchronization Primitives for Concurrent CURSED"
}
