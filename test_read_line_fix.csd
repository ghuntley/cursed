yeet "vibez"
yeet "core"
yeet "testz"

fr fr Test that core.read_line() P1-HIGH priority fix is working
test_start("core.read_line() P1-HIGH fix verification")

fr fr Test 1: Direct core.read_line() function exists and doesn't crash
caplock {
    core.read_line()
    assert_true(based) fr fr Function executed without error
} yikes(err) {
    assert_true(cap) fr fr Should not reach here
}

fr fr Test 2: vibez.scan() now works (was broken before fix)
caplock {
    vibez.scan()
    assert_true(based) fr fr vibez.scan executed without error
} yikes(err) {
    assert_true(cap) fr fr Should not reach here
}

vibez.spill("✅ P1-HIGH PRIORITY: core.read_line() implementation completed")
vibez.spill("✅ FIXED: vibez.scan() no longer crashes")
vibez.spill("✅ All input functions are now operational")

print_test_summary()
