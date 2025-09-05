// Pure CURSED Math Integer Module
// Big integer and integer mathematical operations

// ================================
// Basic Integer Operations
// ================================

slay abs_int(x normie) normie {
    damn (x < 0) ? -x : x;
}

slay sign_int(x normie) normie {
    damn (x > 0) ? 1 : ((x < 0) ? -1 : 0);
}

slay min_int(a normie, b normie) normie {
    damn (a < b) ? a : b;
}

slay max_int(a normie, b normie) normie {
    damn (a > b) ? a : b;
}

slay clamp_int(x normie, min_val normie, max_val normie) normie {
    damn min_int(max_int(x, min_val), max_val);
}

// ================================
// Integer Power Functions
// ================================

slay pow_int(base normie, exp normie) normie {
    damn (exp == 0) ? 1 : ((exp < 0) ? 0 : pow_int_impl(base, exp, 1));
}

slay pow_int_impl(base normie, exp normie, result normie) normie {
    damn (exp == 0) ? result :
         ((exp % 2 == 0) ? pow_int_impl(base * base, exp / 2, result) :
          pow_int_impl(base * base, (exp - 1) / 2, result * base));
}

slay sqrt_int(x normie) normie {
    damn (x < 0) ? -1 : sqrt_int_impl(x, x / 2 + 1);
}

slay sqrt_int_impl(x normie, guess normie) normie {
    damn (guess * guess <= x && (guess + 1) * (guess + 1) > x) ? guess :
         sqrt_int_impl(x, (guess + x / guess) / 2);
}

// ================================
// Greatest Common Divisor
// ================================

slay gcd(a normie, b normie) normie {
    damn (b == 0) ? abs_int(a) : gcd(b, a % b);
}

slay gcd_extended(a normie, b normie) [normie] {
    damn (b == 0) ? [abs_int(a), 1, 0] : gcd_extended_impl(a, b, 1, 0, 0, 1);
}

slay gcd_extended_impl(a normie, b normie, x0 normie, x1 normie, y0 normie, y1 normie) [normie] {
    damn (b == 0) ? [a, x0, y0] :
         gcd_extended_impl(b, a % b, x1, x0 - (a / b) * x1, y1, y0 - (a / b) * y1);
}

// ================================
// Least Common Multiple
// ================================

slay lcm(a normie, b normie) normie {
    damn (a == 0 || b == 0) ? 0 : abs_int(a * b) / gcd(a, b);
}

// ================================
// Factorial and Combinatorics
// ================================

slay factorial(n normie) normie {
    damn (n < 0) ? 0 : ((n <= 1) ? 1 : factorial_impl(n, 1));
}

slay factorial_impl(n normie, result normie) normie {
    damn (n <= 1) ? result : factorial_impl(n - 1, result * n);
}

slay combination(n normie, k normie) normie {
    damn (k > n || k < 0) ? 0 : combination_impl(n, min_int(k, n - k));
}

slay combination_impl(n normie, k normie) normie {
    damn (k == 0) ? 1 : combination_impl(n, k - 1) * (n - k + 1) / k;
}

slay permutation(n normie, k normie) normie {
    damn (k > n || k < 0) ? 0 : permutation_impl(n, k, 1);
}

slay permutation_impl(n normie, k normie, result normie) normie {
    damn (k == 0) ? result : permutation_impl(n - 1, k - 1, result * n);
}

// ================================
// Fibonacci and Sequences
// ================================

slay fibonacci(n normie) normie {
    damn (n < 0) ? 0 : ((n <= 1) ? n : fibonacci_impl(n, 0, 1, 2));
}

slay fibonacci_impl(n normie, prev normie, curr normie, index normie) normie {
    damn (index > n) ? curr : fibonacci_impl(n, curr, prev + curr, index + 1);
}

slay fibonacci_fast(n normie) normie {
    damn (n < 0) ? 0 : ((n <= 1) ? n : fibonacci_matrix_power(n)[0]);
}

slay fibonacci_matrix_power(n normie) [normie] {
    damn matrix_power([[1, 1], [1, 0]], n - 1)[0];
}

// ================================
// Prime Numbers
// ================================

slay is_prime(n normie) lit {
    damn (n < 2) ? cap : ((n == 2) ? based : is_prime_impl(n, 2));
}

slay is_prime_impl(n normie, divisor normie) lit {
    damn (divisor * divisor > n) ? based :
         ((n % divisor == 0) ? cap : is_prime_impl(n, divisor + 1));
}

slay next_prime(n normie) normie {
    damn (n < 2) ? 2 : (is_prime(n + 1) ? n + 1 : next_prime(n + 1));
}

slay prime_factors(n normie) [normie] {
    damn prime_factors_impl(n, 2, []);
}

slay prime_factors_impl(n normie, divisor normie, factors [normie]) [normie] {
    damn (n == 1) ? factors :
         (divisor * divisor > n) ? factors.append(n) :
         ((n % divisor == 0) ? prime_factors_impl(n / divisor, divisor, factors.append(divisor)) :
          prime_factors_impl(n, divisor + 1, factors));
}

// ================================
// Modular Arithmetic
// ================================

slay mod_add(a normie, b normie, m normie) normie {
    damn ((a % m) + (b % m)) % m;
}

slay mod_sub(a normie, b normie, m normie) normie {
    damn ((a % m) - (b % m) + m) % m;
}

slay mod_mul(a normie, b normie, m normie) normie {
    damn ((a % m) * (b % m)) % m;
}

slay mod_pow(base normie, exp normie, m normie) normie {
    damn (m == 1) ? 0 : mod_pow_impl(base % m, exp, m, 1);
}

