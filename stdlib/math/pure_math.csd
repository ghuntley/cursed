fr fr Pure CURSED Math Library - No FFI Dependencies
fr fr Demonstrates native CURSED implementations of mathematical functions

fr fr Import testing framework
yeet "testz"

fr fr ================================
fr fr Basic Float Math Functions (Pure CURSED)
fr fr ================================

slay math_abs_pure(x meal) meal {
    lowkey x < 0.0 {
        damn -x
    } highkey {
        damn x
    }
}

slay math_min_pure(a meal, b meal) meal {
    lowkey a < b {
        damn a
    } highkey {
        damn b
    }
}

slay math_max_pure(a meal, b meal) meal {
    lowkey a > b {
        damn a
    } highkey {
        damn b
    }
}

slay math_sign_pure(x meal) normie {
    lowkey x > 0.0 {
        damn 1
    } highkey {
        lowkey x < 0.0 {
            damn -1
        } highkey {
            damn 0
        }
    }
}

fr fr ================================
fr fr Basic Integer Math Functions (Pure CURSED)
fr fr ================================

slay math_abs_int_pure(x normie) normie {
    lowkey x < 0 {
        damn -x
    } highkey {
        damn x
    }
}

slay math_min_int_pure(a normie, b normie) normie {
    lowkey a < b {
        damn a
    } highkey {
        damn b
    }
}

slay math_max_int_pure(a normie, b normie) normie {
    lowkey a > b {
        damn a
    } highkey {
        damn b
    }
}

slay math_sign_int_pure(x normie) normie {
    lowkey x > 0 {
        damn 1
    } highkey {
        lowkey x < 0 {
            damn -1
        } highkey {
            damn 0
        }
    }
}

fr fr ================================
fr fr Advanced Math Functions (Pure CURSED)
fr fr ================================

slay math_sqrt_approx(x meal) meal {
    fr fr Newton's method for square root approximation
    lowkey x < 0.0 {
        damn 0.0  fr fr Invalid input
    }
    
    lowkey x == 0.0 {
        damn 0.0
    }
    
    sus guess meal = x / 2.0
    sus prev_guess meal = 0.0
    sus iterations normie = 0
    
    fr fr Newton's method: guess = (guess + x/guess) / 2
    bestie iterations := 0; iterations < 10; iterations++ {
        prev_guess = guess
        guess = (guess + x / guess) / 2.0
        
        fr fr Check convergence (simple approximation)
        lowkey math_abs_pure(guess - prev_guess) < 0.001 {
            ghosted
        }
    }
    
    damn guess
}

slay math_pow_int_small(base meal, exp normie) meal {
    fr fr Integer power using repeated multiplication
    lowkey exp < 0 {
        damn 1.0 / math_pow_int_small(base, -exp)
    }
    
    lowkey exp == 0 {
        damn 1.0
    }
    
    sus result meal = 1.0
    sus i normie = 0
    
    bestie i := 0; i < exp; i++ {
        result = result * base
    }
    
    damn result
}

slay math_factorial_small(n normie) normie {
    fr fr Factorial using conditional logic
    lowkey n < 0 {
        damn 0  fr fr Invalid input
    }
    
    lowkey n == 0 {
        damn 1
    }
    
    lowkey n == 1 {
        damn 1
    }
    
    fr fr Use lookup table for small values
    lowkey n == 2 {
        damn 2
    }
    
    lowkey n == 3 {
        damn 6
    }
    
    lowkey n == 4 {
        damn 24
    }
    
    lowkey n == 5 {
        damn 120
    }
    
    fr fr For larger values, use iteration
    sus result normie = 1
    sus i normie = 2
    
    bestie i := 2; i <= n; i++ {
        result = result * i
    }
    
    damn result
}

fr fr ================================
fr fr Utility Functions (Pure CURSED)
fr fr ================================

slay math_clamp_pure(x meal, min_val meal, max_val meal) meal {
    lowkey x < min_val {
        damn min_val
    } highkey {
        lowkey x > max_val {
            damn max_val
        } highkey {
            damn x
        }
    }
}

