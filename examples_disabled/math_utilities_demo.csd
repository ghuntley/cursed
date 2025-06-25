// Mathematical Utilities Demo for CURSED Programming Language
// Demonstrates comprehensive mathematical functions including number theory,
// combinatorics, special functions, numerical methods, sequences, and modular arithmetic

import "stdlib::math";
import "stdlib::io";

fun demonstrate_number_theory() {
    println("=== Number Theory Functions ===");
    
    // Prime checking
    let num = 97;
    printf("Is {} prime? {}\n", &[num, is_prime(num)]);
    
    // Prime factorization
    let n = 60;
    match prime_factorization(n) {
        Ok(factors) => {
            printf("Prime factorization of {}: ", &[n]);
            for (prime, power) in factors {
                printf("{}^{} ", &[prime, power]);
            }
            println("");
        }
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Greatest Common Divisor using Extended Euclidean Algorithm
    let a = 48, b = 18;
    let (gcd, x, y) = extended_gcd(a, b);
    printf("Extended GCD of {} and {}: gcd={}, {}*{} + {}*{} = {}\n", 
           &[a, b, gcd, a, x, b, y, gcd]);
    
    // Euler's totient function
    let n = 12;
    match euler_totient(n) {
        Ok(phi) => printf("φ({}) = {} (count of numbers ≤ {} coprime to {})\n", &[n, phi, n, n]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Generate primes using Sieve of Eratosthenes
    let primes = sieve_of_eratosthenes(50);
    print("Primes up to 50: ");
    for prime in primes {
        printf("{} ", &[prime]);
    }
    println("");
    println("");
}

fun demonstrate_combinatorics() {
    println("=== Combinatorics Functions ===");
    
    // Factorial and variations
    let n = 10;
    match factorial(n) {
        Ok(fact) => printf("{}! = {}\n", &[n, fact]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    match double_factorial(n) {
        Ok(dfact) => printf("{}!! = {}\n", &[n, dfact]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Stirling's approximation for large factorials
    let large_n = 50;
    match factorial_stirling(large_n) {
        Ok(stirling) => printf("{}! ≈ {:.3e} (Stirling's approximation)\n", &[large_n, stirling]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Permutations and combinations
    let n = 10, r = 3;
    match permutations(n, r) {
        Ok(perm) => printf("P({},{}) = {} (permutations)\n", &[n, r, perm]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    match combinations(n, r) {
        Ok(comb) => printf("C({},{}) = {} (combinations)\n", &[n, r, comb]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Catalan numbers
    let cat_n = 5;
    match catalan_number(cat_n) {
        Ok(catalan) => printf("Catalan number C_{} = {}\n", &[cat_n, catalan]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Multicombinations (combinations with repetition)
    match multicombinations(4, 3) {
        Ok(multi) => printf("Multicombinations(4,3) = {}\n", &[multi]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    println("");
}

fun demonstrate_special_functions() {
    println("=== Special Mathematical Functions ===");
    
    // Gamma function
    let x = 3.5;
    match gamma_function(x) {
        Ok(gamma) => printf("Γ({}) = {:.6f}\n", &[x, gamma]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Beta function
    let x = 2.5, y = 3.0;
    match beta_function(x, y) {
        Ok(beta) => printf("B({},{}) = {:.6f}\n", &[x, y, beta]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Error function
    let values = [0.0, 0.5, 1.0, 1.5, 2.0];
    println("Error function values:");
    for val in values {
        match error_function(val) {
            Ok(erf) => printf("erf({}) = {:.6f}\n", &[val, erf]),
            Err(e) => eprintln(&format!("Error: {}", e));
        }
    }
    
    // Complementary error function
    let x = 1.0;
    match complementary_error_function(x) {
        Ok(erfc) => printf("erfc({}) = {:.6f}\n", &[x, erfc]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    println("");
}

fun demonstrate_numerical_methods() {
    println("=== Numerical Methods ===");
    
    // Numerical integration using Simpson's rule
    // Integrate x^2 from 0 to 2 (analytical result: 8/3 ≈ 2.667)
    match simpson_integration(|x| x * x, 0.0, 2.0, 100) {
        Ok(integral) => printf("∫₀² x² dx ≈ {:.6f} (analytical: {:.6f})\n", &[integral, 8.0/3.0]),
        Err(e) => eprintln(&format!("Integration error: {}", e));
    }
    
    // Numerical differentiation
    // Derivative of x^3 at x=2 (analytical result: 3*2^2 = 12)
    match numerical_derivative(|x| x * x * x, 2.0, 1e-6) {
        Ok(derivative) => printf("d/dx(x³) at x=2 ≈ {:.6f} (analytical: 12.0)\n", &[derivative]),
        Err(e) => eprintln(&format!("Differentiation error: {}", e));
    }
    
    // Root finding using Newton-Raphson method
    // Find root of x^2 - 2 = 0 (should be √2 ≈ 1.414)
    let f = |x| x * x - 2.0;
    let df = |x| 2.0 * x;
    match newton_raphson(f, df, 1.0, 1e-10, 100) {
        Ok(root) => printf("Root of x² - 2 = 0: x ≈ {:.10f} (√2 ≈ {:.10f})\n", &[root, 2.0.sqrt()]),
        Err(e) => eprintln(&format!("Newton-Raphson error: {}", e));
    }
    
    // Root finding using bisection method
    // Find root of cos(x) = 0 between 0 and 2 (should be π/2 ≈ 1.571)
    match bisection_method(f64::cos, 0.0, 2.0, 1e-10, 100) {
        Ok(root) => {
            let pi_half = std::f64::consts::PI / 2.0;
            printf("Root of cos(x) = 0: x ≈ {:.10f} (π/2 ≈ {:.10f})\n", &[root, pi_half]);
        }
        Err(e) => eprintln(&format!("Bisection error: {}", e));
    }
    println("");
}

fun demonstrate_sequences() {
    println("=== Sequence and Series Functions ===");
    
    // Fibonacci sequence
    print("Fibonacci sequence (first 15 terms): ");
    for i in 0..15 {
        match fibonacci(i) {
            Ok(fib) => printf("{} ", &[fib]),
            Err(e) => eprintln(&format!("Error: {}", e));
        }
    }
    println("");
    
    // Lucas numbers
    print("Lucas sequence (first 10 terms): ");
    for i in 0..10 {
        match lucas_number(i) {
            Ok(lucas) => printf("{} ", &[lucas]),
            Err(e) => eprintln(&format!("Error: {}", e));
        }
    }
    println("");
    
    // Tribonacci sequence
    print("Tribonacci sequence (first 12 terms): ");
    for i in 0..12 {
        match tribonacci(i) {
            Ok(trib) => printf("{} ", &[trib]),
            Err(e) => eprintln(&format!("Error: {}", e));
        }
    }
    println("");
    
    // Harmonic numbers
    println("Harmonic numbers:");
    for n in 1..=10 {
        match harmonic_number(n) {
            Ok(h_n) => printf("H_{} = {:.6f}\n", &[n, h_n]),
            Err(e) => eprintln(&format!("Error: {}", e));
        }
    }
    
    // Factorial sequence sum
    let n = 6;
    match factorial_sequence_sum(n) {
        Ok(sum) => printf("Sum of factorials 0! + 1! + ... + {}! = {}\n", &[n, sum]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    println("");
}

fun demonstrate_modular_arithmetic() {
    println("=== Modular Arithmetic and Base Conversions ===");
    
    // Modular exponentiation
    let base = 3, exp = 4, modulus = 7;
    match mod_pow(base, exp, modulus) {
        Ok(result) => printf("{}^{} mod {} = {}\n", &[base, exp, modulus, result]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Modular multiplicative inverse
    let a = 3, m = 7;
    match mod_inverse(a, m) {
        Ok(inverse) => printf("Multiplicative inverse of {} mod {} = {}\n", &[a, m, inverse]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Base conversions
    let number = "255";
    match convert_base(number, 10, 16) {
        Ok(hex) => printf("Decimal {} in hexadecimal: {}\n", &[number, hex]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    match convert_base("FF", 16, 2) {
        Ok(binary) => printf("Hexadecimal FF in binary: {}\n", &[binary]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // GCD and LCM of multiple numbers
    let numbers = [48, 18, 24];
    match gcd_multiple(&numbers) {
        Ok(gcd) => printf("GCD of {:?} = {}\n", &[numbers, gcd]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    match lcm_multiple(&numbers) {
        Ok(lcm) => printf("LCM of {:?} = {}\n", &[numbers, lcm]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    println("");
}

fun demonstrate_advanced_utilities() {
    println("=== Advanced Mathematical Utilities ===");
    
    // Memoized Fibonacci for performance
    let mut fib_memo = FibonacciMemo::new();
    let large_fib_n = 30;
    match fib_memo.fibonacci(large_fib_n) {
        Ok(fib) => printf("Fibonacci({}) = {} (using memoization)\n", &[large_fib_n, fib]),
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Perfect number checking
    let perfect_candidates = [6, 28, 12, 100];
    println("Perfect number checking:");
    for num in perfect_candidates {
        match is_perfect_number(num) {
            Ok(is_perfect) => printf("{} is {}perfect\n", &[num, if is_perfect { "" } else { "not " }]),
            Err(e) => eprintln(&format!("Error: {}", e));
        }
    }
    
    // Digital root calculation
    let numbers = [123, 456, 999, 38];
    println("Digital roots:");
    for num in numbers {
        match digital_root(num) {
            Ok(root) => printf("Digital root of {} = {}\n", &[num, root]),
            Err(e) => eprintln(&format!("Error: {}", e));
        }
    }
    println("");
}

fun demonstrate_practical_applications() {
    println("=== Practical Applications ===");
    
    // Cryptography: RSA key generation concepts
    println("RSA Key Generation Concepts:");
    let p = 61, q = 53;  // Small primes for demonstration
    let n = p * q;
    match euler_totient(n) {
        Ok(phi_n) => {
            printf("p = {}, q = {}, n = p*q = {}\n", &[p, q, n]);
            printf("φ(n) = φ({}) = {}\n", &[n, phi_n]);
            
            // Find e coprime to φ(n)
            let e = 17;  // Common choice
            let (gcd, _, _) = extended_gcd(e, phi_n);
            if gcd == 1 {
                match mod_inverse(e, phi_n) {
                    Ok(d) => printf("Public key: (e={}, n={}), Private key: (d={}, n={})\n", &[e, n, d, n]),
                    Err(e) => eprintln(&format!("Error finding private key: {}", e));
                }
            } else {
                printf("e={} is not coprime to φ(n)={}\n", &[e, phi_n]);
            }
        }
        Err(e) => eprintln(&format!("Error: {}", e));
    }
    
    // Probability and Statistics: Pascal's Triangle
    println("\nPascal's Triangle (first 8 rows):");
    for n in 0..8 {
        for k in 0..=n {
            match combinations(n, k) {
                Ok(coeff) => printf("{:4}", &[coeff]),
                Err(_) => printf("   ?");
            }
        }
        println("");
    }
    
    // Physics: Harmonic oscillator energy levels
    println("\nQuantum Harmonic Oscillator Energy Levels:");
    for n in 0..6 {
        // E_n = ℏω(n + 1/2), using ℏω = 1 for simplicity
        let energy = n as f64 + 0.5;
        printf("n = {}: E_{} = {:.1f}ℏω\n", &[n, n, energy]);
    }
    
    // Engineering: Signal processing with numerical integration
    println("\nSignal Analysis: RMS value of sin(x) over one period:");
    // RMS = √(1/T ∫₀ᵀ sin²(x) dx) for sin(x) over [0, 2π]
    let period = 2.0 * std::f64::consts::PI;
    match simpson_integration(|x| (x.sin()).powi(2), 0.0, period, 1000) {
        Ok(integral) => {
            let rms = (integral / period).sqrt();
            printf("RMS value ≈ {:.6f} (theoretical: {:.6f})\n", &[rms, 1.0/2.0_f64.sqrt()]);
        }
        Err(e) => eprintln(&format!("Integration error: {}", e));
    }
    println("");
}

fun main() -> Result<(), String> {
    println("🧮 CURSED Mathematical Utilities Comprehensive Demo");
    println("==================================================");
    println("");
    
    demonstrate_number_theory();
    demonstrate_combinatorics();
    demonstrate_special_functions();
    demonstrate_numerical_methods();
    demonstrate_sequences();
    demonstrate_modular_arithmetic();
    demonstrate_advanced_utilities();
    demonstrate_practical_applications();
    
    println("✅ Mathematical utilities demonstration completed!");
    println("This showcases the comprehensive mathematical capabilities");
    println("available in the CURSED standard library for scientific,");
    println("engineering, and mathematical computing applications.");
    
    Ok(())
}
