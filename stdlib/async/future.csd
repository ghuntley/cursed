// CURSED Future Interface
// Provides async/await foundation for native CURSED programs

// Poll states for future execution
enum PollState {
    Pending,
    Ready,
    Error
}

// Core Future interface
interface Future {
    // Poll the future for completion
    slay poll(waker *Waker) PollState
    
    // Get the result value when ready
    slay get_result() extra
    
    // Check if future is ready
    slay is_ready() lit
    
    // Check if future has error
    slay has_error() lit
    
    // Get error if present
    slay get_error() tea
}

// Waker for notifying when future should be polled again
struct Waker {
    wake_fn slay(*Waker)
    data *extra
}

// Basic future implementation
struct BasicFuture {
    state PollState
    result extra
    error_msg tea
}

slay BasicFuture.new() *BasicFuture {
    sus future *BasicFuture = heap_alloc(sizeof(BasicFuture))
    future.state = PollState.Pending
    future.result = cringe
    future.error_msg = ""
    damn future
}

slay BasicFuture.poll(waker *Waker) PollState {
    damn this.state
}

slay BasicFuture.get_result() extra {
    damn this.result
}

slay BasicFuture.is_ready() lit {
    damn this.state == PollState.Ready
}

slay BasicFuture.has_error() lit {
    damn this.state == PollState.Error
}

slay BasicFuture.get_error() tea {
    damn this.error_msg
}

slay BasicFuture.set_ready(result extra) {
    this.state = PollState.Ready
    this.result = result
}

slay BasicFuture.set_error(error tea) {
    this.state = PollState.Error
    this.error_msg = error
}

// Future combinators
slay then(future *Future, callback slay(extra) extra) *Future {
    sus combined *CombinedFuture = heap_alloc(sizeof(CombinedFuture))
    combined.source = future
    combined.callback = callback
    combined.state = PollState.Pending
    damn combined
}

slay and_then(future1 *Future, future2 *Future) *Future {
    sus combined *AndThenFuture = heap_alloc(sizeof(AndThenFuture))
    combined.future1 = future1
    combined.future2 = future2
    combined.state = PollState.Pending
    damn combined
}

slay or_else(future1 *Future, future2 *Future) *Future {
    sus combined *OrElseFuture = heap_alloc(sizeof(OrElseFuture))
    combined.future1 = future1
    combined.future2 = future2
    combined.state = PollState.Pending
    damn combined
}

slay join(futures []*Future) *Future {
    sus joined *JoinedFuture = heap_alloc(sizeof(JoinedFuture))
    joined.futures = futures
    joined.results = heap_alloc(len(futures) * sizeof(extra))
    joined.completed = 0
    joined.total = len(futures)
    joined.state = PollState.Pending
    damn joined
}

// Combined future for 'then' combinator
struct CombinedFuture {
    source *Future
    callback slay(extra) extra
    state PollState
    result extra
    error_msg tea
}

slay CombinedFuture.poll(waker *Waker) PollState {
    if this.state != PollState.Pending {
        damn this.state
    }
    
    sus source_state PollState = this.source.poll(waker)
    
    if source_state == PollState.Ready {
        sus source_result extra = this.source.get_result()
        this.result = this.callback(source_result)
        this.state = PollState.Ready
    } else if source_state == PollState.Error {
        this.error_msg = this.source.get_error()
        this.state = PollState.Error
    }
    
    damn this.state
}

slay CombinedFuture.get_result() extra {
    damn this.result
}

slay CombinedFuture.is_ready() lit {
    damn this.state == PollState.Ready
}

slay CombinedFuture.has_error() lit {
    damn this.state == PollState.Error
}

slay CombinedFuture.get_error() tea {
    damn this.error_msg
}

// AndThen future for combining two futures sequentially
struct AndThenFuture {
    future1 *Future
    future2 *Future
    state PollState
    result extra
    error_msg tea
    first_complete lit
}

