// Simple Pure CURSED Math Float Module

// Mathematical Constants
slay PI() meal {
    damn 3.141592653589793;
}

slay E() meal {
    damn 2.718281828459045;
}

slay TAU() meal {
    damn 6.283185307179586;
}

// Basic Operations
slay abs_float(x meal) meal {
    sus result meal = x;
    bestie (x < 0.0) {
        result = -x;
    }
    damn result;
}

slay min_float(a meal, b meal) meal {
    sus result meal = a;
    bestie (b < a) {
        result = b;
    }
    damn result;
}

slay max_float(a meal, b meal) meal {
    sus result meal = a;
    bestie (b > a) {
        result = b;
    }
    damn result;
}

// Power Functions
slay sqrt_simple(x meal) meal {
    bestie (x < 0.0) {
        damn 0.0 / 0.0; // NaN
    }
    bestie (x == 0.0) {
        damn 0.0;
    }
    
    sus guess meal = x / 2.0;
    bestie i := 0; i < 10; i++ {
        guess = (guess + x / guess) / 2.0;
    }
    damn guess;
}

// Trigonometric Functions (simplified Taylor series)
slay sin_simple(x meal) meal {
    // Use simple Taylor series: sin(x) ≈ x - x³/6 + x⁵/120
    sus x2 meal = x * x;
    sus x3 meal = x2 * x;
    sus x5 meal = x3 * x2;
    damn x - x3 / 6.0 + x5 / 120.0;
}

slay cos_simple(x meal) meal {
    // Use simple Taylor series: cos(x) ≈ 1 - x²/2 + x⁴/24
    sus x2 meal = x * x;
    sus x4 meal = x2 * x2;
    damn 1.0 - x2 / 2.0 + x4 / 24.0;
}

// Exponential function (simplified)
slay exp_simple(x meal) meal {
    // Use simple Taylor series: e^x ≈ 1 + x + x²/2 + x³/6
    sus x2 meal = x * x;
    sus x3 meal = x2 * x;
    damn 1.0 + x + x2 / 2.0 + x3 / 6.0;
}

// Natural logarithm (simplified)
slay ln_simple(x meal) meal {
    bestie (x <= 0.0) {
        damn 0.0 / 0.0; // NaN
    }
    
    // Very simple approximation for ln(x) near x=1
    // ln(1+u) ≈ u - u²/2 + u³/3 for small u
    sus u meal = x - 1.0;
    sus u2 meal = u * u;
    sus u3 meal = u2 * u;
    damn u - u2 / 2.0 + u3 / 3.0;
}

// Utility functions
slay is_finite_simple(x meal) lit {
    // Simple check - not perfect but works for basic cases
    damn x == x && x != x + 1.0 && x != x * 2.0;
}

slay approximately_equal_simple(a meal, b meal, epsilon meal) lit {
    sus diff meal = abs_float(a - b);
    damn diff < epsilon;
}
