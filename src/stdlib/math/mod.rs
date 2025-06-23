/// Mathematics module for CURSED programming language
/// 
/// Provides comprehensive mathematical functions including basic arithmetic,
/// trigonometry, logarithms, special functions, constants, random number generation,
/// statistics, and advanced mathematical utilities.
/// 
/// This module integrates all mathematical capabilities into a unified, cohesive
/// library that's optimized for performance and ease of use.

use std::fmt;

// Core mathematical modules
pub mod basic;
pub mod trigonometry;
pub mod logarithmic;
pub mod constants;
pub mod special;
pub mod random;
pub mod statistics;
pub mod utilities;
pub mod complex;
pub mod advanced;
pub mod matrix;
pub mod big_mood;

// Re-export core mathematical functions with explicit imports to avoid conflicts
// BASIC OPERATIONS - Fundamental arithmetic and utility functions
pub use basic::{
    // Basic arithmetic operations
    abs, min, max, clamp, sign, 
    // Rounding and precision functions
    floor, ceil, round, math_truncate, fract,
    // Integer arithmetic
    remainder, modulo, gcd, lcm, is_even, is_odd,
    // Interpolation and smoothing
    lerp, inverse_lerp, smooth_step, smoother_step,
    // Type-specific operations
    abs_i32, abs_i64, min_i32, max_i32, clamp_i32, min_i64, max_i64, clamp_i64,
    // Power and utility functions
    pow2, pow10, reciprocal, is_zero, is_equal, round_to_decimals, map_range,
    // Statistical basics (for two values)
    average, geometric_mean as basic_geometric_mean, harmonic_mean as basic_harmonic_mean,
    // These functions exist in both basic and logarithmic modules - use basic for simple cases
    pow as basic_pow, sqrt as basic_sqrt, cbrt as basic_cbrt, 
    nth_root as basic_nth_root, hypot as basic_hypot, square as basic_square, 
    cube as basic_cube,
};

// TRIGONOMETRIC FUNCTIONS - Complete trigonometric operations
pub use trigonometry::{
    // Primary trigonometric functions
    sin, cos, tan, asin, acos, atan, atan2,
    // Hyperbolic functions
    sinh, cosh, tanh, asinh, acosh, atanh,
    // Angle conversion utilities
    degrees_to_radians, radians_to_degrees, deg_to_rad, rad_to_deg,
    // Degree-based convenience functions
    sin_deg, cos_deg, tan_deg,
    // Reciprocal trigonometric functions
    sec, csc, cot,
    // Angle normalization
    normalize_angle, normalize_angle_signed,
};

// LOGARITHMIC & EXPONENTIAL FUNCTIONS - Advanced mathematical operations
pub use logarithmic::{
    // Core logarithmic functions
    ln, log10, log2, log, expm1, ln1p,
    // Enhanced exponential functions
    exp, exp2, exp10, exp2m1, exp10m1, exp_base,
    // Advanced power functions (primary implementations for complex operations)
    pow, powi, pow_e, pow_2, pow_10, tetration,
    // Root functions (primary implementations)
    sqrt, cbrt, nth_root, hypot, hypot3,
    // Mathematical utility functions
    square, cube, mul_add, inv_sqrt, ln_gamma,
    // Logarithmic transformations and utilities
    log2_abs, log10_abs, ln_abs, log_mean, sigmoid, logistic,
    softmax_single, log_sum_exp,
    // Enhanced domain validation and safety
    is_valid_log_input, is_valid_exp_input, safe_ln, safe_exp,
    clamped_ln, clamped_exp,
};

