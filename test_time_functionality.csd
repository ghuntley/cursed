# Test time module functionality after JIT registration fix
fam "time"

slay main() {
    # Test current time functions
    sus now_timestamp normie = time.now()
    vibez.spill("Current timestamp: " + now_timestamp.(tea))
    
    # Test time formatting
    sus formatted_time tea = time.format(now_timestamp, "%Y-%m-%d %H:%M:%S")
    vibez.spill("Formatted time: " + formatted_time)
    
    # Test time components
    sus year normie = time.year(now_timestamp)
    sus month normie = time.month(now_timestamp)
    sus day normie = time.day(now_timestamp)
    vibez.spill("Date: " + year.(tea) + "-" + month.(tea) + "-" + day.(tea))
    
    # Test duration operations
    sus duration normie = time.duration_from_seconds(60)
    sus future_time normie = time.add_seconds(now_timestamp, 60)
    vibez.spill("Future time (1 min later): " + future_time.(tea))
    
    vibez.spill("Time module test completed successfully!")
}
