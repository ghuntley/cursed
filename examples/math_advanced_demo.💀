fr fr Advanced Mathematical Functions Demo for CURSED Programming Language
fr fr 
fr fr This program demonstrates the sophisticated mathematical capabilities
fr fr provided by the advanced math module, including numerical methods,
fr fr optimization algorithms, Fourier transforms, and mathematical modeling.

yeet "stdlib::math::advanced"
yeet "stdlib::io"

fr fr Demonstrates numerical differentiation
slay demonstrate_numerical_methods() {
    println("=== NUMERICAL METHODS DEMO ===");
    
    // Numerical derivative example
    println("1. Numerical Derivative:");
    
    // f(x) = x³ - 2x² + x + 1, f'(x) = 3x² - 4x + 1
    facts f = |x| x * x * x - 2.0 * x * x + x + 1.0;
    facts derivative_at_2 = numerical_derivative(f, 2.0, None)?;
    println(format!("   f'(2) ≈ {} (analytical: 5.0)", derivative_at_2));
    
    // Adaptive integration example
    println("2. Adaptive Simpson Integration:");
    
    // ∫sin(x)dx from 0 to π should equal 2
    facts sin_integral = adaptive_simpson_integration(|x| x.sin(), 0.0, PI, None, None)?;
    println(format!("   ∫sin(x)dx from 0 to π ≈ {} (analytical: 2.0)", sin_integral));
    
    // Multi-dimensional Newton-Raphson
    println("3. Newton-Raphson Root Finding:");
    println("   Solving system: x² + y² = 1, x - y = 0");
    
    facts functions = |vars: &[f64]| -> Vec<f64> {
        facts x = vars[0];
        facts y = vars[1];
        vec![x * x + y * y - 1.0, x - y]
    };
    
    facts jacobian = |vars: &[f64]| -> Vec<Vec<f64>> {
        facts x = vars[0];
        facts y = vars[1];
        vec![
            vec![2.0 * x, 2.0 * y],
            vec![1.0, -1.0],
        ]
    };
    
    facts initial_guess = vec![0.5, 0.5];
    facts solution = multidimensional_newton_raphson(functions, jacobian, &initial_guess, None, None)?;
    println(format!("   Solution: x = {:.6}, y = {:.6}", solution[0], solution[1]));
    println();
}

fr fr Demonstrates optimization algorithms
slay demonstrate_optimization() {
    println("=== OPTIMIZATION ALGORITHMS DEMO ===");
    
    // Golden section search
    println("1. Golden Section Search:");
    println("   Finding minimum of f(x) = (x - 3)² + 2");
    
    facts f_quad = |x: f64| (x - 3.0) * (x - 3.0) + 2.0;
    facts min_x = golden_section_search(f_quad, 0.0, 6.0, None)?;
    println(format!("   Minimum at x = {:.6} (analytical: 3.0)", min_x));
    
    // Gradient descent
    println("2. Gradient Descent:");
    println("   Minimizing f(x,y) = x² + 2y² - 4x + 8y + 21");
    
    facts f_multi = |vars: &[f64]| {
        facts x = vars[0];
        facts y = vars[1];
        x * x + 2.0 * y * y - 4.0 * x + 8.0 * y + 21.0
    };
    
    facts gradient = |vars: &[f64]| {
        facts x = vars[0];
        facts y = vars[1];
        vec![2.0 * x - 4.0, 4.0 * y + 8.0]
    };
    
    facts initial_point = vec![0.0, 0.0];
    facts minimum = gradient_descent(f_multi, gradient, &initial_point, None, None, None)?;
    println(format!("   Minimum at ({:.6}, {:.6}) (analytical: (2, -2))", minimum[0], minimum[1]));
    println();
}

