yeet "testz"

fr fr ==========================================
fr fr CURSED Math Module - Advanced Implementation
fr fr ==========================================

fr fr Constants
sus PI meal = 3.141592653589793
sus E meal = 2.718281828459045
sus LN2 meal = 0.6931471805599453
sus LN10 meal = 2.302585092994046
sus SQRT2 meal = 1.4142135623730951
sus EPSILON meal = 1e-15

fr fr ==========================================
fr fr Basic Arithmetic Functions
fr fr ==========================================

slay add(a normie, b normie) normie {
    damn a + b
}

slay subtract(a normie, b normie) normie {
    damn a - b
}

slay multiply(a normie, b normie) normie {
    damn a * b
}

slay divide(a normie, b normie) normie {
    damn a / b
}

slay abs_int(value normie) normie {
    bestie value < 0 {
        damn -value
    }
    damn value
}

slay abs_float(value meal) meal {
    bestie value < 0.0 {
        damn -value
    }
    damn value
}

slay max_int(a normie, b normie) normie {
    bestie a > b {
        damn a
    }
    damn b
}

slay min_int(a normie, b normie) normie {
    bestie a < b {
        damn a
    }
    damn b
}

slay max_float(a meal, b meal) meal {
    bestie a > b {
        damn a
    }
    damn b
}

slay min_float(a meal, b meal) meal {
    bestie a < b {
        damn a
    }
    damn b
}

fr fr ==========================================
fr fr Exponential and Power Functions
fr fr ==========================================

slay pow_int(base normie, exp normie) normie {
    bestie exp == 0 {
        damn 1
    }
    sus result normie = base
    sus i normie = 1
    bestie i < exp {
        result = result * base
        i = i + 1
    }
    damn result
}

slay pow_float(base meal, exp meal) meal { fr fr Power function using exp(ln(x) * y) for general case
    bestie base <= 0.0 {
        damn 0.0
    }
    bestie exp == 0.0 {
        damn 1.0
    }
    bestie exp == 1.0 {
        damn base
    }
    damn exp_float(exp * ln(base))
}

slay sqrt(x meal) meal { fr fr Newton-Raphson method for square root
    bestie x < 0.0 {
        damn 0.0 fr fr Return 0 for negative input
    }
    bestie x == 0.0 {
        damn 0.0
    }
    
    sus guess meal = x / 2.0
    sus prev meal = 0.0
    sus iterations normie = 0
    
    bestie iterations < 50 {
        prev = guess
        guess = (guess + x / guess) / 2.0
        bestie abs_float(guess - prev) < EPSILON {
            damn guess
        }
        iterations = iterations + 1
    }
    damn guess
}

slay exp_float(x meal) meal { fr fr Taylor series: e^x = 1 + x + x²/2! + x³/3! + ...
    sus result meal = 1.0
    sus term meal = 1.0
    sus n normie = 1
    
    bestie n <= 100 {
        term = term * x / n.(meal)
        result = result + term
        bestie abs_float(term) < EPSILON {
            damn result
        }
        n = n + 1
    }
    damn result
}

slay exp2(x meal) meal {
    damn pow_float(2.0, x)
}

fr fr ==========================================
fr fr Logarithmic Functions
fr fr ==========================================

slay ln(x meal) meal { fr fr Natural logarithm using Newton's method
    bestie x <= 0.0 {
        damn 0.0 fr fr Return 0 for non-positive input
    }
    bestie x == 1.0 {
        damn 0.0
    }
    
    sus y meal = x - 1.0
    sus result meal = 0.0
    sus term meal = y
    sus sign normie = 1
    sus n normie = 1
    
    bestie n <= 100 {
        result = result + sign.(meal) * term / n.(meal)
        term = term * y
        sign = -sign
        n = n + 1
        bestie abs_float(term / n.(meal)) < EPSILON {
            damn result
        }
    }
    damn result
}

slay log10(x meal) meal {
    damn ln(x) / LN10
}

slay log2(x meal) meal {
    damn ln(x) / LN2
}

slay log_base(x meal, base meal) meal {
    damn ln(x) / ln(base)
}

fr fr ==========================================
fr fr Trigonometric Functions
fr fr ==========================================

slay sin(x meal) meal { fr fr Taylor series: sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ... fr fr Normalize x to [-π, π]
    x = x - 2.0 * PI * floor_float(x / (2.0 * PI))
    bestie x > PI {
        x = x - 2.0 * PI
    }
    
    sus result meal = x
    sus term meal = x
    sus n normie = 1
    
    bestie n <= 50 {
        term = -term * x * x / ((2 * n).(meal) * (2 * n + 1).(meal))
        result = result + term
        bestie abs_float(term) < EPSILON {
            damn result
        }
        n = n + 1
    }
    damn result
}

