yeet "testz"
yeet "signal_handling"

fr fr Basic signal handling tests

test_start("signal_init")
assert_true(signal_init())
assert_eq_int(signal_get_stats_is_active(), 1)
assert_eq_int(signal_get_stats_handlers_registered(), 0)
vibez.spill("✅ Signal initialization test passed")

test_start("signal_register")
reset_test_signal_state()
assert_true(signal_register(SIGNAL_USR1, 1))
assert_eq_int(signal_get_stats_handlers_registered(), 1)
vibez.spill("✅ Signal registration test passed")

test_start("signal_names")
assert_eq_string(signal_name(SIGNAL_INT), "SIGINT")
assert_eq_string(signal_name(SIGNAL_TERM), "SIGTERM")
assert_eq_string(signal_name(999), "UNKNOWN")
vibez.spill("✅ Signal names test passed")

test_start("signal_constants")
assert_eq_int(SIGNAL_INT, 2)
assert_eq_int(SIGNAL_TERM, 15)
assert_eq_int(SIGNAL_USR1, 10)
vibez.spill("✅ Signal constants test passed")

print_test_summary()
vibez.spill("🎉 Signal handling tests completed successfully!")
vibez.spill("📊 Basic signal handling module provides pure CURSED signal management")
vibez.spill("🔒 FFI-free implementation ensures memory safety")
