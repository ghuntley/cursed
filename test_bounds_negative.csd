// Test negative array index - should trigger bounds check
sus arr []drip = [10, 20, 30]
vibez.spill("Accessing valid index 0:", arr[0])
vibez.spill("About to access negative index...")
vibez.spill("Negative access:", arr[-1])  // This should trigger bounds error
vibez.spill("This should not print!")
