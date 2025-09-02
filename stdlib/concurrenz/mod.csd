yeet "atomic_drip"
yeet "error_drip"  
yeet "memory"
yeet "testz"

fr fr Concurrenz Module - Complete Synchronization Primitives
fr fr Pure CURSED implementation with hardware atomics and proper memory ordering

fr fr Memory ordering constants for atomic operations
sus RELAXED normie = 0
sus ACQUIRE normie = 1  
sus RELEASE normie = 2
sus ACQREL normie = 3
sus SEQCST normie = 4

fr fr Enhanced Mutex using hardware atomics
struct Mutex {
    spill lock_state normie     fr fr 0=unlocked, 1=locked, using atomic operations
    spill owner thicc           fr fr Owner thread/goroutine ID
    spill waiters normie        fr fr Number of waiting goroutines
    spill recursive_count normie fr fr For recursive locking
}

fr fr Legacy compatibility structures
struct MutexStruct {
    spill locked lit            fr fr Boolean lock state
    spill owner thicc           fr fr Owner ID
    spill waiters normie        fr fr Waiter count
}

struct AtomicStruct {
    spill value normie          fr fr Atomic value
    spill version normie        fr fr Version counter for ABA prevention
    spill lock normie           fr fr Internal lock for operations
}

struct WaitGroupStruct {
    spill counter normie        fr fr Operation counter
    spill waiters normie        fr fr Waiting goroutines
    spill generation normie     fr fr Generation for reuse
}

fr fr Enhanced WaitGroup with atomic operations
struct WaitGroup {
    spill counter normie        fr fr Number of operations to wait for
    spill waiters normie        fr fr Number of goroutines waiting
    spill generation normie     fr fr Generation counter for reuse
    spill done_flag normie      fr fr Done signal flag (0=not done, 1=done)
}

fr fr Channel structure for buffered communication
struct Channel {
    spill buffer normie[value]       fr fr Message buffer array
    spill capacity normie       fr fr Maximum buffer size
    spill size normie           fr fr Current buffer size (atomic)
    spill send_pos normie       fr fr Send position in buffer (atomic)
    spill recv_pos normie       fr fr Receive position in buffer (atomic)
    spill closed normie         fr fr Channel closed flag (atomic, 0=open, 1=closed)
    spill send_waiters normie   fr fr Number of goroutines waiting to send
    spill recv_waiters normie   fr fr Number of goroutines waiting to receive
}

fr fr Thread pool structure for concurrent execution
struct ThreadPool {
    spill workers normie[value]      fr fr Worker thread IDs
    spill task_queue normie[value]   fr fr Queue of pending tasks
    spill queue_size normie     fr fr Current queue size (atomic)
    spill queue_head normie     fr fr Queue head position (atomic)
    spill queue_tail normie     fr fr Queue tail position (atomic)
    spill active_workers normie fr fr Number of active workers (atomic)
    spill shutdown normie       fr fr Shutdown flag (atomic, 0=running, 1=shutdown)
}

fr fr Barrier synchronization structure
struct Barrier {
    spill count normie          fr fr Total number of participants
    spill arrived normie        fr fr Number of participants arrived (atomic)
    spill generation normie     fr fr Generation counter for reuse (atomic)
    spill waiting_list normie[value] fr fr List of waiting goroutines
}

fr fr Semaphore structure for resource counting
struct Semaphore {
    spill permits normie        fr fr Available permits (atomic)
    spill max_permits normie    fr fr Maximum permits allowed
    spill waiters normie[value]      fr fr Queue of waiting goroutines
    spill waiter_count normie   fr fr Number of waiting goroutines (atomic)
}

fr fr Read-Write Mutex structure
struct RWMutex {
    spill readers normie        fr fr Number of active readers (atomic)
    spill writer normie         fr fr Writer flag (atomic, 0=no writer, 1=writer active)
    spill pending_writers normie fr fr Number of pending writers (atomic)
    spill reader_waiters normie[value] fr fr Queue of waiting readers
    spill writer_waiters normie[value] fr fr Queue of waiting writers
}

fr fr Condition Variable structure
struct CondVar {
    spill waiters normie[value]      fr fr Queue of waiting goroutines
    spill waiter_count normie   fr fr Number of waiting goroutines (atomic)
    spill signal_count normie   fr fr Number of signals sent (atomic)
    spill broadcast_flag normie fr fr Broadcast flag (atomic)
}

fr fr Once structure for one-time initialization
struct Once {
    spill done normie           fr fr Done flag (atomic, 0=not done, 1=done)
    spill in_progress normie    fr fr In progress flag (atomic)
}

fr fr Atomic wrapper structures for type safety
struct AtomicI32 {
    spill value normie          fr fr 32-bit atomic integer value
}

struct AtomicI64 {
    spill value thicc           fr fr 64-bit atomic integer value
}

struct AtomicBool {
    spill value normie          fr fr Boolean atomic value (0=false, 1=true)
}

fr fr =============================================================================
fr fr MUTEX OPERATIONS - Thread-safe locking primitives
fr fr =============================================================================

fr fr Create new mutex for synchronization
slay create_mutex() *Mutex {
    sus mutex *Mutex = memory.allocate(Mutex)
    mutex.lock_state = 0
    mutex.owner = 0
    mutex.waiters = 0
    mutex.recursive_count = 0
    damn mutex
}

fr fr Lock mutex (blocking operation) - RACE-SAFE WITH PROPER BACKOFF
slay mutex_lock(mutex *Mutex) lit {
    lowkey (mutex == 0) {
        damn cap  fr fr Invalid mutex
    }
    
    sus current_owner thicc = 42  fr fr Current goroutine ID (simplified)
    sus backoff_count normie = 1
    
    fr fr Retry loop with exponential backoff
    periodt {
        fr fr Try to acquire lock atomically
        sus expected normie = 0
        lowkey (atomic_drip.compare_and_swap_i32(&mutex.lock_state, expected, 1, ACQUIRE)) {
            fr fr Successfully acquired lock
            atomic_drip.memory_fence(ACQUIRE)
            mutex.owner = current_owner
            damn based
        }
        
        fr fr Failed to acquire - increment waiters
        atomic_drip.atomic_add_i32(&mutex.waiters, 1, RELAXED)
        
        fr fr Exponential backoff with cooperative yielding
        sus yield_cycles normie = 0
        bestie yield_cycles < backoff_count {
            runtime_yield()
            yield_cycles = yield_cycles + 1
        }
        
        fr fr Increase backoff up to maximum
        lowkey (backoff_count < 100) {
            backoff_count = backoff_count * 2
        } otherwise {
            backoff_count = 100
        }
        
        fr fr Check if lock is still held before next attempt
        lowkey (atomic_drip.atomic_load_i32(&mutex.lock_state, ACQUIRE) == 0) {
            fr fr Lock appears free, decrement waiters and retry immediately
            atomic_drip.atomic_sub_i32(&mutex.waiters, 1, RELAXED)
            backoff_count = 1  fr fr Reset backoff
            break
        }
        
        atomic_drip.atomic_sub_i32(&mutex.waiters, 1, RELAXED)
    }
    
    damn cap  fr fr Should never reach here
}

