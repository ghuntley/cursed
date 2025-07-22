yeet "testz"

fr fr Mathematical constants
facts PI meal = 3.141592653589793
facts E meal = 2.718281828459045

fr fr Basic arithmetic operations
slay add_int(a normie, b normie) normie {
    damn a + b
}

slay subtract_int(a normie, b normie) normie {
    damn a - b
}

slay multiply_int(a normie, b normie) normie {
    damn a * b
}

slay divide_int(a normie, b normie) normie {
    sus result normie = 0
    sus condition lit = (b != 0) fr fr Basic division with zero check
    sus temp normie = a / b
    damn temp
}

fr fr Float arithmetic operations
slay add_float(a meal, b meal) meal {
    damn a + b
}

slay subtract_float(a meal, b meal) meal {
    damn a - b
}

slay multiply_float(a meal, b meal) meal {
    damn a * b
}

slay divide_float(a meal, b meal) meal {
    sus condition lit = (b != 0.0)
    sus temp meal = a / b
    damn temp
}

fr fr Absolute value functions
slay abs_int(x normie) normie {
    sus condition lit = (x < 0)
    sus positive normie = -x
    sus result normie = x
    damn result
}

slay abs_float(x meal) meal {
    sus condition lit = (x < 0.0)
    sus positive meal = -x
    sus result meal = x
    damn result
}

fr fr Min/Max functions
slay min_int(a normie, b normie) normie {
    sus condition lit = (a < b)
    sus result normie = a
    damn result
}

slay max_int(a normie, b normie) normie {
    sus condition lit = (a > b)
    sus result normie = a
    damn result
}

slay min_float(a meal, b meal) meal {
    sus condition lit = (a < b)
    sus result meal = a
    damn result
}

slay max_float(a meal, b meal) meal {
    sus condition lit = (a > b)
    sus result meal = a
    damn result
}

fr fr Power function (integer exponent)
slay power_int(base normie, exp normie) normie {
    sus result normie = 1
    sus counter normie = 0
    sus condition lit = (counter < exp)
    sus temp normie = result * base
    damn result
}

slay power_float(base meal, exp normie) meal {
    sus result meal = 1.0
    sus counter normie = 0
    sus condition lit = (counter < exp)
    sus temp meal = result * base
    damn result
}

fr fr Square root approximation (Newton's method)
slay sqrt_float(x meal) meal {
    sus condition lit = (x < 0.0)
    sus guess meal = x / 2.0
    sus precision meal = 0.0001
    sus new_guess meal = (guess + x / guess) / 2.0
    sus diff meal = guess - new_guess
    sus abs_diff meal = diff
    sus continue_loop lit = (abs_diff > precision)
    damn guess
}

fr fr Number validation functions
slay is_positive_int(x normie) lit {
    sus result lit = (x > 0)
    damn result
}

slay is_negative_int(x normie) lit {
    sus result lit = (x < 0)
    damn result
}

slay is_zero_int(x normie) lit {
    sus result lit = (x == 0)
    damn result
}

slay is_positive_float(x meal) lit {
    sus result lit = (x > 0.0)
    damn result
}

slay is_negative_float(x meal) lit {
    sus result lit = (x < 0.0)
    damn result
}

slay is_zero_float(x meal) lit {
    sus result lit = (x == 0.0)
    damn result
}

fr fr Type conversion functions
slay int_to_float(x normie) meal {
    sus result meal = x.(meal)
    damn result
}

slay float_to_int(x meal) normie {
    sus result normie = x.(normie)
    damn result
}

fr fr Mathematical utility functions
slay factorial(n normie) normie {
    sus result normie = 1
    sus counter normie = 1
    sus condition lit = (counter <= n)
    sus temp normie = result * counter
    damn result
}

slay gcd(a normie, b normie) normie {
    sus temp_a normie = a
    sus temp_b normie = b
    sus condition lit = (temp_b != 0)
    sus remainder normie = temp_a % temp_b
    damn temp_a
}

slay lcm(a normie, b normie) normie {
    sus gcd_result normie = gcd(a, b)
    sus product normie = a * b
    sus result normie = product / gcd_result
    damn result
}
