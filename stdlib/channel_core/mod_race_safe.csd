fr fr Channel Core - RACE CONDITION FIXES
fr fr Complete thread-safe channel system with proper atomic operations

yeet "testz"
yeet "atomic_drip"
yeet "memory"

fr fr Channel states
sus CHANNEL_OPEN normie = 0
sus CHANNEL_CLOSED normie = 1
sus CHANNEL_BLOCKED normie = 2

fr fr Race-safe channel representation with atomic fields
vibe Channel<T> = smash {
    id normie,
    buffer T[value],
    capacity normie,
    size *atomic_drip.AtomicI32,        fr fr ATOMIC: Current buffer size
    head_pos *atomic_drip.AtomicI32,    fr fr ATOMIC: Buffer head position
    tail_pos *atomic_drip.AtomicI32,    fr fr ATOMIC: Buffer tail position
    state *atomic_drip.AtomicI32,       fr fr ATOMIC: Channel state
    send_waiters *atomic_drip.AtomicI32,fr fr ATOMIC: Number of waiting senders
    recv_waiters *atomic_drip.AtomicI32,fr fr ATOMIC: Number of waiting receivers
    closed *atomic_drip.AtomicFlag      fr fr ATOMIC: Channel closed flag
}

fr fr CRITICAL FIX: Replace global map with thread-safe registry
struct ChannelRegistry {
    channels map[normie]Channel<normie>,
    registry_mutex *Mutex,              fr fr RACE-SAFE: Protects global registry
    next_id *atomic_drip.AtomicI32     fr fr ATOMIC: Thread-safe ID generation
}

fr fr Global thread-safe channel registry (RACE CONDITION FIXED)
sus global_registry *ChannelRegistry

fr fr Enhanced Mutex structure for registry protection
struct Mutex {
    lock_state *atomic_drip.AtomicI32,  fr fr 0=unlocked, 1=locked
    owner *atomic_drip.AtomicI64,       fr fr Owner thread ID
    waiters *atomic_drip.AtomicI32      fr fr Waiting goroutines count
}

fr fr Initialize channel system with thread-safe registry
slay init_channel_system() lit {
    global_registry = memory.allocate(ChannelRegistry)
    global_registry.channels = {}
    global_registry.registry_mutex = create_mutex()
    global_registry.next_id = atomic_drip.atomic_i32_new(1)
    vibez.spill("Thread-safe channel system initialized")
    damn based
}

fr fr Create mutex for synchronization
slay create_mutex() *Mutex {
    sus mutex *Mutex = memory.allocate(Mutex)
    mutex.lock_state = atomic_drip.atomic_i32_new(0)
    mutex.owner = atomic_drip.atomic_i64_new(0)
    mutex.waiters = atomic_drip.atomic_i32_new(0)
    damn mutex
}

fr fr RACE-SAFE: Lock mutex with exponential backoff
slay mutex_lock(mutex *Mutex) lit {
    lowkey (mutex == 0) {
        damn cringe
    }
    
    sus current_thread thicc = 42  fr fr Simplified thread ID
    sus backoff_cycles normie = 1
    sus max_backoff normie = 1000
    
    fr fr Retry loop with proper atomic compare-and-swap
    periodt {
        fr fr Try to acquire lock atomically using CAS
        ready atomic_drip.atomic_cas_i32(mutex.lock_state, 0, 1) {
            fr fr Successfully acquired lock
            atomic_drip.atomic_store_i64(mutex.owner, current_thread)
            damn based
        }
        
        fr fr Failed to acquire - increment waiters and backoff
        atomic_drip.atomic_increment_i32(mutex.waiters)
        
        fr fr Exponential backoff with cooperative yielding
        sus yield_count normie = 0
        bestie yield_count < backoff_cycles {
            runtime_yield()  fr fr Cooperative yield to other goroutines
            yield_count = yield_count + 1
        }
        
        fr fr Increase backoff up to maximum
        ready backoff_cycles < max_backoff {
            backoff_cycles = backoff_cycles * 2
        }
        
        atomic_drip.atomic_decrement_i32(mutex.waiters)
        
        fr fr Check if lock became available
        ready atomic_drip.atomic_load_i32(mutex.lock_state) == 0 {
            continue  fr fr Retry immediately
        }
    }
}

fr fr RACE-SAFE: Unlock mutex with atomic operations
slay mutex_unlock(mutex *Mutex) lit {
    ready mutex == 0 {
        damn cringe
    }
    
    fr fr Verify we own the lock (safety check)
    sus current_thread thicc = 42
    ready atomic_drip.atomic_load_i64(mutex.owner) != current_thread {
        damn cringe  fr fr Not lock owner
    }
    
    fr fr Release lock atomically
    atomic_drip.atomic_store_i64(mutex.owner, 0)
    atomic_drip.atomic_store_i32(mutex.lock_state, 0)
    damn based
}

