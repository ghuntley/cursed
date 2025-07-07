// Comprehensive composite literal tests

// Test all supported features
sus arr1 [5]normie = [5]normie{1, 2, 3, 4, 5}
vibez.spill("Full array:")
vibez.spill(arr1[0])
vibez.spill(arr1[4])

sus arr2 [3]normie = [3]normie{10, 20}
vibez.spill("Partial array (zero-filled):")
vibez.spill(arr2[0])  
vibez.spill(arr2[1])  
vibez.spill(arr2[2])  // Should be 0

sus arr3 [2]normie = [2]normie{}
vibez.spill("Empty array (zero-filled):")
vibez.spill(arr3[0])  // Should be 0
vibez.spill(arr3[1])  // Should be 0

sus slice1 []normie = []normie{100, 200, 300}
vibez.spill("Dynamic slice:")
vibez.spill(slice1[0])
vibez.spill(slice1[1])
vibez.spill(slice1[2])

sus slice2 []normie = []normie{}
vibez.spill("Empty slice created successfully")

// Test with different types
sus floats [2]meal = [2]meal{3.14, 2.71}
vibez.spill("Float array:")
vibez.spill(floats[0])
vibez.spill(floats[1])

sus bools [2]lit = [2]lit{based, cap}
vibez.spill("Boolean array:")
vibez.spill(bools[0])
vibez.spill(bools[1])

vibez.spill("All composite literal tests passed!")
