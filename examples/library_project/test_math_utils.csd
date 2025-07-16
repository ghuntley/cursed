yeet "testz"
yeet "lib"

test_start("Math Utils Library Tests")

# Test basic arithmetic
slay test_basic_arithmetic() {
    assert_eq_int(lib.add(5, 3), 8)
    assert_eq_int(lib.multiply(4, 6), 24)
    assert_eq_int(lib.power(2, 3), 8)
    assert_eq_int(lib.power(5, 0), 1)
    
    vibez.spill("✅ Basic arithmetic tests passed")
}

# Test factorial function
slay test_factorial() {
    assert_eq_int(lib.factorial(0), 1)
    assert_eq_int(lib.factorial(1), 1)
    assert_eq_int(lib.factorial(5), 120)
    assert_eq_int(lib.factorial(6), 720)
    
    vibez.spill("✅ Factorial tests passed")
}

# Test fibonacci sequence
slay test_fibonacci() {
    assert_eq_int(lib.fibonacci(0), 0)
    assert_eq_int(lib.fibonacci(1), 1)
    assert_eq_int(lib.fibonacci(5), 5)
    assert_eq_int(lib.fibonacci(8), 21)
    
    vibez.spill("✅ Fibonacci tests passed")
}

# Test prime number detection
slay test_prime_detection() {
    assert_false(lib.is_prime(1))
    assert_true(lib.is_prime(2))
    assert_true(lib.is_prime(7))
    assert_true(lib.is_prime(17))
    assert_false(lib.is_prime(15))
    assert_false(lib.is_prime(25))
    
    vibez.spill("✅ Prime detection tests passed")
}

# Test GCD and LCM
slay test_gcd_lcm() {
    assert_eq_int(lib.gcd(12, 8), 4)
    assert_eq_int(lib.gcd(17, 13), 1)
    assert_eq_int(lib.lcm(4, 6), 12)
    assert_eq_int(lib.lcm(7, 3), 21)
    
    vibez.spill("✅ GCD/LCM tests passed")
}

# Main test runner
slay main() {
    test_basic_arithmetic()
    test_factorial()
    test_fibonacci()
    test_prime_detection()
    test_gcd_lcm()
    
    print_test_summary()
    vibez.spill("🧮 Math Utils Library: All tests passed!")
}
