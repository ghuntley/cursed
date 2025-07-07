yeet "time"

slay main() {
    vibez.spill("Time Module Demo");
    
    // Get current time
    sus now normie = time_now();
    vibez.spill("Current timestamp:", now);
    
    // Get current time in milliseconds
    sus millis normie = time_now_millis();
    vibez.spill("Current time in milliseconds:", millis);
    
    // Test time component extraction
    sus year normie = time_year(now);
    sus month normie = time_month(now);
    sus day normie = time_day(now);
    
    vibez.spill("Current date: ", year, "-", month, "-", day);
    
    // Test time arithmetic
    sus tomorrow normie = time_add_days(now, 1);
    vibez.spill("Tomorrow timestamp:", tomorrow);
    
    // Test sleep function
    vibez.spill("Sleeping for 1 second...");
    time_sleep(1);
    vibez.spill("Wake up!");
    
    vibez.spill("Time module is working!");
}
