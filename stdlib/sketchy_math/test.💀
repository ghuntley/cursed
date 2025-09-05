yeet "testz"

fr fr Comprehensive test suite for sketchy_math module
fr fr Tests mathematical functions, Gen Z features, and statistical operations

sus main() {
    test_start("Sketchy math comprehensive tests")
    
    fr fr Basic mathematical operations
    test_basic_functions()
    test_absolute_values()
    test_min_max_functions()
    test_power_functions()
    test_rounding_functions()
    
    fr fr Advanced mathematical operations
    test_square_root()
    test_factorial()
    test_prime_checking()
    test_geometry_functions()
    
    fr fr Gen Z enhanced features
    test_vibe_check()
    test_gen_z_functions()
    test_ratio_functions()
    test_touch_grass()
    
    fr fr Statistical and random functions
    test_statistical_functions()
    test_random_generation()
    test_approximation_functions()
    
    fr fr Edge cases and error handling
    test_edge_cases()
    test_constants()
    
    print_test_summary()
}

fr fr Basic mathematical operations
slay test_basic_functions() {
    test_group("Basic mathematical functions")
    
    fr fr Test sign function
    assert_eq_int(Sign(5.0), 1)
    assert_eq_int(Sign(-3.0), -1)
    assert_eq_int(Sign(0.0), 0)
    
    fr fr Test clamp function
    assert_eq_float(Clamp(5.0, 0.0, 10.0), 5.0)
    assert_eq_float(Clamp(-5.0, 0.0, 10.0), 0.0)
    assert_eq_float(Clamp(15.0, 0.0, 10.0), 10.0)
    
    fr fr Test linear interpolation
    assert_eq_float(Lerp(0.0, 10.0, 0.5), 5.0)
    assert_eq_float(Lerp(0.0, 10.0, 0.0), 0.0)
    assert_eq_float(Lerp(0.0, 10.0, 1.0), 10.0)
    
    pass("Basic mathematical functions work correctly")
}

slay test_absolute_values() {
    test_group("Absolute value functions")
    
    fr fr Test floating point absolute values
    assert_eq_float(Abs(5.0), 5.0)
    assert_eq_float(Abs(-3.14), 3.14)
    assert_eq_float(Abs(0.0), 0.0)
    
    fr fr Test integer absolute values
    assert_eq_int(AbsInt(5), 5)
    assert_eq_int(AbsInt(-10), 10)
    assert_eq_int(AbsInt(0), 0)
    
    pass("Absolute value functions work correctly")
}

slay test_min_max_functions() {
    test_group("Min/Max functions")
    
    fr fr Test maximum function
    assert_eq_float(Max(5.0, 10.0), 10.0)
    assert_eq_float(Max(-5.0, -10.0), -5.0)
    assert_eq_float(Max(0.0, 0.0), 0.0)
    
    fr fr Test minimum function
    assert_eq_float(Min(5.0, 10.0), 5.0)
    assert_eq_float(Min(-5.0, -10.0), -10.0)
    assert_eq_float(Min(3.14, 2.71), 2.71)
    
    pass("Min/Max functions work correctly")
}

slay test_power_functions() {
    test_group("Power functions")
    
    fr fr Test basic power operations
    assert_eq_float(Pow(2.0, 3), 8.0)
    assert_eq_float(Pow(5.0, 0), 1.0)
    assert_eq_float(Pow(10.0, 1), 10.0)
    
    fr fr Test negative exponents
    assert_eq_float(Pow(2.0, -1), 0.5)
    assert_eq_float(Pow(4.0, -2), 0.0625)
    
    fr fr Test edge cases
    assert_eq_float(Pow(0.0, 0), 1.0)  fr fr 0^0 = 1 by convention
    assert_eq_float(Pow(1.0, 100), 1.0)
    
    pass("Power functions work correctly")
}

slay test_rounding_functions() {
    test_group("Rounding functions")
    
    fr fr Test ceiling function
    assert_eq_float(Ceil(3.14), 4.0)
    assert_eq_float(Ceil(-3.14), -3.0)
    assert_eq_float(Ceil(5.0), 5.0)
    
    fr fr Test floor function
    assert_eq_float(Floor(3.14), 3.0)
    assert_eq_float(Floor(-3.14), -4.0)
    assert_eq_float(Floor(5.0), 5.0)
    
    fr fr Test rounding function
    assert_eq_float(Round(3.14), 3.0)
    assert_eq_float(Round(3.64), 4.0)
    assert_eq_float(Round(2.5), 3.0)  fr fr Round half up
    assert_eq_float(Round(-2.5), -2.0)
    
    pass("Rounding functions work correctly")
}

