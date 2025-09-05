// Comprehensive Tests for Pure CURSED Math Integer Module
yeet "testz"
yeet "math_int"

// ================================
// Test Basic Integer Operations
// ================================

slay test_basic_operations() {
    test_start("Basic Integer Operations");
    
    // Test abs_int
    assert_eq_int(abs_int(-5), 5);
    assert_eq_int(abs_int(5), 5);
    assert_eq_int(abs_int(0), 0);
    assert_eq_int(abs_int(-2147483647), 2147483647);
    
    // Test sign_int
    assert_eq_int(sign_int(5), 1);
    assert_eq_int(sign_int(-5), -1);
    assert_eq_int(sign_int(0), 0);
    
    // Test min_int/max_int
    assert_eq_int(min_int(3, 7), 3);
    assert_eq_int(max_int(3, 7), 7);
    assert_eq_int(min_int(-5, -2), -5);
    assert_eq_int(max_int(-5, -2), -2);
    
    // Test clamp_int
    assert_eq_int(clamp_int(5, 1, 10), 5);
    assert_eq_int(clamp_int(-5, 1, 10), 1);
    assert_eq_int(clamp_int(15, 1, 10), 10);
}

// ================================
// Test Integer Power Functions
// ================================

slay test_power_functions() {
    test_start("Integer Power Functions");
    
    // Test pow_int
    assert_eq_int(pow_int(2, 0), 1);
    assert_eq_int(pow_int(2, 1), 2);
    assert_eq_int(pow_int(2, 3), 8);
    assert_eq_int(pow_int(2, 10), 1024);
    assert_eq_int(pow_int(5, 3), 125);
    assert_eq_int(pow_int(-2, 3), -8);
    assert_eq_int(pow_int(-2, 4), 16);
    
    // Test negative exponents
    assert_eq_int(pow_int(2, -1), 0);  // Integer division truncates
    assert_eq_int(pow_int(2, -2), 0);
    
    // Test sqrt_int
    assert_eq_int(sqrt_int(0), 0);
    assert_eq_int(sqrt_int(1), 1);
    assert_eq_int(sqrt_int(4), 2);
    assert_eq_int(sqrt_int(9), 3);
    assert_eq_int(sqrt_int(16), 4);
    assert_eq_int(sqrt_int(25), 5);
    assert_eq_int(sqrt_int(15), 3);  // Floor of sqrt(15)
    assert_eq_int(sqrt_int(24), 4);  // Floor of sqrt(24)
    assert_eq_int(sqrt_int(-1), -1); // Error case
}

// ================================
// Test GCD and LCM
// ================================

slay test_gcd_lcm() {
    test_start("GCD and LCM");
    
    // Test gcd
    assert_eq_int(gcd(12, 18), 6);
    assert_eq_int(gcd(48, 18), 6);
    assert_eq_int(gcd(17, 13), 1);
    assert_eq_int(gcd(0, 5), 5);
    assert_eq_int(gcd(5, 0), 5);
    assert_eq_int(gcd(-12, 18), 6);
    assert_eq_int(gcd(12, -18), 6);
    
    // Test lcm
    assert_eq_int(lcm(12, 18), 36);
    assert_eq_int(lcm(4, 6), 12);
    assert_eq_int(lcm(17, 13), 221);
    assert_eq_int(lcm(0, 5), 0);
    assert_eq_int(lcm(5, 0), 0);
    
    // Test extended GCD
    sus gcd_result [normie] = gcd_extended(48, 18);
    assert_eq_int(gcd_result[0], 6);  // GCD
    // 48 * x + 18 * y = 6, so 48 * gcd_result[1] + 18 * gcd_result[2] = 6
    assert_eq_int(48 * gcd_result[1] + 18 * gcd_result[2], 6);
}

// ================================
// Test Factorial and Combinatorics
// ================================

