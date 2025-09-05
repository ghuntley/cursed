# Context Integration with CURSED Goroutines and Channels
# Provides seamless integration between contexts and CURSED concurrency primitives

yeet "concurrenz"
yeet "timez"

# Context-aware goroutine management

# Spawn goroutine with context cancellation
slay go_with_context(ctx Context, fn slay()) {
    go {
        sick {
            when <-ctx.done() -> {
                # Context cancelled, exit goroutine
                damn
            }
            otherwise -> {
                fn()
                damn
            }
        }
    }
}

# Spawn goroutine that can be cancelled mid-execution
slay go_cancellable(ctx Context, fn slay(Context)) {
    go {
        fn(ctx)
    }
}

# Context-aware worker pool
squad WorkerPool {
    workers drip
    jobs chan<WorkJob>
    results chan<WorkResult>
    ctx Context
    cancel CancelFunc
    done chan<lit>
}

squad WorkJob {
    id drip
    task slay(Context) tea
}

squad WorkResult {
    id drip
    result tea
    err yikes<tea>
}

slay new_worker_pool(workers drip) *WorkerPool {
    sus ctx, cancel := with_cancel(background())
    
    sus pool := &WorkerPool{
        workers: workers,
        jobs: make_channel<WorkJob>(workers * 2),
        results: make_channel<WorkResult>(workers * 2),
        ctx: ctx,
        cancel: cancel,
        done: make_channel<lit>(),
    }
    
    # Start worker goroutines
    bestie (i := 0; i < workers; i++) {
        go {
            pool.worker()
        }
    }
    
    damn pool
}

