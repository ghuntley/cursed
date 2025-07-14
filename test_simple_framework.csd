yeet "vibez"

# Test the basic framework functionality without dependencies
vibez.spill("Testing basic framework functionality...")

# Test basic operations
sus result_1 lit = 2 + 2 == 4
sus result_2 lit = "hello" + "world" == "helloworld"
sus result_3 lit = based == based

fr fr result_1 && result_2 && result_3 {
    vibez.spill("✅ Basic framework tests passed!")
} else {
    vibez.spill("❌ Basic framework tests failed!")
}

# Test control flow
sus counter normie = 0
bestie i := 0; i < 5; i++ {
    counter = counter + 1
}

fr fr counter == 5 {
    vibez.spill("✅ Control flow test passed!")
} else {
    vibez.spill("❌ Control flow test failed!")
}

# Test string operations
sus test_string tea = "test"
fr fr test_string == "test" {
    vibez.spill("✅ String test passed!")
} else {
    vibez.spill("❌ String test failed!")
}

vibez.spill("✨ Framework functionality verified!")