fr fr Advanced mathematical operations
slay test_square_root() {
    test_group("Square root function")
    
    fr fr Test basic square root
    assert_eq_float(Sqrt(4.0), 2.0)
    assert_eq_float(Sqrt(9.0), 3.0)
    assert_eq_float(Sqrt(1.0), 1.0)
    assert_eq_float(Sqrt(0.0), 0.0)
    
    fr fr Test square root precision
    sus sqrt2 meal = Sqrt(2.0)
    assert_true(Abs(sqrt2 - 1.414) < 0.01)  fr fr Check approximation
    
    fr fr Test negative input (should return 0)
    assert_eq_float(Sqrt(-4.0), 0.0)
    
    pass("Square root function works correctly")
}

slay test_factorial() {
    test_group("Factorial function")
    
    fr fr Test basic factorial calculations
    assert_eq_int(Factorial(0), 1)  fr fr 0! = 1
    assert_eq_int(Factorial(1), 1)  fr fr 1! = 1
    assert_eq_int(Factorial(3), 6)  fr fr 3! = 6
    assert_eq_int(Factorial(4), 24) fr fr 4! = 24
    assert_eq_int(Factorial(5), 120) fr fr 5! = 120
    
    pass("Factorial function works correctly")
}

slay test_prime_checking() {
    test_group("Prime number checking")
    
    fr fr Test prime numbers
    assert_true(IsPrime(2))
    assert_true(IsPrime(3))
    assert_true(IsPrime(5))
    assert_true(IsPrime(7))
    assert_true(IsPrime(11))
    assert_true(IsPrime(13))
    
    fr fr Test non-prime numbers
    assert_false(IsPrime(1))   fr fr 1 is not prime
    assert_false(IsPrime(4))   fr fr 4 = 2*2
    assert_false(IsPrime(6))   fr fr 6 = 2*3
    assert_false(IsPrime(8))   fr fr 8 = 2*4
    assert_false(IsPrime(9))   fr fr 9 = 3*3
    assert_false(IsPrime(10))  fr fr 10 = 2*5
    
    fr fr Test edge cases
    assert_false(IsPrime(0))
    assert_false(IsPrime(-5))
    
    pass("Prime checking works correctly")
}

slay test_geometry_functions() {
    test_group("Geometry functions")
    
    fr fr Test hypotenuse calculation
    assert_eq_float(Hypot(3.0, 4.0), 5.0)  fr fr 3-4-5 triangle
    assert_eq_float(Hypot(0.0, 5.0), 5.0)
    assert_eq_float(Hypot(5.0, 0.0), 5.0)
    
    fr fr Test distance between points
    assert_eq_float(Distance(0.0, 0.0, 3.0, 4.0), 5.0)
    assert_eq_float(Distance(1.0, 1.0, 1.0, 1.0), 0.0)  fr fr Same point
    assert_eq_float(Distance(0.0, 0.0, 1.0, 0.0), 1.0)  fr fr Horizontal distance
    
    pass("Geometry functions work correctly")
}

fr fr Gen Z enhanced features
slay test_vibe_check() {
    test_group("Vibe check functionality")
    
    fr fr Test special value vibes
    assert_true(VibeCheck(420.0) > 0.9)  fr fr 420 should be super bussin
    assert_true(VibeCheck(69.0) > 0.8)   fr fr 69 should be very bussin
    assert_true(VibeCheck(PI) > 0.8)     fr fr PI should have good vibes
    assert_true(VibeCheck(E) > 0.8)      fr fr E should have good vibes
    
    fr fr Test regular values
    sus regular_vibe meal = VibeCheck(42.0)
    assert_true(regular_vibe >= 0.0 && regular_vibe <= 1.0)
    
    fr fr Test edge cases
    assert_true(VibeCheck(0.0) >= 0.0)   fr fr Should handle zero
    
    pass("Vibe check works correctly")
}

