yeet "vibez"

// Test ArrayList memory cleanup in functions
slay test_arrays() drip {
    sus arr1 []drip = [1, 2, 3, 4, 5]
    sus arr2 []drip = [10, 20, 30]
    damn arr1[0] + arr2[1]
}

sus result1 drip = test_arrays()
sus result2 drip = test_arrays() 
sus result3 drip = test_arrays()

vibez.spill("Results:", result1, result2, result3)

// Test multiple function calls with arguments
slay add_nums(a drip, b drip) drip {
    damn a + b
}

sus total drip = add_nums(add_nums(1, 2), add_nums(3, 4))
vibez.spill("Total:", total)

vibez.spill("Memory test completed successfully")
