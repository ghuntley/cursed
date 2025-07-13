yeet "timez"

# Demo of timez module functionality
vibez.spill("=== CURSED timez Module Demo ===")

# Get current time
sus current := timez.now()
sus current_val normie = current.(normie)
vibez.spill("Current time (Unix): " + current_val)

# Create durations
sus five_seconds := timez.seconds(5)
sus hundred_millis := timez.milliseconds(100)
sus thousand_nanos := timez.nanoseconds(1000)

vibez.spill("Created durations:")
sus five_sec_val normie = five_seconds.(normie)
sus hundred_ms_val normie = hundred_millis.(normie)
sus thousand_ns_val normie = thousand_nanos.(normie)
vibez.spill("5 seconds: " + five_sec_val + " nanoseconds")
vibez.spill("100 milliseconds: " + hundred_ms_val + " nanoseconds")
vibez.spill("1000 nanoseconds: " + thousand_ns_val + " nanoseconds")

# Time arithmetic
sus future := timez.add_duration(current, five_seconds)
sus past := timez.sub_duration(current, hundred_millis)

vibez.spill("Time arithmetic:")
sus future_val normie = future.(normie)
sus past_val normie = past.(normie)
vibez.spill("Future time: " + future_val)
vibez.spill("Past time: " + past_val)

# Time comparison
sus is_future_after := timez.is_after(future, current)
sus is_past_before := timez.is_before(past, current)
vibez.spill("Future is after current: " + is_future_after)
vibez.spill("Past is before current: " + is_past_before)

# Formatting
sus rfc_formatted := timez.format_rfc3339(current)
sus unix_formatted := timez.format_unix(current)
sus human_formatted := timez.format_human(current)

vibez.spill("Formatted times:")
vibez.spill("RFC3339: " + rfc_formatted)
vibez.spill("Unix: " + unix_formatted)
vibez.spill("Human: " + human_formatted)

# Duration operations
sus combined := timez.add_durations(five_seconds, hundred_millis)
sus doubled := timez.multiply_duration(five_seconds, 2)

sus combined_val normie = combined.(normie)
sus doubled_val normie = doubled.(normie)
vibez.spill("Combined duration: " + combined_val + " nanoseconds")
vibez.spill("Doubled 5 seconds: " + doubled_val + " nanoseconds")

vibez.spill("=== timez Demo Complete ===")
