// Error recovery test 42
sus invalid_array []drip = [1, 2, "string", 4]  // Type error
sus index_error drip = invalid_array[100]  // Index error
vibez.spill("Recovery 42: should not reach here")
