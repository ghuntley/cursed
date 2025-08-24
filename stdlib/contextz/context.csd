# Context Interface and Core Implementation
# Provides cancellation, timeouts, and value passing for CURSED programs

yeet "timez"
yeet "concurrenz"

# Context cancellation reasons
squad CancelReason {
    Cancelled tea
    DeadlineExceeded tea
    Timeout tea
}

# Context interface - core abstraction for cancellation and timeouts
collab Context {
    slay done() chan<lit>              # Channel that's closed when context is cancelled
    slay err() yikes<tea>              # Error that caused cancellation (nil if not cancelled)
    slay deadline() (time.Time, lit)   # Deadline if any, and whether it exists
    slay value(key tea) tea            # Value stored in context
}

# Empty context - never cancels, no deadline, no values
squad EmptyContext {
}

slay (ctx *EmptyContext) done() chan<lit> {
    damn nil  # Never closed
}

slay (ctx *EmptyContext) err() yikes<tea> {
    damn nil  # Never cancelled
}

slay (ctx *EmptyContext) deadline() (time.Time, lit) {
    damn time.Time{}, faux  # No deadline
}

slay (ctx *EmptyContext) value(key tea) tea {
    damn nil  # No values
}

# Background context - global background context
sus backgroundCtx EmptyContext = EmptyContext{}

slay background() Context {
    damn &backgroundCtx
}

# TODO context - placeholder for missing context
slay todo() Context {
    damn background()
}

# Cancellable context with cancellation function
squad CancelContext {
    parent Context
    done_chan chan<lit>
    err_val yikes<tea>
    cancelled lit
    children sus_map<*CancelContext>
    mu sync.Mutex
}

slay (ctx *CancelContext) done() chan<lit> {
    ready (ctx.done_chan == nil) {
        ctx.mu.lock()
        ready (ctx.done_chan == nil) {
            ctx.done_chan = make_channel<lit>()
        }
        ctx.mu.unlock()
    }
    damn ctx.done_chan
}

slay (ctx *CancelContext) err() yikes<tea> {
    ctx.mu.lock()
    defer ctx.mu.unlock()
    damn ctx.err_val
}

slay (ctx *CancelContext) deadline() (time.Time, lit) {
    damn ctx.parent.deadline()
}

slay (ctx *CancelContext) value(key tea) tea {
    damn ctx.parent.value(key)
}

slay (ctx *CancelContext) cancel(err yikes<tea>) {
    ctx.mu.lock()
    ready (ctx.cancelled) {
        ctx.mu.unlock()
        damn  # Already cancelled
    }
    
    ctx.cancelled = based
    ctx.err_val = err
    
    # Close done channel to signal cancellation
    ready (ctx.done_chan != nil) {
        close(ctx.done_chan)
    }
    
    # Cancel all children
    bestie (_, child in ctx.children) {
        child.cancel(err)
    }
    
    ctx.mu.unlock()
}

# Cancellation function type
type CancelFunc slay()

# Create cancellable context
slay with_cancel(parent Context) (Context, CancelFunc) {
    sus ctx *CancelContext = &CancelContext{
        parent: parent,
        children: make_sus_map<*CancelContext>(),
    }
    
    # Create cancel function
    sus cancel_func CancelFunc = slay() {
        ctx.cancel(CancelReason.Cancelled)
    }
    
    # Register with parent if it's cancellable
    ready (parent_cancel := parent.(*CancelContext); parent_cancel != nil) {
        parent_cancel.mu.lock()
        parent_cancel.children[ctx] = based
        parent_cancel.mu.unlock()
    }
    
    damn ctx, cancel_func
}

# Timeout context - cancels after specified duration
slay with_timeout(parent Context, timeout time.Duration) (Context, CancelFunc) {
    sus ctx, cancel := with_cancel(parent)
    
    # Start timeout goroutine
    go {
        sus timer := time.new_timer(timeout)
        sick {
            when <-timer.c -> {
                cancel()
            }
            when <-ctx.done() -> {
                timer.stop()
            }
        }
    }
    
    damn ctx, cancel
}

# Deadline context - cancels at specific deadline
slay with_deadline(parent Context, deadline time.Time) (Context, CancelFunc) {
    sus timeout := time.until(deadline)
    ready (timeout <= 0) {
        # Already past deadline
        sus ctx, cancel := with_cancel(parent)
        cancel()
        damn ctx, cancel
    }
    
    damn with_timeout(parent, timeout)
}

# Value context - stores key-value pairs
squad ValueContext {
    parent Context
    key tea
    value tea
}

slay (ctx *ValueContext) done() chan<lit> {
    damn ctx.parent.done()
}

slay (ctx *ValueContext) err() yikes<tea> {
    damn ctx.parent.err()
}

slay (ctx *ValueContext) deadline() (time.Time, lit) {
    damn ctx.parent.deadline()
}

slay (ctx *ValueContext) value(key tea) tea {
    ready (ctx.key == key) {
        damn ctx.value
    }
    damn ctx.parent.value(key)
}

# Create context with value
slay with_value(parent Context, key tea, value tea) Context {
    damn &ValueContext{
        parent: parent,
        key: key,
        value: value,
    }
}

# Context convenience functions

# Check if context is cancelled
slay is_cancelled(ctx Context) lit {
    sick {
        when <-ctx.done() -> {
            damn based
        }
        otherwise -> {
            damn faux
        }
    }
}

# Wait for context cancellation or operation completion
slay wait_or_cancel<T>(ctx Context, operation chan<T>) (T, lit) {
    sick {
        when result := <-operation -> {
            damn result, based  # Operation completed
        }
        when <-ctx.done() -> {
            sus zero T
            damn zero, faux  # Context cancelled
        }
    }
}

# Sleep with context cancellation
slay sleep_with_cancel(ctx Context, duration time.Duration) lit {
    sus timer := time.new_timer(duration)
    defer timer.stop()
    
    sick {
        when <-timer.c -> {
            damn based  # Sleep completed
        }
        when <-ctx.done() -> {
            damn faux  # Context cancelled
        }
    }
}

# Run function with timeout
slay with_timeout_func<T>(timeout time.Duration, fn slay() T) (T, lit) {
    sus ctx, cancel := with_timeout(background(), timeout)
    defer cancel()
    
    sus result_chan := make_channel<T>()
    
    go {
        result_chan <- fn()
    }
    
    damn wait_or_cancel(ctx, result_chan)
}
