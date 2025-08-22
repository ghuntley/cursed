// Memory Management Test
// Test memory allocation/deallocation, arrays, strings

// Test string allocation and manipulation
sus str1 tea = "Hello"
sus str2 tea = "World"
sus combined tea = str1 + " " + str2

spill("Memory Management Tests:")
spill("String concatenation:", combined)

// Test array allocation
sus dynamic_array []drip = []
push(dynamic_array, 1)
push(dynamic_array, 2)
push(dynamic_array, 3)
push(dynamic_array, 4)
push(dynamic_array, 5)

spill("Dynamic array length:", len(dynamic_array))

// Test multiple string operations
bestie (i drip = 0; i < 10; i = i + 1) {
    sus temp_str tea = "iteration_" + str(i)
    spill("Loop iteration:", i, "string:", temp_str)
}

// Test function with local memory allocations
slay memory_intensive_function() drip {
    sus local_strings []tea = []
    bestie (j drip = 0; j < 5; j = j + 1) {
        sus temp tea = "local_" + str(j)
        push(local_strings, temp)
    }
    damn len(local_strings)
}

sus result drip = memory_intensive_function()
spill("Memory intensive function result:", result)
