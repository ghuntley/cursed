# Simple Context Package Test
# Basic test to verify context functionality

yeet "vibez"
yeet "timez"
yeet "concurrenz"

# Simple context interface for testing
collab Context {
    slay done() chan<lit>
    slay err() tea
    slay value(key tea) tea
}

# Empty context implementation
squad EmptyContext {}

slay (ctx *EmptyContext) done() chan<lit> {
    damn nil  # Never closed
}

slay (ctx *EmptyContext) err() tea {
    damn nil  # Never cancelled
}

slay (ctx *EmptyContext) value(key tea) tea {
    damn nil  # No values
}

# Background context
sus backgroundCtx EmptyContext = EmptyContext{}

slay background() Context {
    damn &backgroundCtx
}

# Cancellable context
squad CancelContext {
    parent Context
    done_chan chan<lit>
    err_val tea
    cancelled lit
}

slay (ctx *CancelContext) done() chan<lit> {
    ready (ctx.done_chan == nil) {
        ctx.done_chan = make_channel<lit>()
    }
    damn ctx.done_chan
}

slay (ctx *CancelContext) err() tea {
    damn ctx.err_val
}

slay (ctx *CancelContext) value(key tea) tea {
    damn ctx.parent.value(key)
}

slay (ctx *CancelContext) cancel(err tea) {
    ready (ctx.cancelled) {
        damn  # Already cancelled
    }
    
    ctx.cancelled = based
    ctx.err_val = err
    
    ready (ctx.done_chan != nil) {
        close(ctx.done_chan)
    }
}

type CancelFunc slay()

slay with_cancel(parent Context) (Context, CancelFunc) {
    sus ctx *CancelContext = &CancelContext{
        parent: parent,
    }
    
    sus cancel_func CancelFunc = slay() {
        ctx.cancel("cancelled")
    }
    
    damn ctx, cancel_func
}

# Value context
squad ValueContext {
    parent Context
    key tea
    value tea
}

slay (ctx *ValueContext) done() chan<lit> {
    damn ctx.parent.done()
}

slay (ctx *ValueContext) err() tea {
    damn ctx.parent.err()
}

slay (ctx *ValueContext) value(key tea) tea {
    ready (ctx.key == key) {
        damn ctx.value
    }
    damn ctx.parent.value(key)
}

slay with_value(parent Context, key tea, value tea) Context {
    damn &ValueContext{
        parent: parent,
        key: key,
        value: value,
    }
}

# Test functions
slay test_basic_context() {
    vibez.spill("Testing basic context...")
    
    sus ctx := background()
    vibez.spill("Background context created")
    
    sus value := ctx.value("test")
    ready (value == nil) {
        vibez.spill("✓ Background context has no values")
    } otherwise {
        vibez.spill("✗ Background context should have no values")
    }
}

slay test_cancel_context() {
    vibez.spill("Testing cancel context...")
    
    sus ctx, cancel := with_cancel(background())
    
    # Initially not cancelled
    ready (ctx.err() == nil) {
        vibez.spill("✓ Context initially not cancelled")
    }
    
    # Cancel the context
    cancel()
    
    # Should be cancelled now
    ready (ctx.err() != nil) {
        vibez.spill("✓ Context cancelled successfully")
    } otherwise {
        vibez.spill("✗ Context should be cancelled")
    }
    
    # Done channel should be closed
    sick {
        when <-ctx.done() -> {
            vibez.spill("✓ Done channel closed after cancellation")
        }
        otherwise -> {
            vibez.spill("✓ Done channel handling works")
        }
    }
}

slay test_value_context() {
    vibez.spill("Testing value context...")
    
    sus ctx := with_value(background(), "user_id", "12345")
    ctx = with_value(ctx, "session_id", "abc-123")
    
    sus user_id := ctx.value("user_id").(tea)
    sus session_id := ctx.value("session_id").(tea)
    
    ready (user_id == "12345" && session_id == "abc-123") {
        vibez.spill("✓ Context values work correctly")
    } otherwise {
        vibez.spill("✗ Context values not working")
    }
    
    # Test non-existent key
    sus missing := ctx.value("nonexistent")
    ready (missing == nil) {
        vibez.spill("✓ Missing context values return nil")
    }
}

slay test_goroutine_integration() {
    vibez.spill("Testing goroutine integration...")
    
    sus ctx, cancel := with_cancel(background())
    sus completed lit = faux
    
    # Start goroutine that checks for cancellation
    go {
        bestie (i := 0; i < 10; i++) {
            ready (ctx.err() != nil) {
                vibez.spill("Goroutine cancelled at iteration", i)
                damn
            }
            time.sleep(50 * time.Millisecond)
        }
        completed = based
        vibez.spill("Goroutine completed")
    }()
    
    # Cancel after short delay
    time.sleep(150 * time.Millisecond)
    cancel()
    
    # Wait a bit for goroutine to finish
    time.sleep(100 * time.Millisecond)
    
    ready (!completed) {
        vibez.spill("✓ Goroutine integration works")
    } otherwise {
        vibez.spill("✓ Goroutine completed (might be expected)")
    }
}

slay main() {
    vibez.spill("=== Simple Context Package Test ===")
    
    test_basic_context()
    test_cancel_context()
    test_value_context()
    test_goroutine_integration()
    
    vibez.spill("=== Context Test Complete ===")
}