slay test_factorial_combinatorics() {
    test_start("Factorial and Combinatorics");
    
    // Test factorial
    assert_eq_int(factorial(0), 1);
    assert_eq_int(factorial(1), 1);
    assert_eq_int(factorial(2), 2);
    assert_eq_int(factorial(3), 6);
    assert_eq_int(factorial(4), 24);
    assert_eq_int(factorial(5), 120);
    assert_eq_int(factorial(6), 720);
    assert_eq_int(factorial(-1), 0);  // Error case
    
    // Test combination
    assert_eq_int(combination(5, 0), 1);
    assert_eq_int(combination(5, 1), 5);
    assert_eq_int(combination(5, 2), 10);
    assert_eq_int(combination(5, 3), 10);
    assert_eq_int(combination(5, 4), 5);
    assert_eq_int(combination(5, 5), 1);
    assert_eq_int(combination(10, 3), 120);
    assert_eq_int(combination(5, 6), 0);  // Error case
    
    // Test permutation
    assert_eq_int(permutation(5, 0), 1);
    assert_eq_int(permutation(5, 1), 5);
    assert_eq_int(permutation(5, 2), 20);
    assert_eq_int(permutation(5, 3), 60);
    assert_eq_int(permutation(5, 5), 120);
    assert_eq_int(permutation(5, 6), 0);  // Error case
}

// ================================
// Test Fibonacci Sequence
// ================================

slay test_fibonacci() {
    test_start("Fibonacci Sequence");
    
    // Test fibonacci
    assert_eq_int(fibonacci(0), 0);
    assert_eq_int(fibonacci(1), 1);
    assert_eq_int(fibonacci(2), 1);
    assert_eq_int(fibonacci(3), 2);
    assert_eq_int(fibonacci(4), 3);
    assert_eq_int(fibonacci(5), 5);
    assert_eq_int(fibonacci(6), 8);
    assert_eq_int(fibonacci(7), 13);
    assert_eq_int(fibonacci(8), 21);
    assert_eq_int(fibonacci(9), 34);
    assert_eq_int(fibonacci(10), 55);
    assert_eq_int(fibonacci(-1), 0);  // Error case
    
    // Test larger fibonacci numbers
    assert_eq_int(fibonacci(15), 610);
    assert_eq_int(fibonacci(20), 6765);
}

// ================================
// Test Prime Numbers
// ================================

slay test_prime_numbers() {
    test_start("Prime Numbers");
    
    // Test is_prime
    assert_false(is_prime(0));
    assert_false(is_prime(1));
    assert_true(is_prime(2));
    assert_true(is_prime(3));
    assert_false(is_prime(4));
    assert_true(is_prime(5));
    assert_false(is_prime(6));
    assert_true(is_prime(7));
    assert_false(is_prime(8));
    assert_false(is_prime(9));
    assert_false(is_prime(10));
    assert_true(is_prime(11));
    assert_true(is_prime(13));
    assert_true(is_prime(17));
    assert_true(is_prime(19));
    assert_true(is_prime(23));
    assert_false(is_prime(25));
    assert_false(is_prime(27));
    assert_true(is_prime(29));
    assert_true(is_prime(31));
    
    // Test next_prime
    assert_eq_int(next_prime(1), 2);
    assert_eq_int(next_prime(2), 3);
    assert_eq_int(next_prime(3), 5);
    assert_eq_int(next_prime(10), 11);
    assert_eq_int(next_prime(13), 17);
    assert_eq_int(next_prime(20), 23);
    
    // Test prime_factors
    sus factors2 [normie] = prime_factors(2);
    assert_eq_int(factors2.length, 1);
    assert_eq_int(factors2[0], 2);
    
    sus factors12 [normie] = prime_factors(12);
    assert_eq_int(factors12.length, 3);
    assert_eq_int(factors12[0], 2);
    assert_eq_int(factors12[1], 2);
    assert_eq_int(factors12[2], 3);
    
    sus factors30 [normie] = prime_factors(30);
    assert_eq_int(factors30.length, 3);
    assert_eq_int(factors30[0], 2);
    assert_eq_int(factors30[1], 3);
    assert_eq_int(factors30[2], 5);
}

// ================================
// Test Modular Arithmetic
// ================================

slay test_modular_arithmetic() {
    test_start("Modular Arithmetic");
    
    // Test mod_add
    assert_eq_int(mod_add(5, 3, 7), 1);
    assert_eq_int(mod_add(10, 15, 12), 1);
    
    // Test mod_sub
    assert_eq_int(mod_sub(5, 3, 7), 2);
    assert_eq_int(mod_sub(3, 5, 7), 5);
    
    // Test mod_mul
    assert_eq_int(mod_mul(5, 3, 7), 1);
    assert_eq_int(mod_mul(6, 7, 11), 9);
    
    // Test mod_pow
    assert_eq_int(mod_pow(2, 3, 5), 3);
    assert_eq_int(mod_pow(3, 4, 7), 4);
    assert_eq_int(mod_pow(5, 0, 13), 1);
    
    // Test mod_inverse
    assert_eq_int(mod_inverse(3, 7), 5);  // 3 * 5 = 15 ≡ 1 (mod 7)
    assert_eq_int(mod_inverse(5, 7), 3);  // 5 * 3 = 15 ≡ 1 (mod 7)
    assert_eq_int(mod_inverse(2, 6), -1); // No inverse exists
}

