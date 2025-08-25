fr fr Test Enhanced Mathematical Operations
yeet "mathz"

fr fr Test basic mathematical operations
sus a drip = 48
sus b drip = 18
sus gcd_result drip = gcd(a, b)
vibez.spill("Enhanced GCD(48, 18) =", gcd_result)

sus lcm_result drip = lcm(a, b)
vibez.spill("LCM(48, 18) =", lcm_result)

fr fr Test square root
sus sqrt_test drip = 25
sus sqrt_result drip = sqrt_integer(sqrt_test)
vibez.spill("Integer sqrt(25) =", sqrt_result)

fr fr Test statistical functions
sus test_data []drip = [1, 2, 3, 4, 5]
sus mean_val drip = add_two(1, 2)
vibez.spill("Basic addition test:", mean_val)

fr fr Test prime checking
sus prime_test drip = 17
sus is_prime_result lit = is_prime(prime_test)
ready (is_prime_result) {
    vibez.spill("17 is prime: true")
} sus {
    vibez.spill("17 is prime: false")
}

fr fr Test factorial
sus factorial_result drip = factorial(5)
vibez.spill("Factorial(5) =", factorial_result)

vibez.spill("Enhanced mathematical operations test completed!")