slay math_lerp_pure(a meal, b meal, t meal) meal {
    fr fr Linear interpolation
    damn a + t * (b - a)
}

slay math_is_even_pure(n normie) lit {
    damn (n % 2) == 0
}

slay math_is_odd_pure(n normie) lit {
    damn (n % 2) == 1
}

slay math_gcd_pure(a normie, b normie) normie {
    fr fr Greatest common divisor using Euclidean algorithm
    sus x normie = math_abs_int_pure(a)
    sus y normie = math_abs_int_pure(b)
    
    periodt y > 0 {
        sus temp normie = y
        y = x % y
        x = temp
    }
    
    damn x
}

fr fr ================================
fr fr Test Suite (Pure CURSED Testing)
fr fr ================================

slay test_basic_float_functions() {
    test_start("Basic Float Functions")
    
    fr fr Test absolute value
    assert_eq_int(math_abs_pure(-5.5).(normie), 5)
    assert_eq_int(math_abs_pure(3.2).(normie), 3)
    
    fr fr Test min/max
    assert_eq_int(math_min_pure(3.0, 7.0).(normie), 3)
    assert_eq_int(math_max_pure(3.0, 7.0).(normie), 7)
    
    fr fr Test sign
    assert_eq_int(math_sign_pure(5.0), 1)
    assert_eq_int(math_sign_pure(-3.0), -1)
    assert_eq_int(math_sign_pure(0.0), 0)
}

slay test_basic_int_functions() {
    test_start("Basic Integer Functions")
    
    fr fr Test absolute value
    assert_eq_int(math_abs_int_pure(-42), 42)
    assert_eq_int(math_abs_int_pure(17), 17)
    
    fr fr Test min/max
    assert_eq_int(math_min_int_pure(10, 20), 10)
    assert_eq_int(math_max_int_pure(10, 20), 20)
    
    fr fr Test sign
    assert_eq_int(math_sign_int_pure(15), 1)
    assert_eq_int(math_sign_int_pure(-8), -1)
    assert_eq_int(math_sign_int_pure(0), 0)
}

slay test_advanced_functions() {
    test_start("Advanced Math Functions")
    
    fr fr Test square root (approximate)
    sus sqrt_4 meal = math_sqrt_approx(4.0)
    assert_true(sqrt_4 > 1.9)
    assert_true(sqrt_4 < 2.1)
    
    fr fr Test integer power
    assert_eq_int(math_pow_int_small(2.0, 3).(normie), 8)
    assert_eq_int(math_pow_int_small(5.0, 2).(normie), 25)
    
    fr fr Test factorial
    assert_eq_int(math_factorial_small(5), 120)
    assert_eq_int(math_factorial_small(4), 24)
    assert_eq_int(math_factorial_small(3), 6)
}

slay test_utility_functions() {
    test_start("Utility Functions")
    
    fr fr Test clamp
    assert_eq_int(math_clamp_pure(5.0, 1.0, 10.0).(normie), 5)
    assert_eq_int(math_clamp_pure(-2.0, 1.0, 10.0).(normie), 1)
    assert_eq_int(math_clamp_pure(15.0, 1.0, 10.0).(normie), 10)
    
    fr fr Test even/odd
    assert_true(math_is_even_pure(4))
    assert_false(math_is_even_pure(5))
    assert_true(math_is_odd_pure(7))
    assert_false(math_is_odd_pure(8))
    
    fr fr Test GCD
    assert_eq_int(math_gcd_pure(12, 8), 4)
    assert_eq_int(math_gcd_pure(15, 10), 5)
}

fr fr ================================
fr fr Main Test Execution
fr fr ================================

slay main() {
    vibez.spill("🧮 Pure CURSED Math Library Tests")
    vibez.spill("No FFI dependencies - 100% native CURSED!")
    vibez.spill("")
    
    test_basic_float_functions()
    test_basic_int_functions()
    test_advanced_functions()
    test_utility_functions()
    
    print_test_summary()
}
