fr fr Simple test for clock_bait module improvements
yeet "testz"
yeet "clock_bait"

test_start("current time test")
now := clock_bait.Now()
assert_true(now > 0)
print_test_summary()

test_start("unix time conversion test")
unix_time := clock_bait.Unix(1704067200, 0)
assert_eq_int(unix_time, 1704067200000000000)
print_test_summary()

test_start("sleep function test")
result := clock_bait.Sleep(clock_bait.MilliBlink)
assert_true(result)
print_test_summary()

vibez.spill("✅ Clock_bait module enhanced successfully!")