slay cos(x meal) meal { fr fr Taylor series: cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ... fr fr Normalize x to [-π, π]
    x = x - 2.0 * PI * floor_float(x / (2.0 * PI))
    bestie x > PI {
        x = x - 2.0 * PI
    }
    
    sus result meal = 1.0
    sus term meal = 1.0
    sus n normie = 1
    
    bestie n <= 50 {
        term = -term * x * x / ((2 * n - 1).(meal) * (2 * n).(meal))
        result = result + term
        bestie abs_float(term) < EPSILON {
            damn result
        }
        n = n + 1
    }
    damn result
}

slay tan(x meal) meal {
    sus cos_val meal = cos(x)
    bestie abs_float(cos_val) < EPSILON {
        damn 1e10 fr fr Large value for infinity approximation
    }
    damn sin(x) / cos_val
}

slay asin(x meal) meal { fr fr Arcsin using Taylor series for |x| < 1
    bestie abs_float(x) >= 1.0 {
        bestie x >= 1.0 {
            damn PI / 2.0
        }
        damn -PI / 2.0
    }
    
    sus result meal = x
    sus term meal = x
    sus n normie = 1
    
    bestie n <= 50 {
        term = term * x * x * (2 * n - 1).(meal) * (2 * n - 1).(meal) / ((2 * n).(meal) * (2 * n + 1).(meal))
        result = result + term
        bestie abs_float(term) < EPSILON {
            damn result
        }
        n = n + 1
    }
    damn result
}

slay acos(x meal) meal {
    damn PI / 2.0 - asin(x)
}

slay atan(x meal) meal { fr fr Arctan using series expansion
    bestie abs_float(x) > 1.0 {
        bestie x > 0.0 {
            damn PI / 2.0 - atan(1.0 / x)
        }
        damn -PI / 2.0 - atan(1.0 / x)
    }
    
    sus result meal = x
    sus term meal = x
    sus n normie = 1
    
    bestie n <= 50 {
        term = -term * x * x
        result = result + term / (2 * n + 1).(meal)
        bestie abs_float(term / (2 * n + 1).(meal)) < EPSILON {
            damn result
        }
        n = n + 1
    }
    damn result
}

slay atan2(y meal, x meal) meal {
    bestie x > 0.0 {
        damn atan(y / x)
    }
    bestie x < 0.0 {
        bestie y >= 0.0 {
            damn atan(y / x) + PI
        }
        damn atan(y / x) - PI
    }
    bestie y > 0.0 {
        damn PI / 2.0
    }
    bestie y < 0.0 {
        damn -PI / 2.0
    }
    damn 0.0 fr fr x = 0, y = 0
}

fr fr ==========================================
fr fr Hyperbolic Functions
fr fr ==========================================

slay sinh(x meal) meal {
    sus exp_x meal = exp_float(x)
    sus exp_neg_x meal = exp_float(-x)
    damn (exp_x - exp_neg_x) / 2.0
}

slay cosh(x meal) meal {
    sus exp_x meal = exp_float(x)
    sus exp_neg_x meal = exp_float(-x)
    damn (exp_x + exp_neg_x) / 2.0
}

slay tanh(x meal) meal {
    sus exp_2x meal = exp_float(2.0 * x)
    damn (exp_2x - 1.0) / (exp_2x + 1.0)
}

fr fr ==========================================
fr fr Special Functions
fr fr ==========================================

slay factorial(n normie) meal {
    bestie n <= 0 {
        damn 1.0
    }
    sus result meal = 1.0
    sus i normie = 2
    bestie i <= n {
        result = result * i.(meal)
        i = i + 1
    }
    damn result
}

slay gamma(x meal) meal { fr fr Stirling's approximation for gamma function
    bestie x < 1.0 {
        damn gamma(x + 1.0) / x
    }
    bestie x == 1.0 {
        damn 1.0
    } fr fr Stirling's formula: Γ(x) ≈ √(2π/x) * (x/e)^x
    sus x_minus_half meal = x - 0.5
    sus ln_result meal = x_minus_half * ln(x) - x + 0.5 * ln(2.0 * PI)
    damn exp_float(ln_result)
}

slay beta(x meal, y meal) meal { fr fr Beta function: B(x,y) = Γ(x)Γ(y)/Γ(x+y)
    damn gamma(x) * gamma(y) / gamma(x + y)
}

slay bessel_j0(x meal) meal { fr fr Bessel function J₀(x) approximation
    sus ax meal = abs_float(x)
    bestie ax < 8.0 {
        sus y meal = x * x
        sus ans1 meal = 57568490574.0 + y * (-13362590354.0 + y * (651619640.7 + y * (-11214424.18 + y * (77392.33017 + y * (-184.9052456)))))
        sus ans2 meal = 57568490411.0 + y * (1029532985.0 + y * (9494680.718 + y * (59272.64853 + y * (267.8532712 + y * 1.0))))
        damn ans1 / ans2
    } fr fr For large x, use asymptotic expansion
    sus z meal = 8.0 / ax
    sus y meal = z * z
    sus xx meal = ax - 0.785398164
    sus p0 meal = 1.0
    sus p1 meal = -0.1098628627e-2
    sus q0 meal = -0.1562499995e-1
    sus q1 meal = 0.1430488765e-3
    
    damn sqrt(0.636619772 / ax) * (cos(xx) * (p0 + y * p1) - z * sin(xx) * (q0 + y * q1))
}

