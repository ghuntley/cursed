vibe main

yeet "vibez"  fr fr For printing results
yeet "timez"  fr fr Time functions package

slay main() {
    vibez.spill("Testing timez package")
    
    fr fr Get current time
    tea now := timez.Now()
    vibez.spill("Current time:", now)
    
    fr fr Get Unix timestamp
    tea timestamp := timez.UnixTimestamp(now)
    vibez.spill("Unix timestamp:", timestamp)
    
    fr fr Test time constants
    vibez.spill("1 Second (in ns):", timez.Second)
    vibez.spill("1 Minute (in ns):", timez.Minute)
    vibez.spill("1 Hour (in ns):", timez.Hour)
    
    fr fr Test sleep function (but keep it short for testing)
    vibez.spill("Sleeping for 10ms...")
    timez.Sleep(10)  fr fr 10 milliseconds
    vibez.spill("Awake now!")
    
    fr fr Create durations and add/subtract
    tea duration := timez.DurationFromSecs(1.5)  fr fr 1.5 seconds
    vibez.spill("Duration (1.5s) in seconds:", timez.ToSeconds(duration))
    vibez.spill("Duration (1.5s) in milliseconds:", timez.ToMillis(duration))
    
    fr fr Add duration to time
    tea later := timez.AddDuration(now, duration)
    vibez.spill("Time after adding 1.5s:", later)
    
    fr fr Calculate time difference
    tea diff := timez.Since(now)
    vibez.spill("Time passed since 'now':", timez.ToMillis(diff), "ms")
    
    fr fr Time until future time
    tea future := timez.AddDuration(now, timez.Hour)
    tea until := timez.Until(future)
    vibez.spill("Time until 1 hour from now:", timez.ToSeconds(until), "seconds")
}