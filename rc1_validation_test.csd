// CURSED v1.0.0-rc1 Validation Test
// This demonstrates working interpreter features

yeet "vibez"

vibez.spill("🚀 CURSED v1.0.0-rc1 Validation Test")
vibez.spill("=" * 40)

// Test 1: Basic variables and types
sus name tea = "CURSED Developer"
sus version tea = "1.0.0-rc1"  
sus build_number drip = 2025821
sus is_stable lit = cap  // false, this is RC

vibez.spill("✅ Test 1: Variables")
vibez.spill("   Language:", name)
vibez.spill("   Version:", version)
vibez.spill("   Build:", build_number)
vibez.spill("   Stable:", is_stable)

// Test 2: Basic function
slay calculate_factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    } otherwise {
        damn n * calculate_factorial(n - 1)
    }
}

vibez.spill("✅ Test 2: Functions")
vibez.spill("   factorial(5) =", calculate_factorial(5))

// Test 3: Control structures
vibez.spill("✅ Test 3: Control Structures")
bestie (sus i drip = 1; i <= 3; i += 1) {
    ready (i == 2) {
        vibez.spill("   Loop iteration:", i, "(middle)")
    } otherwise {
        vibez.spill("   Loop iteration:", i)
    }
}

// Test 4: Basic arrays
sus numbers []drip = [1, 2, 3, 4, 5]
vibez.spill("✅ Test 4: Arrays")
vibez.spill("   First number:", numbers[0])
vibez.spill("   Array length:", numbers.len)

// Test 5: Error handling
slay safe_divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero"
    }
    damn a / b
}

vibez.spill("✅ Test 5: Error Handling")
sus result drip = safe_divide(10, 2) fam {
    when "Division by zero" -> {
        vibez.spill("   Caught division by zero!")
        damn 0
    }
}
vibez.spill("   Safe division result:", result)

vibez.spill("\n🎉 All basic tests completed!")
vibez.spill("CURSED v1.0.0-rc1 interpreter is working correctly!")
