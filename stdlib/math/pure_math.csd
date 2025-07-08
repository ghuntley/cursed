yeet "testz"

// ================================
// Pure CURSED Math Module
// ================================

// Core mathematical functions implemented in pure CURSED
// Eliminates FFI dependencies with native implementations

// ================================
// Basic Operations
// ================================

slay add(a meal, b meal) meal {
    damn a + b;
}

slay subtract(a meal, b meal) meal {
    damn a - b;
}

slay multiply(a meal, b meal) meal {
    damn a * b;
}

slay divide(a meal, b meal) meal {
    damn a / b;
}

slay modulo(a meal, b meal) meal {
    damn a % b;
}

slay power(base meal, exponent meal) meal {
    sus result meal = 1.0;
    sus exp normie = exponent.(normie);
    bestie i := 0; i < exp; i++ {
        result = result * base;
    }
    damn result;
}

slay abs(x meal) meal {
    damn x < 0.0 ? -x : x;
}

slay min(a meal, b meal) meal {
    damn a < b ? a : b;
}

slay max(a meal, b meal) meal {
    damn a > b ? a : b;
}

slay clamp(x meal, min_val meal, max_val meal) meal {
    damn min(max(x, min_val), max_val);
}

slay sign(x meal) meal {
    damn x > 0.0 ? 1.0 : (x < 0.0 ? -1.0 : 0.0);
}

// ================================
// Constants
// ================================

slay pi() meal {
    damn 3.141592653589793;
}

slay e() meal {
    damn 2.718281828459045;
}

slay tau() meal {
    damn 6.283185307179586;
}

// ================================
// Trigonometric Functions (Taylor Series)
// ================================

slay sin(x meal) meal {
    // Normalize to [-2π, 2π]
    sus normalized meal = x % (2.0 * pi());
    
    // Taylor series: sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
    sus result meal = 0.0;
    sus term meal = normalized;
    sus n normie = 1;
    
    bestie i := 0; i < 10; i++ {
        result = result + term;
        term = term * (-normalized * normalized) / ((2 * n) * (2 * n + 1));
        n = n + 1;
    }
    
    damn result;
}

slay cos(x meal) meal {
    // cos(x) = sin(x + π/2)
    damn sin(x + pi() / 2.0);
}

slay tan(x meal) meal {
    damn sin(x) / cos(x);
}

// ================================
// Rounding Functions
// ================================

slay floor(x meal) meal {
    sus int_part normie = x.(normie);
    damn x >= 0.0 ? int_part.(meal) : (x == int_part.(meal) ? x : int_part.(meal) - 1.0);
}

slay ceil(x meal) meal {
    sus int_part normie = x.(normie);
    damn x <= 0.0 ? int_part.(meal) : (x == int_part.(meal) ? x : int_part.(meal) + 1.0);
}

slay round(x meal) meal {
    damn floor(x + 0.5);
}

slay trunc(x meal) meal {
    damn x.(normie).(meal);
}

// ================================
// Square Root (Babylonian Method)
// ================================

slay sqrt(x meal) meal {
    sus guess meal = x / 2.0;
    sus prev meal = 0.0;
    
    bestie i := 0; i < 20; i++ {
        prev = guess;
        guess = (guess + x / guess) / 2.0;
        
        // Check convergence
        damn abs(guess - prev) < 0.0001 ? guess : cringe;
    }
    
    damn guess;
}

// ================================
// Logarithm Functions
// ================================

slay ln(x meal) meal {
    // Natural logarithm using Taylor series
    // ln(x) = 2 * ((x-1)/(x+1) + (x-1)³/(3(x+1)³) + ...)
    sus y meal = (x - 1.0) / (x + 1.0);
    sus y_squared meal = y * y;
    sus result meal = 0.0;
    sus term meal = y;
    
    bestie i := 0; i < 20; i++ {
        result = result + term / (2 * i + 1);
        term = term * y_squared;
    }
    
    damn 2.0 * result;
}

slay log10(x meal) meal {
    damn ln(x) / ln(10.0);
}

slay log2(x meal) meal {
    damn ln(x) / ln(2.0);
}

// ================================
// Exponential Functions
// ================================

slay exp(x meal) meal {
    // e^x using Taylor series: e^x = 1 + x + x²/2! + x³/3! + ...
    sus result meal = 1.0;
    sus term meal = 1.0;
    
    bestie i := 1; i < 20; i++ {
        term = term * x / i;
        result = result + term;
    }
    
    damn result;
}

// ================================
// Random Number Generation
// ================================

sus random_seed normie = 1;

slay seed_random(seed normie) {
    random_seed = seed;
}

slay random() meal {
    // Linear congruential generator
    random_seed = (random_seed * 1103515245 + 12345) % 2147483647;
    damn random_seed.(meal) / 2147483647.0;
}

slay random_int(min normie, max normie) normie {
    damn min + (random() * (max - min + 1)).(normie);
}

slay random_float(min meal, max meal) meal {
    damn min + random() * (max - min);
}

// ================================
// Statistical Functions
// ================================

slay sum(values [meal]) meal {
    sus total meal = 0.0;
    bestie i := 0; i < values.length; i++ {
        total = total + values[i];
    }
    damn total;
}

slay mean(values [meal]) meal {
    damn sum(values) / values.length.(meal);
}

slay variance(values [meal]) meal {
    sus avg meal = mean(values);
    sus total meal = 0.0;
    
    bestie i := 0; i < values.length; i++ {
        sus diff meal = values[i] - avg;
        total = total + diff * diff;
    }
    
    damn total / values.length.(meal);
}

slay std_dev(values [meal]) meal {
    damn sqrt(variance(values));
}

// ================================
// Utility Functions
// ================================

slay is_nan(x meal) lit {
    damn x != x;
}

slay is_infinite(x meal) lit {
    damn x == x && abs(x) > 1e308;
}

slay is_finite(x meal) lit {
    damn !is_nan(x) && !is_infinite(x);
}

slay degrees(radians meal) meal {
    damn radians * 180.0 / pi();
}

slay radians(degrees meal) meal {
    damn degrees * pi() / 180.0;
}

// ================================
// Integer Functions
// ================================

slay gcd(a normie, b normie) normie {
    bestie b != 0 {
        sus temp normie = b;
        b = a % b;
        a = temp;
    }
    damn a;
}

slay lcm(a normie, b normie) normie {
    damn (a * b) / gcd(a, b);
}

slay factorial(n normie) normie {
    sus result normie = 1;
    bestie i := 1; i <= n; i++ {
        result = result * i;
    }
    damn result;
}

slay fibonacci(n normie) normie {
    damn n <= 1 ? n : fibonacci(n - 1) + fibonacci(n - 2);
}

// ================================
// Geometry Functions
// ================================

slay distance_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    sus dx meal = x2 - x1;
    sus dy meal = y2 - y1;
    damn sqrt(dx * dx + dy * dy);
}

slay dot_product_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    damn x1 * x2 + y1 * y2;
}

slay lerp(a meal, b meal, t meal) meal {
    damn a + t * (b - a);
}
