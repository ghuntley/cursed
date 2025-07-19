fr fr CURSED Math Library Comprehensive Demo
fr fr Demonstrates the complete mathematical capabilities of the CURSED standard library

yeet "stdlib::math"
yeet "stdlib::io"

fr fr Demonstrate basic mathematical operations
slay demonstrate_basic_math() {
    println("=== Basic Mathematical Operations ===")?;
    
    // Basic arithmetic
    facts a = 15.7;
    facts b = -8.3;
    
    printf("abs({}) = {}\n", &[a, abs(a)])?;
    printf("abs({}) = {}\n", &[b, abs(b)])?;
    printf("min({}, {}) = {}\n", &[a, b, min(a, b)])?;
    printf("max({}, {}) = {}\n", &[a, b, max(a, b)])?;
    printf("clamp({}, {}, {}) = {}\n", &[a, b, 20.0, clamp(a, b, 20.0)])?;
    
    // Rounding operations
    facts x = 3.14159;
    printf("floor({}) = {}\n", &[x, floor(x)])?;
    printf("ceil({}) = {}\n", &[x, ceil(x)])?;
    printf("round({}) = {}\n", &[x, round(x)])?;
    printf("round_to_decimals({}, 2) = {}\n", &[x, round_to_decimals(x, 2)])?;
    
    // Integer operations
    facts num1 = 48;
    facts num2 = 18;
    printf("gcd({}, {}) = {}\n", &[num1, num2, gcd(num1, num2)])?;
    printf("lcm({}, {}) = {}\n", &[num1, num2, lcm(num1, num2)])?;
    printf("is_even({}) = {}\n", &[num1, is_even(num1)])?;
    printf("is_odd({}) = {}\n", &[num2, is_odd(num2)])?;
    
    println("")?;
}

fr fr Demonstrate trigonometric functions
slay demonstrate_trigonometry() {
    println("=== Trigonometric Functions ===")?;
    
    // Basic trigonometric functions with radians
    facts angle_rad = PI / 4.0; // 45 degrees
    printf("sin(π/4) = {}\n", &[sin(angle_rad)])?;
    printf("cos(π/4) = {}\n", &[cos(angle_rad)])?;
    printf("tan(π/4) = {}\n", &[tan(angle_rad)])?;
    
    // Degree-based functions for convenience
    facts angle_deg = 45.0;
    printf("sin(45°) = {}\n", &[sin_deg(angle_deg)])?;
    printf("cos(45°) = {}\n", &[cos_deg(angle_deg)])?;
    printf("tan(45°) = {}\n", &[tan_deg(angle_deg)])?;
    
    // Inverse functions
    facts value = 0.707; // approximately sin(45°)
    printf("asin({}) = {} rad = {}°\n", &[value, asin(value), radians_to_degrees(asin(value))])?;
    printf("acos({}) = {} rad = {}°\n", &[value, acos(value), radians_to_degrees(acos(value))])?;
    
    // Hyperbolic functions
    facts x = 1.0;
    printf("sinh({}) = {}\n", &[x, sinh(x)])?;
    printf("cosh({}) = {}\n", &[x, cosh(x)])?;
    printf("tanh({}) = {}\n", &[x, tanh(x)])?;
    
    // Verify trigonometric identity: sin²(x) + cos²(x) = 1
    facts identity_check = pow(sin(angle_rad), 2.0) + pow(cos(angle_rad), 2.0);
    printf("sin²(π/4) + cos²(π/4) = {} (should be 1.0)\n", &[identity_check])?;
    
    println("")?;
}

