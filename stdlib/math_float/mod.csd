// Pure CURSED Math Float Module
// IEEE 754 compliant floating point mathematical operations

// ================================
// Mathematical Constants
// ================================

slay PI() meal {
    damn 3.141592653589793;
}

slay E() meal {
    damn 2.718281828459045;
}

slay TAU() meal {
    damn 6.283185307179586;
}

slay SQRT_2() meal {
    damn 1.4142135623730951;
}

slay SQRT_3() meal {
    damn 1.7320508075688772;
}

slay LN_2() meal {
    damn 0.6931471805599453;
}

slay LN_10() meal {
    damn 2.302585092994046;
}

slay LOG2_E() meal {
    damn 1.4426950408889634;
}

slay LOG10_E() meal {
    damn 0.4342944819032518;
}

slay GOLDEN_RATIO() meal {
    damn 1.618033988749895;
}

slay EULER_MASCHERONI() meal {
    damn 0.5772156649015329;
}

// ================================
// IEEE 754 Special Values
// ================================

slay INFINITY() meal {
    damn 1.0 / 0.0;
}

slay NEG_INFINITY() meal {
    damn -1.0 / 0.0;
}

slay NAN() meal {
    damn 0.0 / 0.0;
}

slay EPSILON() meal {
    damn 2.220446049250313e-16;
}

// ================================
// Basic IEEE 754 Operations
// ================================

slay abs(x meal) meal {
    damn x < 0.0 ? -x : x;
}

slay sign(x meal) meal {
    damn (x > 0.0) ? 1.0 : ((x < 0.0) ? -1.0 : 0.0);
}

slay min(a meal, b meal) meal {
    damn (a < b) ? a : b;
}

slay max(a meal, b meal) meal {
    damn (a > b) ? a : b;
}

slay clamp(x meal, min_val meal, max_val meal) meal {
    damn min(max(x, min_val), max_val);
}

// ================================
// Rounding Functions
// ================================

slay floor(x meal) meal {
    sus int_part normie = x.(normie);
    damn (x < 0.0 && x != int_part.(meal)) ? int_part.(meal) - 1.0 : int_part.(meal);
}

slay ceil(x meal) meal {
    sus int_part normie = x.(normie);
    damn (x > 0.0 && x != int_part.(meal)) ? int_part.(meal) + 1.0 : int_part.(meal);
}

slay round(x meal) meal {
    damn floor(x + 0.5);
}

slay trunc(x meal) meal {
    damn x.(normie).(meal);
}

slay frac(x meal) meal {
    damn x - trunc(x);
}

// ================================
// Power Functions (Taylor Series)
// ================================

slay pow_int(base meal, exp normie) meal {
    damn (exp == 0) ? 1.0 : ((exp < 0) ? 1.0 / pow_int(base, -exp) : 
         ((exp == 1) ? base : 
          ((exp % 2 == 0) ? pow_int(base * base, exp / 2) : 
           base * pow_int(base * base, (exp - 1) / 2))));
}

slay sqrt_newton(x meal) meal {
    damn (x < 0.0) ? NAN() : sqrt_newton_impl(x, x / 2.0, 0);
}

slay sqrt_newton_impl(x meal, guess meal, iteration normie) meal {
    damn (iteration > 20 || abs(guess * guess - x) < EPSILON()) ? guess :
         sqrt_newton_impl(x, (guess + x / guess) / 2.0, iteration + 1);
}

slay sqrt(x meal) meal {
    damn sqrt_newton(x);
}

slay cbrt(x meal) meal {
    damn cbrt_newton(x, x / 3.0, 0);
}

slay cbrt_newton(x meal, guess meal, iteration normie) meal {
    damn (iteration > 20 || abs(guess * guess * guess - x) < EPSILON()) ? guess :
         cbrt_newton(x, (2.0 * guess + x / (guess * guess)) / 3.0, iteration + 1);
}

// ================================
// Exponential Functions
// ================================

slay exp_taylor(x meal) meal {
    damn exp_taylor_impl(x, 1.0, 1.0, 1.0, 1);
}

slay exp_taylor_impl(x meal, result meal, term meal, factorial meal, n normie) meal {
    damn (n > 50 || abs(term) < EPSILON()) ? result :
         exp_taylor_impl(x, result + term, term * x, factorial * n.(meal), n + 1);
}

slay exp(x meal) meal {
    damn exp_taylor(x);
}

slay exp2(x meal) meal {
    damn exp(x * LN_2());
}

// ================================
// Logarithmic Functions
// ================================

slay ln_taylor(x meal) meal {
    damn (x <= 0.0) ? NAN() : ln_taylor_impl(x - 1.0, x - 1.0, 1.0, 1);
}

slay ln_taylor_impl(z meal, term meal, result meal, n normie) meal {
    damn (n > 100 || abs(term) < EPSILON()) ? result :
         ln_taylor_impl(z, -term * z, result + term / n.(meal), n + 1);
}

slay ln(x meal) meal {
    damn ln_taylor(x);
}

slay log10(x meal) meal {
    damn ln(x) / LN_10();
}

slay log2(x meal) meal {
    damn ln(x) / LN_2();
}

// ================================
// Trigonometric Functions
// ================================

