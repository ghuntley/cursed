yeet "vibez"
yeet "timez"

fr fr Simple timez demonstration

slay main() {
    vibez.spill("=== CURSED timez Module Demo ===")
    
    fr fr Test current time
    sus current_time Time = timez.now()
    sus formatted_time tea = timez.format_rfc3339(current_time)
    vibez.spillf("Current time: %s", formatted_time)
    
    fr fr Test timestamp
    sus ts normie = timez.timestamp()
    vibez.spillf("Timestamp (ms): %d", ts)
    
    fr fr Test duration creation
    sus one_hour Duration = timez.hours(1)
    sus thirty_mins Duration = timez.minutes(30)
    sus total_duration Duration = timez.add_durations(one_hour, thirty_mins)
    sus total_mins normie = timez.duration_minutes(total_duration)
    vibez.spillf("1.5 hours = %d minutes", total_mins)
    
    fr fr Test time arithmetic
    sus future_time Time = timez.add_hours(current_time, 2)
    sus formatted_future tea = timez.format_human(future_time)
    vibez.spillf("In 2 hours: %s", formatted_future)
    
    fr fr Test timezone operations
    sus utc_time Time = timez.to_utc(current_time)
    sus offset normie = timez.timezone_offset()
    vibez.spillf("UTC offset: %d seconds", offset)
    
    vibez.spill("=== timez Demo Complete ===")
}
