yeet "time"
yeet "vibez"

slay test_time() {
    sus start_time normie = time.current_time_millis()
    vibez.spill("Got current time in milliseconds")
    
    sus end_time normie = time.current_time_millis()
    sus diff normie = time.time_diff(start_time, end_time)
    vibez.spill("Time difference calculated")
    
    sus result lit = time.sleep(100)
    vibez.spill("Sleep function called")
}

slay main_character() {
    test_time()
    vibez.spill("Time module test completed!")
}
