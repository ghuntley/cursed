# mathz - Essential Mathematics Module
# Clean implementation with only working functions

# Mathematical Constants
sus PI meal = 3.141592653589793
sus E meal = 2.718281828459045
sus TAU meal = 6.283185307179586

# Absolute Value
slay abs_meal(x meal) meal {
    lowkey x < 0.0 {
        damn -x
    }
    damn x
}

slay abs_normie(x normie) normie {
    lowkey x < 0 {
        damn -x
    }
    damn x
}

# Maximum of two values
slay max_meal(a meal, b meal) meal {
    lowkey a > b {
        damn a
    }
    damn b
}

slay max_normie(a normie, b normie) normie {
    lowkey a > b {
        damn a
    }
    damn b
}

# Minimum of two values
slay min_meal(a meal, b meal) meal {
    lowkey a < b {
        damn a
    }
    damn b
}

slay min_normie(a normie, b normie) normie {
    lowkey a < b {
        damn a
    }
    damn b
}

# Power function (integer exponent only for simplicity)
slay pow_meal(base meal, exp normie) meal {
    sus result meal = 1.0
    sus i normie = 0
    bestie (i < exp) {
        result = result * base
        i = i + 1
    }
    damn result
}

# Square root (simple Newton's method)
slay sqrt_meal(x meal) meal {
    lowkey x < 0.0 {
        damn 0.0
    }
    lowkey x == 0.0 {
        damn 0.0
    }
    
    sus guess meal = x / 2.0
    sus prev meal = 0.0
    sus diff meal = 1.0
    
    bestie (diff > 0.000001) {
        prev = guess
        guess = (guess + (x / guess)) / 2.0
        diff = abs_meal(guess - prev)
    }
    
    damn guess
}

# Simple utility functions
slay is_positive_meal(x meal) lit {
    damn x > 0.0
}

slay is_negative_meal(x meal) lit {
    damn x < 0.0
}

slay is_zero_meal(x meal) lit {
    damn x == 0.0
}