fr fr Demonstrate logarithmic and exponential functions
slay demonstrate_logarithmic_exponential() {
    println("=== Logarithmic and Exponential Functions ===")?;
    
    // Basic logarithmic functions
    facts x = 100.0;
    printf("ln({}) = {}\n", &[x, ln(x)])?;
    printf("log10({}) = {}\n", &[x, log10(x)])?;
    printf("log2({}) = {}\n", &[x, log2(x)])?;
    printf("log({}, 3) = {}\n", &[x, 3.0, log(x, 3.0)])?;
    
    // Exponential functions
    facts y = 2.0;
    printf("exp({}) = {}\n", &[y, exp(y)])?;
    printf("exp2({}) = {}\n", &[y, exp2(y)])?;
    printf("exp10({}) = {}\n", &[y, exp10(y)])?;
    
    // Power functions
    facts base = 2.0;
    facts exponent = 8.0;
    printf("pow({}, {}) = {}\n", &[base, exponent, pow(base, exponent)])?;
    printf("powi({}, {}) = {}\n", &[base, exponent as i32, powi(base, exponent as i32)])?;
    
    // Root functions
    facts num = 64.0;
    printf("sqrt({}) = {}\n", &[num, sqrt(num)])?;
    printf("cbrt({}) = {}\n", &[num, cbrt(num)])?;
    printf("nth_root({}, 6) = {}\n", &[num, nth_root(num, 6.0)])?;
    
    // Pythagorean theorem
    facts a = 3.0;
    facts b = 4.0;
    printf("hypot({}, {}) = {}\n", &[a, b, hypot(a, b)])?;
    
    // Advanced logarithmic functions
    facts small_x = 0.1;
    printf("expm1({}) = {} (exp(x) - 1, precise for small x)\n", &[small_x, expm1(small_x)])?;
    printf("ln1p({}) = {} (ln(1 + x), precise for small x)\n", &[small_x, ln1p(small_x)])?;
    
    // Sigmoid function for machine learning
    facts sigmoid_input = 2.0;
    printf("sigmoid({}) = {}\n", &[sigmoid_input, sigmoid(sigmoid_input)])?;
    
    println("")?;
}

fr fr Demonstrate mathematical constants
slay demonstrate_constants() {
    println("=== Mathematical Constants ===")?;
    
    printf("π (PI) = {}\n", &[PI])?;
    printf("τ (TAU) = 2π = {}\n", &[TAU])?;
    printf("e (E) = {}\n", &[E])?;
    printf("φ (PHI) = {}\n", &[PHI])?;
    printf("γ (EULER_GAMMA) = {}\n", &[EULER_GAMMA])?;
    
    // Derived constants
    printf("π/2 = {}\n", &[FRAC_PI_2])?;
    printf("√2 = {}\n", &[SQRT_2])?;
    printf("√π = {}\n", &[SQRT_PI])?;
    printf("ln(2) = {}\n", &[LN_2])?;
    printf("ln(10) = {}\n", &[LN_10])?;
    
    // Conversion factors
    printf("degrees to radians factor = {}\n", &[DEG_TO_RAD])?;
    printf("radians to degrees factor = {}\n", &[RAD_TO_DEG])?;
    
    // Floating point limits
    printf("machine epsilon = {}\n", &[EPSILON])?;
    printf("infinity = {}\n", &[INFINITY])?;
    
    println("")?;
}

fr fr Demonstrate random number generation
slay demonstrate_random() {
    println("=== Random Number Generation ===")?;
    
    // Set seed for reproducible results
    set_seed(12345);
    
    // Basic random generation
    printf("random() = {} (0.0 to 1.0)\n", &[random()])?;
    printf("random_range(10.0, 20.0) = {}\n", &[random_range(10.0, 20.0)])?;
    printf("random_int(1, 100) = {}\n", &[random_int(1, 100)])?;
    printf("random_bool() = {}\n", &[random_bool()])?;
    
    // Statistical distributions
    printf("random_normal(0.0, 1.0) = {} (standard normal)\n", &[random_normal(0.0, 1.0)])?;
    printf("random_exponential(1.0) = {} (exponential)\n", &[random_exponential(1.0)])?;
    printf("random_uniform(5.0, 15.0) = {} (uniform)\n", &[random_uniform(5.0, 15.0)])?;
    
    // Random strings and bytes
    printf("random_string(8) = '{}'\n", &[random_string(8)])?;
    printf("random_hex(16) = '{}'\n", &[random_hex(16)])?;
    printf("random_alphanumeric(10) = '{}'\n", &[random_alphanumeric(10)])?;
    
    // Random choice from collections
    facts options = ["apple", "banana", "cherry", "date"];
    facts chosen = choice(&options);
    printf("choice from fruits = '{}'\n", &[chosen])?;
    
    println("")?;
}

