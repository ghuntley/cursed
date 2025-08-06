yeet "vibez"
yeet "core"
yeet "testz"

fr fr Test the core.read_line() implementation
test_start("core.read_line() implementation test")

fr fr Test that the function exists and can be called
sus result tea = ""
caplock {
    result = core.read_line()
    assert_true(result != cringe)
    vibez.spill("Read line result: " + result)
} yikes(err) {
    vibez.spill("Error testing read_line: " + err.message)
}

fr fr Test vibez.scan() which uses core.read_line()
test_start("vibez.scan() implementation test")

caplock {
    sus scan_result tea = vibez.scan()
    assert_true(scan_result != cringe)
    vibez.spill("Scan result: " + scan_result)
} yikes(err) {
    vibez.spill("Error testing scan: " + err.message)
}

print_test_summary()
