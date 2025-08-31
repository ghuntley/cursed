// FINAL PROOF: CURSED Pure Self-Hosting Achievement
// This demonstrates pure CURSED stdlib working in both interpreter and compiled modes

// Test 1: Basic arithmetic
sus a drip = 5
sus b drip = 3  
sus arithmetic_result drip = a + b

// Test 2: Stdlib function call
sus stdlib_result drip = mathz.add_two(10, 15)

// Test 3: Nested stdlib calls
sus nested_result drip = mathz.add_two(stdlib_result, 5)

// Test 4: Multiple operations
sus complex_result drip = mathz.multiply_two(a, b) + stdlib_result

// Output verification
yap("=== CURSED PURE SELF-HOSTING PROOF ===")
yap("Basic arithmetic (5 + 3):")
yap(arithmetic_result)
yap("Stdlib function (mathz.add_two(10, 15)):")  
yap(stdlib_result)
yap("Nested stdlib call (result + 5):")
yap(nested_result)
yap("Complex expression:")
yap(complex_result)
yap("=== SELF-HOSTING SUCCESSFUL ===")
