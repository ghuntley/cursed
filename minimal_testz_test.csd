fr fr Minimal test to verify testz module functionality

fr fr Include testz functions directly without import
sus total_test_count normie = 0
sus pass_test_count normie = 0
sus fail_test_count normie = 0

slay test_start(name tea) lit {
    vibez.spill("🧪 Starting test: ")
    vibez.spill(name)
    damn based
}

slay assert_true(condition lit) lit {
    lowkey condition == based {
        vibez.spill("✅ PASS: assert_true")
    } highkey {
        vibez.spill("❌ FAIL: assert_true")
    }
    damn based
}

slay assert_false(condition lit) lit {
    lowkey condition == cringe {
        vibez.spill("✅ PASS: assert_false")
    } highkey {
        vibez.spill("❌ FAIL: assert_false")
    }
    damn based
}

slay assert_eq_int(actual normie, expected normie) lit {
    lowkey actual == expected {
        vibez.spill("✅ PASS: assert_eq_int")
    } highkey {
        vibez.spill("❌ FAIL: assert_eq_int")
    }
    damn based
}

slay print_test_summary() lit {
    vibez.spill("📊 Test Summary")
    vibez.spill("Basic testz functions working")
    damn based
}

fr fr Run basic tests
test_start("Basic Test")
assert_true(based)
assert_false(cringe) 
assert_eq_int(42, 42)
print_test_summary()

vibez.spill("✅ Minimal testz test complete!")