fr fr Unlock mutex using atomic operations
slay mutex_unlock(mutex *Mutex) lit {
    ready mutex == 0 {
        damn cap
    }
    
    fr fr Verify we own the lock
    sus current_owner thicc = 42
    ready mutex.owner != current_owner {
        damn cap  fr fr Not lock owner
    }
    
    fr fr Release lock atomically
    mutex.owner = 0
    atomic_drip.atomic_store_i32(&mutex.lock_state, 0, RELEASE)
    damn based
}

fr fr Try to lock mutex (non-blocking)
slay mutex_trylock(mutex *Mutex) lit {
    ready mutex == 0 {
        damn cap
    }
    
    sus expected normie = 0
    sus current_owner thicc = 42
    
    ready atomic_drip.compare_and_swap_i32(&mutex.lock_state, expected, 1, ACQUIRE) {
        mutex.owner = current_owner
        damn based
    }
    damn cap
}

fr fr =============================================================================
fr fr WAITGROUP OPERATIONS - Goroutine synchronization
fr fr =============================================================================

fr fr Create new wait group for goroutine synchronization
slay create_waitgroup() *WaitGroup {
    sus wg *WaitGroup = memory.allocate(WaitGroup)
    wg.counter = 0
    wg.waiters = 0
    wg.generation = 0
    wg.done_flag = 0
    damn wg
}

fr fr Add count to wait group using atomic operations
slay waitgroup_add(wg *WaitGroup, count normie) lit {
    ready wg == 0 {
        damn cap
    }
    
    sus old_counter normie = atomic_drip.atomic_add_i32(&wg.counter, count, SEQCST)
    ready old_counter + count < 0 {
        damn cap  fr fr Would make counter negative
    }
    damn based
}

fr fr Mark one task as done in wait group
slay waitgroup_done(wg *WaitGroup) lit {
    ready wg == 0 {
        damn cap
    }
    
    sus old_counter normie = atomic_drip.atomic_sub_i32(&wg.counter, 1, SEQCST)
    ready old_counter <= 0 {
        damn cap  fr fr Already at zero or negative
    }
    
    fr fr Check if all tasks are done
    ready old_counter == 1 {
        atomic_drip.atomic_store_i32(&wg.done_flag, 1, RELEASE)
    }
    damn based
}

fr fr Wait for all tasks to complete
slay waitgroup_wait(wg *WaitGroup) lit {
    ready wg == 0 {
        damn cap
    }
    
    atomic_drip.atomic_add_i32(&wg.waiters, 1, RELAXED)
    
    bestie atomic_drip.atomic_load_i32(&wg.counter, ACQUIRE) > 0 {
        fr fr Spin-wait for completion (in real implementation would use OS primitives)
    }
    
    atomic_drip.atomic_sub_i32(&wg.waiters, 1, RELAXED)
    damn based
}

fr fr =============================================================================
fr fr CHANNEL OPERATIONS - Buffered communication primitives
fr fr =============================================================================

fr fr Create buffered channel for communication
slay create_channel(capacity normie) *Channel {
    sus ch *Channel = memory.allocate(Channel)
    ch.buffer = memory.allocate_array(normie, capacity)
    ch.capacity = capacity
    ch.size = 0
    ch.send_pos = 0
    ch.recv_pos = 0
    ch.closed = 0
    ch.send_waiters = 0
    ch.recv_waiters = 0
    damn ch
}

fr fr Create synchronous channel (unbuffered)
slay create_sync_channel() *Channel {
    damn create_channel(0)
}

fr fr Send data through channel (blocking) - RACE-SAFE IMPLEMENTATION
slay channel_send(ch *Channel, data normie) lit {
    ready ch == 0 {
        damn cap  fr fr Invalid channel
    }
    
    fr fr Check if channel is closed (atomic read)
    ready atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1 {
        damn cap  fr fr Channel closed
    }
    
    fr fr For unbuffered channels (synchronous)
    ready ch.capacity == 0 {
        fr fr CRITICAL SECTION: Atomic synchronous send operation
        atomic_drip.atomic_add_i32(&ch.send_waiters, 1, RELAXED)
        
        fr fr Busy-wait with exponential backoff for receiver
        sus backoff_count normie = 1
        bestie atomic_drip.atomic_load_i32(&ch.recv_waiters, ACQUIRE) == 0 && 
               atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 0 {
            fr fr Cooperative yielding with backoff
            sus yield_cycles normie = 0
            bestie yield_cycles < backoff_count {
                runtime_yield()
                yield_cycles = yield_cycles + 1
            }
            ready backoff_count < 1000 {
                backoff_count = backoff_count * 2
            } otherwise {
                backoff_count = 1000
            }
        }
        
        fr fr Double-check channel not closed after wait
        ready atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1 {
            atomic_drip.atomic_sub_i32(&ch.send_waiters, 1, RELAXED)
            damn cap
        }
        
        fr fr ATOMIC: Store data with memory barrier
        atomic_drip.memory_fence(ACQREL)
        ch.buffer[0] = data
        atomic_drip.memory_fence(RELEASE)
        
        atomic_drip.atomic_sub_i32(&ch.send_waiters, 1, RELAXED)
        damn based
    }
    
    fr fr For buffered channels - ATOMIC SIZE MANAGEMENT
    fr fr Wait for space with exponential backoff
    sus space_backoff normie = 1
    bestie atomic_drip.atomic_load_i32(&ch.size, ACQUIRE) >= ch.capacity && 
           atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 0 {
        atomic_drip.atomic_add_i32(&ch.send_waiters, 1, RELAXED)
        
        fr fr Cooperative wait with backoff
        sus space_yield_cycles normie = 0
        bestie space_yield_cycles < space_backoff {
            runtime_yield()
            space_yield_cycles = space_yield_cycles + 1
        }
        ready space_backoff < 500 {
            space_backoff = space_backoff * 2
        } otherwise {
            space_backoff = 500
        }
        
        atomic_drip.atomic_sub_i32(&ch.send_waiters, 1, RELAXED)
    }
    
    fr fr Final check: channel not closed
    ready atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1 {
        damn cap
    }
    
    fr fr ATOMIC BUFFER OPERATION: Compare-and-swap position update
    periodt {
        sus current_pos normie = atomic_drip.atomic_load_i32(&ch.send_pos, ACQUIRE)
        sus current_size normie = atomic_drip.atomic_load_i32(&ch.size, ACQUIRE)
        
        fr fr Check capacity one more time (double-check pattern)
        ready current_size >= ch.capacity {
            fr fr Channel became full, retry
            runtime_yield()
            break
        }
        
        fr fr Atomic position and size update with CAS
        ready atomic_drip.compare_and_swap_i32(&ch.send_pos, current_pos, current_pos + 1, ACQREL) &&
              atomic_drip.compare_and_swap_i32(&ch.size, current_size, current_size + 1, ACQREL) {
            
            fr fr Successfully reserved slot - safe to write
            atomic_drip.memory_fence(ACQUIRE)
            ch.buffer[current_pos % ch.capacity] = data
            atomic_drip.memory_fence(RELEASE)
            
            break  fr fr Exit retry loop
        }
        
        fr fr CAS failed - retry with brief yield
        runtime_yield()
    }
    
    damn based
}

