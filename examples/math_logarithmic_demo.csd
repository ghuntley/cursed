fr fr Demo program showcasing comprehensive logarithmic and exponential functions

yeet "stdlib::math"
yeet "stdlib::io"

slay main() -> nil {
    println("🧮 CURSED Logarithmic and Exponential Functions Demo")?;
    println("===================================================\n")?;
    
    // Basic logarithmic functions
    println("📊 Basic Logarithmic Functions:")?;
    demo_basic_logarithms()?;
    println("")?;
    
    // Enhanced exponential functions
    println("⚡ Enhanced Exponential Functions:")?;
    demo_exponential_functions()?;
    println("")?;
    
    // Power functions
    println("💪 Power Functions:")?;
    demo_power_functions()?;
    println("")?;
    
    // Logarithmic utilities
    println("🔧 Logarithmic Utilities and Transformations:")?;
    demo_logarithmic_utilities()?;
    println("")?;
    
    // Domain validation
    println("✅ Domain Validation and Safety:")?;
    demo_domain_validation()?;
    println("")?;
    
    // Mathematical applications
    println("🎯 Mathematical Applications:")?;
    demo_mathematical_applications()?;
    
    println("\n🎉 Demo completed successfully!")?;
}

slay demo_basic_logarithms() -> Result<nil, CursedError> {
    // Natural logarithm
    facts ln_e = ln(E)?; // ln(e) = 1
    printf("ln(e) = {:.6}\n", &[ln_e])?;
    
    facts ln_10 = ln(10.0)?;
    printf("ln(10) = {:.6}\n", &[ln_10])?;
    
    // Common logarithm (base 10)
    facts log10_100 = log10(100.0)?; // log10(100) = 2
    printf("log10(100) = {:.6}\n", &[log10_100])?;
    
    facts log10_1000 = log10(1000.0)?; // log10(1000) = 3
    printf("log10(1000) = {:.6}\n", &[log10_1000])?;
    
    // Binary logarithm (base 2)
    facts log2_16 = log2(16.0)?; // log2(16) = 4
    printf("log2(16) = {:.6}\n", &[log2_16])?;
    
    facts log2_1024 = log2(1024.0)?; // log2(1024) = 10
    printf("log2(1024) = {:.6}\n", &[log2_1024])?;
    
    // Arbitrary base logarithm
    facts log_base_3_27 = log(27.0, 3.0)?; // log_3(27) = 3
    printf("log_3(27) = {:.6}\n", &[log_base_3_27])?;
    
    facts log_base_5_125 = log(125.0, 5.0)?; // log_5(125) = 3
    printf("log_5(125) = {:.6}\n", &[log_base_5_125])?;
    
    Ok(nil)
}

slay demo_exponential_functions() -> Result<nil, CursedError> {
    // Natural exponential
    facts exp_1 = exp(1.0)?; // e^1 = e
    printf("e^1 = {:.6}\n", &[exp_1])?;
    
    facts exp_2 = exp(2.0)?; // e^2
    printf("e^2 = {:.6}\n", &[exp_2])?;
    
    // Base 2 exponential
    facts exp2_3 = exp2(3.0)?; // 2^3 = 8
    printf("2^3 = {:.6}\n", &[exp2_3])?;
    
    facts exp2_10 = exp2(10.0)?; // 2^10 = 1024
    printf("2^10 = {:.6}\n", &[exp2_10])?;
    
    // Base 10 exponential
    facts exp10_3 = exp10(3.0)?; // 10^3 = 1000
    printf("10^3 = {:.6}\n", &[exp10_3])?;
    
    // Enhanced exponential functions
    facts exp2m1_small = exp2m1(0.1)?; // 2^0.1 - 1 (accurate for small values)
    printf("2^0.1 - 1 = {:.6}\n", &[exp2m1_small])?;
    
    facts exp10m1_small = exp10m1(0.01)?; // 10^0.01 - 1
    printf("10^0.01 - 1 = {:.6}\n", &[exp10m1_small])?;
    
    // Arbitrary base exponential
    facts exp_base_3_4 = exp_base(3.0, 4.0)?; // 3^4 = 81
    printf("3^4 = {:.6}\n", &[exp_base_3_4])?;
    
    facts exp_base_7_2 = exp_base(7.0, 2.0)?; // 7^2 = 49
    printf("7^2 = {:.6}\n", &[exp_base_7_2])?;
    
    Ok(nil)
}