fr fr Demonstrates Fourier transforms and signal processing
slay demonstrate_signal_processing() {
    println("=== SIGNAL PROCESSING DEMO ===");
    
    // FFT demonstration
    println("1. Fast Fourier Transform:");
    
    // Create a simple signal: DC + sine wave
    facts sample_rate = 8.0;
    facts frequency = 1.0;
    sus signal = vec![];
    lowkey (sus i = 0; i < 8; i++) {
        facts t = i as f64 / sample_rate;
        facts value = 1.0 + 0.5 * (2.0 * PI * frequency * t).sin();
        signal.push(value);
    }
    
    println("   Original signal (DC + 1Hz sine):");
    lowkey (sus i = 0; i < signal.len(); i++) {
        printf("   t={:.1}s: {:.3}", &[i as f64 / sample_rate, signal[i]]);
    }
    
    facts fft_result = fast_fourier_transform(&signal)?;
    println("   FFT magnitude spectrum:");
    lowkey (sus i = 0; i < fft_result.len(); i++) {
        facts magnitude = fft_result[i].magnitude();
        printf("   Bin {}: {:.3}", &[i, magnitude]);
    }
    
    // Convolution filtering
    println("2. Convolution Filter (Moving Average):");
    facts noisy_signal = vec![1.0, 3.0, 2.0, 5.0, 4.0, 2.0, 6.0, 3.0];
    facts filter_coeffs = vec![0.25, 0.5, 0.25]; // Simple smoothing filter
    
    facts filtered = convolution_filter(&noisy_signal, &filter_coeffs)?;
    println("   Original: [1, 3, 2, 5, 4, 2, 6, 3]");
    printf("   Filtered: ");
    lowkey (sus i = 0; i < filtered.len(); i++) {
        printf("{:.2} ", &[filtered[i]]);
    }
    println();
    println();
}

fr fr Demonstrates interpolation and approximation
slay demonstrate_interpolation() {
    println("=== INTERPOLATION & APPROXIMATION DEMO ===");
    
    // Cubic spline interpolation
    println("1. Cubic Spline Interpolation:");
    facts x_data = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    facts y_data = vec![0.0, 1.0, 4.0, 9.0, 16.0]; // y = x²
    
    println("   Data points (x²): (0,0), (1,1), (2,4), (3,9), (4,16)");
    
    facts interp_points = vec![0.5, 1.5, 2.5, 3.5];
    lowkey (sus point in interp_points) {
        facts interpolated = cubic_spline_interpolation(&x_data, &y_data, point)?;
        facts analytical = point * point;
        printf("   x={:.1}: interpolated={:.3}, analytical={:.3}", &[point, interpolated, analytical]);
    }
    
    // Chebyshev approximation
    println("2. Chebyshev Polynomial Approximation:");
    println("   Approximating f(x) = e^x on [-1, 1]");
    
    facts f_exp = |x: f64| x.exp();
    facts chebyshev_coeffs = chebyshev_approximation(f_exp, -1.0, 1.0, 5)?;
    
    printf("   Chebyshev coefficients: ");
    lowkey (sus coeff in chebyshev_coeffs) {
        printf("{:.4} ", &[coeff]);
    }
    println();
    println();
}

fr fr Demonstrates matrix operations
slay demonstrate_matrix_operations() {
    println("=== MATRIX OPERATIONS DEMO ===");
    
    // Linear system solving
    println("1. Linear System Solution:");
    println("   Solving: 2x + 3y = 8, x - y = -1");
    
    facts matrix = vec![
        vec![2.0, 3.0],
        vec![1.0, -1.0],
    ];
    facts vector = vec![8.0, -1.0];
    
    facts solution = solve_linear_system(&matrix, &vector)?;
    printf("   Solution: x = {:.3}, y = {:.3}", &[solution[0], solution[1]]);
    
    // Matrix determinant
    println("2. Matrix Determinant:");
    facts test_matrix = vec![
        vec![1.0, 2.0, 3.0],
        vec![0.0, 1.0, 4.0],
        vec![5.0, 6.0, 0.0],
    ];
    
    facts det = matrix_determinant(&test_matrix)?;
    println("   3×3 matrix determinant: {:.3}", det);
    println();
}