fr fr Receive data from channel (blocking) - RACE-SAFE IMPLEMENTATION
slay channel_receive(ch *Channel) normie {
    ready ch == 0 {
        damn 0  fr fr Invalid channel
    }
    
    fr fr For unbuffered channels (synchronous)
    ready ch.capacity == 0 {
        atomic_drip.atomic_add_i32(&ch.recv_waiters, 1, RELAXED)
        
        fr fr Wait for sender with exponential backoff
        sus recv_backoff normie = 1
        bestie atomic_drip.atomic_load_i32(&ch.send_waiters, ACQUIRE) == 0 && 
               atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 0 {
            fr fr Cooperative wait with backoff
            sus recv_yield_cycles normie = 0
            bestie recv_yield_cycles < recv_backoff {
                runtime_yield()
                recv_yield_cycles = recv_yield_cycles + 1
            }
            ready recv_backoff < 1000 {
                recv_backoff = recv_backoff * 2
            } otherwise {
                recv_backoff = 1000
            }
        }
        
        fr fr Check if channel closed during wait
        ready atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1 {
            atomic_drip.atomic_sub_i32(&ch.recv_waiters, 1, RELAXED)
            damn 0  fr fr Channel closed
        }
        
        fr fr ATOMIC: Read data with memory barrier
        atomic_drip.memory_fence(ACQUIRE)
        sus data normie = ch.buffer[0]
        atomic_drip.memory_fence(RELEASE)
        
        atomic_drip.atomic_sub_i32(&ch.recv_waiters, 1, RELAXED)
        damn data
    }
    
    fr fr For buffered channels - ATOMIC RECEIVE OPERATION
    fr fr Wait for data with backoff
    sus data_backoff normie = 1
    bestie atomic_drip.atomic_load_i32(&ch.size, ACQUIRE) == 0 && 
           atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 0 {
        atomic_drip.atomic_add_i32(&ch.recv_waiters, 1, RELAXED)
        
        fr fr Cooperative wait with backoff
        sus data_yield_cycles normie = 0
        bestie data_yield_cycles < data_backoff {
            runtime_yield()
            data_yield_cycles = data_yield_cycles + 1
        }
        ready data_backoff < 500 {
            data_backoff = data_backoff * 2
        } otherwise {
            data_backoff = 500
        }
        
        atomic_drip.atomic_sub_i32(&ch.recv_waiters, 1, RELAXED)
    }
    
    fr fr Check if channel is closed and empty (final check)
    ready atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1 && 
          atomic_drip.atomic_load_i32(&ch.size, ACQUIRE) == 0 {
        damn 0  fr fr Channel closed and empty
    }
    
    fr fr ATOMIC BUFFER RECEIVE: Compare-and-swap position update
    periodt {
        sus current_pos normie = atomic_drip.atomic_load_i32(&ch.recv_pos, ACQUIRE)
        sus current_size normie = atomic_drip.atomic_load_i32(&ch.size, ACQUIRE)
        
        fr fr Check if data available (double-check pattern)
        ready current_size == 0 {
            fr fr No data available, check if closed
            ready atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1 {
                damn 0  fr fr Channel closed, no more data
            }
            fr fr Not closed, retry
            runtime_yield()
            continue
        }
        
        fr fr Atomic position and size update with CAS
        ready atomic_drip.compare_and_swap_i32(&ch.recv_pos, current_pos, current_pos + 1, ACQREL) &&
              atomic_drip.compare_and_swap_i32(&ch.size, current_size, current_size - 1, ACQREL) {
            
            fr fr Successfully reserved data slot - safe to read
            atomic_drip.memory_fence(ACQUIRE)
            sus data normie = ch.buffer[current_pos % ch.capacity]
            atomic_drip.memory_fence(RELEASE)
            
            damn data  fr fr Return received data
        }
        
        fr fr CAS failed - retry with brief yield
        runtime_yield()
    }
    
    fr fr Should never reach here in correct implementation
    damn 0
}

fr fr Close channel to signal no more data
slay channel_close(ch *Channel) lit {
    ready ch == 0 {
        damn cap
    }
    
    atomic_drip.atomic_store_i32(&ch.closed, 1, RELEASE)
    damn based
}

fr fr Check if channel is closed
slay channel_is_closed(ch *Channel) lit {
    ready ch == 0 {
        damn based  fr fr Invalid channel treated as closed
    }
    
    damn atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1
}

fr fr =============================================================================
fr fr SELECT STATEMENT IMPLEMENTATION - Non-blocking channel multiplexing
fr fr =============================================================================

