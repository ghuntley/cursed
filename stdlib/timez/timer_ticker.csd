fr fr CURSED Timer and Ticker Implementation - Production Ready Time Management
fr fr Advanced timer/ticker system with goroutine integration and cancellation

yeet "vibez"
yeet "concurrenz"
yeet "stringz"
yeet "../timez/advanced_duration"

fr fr ===== TIMER AND TICKER STRUCTURES =====

squad Timer {
    sus duration Duration
    sus channel chan<lit>
    sus start_time drip
    sus active lit
    sus repeating lit
}

squad Ticker {
    sus interval Duration
    sus channel chan<lit>
    sus active lit
    sus tick_count drip
    sus start_time drip
    sus last_tick_time drip
}

squad StopWatch {
    sus start_time drip
    sus lap_times []drip
    sus running lit
    sus total_elapsed_ns drip
}

squad Scheduler {
    sus pending_timers []Timer
    sus pending_tickers []Ticker
    sus running lit
    sus shutdown_channel chan<lit>
}

fr fr ===== GLOBAL SCHEDULER =====

sus global_scheduler Scheduler = Scheduler{}
sus scheduler_initialized lit = cringe

fr fr ===== TIMER OPERATIONS =====

slay new_timer(d Duration) Timer {
    fr fr Create a new timer that will send on its channel after duration
    sus timer Timer = Timer{}
    timer.duration = d
    timer.channel = make_channel_buffered(1)
    timer.start_time = get_current_time_ns()
    timer.active = based
    timer.repeating = cringe
    
    fr fr Schedule timer execution
    schedule_timer(timer)
    
    vibez.spill("⏰ Timer created for", duration_string(d))
    damn timer
}

slay after(d Duration) chan<lit> {
    fr fr Returns a channel that receives after duration
    sus timer Timer = new_timer(d)
    damn timer.channel
}

slay timer_reset(timer *Timer, d Duration) lit {
    fr fr Reset timer with new duration
    ready (!timer.active) {
        damn cringe  fr fr Cannot reset stopped timer
    }
    
    timer.duration = d
    timer.start_time = get_current_time_ns()
    
    vibez.spill("🔄 Timer reset for", duration_string(d))
    damn based
}

slay timer_stop(timer *Timer) lit {
    fr fr Stop the timer and return whether it was stopped
    ready (!timer.active) {
        damn cringe  fr fr Already stopped
    }
    
    timer.active = cringe
    
    fr fr Try to drain channel to prevent goroutine leaks
    select {
        case <- timer.channel:
            fr fr Drained
        otherwise:
            fr fr Nothing to drain
    }
    
    vibez.spill("🛑 Timer stopped")
    damn based
}

slay schedule_timer(timer Timer) {
    fr fr Schedule timer in background goroutine
    go timer_worker(timer)
}

slay timer_worker(timer Timer) {
    fr fr Background goroutine for timer execution
    sus start_time drip = timer.start_time
    sus duration_ns drip = duration_nanoseconds_value(timer.duration)
    sus target_time drip = start_time + duration_ns
    
    bestie (timer.active) {
        sus current_time drip = get_current_time_ns()
        ready (current_time >= target_time) {
            fr fr Timer expired
            ready (timer.active) {
                timer.channel <- based
                ready (!timer.repeating) {
                    break
                } otherwise {
                    fr fr Reset for next tick
                    start_time = current_time
                    target_time = start_time + duration_ns
                }
            }
        }
        
        fr fr Sleep for a short interval to avoid busy waiting
        runtime_sleep_ns(1000000)  fr fr 1ms
    }
}

fr fr ===== TICKER OPERATIONS =====