fr fr Demonstrate statistical functions
slay demonstrate_statistics() {
    println("=== Statistical Functions ===")?;
    
    facts data = [2.5, 4.1, 3.7, 5.2, 3.9, 4.8, 3.3, 4.6, 3.8, 4.2];
    
    // Descriptive statistics
    printf("Data: {:?}\n", &[data])?;
    printf("mean = {}\n", &[mean(&data)])?;
    printf("median = {}\n", &[median(&data)])?;
    printf("variance = {}\n", &[variance(&data)])?;
    printf("standard deviation = {}\n", &[standard_deviation(&data)])?;
    
    // Range and spread
    facts (min_val, max_val) = min_max(&data);
    printf("min = {}, max = {}\n", &[min_val, max_val])?;
    printf("range = {}\n", &[range(&data)])?;
    printf("iqr = {}\n", &[iqr(&data)])?;
    
    // Advanced statistics
    printf("skewness = {}\n", &[skewness(&data)])?;
    printf("kurtosis = {}\n", &[kurtosis(&data)])?;
    printf("coefficient of variation = {}\n", &[coefficient_of_variation(&data)])?;
    
    // Quantiles
    printf("25th percentile = {}\n", &[quantile(&data, 0.25)])?;
    printf("75th percentile = {}\n", &[quantile(&data, 0.75)])?;
    
    // Generate another dataset for correlation
    facts data2 = [5.1, 7.8, 6.9, 9.2, 7.1, 8.4, 6.7, 8.9, 7.2, 8.1];
    printf("correlation with second dataset = {}\n", &[correlation(&data, &data2)])?;
    printf("covariance with second dataset = {}\n", &[covariance(&data, &data2)])?;
    
    println("")?;
}

fr fr Demonstrate special mathematical functions
slay demonstrate_special_functions() {
    println("=== Special Mathematical Functions ===")?;
    
    // Factorial and gamma functions
    lowkey (sus i = 1; i <= 10; i++) {
        printf("{}! = {}\n", &[i, factorial(i)])?;
    }
    
    printf("Γ(5.5) = {}\n", &[gamma(5.5)])?;
    printf("Γ(4) = {} (should equal 3! = 6)\n", &[gamma(4.0)])?;
    
    // Beta function
    printf("B(2, 3) = {}\n", &[beta(2.0, 3.0)])?;
    
    // Binomial coefficients
    printf("C(10, 3) = {} (10 choose 3)\n", &[binomial(10, 3)])?;
    printf("C(20, 5) = {} (20 choose 5)\n", &[binomial(20, 5)])?;
    
    // Famous sequences
    println("Fibonacci sequence (first 15 terms):")?;
    lowkey (sus i = 1; i <= 15; i++) {
        printf("F({}) = {}, ", &[i, fibonacci(i)])?;
    }
    println("")?;
    
    println("Lucas sequence (first 10 terms):")?;
    lowkey (sus i = 1; i <= 10; i++) {
        printf("L({}) = {}, ", &[i, lucas_number(i)])?;
    }
    println("")?;
    
    // Error functions
    facts x = 1.0;
    printf("erf({}) = {}\n", &[x, erf(x)])?;
    printf("erfc({}) = {}\n", &[x, erfc(x)])?;
    printf("erf({}) + erfc({}) = {} (should be 1.0)\n", &[x, x, erf(x) + erfc(x)])?;
    
    // Bessel functions
    printf("J₀(1.0) = {}\n", &[bessel_j0(1.0)])?;
    printf("J₁(1.0) = {}\n", &[bessel_j1(1.0)])?;
    
    println("")?;
}

fr fr Demonstrate advanced mathematical utilities
slay demonstrate_utilities() {
    println("=== Advanced Mathematical Utilities ===")?;
    
    // Number theory
    facts n1 = 48;
    facts n2 = 18;
    facts (gcd_val, x, y) = extended_gcd(n1, n2);
    printf("Extended GCD({}, {}) = {}, coefficients: {}, {}\n", &[n1, n2, gcd_val, x, y])?;
    printf("Verification: {} * {} + {} * {} = {}\n", &[x, n1, y, n2, x * n1 + y * n2])?;
    
    // Prime testing
    lowkey (sus i = 2; i <= 30; i++) {
        lowkey is_prime(i) {
            printf("{} is prime, ", &[i])?;
        }
    }
    println("")?;
    
    // Prime factorization
    facts number = 60;
    facts factors = prime_factorization(number);
    printf("Prime factorization of {}: {:?}\n", &[number, factors])?;
    
    // Combinatorics
    printf("P(10, 4) = {} (permutations)\n", &[permutations(10, 4)])?;
    printf("C(10, 4) = {} (combinations)\n", &[combinations(10, 4)])?;
    
    // Modular arithmetic
    facts base = 3;
    facts exp = 10;
    facts modulus = 7;
    printf("{}^{} mod {} = {}\n", &[base, exp, modulus, mod_pow(base, exp, modulus)])?;
    
    // Perfect numbers
    printf("Perfect numbers up to 1000: ");
    lowkey (sus i = 1; i <= 1000; i++) {
        lowkey is_perfect_number(i) {
            printf("{} ", &[i])?;
        }
    }
    println("")?;
    
    // Digital root
    facts large_num = 12345;
    printf("Digital root of {} = {}\n", &[large_num, digital_root(large_num)])?;
    
    println("")?;
}

