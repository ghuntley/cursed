// Standard math library

// ================================
// Constants
// ================================

fn math_pi() -> float {
    return 3.141592653589793;
}

fn math_e() -> float {
    return 2.718281828459045;
}

fn math_tau() -> float {
    return 6.283185307179586;
}

// ================================
// Basic operations
// ================================

fn math_abs(x: float) -> float {
    return math_abs_impl(x);
}

fn math_abs_int(x: int) -> int {
    return math_abs_int_impl(x);
}

fn math_min(a: float, b: float) -> float {
    return math_min_impl(a, b);
}

fn math_max(a: float, b: float) -> float {
    return math_max_impl(a, b);
}

fn math_min_int(a: int, b: int) -> int {
    return math_min_int_impl(a, b);
}

fn math_max_int(a: int, b: int) -> int {
    return math_max_int_impl(a, b);
}

fn math_clamp(x: float, min: float, max: float) -> float {
    return math_clamp_impl(x, min, max);
}

fn math_sign(x: float) -> float {
    return math_sign_impl(x);
}

// ================================
// Power and logarithm functions
// ================================

fn math_pow(base: float, exponent: float) -> float {
    return math_pow_impl(base, exponent);
}

fn math_sqrt(x: float) -> float {
    return math_sqrt_impl(x);
}

fn math_cbrt(x: float) -> float {
    return math_cbrt_impl(x);
}

fn math_log(x: float) -> float {
    return math_log_impl(x);
}

fn math_log10(x: float) -> float {
    return math_log10_impl(x);
}

fn math_log2(x: float) -> float {
    return math_log2_impl(x);
}

fn math_exp(x: float) -> float {
    return math_exp_impl(x);
}

fn math_exp2(x: float) -> float {
    return math_exp2_impl(x);
}

// ================================
// Trigonometric functions
// ================================

fn math_sin(x: float) -> float {
    return math_sin_impl(x);
}

fn math_cos(x: float) -> float {
    return math_cos_impl(x);
}

fn math_tan(x: float) -> float {
    return math_tan_impl(x);
}

fn math_asin(x: float) -> float {
    return math_asin_impl(x);
}

fn math_acos(x: float) -> float {
    return math_acos_impl(x);
}

fn math_atan(x: float) -> float {
    return math_atan_impl(x);
}

fn math_atan2(y: float, x: float) -> float {
    return math_atan2_impl(y, x);
}

fn math_sinh(x: float) -> float {
    return math_sinh_impl(x);
}

fn math_cosh(x: float) -> float {
    return math_cosh_impl(x);
}

fn math_tanh(x: float) -> float {
    return math_tanh_impl(x);
}

// ================================
// Rounding and truncation
// ================================

fn math_floor(x: float) -> float {
    return math_floor_impl(x);
}

fn math_ceil(x: float) -> float {
    return math_ceil_impl(x);
}

fn math_round(x: float) -> float {
    return math_round_impl(x);
}

fn math_trunc(x: float) -> float {
    return math_trunc_impl(x);
}

fn math_frac(x: float) -> float {
    return math_frac_impl(x);
}

// ================================
// Statistical functions
// ================================

fn math_sum(values: array) -> float {
    return math_sum_impl(values);
}

fn math_mean(values: array) -> float {
    return math_mean_impl(values);
}

fn math_median(values: array) -> float {
    return math_median_impl(values);
}

fn math_variance(values: array) -> float {
    return math_variance_impl(values);
}

fn math_std_dev(values: array) -> float {
    return math_std_dev_impl(values);
}

// ================================
// Random numbers
// ================================

fn math_random() -> float {
    return math_random_impl();
}

fn math_random_int(min: int, max: int) -> int {
    return math_random_int_impl(min, max);
}

fn math_random_float(min: float, max: float) -> float {
    return math_random_float_impl(min, max);
}

fn math_seed_random(seed: int) -> void {
    math_seed_random_impl(seed);
}

// ================================
// Utility functions
// ================================

fn math_is_nan(x: float) -> bool {
    return math_is_nan_impl(x);
}

fn math_is_infinite(x: float) -> bool {
    return math_is_infinite_impl(x);
}

fn math_is_finite(x: float) -> bool {
    return math_is_finite_impl(x);
}

fn math_degrees(radians: float) -> float {
    return radians * 180.0 / math_pi();
}

fn math_radians(degrees: float) -> float {
    return degrees * math_pi() / 180.0;
}

fn math_gcd(a: int, b: int) -> int {
    return math_gcd_impl(a, b);
}

fn math_lcm(a: int, b: int) -> int {
    return math_lcm_impl(a, b);
}

fn math_factorial(n: int) -> int {
    return math_factorial_impl(n);
}

fn math_fibonacci(n: int) -> int {
    return math_fibonacci_impl(n);
}

// ================================
// Linear interpolation
// ================================

fn math_lerp(a: float, b: float, t: float) -> float {
    return a + t * (b - a);
}

fn math_inverse_lerp(a: float, b: float, value: float) -> float {
    return (value - a) / (b - a);
}

fn math_smoothstep(edge0: float, edge1: float, x: float) -> float {
    return math_smoothstep_impl(edge0, edge1, x);
}

// ================================
// Distance and geometry
// ================================

fn math_distance_2d(x1: float, y1: float, x2: float, y2: float) -> float {
    return math_sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1));
}

fn math_distance_3d(x1: float, y1: float, z1: float, x2: float, y2: float, z2: float) -> float {
    return math_sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) + (z2 - z1) * (z2 - z1));
}

fn math_dot_product_2d(x1: float, y1: float, x2: float, y2: float) -> float {
    return x1 * x2 + y1 * y2;
}

fn math_cross_product_2d(x1: float, y1: float, x2: float, y2: float) -> float {
    return x1 * y2 - y1 * x2;
}

fn math_magnitude_2d(x: float, y: float) -> float {
    return math_sqrt(x * x + y * y);
}

fn math_normalize_2d(x: float, y: float) -> array {
    let mag = math_magnitude_2d(x, y);
    return [x / mag, y / mag];
}