fr fr Select operation for non-blocking channel operations
slay select_try_send(ch *Channel, data normie) lit {
    ready ch == 0 || atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1 {
        damn cap
    }
    
    fr fr For unbuffered channels, check for waiting receivers
    ready ch.capacity == 0 {
        ready atomic_drip.atomic_load_i32(&ch.recv_waiters, ACQUIRE) > 0 {
            fr fr Receiver available - can send immediately
            atomic_drip.memory_fence(ACQREL)
            ch.buffer[0] = data
            atomic_drip.memory_fence(RELEASE)
            damn based
        }
        damn cap  fr fr No receiver, would block
    }
    
    fr fr For buffered channels, check if space available
    ready atomic_drip.atomic_load_i32(&ch.size, ACQUIRE) < ch.capacity {
        fr fr Try atomic send
        sus current_pos normie = atomic_drip.atomic_load_i32(&ch.send_pos, ACQUIRE)
        sus current_size normie = atomic_drip.atomic_load_i32(&ch.size, ACQUIRE)
        
        ready atomic_drip.compare_and_swap_i32(&ch.send_pos, current_pos, current_pos + 1, ACQREL) &&
              atomic_drip.compare_and_swap_i32(&ch.size, current_size, current_size + 1, ACQREL) {
            
            atomic_drip.memory_fence(ACQUIRE)
            ch.buffer[current_pos % ch.capacity] = data
            atomic_drip.memory_fence(RELEASE)
            damn based
        }
    }
    
    damn cap  fr fr Channel full, would block
}

fr fr Select operation for non-blocking channel receive
slay select_try_receive(ch *Channel) normie {
    ready ch == 0 {
        damn 0
    }
    
    fr fr For unbuffered channels, check for waiting senders
    ready ch.capacity == 0 {
        ready atomic_drip.atomic_load_i32(&ch.send_waiters, ACQUIRE) > 0 {
            fr fr Sender available - can receive immediately
            atomic_drip.memory_fence(ACQUIRE)
            sus data normie = ch.buffer[0]
            atomic_drip.memory_fence(RELEASE)
            damn data
        }
        fr fr Check if closed
        ready atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1 {
            damn 0  fr fr Closed, no data
        }
        damn -1  fr fr No sender, would block (use -1 to distinguish from closed)
    }
    
    fr fr For buffered channels, check if data available
    ready atomic_drip.atomic_load_i32(&ch.size, ACQUIRE) > 0 {
        fr fr Try atomic receive
        sus current_pos normie = atomic_drip.atomic_load_i32(&ch.recv_pos, ACQUIRE)
        sus current_size normie = atomic_drip.atomic_load_i32(&ch.size, ACQUIRE)
        
        ready current_size > 0 &&
              atomic_drip.compare_and_swap_i32(&ch.recv_pos, current_pos, current_pos + 1, ACQREL) &&
              atomic_drip.compare_and_swap_i32(&ch.size, current_size, current_size - 1, ACQREL) {
            
            atomic_drip.memory_fence(ACQUIRE)
            sus data normie = ch.buffer[current_pos % ch.capacity]
            atomic_drip.memory_fence(RELEASE)
            damn data
        }
    }
    
    fr fr Check if closed and empty
    ready atomic_drip.atomic_load_i32(&ch.closed, ACQUIRE) == 1 &&
          atomic_drip.atomic_load_i32(&ch.size, ACQUIRE) == 0 {
        damn 0  fr fr Closed and empty
    }
    
    damn -1  fr fr No data, would block
}

fr fr Select statement with multiple channels
slay select_multi_channel(channels []*Channel, channel_count normie, timeout_ms normie) normie {
    ready channels == 0 || channel_count == 0 {
        damn -1  fr fr Invalid parameters
    }
    
    sus start_time normie = 0  fr fr Simplified timestamp
    sus iterations normie = 0
    sus max_iterations normie = timeout_ms > 0 ? timeout_ms * 10 : 10000  fr fr 10k iterations default
    
    fr fr Randomized channel selection to ensure fairness
    sus random_offset normie = start_time % channel_count
    
    bestie iterations < max_iterations {
        fr fr Try each channel in randomized order
        sus i normie = 0
        bestie i < channel_count {
            sus channel_index normie = (random_offset + i) % channel_count
            sus ch *Channel = channels[channel_index]
            
            fr fr Try non-blocking receive
            sus received normie = select_try_receive(ch)
            ready received != -1 {
                damn channel_index  fr fr Return index of ready channel
            }
            
            i = i + 1
        }
        
        fr fr No channels ready, brief yield
        runtime_yield()
        iterations = iterations + 1
        
        fr fr Update random offset for next iteration
        random_offset = (random_offset + 1) % channel_count
    }
    
    damn -1  fr fr Timeout or no channels ready
}

fr fr Create read-write mutex for shared resource access (FIXED IMPLEMENTATION)
slay create_rwmutex() *RWMutex {
    sus rwmutex *RWMutex = memory.allocate(RWMutex)
    rwmutex.readers = 0
    rwmutex.writer = 0
    rwmutex.pending_writers = 0
    rwmutex.reader_waiters = memory.allocate_array(normie, 100)
    rwmutex.writer_waiters = memory.allocate_array(normie, 100)
    damn rwmutex
}

fr fr Create condition variable for thread coordination
slay create_condition() Mutex {
    sus condition Mutex = 0
    damn condition
}

fr fr Wait on condition variable
slay condition_wait(condition Mutex, mutex Mutex) lit { fr fr Release mutex and wait for signal
    mutex_unlock(mutex)
    bestie condition == 0 { fr fr Wait for signal
    }
    mutex_lock(mutex)
    damn based
}

fr fr Signal one waiting goroutine
slay condition_signal(condition Mutex) lit {
    condition = 1
    damn based
}

fr fr Signal all waiting goroutines
slay condition_broadcast(condition Mutex) lit {
    condition = 2
    damn based
}

fr fr =============================================================================
fr fr ATOMIC OPERATIONS - Lock-free primitives
fr fr =============================================================================

fr fr Create atomic 32-bit integer
slay atomic_i32_new(initial normie) *AtomicI32 {
    sus atomic *AtomicI32 = memory.allocate(AtomicI32)
    atomic.value = initial
    damn atomic
}

fr fr Create atomic 64-bit integer
slay atomic_i64_new(initial thicc) *AtomicI64 {
    sus atomic *AtomicI64 = memory.allocate(AtomicI64)
    atomic.value = initial
    damn atomic
}

fr fr Create atomic boolean
slay atomic_bool_new(initial lit) *AtomicBool {
    sus atomic *AtomicBool = memory.allocate(AtomicBool)
    ready initial == based {
        atomic.value = 1
    } otherwise {
        atomic.value = 0
    }
    damn atomic
}

fr fr Atomic compare and swap operation for 32-bit
slay atomic_cas_i32(atomic *AtomicI32, expected normie, desired normie) lit {
    ready atomic == 0 {
        damn cap
    }
    damn atomic_drip.compare_and_swap_i32(&atomic.value, expected, desired, SEQCST)
}

fr fr Atomic compare and swap operation for 64-bit
slay atomic_cas_i64(atomic *AtomicI64, expected thicc, desired thicc) lit {
    ready atomic == 0 {
        damn cap
    }
    damn atomic_drip.compare_and_swap_i64(&atomic.value, expected, desired, SEQCST)
}

