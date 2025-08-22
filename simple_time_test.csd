fr fr Simple Time Test - Tests runtime time functions

yeet "vibez"

slay main() drip {
    vibez.spill("Testing Time Functions")
    
    fr fr Test external runtime functions directly
    sus current_ms drip = runtime_get_current_time_ms()
    vibez.spill("Current timestamp (should not be 0):")
    
    ready (current_ms > 1000000000000) {
        vibez.spill("✅ Timestamp looks realistic")
    } otherwise {
        vibez.spill("❌ Timestamp seems wrong")
    }
    
    fr fr Test timezone offset
    sus offset drip = runtime_get_timezone_offset()
    vibez.spill("Timezone offset in minutes:")
    
    ready (offset >= -720 && offset <= 720) {
        vibez.spill("✅ Offset is reasonable") 
    } otherwise {
        vibez.spill("❌ Offset is out of range")
    }
    
    fr fr Test timezone name  
    sus tz_name tea = runtime_get_timezone_name()
    vibez.spill("Timezone name (should not be empty):")
    
    ready (tz_name != "") {
        vibez.spill("✅ Timezone name retrieved")
    } otherwise {
        vibez.spill("❌ Timezone name is empty")
    }
    
    fr fr Test sleep function
    vibez.spill("Testing sleep (50ms)...")
    runtime_sleep_ms(50)
    vibez.spill("Sleep completed!")
    
    vibez.spill("🎯 Basic time functions test completed")
    
    damn 0
}