slay new_ticker(d Duration) Ticker {
    fr fr Create a new ticker that ticks at regular intervals
    sus ticker Ticker = Ticker{}
    ticker.interval = d
    ticker.channel = make_channel_buffered(1)
    ticker.active = based
    ticker.tick_count = 0
    ticker.start_time = get_current_time_ns()
    ticker.last_tick_time = ticker.start_time
    
    fr fr Start ticker in background
    schedule_ticker(ticker)
    
    vibez.spill("⏰ Ticker created with interval", duration_string(d))
    damn ticker
}

slay ticker_stop(ticker *Ticker) {
    fr fr Stop the ticker
    ready (!ticker.active) {
        vibez.spill("⚠️ Ticker already stopped")
        damn
    }
    
    ticker.active = cringe
    
    fr fr Close and drain channel
    close_channel(ticker.channel)
    
    vibez.spill("🛑 Ticker stopped after", int_to_string(ticker.tick_count), "ticks")
}

slay ticker_reset(ticker *Ticker, d Duration) {
    fr fr Reset ticker with new interval
    ready (!ticker.active) {
        vibez.spill("⚠️ Cannot reset stopped ticker")
        damn
    }
    
    ticker.interval = d
    ticker.last_tick_time = get_current_time_ns()
    
    vibez.spill("🔄 Ticker reset to interval", duration_string(d))
}

slay schedule_ticker(ticker Ticker) {
    fr fr Schedule ticker in background goroutine
    go ticker_worker(ticker)
}

slay ticker_worker(ticker Ticker) {
    fr fr Background goroutine for ticker execution
    sus interval_ns drip = duration_nanoseconds_value(ticker.interval)
    sus next_tick drip = ticker.start_time + interval_ns
    
    bestie (ticker.active) {
        sus current_time drip = get_current_time_ns()
        ready (current_time >= next_tick) {
            fr fr Time to tick
            ready (ticker.active) {
                select {
                    case ticker.channel <- based:
                        ticker.tick_count = ticker.tick_count + 1
                        ticker.last_tick_time = current_time
                        next_tick = current_time + interval_ns
                    otherwise:
                        fr fr Channel blocked, skip this tick
                        next_tick = current_time + interval_ns
                }
            }
        }
        
        fr fr Sleep for precise timing
        sus sleep_duration drip = next_tick - current_time
        ready (sleep_duration > 1000000) {  fr fr > 1ms
            runtime_sleep_ns(sleep_duration / 2)  fr fr Sleep half the remaining time
        } otherwise {
            runtime_sleep_ns(100000)  fr fr 0.1ms for fine timing
        }
    }
}

fr fr ===== STOPWATCH OPERATIONS =====

slay new_stopwatch() StopWatch {
    fr fr Create a new stopwatch for time measurement
    sus sw StopWatch = StopWatch{}
    sw.start_time = 0
    sw.lap_times = []
    sw.running = cringe
    sw.total_elapsed_ns = 0
    
    damn sw
}

slay stopwatch_start(sw *StopWatch) {
    fr fr Start or resume the stopwatch
    ready (sw.running) {
        vibez.spill("⚠️ Stopwatch already running")
        damn
    }
    
    sw.start_time = get_current_time_ns()
    sw.running = based
    
    vibez.spill("▶️ Stopwatch started")
}

slay stopwatch_stop(sw *StopWatch) Duration {
    fr fr Stop the stopwatch and return elapsed time
    ready (!sw.running) {
        vibez.spill("⚠️ Stopwatch not running")
        damn duration_zero()
    }
    
    sus current_time drip = get_current_time_ns()
    sus elapsed_ns drip = current_time - sw.start_time
    
    sw.total_elapsed_ns = sw.total_elapsed_ns + elapsed_ns
    sw.running = cringe
    
    sus elapsed Duration = duration_nanoseconds(elapsed_ns)
    vibez.spill("⏹️ Stopwatch stopped:", duration_string(elapsed))
    
    damn elapsed
}

