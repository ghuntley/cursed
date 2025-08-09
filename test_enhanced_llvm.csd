// Test enhanced LLVM backend with pattern matching and ARM64 support
sus x drip = 42;
sus y tea = "Hello World";

// Test pattern matching with guards
ready (x) {
    when 0 -> vibez.spill("zero")
    when n ready (n > 0 && n < 10) -> vibez.spill("small positive")
    when n ready (n >= 10 && n < 100) -> vibez.spill("medium positive")
    when _ -> vibez.spill("other")
}

// Test struct for ARM64 calling convention
squad Point {
    spill x drip
    spill y drip
}

// Test function that returns struct ≤16 bytes (should use registers on ARM64)
slay createPoint(x drip, y drip) Point {
    damn Point{ x: x, y: y }
}

// Test function that would use X8 indirect return on ARM64
squad LargeStruct {
    spill field1 drip
    spill field2 drip  
    spill field3 drip
    spill field4 drip
    spill field5 drip  // >16 bytes total
}

slay createLarge() LargeStruct {
    damn LargeStruct{
        field1: 1,
        field2: 2, 
        field3: 3,
        field4: 4,
        field5: 5
    }
}

vibez.spill("Enhanced LLVM backend test complete!")
