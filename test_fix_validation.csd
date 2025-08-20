# Variable dereferencing validation test
# This test shows the exact issue that was fixed

# Variables for testing
sus x drip = 42
sus y drip = 100
sus name tea = "CURSED"

# The issue: Variables in array literals were not dereferenced
# Before fix: [x, y] would show as [] (failed parsing)
# After fix: [x, y] should show as [42, 100] 

sus test_array []drip = [x, y]
vibez.spill("Array should show [42, 100]:", test_array)

# Complex expressions should also work  
sus expr_array []drip = [x + 10, y - 5]
vibez.spill("Expression array should show [52, 95]:", expr_array)

# Mixed literals and variables
sus mixed_array []drip = [x, 50, y]  
vibez.spill("Mixed array should show [42, 50, 100]:", mixed_array)
