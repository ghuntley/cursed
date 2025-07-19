# Basic standalone test without importing testz
# This will test if the core functionality works

vibez.spill("🧪 Starting basic test validation...")

sus test_name tea = "basic functionality"
sus pass_count normie = 0
sus fail_count normie = 0

vibez.spill("Testing integer equality...")
sus result1 normie = 42
sus expected1 normie = 42
lowkey result1 == expected1 {
    pass_count = pass_count + 1
    vibez.spill("✅ Integer test passed")
} else {
    fail_count = fail_count + 1
    vibez.spill("❌ Integer test failed")
}

vibez.spill("Testing string equality...")
sus result2 tea = "hello"
sus expected2 tea = "hello"
lowkey result2 == expected2 {
    pass_count = pass_count + 1
    vibez.spill("✅ String test passed")
} else {
    fail_count = fail_count + 1
    vibez.spill("❌ String test failed")
}

vibez.spill("Testing boolean values...")
sus result3 lit = based
lowkey result3 == based {
    pass_count = pass_count + 1
    vibez.spill("✅ Boolean test passed")
} else {
    fail_count = fail_count + 1
    vibez.spill("❌ Boolean test failed")
}

vibez.spill("")
vibez.spill("📊 Test Summary:")
vibez.spill("Total tests: ", pass_count + fail_count)
vibez.spill("Passed: ", pass_count)
vibez.spill("Failed: ", fail_count)

lowkey fail_count == 0 {
    vibez.spill("🎉 ALL TESTS PASSED!")
} else {
    vibez.spill("💥 SOME TESTS FAILED!")
}
