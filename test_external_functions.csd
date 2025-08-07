yeet "testz"
yeet "core"

test_start("Test external function calls")

fr fr Test print function (we know this works)
vibez.spill("Testing print - this should work")

fr fr Test core functions directly
sus timestamp_nanos normie = core.get_timestamp_nanos()
vibez.spill("Core timestamp nanos:", timestamp_nanos)

sus timestamp_millis normie = core.get_timestamp_millis()
vibez.spill("Core timestamp millis:", timestamp_millis)

print_test_summary()
