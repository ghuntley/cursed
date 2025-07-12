// Simple manual test for sketchy_math module
vibez.spill("Testing sketchy_math module")

// Test basic math functions
vibez.spill("abs(-5.0) = " + abs(-5.0))
vibez.spill("abs(5.0) = " + abs(5.0))
vibez.spill("abs_int(-5) = " + abs_int(-5))
vibez.spill("abs_int(5) = " + abs_int(5))

// Test sqrt
vibez.spill("sqrt(4.0) = " + sqrt(4.0))
vibez.spill("sqrt(9.0) = " + sqrt(9.0))
vibez.spill("sqrt(16.0) = " + sqrt(16.0))

// Test pow
vibez.spill("pow(2.0, 3.0) = " + pow(2.0, 3.0))
vibez.spill("pow(5.0, 2.0) = " + pow(5.0, 2.0))

// Test trigonometric functions
vibez.spill("sin(0.0) = " + sin(0.0))
vibez.spill("cos(0.0) = " + cos(0.0))
vibez.spill("tan(0.0) = " + tan(0.0))

// Test rounding
vibez.spill("ceil(3.1) = " + ceil(3.1))
vibez.spill("floor(3.9) = " + floor(3.9))
vibez.spill("round(3.6) = " + round(3.6))

// Test utility functions
vibez.spill("min(5.0, 3.0) = " + min(5.0, 3.0))
vibez.spill("max(5.0, 3.0) = " + max(5.0, 3.0))
vibez.spill("clamp(15.0, 0.0, 10.0) = " + clamp(15.0, 0.0, 10.0))

// Test Gen Z functions
vibez.spill("vibecheck(420.0) = " + vibecheck(420.0))
vibez.spill("super_bussin(420.0) = " + super_bussin(420.0))
vibez.spill("no_cap(5.0) = " + no_cap(5.0))
vibez.spill("yeet_clamp(15.0, 0.0, 10.0) = " + yeet_clamp(15.0, 0.0, 10.0))

// Test random numbers
set_random_seed(42)
vibez.spill("random_float64() = " + random_float64())
vibez.spill("random_float64_range(10.0, 20.0) = " + random_float64_range(10.0, 20.0))
vibez.spill("random_int_range(1, 10) = " + random_int_range(1, 10))

// Test fast approximations
vibez.spill("fast_sqrt(4.0) = " + fast_sqrt(4.0))
vibez.spill("fast_sin(0.0) = " + fast_sin(0.0))
vibez.spill("fast_cos(0.0) = " + fast_cos(0.0))

// Test factorial
vibez.spill("factorial(5) = " + factorial(5))
vibez.spill("factorial(0) = " + factorial(0))

// Test combinations
vibez.spill("combination(5, 2) = " + combination(5, 2))
vibez.spill("permutation(5, 2) = " + permutation(5, 2))

// Test constants
vibez.spill("PI = " + PI)
vibez.spill("E = " + E)
vibez.spill("PHI = " + PHI)
vibez.spill("SQRT2 = " + SQRT2)

vibez.spill("✅ All tests completed successfully!")
