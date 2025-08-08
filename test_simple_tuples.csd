// Test simple tuple creation and access

// This should eventually parse as a tuple literal
vibez.spill("Testing tuple creation...")

// For now, test individual values that could be in a tuple
sus value1 drip = 42
sus value2 tea = "hello"

vibez.spill("Value 1:", value1)
vibez.spill("Value 2:", value2)

// Test function call evaluation  
slay simple_func() {
    vibez.spill("Simple function called")
    damn 123
}

vibez.spill("About to call function...")
sus func_result = simple_func()
vibez.spill("Function result:", func_result)

vibez.spill("Simple tuple tests completed!")
