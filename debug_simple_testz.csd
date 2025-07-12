vibez.spill("Testing testz import")

slay test_start(name tea) {
    vibez.spill("Test started: " + name)
}

slay assert_true(condition lit) {
    vibez.spill("Assert true called")
}

slay print_test_summary() {
    vibez.spill("Test summary")
}

test_start("simple test")
assert_true(based)
print_test_summary()
