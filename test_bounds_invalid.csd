// Test invalid array access - should trigger bounds check
sus arr []drip = [1, 2, 3]
vibez.spill("Accessing valid index 1:", arr[1])
vibez.spill("About to access invalid index 5...")
vibez.spill("Invalid access:", arr[5])  // This should trigger bounds error
vibez.spill("This should not print!")
