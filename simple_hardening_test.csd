/// Simple hardening validation test
yeet "vibez"

slay test_basic_array() {
    vibez.spill("Testing basic array operations...")
    
    sus arr []drip = [1, 2, 3]
    sus valid drip = arr[1]
    vibez.spill("Array access successful: ", valid)
}

slay test_function_calls() {
    vibez.spill("Testing function call chains...")
    
    slay inner() drip yikes<tea> {
        damn 42
    }
    
    sus result drip = inner() fam {
        when _ -> damn -1
    }
    
    vibez.spill("Function result: ", result)
}

slay main() {
    vibez.spill("=== Simple Hardening Test ===")
    test_basic_array()
    test_function_calls()
    vibez.spill("=== Test Complete ===")
}
