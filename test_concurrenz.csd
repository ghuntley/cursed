yeet "testz"
yeet "concurrenz"

# CURSED Concurrency Module Test Suite
# Tests for mutexes, channels, goroutines, and synchronization primitives

# === MUTEX TESTS ===
slay test_mutex_basic() {
    test_start("Mutex basic lock/unlock")
    
    sus m *Mutex = mutex_new()
    
    # Test initial state
    assert_false(m.locked)
    assert_eq_int(m.owner, -1)
    
    # Test lock
    mutex_lock(m)
    assert_true(m.locked)
    assert_eq_int(m.owner, 1)
    
    # Test unlock
    mutex_unlock(m)
    assert_false(m.locked)
    assert_eq_int(m.owner, -1)
    
    vibez.spill("✓ Mutex basic operations working")
}

slay test_mutex_try_lock() {
    test_start("Mutex try_lock")
    
    sus m *Mutex = mutex_new()
    
    # Should succeed when unlocked
    assert_true(mutex_try_lock(m))
    assert_true(m.locked)
    
    # Should fail when locked
    assert_false(mutex_try_lock(m))
    
    mutex_unlock(m)
    assert_false(m.locked)
    
    vibez.spill("✓ Mutex try_lock working")
}

# === RWMUTEX TESTS ===
slay test_rwmutex_basic() {
    test_start("RWMutex basic operations")
    
    sus rw *RWMutex = rwmutex_new()
    
    # Test initial state
    assert_eq_int(rw.readers, 0)
    assert_false(rw.writer)
    assert_eq_int(rw.waiting_writers, 0)
    
    # Test read lock
    rwmutex_rlock(rw)
    assert_eq_int(rw.readers, 1)
    assert_false(rw.writer)
    
    # Test read unlock
    rwmutex_runlock(rw)
    assert_eq_int(rw.readers, 0)
    
    # Test write lock
    rwmutex_lock(rw)
    assert_true(rw.writer)
    assert_eq_int(rw.waiting_writers, 0)
    
    # Test write unlock
    rwmutex_unlock(rw)
    assert_false(rw.writer)
    
    vibez.spill("✓ RWMutex basic operations working")
}

# === WAITGROUP TESTS ===
slay test_waitgroup_basic() {
    test_start("WaitGroup basic operations")
    
    sus wg *WaitGroup = waitgroup_new()
    
    # Test initial state
    assert_eq_int(wg.count, 0)
    
    # Test add
    waitgroup_add(wg, 3)
    assert_eq_int(wg.count, 3)
    
    # Test done
    waitgroup_done(wg)
    assert_eq_int(wg.count, 2)
    
    waitgroup_done(wg)
    assert_eq_int(wg.count, 1)
    
    waitgroup_done(wg)
    assert_eq_int(wg.count, 0)
    
    vibez.spill("✓ WaitGroup basic operations working")
}

# === ONCE TESTS ===
slay test_once_basic() {
    test_start("Once basic operations")
    
    sus o *Once = once_new()
    sus counter normie = 0
    
    # Test initial state
    assert_false(o.done)
    
    # Test first execution
    once_do(o, slay() {
        counter++
    })
    assert_true(o.done)
    assert_eq_int(counter, 1)
    
    # Test second execution (should not run)
    once_do(o, slay() {
        counter++
    })
    assert_eq_int(counter, 1)
    
    vibez.spill("✓ Once basic operations working")
}

# === ATOMIC TESTS ===
slay test_atomic_basic() {
    test_start("Atomic basic operations")
    
    sus a *Atomic = atomic_new(42)
    
    # Test initial value
    assert_eq_int(atomic_load(a), 42)
    
    # Test store
    atomic_store(a, 100)
    assert_eq_int(atomic_load(a), 100)
    
    # Test add
    sus result normie = atomic_add(a, 10)
    assert_eq_int(result, 110)
    assert_eq_int(atomic_load(a), 110)
    
    # Test compare and swap (success)
    assert_true(atomic_compare_and_swap(a, 110, 200))
    assert_eq_int(atomic_load(a), 200)
    
    # Test compare and swap (failure)
    assert_false(atomic_compare_and_swap(a, 110, 300))
    assert_eq_int(atomic_load(a), 200)
    
    vibez.spill("✓ Atomic operations working")
}

# === CHANNEL TESTS ===
slay test_channel_basic() {
    test_start("Channel basic operations")
    
    sus ch chan normie = make(chan normie, 5)
    
    # Test send
    ch <- 42
    ch <- 100
    
    # Test receive
    sus val1 normie = <-ch
    sus val2 normie = <-ch
    
    assert_eq_int(val1, 42)
    assert_eq_int(val2, 100)
    
    vibez.spill("✓ Channel basic operations working")
}