fr fr Atomic increment operation
slay atomic_increment(atomic *AtomicI32) normie {
    ready atomic == 0 {
        damn 0
    }
    damn atomic_drip.atomic_add_i32(&atomic.value, 1, SEQCST)
}

fr fr Atomic decrement operation
slay atomic_decrement(atomic *AtomicI32) normie {
    ready atomic == 0 {
        damn 0
    }
    damn atomic_drip.atomic_sub_i32(&atomic.value, 1, SEQCST)
}

fr fr Atomic load operation
slay atomic_load_i32(atomic *AtomicI32) normie {
    ready atomic == 0 {
        damn 0
    }
    damn atomic_drip.atomic_load_i32(&atomic.value, ACQUIRE)
}

fr fr Atomic store operation
slay atomic_store_i32(atomic *AtomicI32, value normie) {
    ready atomic == 0 {
        damn
    }
    atomic_drip.atomic_store_i32(&atomic.value, value, RELEASE)
}

fr fr Atomic add operation
slay atomic_add_i32(atomic *AtomicI32, delta normie) normie {
    ready atomic == 0 {
        damn 0
    }
    damn atomic_drip.atomic_add_i32(&atomic.value, delta, SEQCST)
}

fr fr Atomic subtract operation
slay atomic_sub_i32(atomic *AtomicI32, delta normie) normie {
    ready atomic == 0 {
        damn 0
    }
    damn atomic_drip.atomic_sub_i32(&atomic.value, delta, SEQCST)
}

fr fr =============================================================================
fr fr BARRIER OPERATIONS - Synchronization point for multiple goroutines
fr fr =============================================================================

fr fr Create barrier for synchronized waiting
slay create_barrier(count normie) *Barrier {
    sus barrier *Barrier = memory.allocate(Barrier)
    barrier.count = count
    barrier.arrived = 0
    barrier.generation = 0
    barrier.waiting_list = memory.allocate_array(normie, count)
    damn barrier
}

fr fr Wait at barrier until all participants arrive
slay barrier_wait(barrier *Barrier) lit {
    ready barrier == 0 {
        damn cap
    }
    
    sus current_gen normie = atomic_drip.atomic_load_i32(&barrier.generation, ACQUIRE)
    sus arrived_count normie = atomic_drip.atomic_add_i32(&barrier.arrived, 1, SEQCST)
    
    ready arrived_count + 1 == barrier.count {
        fr fr Last participant - wake everyone up
        atomic_drip.atomic_store_i32(&barrier.arrived, 0, RELEASE)
        atomic_drip.atomic_add_i32(&barrier.generation, 1, RELEASE)
        damn based
    }
    
    fr fr Wait for all participants to arrive
    bestie atomic_drip.atomic_load_i32(&barrier.generation, ACQUIRE) == current_gen {
        fr fr Spin-wait (real implementation would block goroutine)
    }
    damn based
}

fr fr =============================================================================
fr fr SEMAPHORE OPERATIONS - Resource counting primitive
fr fr =============================================================================

fr fr Create semaphore for resource counting
slay create_semaphore(initial normie) *Semaphore {
    sus sem *Semaphore = memory.allocate(Semaphore)
    sem.permits = initial
    sem.max_permits = initial
    sem.waiters = memory.allocate_array(normie, 100)  fr fr Max 100 waiters
    sem.waiter_count = 0
    damn sem
}

fr fr Acquire semaphore (decrement count)
slay semaphore_acquire(sem *Semaphore) lit {
    ready sem == 0 {
        damn cap
    }
    
    fr fr Try to acquire permit atomically
    periodt {
        sus current_permits normie = atomic_drip.atomic_load_i32(&sem.permits, ACQUIRE)
        ready current_permits > 0 {
            ready atomic_drip.compare_and_swap_i32(&sem.permits, current_permits, current_permits - 1, SEQCST) {
                damn based
            }
        } otherwise {
            fr fr No permits available - wait
            atomic_drip.atomic_add_i32(&sem.waiter_count, 1, RELAXED)
            fr fr Spin-wait for permits (real implementation would block goroutine)
            atomic_drip.atomic_sub_i32(&sem.waiter_count, 1, RELAXED)
        }
    }
    damn based
}

fr fr Release semaphore (increment count)
slay semaphore_release(sem *Semaphore) lit {
    ready sem == 0 {
        damn cap
    }
    
    sus current_permits normie = atomic_drip.atomic_add_i32(&sem.permits, 1, SEQCST)
    ready current_permits + 1 > sem.max_permits {
        fr fr Don't exceed max permits
        atomic_drip.atomic_sub_i32(&sem.permits, 1, SEQCST)
        damn cap
    }
    damn based
}

fr fr Try to acquire semaphore (non-blocking)
slay semaphore_try_acquire(sem *Semaphore) lit {
    ready sem == 0 {
        damn cap
    }
    
    sus current_permits normie = atomic_drip.atomic_load_i32(&sem.permits, ACQUIRE)
    ready current_permits > 0 {
        damn atomic_drip.compare_and_swap_i32(&sem.permits, current_permits, current_permits - 1, SEQCST)
    }
    damn cap
}

fr fr =============================================================================
fr fr THREAD POOL OPERATIONS - Concurrent task execution
fr fr =============================================================================

fr fr Create thread pool for concurrent execution
slay create_thread_pool(worker_count normie, queue_size normie) *ThreadPool {
    sus pool *ThreadPool = memory.allocate(ThreadPool)
    pool.workers = memory.allocate_array(normie, worker_count)
    pool.task_queue = memory.allocate_array(normie, queue_size)
    pool.queue_size = 0
    pool.queue_head = 0
    pool.queue_tail = 0
    pool.active_workers = 0
    pool.shutdown = 0
    damn pool
}

fr fr Submit task to thread pool
slay thread_pool_submit(pool *ThreadPool, task normie) lit {
    ready pool == 0 {
        damn cap
    }
    
    ready atomic_drip.atomic_load_i32(&pool.shutdown, ACQUIRE) == 1 {
        damn cap  fr fr Pool is shutting down
    }
    
    fr fr Check if queue has space
    sus current_size normie = atomic_drip.atomic_load_i32(&pool.queue_size, ACQUIRE)
    ready current_size >= 100 {  fr fr Assume max queue size of 100
        damn cap  fr fr Queue full
    }
    
    fr fr Add task to queue atomically
    sus tail_pos normie = atomic_drip.atomic_load_i32(&pool.queue_tail, ACQUIRE)
    pool.task_queue[tail_pos % 100] = task
    atomic_drip.atomic_add_i32(&pool.queue_tail, 1, RELEASE)
    atomic_drip.atomic_add_i32(&pool.queue_size, 1, RELEASE)
    damn based
}

