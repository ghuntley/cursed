# CURSED MATHZ Module - Optimized Performance Version
# High-performance mathematical operations with vectorization and caching

yeet "vibez"

# Memoization cache for expensive calculations
sus fibonacci_cache drip[value] = []
sus factorial_cache drip[value] = []
sus prime_cache lit[value] = []
sus cache_initialized lit = cap

slay initialize_math_cache() lit {
    ready (cache_initialized) {
        damn based
    }
    
    # Pre-compute common values
    fibonacci_cache = create_array(100)
    factorial_cache = create_array(21)  # Up to 20!
    prime_cache = create_array(1000)    # Primes up to 1000
    
    # Initialize fibonacci cache
    set_array_element(fibonacci_cache, 0, 0)
    set_array_element(fibonacci_cache, 1, 1)
    bestie (sus i drip = 2; i < 100; i++) {
        sus val drip = get_array_element(fibonacci_cache, i-1) + get_array_element(fibonacci_cache, i-2)
        set_array_element(fibonacci_cache, i, val)
    }
    
    # Initialize factorial cache
    set_array_element(factorial_cache, 0, 1)
    bestie (sus i drip = 1; i < 21; i++) {
        sus val drip = get_array_element(factorial_cache, i-1) * i
        set_array_element(factorial_cache, i, val)
    }
    
    # Initialize prime cache using Sieve of Eratosthenes
    precompute_primes(1000)
    
    cache_initialized = based
    damn based
}

# Optimized Sieve of Eratosthenes for prime generation
slay precompute_primes(limit drip) lit {
    sus is_prime lit[value] = create_boolean_array(limit + 1)
    
    # Initialize all as prime
    bestie (sus i drip = 2; i <= limit; i++) {
        set_boolean_element(is_prime, i, based)
    }
    
    sus p drip = 2
    bestie (p * p <= limit) {
        ready (get_boolean_element(is_prime, p)) {
            # Mark all multiples of p as composite
            sus multiple drip = p * p
            bestie (multiple <= limit) {
                set_boolean_element(is_prime, multiple, cap)
                multiple = multiple + p
            }
        }
        p = p + 1
    }
    
    # Store results in prime cache
    bestie (sus i drip = 2; i <= limit; i++) {
        set_boolean_element(prime_cache, i, get_boolean_element(is_prime, i))
    }
    
    damn based
}

# Vectorized array sum for large datasets
slay array_sum_vectorized(arr drip[value]) drip {
    sus length drip = len(arr)
    ready (length == 0) { damn 0 }
    
    sus sum drip = 0
    sus vector_size drip = 8  # Process 8 elements at a time
    
    # Vectorized loop for bulk processing
    sus i drip = 0
    bestie (i + vector_size <= length) {
        sus partial_sum drip = 0
        
        # Unroll loop for better performance
        partial_sum = partial_sum + get_array_element(arr, i)
        partial_sum = partial_sum + get_array_element(arr, i + 1)
        partial_sum = partial_sum + get_array_element(arr, i + 2)
        partial_sum = partial_sum + get_array_element(arr, i + 3)
        partial_sum = partial_sum + get_array_element(arr, i + 4)
        partial_sum = partial_sum + get_array_element(arr, i + 5)
        partial_sum = partial_sum + get_array_element(arr, i + 6)
        partial_sum = partial_sum + get_array_element(arr, i + 7)
        
        sum = sum + partial_sum
        i = i + vector_size
    }
    
    # Handle remaining elements
    bestie (i < length) {
        sum = sum + get_array_element(arr, i)
        i = i + 1
    }
    
    damn sum
}

# Fast integer power using exponentiation by squaring
slay power_fast(base drip, exponent drip) drip {
    ready (exponent == 0) { damn 1 }
    ready (exponent == 1) { damn base }
    ready (exponent < 0) { damn 0 }  # Integer division would be 0 for most cases
    
    sus result drip = 1
    sus b drip = base
    sus exp drip = exponent
    
    bestie (exp > 0) {
        ready (exp % 2 == 1) {
            result = result * b
        }
        b = b * b
        exp = exp / 2
    }
    
    damn result
}

# Optimized GCD using binary GCD algorithm
slay gcd_optimized(a drip, b drip) drip {
    ready (a == 0) { damn b }
    ready (b == 0) { damn a }
    
    sus x drip = a
    sus y drip = b
    ready (x < 0) { x = -x }
    ready (y < 0) { y = -y }
    
    # Use binary GCD (Stein's algorithm) for better performance
    sus shift drip = 0
    
    # Factor out powers of 2
    bestie ((x | y) & 1 == 0) {
        x = x >> 1
        y = y >> 1
        shift = shift + 1
    }
    
    bestie (x & 1 == 0) { x = x >> 1 }
    
    bestie (y != 0) {
        bestie (y & 1 == 0) { y = y >> 1 }
        
        ready (x > y) {
            sus temp drip = x
            x = y
            y = temp
        }
        
        y = y - x
    }
    
    damn x << shift
}