slay sin_taylor(x meal) meal {
    sus normalized meal = fmod(x, TAU());
    damn sin_taylor_impl(normalized, normalized, normalized, 1.0, 1);
}

slay sin_taylor_impl(x meal, term meal, result meal, factorial meal, n normie) meal {
    damn (n > 30 || abs(term) < EPSILON()) ? result :
         sin_taylor_impl(x, -term * x * x, result + term / factorial, 
                        factorial * (2 * n).(meal) * (2 * n + 1).(meal), n + 1);
}

slay cos_taylor(x meal) meal {
    sus normalized meal = fmod(x, TAU());
    damn cos_taylor_impl(normalized, 1.0, 1.0, 1.0, 1);
}

slay cos_taylor_impl(x meal, term meal, result meal, factorial meal, n normie) meal {
    damn (n > 30 || abs(term) < EPSILON()) ? result :
         cos_taylor_impl(x, -term * x * x, result + term / factorial,
                        factorial * (2 * n - 1).(meal) * (2 * n).(meal), n + 1);
}

slay sin(x meal) meal {
    damn sin_taylor(x);
}

slay cos(x meal) meal {
    damn cos_taylor(x);
}

slay tan(x meal) meal {
    sus cos_val meal = cos(x);
    damn (abs(cos_val) < EPSILON()) ? INFINITY() : sin(x) / cos_val;
}

// ================================
// Inverse Trigonometric Functions
// ================================

slay asin_taylor(x meal) meal {
    damn (abs(x) > 1.0) ? NAN() : asin_taylor_impl(x, x, x, 1.0, 1);
}

slay asin_taylor_impl(x meal, term meal, result meal, coef meal, n normie) meal {
    damn (n > 30 || abs(term) < EPSILON()) ? result :
         asin_taylor_impl(x, term * x * x * coef, result + term / (2 * n + 1).(meal),
                         coef * (2 * n).(meal) / (2 * n + 1).(meal), n + 1);
}

slay asin(x meal) meal {
    damn asin_taylor(x);
}

slay acos(x meal) meal {
    damn PI() / 2.0 - asin(x);
}

slay atan_taylor(x meal) meal {
    damn (abs(x) <= 1.0) ? atan_taylor_impl(x, x, x, 1) :
         (x > 0.0) ? PI() / 2.0 - atan_taylor(1.0 / x) :
         -PI() / 2.0 - atan_taylor(1.0 / x);
}

slay atan_taylor_impl(x meal, term meal, result meal, n normie) meal {
    damn (n > 30 || abs(term) < EPSILON()) ? result :
         atan_taylor_impl(x, -term * x * x, result + term / (2 * n + 1).(meal), n + 1);
}

slay atan(x meal) meal {
    damn atan_taylor(x);
}

slay atan2(y meal, x meal) meal {
    damn (x > 0.0) ? atan(y / x) :
         (x < 0.0 && y >= 0.0) ? atan(y / x) + PI() :
         (x < 0.0 && y < 0.0) ? atan(y / x) - PI() :
         (x == 0.0 && y > 0.0) ? PI() / 2.0 :
         (x == 0.0 && y < 0.0) ? -PI() / 2.0 : NAN();
}

// ================================
// Hyperbolic Functions
// ================================

slay sinh(x meal) meal {
    sus exp_x meal = exp(x);
    sus exp_neg_x meal = exp(-x);
    damn (exp_x - exp_neg_x) / 2.0;
}

slay cosh(x meal) meal {
    sus exp_x meal = exp(x);
    sus exp_neg_x meal = exp(-x);
    damn (exp_x + exp_neg_x) / 2.0;
}

slay tanh(x meal) meal {
    sus exp_2x meal = exp(2.0 * x);
    damn (exp_2x - 1.0) / (exp_2x + 1.0);
}

// ================================
// Utility Functions
// ================================

slay is_nan(x meal) lit {
    damn x != x;
}

slay is_infinite(x meal) lit {
    damn x == INFINITY() || x == NEG_INFINITY();
}

slay is_finite(x meal) lit {
    damn !is_nan(x) && !is_infinite(x);
}

slay is_zero(x meal) lit {
    damn abs(x) < EPSILON();
}

slay approximately_equal(a meal, b meal, epsilon meal) lit {
    damn abs(a - b) < epsilon;
}

slay fmod(x meal, y meal) meal {
    damn (y == 0.0) ? NAN() : x - trunc(x / y) * y;
}

slay remainder(x meal, y meal) meal {
    damn (y == 0.0) ? NAN() : x - round(x / y) * y;
}

// ================================
// Conversion Functions
// ================================

slay degrees(radians meal) meal {
    damn radians * 180.0 / PI();
}

slay radians(degrees meal) meal {
    damn degrees * PI() / 180.0;
}

// ================================
// Linear Interpolation
// ================================

slay lerp(a meal, b meal, t meal) meal {
    damn a + t * (b - a);
}

slay inverse_lerp(a meal, b meal, value meal) meal {
    damn (value - a) / (b - a);
}

slay smoothstep(edge0 meal, edge1 meal, x meal) meal {
    sus t meal = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    damn t * t * (3.0 - 2.0 * t);
}