slay demo_power_functions() -> Result<nil, CursedError> {
    // Basic power function
    facts pow_2_8 = pow(2.0, 8.0)?; // 2^8 = 256
    printf("pow(2, 8) = {:.6}\n", &[pow_2_8])?;
    
    // Integer power (optimized)
    facts powi_3_5 = powi(3.0, 5)?; // 3^5 = 243
    printf("powi(3, 5) = {:.6}\n", &[powi_3_5])?;
    
    // Root functions
    facts sqrt_49 = sqrt(49.0)?; // sqrt(49) = 7
    printf("sqrt(49) = {:.6}\n", &[sqrt_49])?;
    
    facts cbrt_64 = cbrt(64.0)?; // cbrt(64) = 4
    printf("cbrt(64) = {:.6}\n", &[cbrt_64])?;
    
    facts nth_root_32_5 = nth_root(32.0, 5.0)?; // 5th root of 32 = 2
    printf("nth_root(32, 5) = {:.6}\n", &[nth_root_32_5])?;
    
    // Hypotenuse (Pythagorean theorem)
    facts hypot_3_4 = hypot(3.0, 4.0)?; // sqrt(3^2 + 4^2) = 5
    printf("hypot(3, 4) = {:.6}\n", &[hypot_3_4])?;
    
    facts hypot3_2_3_6 = hypot3(2.0, 3.0, 6.0)?; // sqrt(2^2 + 3^2 + 6^2) = 7
    printf("hypot3(2, 3, 6) = {:.6}\n", &[hypot3_2_3_6])?;
    
    // Tetration (power tower)
    facts tetration_2_3 = tetration(2.0, 3)?; // 2^(2^2) = 2^4 = 16
    printf("tetration(2, 3) = {:.6}\n", &[tetration_2_3])?;
    
    Ok(nil)
}

slay demo_logarithmic_utilities() -> Result<nil, CursedError> {
    // Absolute value logarithms
    facts log2_abs_neg8 = log2_abs(-8.0)?; // log2(|-8|) = log2(8) = 3
    printf("log2_abs(-8) = {:.6}\n", &[log2_abs_neg8])?;
    
    facts ln_abs_neg_e = ln_abs(-E)?; // ln(|-e|) = ln(e) = 1
    printf("ln_abs(-e) = {:.6}\n", &[ln_abs_neg_e])?;
    
    // Logarithmic mean
    facts log_mean_2_8 = log_mean(2.0, 8.0)?; // Logarithmic mean of 2 and 8
    printf("log_mean(2, 8) = {:.6}\n", &[log_mean_2_8])?;
    
    // Sigmoid function (machine learning activation)
    facts sigmoid_0 = sigmoid(0.0)?; // sigmoid(0) = 0.5
    printf("sigmoid(0) = {:.6}\n", &[sigmoid_0])?;
    
    facts sigmoid_2 = sigmoid(2.0)?; // sigmoid(2) ≈ 0.881
    printf("sigmoid(2) = {:.6}\n", &[sigmoid_2])?;
    
    facts sigmoid_neg2 = sigmoid(-2.0)?; // sigmoid(-2) ≈ 0.119
    printf("sigmoid(-2) = {:.6}\n", &[sigmoid_neg2])?;
    
    // Logistic function (generalized sigmoid)
    facts logistic_result = logistic(1.0, 10.0, 2.0, 0.5)?; // L=10, k=2, x0=0.5
    printf("logistic(1, L=10, k=2, x0=0.5) = {:.6}\n", &[logistic_result])?;
    
    // Log-sum-exp for numerical stability
    facts values = [1.0, 2.0, 3.0, 4.0];
    facts lse = log_sum_exp(&values)?;
    printf("log_sum_exp([1,2,3,4]) = {:.6}\n", &[lse])?;
    
    // Softmax single value
    facts reference_vals = [1.0, 2.0, 3.0];
    facts softmax_result = softmax_single(2.5, &reference_vals)?;
    printf("softmax_single(2.5, [1,2,3]) = {:.6}\n", &[softmax_result])?;
    
    Ok(nil)
}