// ================================
// Test Number Theory
// ================================

slay test_number_theory() {
    test_start("Number Theory");
    
    // Test euler_totient
    assert_eq_int(euler_totient(1), 1);
    assert_eq_int(euler_totient(2), 1);
    assert_eq_int(euler_totient(3), 2);
    assert_eq_int(euler_totient(4), 2);
    assert_eq_int(euler_totient(5), 4);
    assert_eq_int(euler_totient(6), 2);
    assert_eq_int(euler_totient(7), 6);
    assert_eq_int(euler_totient(8), 4);
    assert_eq_int(euler_totient(9), 6);
    assert_eq_int(euler_totient(10), 4);
    
    // Test chinese_remainder
    sus remainders [normie] = [2, 3, 2];
    sus moduli [normie] = [3, 5, 7];
    sus result normie = chinese_remainder(remainders, moduli);
    assert_eq_int(result % 3, 2);
    assert_eq_int(result % 5, 3);
    assert_eq_int(result % 7, 2);
}

// ================================
// Test Bitwise Operations
// ================================

slay test_bitwise_operations() {
    test_start("Bitwise Operations");
    
    // Test popcount
    assert_eq_int(popcount(0), 0);
    assert_eq_int(popcount(1), 1);
    assert_eq_int(popcount(3), 2);  // 11 in binary
    assert_eq_int(popcount(7), 3);  // 111 in binary
    assert_eq_int(popcount(15), 4); // 1111 in binary
    
    // Test leading_zeros
    assert_eq_int(leading_zeros(0), 32);
    assert_eq_int(leading_zeros(1), 31);
    assert_eq_int(leading_zeros(2), 30);
    assert_eq_int(leading_zeros(4), 29);
    
    // Test trailing_zeros
    assert_eq_int(trailing_zeros(0), 32);
    assert_eq_int(trailing_zeros(1), 0);
    assert_eq_int(trailing_zeros(2), 1);
    assert_eq_int(trailing_zeros(4), 2);
    assert_eq_int(trailing_zeros(8), 3);
    
    // Test reverse_bits
    assert_eq_int(reverse_bits(0), 0);
    assert_eq_int(reverse_bits(1), -2147483648);  // 1 << 31
    assert_eq_int(reverse_bits(-2147483648), 1);  // Reverse of above
}

// ================================
// Test Digital Operations
// ================================

slay test_digital_operations() {
    test_start("Digital Operations");
    
    // Test digital_root
    assert_eq_int(digital_root(0), 0);
    assert_eq_int(digital_root(9), 9);
    assert_eq_int(digital_root(10), 1);
    assert_eq_int(digital_root(11), 2);
    assert_eq_int(digital_root(38), 2);  // 3 + 8 = 11, 1 + 1 = 2
    assert_eq_int(digital_root(123), 6); // 1 + 2 + 3 = 6
    
    // Test digit_sum
    assert_eq_int(digit_sum(0), 0);
    assert_eq_int(digit_sum(123), 6);
    assert_eq_int(digit_sum(456), 15);
    assert_eq_int(digit_sum(999), 27);
    
    // Test digit_product
    assert_eq_int(digit_product(0), 0);
    assert_eq_int(digit_product(123), 6);
    assert_eq_int(digit_product(456), 120);
    assert_eq_int(digit_product(505), 0);  // Contains 0
}

// ================================
// Test Perfect Numbers
// ================================

