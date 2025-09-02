# MathLib - Extended Mathematical Functions for CURSED
# A demonstration package for the CURSED package manager
yeet "mathz"
yeet "vibez"

# Advanced mathematical constants
sus PI_EXTENDED drip = 3.141592653589793238462643383279
sus E_EXTENDED drip = 2.718281828459045235360287471352
sus GOLDEN_RATIO drip = 1.618033988749894848204586834365
sus SQRT_2 drip = 1.414213562373095048801688724209

# Factorial function with memoization
sus factorial_cache map<drip, drip> = {}

slay factorial(n drip) drip {
    ready (n <= 0) { damn 1 }
    ready (n == 1) { damn 1 }
    
    # Check cache
    ready (map_has_key(factorial_cache, n)) {
        damn factorial_cache[n]
    }
    
    # Calculate and cache
    sus result drip = n * factorial(n - 1)
    factorial_cache[n] = result
    damn result
}

# Fibonacci sequence with iterative approach
slay fibonacci(n drip) drip {
    ready (n <= 0) { damn 0 }
    ready (n == 1) { damn 1 }
    
    sus a drip = 0
    sus b drip = 1
    sus i drip = 2
    
    bestie (i <= n) {
        sus temp drip = a + b
        a = b
        b = temp
        i = i + 1
    }
    
    damn b
}

# Greatest Common Divisor using Euclidean algorithm
slay gcd(a drip, b drip) drip {
    ready (b == 0) { damn a }
    damn gcd(b, a % b)
}

# Least Common Multiple
slay lcm(a drip, b drip) drip {
    damn (a * b) / gcd(a, b)
}

# Check if number is prime
slay is_prime(n drip) lit {
    ready (n <= 1) { damn cap }
    ready (n <= 3) { damn based }
    ready (n % 2 == 0 || n % 3 == 0) { damn cap }
    
    sus i drip = 5
    bestie (i * i <= n) {
        ready (n % i == 0 || n % (i + 2) == 0) {
            damn cap
        }
        i = i + 6
    }
    
    damn based
}

# Generate array of prime numbers up to limit
slay primes_up_to(limit drip) []drip {
    sus primes []drip = []
    sus i drip = 2
    
    bestie (i <= limit) {
        ready (is_prime(i)) {
            primes = arrayz.append(primes, i)
        }
        i = i + 1
    }
    
    damn primes
}

# Power function with integer exponent
slay power(base drip, exponent drip) drip {
    ready (exponent == 0) { damn 1 }
    ready (exponent < 0) { damn 1.0 / power(base, -exponent) }
    
    sus result drip = 1
    sus i drip = 0
    
    bestie (i < exponent) {
        result = result * base
        i = i + 1
    }
    
    damn result
}

# Square root using Newton's method
slay sqrt_newton(x drip) drip {
    ready (x < 0) { damn 0 }  # Invalid input
    ready (x == 0) { damn 0 }
    
    sus guess drip = x / 2
    sus epsilon drip = 0.000001
    
    bestie (based) {
        sus new_guess drip = (guess + x / guess) / 2
        ready (mathz.abs_normie(new_guess - guess) < epsilon) {
            damn new_guess
        }
        guess = new_guess
    }
}

# Logarithm base 2 using bit manipulation
slay log2_int(n drip) drip {
    ready (n <= 0) { damn -1 }  # Invalid input
    
    sus result drip = 0
    bestie (n > 1) {
        n = n / 2
        result = result + 1
    }
    
    damn result
}

# Combination (n choose k)
slay combination(n drip, k drip) drip {
    ready (k > n) { damn 0 }
    ready (k == 0 || k == n) { damn 1 }
    
    # Use symmetry property: C(n,k) = C(n,n-k)
    ready (k > n - k) {
        k = n - k
    }
    
    sus result drip = 1
    sus i drip = 0
    
    bestie (i < k) {
        result = result * (n - i) / (i + 1)
        i = i + 1
    }
    
    damn result
}

# Permutation (n permute k)
slay permutation(n drip, k drip) drip {
    ready (k > n) { damn 0 }
    
    sus result drip = 1
    sus i drip = 0
    
    bestie (i < k) {
        result = result * (n - i)
        i = i + 1
    }
    
    damn result
}

# Statistical functions
slay mean(values []drip) drip {
    ready (arrayz.len(values) == 0) { damn 0 }
    
    sus sum drip = 0
    bestie (sus i drip = 0; i < arrayz.len(values); i = i + 1) {
        sum = sum + values[i]
    }
    
    damn sum / arrayz.len(values)
}