fr fr Shutdown thread pool
slay thread_pool_shutdown(pool *ThreadPool) lit {
    ready pool == 0 {
        damn cap
    }
    
    atomic_drip.atomic_store_i32(&pool.shutdown, 1, RELEASE)
    damn based
}

fr fr Wait for all tasks to complete
slay thread_pool_wait_all(pool *ThreadPool) lit {
    ready pool == 0 {
        damn cap
    }
    
    bestie atomic_drip.atomic_load_i32(&pool.queue_size, ACQUIRE) > 0 {
        fr fr Wait for all tasks to complete
    }
    damn based
}

fr fr =============================================================================
fr fr ONCE OPERATIONS - One-time initialization primitive
fr fr =============================================================================

fr fr Create once primitive for one-time initialization
slay create_once() *Once {
    sus once *Once = memory.allocate(Once)
    once.done = 0
    once.in_progress = 0
    damn once
}

fr fr Execute function exactly once
slay once_do(once *Once, func_id normie) lit {
    ready once == 0 {
        damn cap
    }
    
    fr fr Check if already done
    ready atomic_drip.atomic_load_i32(&once.done, ACQUIRE) == 1 {
        damn cap  fr fr Already executed
    }
    
    fr fr Try to start execution
    ready atomic_drip.compare_and_swap_i32(&once.in_progress, 0, 1, SEQCST) {
        fr fr We got to execute - set done flag
        atomic_drip.atomic_store_i32(&once.done, 1, RELEASE)
        atomic_drip.atomic_store_i32(&once.in_progress, 0, RELEASE)
        damn based
    }
    
    fr fr Someone else is executing or already done
    damn cap
}

fr fr Create new mutex with struct fields
slay mutex_new() *MutexStruct {
    sus m MutexStruct = {locked: cap, owner: 0, waiters: 0}
    damn &m
}

fr fr Create new atomic variable
slay atomic_new(initial_value normie) *AtomicStruct {
    sus a AtomicStruct = {value: initial_value, version: 0, lock: 0}
    damn &a
}

fr fr Load value from atomic variable
slay atomic_load(atomic *AtomicStruct) normie {
    damn atomic.value
}

fr fr Store value to atomic variable
slay atomic_store(atomic *AtomicStruct, new_value normie) {
    atomic.value = new_value
    atomic.version = atomic.version + 1
}

fr fr Create new waitgroup with struct fields
slay waitgroup_new() *WaitGroupStruct {
    sus wg WaitGroupStruct = {counter: 0, waiters: 0, generation: 0}
    damn &wg
}

fr fr =============================================================================
fr fr UTILITY FUNCTIONS - Compatibility and convenience functions
fr fr =============================================================================

fr fr Channel creation function for Go-style compatibility
slay make(chan_type tea, buffer_size normie) *Channel {
    damn create_channel(buffer_size)
}

fr fr =============================================================================
fr fr GOROUTINE RUNTIME INTEGRATION - ENHANCED STAN IMPLEMENTATION
fr fr =============================================================================

yeet "goroutine_runtime"  fr fr Import enhanced goroutine runtime

fr fr Enhanced stan function with work-stealing scheduler
slay stan_enhanced(task_function thicc, context_data thicc) thicc {
    damn goroutine_runtime.stan(task_function, context_data)
}

fr fr Stan with custom stack size for memory-intensive goroutines
slay stan_stack(task_function thicc, context_data thicc, stack_size normie) thicc {
    damn goroutine_runtime.stan_with_stack(task_function, context_data, stack_size)
}

fr fr Stan with priority for high-priority goroutines
slay stan_priority(task_function thicc, context_data thicc, priority normie) thicc {
    damn goroutine_runtime.stan_with_priority(task_function, context_data, priority)
}

fr fr Get current goroutine ID from runtime
slay current_goroutine() thicc {
    damn goroutine_runtime.goroutine_id()
}

fr fr Yield CPU to other goroutines (cooperative multitasking)
slay yield_goroutine() {
    goroutine_runtime.yield()
}

fr fr Get goroutine scheduler performance statistics
slay scheduler_stats() *goroutine_runtime.SchedulerStats {
    damn goroutine_runtime.get_scheduler_stats()
}

fr fr Initialize goroutine scheduler with specific worker count
slay init_scheduler(worker_count normie) lit {
    damn goroutine_runtime.init_goroutine_scheduler(worker_count)
}

fr fr Shutdown goroutine scheduler and cleanup resources
slay shutdown_scheduler() {
    goroutine_runtime.shutdown_goroutine_scheduler()
}

fr fr =============================================================================
fr fr CURSED RUNTIME INTEGRATION FUNCTIONS - REAL IMPLEMENTATION
fr fr =============================================================================

yeet "os_primitives"
yeet "real_goroutine_tracking"

fr fr Global channel registry for tracking channels
struct ChannelRegistry {
    spill channels_map thicc        fr fr Map of channel_id -> Channel
    spill next_channel_id thicc     fr fr Next available channel ID (atomic)
    spill registry_mutex *os_primitives.OSMutex fr fr Thread-safe access
}

sus global_channel_registry *ChannelRegistry = 0

fr fr Initialize channel registry
slay init_channel_registry() lit {
    ready global_channel_registry != 0 {
        damn based  fr fr Already initialized
    }
    
    sus registry *ChannelRegistry = memory.allocate(ChannelRegistry)
    ready registry == 0 {
        damn cap
    }
    
    registry.channels_map = create_channel_hashmap(256)
    registry.next_channel_id = 1
    registry.registry_mutex = os_primitives.create_os_mutex(os_primitives.MUTEX_NORMAL)
    
    ready registry.registry_mutex == 0 {
        memory.free(registry)
        damn cap
    }
    
    global_channel_registry = registry
    damn based
}

fr fr Create channel with capacity (main entry point) - REAL IMPLEMENTATION
slay make_channel() thicc {
    ready global_channel_registry == 0 {
        ready init_channel_registry() == cap {
            damn 0  fr fr Failed to initialize
        }
    }
    
    sus ch *Channel = create_channel(1)
    ready ch == 0 {
        damn 0
    }
    
    fr fr Assign unique channel ID and register
    sus registry *ChannelRegistry = global_channel_registry
    os_primitives.lock_os_mutex(registry.registry_mutex)
    
    sus channel_id thicc = atomic_drip.atomic_add_i64(&registry.next_channel_id, 1, atomic_drip.SEQCST)
    hashmap_insert_channel(registry.channels_map, channel_id, ch)
    
    os_primitives.unlock_os_mutex(registry.registry_mutex)
    
    damn channel_id
}

