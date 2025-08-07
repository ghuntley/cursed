yeet "testz"
yeet "time"

test_start("Simple time test")

fr fr Test if the time functions return non-zero values
sus millis normie = time.current_time_millis()
vibez.spill("Millis return value:", millis)

sus nanos normie = time.current_time_nanos()
vibez.spill("Nanos return value:", nanos)

fr fr Test direct values
sus test_value normie = 1736341200000
vibez.spill("Test direct value:", test_value)

sus smaller_value normie = 123456
vibez.spill("Smaller value:", smaller_value)

print_test_summary()