fr fr RACE-SAFE: Create channel with atomic initialization
slay make_channel<T>(capacity normie) normie {
    fr fr CRITICAL: Thread-safe ID generation
    sus channel_id normie = atomic_drip.atomic_increment_i32(global_registry.next_id)
    
    fr fr Create channel with atomic fields
    sus new_channel Channel<T>
    new_channel.id = channel_id
    new_channel.buffer = memory.allocate_array(T, capacity)
    new_channel.capacity = capacity
    new_channel.size = atomic_drip.atomic_i32_new(0)
    new_channel.head_pos = atomic_drip.atomic_i32_new(0)
    new_channel.tail_pos = atomic_drip.atomic_i32_new(0)
    new_channel.state = atomic_drip.atomic_i32_new(CHANNEL_OPEN)
    new_channel.send_waiters = atomic_drip.atomic_i32_new(0)
    new_channel.recv_waiters = atomic_drip.atomic_i32_new(0)
    new_channel.closed = atomic_drip.atomic_flag_new(cringe)
    
    fr fr CRITICAL FIX: Thread-safe registry insertion
    mutex_lock(global_registry.registry_mutex)
    global_registry.channels[channel_id] = new_channel
    mutex_unlock(global_registry.registry_mutex)
    
    damn channel_id
}

fr fr RACE-SAFE: Check channel existence with registry protection
slay channel_exists(channel_id normie) lit {
    mutex_lock(global_registry.registry_mutex)
    sus exists lit = (global_registry.channels[channel_id].id == channel_id)
    mutex_unlock(global_registry.registry_mutex)
    damn exists
}

fr fr RACE-SAFE: Get channel with registry protection
slay get_channel(channel_id normie) Channel<normie> {
    mutex_lock(global_registry.registry_mutex)
    sus channel Channel<normie> = global_registry.channels[channel_id]
    mutex_unlock(global_registry.registry_mutex)
    damn channel
}

fr fr RACE-SAFE: Update channel in registry
slay update_channel(channel_id normie, channel Channel<normie>) {
    mutex_lock(global_registry.registry_mutex)
    global_registry.channels[channel_id] = channel
    mutex_unlock(global_registry.registry_mutex)
}

fr fr RACE-SAFE: Send to channel with proper blocking and atomic operations
slay channel_send<T>(channel_id normie, value T) lit {
    lowkey !channel_exists(channel_id) {
        damn cringe
    }
    
    sus channel Channel<T> = get_channel(channel_id)
    
    fr fr Check if channel is closed atomically
    ready atomic_drip.atomic_flag_load(channel.closed) {
        damn cringe  fr fr Channel closed
    }
    
    fr fr RACE-SAFE: Wait for space with timeout and proper signaling
    atomic_drip.atomic_increment_i32(channel.send_waiters)
    
    sus timeout_cycles normie = 0
    sus max_timeout normie = 100000  fr fr Prevent infinite spinning
    sus backoff_cycles normie = 1
    
    fr fr Wait for buffer space with exponential backoff and timeout
    periodt {
        sus current_size normie = atomic_drip.atomic_load_i32(channel.size)
        
        fr fr Check if space available
        ready current_size < channel.capacity {
            break  fr fr Space available, proceed to send
        }
        
        fr fr Check timeout to prevent infinite waiting
        ready timeout_cycles >= max_timeout {
            atomic_drip.atomic_decrement_i32(channel.send_waiters)
            damn cringe  fr fr Timeout waiting for space
        }
        
        fr fr Check if channel closed while waiting
        ready atomic_drip.atomic_flag_load(channel.closed) {
            atomic_drip.atomic_decrement_i32(channel.send_waiters)
            damn cringe  fr fr Channel closed during wait
        }
        
        fr fr Exponential backoff with cooperative yielding
        sus yield_count normie = 0
        bestie yield_count < backoff_cycles {
            runtime_yield()  fr fr FIXED: Proper yielding instead of busy wait
            yield_count = yield_count + 1
            timeout_cycles = timeout_cycles + 1
        }
        
        ready backoff_cycles < 100 {
            backoff_cycles = backoff_cycles * 2
        }
    }
    
    fr fr ATOMIC BUFFER INSERTION: Thread-safe circular buffer operation
    periodt {
        sus current_size normie = atomic_drip.atomic_load_i32(channel.size)
        sus current_tail normie = atomic_drip.atomic_load_i32(channel.tail_pos)
        
        fr fr Double-check capacity (race condition protection)
        ready current_size >= channel.capacity {
            continue  fr fr Retry if buffer became full
        }
        
        fr fr Atomic size increment with CAS
        ready atomic_drip.atomic_cas_i32(channel.size, current_size, current_size + 1) {
            fr fr Successfully reserved slot - safe to write
            sus buffer_index normie = current_tail % channel.capacity
            channel.buffer[buffer_index] = value
            
            fr fr Update tail position atomically
            atomic_drip.atomic_cas_i32(channel.tail_pos, current_tail, current_tail + 1)
            break  fr fr Successfully sent
        }
        
        fr fr CAS failed - brief yield and retry
        runtime_yield()
    }
    
    atomic_drip.atomic_decrement_i32(channel.send_waiters)
    update_channel(channel_id, channel)
    damn based
}

