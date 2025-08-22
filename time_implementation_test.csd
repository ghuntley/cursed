fr fr CURSED Time Implementation Test
fr fr Tests the real system time integration fixes

yeet "timez"
yeet "vibez"

slay main() drip {
    vibez.spill("🕐 Testing Real Time Implementation")
    vibez.spill("=====================================")
    
    fr fr Test 1: Current Time (should not be hardcoded)
    vibez.spill("\n1. Testing current timestamp:")
    sus current_timestamp drip = time_unix_timestamp()
    vibez.spill("  Current Unix timestamp: " + json_number_to_string(current_timestamp))
    
    fr fr Test 2: Current DateTime 
    vibez.spill("\n2. Testing current datetime:")
    sus now DateTime = time_now()
    vibez.spill("  Year: " + json_number_to_string(now.year))
    vibez.spill("  Month: " + json_number_to_string(now.month)) 
    vibez.spill("  Day: " + json_number_to_string(now.day))
    vibez.spill("  Hour: " + json_number_to_string(now.hour))
    vibez.spill("  Timezone: " + now.timezone_name)
    vibez.spill("  Offset: " + json_number_to_string(now.timezone_offset))
    
    fr fr Test 3: UTC Time
    vibez.spill("\n3. Testing UTC time:")
    sus utc DateTime = time_utc_now()
    vibez.spill("  UTC Year: " + json_number_to_string(utc.year))
    vibez.spill("  UTC Timezone: " + utc.timezone_name)
    vibez.spill("  UTC Offset: " + json_number_to_string(utc.timezone_offset))
    
    fr fr Test 4: Timestamp Conversion
    vibez.spill("\n4. Testing timestamp conversion:")
    sus test_timestamp drip = 1640995200000  fr fr Jan 1, 2022 00:00:00 UTC
    sus converted DateTime = timestamp_to_datetime(test_timestamp)
    vibez.spill("  Test timestamp: " + json_number_to_string(test_timestamp))
    vibez.spill("  Converted Year: " + json_number_to_string(converted.year))
    vibez.spill("  Converted Month: " + json_number_to_string(converted.month))
    vibez.spill("  Converted Day: " + json_number_to_string(converted.day))
    
    fr fr Test 5: DateTime to Timestamp  
    vibez.spill("\n5. Testing datetime to timestamp:")
    sus test_dt DateTime = time_create(2024, 1, 1, 12, 0, 0)
    sus back_timestamp drip = datetime_to_timestamp(test_dt)
    vibez.spill("  Created DateTime: 2024-01-01 12:00:00")
    vibez.spill("  Back to timestamp: " + json_number_to_string(back_timestamp))
    
    fr fr Test 6: Formatting
    vibez.spill("\n6. Testing time formatting:")
    sus formatted tea = time_format(now, "YYYY-MM-DD HH:mm:ss")
    vibez.spill("  Formatted current time: " + formatted)
    
    sus iso tea = time_to_iso8601(now)
    vibez.spill("  ISO 8601 format: " + iso)
    
    fr fr Test 7: Sleep Test
    vibez.spill("\n7. Testing sleep function:")
    vibez.spill("  Sleeping for 100ms...")
    time_sleep(100)
    vibez.spill("  Sleep completed!")
    
    fr fr Test 8: Timer Test
    vibez.spill("\n8. Testing timer:")
    sus timer Timer = timer_start()
    time_sleep(50)  fr fr Sleep 50ms
    timer = timer_stop(timer)
    sus elapsed drip = timer_elapsed(timer)
    vibez.spill("  Timer elapsed: " + json_number_to_string(elapsed) + "ms")
    
    fr fr Test 9: Date Arithmetic
    vibez.spill("\n9. Testing date arithmetic:")
    sus future DateTime = time_add_days(now, 30)
    vibez.spill("  30 days from now: " + json_number_to_string(future.year) + "-" + 
                json_number_to_string(future.month) + "-" + json_number_to_string(future.day))
    
    sus past DateTime = time_add_days(now, -30)
    vibez.spill("  30 days ago: " + json_number_to_string(past.year) + "-" + 
                json_number_to_string(past.month) + "-" + json_number_to_string(past.day))
    
    fr fr Test 10: Time Difference
    vibez.spill("\n10. Testing time differences:")
    sus diff_days drip = time_diff_days(past, future)
    vibez.spill("  Days between past and future: " + json_number_to_string(diff_days))
    
    fr fr Test 11: Timezone Conversions
    vibez.spill("\n11. Testing timezone conversions:")
    sus est_time DateTime = time_from_utc(utc, "EST")
    vibez.spill("  UTC to EST hour: " + json_number_to_string(est_time.hour))
    vibez.spill("  EST timezone: " + est_time.timezone_name)
    
    fr fr Test 12: Validation Test
    vibez.spill("\n12. Validation Results:")
    
    fr fr Check if timestamps are realistic (not hardcoded 2021 values)
    ready (now.year >= 2024) {
        vibez.spill("  ✅ Current year is realistic: " + json_number_to_string(now.year))
    } otherwise {
        vibez.spill("  ❌ Current year seems wrong: " + json_number_to_string(now.year))
    }
    
    fr fr Check if timestamp is not the old hardcoded value (1640995200000)
    ready (current_timestamp != 1640995200000) {
        vibez.spill("  ✅ Timestamp is not hardcoded")
    } otherwise {
        vibez.spill("  ❌ Timestamp appears to be hardcoded!")
    }
    
    fr fr Check if timezone offset is realistic
    sus offset drip = get_local_timezone_offset()
    ready (offset >= -720 && offset <= 720) {
        vibez.spill("  ✅ Timezone offset is realistic: " + json_number_to_string(offset))
    } otherwise {
        vibez.spill("  ❌ Timezone offset seems wrong: " + json_number_to_string(offset))
    }
    
    vibez.spill("\n🎯 Time implementation test completed!")
    vibez.spill("If timestamps show current date/time, the fix is working!")
    
    damn 0
}