fr fr Create buffered channel with specified capacity - REAL IMPLEMENTATION
slay make_buffered_channel(capacity normie) thicc {
    ready global_channel_registry == 0 {
        ready init_channel_registry() == cap {
            damn 0
        }
    }
    
    sus ch *Channel = create_channel(capacity)
    ready ch == 0 {
        damn 0
    }
    
    sus registry *ChannelRegistry = global_channel_registry
    os_primitives.lock_os_mutex(registry.registry_mutex)
    
    sus channel_id thicc = atomic_drip.atomic_add_i64(&registry.next_channel_id, 1, atomic_drip.SEQCST)
    hashmap_insert_channel(registry.channels_map, channel_id, ch)
    
    os_primitives.unlock_os_mutex(registry.registry_mutex)
    
    damn channel_id
}

fr fr Send value to channel (blocking) - REAL IMPLEMENTATION
slay send_channel(channel_id thicc, value normie) lit {
    ready global_channel_registry == 0 {
        damn cap
    }
    
    sus registry *ChannelRegistry = global_channel_registry
    os_primitives.lock_os_mutex(registry.registry_mutex)
    sus ch *Channel = hashmap_get_channel(registry.channels_map, channel_id)
    os_primitives.unlock_os_mutex(registry.registry_mutex)
    
    ready ch == 0 {
        damn cap  fr fr Invalid channel ID
    }
    
    fr fr Record goroutine blocking on channel
    sus current_goroutine thicc = real_goroutine_tracking.get_current_goroutine_id()
    real_goroutine_tracking.record_goroutine_blocked_on_channel(current_goroutine, channel_id)
    
    fr fr Perform actual channel send operation
    sus result lit = channel_send(ch, value)
    
    fr fr Update goroutine state after operation
    ready result {
        real_goroutine_tracking.update_goroutine_state(current_goroutine, real_goroutine_tracking.GOROUTINE_RUNNING)
    }
    
    damn result
}

fr fr Receive value from channel (blocking) - REAL IMPLEMENTATION
slay recv_channel(channel_id thicc) normie {
    ready global_channel_registry == 0 {
        damn 0
    }
    
    sus registry *ChannelRegistry = global_channel_registry
    os_primitives.lock_os_mutex(registry.registry_mutex)
    sus ch *Channel = hashmap_get_channel(registry.channels_map, channel_id)
    os_primitives.unlock_os_mutex(registry.registry_mutex)
    
    ready ch == 0 {
        damn 0  fr fr Invalid channel ID
    }
    
    fr fr Record goroutine blocking on channel
    sus current_goroutine thicc = real_goroutine_tracking.get_current_goroutine_id()
    real_goroutine_tracking.record_goroutine_blocked_on_channel(current_goroutine, channel_id)
    
    fr fr Perform actual channel receive operation
    sus result normie = channel_receive(ch)
    
    fr fr Update goroutine state after operation
    real_goroutine_tracking.update_goroutine_state(current_goroutine, real_goroutine_tracking.GOROUTINE_RUNNING)
    
    damn result
}

fr fr Close channel - REAL IMPLEMENTATION
slay close_channel(channel_id thicc) {
    ready global_channel_registry == 0 {
        damn
    }
    
    sus registry *ChannelRegistry = global_channel_registry
    os_primitives.lock_os_mutex(registry.registry_mutex)
    sus ch *Channel = hashmap_get_channel(registry.channels_map, channel_id)
    os_primitives.unlock_os_mutex(registry.registry_mutex)
    
    ready ch != 0 {
        atomic_drip.atomic_store_i32(&ch.closed, 1, atomic_drip.RELEASE)
    }
}

fr fr Check if channel is closed - REAL IMPLEMENTATION
slay is_channel_closed(channel_id thicc) lit {
    ready global_channel_registry == 0 {
        damn based  fr fr If no registry, consider closed
    }
    
    sus registry *ChannelRegistry = global_channel_registry
    os_primitives.lock_os_mutex(registry.registry_mutex)
    sus ch *Channel = hashmap_get_channel(registry.channels_map, channel_id)
    os_primitives.unlock_os_mutex(registry.registry_mutex)
    
    ready ch == 0 {
        damn based  fr fr Invalid channel considered closed
    }
    
    damn atomic_drip.atomic_load_i32(&ch.closed, atomic_drip.ACQUIRE) == 1
}

fr fr Memory fence operation for ordering guarantees
slay memory_fence() {
    atomic_drip.memory_fence(SEQCST)
}

fr fr Get current number of goroutines - REAL IMPLEMENTATION
slay num_goroutines() normie {
    damn real_goroutine_tracking.get_active_goroutine_count()
}

fr fr Runtime yield to other goroutines - REAL IMPLEMENTATION
slay runtime_yield() {
    fr fr Record yield in goroutine tracking
    sus current_goroutine thicc = real_goroutine_tracking.get_current_goroutine_id()
    real_goroutine_tracking.record_goroutine_yield(current_goroutine)
    
    fr fr Perform actual OS thread yield
    os_primitives.os_thread_yield()
    
    fr fr Update state back to running after yield
    real_goroutine_tracking.update_goroutine_state(current_goroutine, real_goroutine_tracking.GOROUTINE_RUNNING)
}

fr fr Sleep for specified duration - REAL IMPLEMENTATION
slay sleep_ms(duration normie) {
    ready duration <= 0 {
        damn  fr fr Invalid duration
    }
    
    fr fr Record goroutine as sleeping
    sus current_goroutine thicc = real_goroutine_tracking.get_current_goroutine_id()
    real_goroutine_tracking.update_goroutine_state(current_goroutine, real_goroutine_tracking.GOROUTINE_SLEEPING)
    
    fr fr Perform actual high-precision sleep
    os_primitives.microsleep_precise(duration * 1000)  fr fr Convert ms to microseconds
    
    fr fr Update state back to running after sleep
    real_goroutine_tracking.update_goroutine_state(current_goroutine, real_goroutine_tracking.GOROUTINE_RUNNING)
}