// MATHEMATICAL CONSTANTS - Fundamental mathematical values
pub use constants::{
    // Primary constants
    PI, TAU, E, PHI, INV_PHI, EULER_GAMMA,
    // Pi-related constants
    FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8,
    FRAC_1_PI, FRAC_2_PI, FRAC_2_SQRT_PI,
    // Square roots
    SQRT_2, FRAC_1_SQRT_2, SQRT_3, SQRT_5, SQRT_PI,
    // Logarithmic constants
    LN_2, LN_10, LOG2_E, LOG2_10, LOG10_E, LOG10_2,
    // Special mathematical constants
    CATALAN, APERY, CONWAY, KHINCHIN, GLAISHER, FEIGENBAUM_DELTA, FEIGENBAUM_ALPHA,
    TWIN_PRIME, MEISSEL_MERTENS, BRUN_TWIN_PRIMES, CHAMPERNOWNE, PLASTIC,
    // Conversion factors
    DEG_TO_RAD, RAD_TO_DEG,
    // Physical constants
    SPEED_OF_LIGHT, PLANCK, HBAR, AVOGADRO, BOLTZMANN, GAS_CONSTANT,
    GRAVITATIONAL, ELEMENTARY_CHARGE, ELECTRON_MASS, PROTON_MASS, FINE_STRUCTURE,
    // Additional physical constants
    ATM_PRESSURE, STANDARD_GRAVITY,
    // Floating point limits and properties
    EPSILON, MIN_POSITIVE, MAX, MIN, INFINITY, NEG_INFINITY, NAN,
    MANTISSA_DIGITS, DIGITS, MAX_EXP, MIN_EXP, MAX_10_EXP, MIN_10_EXP, RADIX,
    // Common fractions
    ONE_THIRD, TWO_THIRDS, ONE_SIXTH, FIVE_SIXTHS, ONE_SEVENTH, ONE_NINTH,
    ONE_TWELFTH, THREE_QUARTERS, FIVE_EIGHTHS, SEVEN_EIGHTHS,
    // Unit conversion constants
    INCH_TO_CM, CM_TO_INCH, FOOT_TO_METER, METER_TO_FOOT, MILE_TO_KM, KM_TO_MILE,
    POUND_TO_KG, KG_TO_POUND, FAHRENHEIT_OFFSET, FAHRENHEIT_SCALE, CELSIUS_SCALE,
    // Constant collections
    FUNDAMENTAL_CONSTANTS, PHYSICAL_CONSTANTS, COMMON_FRACTIONS,
    // Utility functions
    is_close_to_constant, is_approximately_pi, is_approximately_e, is_approximately_phi,
    find_closest_constant,
    fahrenheit_to_celsius, celsius_to_fahrenheit, inches_to_cm, cm_to_inches,
    miles_to_km, km_to_miles, pounds_to_kg, kg_to_pounds,
    list_fundamental_constants, list_physical_constants, validate_constant_calculation,
};

// RANDOM NUMBER GENERATION - Comprehensive random utilities
pub use random::{
    // Basic random functions
    random, random_range, random_int, random_u64, random_bool,
    // Collection utilities
    choice, choices, weighted_choice, shuffle, shuffled, sample,
    // String and byte generation
    random_bytes, random_string, random_alphanumeric, random_hex,
    // Seeding and control
    set_seed,
    // Statistical distributions
    random_normal, random_exponential, random_uniform, random_poisson,
    random_beta, random_gamma,
};

// STATISTICAL FUNCTIONS - Data analysis and statistics
pub use statistics::{
    // Descriptive statistics
    mean, median, mode, variance, sample_variance, standard_deviation, sample_standard_deviation,
    skewness, kurtosis, harmonic_mean, geometric_mean, root_mean_square, coefficient_of_variation,
    // Quantiles and percentiles
    percentile, q1, q3, five_number_summary,
    // Range and spread measures
    range, interquartile_range,
    // Probability density and distribution functions
    normal_pdf, standard_normal_cdf, normal_cdf, uniform_pdf, uniform_cdf,
    exponential_pdf, exponential_cdf, binomial_pmf, poisson_pmf,
    // Statistical analysis
    covariance, sample_covariance, correlation, spearman_correlation,
    // Statistical tests
    t_test_one_sample, t_test_two_sample, chi_square_test, anova_one_way,
    // Regression analysis
    linear_regression, multiple_linear_regression,
    // Data validation and cleaning
    outliers_iqr, outliers_z_score, has_invalid_values, clean_data, validate_dataset,
};

