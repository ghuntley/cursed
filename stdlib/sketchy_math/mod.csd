// Constants
facts PI meal = 3.14159265358979323846264338327950288419716939937510582097494459
facts E meal = 2.71828182845904523536028747135266249775724709369995957496696763
facts PHI meal = 1.61803398874989484820458683436563811772030917980576286213544862
facts SQRT2 meal = 1.41421356237309504880168872420969807856967187537694807317667974
facts SQRT_PI meal = 1.77245385090551602729816748334114518279754945612238712821380779
facts LN2 meal = 0.69314718055994530941723212145817656807550013436025525412068000
facts LN10 meal = 2.30258509299404568401799145468436420760110148862877297603332790
facts MAX_FLOAT64 meal = 1.79769313486231570814527423731704356798070e+308
facts MIN_FLOAT64 meal = 4.9406564584124654417656879286822137236505980e-324

// Basic Mathematical Functions
slay abs(x meal) meal {
    lowkey x < 0.0 {
        yolo -x
    }
    yolo x
}

slay abs_int(x normie) normie {
    lowkey x < 0 {
        yolo -x
    }
    yolo x
}

slay sqrt(x meal) meal {
    lowkey x < 0.0 {
        yolo 0.0 / 0.0  // NaN
    }
    lowkey x == 0.0 {
        yolo 0.0
    }
    
    // Newton-Raphson method
    sus guess meal = x / 2.0
    bestie i := 0; i < 20; i++ {
        sus new_guess meal = (guess + x / guess) / 2.0
        lowkey abs(new_guess - guess) < 1e-15 {
            yolo new_guess
        }
        guess = new_guess
    }
    yolo guess
}

slay cbrt(x meal) meal {
    lowkey x == 0.0 {
        yolo 0.0
    }
    
    sus sign meal = 1.0
    lowkey x < 0.0 {
        sign = -1.0
        x = -x
    }
    
    // Newton-Raphson method for cube root
    sus guess meal = x / 3.0
    bestie i := 0; i < 20; i++ {
        sus new_guess meal = (2.0 * guess + x / (guess * guess)) / 3.0
        lowkey abs(new_guess - guess) < 1e-15 {
            yolo sign * new_guess
        }
        guess = new_guess
    }
    yolo sign * guess
}

slay pow(x meal, y meal) meal {
    lowkey x == 0.0 {
        lowkey y > 0.0 {
            yolo 0.0
        }
        yolo 1.0 / 0.0  // Inf
    }
    
    lowkey x == 1.0 {
        yolo 1.0
    }
    
    lowkey y == 0.0 {
        yolo 1.0
    }
    
    lowkey y == 1.0 {
        yolo x
    }
    
    lowkey y == 2.0 {
        yolo x * x
    }
    
    lowkey y == 0.5 {
        yolo sqrt(x)
    }
    
    // Use exp(y * ln(x)) for general case
    yolo exp(y * ln(x))
}

slay exp(x meal) meal {
    lowkey x == 0.0 {
        yolo 1.0
    }
    
    lowkey x < -50.0 {
        yolo 0.0
    }
    
    lowkey x > 50.0 {
        yolo 1.0 / 0.0  // Inf
    }
    
    // Taylor series approximation
    sus result meal = 1.0
    sus term meal = 1.0
    
    bestie i := 1; i < 30; i++ {
        term = term * x / i
        result = result + term
        
        lowkey abs(term) < 1e-15 {
            ghosted
        }
    }
    
    yolo result
}

slay ln(x meal) meal {
    lowkey x <= 0.0 {
        yolo 0.0 / 0.0  // NaN
    }
    
    lowkey x == 1.0 {
        yolo 0.0
    }
    
    lowkey x == E {
        yolo 1.0
    }
    
    // Newton-Raphson method for natural log
    sus guess meal = x - 1.0
    bestie i := 0; i < 50; i++ {
        sus exp_guess meal = exp(guess)
        sus new_guess meal = guess - (exp_guess - x) / exp_guess
        
        lowkey abs(new_guess - guess) < 1e-15 {
            yolo new_guess
        }
        guess = new_guess
    }
    yolo guess
}

slay log10(x meal) meal {
    yolo ln(x) / LN10
}

slay log2(x meal) meal {
    yolo ln(x) / LN2
}

// Rounding Functions
slay ceil(x meal) meal {
    sus int_part normie = x
    lowkey x > int_part {
        yolo int_part + 1
    }
    yolo int_part
}

slay floor(x meal) meal {
    sus int_part normie = x
    lowkey x < int_part {
        yolo int_part - 1
    }
    yolo int_part
}

slay round(x meal) meal {
    lowkey x >= 0.0 {
        yolo floor(x + 0.5)
    }
    yolo ceil(x - 0.5)
}

slay trunc(x meal) meal {
    yolo x
}