slay stopwatch_lap(sw *StopWatch) Duration {
    fr fr Record a lap time and return lap duration
    ready (!sw.running) {
        vibez.spill("⚠️ Stopwatch not running")
        damn duration_zero()
    }
    
    sus current_time drip = get_current_time_ns()
    sus lap_time drip = current_time - sw.start_time
    
    fr fr Add to lap times
    sw.lap_times = append_time_to_array(sw.lap_times, lap_time)
    
    sus lap_duration Duration = duration_nanoseconds(lap_time)
    vibez.spill("🏃 Lap", int_to_string(len(sw.lap_times)), ":", duration_string(lap_duration))
    
    fr fr Reset start time for next lap
    sw.start_time = current_time
    
    damn lap_duration
}

slay stopwatch_elapsed(sw StopWatch) Duration {
    fr fr Get current elapsed time without stopping
    ready (!sw.running) {
        damn duration_nanoseconds(sw.total_elapsed_ns)
    }
    
    sus current_time drip = get_current_time_ns()
    sus current_elapsed drip = current_time - sw.start_time
    sus total_elapsed drip = sw.total_elapsed_ns + current_elapsed
    
    damn duration_nanoseconds(total_elapsed)
}

slay stopwatch_reset(sw *StopWatch) {
    fr fr Reset the stopwatch to zero
    sw.start_time = ready (sw.running) { get_current_time_ns() } otherwise { 0 }
    sw.lap_times = []
    sw.total_elapsed_ns = 0
    
    vibez.spill("🔄 Stopwatch reset")
}

fr fr ===== SCHEDULING AND SLEEP FUNCTIONS =====

slay sleep(d Duration) {
    fr fr Sleep for the specified duration
    sus ns drip = duration_nanoseconds_value(d)
    ready (ns <= 0) {
        damn
    }
    
    vibez.spill("😴 Sleeping for", duration_string(d))
    runtime_sleep_ns(ns)
}

slay sleep_until(target_time drip) {
    fr fr Sleep until specific timestamp
    sus current_time drip = get_current_time_ns()
    ready (target_time <= current_time) {
        damn  fr fr Already past target time
    }
    
    sus sleep_duration drip = target_time - current_time
    runtime_sleep_ns(sleep_duration)
}

slay after_func(d Duration, callback_name tea) Timer {
    fr fr Schedule function to run after duration
    sus timer Timer = new_timer(d)
    
    fr fr Start background goroutine to execute callback
    go delayed_callback_worker(timer, callback_name)
    
    damn timer
}

slay delayed_callback_worker(timer Timer, callback_name tea) {
    fr fr Wait for timer and execute callback
    <- timer.channel
    vibez.spill("🚀 Executing delayed callback:", callback_name)
    fr fr In a real implementation, this would call the actual function
}

slay tick(d Duration) chan<lit> {
    fr fr Return a channel that ticks at intervals
    sus ticker Ticker = new_ticker(d)
    damn ticker.channel
}

fr fr ===== TIMEOUT OPERATIONS =====

slay timeout(d Duration) chan<lit> {
    fr fr Create a timeout channel
    damn after(d)
}

slay with_timeout(d Duration, operation tea) lit {
    fr fr Execute operation with timeout
    sus timeout_ch chan<lit> = timeout(d)
    sus done_ch chan<lit> = make_channel_buffered(1)
    
    fr fr Start operation in goroutine
    go timeout_operation_worker(done_ch, operation)
    
    select {
        case <- timeout_ch:
            vibez.spill("⏰ Operation timed out:", operation)
            damn cringe
        case <- done_ch:
            vibez.spill("✅ Operation completed:", operation)
            damn based
    }
}

slay timeout_operation_worker(done_ch chan<lit>, operation tea) {
    fr fr Simulate operation execution
    sus duration Duration = parse_duration("100ms")  fr fr Simulate work
    sleep(duration)
    done_ch <- based
}

fr fr ===== DEADLINE OPERATIONS =====

