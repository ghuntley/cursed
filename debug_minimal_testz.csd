slay test_start(name tea) {
    vibez.spill("Test: " + name)
}

slay assert_true(condition lit) {
    lowkey condition == based {
        vibez.spill("PASS: condition is based")
    } else {
        vibez.spill("FAIL: got " + tea(condition) + ", expected based")
    }
}