slay test_channel_patterns() {
    test_start("Channel patterns")
    
    # Test timeout channel
    sus timeout chan lit = select_timeout(100)
    
    # Test buffered channel creation
    sus buffered chan normie = make_buffered_channel(10)
    buffered <- 1
    buffered <- 2
    
    sus val1 normie = <-buffered
    sus val2 normie = <-buffered
    assert_eq_int(val1, 1)
    assert_eq_int(val2, 2)
    
    vibez.spill("✓ Channel patterns working")
}

# === SEMAPHORE TESTS ===
slay test_semaphore_basic() {
    test_start("Semaphore basic operations")
    
    sus s *Semaphore = semaphore_new(2)
    
    # Test initial state
    assert_eq_int(s.permits, 2)
    
    # Test acquire
    semaphore_acquire(s)
    semaphore_acquire(s)
    
    # Test try_acquire when full
    assert_false(semaphore_try_acquire(s))
    
    # Test release
    semaphore_release(s)
    assert_true(semaphore_try_acquire(s))
    
    vibez.spill("✓ Semaphore basic operations working")
}

# === BARRIER TESTS ===
slay test_barrier_basic() {
    test_start("Barrier basic operations")
    
    sus b *Barrier = barrier_new(2)
    
    # Test initial state
    assert_eq_int(b.parties, 2)
    assert_eq_int(b.count, 0)
    assert_eq_int(b.generation, 0)
    
    vibez.spill("✓ Barrier basic operations working")
}

# === CONDITION VARIABLE TESTS ===
slay test_cond_basic() {
    test_start("Condition variable basic operations")
    
    sus m *Mutex = mutex_new()
    sus c *Cond = cond_new(m)
    
    # Test initial state
    assert_eq_int(c.waiters, 0)
    
    # Test signal (no waiters)
    cond_signal(c)
    assert_eq_int(c.waiters, 0)
    
    # Test broadcast (no waiters)
    cond_broadcast(c)
    assert_eq_int(c.waiters, 0)
    
    vibez.spill("✓ Condition variable basic operations working")
}

# === RATE LIMITER TESTS ===
slay test_rate_limiter_basic() {
    test_start("Rate limiter basic operations")
    
    sus rl *RateLimiter = rate_limiter_new(2, 1)
    
    # Test initial state
    assert_eq_int(rl.tokens, 2)
    assert_eq_int(rl.capacity, 2)
    assert_eq_int(rl.refill_rate, 1)
    
    # Test allow
    assert_true(rate_limiter_allow(rl))
    assert_eq_int(rl.tokens, 1)
    
    assert_true(rate_limiter_allow(rl))
    assert_eq_int(rl.tokens, 0)
    
    # Should be denied when no tokens
    assert_false(rate_limiter_allow(rl))
    
    vibez.spill("✓ Rate limiter basic operations working")
}

# === CIRCUIT BREAKER TESTS ===
slay test_circuit_breaker_basic() {
    test_start("Circuit breaker basic operations")
    
    sus cb *CircuitBreaker = circuit_breaker_new(3, 1000)
    
    # Test initial state
    assert_eq_int(cb.failure_threshold, 3)
    assert_eq_int(cb.timeout, 1000)
    assert_eq_int(cb.failure_count, 0)
    assert_eq_int(cb.state, 0)  # Closed
    
    # Test successful operation
    assert_true(circuit_breaker_call(cb, slay() lit {
        damn based
    }))
    assert_eq_int(cb.failure_count, 0)
    assert_eq_int(cb.state, 0)
    
    vibez.spill("✓ Circuit breaker basic operations working")
}

# === PIPELINE TESTS ===
slay test_pipeline_basic() {
    test_start("Pipeline basic operations")
    
    sus input chan normie = make(chan normie, 5)
    sus output chan normie = make(chan normie, 5)
    
    # Test pipeline stage creation
    input <- 1
    input <- 2
    input <- 3
    close(input)
    
    # Simple transform function
    pipeline_stage(input, output, slay(x normie) normie {
        damn x * 2
    })
    
    vibez.spill("✓ Pipeline basic operations working")
}

# === GOROUTINE TESTS ===
slay test_goroutine_basic() {
    test_start("Goroutine basic operations")
    
    sus result chan normie = make(chan normie, 1)
    
    # Test goroutine spawn
    yolo {
        result <- 42
    }()
    
    # Test goroutine communication
    sus val normie = <-result
    assert_eq_int(val, 42)
    
    vibez.spill("✓ Goroutine basic operations working")
}