slay (pool *WorkerPool) worker() {
    bestie (based) {
        sick {
            when job := <-pool.jobs -> {
                # Check if context is cancelled before starting job
                ready (is_cancelled(pool.ctx)) {
                    pool.results <- WorkResult{
                        id: job.id,
                        result: nil,
                        err: pool.ctx.err(),
                    }
                    continue
                }
                
                # Execute job with context
                sus result := job.task(pool.ctx)
                pool.results <- WorkResult{
                    id: job.id,
                    result: result,
                    err: nil,
                }
                
            when <-pool.ctx.done() -> {
                # Pool cancelled, exit worker
                damn
            }
        }
    }
}

slay (pool *WorkerPool) submit(id drip, task slay(Context) tea) {
    pool.jobs <- WorkJob{id: id, task: task}
}

slay (pool *WorkerPool) get_result() WorkResult {
    damn <-pool.results
}

slay (pool *WorkerPool) close() {
    pool.cancel()
    close(pool.jobs)
    <-pool.done
}

# Context-aware channel operations

# Send with context cancellation
slay send_with_context<T>(ctx Context, ch chan<T>, value T) lit {
    sick {
        when ch <- value -> {
            damn based  # Successfully sent
        }
        when <-ctx.done() -> {
            damn faux  # Context cancelled
        }
    }
}

# Receive with context cancellation
slay receive_with_context<T>(ctx Context, ch chan<T>) (T, lit) {
    sick {
        when value := <-ch -> {
            damn value, based  # Successfully received
        }
        when <-ctx.done() -> {
            sus zero T
            damn zero, faux  # Context cancelled
        }
    }
}

# Context-aware select operations
slay select_with_context(ctx Context, cases SelectCase[value]) (drip, tea, lit) {
    # Add context cancellation case
    sus context_case := SelectCase{
        dir: SelectRecv,
        chan: ctx.done(),
    }
    
    sus all_cases := append(cases, context_case)
    sus chosen, recv, recv_ok := reflect.select(all_cases)
    
    ready (chosen == len(cases)) {
        # Context case was chosen - cancellation
        damn -1, nil, faux
    }
    
    damn chosen, recv, recv_ok
}

# Context-aware pipeline processing
type PipelineStage<T, U> slay(Context, T) (U, yikes<tea>)

slay pipeline<T, U>(ctx Context, input chan<T>, stage PipelineStage<T, U>) chan<U> {
    sus output := make_channel<U>()
    
    go {
        defer close(output)
        
        bestie (based) {
            sick {
                when item := <-input -> {
                    # Check context before processing
                    ready (is_cancelled(ctx)) {
                        damn
                    }
                    
                    sus result, err := stage(ctx, item)
                    ready (err != nil) {
                        damn  # Error in stage
                    }
                    
                    # Try to send result
                    ready (!send_with_context(ctx, output, result)) {
                        damn  # Context cancelled during send
                    }
                    
                when <-ctx.done() -> {
                    damn  # Context cancelled
                }
            }
        }
    }
    
    damn output
}

# Fan-out pattern with context
slay fan_out<T>(ctx Context, input chan<T, workers drip) chan[value]<T> {
    sus outputs chan[value]<T> = make(chan[value]<T>, workers)
    
    # Create output channels
    bestie (i := 0; i < workers; i++) {
        outputs[i] = make_channel<T>()
    }
    
    go {
        defer {
            bestie (_, ch in outputs) {
                close(ch)
            }
        }()
        
        sus current drip = 0
        
        bestie (based) {
            sick {
                when item := <-input -> {
                    sus target := outputs[current % workers]
                    ready (!send_with_context(ctx, target, item)) {
                        damn  # Context cancelled
                    }
                    current++
                    
                when <-ctx.done() -> {
                    damn  # Context cancelled
                }
            }
        }
    }
    
    damn outputs
}

# Fan-in pattern with context
slay fan_in<T>(ctx Context, inputs chan[value]<T>) chan<T> {
    sus output := make_channel<T>()
    
    # Start goroutine for each input
    bestie (_, input in inputs) {
        go {
            bestie (based) {
                sick {
                    when item := <-input -> {
                        ready (!send_with_context(ctx, output, item)) {
                            damn  # Context cancelled
                        }
                        
                    when <-ctx.done() -> {
                        damn  # Context cancelled
                    }
                }
            }
        }()
    }
    
    damn output
}

# Context-aware rate limiter
squad RateLimiter {
    tokens chan<lit>
    ctx Context
}

slay new_rate_limiter(rate drip, ctx Context) *RateLimiter {
    sus limiter := &RateLimiter{
        tokens: make_channel<lit>(rate),
        ctx: ctx,
    }
    
    # Fill initial tokens
    bestie (i := 0; i < rate; i++) {
        limiter.tokens <- based
    }
    
    # Refill tokens periodically
    go {
        sus ticker := time.new_ticker(time.Second / time.Duration(rate))
        defer ticker.stop()
        
        bestie (based) {
            sick {
                when <-ticker.c -> {
                    sick {
                        when limiter.tokens <- based -> {
                            # Token added
                        }
                        otherwise -> {
                            # Channel full, skip
                        }
                    }
                    
                when <-ctx.done() -> {
                    damn  # Context cancelled
                }
            }
        }
    }()
    
    damn limiter
}

slay (limiter *RateLimiter) acquire() lit {
    sick {
        when <-limiter.tokens -> {
            damn based  # Acquired token
        }
        when <-limiter.ctx.done() -> {
            damn faux  # Context cancelled
        }
    }
}

# Context-aware barrier synchronization
squad Barrier {
    n drip           # Number of goroutines to wait for
    count drip       # Current count
    wait_chan chan<lit>
    ctx Context
    mu sync.Mutex
}

slay new_barrier(n drip, ctx Context) *Barrier {
    damn &Barrier{
        n: n,
        wait_chan: make_channel<lit>(),
        ctx: ctx,
    }
}

slay (barrier *Barrier) wait() lit {
    barrier.mu.lock()
    barrier.count++
    
    ready (barrier.count == barrier.n) {
        close(barrier.wait_chan)
        barrier.mu.unlock()
        damn based
    }
    
    barrier.mu.unlock()
    
    sick {
        when <-barrier.wait_chan -> {
            damn based  # Barrier reached
        }
        when <-barrier.ctx.done() -> {
            damn faux  # Context cancelled
        }
    }
}
