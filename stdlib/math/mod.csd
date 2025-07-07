// Standard math library

// ================================
// Constants
// ================================

slay math_pi() meal {
    damn 3.141592653589793;
}

slay math_e() meal {
    damn 2.718281828459045;
}

slay math_tau() meal {
    damn 6.283185307179586;
}

// ================================
// Basic operations
// ================================

slay math_abs(x meal) meal {
    damn math_abs_impl(x);
}

slay math_abs_int(x normie) normie {
    damn math_abs_int_impl(x);
}

slay math_min(a meal, b meal) meal {
    damn math_min_impl(a, b);
}

slay math_max(a meal, b meal) meal {
    damn math_max_impl(a, b);
}

slay math_min_int(a normie, b normie) normie {
    damn math_min_int_impl(a, b);
}

slay math_max_int(a normie, b normie) normie {
    damn math_max_int_impl(a, b);
}

slay math_clamp(x meal, min meal, max meal) meal {
    damn math_clamp_impl(x, min, max);
}

slay math_sign(x meal) meal {
    damn math_sign_impl(x);
}

// ================================
// Power and logarithm functions
// ================================

slay math_pow(base meal, exponent meal) meal {
    damn math_pow_impl(base, exponent);
}

slay math_sqrt(x meal) meal {
    damn math_sqrt_impl(x);
}

slay math_cbrt(x meal) meal {
    damn math_cbrt_impl(x);
}

slay math_log(x meal) meal {
    damn math_log_impl(x);
}

slay math_log10(x meal) meal {
    damn math_log10_impl(x);
}

slay math_log2(x meal) meal {
    damn math_log2_impl(x);
}

slay math_exp(x meal) meal {
    damn math_exp_impl(x);
}

slay math_exp2(x meal) meal {
    damn math_exp2_impl(x);
}

// ================================
// Trigonometric functions
// ================================

slay math_sin(x meal) meal {
    damn math_sin_impl(x);
}

slay math_cos(x meal) meal {
    damn math_cos_impl(x);
}

slay math_tan(x meal) meal {
    damn math_tan_impl(x);
}

slay math_asin(x meal) meal {
    damn math_asin_impl(x);
}

slay math_acos(x meal) meal {
    damn math_acos_impl(x);
}

slay math_atan(x meal) meal {
    damn math_atan_impl(x);
}

slay math_atan2(y meal, x meal) meal {
    damn math_atan2_impl(y, x);
}

slay math_sinh(x meal) meal {
    damn math_sinh_impl(x);
}

slay math_cosh(x meal) meal {
    damn math_cosh_impl(x);
}

slay math_tanh(x meal) meal {
    damn math_tanh_impl(x);
}

// ================================
// Rounding and truncation
// ================================

slay math_floor(x meal) meal {
    damn math_floor_impl(x);
}

slay math_ceil(x meal) meal {
    damn math_ceil_impl(x);
}

slay math_round(x meal) meal {
    damn math_round_impl(x);
}

slay math_trunc(x meal) meal {
    damn math_trunc_impl(x);
}

slay math_frac(x meal) meal {
    damn math_frac_impl(x);
}

// ================================
// Statistical functions
// ================================

slay math_sum(values [meal]) meal {
    damn math_sum_impl(values);
}

slay math_mean(values [meal]) meal {
    damn math_mean_impl(values);
}

slay math_median(values [meal]) meal {
    damn math_median_impl(values);
}

slay math_variance(values [meal]) meal {
    damn math_variance_impl(values);
}

slay math_std_dev(values [meal]) meal {
    damn math_std_dev_impl(values);
}

// ================================
// Random numbers
// ================================

slay math_random() meal {
    damn math_random_impl();
}

slay math_random_int(min normie, max normie) normie {
    damn math_random_int_impl(min, max);
}

slay math_random_float(min meal, max meal) meal {
    damn math_random_float_impl(min, max);
}

slay math_seed_random(seed normie) {
    math_seed_random_impl(seed);
}

// ================================
// Utility functions
// ================================

slay math_is_nan(x meal) lit {
    damn math_is_nan_impl(x);
}

slay math_is_infinite(x meal) lit {
    damn math_is_infinite_impl(x);
}

slay math_is_finite(x meal) lit {
    damn math_is_finite_impl(x);
}

slay math_degrees(radians meal) meal {
    damn radians * 180.0 / math_pi();
}

slay math_radians(degrees meal) meal {
    damn degrees * math_pi() / 180.0;
}

slay math_gcd(a normie, b normie) normie {
    damn math_gcd_impl(a, b);
}

slay math_lcm(a normie, b normie) normie {
    damn math_lcm_impl(a, b);
}

slay math_factorial(n normie) normie {
    damn math_factorial_impl(n);
}

slay math_fibonacci(n normie) normie {
    damn math_fibonacci_impl(n);
}

// ================================
// Linear interpolation
// ================================

slay math_lerp(a meal, b meal, t meal) meal {
    damn a + t * (b - a);
}

slay math_inverse_lerp(a meal, b meal, value meal) meal {
    damn (value - a) / (b - a);
}

slay math_smoothstep(edge0 meal, edge1 meal, x meal) meal {
    damn math_smoothstep_impl(edge0, edge1, x);
}

// ================================
// Distance and geometry
// ================================

slay math_distance_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    damn math_sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1));
}

slay math_distance_3d(x1 meal, y1 meal, z1 meal, x2 meal, y2 meal, z2 meal) meal {
    damn math_sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) + (z2 - z1) * (z2 - z1));
}

slay math_dot_product_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    damn x1 * x2 + y1 * y2;
}

slay math_cross_product_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    damn x1 * y2 - y1 * x2;
}

slay math_magnitude_2d(x meal, y meal) meal {
    damn math_sqrt(x * x + y * y);
}

slay math_normalize_2d(x meal, y meal) [meal] {
    sus mag meal = math_magnitude_2d(x, y);
    damn [x / mag, y / mag];
}
