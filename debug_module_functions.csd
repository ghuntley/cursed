slay test_start(name tea) {
    vibez.spill("Test: " + name)
}

slay assert_true(condition lit) {
    vibez.spill("Assert true called")
}

test_start("debug")