fr fr Demonstrates mathematical modeling
slay demonstrate_mathematical_modeling() {
    println("=== MATHEMATICAL MODELING DEMO ===");
    
    // Polynomial fitting
    println("1. Polynomial Curve Fitting:");
    println("   Fitting quadratic to noisy data");
    
    // Generate noisy quadratic data: y = 0.5x² + 2x + 1
    facts x_data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    facts y_data = vec![1.1, 3.4, 6.8, 11.2, 16.9, 24.1]; // With small noise
    
    facts poly_coeffs = polynomial_fit(&x_data, &y_data, 2)?;
    printf("   Fitted polynomial: {:.3} + {:.3}x + {:.3}x²", &[poly_coeffs[0], poly_coeffs[1], poly_coeffs[2]]);
    println("   Original polynomial: 1.0 + 2.0x + 0.5x²");
    
    // Evaluate fitted polynomial
    println("2. Polynomial Evaluation:");
    facts test_x = 2.5;
    facts fitted_value = evaluate_polynomial(&poly_coeffs, test_x)?;
    facts analytical_value = 1.0 + 2.0 * test_x + 0.5 * test_x * test_x;
    printf("   At x={:.1}: fitted={:.3}, analytical={:.3}", &[test_x, fitted_value, analytical_value]);
    println();
}

fr fr Demonstrates engineering applications
slay demonstrate_engineering_applications() {
    println("=== ENGINEERING APPLICATIONS DEMO ===");
    
    // Signal analysis
    println("1. Vibration Analysis (Simulated):");
    
    // Simulate a vibration signal with multiple frequencies
    facts time_points = 16;
    facts sampling_rate = 16.0;
    sus vibration_signal = vec![];
    
    lowkey (sus i = 0; i < time_points; i++) {
        facts t = i as f64 / sampling_rate;
        facts signal = 
            1.0 * (2.0 * PI * 2.0 * t).sin() +  // 2 Hz component
            0.5 * (2.0 * PI * 4.0 * t).sin() +  // 4 Hz component  
            0.2 * (2.0 * PI * 8.0 * t).sin();   // 8 Hz component
        vibration_signal.push(signal);
    }
    
    facts frequency_spectrum = fast_fourier_transform(&vibration_signal)?;
    
    println("   Frequency analysis (magnitude spectrum):");
    lowkey (sus i = 0; i < frequency_spectrum.len() / 2; i++) { // Only positive frequencies
        facts frequency = i as f64 * sampling_rate / time_points as f64;
        facts magnitude = frequency_spectrum[i].magnitude() / time_points as f64 * 2.0;
        printf("   {:.1} Hz: {:.3}", &[frequency, magnitude]);
    }
    
    // Control system simulation
    println("2. Control System Response:");
    println("   Second-order system step response approximation");
    
    // Simulate step response of damped harmonic oscillator
    facts time_samples = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    sus response_data = vec![];
    
    // Step response: y(t) = 1 - e^(-ζωₙt) * (cos(ωdt) + (ζ/√(1-ζ²)) * sin(ωdt))
    facts zeta = 0.5;     // Damping ratio
    facts omega_n = 5.0;  // Natural frequency
    facts omega_d = omega_n * (1.0 - zeta * zeta).sqrt(); // Damped frequency
    
    lowkey (sus t in time_samples) {
        facts exponential = (-zeta * omega_n * t).exp();
        facts cosine_term = (omega_d * t).cos();
        facts sine_term = (zeta / (1.0 - zeta * zeta).sqrt()) * (omega_d * t).sin();
        facts response = 1.0 - exponential * (cosine_term + sine_term);
        response_data.push(response);
        printf("   t={:.1}s: y={:.3}", &[t, response]);
    }
    
    println();
}