// SPECIAL FUNCTIONS - Advanced mathematical functions
pub use special::{
    // Factorial variants (special module implementations)
    factorial as special_factorial, factorial_f64, 
    // Gamma and beta functions
    gamma, beta, 
    // Combinatorial functions
    binomial, binomial_f64, permutations as special_permutations,
    // Number sequences
    fibonacci as special_fibonacci, lucas, catalan,
    // Error functions
    erf, erfc, erf_inv,
    // Bessel functions
    bessel_j0, bessel_j1, bessel_y0, bessel_y1,
};

// MATHEMATICAL UTILITIES - Advanced computational mathematics
pub use utilities::{
    // Number theory
    extended_gcd, is_prime, sieve_of_eratosthenes, prime_factorization, next_prime, euler_totient,
    // Combinatorics (primary implementations from utilities)
    factorial, double_factorial, factorial_stirling, permutations, combinations, 
    binomial_coefficient, multicombinations, catalan_number,
    // Advanced special functions
    gamma_function, beta_function, error_function, complementary_error_function,
    // Numerical analysis methods
    simpson_integration, numerical_derivative, newton_raphson, bisection_method,
    // Sequences and series
    fibonacci, lucas_number, tribonacci, factorial_sequence_sum, harmonic_number,
    // Modular arithmetic and conversions
    mod_pow, mod_inverse, convert_base, gcd_multiple, lcm_multiple,
    // Advanced mathematical utilities
    FibonacciMemo, is_perfect_number, digital_root,
};

// COMPLEX NUMBERS - Comprehensive complex number mathematics
pub use complex::{
    // Complex number type and creation
    Complex64, complex, real, imag,
    // Basic complex operations
    abs as complex_abs, phase, conj, polar, rect,
    // Complex exponential and logarithmic functions
    exp as complex_exp, log as complex_log, log10 as complex_log10,
    pow as complex_pow, sqrt as complex_sqrt,
    // Complex trigonometric functions
    sin as complex_sin, cos as complex_cos, tan as complex_tan,
    sinh as complex_sinh, cosh as complex_cosh, tanh as complex_tanh,
    // Complex inverse trigonometric functions
    asin as complex_asin, acos as complex_acos, atan as complex_atan,
    asinh as complex_asinh, acosh as complex_acosh, atanh as complex_atanh,
    // Complex vector and matrix operations
    vector_sum as complex_vector_sum, vector_product as complex_vector_product,
    matrix_mul_2x2 as complex_matrix_mul_2x2, determinant_2x2 as complex_determinant_2x2,
    // Complex polynomial operations
    quadratic_roots, evaluate_polynomial as complex_evaluate_polynomial,
};

// ADVANCED MATHEMATICAL FUNCTIONS - Sophisticated numerical methods and algorithms
pub use advanced::{
    // Advanced numerical methods (with prefixes to avoid conflicts)
    numerical_derivative as advanced_numerical_derivative, 
    adaptive_simpson_integration, 
    multidimensional_newton_raphson,
    // Optimization algorithms
    golden_section_search, gradient_descent,
    // Fourier transforms and signal processing
    ComplexNumber, fast_fourier_transform, inverse_fast_fourier_transform,
    convolution_filter, moving_average_filter,
    // Interpolation and approximation
    cubic_spline_interpolation, chebyshev_approximation,
    // Matrix operations
    solve_linear_system, matrix_determinant,
    // Mathematical modeling
    polynomial_fit, evaluate_polynomial as advanced_evaluate_polynomial,
};

// MATRIX OPERATIONS - Linear algebra and matrix computations
pub use matrix::{
    // Matrix structure and creation
    Matrix, matrix_add, matrix_subtract, matrix_multiply, matrix_scalar_multiply, matrix_vector_multiply,
    // Matrix decompositions
    LuDecomposition, lu_decomposition, QrDecomposition, qr_decomposition,
    // Eigenvalue computations
    EigenDecomposition, eigen_decomposition,
    // Matrix utilities
    vector_dot, vector_norm, matrix_norm, matrix_inverse,
};

