# Test file to verify integer overflow fixes
# This tests various scenarios that might trigger overflow

# Test very large functions with many parameters
slay test_many_params(
    a1 drip, a2 drip, a3 drip, a4 drip, a5 drip, 
    a6 drip, a7 drip, a8 drip, a9 drip, a10 drip,
    a11 drip, a12 drip, a13 drip, a14 drip, a15 drip,
    a16 drip, a17 drip, a18 drip, a19 drip, a20 drip
) drip {
    damn a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 +
         a11 + a12 + a13 + a14 + a15 + a16 + a17 + a18 + a19 + a20
}

# Test nested braces (should handle gracefully)
slay test_deep_nesting() drip {
    ready (based) {
        ready (based) {
            ready (based) {
                ready (based) {
                    ready (based) {
                        vibez.spill("Deep nesting")
                    }
                }
            }
        }
    }
    damn 42
}

# Test large array indexing
sus test_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]

# Test complex arithmetic expressions
sus complex_result drip = 
    (1000 + 2000) * 
    (3000 + 4000) * 
    (5000 + 6000) / 
    (7000 + 8000)

# Test very long string literal
sus long_string tea = "This is an extremely long string that is designed to test the string length validation in the LLVM backend compilation process to ensure that integer overflow does not occur when processing string literals of substantial size during the code generation phase of the compilation pipeline."

vibez.spill("Testing overflow fixes...")
vibez.spill("Many params result:", test_many_params(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20))
vibez.spill("Deep nesting result:", test_deep_nesting())
vibez.spill("Array element:", test_array[5])
vibez.spill("Complex result:", complex_result)
vibez.spill("String length test passed")