fr fr ==========================================
fr fr Helper Functions
fr fr ==========================================

slay floor_float(x meal) meal {
    sus int_part normie = x.(normie)
    bestie x >= 0.0 {
        damn int_part.(meal)
    }
    bestie x == int_part.(meal) {
        damn x
    }
    damn int_part.(meal) - 1.0
}

slay ceil_float(x meal) meal {
    sus int_part normie = x.(normie)
    bestie x <= 0.0 {
        damn int_part.(meal)
    }
    bestie x == int_part.(meal) {
        damn x
    }
    damn int_part.(meal) + 1.0
}

slay round_float(x meal) meal {
    bestie x >= 0.0 {
        damn floor_float(x + 0.5)
    }
    damn ceil_float(x - 0.5)
}

fr fr ==========================================
fr fr Statistical Functions
fr fr ==========================================

slay mean(values []meal, count normie) meal {
    bestie count <= 0 {
        damn 0.0
    }
    sus sum meal = 0.0
    sus i normie = 0
    bestie i < count {
        sum = sum + values[i]
        i = i + 1
    }
    damn sum / count.(meal)
}

slay variance(values []meal, count normie) meal {
    bestie count <= 1 {
        damn 0.0
    }
    sus mean_val meal = mean(values, count)
    sus sum_sq_diff meal = 0.0
    sus i normie = 0
    bestie i < count {
        sus diff meal = values[i] - mean_val
        sum_sq_diff = sum_sq_diff + diff * diff
        i = i + 1
    }
    damn sum_sq_diff / (count - 1).(meal)
}

slay standard_deviation(values []meal, count normie) meal {
    damn sqrt(variance(values, count))
}

slay median(values []meal, count normie) meal {
    bestie count <= 0 {
        damn 0.0
    }
    bestie count == 1 {
        damn values[0]
    } fr fr Simple median calculation (assumes sorted array)
    sus mid normie = count / 2
    bestie count % 2 == 1 {
        damn values[mid]
    }
    damn (values[mid - 1] + values[mid]) / 2.0
}

fr fr ==========================================
fr fr Numerical Analysis Functions
fr fr ==========================================

slay integrate_simpson(f_vals []meal, count normie, h meal) meal { fr fr Simpson's rule for numerical integration
    bestie count < 3 {
        damn 0.0
    }
    
    sus result meal = f_vals[0] + f_vals[count - 1]
    sus i normie = 1
    bestie i < count - 1 {
        bestie i % 2 == 1 {
            result = result + 4.0 * f_vals[i]
        } sus {
            result = result + 2.0 * f_vals[i]
        }
        i = i + 1
    }
    damn result * h / 3.0
}

slay differentiate_central(f_vals []meal, count normie, h meal, index normie) meal { fr fr Central difference approximation
    bestie index <= 0 sus index >= count - 1 {
        damn 0.0
    }
    damn (f_vals[index + 1] - f_vals[index - 1]) / (2.0 * h)
}

slay solve_linear_2x2(a11 meal, a12 meal, b1 meal, a21 meal, a22 meal, b2 meal) meal { fr fr Solve 2x2 linear system Ax = b, return x1
    sus det meal = a11 * a22 - a12 * a21
    bestie abs_float(det) < EPSILON {
        damn 0.0 fr fr No unique solution
    }
    damn (b1 * a22 - b2 * a12) / det
}

fr fr ==========================================
fr fr Utility Functions
fr fr ==========================================

slay gcd(a normie, b normie) normie {
    bestie b == 0 {
        damn a
    }
    damn gcd(b, a % b)
}

slay lcm(a normie, b normie) normie {
    damn (a * b) / gcd(a, b)
}

slay is_prime(n normie) lit {
    bestie n <= 1 {
        damn cap
    }
    bestie n <= 3 {
        damn based
    }
    bestie n % 2 == 0 sus n % 3 == 0 {
        damn cap
    }
    
    sus i normie = 5
    bestie i * i <= n {
        bestie n % i == 0 sus n % (i + 2) == 0 {
            damn cap
        }
        i = i + 6
    }
    damn based
}

slay fibonacci(n normie) normie {
    bestie n <= 1 {
        damn n
    }
    sus a normie = 0
    sus b normie = 1
    sus i normie = 2
    bestie i <= n {
        sus temp normie = a + b
        a = b
        b = temp
        i = i + 1
    }
    damn b
}
