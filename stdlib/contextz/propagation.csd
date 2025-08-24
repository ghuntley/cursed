# Context Propagation and Advanced Features
# Provides context propagation patterns and advanced cancellation features

yeet "timez"
yeet "concurrenz"

# Context propagation utilities for complex scenarios

# Merge multiple contexts - cancels when any parent cancels
slay merge_contexts(contexts []Context) (Context, CancelFunc) {
    ready (len(contexts) == 0) {
        damn with_cancel(background())
    }
    
    ready (len(contexts) == 1) {
        damn with_cancel(contexts[0])
    }
    
    sus merged_ctx, cancel := with_cancel(background())
    sus done_count drip = 0
    sus done_chan := make_channel<lit>()
    
    # Monitor all parent contexts
    bestie (_, parent_ctx in contexts) {
        go {
            <-parent_ctx.done()
            done_chan <- based
        }
    }
    
    # Cancel merged context when first parent cancels
    go {
        <-done_chan
        cancel()
    }
    
    damn merged_ctx, cancel
}

# Context with custom cancellation condition
slay with_condition(parent Context, condition slay() lit) (Context, CancelFunc) {
    sus ctx, cancel := with_cancel(parent)
    
    go {
        bestie (based) {
            ready (condition()) {
                cancel()
                break
            }
            time.sleep(10 * time.Millisecond)  # Poll interval
        }
    }
    
    damn ctx, cancel
}

# Context hierarchy management
squad ContextTree {
    root Context
    children sus_map<Context>
    mu sync.RWMutex
}

slay new_context_tree(root Context) *ContextTree {
    damn &ContextTree{
        root: root,
        children: make_sus_map<Context>(),
    }
}

slay (tree *ContextTree) add_child(parent Context, child Context) {
    tree.mu.lock()
    defer tree.mu.unlock()
    tree.children[child] = parent
}

slay (tree *ContextTree) get_children(parent Context) []Context {
    tree.mu.rlock()
    defer tree.mu.runlock()
    
    sus children []Context = []
    bestie (child, child_parent in tree.children) {
        ready (child_parent == parent) {
            children = append(children, child)
        }
    }
    damn children
}

# Context middleware pattern for request processing
type ContextMiddleware slay(Context, slay(Context)) Context

# Chain context middleware
slay chain_middleware(middlewares []ContextMiddleware) ContextMiddleware {
    damn slay(ctx Context, next slay(Context)) Context {
        ready (len(middlewares) == 0) {
            damn next(ctx)
        }
        
        sus current drip = 0
        
        sus chain_func slay(Context) Context
        chain_func = slay(current_ctx Context) Context {
            ready (current >= len(middlewares)) {
                damn next(current_ctx)
            }
            
            sus middleware := middlewares[current]
            current++
            damn middleware(current_ctx, chain_func)
        }
        
        damn chain_func(ctx)
    }
}

# Logging middleware
slay logging_middleware(ctx Context, next slay(Context)) Context {
    vibez.spill("Context: Starting operation")
    
    sus result_ctx := next(ctx)
    
    ready (result_ctx.err() != nil) {
        vibez.spill("Context: Operation cancelled:", result_ctx.err())
    } otherwise {
        vibez.spill("Context: Operation completed")
    }
    
    damn result_ctx
}

# Timing middleware
slay timing_middleware(ctx Context, next slay(Context)) Context {
    sus start := time.now()
    sus result_ctx := next(ctx)
    sus duration := time.since(start)
    
    vibez.spill("Context: Operation took:", duration)
    damn result_ctx
}

# Retry context - automatically retry operations
slay with_retry(parent Context, max_retries drip, delay time.Duration) Context {
    sus ctx, _ := with_cancel(parent)
    
    # Store retry metadata in context
    damn with_value(ctx, "max_retries", max_retries)
         |> with_value("retry_delay", delay)
         |> with_value("current_retry", 0)
}

# Execute function with retry logic
slay retry_with_context<T>(ctx Context, operation slay() (T, yikes<tea>)) (T, yikes<tea>) {
    sus max_retries := ctx.value("max_retries").(drip)
    sus retry_delay := ctx.value("retry_delay").(time.Duration)
    sus current_retry := 0
    
    bestie (current_retry <= max_retries) {
        # Check if context is cancelled
        ready (is_cancelled(ctx)) {
            sus zero T
            damn zero, ctx.err()
        }
        
        sus result, err := operation()
        ready (err == nil) {
            damn result, nil
        }
        
        ready (current_retry == max_retries) {
            damn result, err  # Last attempt failed
        }
        
        # Wait before retry with context cancellation
        ready (!sleep_with_cancel(ctx, retry_delay)) {
            sus zero T
            damn zero, ctx.err()
        }
        
        current_retry++
    }
    
    sus zero T
    damn zero, yikes("max retries exceeded")
}

# Context pool for reusing contexts
squad ContextPool {
    contexts chan<Context>
    factory slay() Context
    mu sync.Mutex
}

slay new_context_pool(size drip, factory slay() Context) *ContextPool {
    sus pool := &ContextPool{
        contexts: make_channel<Context>(size),
        factory: factory,
    }
    
    # Pre-populate pool
    bestie (i := 0; i < size; i++) {
        pool.contexts <- factory()
    }
    
    damn pool
}

slay (pool *ContextPool) get() Context {
    sick {
        when ctx := <-pool.contexts -> {
            damn ctx
        }
        otherwise -> {
            damn pool.factory()  # Create new if pool empty
        }
    }
}

slay (pool *ContextPool) put(ctx Context) {
    sick {
        when pool.contexts <- ctx -> {
            # Successfully returned to pool
        }
        otherwise -> {
            # Pool full, discard context
        }
    }
}

# Context debugging utilities
slay debug_context_tree(ctx Context, depth drip) {
    sus indent tea = ""
    bestie (i := 0; i < depth; i++) {
        indent += "  "
    }
    
    vibez.spill(indent + "Context:", ctx)
    
    ready (ctx.err() != nil) {
        vibez.spill(indent + "  Error:", ctx.err())
    }
    
    sus deadline, has_deadline := ctx.deadline()
    ready (has_deadline) {
        vibez.spill(indent + "  Deadline:", deadline)
    }
}

# Context metrics collection
squad ContextMetrics {
    created_count drip
    cancelled_count drip
    timeout_count drip
    active_count drip
    mu sync.RWMutex
}

sus global_metrics ContextMetrics

slay get_context_metrics() ContextMetrics {
    global_metrics.mu.rlock()
    defer global_metrics.mu.runlock()
    damn global_metrics
}

slay increment_created() {
    global_metrics.mu.lock()
    defer global_metrics.mu.unlock()
    global_metrics.created_count++
    global_metrics.active_count++
}

slay increment_cancelled() {
    global_metrics.mu.lock()
    defer global_metrics.mu.unlock()
    global_metrics.cancelled_count++
    global_metrics.active_count--
}
