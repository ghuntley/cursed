# Test 8: Memory management and potential leaks
vibez.spill("Memory management test:")

# Test multiple variable allocations
sus count drip = 0
bestie (count < 100) {
    sus temp_str tea = "String number " + count
    sus temp_num drip = count * 2
    ready (count % 10 == 0) {
        vibez.spill("Processing:", count, temp_str, temp_num)
    }
    count = count + 1
}

# Test function calls with string operations
slay string_concat(a tea, b tea) tea {
    damn a + " " + b
}

sus i drip = 0
bestie (i < 50) {
    sus result tea = string_concat("Hello", "World")
    ready (i % 20 == 0) {
        vibez.spill("String concat result:", result)
    }
    i = i + 1
}

# Test struct allocations
squad TempStruct {
    spill value drip
    spill name tea
}

sus j drip = 0
bestie (j < 30) {
    sus temp_struct TempStruct = TempStruct{value: j, name: "temp"}
    ready (j % 15 == 0) {
        vibez.spill("Struct value:", temp_struct.value, temp_struct.name)
    }
    j = j + 1
}

vibez.spill("Memory test completed")