fr fr Demonstrate numerical methods
slay demonstrate_numerical_methods() {
    println("=== Numerical Methods ===")?;
    
    // Numerical integration using Simpson's rule
    // Integrate x² from 0 to 2 (analytical result: 8/3 ≈ 2.667)
    facts integral_result = simpson_integration(|x| x * x, 0.0, 2.0, 1000);
    printf("∫₀² x² dx = {} (analytical: {})\n", &[integral_result, 8.0/3.0])?;
    
    // Numerical derivative
    // Derivative of x³ at x=2 (analytical result: 12)
    facts derivative_result = numerical_derivative(|x| x * x * x, 2.0, 1e-6);
    printf("d/dx(x³) at x=2 = {} (analytical: 12)\n", &[derivative_result])?;
    
    // Newton-Raphson root finding
    // Find root of x² - 4 = 0 (roots: ±2)
    facts polynomial = |x: f64| x * x - 4.0;
    facts derivative = |x: f64| 2.0 * x;
    facts root = newton_raphson(polynomial, derivative, 1.5, 1e-10, 100);
    printf("Root of x² - 4 = 0: x = {} (should be 2.0)\n", &[root])?;
    
    // Bisection method for same polynomial
    facts root_bisection = bisection_method(polynomial, 0.0, 3.0, 1e-10, 100);
    printf("Root using bisection method: x = {}\n", &[root_bisection])?;
    
    println("")?;
}

fr fr Demonstrate real-world mathematical applications
slay demonstrate_applications() {
    println("=== Real-World Mathematical Applications ===")?;
    
    // 1. Compound Interest Calculation
    facts principal = 1000.0; // $1000
    facts rate = 0.05; // 5% annual interest
    facts time = 10.0; // 10 years
    facts compound_amount = principal * exp(rate * time);
    printf("Compound interest: ${} at {}% for {} years = ${}\n", 
           &[principal, rate * 100.0, time, compound_amount])?;
    
    // 2. Signal Processing - Fourier analysis
    println("Signal analysis (sine wave with noise):")?;
    facts sample_rate = 100.0;
    facts frequency = 5.0;
    facts noise_level = 0.1;
    
    set_seed(42);
    lowkey (sus i = 0; i < 20; i++) {
        facts t = i as f64 / sample_rate;
        facts signal = sin(2.0 * PI * frequency * t);
        facts noise = random_normal(0.0, noise_level);
        facts noisy_signal = signal + noise;
        printf("t={:.2}s: signal={:.3}, noise={:.3}, total={:.3}\n", 
               &[t, signal, noise, noisy_signal])?;
    }
    
    // 3. Statistical Quality Control
    println("Quality control analysis:")?;
    facts measurements = [10.2, 9.8, 10.1, 9.9, 10.3, 9.7, 10.0, 10.2, 9.9, 10.1];
    facts target = 10.0;
    facts tolerance = 0.5;
    
    facts sample_mean = mean(&measurements);
    facts sample_std = standard_deviation(&measurements);
    printf("Target: {}, Tolerance: ±{}\n", &[target, tolerance])?;
    printf("Sample mean: {}, Sample std dev: {}\n", &[sample_mean, sample_std])?;
    
    // Process capability
    facts cp = tolerance / (3.0 * sample_std);
    printf("Process capability (Cp): {} (>1.33 is good)\n", &[cp])?;
    
    // 4. Population Growth Model
    println("Population growth model:")?;
    facts initial_population = 1000.0;
    facts growth_rate = 0.03; // 3% per year
    printf("Year 0: {} people\n", &[initial_population])?;
    
    lowkey (sus year = 1; year <= 10; year++) {
        facts population = initial_population * exp(growth_rate * year as f64);
        printf("Year {}: {:.0} people\n", &[year, population])?;
    }
    
    // 5. Cryptographic Application - Large prime generation
    println("Large prime search (cryptographic applications):")?;
    facts start = 1000;
    facts primes_found = 0;
    lowkey (sus candidate = start; primes_found < 5; candidate++) {
        lowkey is_prime(candidate) {
            printf("Prime {}: {}\n", &[primes_found + 1, candidate])?;
            primes_found = primes_found + 1;
        }
    }
    
    println("")?;
}