# Cached factorial with overflow detection
slay factorial_optimized(n drip) drip {
    ready (!cache_initialized) {
        initialize_math_cache()
    }
    
    ready (n < 0) { damn 0 }
    ready (n < 21) {
        damn get_array_element(factorial_cache, n)
    }
    
    # For larger values, compute iteratively with overflow check
    sus result drip = get_array_element(factorial_cache, 20)  # Start from 20!
    bestie (sus i drip = 21; i <= n; i++) {
        # Check for potential overflow
        ready (result > 9223372036854775807 / i) {
            vibez.spill("Factorial overflow detected")
            damn 0
        }
        result = result * i
    }
    
    damn result
}

# Fast fibonacci using cached values and matrix exponentiation
slay fibonacci_optimized(n drip) drip {
    ready (!cache_initialized) {
        initialize_math_cache()
    }
    
    ready (n < 0) { damn 0 }
    ready (n < 100) {
        damn get_array_element(fibonacci_cache, n)
    }
    
    # For larger values, use matrix exponentiation
    damn fibonacci_matrix_power(n)
}

# Matrix exponentiation for large fibonacci numbers
slay fibonacci_matrix_power(n drip) drip {
    ready (n == 0) { damn 0 }
    ready (n == 1) { damn 1 }
    
    # Matrix [[1,1],[1,0]]^n gives [[F(n+1),F(n)],[F(n),F(n-1)]]
    sus matrix drip[value][value] = [[1, 1], [1, 0]]
    sus result drip[value][value] = matrix_power(matrix, n - 1)
    
    damn get_matrix_element(result, 0, 0)
}

# Matrix multiplication for 2x2 matrices
slay matrix_multiply_2x2(a drip[value][value], b drip[value][value]) drip[value][value] {
    sus result drip[value][value] = create_2d_array(2, 2)
    
    set_matrix_element(result, 0, 0, 
        get_matrix_element(a, 0, 0) * get_matrix_element(b, 0, 0) + 
        get_matrix_element(a, 0, 1) * get_matrix_element(b, 1, 0))
    
    set_matrix_element(result, 0, 1,
        get_matrix_element(a, 0, 0) * get_matrix_element(b, 0, 1) + 
        get_matrix_element(a, 0, 1) * get_matrix_element(b, 1, 1))
    
    set_matrix_element(result, 1, 0,
        get_matrix_element(a, 1, 0) * get_matrix_element(b, 0, 0) + 
        get_matrix_element(a, 1, 1) * get_matrix_element(b, 1, 0))
    
    set_matrix_element(result, 1, 1,
        get_matrix_element(a, 1, 0) * get_matrix_element(b, 0, 1) + 
        get_matrix_element(a, 1, 1) * get_matrix_element(b, 1, 1))
    
    damn result
}

# Matrix exponentiation using binary exponentiation
slay matrix_power(matrix drip[value][value], n drip) drip[value][value] {
    ready (n == 0) {
        sus identity drip[value][value] = [[1, 0], [0, 1]]
        damn identity
    }
    ready (n == 1) { damn matrix }
    
    ready (n % 2 == 0) {
        sus half_power drip[value][value] = matrix_power(matrix, n / 2)
        damn matrix_multiply_2x2(half_power, half_power)
    } otherwise {
        sus half_power drip[value][value] = matrix_power(matrix, n / 2)
        sus squared drip[value][value] = matrix_multiply_2x2(half_power, half_power)
        damn matrix_multiply_2x2(matrix, squared)
    }
}

# Optimized prime checking using precomputed sieve and wheel factorization
slay is_prime_optimized(n drip) lit {
    ready (!cache_initialized) {
        initialize_math_cache()
    }
    
    ready (n < 2) { damn cap }
    ready (n < 1000) {
        damn get_boolean_element(prime_cache, n)
    }
    
    # For larger numbers, use wheel factorization (2, 3, 5)
    ready (n % 2 == 0) { damn n == 2 }
    ready (n % 3 == 0) { damn n == 3 }
    ready (n % 5 == 0) { damn n == 5 }
    
    # Check divisibility using 6k±1 pattern
    sus i drip = 7
    bestie (i * i <= n) {
        ready (n % i == 0 || n % (i + 4) == 0) {
            damn cap
        }
        i = i + 6
    }
    
    damn based
}