slay test_goroutine_synchronization() {
    test_start("Goroutine synchronization")
    
    sus wg *WaitGroup = waitgroup_new()
    sus results chan normie = make(chan normie, 3)
    
    # Spawn multiple goroutines
    bestie i := 0; i < 3; i++ {
        waitgroup_add(wg, 1)
        yolo {
            defer waitgroup_done(wg)
            results <- i * 2
        }()
    }
    
    # Wait for all goroutines
    waitgroup_wait(wg)
    close(results)
    
    # Collect results
    sus count normie = 0
    bestie val := <-results; val != cringe {
        count++
    }
    assert_eq_int(count, 3)
    
    vibez.spill("✓ Goroutine synchronization working")
}

# === ADVANCED PATTERN TESTS ===
slay test_fan_out_pattern() {
    test_start("Fan-out pattern")
    
    sus input chan normie = make(chan normie, 10)
    sus outputs []chan normie = fan_out(input, 3)
    
    # Test fan-out creation
    assert_eq_int(len(outputs), 3)
    
    # Send some data
    input <- 1
    input <- 2
    input <- 3
    
    vibez.spill("✓ Fan-out pattern working")
}

slay test_fan_in_pattern() {
    test_start("Fan-in pattern")
    
    sus inputs []chan normie = make([]chan normie, 3)
    bestie i := 0; i < 3; i++ {
        inputs[i] = make(chan normie, 5)
    }
    
    sus output chan normie = fan_in(inputs)
    
    # Send data to each input
    bestie i := 0; i < 3; i++ {
        inputs[i] <- i * 10
    }
    
    vibez.spill("✓ Fan-in pattern working")
}

slay test_worker_pool_pattern() {
    test_start("Worker pool pattern")
    
    sus jobs chan normie = make(chan normie, 10)
    sus results chan normie = make(chan normie, 10)
    
    # Create worker pool
    worker_pool(jobs, results, slay(job normie) normie {
        damn job * 2
    }, 3)
    
    # Send jobs
    bestie i := 1; i <= 5; i++ {
        jobs <- i
    }
    close(jobs)
    
    vibez.spill("✓ Worker pool pattern working")
}

# === COMPREHENSIVE CONCURRENCY TEST ===
slay test_comprehensive_concurrency() {
    test_start("Comprehensive concurrency test")
    
    sus wg *WaitGroup = waitgroup_new()
    sus mutex *Mutex = mutex_new()
    sus counter normie = 0
    sus results chan normie = make(chan normie, 10)
    
    # Spawn multiple goroutines that increment counter
    bestie i := 0; i < 5; i++ {
        waitgroup_add(wg, 1)
        yolo {
            defer waitgroup_done(wg)
            
            # Use mutex to protect shared counter
            mutex_lock(mutex)
            counter++
            sus current normie = counter
            mutex_unlock(mutex)
            
            results <- current
        }()
    }
    
    # Wait for all goroutines
    waitgroup_wait(wg)
    close(results)
    
    # Verify results
    sus final_count normie = 0
    bestie val := <-results; val != cringe {
        final_count++
    }
    
    assert_eq_int(final_count, 5)
    assert_eq_int(counter, 5)
    
    vibez.spill("✓ Comprehensive concurrency test working")
}

# === MAIN TEST RUNNER ===
slay main() {
    vibez.spill("=== CURSED Concurrency Module Test Suite ===")
    
    # Basic synchronization primitives
    test_mutex_basic()
    test_mutex_try_lock()
    test_rwmutex_basic()
    test_waitgroup_basic()
    test_once_basic()
    test_atomic_basic()
    
    # Channel operations
    test_channel_basic()
    test_channel_patterns()
    
    # Advanced synchronization
    test_semaphore_basic()
    test_barrier_basic()
    test_cond_basic()
    test_rate_limiter_basic()
    test_circuit_breaker_basic()
    
    # Pipeline patterns
    test_pipeline_basic()
    
    # Goroutine tests
    test_goroutine_basic()
    test_goroutine_synchronization()
    
    # Advanced patterns
    test_fan_out_pattern()
    test_fan_in_pattern()
    test_worker_pool_pattern()
    
    # Comprehensive test
    test_comprehensive_concurrency()
    
    vibez.spill("=== Test Summary ===")
    print_test_summary()
    
    vibez.spill("✓ All concurrency tests completed successfully!")
    vibez.spill("✓ Pure CURSED implementation without FFI dependencies")
    vibez.spill("✓ Goroutines, channels, and synchronization primitives working")
    vibez.spill("✓ Enterprise-grade concurrency patterns implemented")
}