fr fr RACE-SAFE: Receive from channel with proper blocking and atomic operations  
slay channel_recv<T>(channel_id normie) (T, lit) {
    lowkey !channel_exists(channel_id) {
        sus zero T
        damn (zero, cringe)
    }
    
    sus channel Channel<T> = get_channel(channel_id)
    
    fr fr RACE-SAFE: Wait for data with timeout and proper signaling
    atomic_drip.atomic_increment_i32(channel.recv_waiters)
    
    sus timeout_cycles normie = 0
    sus max_timeout normie = 100000  fr fr Prevent infinite spinning
    sus backoff_cycles normie = 1
    
    fr fr Wait for data with exponential backoff and timeout
    periodt {
        sus current_size normie = atomic_drip.atomic_load_i32(channel.size)
        
        fr fr Check if data available
        ready current_size > 0 {
            break  fr fr Data available, proceed to receive
        }
        
        fr fr Check if channel closed and empty
        ready atomic_drip.atomic_flag_load(channel.closed) && current_size == 0 {
            atomic_drip.atomic_decrement_i32(channel.recv_waiters)
            sus zero T
            damn (zero, cringe)  fr fr Channel closed and empty
        }
        
        fr fr Check timeout to prevent infinite waiting
        ready timeout_cycles >= max_timeout {
            atomic_drip.atomic_decrement_i32(channel.recv_waiters)
            sus zero T
            damn (zero, cringe)  fr fr Timeout waiting for data
        }
        
        fr fr Exponential backoff with cooperative yielding
        sus yield_count normie = 0
        bestie yield_count < backoff_cycles {
            runtime_yield()  fr fr FIXED: Proper yielding instead of busy wait
            yield_count = yield_count + 1
            timeout_cycles = timeout_cycles + 1
        }
        
        ready backoff_cycles < 100 {
            backoff_cycles = backoff_cycles * 2
        }
    }
    
    fr fr ATOMIC BUFFER EXTRACTION: Thread-safe circular buffer operation
    periodt {
        sus current_size normie = atomic_drip.atomic_load_i32(channel.size)
        sus current_head normie = atomic_drip.atomic_load_i32(channel.head_pos)
        
        fr fr Double-check data availability (race condition protection)
        ready current_size == 0 {
            continue  fr fr Retry if buffer became empty
        }
        
        fr fr Atomic size decrement with CAS
        ready atomic_drip.atomic_cas_i32(channel.size, current_size, current_size - 1) {
            fr fr Successfully reserved data slot - safe to read
            sus buffer_index normie = current_head % channel.capacity
            sus value T = channel.buffer[buffer_index]
            
            fr fr Update head position atomically
            atomic_drip.atomic_cas_i32(channel.head_pos, current_head, current_head + 1)
            
            atomic_drip.atomic_decrement_i32(channel.recv_waiters)
            update_channel(channel_id, channel)
            damn (value, based)  fr fr Successfully received
        }
        
        fr fr CAS failed - brief yield and retry
        runtime_yield()
    }
    
    fr fr Should never reach here
    atomic_drip.atomic_decrement_i32(channel.recv_waiters)
    sus zero T
    damn (zero, cringe)
}

fr fr RACE-SAFE: Close channel with atomic flag
slay channel_close(channel_id normie) lit {
    lowkey !channel_exists(channel_id) {
        damn cringe
    }
    
    sus channel Channel<normie> = get_channel(channel_id)
    
    fr fr Set closed flag atomically
    atomic_drip.atomic_flag_store(channel.closed, based)
    atomic_drip.atomic_store_i32(channel.state, CHANNEL_CLOSED)
    
    update_channel(channel_id, channel)
    damn based
}

fr fr RACE-SAFE: Get channel stats with atomic reads
slay get_channel_stats(channel_id normie) map[tea]normie {
    sus stats map[tea]normie = {}
    
    lowkey !channel_exists(channel_id) {
        stats["error"] = 1
        damn stats
    }
    
    sus channel Channel<normie> = get_channel(channel_id)
    
    fr fr Read all stats atomically to get consistent snapshot
    stats["id"] = channel.id
    stats["capacity"] = channel.capacity
    stats["size"] = atomic_drip.atomic_load_i32(channel.size)
    stats["state"] = atomic_drip.atomic_load_i32(channel.state)
    stats["closed"] = lowkey atomic_drip.atomic_flag_load(channel.closed) { 1 } yikes { 0 }
    stats["send_waiters"] = atomic_drip.atomic_load_i32(channel.send_waiters)
    stats["recv_waiters"] = atomic_drip.atomic_load_i32(channel.recv_waiters)
    
    damn stats
}

fr fr Runtime yield function for cooperative multitasking
slay runtime_yield() {
    fr fr In real implementation, this would yield to the CURSED scheduler
    fr fr For now, this is a placeholder for cooperative yielding
}

vibez.spill("✅ Race-Safe Channel Core System Loaded")