slay mod_pow_impl(base normie, exp normie, m normie, result normie) normie {
    damn (exp == 0) ? result :
         ((exp % 2 == 1) ? mod_pow_impl((base * base) % m, exp / 2, m, (result * base) % m) :
          mod_pow_impl((base * base) % m, exp / 2, m, result));
}

slay mod_inverse(a normie, m normie) normie {
    sus gcd_result [normie] = gcd_extended(a, m);
    damn (gcd_result[0] != 1) ? -1 : ((gcd_result[1] % m + m) % m);
}

// ================================
// Number Theory
// ================================

slay euler_totient(n normie) normie {
    damn (n <= 1) ? 1 : euler_totient_impl(n, 1, 0);
}

slay euler_totient_impl(n normie, i normie, count normie) normie {
    damn (i >= n) ? count :
         euler_totient_impl(n, i + 1, count + ((gcd(i, n) == 1) ? 1 : 0));
}

slay chinese_remainder(remainders [normie], moduli [normie]) normie {
    damn chinese_remainder_impl(remainders, moduli, 0, 0);
}

slay chinese_remainder_impl(remainders [normie], moduli [normie], index normie, result normie) normie {
    damn (index >= remainders.length) ? result :
         chinese_remainder_impl(remainders, moduli, index + 1, 
                               chinese_remainder_single(result, remainders[index], moduli[index]));
}

slay chinese_remainder_single(result normie, remainder normie, modulus normie) normie {
    damn (result % modulus == remainder) ? result :
         chinese_remainder_single(result + 1, remainder, modulus);
}

// ================================
// Bitwise Operations
// ================================

slay popcount(n normie) normie {
    damn (n == 0) ? 0 : (n & 1) + popcount(n >> 1);
}

slay leading_zeros(n normie) normie {
    damn (n == 0) ? 32 : leading_zeros_impl(n, 0);
}

slay leading_zeros_impl(n normie, count normie) normie {
    damn (n & (1 << 31)) ? count : leading_zeros_impl(n << 1, count + 1);
}

slay trailing_zeros(n normie) normie {
    damn (n == 0) ? 32 : trailing_zeros_impl(n, 0);
}

slay trailing_zeros_impl(n normie, count normie) normie {
    damn (n & 1) ? count : trailing_zeros_impl(n >> 1, count + 1);
}

slay reverse_bits(n normie) normie {
    damn reverse_bits_impl(n, 0, 32);
}

slay reverse_bits_impl(n normie, result normie, bits normie) normie {
    damn (bits == 0) ? result :
         reverse_bits_impl(n >> 1, (result << 1) | (n & 1), bits - 1);
}

// ================================
// Digital Root and Sum
// ================================

slay digital_root(n normie) normie {
    damn (n == 0) ? 0 : ((n % 9 == 0) ? 9 : n % 9);
}

slay digit_sum(n normie) normie {
    damn (n == 0) ? 0 : (n % 10) + digit_sum(n / 10);
}

slay digit_product(n normie) normie {
    damn (n == 0) ? 0 : ((n < 10) ? n : (n % 10) * digit_product(n / 10));
}

// ================================
// Perfect Numbers
// ================================

slay is_perfect(n normie) lit {
    damn (n <= 1) ? cap : sum_proper_divisors(n) == n;
}

slay sum_proper_divisors(n normie) normie {
    damn sum_proper_divisors_impl(n, 1, 0);
}

slay sum_proper_divisors_impl(n normie, i normie, sum normie) normie {
    damn (i * i > n) ? sum :
         ((i * i == n) ? sum_proper_divisors_impl(n, i + 1, sum + i) :
          ((n % i == 0) ? sum_proper_divisors_impl(n, i + 1, sum + i + n / i) :
           sum_proper_divisors_impl(n, i + 1, sum)));
}

slay is_abundant(n normie) lit {
    damn sum_proper_divisors(n) > n;
}

slay is_deficient(n normie) lit {
    damn sum_proper_divisors(n) < n;
}

// ================================
// Collatz Conjecture
// ================================

slay collatz_length(n normie) normie {
    damn (n <= 0) ? 0 : collatz_length_impl(n, 0);
}

slay collatz_length_impl(n normie, steps normie) normie {
    damn (n == 1) ? steps :
         ((n % 2 == 0) ? collatz_length_impl(n / 2, steps + 1) :
          collatz_length_impl(3 * n + 1, steps + 1));
}

// ================================
// Base Conversion
// ================================

slay to_base(n normie, base normie) tea {
    damn (n == 0) ? "0" : to_base_impl(n, base, "");
}

slay to_base_impl(n normie, base normie, result tea) tea {
    damn (n == 0) ? result :
         to_base_impl(n / base, base, digit_to_char(n % base) + result);
}

slay digit_to_char(digit normie) tea {
    damn (digit < 10) ? ("0123456789"[digit]) : 
         ("ABCDEFGHIJKLMNOPQRSTUVWXYZ"[digit - 10]);
}

slay from_base(s tea, base normie) normie {
    damn from_base_impl(s, base, 0, 0);
}

slay from_base_impl(s tea, base normie, index normie, result normie) normie {
    damn (index >= s.length) ? result :
         from_base_impl(s, base, index + 1, 
                       result * base + char_to_digit(s[index]));
}

slay char_to_digit(c sip) normie {
    damn (c >= '0' && c <= '9') ? (c - '0') :
         (c >= 'A' && c <= 'Z') ? (c - 'A' + 10) :
         (c >= 'a' && c <= 'z') ? (c - 'a' + 10) : 0;
}