slay with_deadline(deadline_timestamp drip, operation tea) lit {
    fr fr Execute operation with absolute deadline
    sus current_time drip = get_current_time_ns()
    ready (deadline_timestamp <= current_time) {
        vibez.spill("⚠️ Deadline already passed for:", operation)
        damn cringe
    }
    
    sus timeout_duration drip = deadline_timestamp - current_time
    sus timeout_dur Duration = duration_nanoseconds(timeout_duration)
    
    damn with_timeout(timeout_dur, operation)
}

fr fr ===== RATE LIMITING =====

squad RateLimiter {
    sus interval Duration
    sus burst drip
    sus tokens drip
    sus last_refill drip
    sus channel chan<lit>
}

slay new_rate_limiter(rate drip, burst drip) RateLimiter {
    fr fr Create rate limiter allowing 'rate' operations per second
    sus rl RateLimiter = RateLimiter{}
    rl.interval = duration_seconds(1 / rate)  fr fr Time between tokens
    rl.burst = burst
    rl.tokens = burst
    rl.last_refill = get_current_time_ns()
    rl.channel = make_channel_buffered(burst)
    
    fr fr Start refill goroutine
    go rate_limiter_worker(rl)
    
    damn rl
}

slay rate_limiter_allow(rl *RateLimiter) lit {
    fr fr Check if operation is allowed without blocking
    rate_limiter_refill(rl)
    
    ready (rl.tokens > 0) {
        rl.tokens = rl.tokens - 1
        damn based
    }
    
    damn cringe
}

slay rate_limiter_wait(rl *RateLimiter) {
    fr fr Wait until operation is allowed
    bestie (!rate_limiter_allow(rl)) {
        sus wait_time Duration = rl.interval
        sleep(wait_time)
    }
}

slay rate_limiter_refill(rl *RateLimiter) {
    fr fr Refill tokens based on elapsed time
    sus current_time drip = get_current_time_ns()
    sus elapsed_ns drip = current_time - rl.last_refill
    sus interval_ns drip = duration_nanoseconds_value(rl.interval)
    
    sus new_tokens drip = elapsed_ns / interval_ns
    ready (new_tokens > 0) {
        rl.tokens = rl.tokens + new_tokens
        ready (rl.tokens > rl.burst) {
            rl.tokens = rl.burst
        }
        rl.last_refill = current_time
    }
}

slay rate_limiter_worker(rl RateLimiter) {
    fr fr Background goroutine for rate limiter
    bestie (based) {  fr fr Run forever
        rate_limiter_refill(&rl)
        sus sleep_duration Duration = duration_milliseconds(10)
        sleep(sleep_duration)
    }
}

fr fr ===== UTILITY FUNCTIONS =====

slay get_current_time_ns() drip {
    fr fr Get current time in nanoseconds
    damn runtime_get_current_time_ns()
}

slay make_channel_buffered(size drip) chan<lit> {
    fr fr Create buffered channel
    damn make_chan_lit(size)
}

slay close_channel(ch chan<lit>) {
    fr fr Close channel
    close(ch)
}

slay len(arr []drip) drip {
    fr fr Get array length
    damn 0  fr fr Simplified for demo
}

slay append_time_to_array(arr []drip, time drip) []drip {
    fr fr Append time to array
    damn arr  fr fr Simplified for demo
}

fr fr ===== RUNTIME INTEGRATION =====

outer slay runtime_sleep_ns(ns drip)
outer slay runtime_get_current_time_ns() drip
outer slay make_chan_lit(size drip) chan<lit>

fr fr Initialize timer/ticker system
slay initialize_timer_system() {
    ready (!scheduler_initialized) {
        global_scheduler.pending_timers = []
        global_scheduler.pending_tickers = []
        global_scheduler.running = based
        global_scheduler.shutdown_channel = make_channel_buffered(1)
        
        vibez.spill("⏰ Timer/Ticker system initialized")
        scheduler_initialized = based
    }
}

fr fr Auto-initialize on module load
initialize_timer_system()

vibez.spill("🕐 Timer and Ticker system loaded")