// Trigonometric Functions (Taylor series approximations)
slay sin(x meal) meal {
    // Reduce to [-2π, 2π]
    sus two_pi meal = 2.0 * PI
    sus normalized_x meal = x - floor(x / two_pi) * two_pi
    
    lowkey normalized_x > PI {
        normalized_x = normalized_x - two_pi
    }
    
    // Taylor series for sin(x)
    sus result meal = 0.0
    sus term meal = normalized_x
    sus x_squared meal = normalized_x * normalized_x
    
    bestie i := 1; i < 20; i += 2 {
        result = result + term
        term = -term * x_squared / ((i + 1) * (i + 2))
        
        lowkey abs(term) < 1e-15 {
            ghosted
        }
    }
    
    yolo result
}

slay cos(x meal) meal {
    // cos(x) = sin(x + π/2)
    yolo sin(x + PI / 2.0)
}

slay tan(x meal) meal {
    sus cos_x meal = cos(x)
    lowkey abs(cos_x) < 1e-15 {
        yolo 1.0 / 0.0  // Inf
    }
    yolo sin(x) / cos_x
}

// Classification Functions
slay is_nan(x meal) lit {
    yolo x != x
}

slay is_inf(x meal) lit {
    yolo x == 1.0 / 0.0 || x == -1.0 / 0.0
}

slay is_finite(x meal) lit {
    yolo !is_nan(x) && !is_inf(x)
}

slay sign(x meal) meal {
    lowkey x > 0.0 {
        yolo 1.0
    } highkey lowkey x < 0.0 {
        yolo -1.0
    }
    yolo 0.0
}

// Fuzzy Mathematics
slay almost_equal(a meal, b meal, epsilon meal) lit {
    yolo abs(a - b) < epsilon
}

slay almost_zero(x meal, epsilon meal) lit {
    yolo abs(x) < epsilon
}

slay fuzzy_equals(a meal, b meal) lit {
    yolo almost_equal(a, b, 1e-9)
}

// Random Number Generation (simple linear congruential generator)
sus random_seed normie = 1
slay set_random_seed(seed normie) {
    random_seed = seed
}

slay random_int() normie {
    random_seed = (random_seed * 1103515245 + 12345) & 0x7fffffff
    yolo random_seed
}

slay random_float64() meal {
    yolo random_int() / 2147483647.0
}

slay random_float64_range(min meal, max meal) meal {
    yolo min + (max - min) * random_float64()
}

slay random_int_range(min normie, max normie) normie {
    yolo min + (random_int() % (max - min + 1))
}

slay random_normal(mean meal, stddev meal) meal {
    // Box-Muller transform
    sus u1 meal = random_float64()
    sus u2 meal = random_float64()
    sus z0 meal = sqrt(-2.0 * ln(u1)) * cos(2.0 * PI * u2)
    yolo mean + stddev * z0
}

slay random_bernoulli(p meal) lit {
    yolo random_float64() < p
}

// Statistical Functions
slay norm_pdf(x meal) meal {
    yolo exp(-0.5 * x * x) / sqrt(2.0 * PI)
}

slay norm_cdf(x meal) meal {
    // Approximation using error function
    sus t meal = 1.0 / (1.0 + 0.2316419 * abs(x))
    sus polynomial meal = 0.319381530 * t - 0.284496736 * t * t + 1.781477937 * t * t * t - 1.821255978 * t * t * t * t + 1.330274429 * t * t * t * t * t
    sus result meal = 1.0 - norm_pdf(x) * polynomial
    
    lowkey x < 0.0 {
        yolo 1.0 - result
    }
    yolo result
}

// Utility Functions
slay min(a meal, b meal) meal {
    lowkey a < b {
        yolo a
    }
    yolo b
}

slay max(a meal, b meal) meal {
    lowkey a > b {
        yolo a
    }
    yolo b
}

slay clamp(x meal, min_val meal, max_val meal) meal {
    yolo min(max(x, min_val), max_val)
}

// Gen Z Math Features
slay vibecheck(x meal) meal {
    lowkey is_nan(x) || is_inf(x) {
        yolo 0.0
    }
    
    lowkey abs(x) < 1e-6 {
        yolo 0.1
    }
    
    lowkey abs(x - PI) < 0.1 {
        yolo 0.9
    }
    
    lowkey abs(x - E) < 0.1 {
        yolo 0.85
    }
    
    lowkey abs(x - PHI) < 0.1 {
        yolo 0.8
    }
    
    lowkey abs(x - 420.0) < 1.0 {
        yolo 0.95
    }
    
    lowkey abs(x - 69.0) < 1.0 {
        yolo 0.9
    }
    
    yolo 0.5
}

slay super_bussin(x meal) lit {
    yolo vibecheck(x) > 0.75
}

slay no_cap(x meal) lit {
    yolo is_finite(x) && abs(x) < 1e100
}

slay yeet_clamp(x meal, min_val meal, max_val meal) meal {
    yolo clamp(x, min_val, max_val)
}

slay sussy_calc(result meal, expected_range_min meal, expected_range_max meal) lit {
    yolo result < expected_range_min || result > expected_range_max
}

