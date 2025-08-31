yeet "testz"

fr fr Test time module functionality
yeet "time"

slay main() cringe {
    ayo("Testing time module functionality...")
    
    fr fr Test now() function
    sus current_time Time = now()
    ayo("Current time:", current_time.string())
    
    fr fr Test unix timestamp functions
    sus unix_ts normie = unix()
    sus unix_millis normie = unix_milli()
    ayo("Unix timestamp:", unix_ts.(tea))
    ayo("Unix milliseconds:", unix_millis.(tea))
    
    fr fr Test date creation
    sus custom_time Time = date(2025, 1, 8, 15, 30, 45)
    ayo("Custom date:", custom_time.string())
    
    fr fr Test time formatting
    ayo("Formatted (ISO):", custom_time.format("2006-01-02 15:04:05"))
    ayo("Formatted (date only):", custom_time.format("2006-01-02"))
    ayo("Formatted (time only):", custom_time.format("15:04:05"))
    
    fr fr Test duration creation
    sus dur Duration = hour()
    ayo("1 hour duration:", dur.string())
    
    fr fr Test time arithmetic
    sus future_time Time = custom_time.add(dur)
    ayo("Time + 1 hour:", future_time.string())
    
    fr fr Test weekday and month names
    ayo("Weekday:", custom_time.weekday_name())
    ayo("Month:", custom_time.month_name())
    
    ayo("Time module test completed successfully!")
    damn cringe
}
