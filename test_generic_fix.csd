# Test generic function fix
# This should test the enhanced generic function resolution

slay generic_function<T>(value T) T {
    damn value
}

# Test basic generic function calls
sus result1 drip = generic_function<drip>(100)
sus result2 tea = generic_function<tea>("hello")

yeet "vibez"
vibez.spill("Generic drip result:", result1)
vibez.spill("Generic tea result:", result2)

# Test that the results are correct
ready (result1 == 100) {
    vibez.spill("✅ Generic drip function works!")
} otherwise {
    vibez.spill("❌ Generic drip function failed!")
}

ready (result2 == "hello") {
    vibez.spill("✅ Generic tea function works!")
} otherwise {
    vibez.spill("❌ Generic tea function failed!")
}