slay demo_domain_validation() -> Result<nil, CursedError> {
    // Safe functions that return None instead of errors
    facts safe_ln_positive = safe_ln(5.0); // Some(ln(5))
    facts safe_ln_negative = safe_ln(-5.0); // None
    
    match safe_ln_positive {
        Some(value) => printf("safe_ln(5.0) = Some({:.6})\n", &[value])?,
        None => println("safe_ln(5.0) = None")?,
    }
    
    match safe_ln_negative {
        Some(value) => printf("safe_ln(-5.0) = Some({:.6})\n", &[value])?,
        None => println("safe_ln(-5.0) = None")?,
    }
    
    facts safe_exp_normal = safe_exp(2.0); // Some(exp(2))
    facts safe_exp_overflow = safe_exp(1000.0); // None (overflow)
    
    match safe_exp_normal {
        Some(value) => printf("safe_exp(2.0) = Some({:.6})\n", &[value])?,
        None => println("safe_exp(2.0) = None")?,
    }
    
    match safe_exp_overflow {
        Some(value) => printf("safe_exp(1000.0) = Some({:.6})\n", &[value])?,
        None => println("safe_exp(1000.0) = None (overflow)")?,
    }
    
    // Domain validation functions
    facts is_valid_log_5 = is_valid_log_input(5.0); // based
    facts is_valid_log_neg5 = is_valid_log_input(-5.0); // cap
    
    printf("is_valid_log_input(5.0) = {}\n", &[is_valid_log_5])?;
    printf("is_valid_log_input(-5.0) = {}\n", &[is_valid_log_neg5])?;
    
    // Clamped functions
    facts clamped_ln_small = clamped_ln(0.1, 1.0)?; // Clamps 0.1 to 1.0, then ln(1.0) = 0
    printf("clamped_ln(0.1, min=1.0) = {:.6}\n", &[clamped_ln_small])?;
    
    facts clamped_exp_large = clamped_exp(10.0, 1000.0)?; // exp(10) but clamped to max 1000
    printf("clamped_exp(10.0, max=1000.0) = {:.6}\n", &[clamped_exp_large])?;
    
    Ok(nil)
}

slay demo_mathematical_applications() -> Result<nil, CursedError> {
    println("📐 Practical Mathematical Applications:")?;
    
    // Compound interest calculation: A = P * e^(rt)
    facts principal = 1000.0;
    facts rate = 0.05; // 5% annual rate
    facts time = 10.0; // 10 years
    facts compound_amount = principal * exp(rate * time)?;
    printf("Compound Interest: ${:.2} after {} years at {:.1}%\n", 
           &[compound_amount, time, rate * 100.0])?;
    
    // Decibel calculation: dB = 20 * log10(ratio)
    facts voltage_ratio = 100.0;
    facts decibels = 20.0 * log10(voltage_ratio)?;
    printf("Voltage gain: {}x = {:.1} dB\n", &[voltage_ratio, decibels])?;
    
    // pH calculation: pH = -log10([H+])
    facts hydrogen_concentration = 1e-7; // pH 7 (neutral)
    facts ph_value = -log10(hydrogen_concentration)?;
    printf("pH of solution with [H+] = {:.0e}: {:.1}\n", 
           &[hydrogen_concentration, ph_value])?;
    
    // Half-life calculation: N(t) = N0 * exp(-λt)
    facts initial_amount = 100.0;
    facts decay_constant = 0.693 / 5730.0; // Carbon-14 half-life ≈ 5730 years
    facts time_elapsed = 11460.0; // 2 half-lives
    facts remaining_amount = initial_amount * exp(-decay_constant * time_elapsed)?;
    printf("Radioactive decay: {:.1}g after {} years = {:.1}g remaining\n",
           &[initial_amount, time_elapsed, remaining_amount])?;
    
    // Shannon entropy calculation: H = -Σ(p * log2(p))
    facts probabilities = [0.5, 0.25, 0.125, 0.125];
    sus total_entropy = 0.0;
    lowkey (sus i = 0; i < probabilities.len(); i++) {
        facts p = probabilities[i];
        bestie (p > 0.0) {
            total_entropy -= p * log2(p)?;
        }
    }
    printf("Shannon entropy: {:.3} bits\n", &[total_entropy])?;
    
    // Growth rate calculation: doubling time = ln(2) / r
    facts growth_rate = 0.07; // 7% growth rate
    facts doubling_time = ln(2.0)? / growth_rate;
    printf("Doubling time at {:.1}% growth: {:.1} time units\n",
           &[growth_rate * 100.0, doubling_time])?;
    
    // Signal-to-noise ratio in dB
    facts signal_power = 1000.0;
    facts noise_power = 10.0;
    facts snr_db = 10.0 * log10(signal_power / noise_power)?;
    printf("SNR: {:.1} dB (signal={:.0}, noise={:.0})\n",
           &[snr_db, signal_power, noise_power])?;
    
    Ok(nil)
}
