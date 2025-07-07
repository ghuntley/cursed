// Test composite literals with type specification

// Test 1: Fixed-size array with integer type
sus arr1 [3]normie = [3]normie{1, 2, 3}
vibez.spill("Fixed-size array [3]normie{1, 2, 3}:")
vibez.spill(arr1[0])
vibez.spill(arr1[1])
vibez.spill(arr1[2])

// Test 2: Fixed-size array with partial initialization (zero-padding)
sus arr2 [5]normie = [5]normie{10, 20}
vibez.spill("Partial initialization [5]normie{10, 20}:")
vibez.spill(arr2[0])
vibez.spill(arr2[1])
vibez.spill(arr2[2])  // Should be 0
vibez.spill(arr2[3])  // Should be 0
vibez.spill(arr2[4])  // Should be 0

// Test 3: Fixed-size array with empty initialization
sus arr3 [3]normie = [3]normie{}
vibez.spill("Empty initialization [3]normie{}:")
vibez.spill(arr3[0])  // Should be 0
vibez.spill(arr3[1])  // Should be 0
vibez.spill(arr3[2])  // Should be 0

// Test 4: Dynamic slice with integer type
sus slice1 []normie = []normie{100, 200, 300, 400}
vibez.spill("Dynamic slice []normie{100, 200, 300, 400}:")
vibez.spill(slice1[0])
vibez.spill(slice1[1])
vibez.spill(slice1[2])
vibez.spill(slice1[3])

// Test 5: Empty slice
sus slice2 []normie = []normie{}
vibez.spill("Empty slice []normie{}:")
vibez.spill("Slice is empty")

// Test 6: Different types - boolean array
sus bools [2]lit = [2]lit{based, cap}
vibez.spill("Boolean array [2]lit{based, cap}:")
vibez.spill(bools[0])
vibez.spill(bools[1])

// Test 7: Float array
sus floats [3]meal = [3]meal{1.5, 2.5, 3.5}
vibez.spill("Float array [3]meal{1.5, 2.5, 3.5}:")
vibez.spill(floats[0])
vibez.spill(floats[1])
vibez.spill(floats[2])

// Test 8: Character array
sus chars [2]sip = [2]sip{'A', 'B'}
vibez.spill("Character array [2]sip{'A', 'B'}:")
vibez.spill(chars[0])
vibez.spill(chars[1])

vibez.spill("All composite literal tests completed!")