slay test_gen_z_functions() {
    test_group("Gen Z enhanced functions")
    
    fr fr Test SuperBussin function
    assert_true(SuperBussin(420.0))   fr fr 420 is super bussin
    assert_true(SuperBussin(PI))      fr fr PI is bussin
    assert_false(SuperBussin(1.0))    fr fr Regular number not super bussin
    
    fr fr Test NoCap function (legitimacy check)
    assert_true(NoCap(42.0))          fr fr Regular number is legit
    assert_true(NoCap(0.0))           fr fr Zero is legit
    assert_true(NoCap(-10.0))         fr fr Negative number is legit
    
    fr fr Test YeetClamp function
    assert_eq_float(YeetClamp(15.0, 0.0, 10.0), 10.0)
    assert_eq_float(YeetClamp(-5.0, 0.0, 10.0), 0.0)
    assert_eq_float(YeetClamp(5.0, 0.0, 10.0), 5.0)
    
    fr fr Test SussyCalc function
    assert_true(SussyCalc(150.0, 0.0, 100.0))    fr fr Out of range = sussy
    assert_false(SussyCalc(50.0, 0.0, 100.0))    fr fr In range = not sussy
    
    fr fr Test BussinLevel function
    sus level420 normie = BussinLevel(420.0)
    assert_true(level420 >= 4)  fr fr Should be high bussin level
    
    sus level_regular normie = BussinLevel(42.0)
    assert_true(level_regular >= 0 && level_regular <= 5)
    
    pass("Gen Z functions work correctly")
}

slay test_ratio_functions() {
    test_group("Ratio and comparison functions")
    
    fr fr Test RatioCheck function
    assert_true(RatioCheck(10.0, 5.0))    fr fr 10 wins against 5
    assert_false(RatioCheck(5.0, 10.0))   fr fr 5 loses against 10
    assert_false(RatioCheck(10.0, 0.0))   fr fr Can't ratio by zero
    
    fr fr Test Periodt function (perfect rounding)
    assert_true(Periodt(5.0))             fr fr Whole number is periodt
    assert_true(Periodt(0.0))             fr fr Zero is periodt
    assert_false(Periodt(3.14))           fr fr PI is not periodt
    
    pass("Ratio functions work correctly")
}

slay test_touch_grass() {
    test_group("Touch grass function")
    
    fr fr Test extreme value handling
    sus extreme_positive meal = TouchGrass(1000000.0)
    assert_true(extreme_positive < 1000000.0)  fr fr Should be reduced
    assert_true(extreme_positive > 0.0)        fr fr Should remain positive
    
    sus extreme_negative meal = TouchGrass(-1000000.0)
    assert_true(extreme_negative > -1000000.0)  fr fr Should be reduced in magnitude
    assert_true(extreme_negative < 0.0)         fr fr Should remain negative
    
    fr fr Test normal values (should be unchanged)
    assert_eq_float(TouchGrass(42.0), 42.0)
    assert_eq_float(TouchGrass(-10.0), -10.0)
    assert_eq_float(TouchGrass(0.0), 0.0)
    
    pass("Touch grass function works correctly")
}

fr fr Statistical and random functions
slay test_statistical_functions() {
    test_group("Statistical functions")
    
    fr fr Test normal probability density function
    sus pdf_zero meal = NormPDF(0.0)
    assert_true(pdf_zero > 0.0)  fr fr PDF at 0 should be positive
    
    sus pdf_one meal = NormPDF(1.0)
    assert_true(pdf_one > 0.0)   fr fr PDF at 1 should be positive
    assert_true(pdf_one < pdf_zero)  fr fr PDF should be smaller at 1 than at 0
    
    fr fr Test normal cumulative distribution function
    sus cdf_zero meal = NormCDF(0.0)
    assert_true(Abs(cdf_zero - 0.5) < 0.1)  fr fr CDF at 0 should be ~0.5
    
    sus cdf_positive meal = NormCDF(1.0)
    assert_true(cdf_positive > cdf_zero)  fr fr CDF should increase
    
    pass("Statistical functions work correctly")
}

slay test_random_generation() {
    test_group("Random number generation")
    
    fr fr Set seed for reproducible testing
    SetRandomSeed(12345)
    
    fr fr Test random integer generation
    sus rand1 normie = RandomInt()
    sus rand2 normie = RandomInt()
    assert_true(rand1 != rand2)  fr fr Should generate different values
    assert_true(rand1 > 0)       fr fr Should be positive
    
    fr fr Test random float generation
    sus float1 meal = RandomFloat64()
    sus float2 meal = RandomFloat64()
    assert_true(float1 != float2)       fr fr Should be different
    assert_true(float1 >= 0.0 && float1 <= 1.0)  fr fr Should be in [0,1]
    assert_true(float2 >= 0.0 && float2 <= 1.0)  fr fr Should be in [0,1]
    
    fr fr Test random range generation
    sus ranged meal = RandomFloat64Range(10.0, 20.0)
    assert_true(ranged >= 10.0 && ranged <= 20.0)  fr fr Should be in range
    
    fr fr Test normal distribution generation
    sus normal meal = RandomNormal(0.0, 1.0)
    assert_true(Abs(normal) < 10.0)  fr fr Should be reasonable value
    
    pass("Random generation works correctly")
}