fr fr Demonstrate cross-module integration
slay demonstrate_integration() {
    println("=== Cross-Module Integration Example ===")?;
    println("Scientific Data Analysis Pipeline")?;
    
    // 1. Generate experimental data
    set_seed(2023);
    facts num_samples = 50;
    facts experimental_data = [];
    
    println("Generating experimental data...")?;
    lowkey (sus i = 0; i < num_samples; i++) {
        facts x = i as f64 * 2.0 * PI / num_samples as f64;
        facts theoretical = sin(2.0 * x) + 0.5 * cos(4.0 * x);
        facts noise = random_normal(0.0, 0.1);
        facts measurement = theoretical + noise;
        experimental_data.push(measurement);
    }
    
    // 2. Statistical analysis
    facts data_mean = mean(&experimental_data);
    facts data_std = standard_deviation(&experimental_data);
    facts data_range = range(&experimental_data);
    
    printf("Statistical summary:\n")?;
    printf("  Mean: {:.4}\n", &[data_mean])?;
    printf("  Std Dev: {:.4}\n", &[data_std])?;
    printf("  Range: {:.4}\n", &[data_range])?;
    
    // 3. Data transformation
    facts log_transformed = experimental_data.iter()
        .map(|&x| ln(abs(x) + 1.0))
        .collect();
    
    facts sqrt_transformed = experimental_data.iter()
        .map(|&x| sqrt(abs(x)))
        .collect();
    
    printf("After log transformation - Mean: {:.4}, Std: {:.4}\n", 
           &[mean(&log_transformed), standard_deviation(&log_transformed)])?;
    printf("After sqrt transformation - Mean: {:.4}, Std: {:.4}\n", 
           &[mean(&sqrt_transformed), standard_deviation(&sqrt_transformed)])?;
    
    // 4. Numerical analysis
    println("Finding extrema using numerical methods...")?;
    facts test_function = |x: f64| sin(2.0 * x) + 0.5 * cos(4.0 * x);
    facts test_derivative = |x: f64| 2.0 * cos(2.0 * x) - 2.0 * sin(4.0 * x);
    
    // Find critical points (where derivative = 0)
    facts critical_point = newton_raphson(test_derivative, 
                                         |x: f64| -4.0 * sin(2.0 * x) - 8.0 * cos(4.0 * x),
                                         0.5, 1e-8, 100);
    facts function_value = test_function(critical_point);
    printf("Critical point at x = {:.6}, f(x) = {:.6}\n", &[critical_point, function_value])?;
    
    // 5. Special function analysis
    facts gamma_analysis = gamma(data_std + 1.0);
    facts factorial_approx = factorial_stirling(round(abs(data_mean)) as usize);
    
    printf("Gamma function analysis: Γ({:.4}) = {:.4}\n", &[data_std + 1.0, gamma_analysis])?;
    printf("Stirling approximation: {}! ≈ {:.2e}\n", &[round(abs(data_mean)), factorial_approx])?;
    
    // 6. Random sampling validation
    println("Monte Carlo validation:")?;
    facts monte_carlo_samples = 10000;
    facts monte_carlo_sum = 0.0;
    
    lowkey (sus i = 0; i < monte_carlo_samples; i++) {
        facts random_x = random_range(0.0, 2.0 * PI);
        facts random_y = test_function(random_x);
        monte_carlo_sum = monte_carlo_sum + random_y;
    }
    
    facts monte_carlo_average = monte_carlo_sum / monte_carlo_samples as f64;
    printf("Monte Carlo average over [0, 2π]: {:.6}\n", &[monte_carlo_average])?;
    
    // 7. Final integration using numerical methods
    facts numerical_integral = simpson_integration(test_function, 0.0, 2.0 * PI, 1000);
    printf("Numerical integration over [0, 2π]: {:.6}\n", &[numerical_integral])?;
    
    println("Integration analysis complete!")?;
    println("")?;
}

fr fr Main demonstration function
slay main() {
    println("CURSED Mathematical Library - Comprehensive Demonstration")?;
    println("========================================================")?;
    println("")?;
    
    demonstrate_basic_math()?;
    demonstrate_trigonometry()?;
    demonstrate_logarithmic_exponential()?;
    demonstrate_constants()?;
    demonstrate_random()?;
    demonstrate_statistics()?;
    demonstrate_special_functions()?;
    demonstrate_utilities()?;
    demonstrate_numerical_methods()?;
    demonstrate_applications()?;
    demonstrate_integration()?;
    
    println("========================================================")?;
    println("Mathematical library demonstration complete!")?;
    println("All modules working together seamlessly.")?;
    
    no_cap 0;
}
