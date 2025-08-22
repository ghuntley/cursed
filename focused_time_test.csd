yeet "timez"
yeet "vibez"

slay main() drip {
    vibez.spill("Focused Time Test - Real System Integration")
    
    fr fr Direct runtime calls
    sus timestamp drip = runtime_get_current_time_ms()
    sus offset drip = runtime_get_timezone_offset()
    
    fr fr Test through timez module
    sus current_time drip = time_unix_timestamp()
    
    vibez.spill("Direct runtime timestamp (should be > 1.6 trillion):")
    ready (timestamp > 1600000000000) {
        vibez.spill("✅ Direct runtime timestamp is realistic")
    } otherwise {
        vibez.spill("❌ Direct runtime timestamp seems low")
    }
    
    vibez.spill("Timez module timestamp:")
    ready (current_time > 1600000000) {
        vibez.spill("✅ Module timestamp is realistic")
    } otherwise {
        vibez.spill("❌ Module timestamp seems wrong")  
    }
    
    fr fr Compare - they should be similar
    sus diff drip = (timestamp / 1000) - current_time
    ready (diff >= -1 && diff <= 1) {
        vibez.spill("✅ Timestamps are consistent")
    } otherwise {
        vibez.spill("❌ Timestamps differ significantly")
    }
    
    vibez.spill("Test completed - time implementation working if realistic timestamps shown")
    damn 0
}