# Fast square root using Newton's method with initial guess
slay sqrt_fast(n drip) drip {
    ready (n <= 0) { damn 0 }
    ready (n == 1) { damn 1 }
    
    # Initial guess using bit manipulation
    sus x drip = n
    sus shift drip = 0
    
    # Find the highest set bit
    sus temp drip = n
    bestie (temp > 1) {
        temp = temp >> 1
        shift = shift + 1
    }
    
    x = 1 << (shift / 2 + 1)
    
    # Newton's method: x = (x + n/x) / 2
    bestie (based) {
        sus next drip = (x + n / x) / 2
        ready (next >= x) {
            damn x
        }
        x = next
    }
    
    damn x
}

# Vectorized array operations
slay array_multiply_scalar(arr drip[value], scalar drip) drip[value]{
    sus length drip = len(arr)
    sus result drip[value] = create_array(length)
    
    sus vector_size drip = 4
    sus i drip = 0
    
    # Vectorized processing
    bestie (i + vector_size <= length) {
        set_array_element(result, i, get_array_element(arr, i) * scalar)
        set_array_element(result, i + 1, get_array_element(arr, i + 1) * scalar)
        set_array_element(result, i + 2, get_array_element(arr, i + 2) * scalar)
        set_array_element(result, i + 3, get_array_element(arr, i + 3) * scalar)
        i = i + vector_size
    }
    
    # Handle remaining elements
    bestie (i < length) {
        set_array_element(result, i, get_array_element(arr, i) * scalar)
        i = i + 1
    }
    
    damn result
}

# Optimized array dot product
slay array_dot_product(a drip[value], b drip[value]) drip {
    sus length drip = min_int(len(a), len(b))
    sus result drip = 0
    
    sus vector_size drip = 4
    sus i drip = 0
    
    # Vectorized dot product
    bestie (i + vector_size <= length) {
        sus partial drip = 0
        partial = partial + get_array_element(a, i) * get_array_element(b, i)
        partial = partial + get_array_element(a, i + 1) * get_array_element(b, i + 1)
        partial = partial + get_array_element(a, i + 2) * get_array_element(b, i + 2)
        partial = partial + get_array_element(a, i + 3) * get_array_element(b, i + 3)
        result = result + partial
        i = i + vector_size
    }
    
    # Handle remaining elements
    bestie (i < length) {
        result = result + get_array_element(a, i) * get_array_element(b, i)
        i = i + 1
    }
    
    damn result
}

# Statistical functions with single-pass algorithms
slay array_mean_variance(arr drip[value]) drip[value]{
    sus length drip = len(arr)
    ready (length == 0) { damn [0, 0] }
    
    # Welford's online algorithm for numerically stable variance
    sus mean drip = 0
    sus m2 drip = 0
    
    bestie (sus i drip = 0; i < length; i++) {
        sus val drip = get_array_element(arr, i)
        sus delta drip = val - mean
        mean = mean + delta / (i + 1)
        sus delta2 drip = val - mean
        m2 = m2 + delta * delta2
    }
    
    sus variance drip = ready (length > 1) { m2 / (length - 1) } otherwise { 0 }
    
    sus result drip[value] = create_array(2)
    set_array_element(result, 0, mean)
    set_array_element(result, 1, variance)
    damn result
}

# Helper functions
slay min_int(a drip, b drip) drip {
    ready (a < b) { damn a } otherwise { damn b }
}

slay max_int(a drip, b drip) drip {
    ready (a > b) { damn a } otherwise { damn b }
}

# Placeholder functions for runtime implementation
slay create_boolean_array(size drip) lit[value]{
    damn []  # Implemented in runtime
}

slay set_boolean_element(arr lit[value], index drip, value lit) lit {
    damn based  # Implemented in runtime
}

slay get_boolean_element(arr lit[value], index drip) lit {
    damn cap  # Implemented in runtime
}

slay create_2d_array(rows drip, cols drip) drip[value][value] {
    damn [[]]  # Implemented in runtime
}

slay get_matrix_element(matrix drip[value][value], row drip, col drip) drip {
    damn 0  # Implemented in runtime
}

slay set_matrix_element(matrix drip[value][value], row drip, col drip, value drip) lit {
    damn based  # Implemented in runtime
}

# Export optimized math functions
slay math_add(a drip, b drip) drip { damn a + b }
slay math_subtract(a drip, b drip) drip { damn a - b }
slay math_multiply(a drip, b drip) drip { damn a * b }
slay math_divide(a drip, b drip) drip { 
    ready (b == 0) { damn 0 }
    damn a / b 
}
slay math_power(base drip, exp drip) drip { damn power_fast(base, exp) }
slay math_gcd(a drip, b drip) drip { damn gcd_optimized(a, b) }
slay math_factorial(n drip) drip { damn factorial_optimized(n) }
slay math_fibonacci(n drip) drip { damn fibonacci_optimized(n) }
slay math_is_prime(n drip) lit { damn is_prime_optimized(n) }
slay math_sqrt(n drip) drip { damn sqrt_fast(n) }
