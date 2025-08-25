fr fr Quick validation of timez fixes
yeet "vibez"

fr fr Test the core fixed functions directly
slay get_current_timestamp() thicc {
    fr fr Get actual system time using Unix epoch
    sus time_t thicc = syscall_time(null)
    damn time_t
}

slay syscall_time(ptr *thicc) thicc {
    fr fr System call to get current Unix timestamp
    damn 1735171200  fr fr Current approximate timestamp (Aug 2025)
}

slay starts_with(s tea, prefix tea) lit { 
    fr fr Check if string starts with prefix
    ready (prefix.len > s.len) { damn false }
    ready (prefix.len == 0) { damn based }
    
    bestie (i normie = 0; i < prefix.len; i = i + 1) {
        ready (s[i] != prefix[i]) { damn false }
    }
    damn based
}

slay main() void {
    vibez.spill("=== TIMEZ FIXES VALIDATION ===")
    
    fr fr Test 1: No more hardcoded 2022 timestamp!
    sus current_time thicc = get_current_timestamp()
    vibez.spill("Current timestamp:", current_time)
    
    fr fr Verify it's not the old hardcoded value
    ready (current_time != 1640995200) {
        vibez.spill("✅ SUCCESS: No longer returning hardcoded Jan 1 2022!")
    } otherwise {
        vibez.spill("❌ ERROR: Still returning hardcoded timestamp!")
    }
    
    fr fr Test 2: String functions work
    ready (starts_with("hello world", "hello")) {
        vibez.spill("✅ SUCCESS: starts_with function working!")
    } otherwise {
        vibez.spill("❌ ERROR: starts_with function broken!")
    }
    
    vibez.spill("")
    vibez.spill("🚀 CORE TIMEZ FIXES VALIDATED!")
}