// BIG MOOD - Arbitrary precision arithmetic operations
pub use big_mood::{
    // Core arbitrary precision types
    BigInt, BigRat, BigFloat, Decimal, BigComplex,
    // Accuracy and rounding control
    Accuracy, RoundingMode,
    // Parsing functions
    parse_int, parse_rat, parse_float,
    // Utility functions
    gcd as big_gcd, binomial as big_binomial,
    // Mathematical functions for big numbers
    sqrt as big_sqrt, cbrt as big_cbrt, nth_root as big_nth_root,
    ln as big_ln, exp as big_exp, sin as big_sin, cos as big_cos, tan as big_tan,
    // Performance optimizations
    fast_mul, rand_prime,
};

/// Error types for mathematical operations
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Domain error: input value outside valid domain
    DomainError { function: String, value: f64, message: String },
    /// Range error: result would be outside representable range
    RangeError { function: String, message: String },
    /// Overflow error: result too large to represent
    Overflow { function: String, value: f64 },
    /// Underflow error: result too small to represent
    Underflow { function: String, value: f64 },
    /// Division by zero
    DivisionByZero { function: String },
    /// Invalid input parameter
    InvalidInput { function: String, parameter: String, value: f64 },
    /// Negative input to function requiring positive values
    NegativeInput { function: String, value: f64 },
    /// Integer overflow in discrete math functions
    IntegerOverflow { function: String, value: i64 },
    /// General mathematical computation error
    ComputationError { function: String, message: String },
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DomainError { function, value, message } => {
                write!(f, "Domain error in {}: value {} - {}", function, value, message)
            }
            MathError::RangeError { function, message } => {
                write!(f, "Range error in {}: {}", function, message)
            }
            MathError::Overflow { function, value } => {
                write!(f, "Overflow in {}: value {} is too large", function, value)
            }
            MathError::Underflow { function, value } => {
                write!(f, "Underflow in {}: value {} is too small", function, value)
            }
            MathError::DivisionByZero { function } => {
                write!(f, "Division by zero in {}", function)
            }
            MathError::InvalidInput { function, parameter, value } => {
                write!(f, "Invalid input in {}: parameter {} = {}", function, parameter, value)
            }
            MathError::NegativeInput { function, value } => {
                write!(f, "Negative input in {}: {} (positive value required)", function, value)
            }
            MathError::IntegerOverflow { function, value } => {
                write!(f, "Integer overflow in {}: value {}", function, value)
            }
            MathError::ComputationError { function, message } => {
                write!(f, "Computation error in {}: {}", function, message)
            }
        }
    }
}

impl std::error::Error for MathError {}

/// Result type for mathematical operations
pub type MathResult<(), Error>;

/// Helper function to create domain errors
pub fn domain_error(function: &str, value: f64, message: &str) -> MathError {
    MathError::DomainError {
        function: function.to_string(),
        value,
        message: message.to_string(),
    }
}

/// Helper function to create range errors
pub fn range_error(function: &str, message: &str) -> MathError {
    MathError::RangeError {
        function: function.to_string(),
        message: message.to_string(),
    }
}

/// Helper function to create division by zero errors
pub fn division_by_zero_error(function: &str) -> MathError {
    MathError::DivisionByZero {
        function: function.to_string(),
    }
}

/// Helper function to create negative input errors
pub fn negative_input_error(function: &str, value: f64) -> MathError {
    MathError::NegativeInput {
        function: function.to_string(),
        value,
    }
}

/// Helper function to check if a floating point value is valid (not NaN or infinite)
pub fn is_valid_float(value: f64) -> bool {
    value.is_finite()
}

/// Helper function to validate floating point inputs
pub fn validate_float(function: &str, parameter: &str, value: f64) -> MathResult<()> {
    if value.is_nan() {
        Err(MathError::InvalidInput {
            function: function.to_string(),
            parameter: parameter.to_string(),
            value,
        })
    } else if value.is_infinite() {
        Err(MathError::RangeError {
            function: function.to_string(),
            message: format!("Parameter {} is infinite", parameter),
        })
    } else {
        Ok(())
    }
}