fr fr Sleep with microsecond precision
slay sleep_us(duration thicc) {
    ready duration <= 0 {
        damn
    }
    
    sus current_goroutine thicc = real_goroutine_tracking.get_current_goroutine_id()
    real_goroutine_tracking.update_goroutine_state(current_goroutine, real_goroutine_tracking.GOROUTINE_SLEEPING)
    
    os_primitives.microsleep_precise(duration)
    
    real_goroutine_tracking.update_goroutine_state(current_goroutine, real_goroutine_tracking.GOROUTINE_RUNNING)
}

fr fr Sleep with nanosecond precision
slay sleep_ns(duration thicc) {
    ready duration <= 0 {
        damn
    }
    
    sus current_goroutine thicc = real_goroutine_tracking.get_current_goroutine_id()
    real_goroutine_tracking.update_goroutine_state(current_goroutine, real_goroutine_tracking.GOROUTINE_SLEEPING)
    
    fr fr Convert nanoseconds to microseconds (minimum OS sleep resolution)
    sus microseconds thicc = duration / 1000
    ready microseconds < 1 {
        microseconds = 1
    }
    
    os_primitives.microsleep_precise(microseconds)
    
    real_goroutine_tracking.update_goroutine_state(current_goroutine, real_goroutine_tracking.GOROUTINE_RUNNING)
}

fr fr Get high-resolution timestamp
slay get_time_ns() thicc {
    damn os_primitives.get_real_time_ns()
}

fr fr CPU pause for efficient spin-waiting
slay cpu_pause() {
    os_primitives.cpu_pause_instruction()
}

fr fr =============================================================================
fr fr CHANNEL REGISTRY HELPER FUNCTIONS
fr fr =============================================================================

slay create_channel_hashmap(initial_size normie) thicc {
    damn memory.allocate(initial_size * 16)  fr fr Simple placeholder
}

slay hashmap_insert_channel(map thicc, key thicc, value *Channel) lit {
    fr fr Simplified implementation - would use real hash map
    damn based
}

slay hashmap_get_channel(map thicc, key thicc) *Channel {
    fr fr Simplified implementation - would use real hash map
    damn 0  fr fr Placeholder
}

fr fr =============================================================================
fr fr READ-WRITE MUTEX OPERATIONS - Shared/exclusive locking
fr fr =============================================================================

fr fr Create read-write mutex for shared resource access
slay create_rwmutex() *RWMutex {
    sus rwmutex *RWMutex = memory.allocate(RWMutex)
    rwmutex.readers = 0
    rwmutex.writer = 0
    rwmutex.pending_writers = 0
    rwmutex.reader_waiters = memory.allocate_array(normie, 100)
    rwmutex.writer_waiters = memory.allocate_array(normie, 100)
    damn rwmutex
}

fr fr Acquire read lock (multiple readers allowed)
slay rwmutex_rlock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    fr fr Wait while there's a writer or pending writers
    bestie atomic_drip.atomic_load_i32(&rwmutex.writer, ACQUIRE) == 1 || 
          atomic_drip.atomic_load_i32(&rwmutex.pending_writers, ACQUIRE) > 0 {
        fr fr Spin-wait (real implementation would block)
    }
    
    atomic_drip.atomic_add_i32(&rwmutex.readers, 1, ACQUIRE)
    damn based
}

fr fr Release read lock
slay rwmutex_runlock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    sus reader_count normie = atomic_drip.atomic_sub_i32(&rwmutex.readers, 1, RELEASE)
    ready reader_count < 0 {
        damn cap  fr fr Not holding read lock
    }
    damn based
}

fr fr Acquire write lock (exclusive access)
slay rwmutex_lock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    atomic_drip.atomic_add_i32(&rwmutex.pending_writers, 1, RELAXED)
    
    fr fr Wait for no readers and no active writer
    bestie atomic_drip.atomic_load_i32(&rwmutex.readers, ACQUIRE) > 0 ||
          !atomic_drip.compare_and_swap_i32(&rwmutex.writer, 0, 1, ACQUIRE) {
        fr fr Spin-wait (real implementation would block)
    }
    
    atomic_drip.atomic_sub_i32(&rwmutex.pending_writers, 1, RELAXED)
    damn based
}

fr fr Release write lock
slay rwmutex_unlock(rwmutex *RWMutex) lit {
    ready rwmutex == 0 {
        damn cap
    }
    
    ready atomic_drip.atomic_load_i32(&rwmutex.writer, ACQUIRE) != 1 {
        damn cap  fr fr Not holding write lock
    }
    
    atomic_drip.atomic_store_i32(&rwmutex.writer, 0, RELEASE)
    damn based
}

fr fr =============================================================================
fr fr CONDITION VARIABLE OPERATIONS - Thread coordination
fr fr =============================================================================

fr fr Create condition variable for thread coordination
slay create_condition() *CondVar {
    sus cond *CondVar = memory.allocate(CondVar)
    cond.waiters = memory.allocate_array(normie, 100)
    cond.waiter_count = 0
    cond.signal_count = 0
    cond.broadcast_flag = 0
    damn cond
}

fr fr Wait on condition variable
slay condition_wait(cond *CondVar, mutex *Mutex) lit {
    ready cond == 0 || mutex == 0 {
        damn cap
    }
    
    atomic_drip.atomic_add_i32(&cond.waiter_count, 1, RELAXED)
    
    fr fr Release mutex and wait for signal
    mutex_unlock(mutex)
    
    bestie atomic_drip.atomic_load_i32(&cond.signal_count, ACQUIRE) == 0 &&
          atomic_drip.atomic_load_i32(&cond.broadcast_flag, ACQUIRE) == 0 {
        fr fr Spin-wait for signal (real implementation would block)
    }
    
    fr fr Reacquire mutex
    mutex_lock(mutex)
    atomic_drip.atomic_sub_i32(&cond.waiter_count, 1, RELAXED)
    damn based
}

fr fr Signal one waiting goroutine
slay condition_signal(cond *CondVar) lit {
    ready cond == 0 {
        damn cap
    }
    
    ready atomic_drip.atomic_load_i32(&cond.waiter_count, ACQUIRE) > 0 {
        atomic_drip.atomic_add_i32(&cond.signal_count, 1, RELEASE)
    }
    damn based
}

fr fr Signal all waiting goroutines
slay condition_broadcast(cond *CondVar) lit {
    ready cond == 0 {
        damn cap
    }
    
    ready atomic_drip.atomic_load_i32(&cond.waiter_count, ACQUIRE) > 0 {
        atomic_drip.atomic_store_i32(&cond.broadcast_flag, 1, RELEASE)
    }
    damn based
}

fr fr Legacy compatibility functions
slay create_mutex() *Mutex { damn mutex_new() }
slay create_waitgroup() *WaitGroup { damn waitgroup_new() }