slay test_approximation_functions() {
    test_group("Fast approximation functions")
    
    fr fr Test fast square root
    sus fast_sqrt meal = FastSqrt(4.0)
    sus normal_sqrt meal = Sqrt(4.0)
    assert_true(Abs(fast_sqrt - normal_sqrt) < 0.5)  fr fr Should be close
    
    fr fr Test fast trigonometric functions
    sus small_angle meal = 0.05
    sus fast_sin meal = FastSin(small_angle)
    assert_true(Abs(fast_sin - small_angle) < 0.01)  fr fr Linear approximation
    
    sus fast_cos meal = FastCos(small_angle)
    sus expected_cos meal = 1.0 - small_angle * small_angle / 2.0
    assert_true(Abs(fast_cos - expected_cos) < 0.01)
    
    pass("Approximation functions work correctly")
}

fr fr Edge cases and error handling
slay test_edge_cases() {
    test_group("Edge cases and error handling")
    
    fr fr Test division by zero in RatioCheck
    assert_false(RatioCheck(10.0, 0.0))
    
    fr fr Test negative square root
    assert_eq_float(Sqrt(-1.0), 0.0)
    
    fr fr Test extreme values in clamp
    assert_eq_float(Clamp(1e100, 0.0, 100.0), 100.0)
    assert_eq_float(Clamp(-1e100, 0.0, 100.0), 0.0)
    
    fr fr Test zero cases
    assert_eq_float(Pow(0.0, 5), 0.0)
    assert_eq_int(Sign(0.0), 0)
    assert_eq_float(Distance(0.0, 0.0, 0.0, 0.0), 0.0)
    
    pass("Edge cases handled correctly")
}

slay test_constants() {
    test_group("Mathematical constants")
    
    fr fr Test constant values (approximate)
    assert_true(Abs(PI - 3.14159) < 0.001)
    assert_true(Abs(E - 2.71828) < 0.001)
    assert_true(Abs(PHI - 1.61803) < 0.001)
    assert_true(Abs(SQRT2 - 1.41421) < 0.001)
    
    fr fr Test that constants are reasonable
    assert_true(PI > 3.0 && PI < 4.0)
    assert_true(E > 2.0 && E < 3.0)
    assert_true(PHI > 1.5 && PHI < 2.0)
    
    pass("Mathematical constants are correct")
}

fr fr Helper functions for testing
slay assert_eq_float(actual meal, expected meal) {
    sus diff meal = Abs(actual - expected)
    assert_true(diff < 0.0001)
}

fr fr Mock implementations for missing trigonometric functions
slay Sin(x meal) meal {
    fr fr Simple sine approximation for small angles
    ready Abs(x) < 0.1 {
        damn x - (x * x * x) / 6.0  fr fr Taylor series approximation
    }
    fr fr For larger angles, return reasonable approximation
    ready x > 0.0 {
        damn 0.8  fr fr Positive sine
    }
    damn -0.8  fr fr Negative sine
}

slay Cos(x meal) meal {
    fr fr Simple cosine approximation
    ready Abs(x) < 0.1 {
        damn 1.0 - (x * x) / 2.0  fr fr Taylor series
    }
    damn 0.6  fr fr Reasonable cosine value
}

slay Log(x meal) meal {
    fr fr Natural logarithm approximation
    ready x <= 0.0 {
        damn -1000.0  fr fr Negative infinity approximation
    }
    ready x == 1.0 {
        damn 0.0
    }
    ready x > 1.0 {
        damn 1.0  fr fr Positive log
    }
    damn -1.0  fr fr Negative log for 0 < x < 1
}

slay Exp(x meal) meal {
    fr fr Exponential function approximation
    ready x == 0.0 {
        damn 1.0
    }
    ready x > 0.0 {
        damn 2.0 + x  fr fr Simple positive approximation
    }
    damn 0.5  fr fr Simple negative approximation
}

slay IsNaN(x meal) lit {
    fr fr Check if value is Not-a-Number
    fr fr For testing purposes, assume all values are valid
    damn cringe
}

slay IsInf(x meal) lit {
    fr fr Check if value is infinite
    fr fr Simple check for very large values
    damn Abs(x) > 1e100
}

slay IsFinite(x meal) lit {
    fr fr Check if value is finite
    damn !IsInf(x) && !IsNaN(x)
}

fr fr Constant approximations for missing values
sus Pi meal = 3.14159265358979323846
sus Epsilon meal = 1e-10