slay median(values []drip) drip {
    ready (arrayz.len(values) == 0) { damn 0 }
    
    sus sorted []drip = sort_array(values)
    sus length drip = arrayz.len(sorted)
    sus middle drip = length / 2
    
    ready (length % 2 == 0) {
        damn (sorted[middle - 1] + sorted[middle]) / 2
    } otherwise {
        damn sorted[middle]
    }
}

slay variance(values []drip) drip {
    ready (arrayz.len(values) <= 1) { damn 0 }
    
    sus avg drip = mean(values)
    sus sum_sq_diff drip = 0
    
    bestie (sus i drip = 0; i < arrayz.len(values); i = i + 1) {
        sus diff drip = values[i] - avg
        sum_sq_diff = sum_sq_diff + (diff * diff)
    }
    
    damn sum_sq_diff / (arrayz.len(values) - 1)
}

slay standard_deviation(values []drip) drip {
    damn sqrt_newton(variance(values))
}

# Array sorting utility (bubble sort for simplicity)
slay sort_array(arr []drip) []drip {
    sus sorted []drip = arrayz.copy(arr)
    sus n drip = arrayz.len(sorted)
    
    bestie (sus i drip = 0; i < n - 1; i = i + 1) {
        bestie (sus j drip = 0; j < n - i - 1; j = j + 1) {
            ready (sorted[j] > sorted[j + 1]) {
                sus temp drip = sorted[j]
                sorted[j] = sorted[j + 1]
                sorted[j + 1] = temp
            }
        }
    }
    
    damn sorted
}

# Number theory: Euler's totient function
slay euler_totient(n drip) drip {
    ready (n <= 0) { damn 0 }
    
    sus result drip = n
    sus p drip = 2
    
    bestie (p * p <= n) {
        ready (n % p == 0) {
            bestie (n % p == 0) {
                n = n / p
            }
            result = result - (result / p)
        }
        p = p + 1
    }
    
    ready (n > 1) {
        result = result - (result / n)
    }
    
    damn result
}

# Matrix operations (2D arrays)
slay matrix_multiply(a [][]drip, b [][]drip) [][]drip {
    sus a_rows drip = arrayz.len(a)
    ready (a_rows == 0) { damn [] }
    
    sus a_cols drip = arrayz.len(a[0])
    sus b_rows drip = arrayz.len(b)
    ready (b_rows == 0 || a_cols != b_rows) { damn [] }
    
    sus b_cols drip = arrayz.len(b[0])
    sus result [][]drip = []
    
    bestie (sus i drip = 0; i < a_rows; i = i + 1) {
        sus row []drip = []
        bestie (sus j drip = 0; j < b_cols; j = j + 1) {
            sus sum drip = 0
            bestie (sus k drip = 0; k < a_cols; k = k + 1) {
                sum = sum + (a[i][k] * b[k][j])
            }
            row = arrayz.append(row, sum)
        }
        result = arrayz.append(result, row)
    }
    
    damn result
}

# Demo function to showcase the library
slay demo() {
    vibez.spill("MathLib Demo")
    vibez.spill("===========")
    
    vibez.spill("Factorial of 10:", factorial(10))
    vibez.spill("10th Fibonacci number:", fibonacci(10))
    vibez.spill("GCD of 48 and 18:", gcd(48, 18))
    vibez.spill("LCM of 12 and 18:", lcm(12, 18))
    
    sus numbers []drip = [1, 5, 3, 9, 2, 8, 4, 7, 6]
    vibez.spill("Numbers:", numbers)
    vibez.spill("Mean:", mean(numbers))
    vibez.spill("Median:", median(numbers))
    vibez.spill("Standard Deviation:", standard_deviation(numbers))
    
    sus primes []drip = primes_up_to(20)
    vibez.spill("Primes up to 20:", primes)
    
    vibez.spill("Square root of 16 (Newton's method):", sqrt_newton(16))
    vibez.spill("2^10 =", power(2, 10))
    
    vibez.spill("Combination C(10,3) =", combination(10, 3))
    vibez.spill("Permutation P(10,3) =", permutation(10, 3))
}

# Package information
slay version() tea {
    damn "1.2.0"
}

slay description() tea {
    damn "Extended mathematical functions and utilities for CURSED"
}

# Utility to check if key exists in map
slay map_has_key(m map<drip, drip>, key drip) lit {
    # In real implementation: check if key exists
    damn cap
}