fr fr Demonstrates scientific computing applications
slay demonstrate_scientific_computing() {
    println("=== SCIENTIFIC COMPUTING DEMO ===");
    
    // Numerical integration for physics
    println("1. Physics: Work Calculation");
    println("   Work = ∫F(x)dx for variable force F(x) = 10 - 0.5x²");
    
    facts force_function = |x: f64| 10.0 - 0.5 * x * x;
    facts work = adaptive_simpson_integration(force_function, 0.0, 4.0, None, None)?;
    printf("   Work from x=0 to x=4: {:.3} Joules", &[work]);
    
    // Optimization for engineering design
    println("2. Engineering: Optimization Problem");
    println("   Minimize material cost: f(r,h) = 2πr² + 2πrh subject to πr²h = 1000");
    
    // Using substitution h = 1000/(πr²), minimize f(r) = 2πr² + 2000/r
    facts cost_function = |r: f64| 2.0 * PI * r * r + 2000.0 / r;
    facts optimal_radius = golden_section_search(cost_function, 0.1, 20.0, None)?;
    facts optimal_height = 1000.0 / (PI * optimal_radius * optimal_radius);
    facts minimum_cost = cost_function(optimal_radius);
    
    printf("   Optimal radius: {:.3} units", &[optimal_radius]);
    printf("   Optimal height: {:.3} units", &[optimal_height]);
    printf("   Minimum cost: {:.3} cost units", &[minimum_cost]);
    
    // Statistical analysis simulation
    println("3. Statistics: Data Analysis");
    println("   Fitting measurement data to theoretical model");
    
    // Simulate experimental data with noise
    facts measurement_x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    facts measurement_y = vec![2.9, 4.1, 5.8, 8.2, 10.1, 12.3]; // Roughly y = x² - x + 2
    
    facts fitted_coeffs = polynomial_fit(&measurement_x, &measurement_y, 2)?;
    printf("   Fitted model: y = {:.3} + {:.3}x + {:.3}x²", &[fitted_coeffs[0], fitted_coeffs[1], fitted_coeffs[2]]);
    
    // Calculate R-squared (coefficient of determination)
    sus residual_sum_squares = 0.0;
    sus total_sum_squares = 0.0;
    facts mean_y = measurement_y.iter().sum::<f64>() / measurement_y.len() as f64;
    
    lowkey (sus i = 0; i < measurement_x.len(); i++) {
        facts predicted = evaluate_polynomial(&fitted_coeffs, measurement_x[i])?;
        facts residual = measurement_y[i] - predicted;
        residual_sum_squares += residual * residual;
        
        facts deviation = measurement_y[i] - mean_y;
        total_sum_squares += deviation * deviation;
    }
    
    facts r_squared = 1.0 - residual_sum_squares / total_sum_squares;
    printf("   R-squared (goodness of fit): {:.3}", &[r_squared]);
    
    println();
}

fr fr Main demonstration function
slay main_character() {
    println("CURSED Advanced Mathematical Functions Demonstration");
    println("===================================================");
    println();
    
    demonstrate_numerical_methods();
    demonstrate_optimization();
    demonstrate_signal_processing();
    demonstrate_interpolation();
    demonstrate_matrix_operations();
    demonstrate_mathematical_modeling();
    demonstrate_engineering_applications();
    demonstrate_scientific_computing();
    
    println("=== DEMO COMPLETE ===");
    println("This demonstration showcased the comprehensive mathematical");
    println("capabilities available in the CURSED advanced math module.");
    println();
    println("Key capabilities demonstrated:");
    println("• Numerical differentiation and integration");
    println("• Multi-dimensional optimization algorithms");
    println("• Fast Fourier Transforms and signal processing");
    println("• Interpolation and function approximation");
    println("• Linear algebra and matrix operations");
    println("• Polynomial fitting and mathematical modeling");
    println("• Engineering and scientific computing applications");
    println();
    println("These tools enable CURSED to handle sophisticated mathematical");
    println("computations required in scientific, engineering, and");
    println("data analysis applications with high accuracy and performance.");
}