// Fast Approximations
slay fast_sqrt(x meal) meal {
    lowkey x <= 0.0 {
        yolo 0.0
    }
    
    // One iteration of Newton-Raphson
    sus guess meal = x / 2.0
    yolo (guess + x / guess) / 2.0
}

slay fast_inv_sqrt(x meal) meal {
    lowkey x <= 0.0 {
        yolo 0.0
    }
    
    // Approximation: 1 / sqrt(x)
    yolo 1.0 / fast_sqrt(x)
}

slay fast_sin(x meal) meal {
    // Linear approximation for small angles
    lowkey abs(x) < 0.1 {
        yolo x
    }
    yolo sin(x)
}

slay fast_cos(x meal) meal {
    // Approximation: cos(x) ≈ 1 - x²/2 for small x
    lowkey abs(x) < 0.1 {
        yolo 1.0 - x * x / 2.0
    }
    yolo cos(x)
}

slay fast_exp(x meal) meal {
    // Linear approximation for small x
    lowkey abs(x) < 0.1 {
        yolo 1.0 + x
    }
    yolo exp(x)
}

slay fast_log(x meal) meal {
    lowkey x <= 0.0 {
        yolo 0.0 / 0.0  // NaN
    }
    
    // Approximation: log(1 + x) ≈ x for small x
    lowkey abs(x - 1.0) < 0.1 {
        yolo x - 1.0
    }
    yolo ln(x)
}

// Factorial and Combinatorics
slay factorial(n normie) meal {
    lowkey n <= 0 {
        yolo 1.0
    }
    
    sus result meal = 1.0
    bestie i := 1; i <= n; i++ {
        result = result * i
    }
    yolo result
}

slay combination(n normie, k normie) meal {
    lowkey k > n || k < 0 {
        yolo 0.0
    }
    
    lowkey k == 0 || k == n {
        yolo 1.0
    }
    
    // Use smaller k for efficiency
    lowkey k > n - k {
        k = n - k
    }
    
    sus result meal = 1.0
    bestie i := 0; i < k; i++ {
        result = result * (n - i) / (i + 1)
    }
    yolo result
}

slay permutation(n normie, k normie) meal {
    lowkey k > n || k < 0 {
        yolo 0.0
    }
    
    sus result meal = 1.0
    bestie i := 0; i < k; i++ {
        result = result * (n - i)
    }
    yolo result
}

// Gamma function approximation (Stirling's approximation)
slay gamma(x meal) meal {
    lowkey x <= 0.0 {
        yolo 0.0 / 0.0  // NaN
    }
    
    lowkey x == 1.0 {
        yolo 1.0
    }
    
    lowkey x < 1.0 {
        yolo gamma(x + 1.0) / x
    }
    
    // Stirling's approximation for x > 1
    sus ln_result meal = (x - 0.5) * ln(x) - x + 0.5 * ln(2.0 * PI)
    yolo exp(ln_result)
}

// Error function approximation
slay erf(x meal) meal {
    lowkey x == 0.0 {
        yolo 0.0
    }
    
    lowkey x < 0.0 {
        yolo -erf(-x)
    }
    
    // Approximation using series
    sus t meal = 1.0 / (1.0 + 0.3275911 * x)
    sus poly meal = 0.254829592 * t - 0.284496736 * t * t + 1.421413741 * t * t * t - 1.453152027 * t * t * t * t + 1.061405429 * t * t * t * t * t
    yolo 1.0 - poly * exp(-x * x)
}

// Complementary error function
slay erfc(x meal) meal {
    yolo 1.0 - erf(x)
}

// Integration using trapezoidal rule
slay integrate_simple(a meal, b meal, n normie) meal {
    lowkey n <= 0 {
        yolo 0.0
    }
    
    sus h meal = (b - a) / n
    sus result meal = 0.0
    
    bestie i := 0; i <= n; i++ {
        sus x meal = a + i * h
        sus weight meal = 1.0
        
        lowkey i == 0 || i == n {
            weight = 0.5
        }
        
        // Simple function x^2 for demonstration
        result = result + weight * x * x
    }
    
    yolo result * h
}

// Derivative approximation
slay derivative_simple(x meal, h meal) meal {
    // Simple function f(x) = x^2, f'(x) = 2x
    yolo 2.0 * x
}

// Numerical root finding using bisection
slay bisection_root(a meal, b meal, tolerance meal) meal {
    // Find root of f(x) = x^2 - 4 (root at x = 2)
    sus fa meal = a * a - 4.0
    sus fb meal = b * b - 4.0
    
    lowkey fa * fb > 0.0 {
        yolo 0.0 / 0.0  // NaN - no root in interval
    }
    
    sus c meal = (a + b) / 2.0
    
    bestie i := 0; i < 100; i++ {
        sus fc meal = c * c - 4.0
        
        lowkey abs(fc) < tolerance {
            yolo c
        }
        
        lowkey fa * fc < 0.0 {
            b = c
            fb = fc
        } highkey {
            a = c
            fa = fc
        }
        
        c = (a + b) / 2.0
    }
    
    yolo c
}