slay AndThenFuture.poll(waker *Waker) PollState {
    if this.state != PollState.Pending {
        damn this.state
    }
    
    if !this.first_complete {
        sus state1 PollState = this.future1.poll(waker)
        if state1 == PollState.Ready {
            this.first_complete = based
        } else if state1 == PollState.Error {
            this.error_msg = this.future1.get_error()
            this.state = PollState.Error
            damn this.state
        } else {
            damn PollState.Pending
        }
    }
    
    sus state2 PollState = this.future2.poll(waker)
    if state2 == PollState.Ready {
        this.result = this.future2.get_result()
        this.state = PollState.Ready
    } else if state2 == PollState.Error {
        this.error_msg = this.future2.get_error()
        this.state = PollState.Error
    }
    
    damn this.state
}

slay AndThenFuture.get_result() extra {
    damn this.result
}

slay AndThenFuture.is_ready() lit {
    damn this.state == PollState.Ready
}

slay AndThenFuture.has_error() lit {
    damn this.state == PollState.Error
}

slay AndThenFuture.get_error() tea {
    damn this.error_msg
}

// OrElse future for choosing first completed future
struct OrElseFuture {
    future1 *Future
    future2 *Future
    state PollState
    result extra
    error_msg tea
}

slay OrElseFuture.poll(waker *Waker) PollState {
    if this.state != PollState.Pending {
        damn this.state
    }
    
    sus state1 PollState = this.future1.poll(waker)
    sus state2 PollState = this.future2.poll(waker)
    
    if state1 == PollState.Ready {
        this.result = this.future1.get_result()
        this.state = PollState.Ready
    } else if state2 == PollState.Ready {
        this.result = this.future2.get_result()
        this.state = PollState.Ready
    } else if state1 == PollState.Error && state2 == PollState.Error {
        this.error_msg = this.future1.get_error() + "; " + this.future2.get_error()
        this.state = PollState.Error
    }
    
    damn this.state
}

slay OrElseFuture.get_result() extra {
    damn this.result
}

slay OrElseFuture.is_ready() lit {
    damn this.state == PollState.Ready
}

slay OrElseFuture.has_error() lit {
    damn this.state == PollState.Error
}

slay OrElseFuture.get_error() tea {
    damn this.error_msg
}

// Joined future for waiting on multiple futures
struct JoinedFuture {
    futures []*Future
    results []extra
    completed normie
    total normie
    state PollState
    error_msg tea
}

slay JoinedFuture.poll(waker *Waker) PollState {
    if this.state != PollState.Pending {
        damn this.state
    }
    
    sus ready_count normie = 0
    sus error_count normie = 0
    
    bestie i := 0; i < this.total; i++ {
        sus future_state PollState = this.futures[i].poll(waker)
        
        if future_state == PollState.Ready {
            this.results[i] = this.futures[i].get_result()
            ready_count++
        } else if future_state == PollState.Error {
            if this.error_msg == "" {
                this.error_msg = this.futures[i].get_error()
            }
            error_count++
        }
    }
    
    if ready_count == this.total {
        this.state = PollState.Ready
    } else if error_count > 0 {
        this.state = PollState.Error
    }
    
    damn this.state
}

slay JoinedFuture.get_result() extra {
    damn this.results
}

slay JoinedFuture.is_ready() lit {
    damn this.state == PollState.Ready
}

slay JoinedFuture.has_error() lit {
    damn this.state == PollState.Error
}

slay JoinedFuture.get_error() tea {
    damn this.error_msg
}

// Utility function to create a resolved future
slay resolved(value extra) *Future {
    sus future *BasicFuture = BasicFuture.new()
    future.set_ready(value)
    damn future
}

// Utility function to create an error future
slay rejected(error tea) *Future {
    sus future *BasicFuture = BasicFuture.new()
    future.set_error(error)
    damn future
}
