# Simple testz implementation that works with current interpreter

# Direct function definitions without complex imports/exports
slay test_start(name tea) {
    vibez.spill("Test: " + name)
}

slay assert_eq_string(actual tea, expected tea) {
    if actual == expected {
        vibez.spill("PASS: string equality")
    } else {
        vibez.spill("FAIL: string equality - expected '" + expected + "' got '" + actual + "'")
    }
}

slay assert_eq_int(actual normie, expected normie) {
    if actual == expected {
        vibez.spill("PASS: int equality")
    } else {
        vibez.spill("FAIL: int equality - expected " + expected + " got " + actual)
    }
}

slay assert_true(condition lit) {
    if condition == based {
        vibez.spill("PASS: assert_true")
    } else {
        vibez.spill("FAIL: assert_true - expected true")
    }
}

slay assert_false(condition lit) {
    if condition == cap {
        vibez.spill("PASS: assert_false")
    } else {
        vibez.spill("FAIL: assert_false - expected false")
    }
}

slay print_test_summary() {
    vibez.spill("Test summary completed")
}