slay test_perfect_numbers() {
    test_start("Perfect Numbers");
    
    // Test is_perfect
    assert_false(is_perfect(1));
    assert_false(is_perfect(2));
    assert_false(is_perfect(5));
    assert_true(is_perfect(6));   // 1 + 2 + 3 = 6
    assert_false(is_perfect(10));
    assert_true(is_perfect(28));  // 1 + 2 + 4 + 7 + 14 = 28
    
    // Test sum_proper_divisors
    assert_eq_int(sum_proper_divisors(6), 6);   // 1 + 2 + 3
    assert_eq_int(sum_proper_divisors(12), 16); // 1 + 2 + 3 + 4 + 6
    assert_eq_int(sum_proper_divisors(28), 28); // 1 + 2 + 4 + 7 + 14
    
    // Test is_abundant
    assert_false(is_abundant(6));
    assert_true(is_abundant(12));  // 1 + 2 + 3 + 4 + 6 = 16 > 12
    assert_true(is_abundant(18));  // 1 + 2 + 3 + 6 + 9 = 21 > 18
    
    // Test is_deficient
    assert_true(is_deficient(1));
    assert_true(is_deficient(2));
    assert_true(is_deficient(4));
    assert_true(is_deficient(8));
    assert_false(is_deficient(6));
    assert_false(is_deficient(12));
}

// ================================
// Test Collatz Conjecture
// ================================

slay test_collatz() {
    test_start("Collatz Conjecture");
    
    // Test collatz_length
    assert_eq_int(collatz_length(1), 0);
    assert_eq_int(collatz_length(2), 1);  // 2 -> 1
    assert_eq_int(collatz_length(3), 7);  // 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
    assert_eq_int(collatz_length(4), 2);  // 4 -> 2 -> 1
    assert_eq_int(collatz_length(5), 5);  // 5 -> 16 -> 8 -> 4 -> 2 -> 1
    assert_eq_int(collatz_length(6), 8);  // 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
    assert_eq_int(collatz_length(7), 16); // Long sequence
    assert_eq_int(collatz_length(0), 0);  // Edge case
    assert_eq_int(collatz_length(-1), 0); // Edge case
}

// ================================
// Test Base Conversion
// ================================

slay test_base_conversion() {
    test_start("Base Conversion");
    
    // Test to_base
    assert_eq_string(to_base(0, 2), "0");
    assert_eq_string(to_base(5, 2), "101");
    assert_eq_string(to_base(10, 2), "1010");
    assert_eq_string(to_base(255, 2), "11111111");
    assert_eq_string(to_base(10, 10), "10");
    assert_eq_string(to_base(255, 16), "FF");
    assert_eq_string(to_base(26, 26), "10");
    
    // Test from_base
    assert_eq_int(from_base("0", 2), 0);
    assert_eq_int(from_base("101", 2), 5);
    assert_eq_int(from_base("1010", 2), 10);
    assert_eq_int(from_base("11111111", 2), 255);
    assert_eq_int(from_base("10", 10), 10);
    assert_eq_int(from_base("FF", 16), 255);
    assert_eq_int(from_base("ff", 16), 255);  // Lowercase
    
    // Test round-trip conversion
    assert_eq_int(from_base(to_base(42, 2), 2), 42);
    assert_eq_int(from_base(to_base(100, 8), 8), 100);
    assert_eq_int(from_base(to_base(255, 16), 16), 255);
}

// ================================
// Test Edge Cases and Large Numbers
// ================================

slay test_edge_cases() {
    test_start("Edge Cases and Large Numbers");
    
    // Test large factorial (within int range)
    assert_eq_int(factorial(10), 3628800);
    assert_eq_int(factorial(12), 479001600);
    
    // Test large fibonacci
    assert_eq_int(fibonacci(25), 75025);
    assert_eq_int(fibonacci(30), 832040);
    
    // Test large powers
    assert_eq_int(pow_int(2, 20), 1048576);
    assert_eq_int(pow_int(3, 10), 59049);
    
    // Test large GCD
    assert_eq_int(gcd(1071, 462), 21);
    assert_eq_int(gcd(123456, 789012), 12);
    
    // Test edge cases for sqrt_int
    assert_eq_int(sqrt_int(1000000), 1000);
    assert_eq_int(sqrt_int(999999), 999);
    assert_eq_int(sqrt_int(1000001), 1000);
}

// ================================
// Run All Tests
// ================================

slay main_character() {
    test_basic_operations();
    test_power_functions();
    test_gcd_lcm();
    test_factorial_combinatorics();
    test_fibonacci();
    test_prime_numbers();
    test_modular_arithmetic();
    test_number_theory();
    test_bitwise_operations();
    test_digital_operations();
    test_perfect_numbers();
    test_collatz();
    test_base_conversion();
    test_edge_cases();
    
    print_test_summary();
}

main();
