/// Complex Number Mathematics Demo for CURSED
/// 
/// This demo showcases the comprehensive complex number support added to 
/// the CURSED math library, demonstrating all major complex mathematical
/// operations including basic arithmetic, transcendental functions,
/// trigonometric and hyperbolic functions, and advanced operations
/// like matrix operations and polynomial root finding.

import "stdlib::math"

slay main() {
    // ===== COMPLEX NUMBER CREATION AND BASIC OPERATIONS =====
    vibez.spill("🔢 CURSED Complex Number Mathematics Demo")
    vibez.spill("==========================================")
    
    // Creating complex numbers
    sus z1 = math.complex(3.0, 4.0) // 3 + 4i
    sus z2 = math.complex(1.0, 2.0) // 1 + 2i
    sus zero = math.Complex64.zero()
    sus one = math.Complex64.one()
    sus i = math.Complex64.i()
    
    vibez.spill("\n📊 Basic Complex Numbers:")
    vibez.spill("z1 = %s", z1)
    vibez.spill("z2 = %s", z2)
    vibez.spill("0 = %s", zero)
    vibez.spill("1 = %s", one)
    vibez.spill("i = %s", i)
    
    // Accessing real and imaginary parts
    vibez.spill("\n🔍 Real and Imaginary Parts:")
    vibez.spill("Re(z1) = %.6f", math.real(z1))
    vibez.spill("Im(z1) = %.6f", math.imag(z1))
    
    // ===== ABSOLUTE VALUE AND PHASE =====
    vibez.spill("\n📐 Absolute Value and Phase:")
    sus abs_z1 = math.complex_abs(z1)
    sus phase_z1 = math.phase(z1)
    
    vibez.spill("|z1| = %.6f", abs_z1)
    vibez.spill("arg(z1) = %.6f radians", phase_z1)
    vibez.spill("arg(z1) = %.6f degrees", phase_z1 * 180.0 / math.PI)
    
    // Complex conjugate
    sus conj_z1 = math.conj(z1)
    vibez.spill("z1* = %s", conj_z1)
    
    // ===== POLAR AND RECTANGULAR CONVERSION =====
    vibez.spill("\n🎯 Polar and Rectangular Conversion:")
    sus (r, theta) = math.polar(z1)
    vibez.spill("z1 in polar form: %.6f ∠ %.6f radians", r, theta)
    
    sus z_from_polar = math.rect(5.0, math.PI / 4.0) // 5∠45°
    vibez.spill("5 ∠ π/4 = %s", z_from_polar)
    
    // ===== EXPONENTIAL AND LOGARITHMIC FUNCTIONS =====
    vibez.spill("\n📈 Exponential and Logarithmic Functions:")
    
    // Euler's identity: e^(iπ) + 1 = 0
    sus euler_exp = math.complex_exp(math.complex(0.0, math.PI))
    vibez.spill("e^(iπ) = %s", euler_exp)
    vibez.spill("e^(iπ) + 1 = %s", math.complex_add(euler_exp, one))
    
    // Natural exponential and logarithm
    sus exp_z1 = math.complex_exp(z1)
    vibez.spill("e^z1 = %s", exp_z1)
    
    sus log_z1 = math.complex_log(z1)
    vibez.spill("ln(z1) = %s", log_z1)
    
    sus log10_z1 = math.complex_log10(z1)
    vibez.spill("log₁₀(z1) = %s", log10_z1)
    
    // Power functions
    vibez.spill("\n⚡ Power Functions:")
    sus i_squared = math.complex_pow(i, math.complex(2.0, 0.0))
    vibez.spill("i² = %s", i_squared)
    
    sus z1_to_z2 = math.complex_pow(z1, z2)
    vibez.spill("z1^z2 = %s", z1_to_z2)
    
    // Square root
    sus sqrt_neg_one = math.complex_sqrt(math.complex(-1.0, 0.0))
    vibez.spill("√(-1) = %s", sqrt_neg_one)
    
    sus sqrt_z1 = math.complex_sqrt(z1)
    vibez.spill("√z1 = %s", sqrt_z1)
    
    // ===== TRIGONOMETRIC FUNCTIONS =====
    vibez.spill("\n📐 Trigonometric Functions:")
    sus sin_z1 = math.complex_sin(z1)
    sus cos_z1 = math.complex_cos(z1)
    sus tan_z1 = math.complex_tan(z1)
    
    vibez.spill("sin(z1) = %s", sin_z1)
    vibez.spill("cos(z1) = %s", cos_z1)
    vibez.spill("tan(z1) = %s", tan_z1)
    
    // Verify trigonometric identity: sin²(z) + cos²(z) = 1
    sus sin_squared = math.complex_pow(sin_z1, math.complex(2.0, 0.0))
    sus cos_squared = math.complex_pow(cos_z1, math.complex(2.0, 0.0))
    sus trig_identity = math.complex_add(sin_squared, cos_squared)
    vibez.spill("sin²(z1) + cos²(z1) = %s", trig_identity)
    
    // ===== HYPERBOLIC FUNCTIONS =====
    vibez.spill("\n🌊 Hyperbolic Functions:")
    sus sinh_z1 = math.complex_sinh(z1)
    sus cosh_z1 = math.complex_cosh(z1)
    sus tanh_z1 = math.complex_tanh(z1)
    
    vibez.spill("sinh(z1) = %s", sinh_z1)
    vibez.spill("cosh(z1) = %s", cosh_z1)
    vibez.spill("tanh(z1) = %s", tanh_z1)
    
    // Verify hyperbolic identity: cosh²(z) - sinh²(z) = 1
    sus cosh_squared = math.complex_pow(cosh_z1, math.complex(2.0, 0.0))
    sus sinh_squared = math.complex_pow(sinh_z1, math.complex(2.0, 0.0))
    sus hyp_identity = math.complex_sub(cosh_squared, sinh_squared)
    vibez.spill("cosh²(z1) - sinh²(z1) = %s", hyp_identity)
    
    // ===== INVERSE FUNCTIONS =====
    vibez.spill("\n↩️ Inverse Trigonometric Functions:")
    sus small_z = math.complex(0.5, 0.3)
    
    sus asin_z = math.complex_asin(small_z)
    sus acos_z = math.complex_acos(small_z)
    sus atan_z = math.complex_atan(small_z)
    
    vibez.spill("asin(%.1f + %.1fi) = %s", math.real(small_z), math.imag(small_z), asin_z)
    vibez.spill("acos(%.1f + %.1fi) = %s", math.real(small_z), math.imag(small_z), acos_z)
    vibez.spill("atan(%.1f + %.1fi) = %s", math.real(small_z), math.imag(small_z), atan_z)
    
    // Verify inverse relationship: sin(asin(z)) = z
    sus sin_asin_z = math.complex_sin(asin_z)
    vibez.spill("sin(asin(z)) = %s", sin_asin_z)
    
    // ===== INVERSE HYPERBOLIC FUNCTIONS =====
    vibez.spill("\n↩️ Inverse Hyperbolic Functions:")
    sus asinh_z = math.complex_asinh(z2)
    sus acosh_z = math.complex_acosh(z1) // z1 has |z| > 1
    sus atanh_z = math.complex_atanh(small_z)
    
    vibez.spill("asinh(z2) = %s", asinh_z)
    vibez.spill("acosh(z1) = %s", acosh_z)
    vibez.spill("atanh(%.1f + %.1fi) = %s", math.real(small_z), math.imag(small_z), atanh_z)
    
    // ===== VECTOR OPERATIONS =====
    vibez.spill("\n📊 Complex Vector Operations:")
    sus complex_vector = [z1, z2, math.complex(2.0, 3.0), math.complex(0.0, 1.0)]
    
    sus vector_sum = math.complex_vector_sum(complex_vector)
    vibez.spill("Vector sum: %s", vector_sum)
    
    sus vector_product = math.complex_vector_product(complex_vector)
    vibez.spill("Vector product: %s", vector_product)
    
    // ===== MATRIX OPERATIONS =====
    vibez.spill("\n🔢 Complex Matrix Operations:")
    sus matrix_a = [
        [math.complex(1.0, 0.0), math.complex(2.0, 1.0)],
        [math.complex(0.0, 1.0), math.complex(3.0, 0.0)],
    ]
    
    sus matrix_b = [
        [math.complex(3.0, 0.0), math.complex(1.0, 1.0)],
        [math.complex(2.0, 1.0), math.complex(0.0, 2.0)],
    ]
    
    vibez.spill("Matrix A:")
    vibez.spill("  [%s, %s]", matrix_a[0][0], matrix_a[0][1])
    vibez.spill("  [%s, %s]", matrix_a[1][0], matrix_a[1][1])
    
    vibez.spill("Matrix B:")
    vibez.spill("  [%s, %s]", matrix_b[0][0], matrix_b[0][1])
    vibez.spill("  [%s, %s]", matrix_b[1][0], matrix_b[1][1])
    
    sus matrix_c = math.complex_matrix_mul_2x2(matrix_a, matrix_b)
    vibez.spill("Matrix C = A × B:")
    vibez.spill("  [%s, %s]", matrix_c[0][0], matrix_c[0][1])
    vibez.spill("  [%s, %s]", matrix_c[1][0], matrix_c[1][1])
    
    sus det_a = math.complex_determinant_2x2(matrix_a)
    vibez.spill("det(A) = %s", det_a)
    
    // ===== POLYNOMIAL OPERATIONS =====
    vibez.spill("\n📈 Complex Polynomial Operations:")
    
    // Quadratic equation: z² - 1 = 0 (roots should be ±1)
    sus a_coeff = math.complex(1.0, 0.0)
    sus b_coeff = math.complex(0.0, 0.0)
    sus c_coeff = math.complex(-1.0, 0.0)
    
    vibez.spill("Solving z² - 1 = 0:")
    sus (root1, root2) = math.quadratic_roots(a_coeff, b_coeff, c_coeff)
    vibez.spill("Root 1: %s", root1)
    vibez.spill("Root 2: %s", root2)
    
    // Quadratic equation: z² + 1 = 0 (roots should be ±i)
    sus c_coeff2 = math.complex(1.0, 0.0)
    vibez.spill("\nSolving z² + 1 = 0:")
    sus (root3, root4) = math.quadratic_roots(a_coeff, b_coeff, c_coeff2)
    vibez.spill("Root 1: %s", root3)
    vibez.spill("Root 2: %s", root4)
    
    // Polynomial evaluation: P(z) = z² + 2z + 1 = (z + 1)²
    sus poly_coeffs = [
        math.complex(1.0, 0.0), // constant term
        math.complex(2.0, 0.0), // linear term
        math.complex(1.0, 0.0), // quadratic term
    ]
    
    sus eval_point = math.complex(-1.0, 0.0)
    sus poly_value = math.complex_evaluate_polynomial(poly_coeffs, eval_point)
    vibez.spill("\nP(z) = z² + 2z + 1")
    vibez.spill("P(-1) = %s", poly_value) // Should be 0
    
    sus eval_point2 = math.complex(1.0, 0.0)
    sus poly_value2 = math.complex_evaluate_polynomial(poly_coeffs, eval_point2)
    vibez.spill("P(1) = %s", poly_value2) // Should be 4
    
    // ===== MATHEMATICAL CONSTANTS WITH COMPLEX NUMBERS =====
    vibez.spill("\n🔬 Mathematical Constants and Complex Numbers:")
    
    // e^(iπ) = -1 (Euler's identity)
    sus euler_identity = math.complex_exp(math.complex(0.0, math.PI))
    vibez.spill("e^(iπ) = %s", euler_identity)
    
    // e^(i*π/2) = i
    sus quarter_turn = math.complex_exp(math.complex(0.0, math.PI / 2.0))
    vibez.spill("e^(iπ/2) = %s", quarter_turn)
    
    // ln(i) = iπ/2
    sus ln_i = math.complex_log(i)
    vibez.spill("ln(i) = %s", ln_i)
    
    // ===== PERFORMANCE AND EDGE CASES =====
    vibez.spill("\n⚠️ Edge Cases and Error Handling:")
    
    // Demonstrate error handling
    vibez.spill("Testing error cases:")
    
    // This would cause an error: log(0)
    // sus log_zero = math.complex_log(zero) // Would return error
    vibez.spill("log(0) is undefined (would cause error)")
    
    // This would cause an error: 0^0
    // sus zero_to_zero = math.complex_pow(zero, zero) // Would return error
    vibez.spill("0^0 is undefined (would cause error)")
    
    // Large numbers
    sus large_real = math.complex(1000.0, 0.0)
    sus exp_large = math.complex_exp(large_real)
    vibez.spill("e^1000 = %s", exp_large)
    
    // ===== PRACTICAL APPLICATIONS =====
    vibez.spill("\n🎯 Practical Applications:")
    
    // Signal processing: complex sinusoid
    sus frequency = 2.0 * math.PI * 5.0 // 5 Hz
    sus time = 0.1 // 0.1 seconds
    sus complex_sinusoid = math.complex_exp(math.complex(0.0, frequency * time))
    vibez.spill("Complex sinusoid e^(i*2π*5*0.1) = %s", complex_sinusoid)
    
    // Impedance calculation (electrical engineering)
    sus resistance = math.complex(100.0, 0.0) // 100 ohms
    sus reactance = math.complex(0.0, 50.0)   // 50 ohms inductive
    sus impedance = math.complex_add(resistance, reactance)
    sus impedance_magnitude = math.complex_abs(impedance)
    sus impedance_phase = math.phase(impedance)
    
    vibez.spill("\nElectrical Impedance:")
    vibez.spill("Z = R + jX = %s", impedance)
    vibez.spill("|Z| = %.2f ohms", impedance_magnitude)
    vibez.spill("∠Z = %.2f degrees", impedance_phase * 180.0 / math.PI)
    
    // Quantum mechanics: complex probability amplitudes
    sus amplitude1 = math.complex(0.6, 0.8)
    sus amplitude2 = math.complex(0.8, -0.6)
    sus prob1 = math.complex_abs(amplitude1) * math.complex_abs(amplitude1)
    sus prob2 = math.complex_abs(amplitude2) * math.complex_abs(amplitude2)
    
    vibez.spill("\nQuantum Probability Amplitudes:")
    vibez.spill("ψ₁ = %s, |ψ₁|² = %.3f", amplitude1, prob1)
    vibez.spill("ψ₂ = %s, |ψ₂|² = %.3f", amplitude2, prob2)
    
    vibez.spill("\n✨ Complex number mathematics demo completed!")
    vibez.spill("CURSED now has comprehensive complex number support! 🎉")
}
