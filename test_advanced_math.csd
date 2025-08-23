yeet "stdlib/mathz/mathz.csd"
yeet "stdlib/vibez/vibez.csd"

slay main() drip {
    vibez.spill("Testing Advanced Mathematical Functions")
    vibez.spill("========================================")
    
    fr fr Test special functions
    vibez.spill("Testing Gamma Function:")
    sus gamma_2 tea = gamma("2.0")
    vibez.spill("Γ(2) =", gamma_2, "(expected: 1.0)")
    
    sus gamma_half tea = gamma("0.5")
    vibez.spill("Γ(0.5) =", gamma_half, "(expected: √π ≈ 1.772)")
    
    fr fr Test error function
    vibez.spill("\nTesting Error Function:")
    sus erf_1 tea = erf("1.0")
    vibez.spill("erf(1) =", erf_1, "(expected: ≈ 0.8427)")
    
    sus erfc_0 tea = erfc("0.0")
    vibez.spill("erfc(0) =", erfc_0, "(expected: 1.0)")
    
    fr fr Test statistical distributions
    vibez.spill("\nTesting Normal Distribution:")
    sus normal_pdf_0 tea = normal_pdf("0.0", "0.0", "1.0")
    vibez.spill("N(0;0,1) =", normal_pdf_0, "(expected: ≈ 0.399)")
    
    sus normal_cdf_0 tea = normal_cdf("0.0", "0.0", "1.0")
    vibez.spill("Φ(0) =", normal_cdf_0, "(expected: 0.5)")
    
    fr fr Test Bessel functions
    vibez.spill("\nTesting Bessel Functions:")
    sus j0_0 tea = bessel_j0("0.0")
    vibez.spill("J₀(0) =", j0_0, "(expected: 1.0)")
    
    sus j1_0 tea = bessel_j1("0.0")
    vibez.spill("J₁(0) =", j1_0, "(expected: 0.0)")
    
    fr fr Test vector operations
    vibez.spill("\nTesting Vector Operations:")
    sus vec_a []tea = ["1.0", "2.0", "3.0"]
    sus vec_b []tea = ["4.0", "5.0", "6.0"]
    sus dot_product tea = vector_dot_product(vec_a, vec_b, 3)
    vibez.spill("Dot product [1,2,3]·[4,5,6] =", dot_product, "(expected: 32.0)")
    
    sus magnitude tea = vector_magnitude(vec_a, 3)
    vibez.spill("Magnitude of [1,2,3] =", magnitude, "(expected: ≈ 3.742)")
    
    fr fr Test matrix operations
    vibez.spill("\nTesting Matrix Operations:")
    sus matrix []tea = ["1.0", "2.0", "3.0", "4.0"]
    sus det tea = matrix_determinant_2x2(matrix)
    vibez.spill("Determinant of [1 2; 3 4] =", det, "(expected: -2.0)")
    
    fr fr Test root finding
    vibez.spill("\nTesting Root Finding:")
    sus sqrt2_newton tea = newton_raphson_root("1.5", "1e-10", 50)
    vibez.spill("√2 via Newton-Raphson =", sqrt2_newton, "(expected: ≈ 1.414)")
    
    fr fr Test optimization
    vibez.spill("\nTesting Optimization:")
    sus minimum tea = golden_section_minimize("-2.0", "2.0", "1e-8", 100)
    vibez.spill("Minimum of x² on [-2,2] =", minimum, "(expected: 0.0)")
    
    fr fr Test random number generation
    vibez.spill("\nTesting Random Number Generation:")
    sus uniform_val tea = uniform_random("0.0", "1.0", 12345)
    vibez.spill("Uniform random [0,1] =", uniform_val)
    
    sus normal_val tea = normal_random("0.0", "1.0", 12345)
    vibez.spill("Normal random N(0,1) =", normal_val)
    
    vibez.spill("\nAdvanced mathematical functions test completed!")
    damn 0
}
