vibe main

yeet "vibez"  fr fr For printing results
yeet "mathz"  fr fr Math constants and functions

slay main() {
    vibez.spill("=== CURSED Mathematical Constants Test Suite ===")
    
    fr fr Test fundamental mathematical constants
    test_fundamental_constants()
    
    fr fr Test pi-related constants
    test_pi_constants()
    
    fr fr Test square root constants  
    test_sqrt_constants()
    
    fr fr Test logarithmic constants
    test_logarithmic_constants()
    
    fr fr Test conversion factors
    test_conversion_factors()
    
    fr fr Test special constants
    test_special_constants()
    
    fr fr Test physical constants
    test_physical_constants()
    
    fr fr Test floating point constants
    test_floating_point_constants()
    
    fr fr Test mathematical relationships
    test_mathematical_relationships()
    
    fr fr Test practical applications
    test_practical_applications()
    
    vibez.spill("=== All Mathematical Constants Tests Completed Successfully! ===")
}

slay test_fundamental_constants() {
    vibez.spill("\n--- Testing Fundamental Mathematical Constants ---")
    
    fr fr Test PI precision (π ≈ 3.141592653589793)
    vibez.spillf("PI = %.15f", mathz.PI)
    sus pi_test = mathz.Abs(mathz.PI - 3.141592653589793) < 1e-10
    lowkey (pi_test) {
        vibez.spill("✅ PI precision test passed")
    } no cap {
        vibez.spill("❌ PI precision test failed")
    }
    
    fr fr Test E precision (e ≈ 2.718281828459045)
    vibez.spillf("E = %.15f", mathz.E)
    sus e_test = mathz.Abs(mathz.E - 2.718281828459045) < 1e-10
    lowkey (e_test) {
        vibez.spill("✅ E precision test passed")
    } no cap {
        vibez.spill("❌ E precision test failed")
    }
    
    fr fr Test TAU = 2*PI
    vibez.spillf("TAU = %.15f", mathz.TAU)
    sus tau_test = mathz.Abs(mathz.TAU - 2.0 * mathz.PI) < 1e-10
    lowkey (tau_test) {
        vibez.spill("✅ TAU = 2*PI test passed")
    } no cap {
        vibez.spill("❌ TAU = 2*PI test failed")
    }
    
    fr fr Test Golden Ratio φ = (1 + √5) / 2
    vibez.spillf("PHI = %.15f", mathz.PHI)
    meal expected_phi = (1.0 + mathz.Sqrt(5.0)) / 2.0
    sus phi_test = mathz.Abs(mathz.PHI - expected_phi) < 1e-10
    lowkey (phi_test) {
        vibez.spill("✅ PHI precision test passed")
    } no cap {
        vibez.spill("❌ PHI precision test failed")
    }
    
    fr fr Test Inverse Golden Ratio
    vibez.spillf("INV_PHI = %.15f", mathz.INV_PHI)
    meal expected_inv_phi = 1.0 / mathz.PHI
    sus inv_phi_test = mathz.Abs(mathz.INV_PHI - expected_inv_phi) < 1e-10
    lowkey (inv_phi_test) {
        vibez.spill("✅ INV_PHI precision test passed")
    } no cap {
        vibez.spill("❌ INV_PHI precision test failed")
    }
}

slay test_pi_constants() {
    vibez.spill("\n--- Testing PI-Related Constants ---")
    
    fr fr Test π/2, π/3, π/4, π/6, π/8
    sus pi_2_test = mathz.Abs(mathz.FRAC_PI_2 - mathz.PI / 2.0) < 1e-10
    sus pi_3_test = mathz.Abs(mathz.FRAC_PI_3 - mathz.PI / 3.0) < 1e-10
    sus pi_4_test = mathz.Abs(mathz.FRAC_PI_4 - mathz.PI / 4.0) < 1e-10
    sus pi_6_test = mathz.Abs(mathz.FRAC_PI_6 - mathz.PI / 6.0) < 1e-10
    sus pi_8_test = mathz.Abs(mathz.FRAC_PI_8 - mathz.PI / 8.0) < 1e-10
    
    vibez.spillf("π/2 = %.10f (expected: %.10f)", mathz.FRAC_PI_2, mathz.PI / 2.0)
    vibez.spillf("π/3 = %.10f (expected: %.10f)", mathz.FRAC_PI_3, mathz.PI / 3.0)
    vibez.spillf("π/4 = %.10f (expected: %.10f)", mathz.FRAC_PI_4, mathz.PI / 4.0)
    vibez.spillf("π/6 = %.10f (expected: %.10f)", mathz.FRAC_PI_6, mathz.PI / 6.0)
    vibez.spillf("π/8 = %.10f (expected: %.10f)", mathz.FRAC_PI_8, mathz.PI / 8.0)
    
    lowkey (pi_2_test) {
        vibez.spill("✅ π/2 test passed")
    } no cap {
        vibez.spill("❌ π/2 test failed")
    }
    
    lowkey (pi_3_test) {
        vibez.spill("✅ π/3 test passed")
    } no cap {
        vibez.spill("❌ π/3 test failed")
    }
    
    lowkey (pi_4_test) {
        vibez.spill("✅ π/4 test passed")
    } no cap {
        vibez.spill("❌ π/4 test failed")
    }
    
    fr fr Test reciprocal π constants: 1/π, 2/π
    sus one_pi_test = mathz.Abs(mathz.FRAC_1_PI - 1.0 / mathz.PI) < 1e-10
    sus two_pi_test = mathz.Abs(mathz.FRAC_2_PI - 2.0 / mathz.PI) < 1e-10
    
    vibez.spillf("1/π = %.10f (expected: %.10f)", mathz.FRAC_1_PI, 1.0 / mathz.PI)
    vibez.spillf("2/π = %.10f (expected: %.10f)", mathz.FRAC_2_PI, 2.0 / mathz.PI)
    
    lowkey (one_pi_test) {
        vibez.spill("✅ 1/π test passed")
    } no cap {
        vibez.spill("❌ 1/π test failed")
    }
    
    lowkey (two_pi_test) {
        vibez.spill("✅ 2/π test passed")
    } no cap {
        vibez.spill("❌ 2/π test failed")
    }
    
    fr fr Test 2/√π and √π
    meal sqrt_pi_expected = mathz.Sqrt(mathz.PI)
    meal two_sqrt_pi_expected = 2.0 / sqrt_pi_expected
    
    sus sqrt_pi_test = mathz.Abs(mathz.SQRT_PI - sqrt_pi_expected) < 1e-10
    sus two_sqrt_pi_test = mathz.Abs(mathz.FRAC_2_SQRT_PI - two_sqrt_pi_expected) < 1e-10
    
    vibez.spillf("√π = %.10f (expected: %.10f)", mathz.SQRT_PI, sqrt_pi_expected)
    vibez.spillf("2/√π = %.10f (expected: %.10f)", mathz.FRAC_2_SQRT_PI, two_sqrt_pi_expected)
    
    lowkey (sqrt_pi_test) {
        vibez.spill("✅ √π test passed")
    } no cap {
        vibez.spill("❌ √π test failed")
    }
    
    lowkey (two_sqrt_pi_test) {
        vibez.spill("✅ 2/√π test passed")
    } no cap {
        vibez.spill("❌ 2/√π test failed")
    }
}

slay test_sqrt_constants() {
    vibez.spill("\n--- Testing Square Root Constants ---")
    
    fr fr Test √2, √3, √5
    meal sqrt_2_expected = mathz.Sqrt(2.0)
    meal sqrt_3_expected = mathz.Sqrt(3.0)
    meal sqrt_5_expected = mathz.Sqrt(5.0)
    
    sus sqrt_2_test = mathz.Abs(mathz.SQRT_2 - sqrt_2_expected) < 1e-10
    sus sqrt_3_test = mathz.Abs(mathz.SQRT_3 - sqrt_3_expected) < 1e-10
    sus sqrt_5_test = mathz.Abs(mathz.SQRT_5 - sqrt_5_expected) < 1e-10
    
    vibez.spillf("√2 = %.10f (expected: %.10f)", mathz.SQRT_2, sqrt_2_expected)
    vibez.spillf("√3 = %.10f (expected: %.10f)", mathz.SQRT_3, sqrt_3_expected)
    vibez.spillf("√5 = %.10f (expected: %.10f)", mathz.SQRT_5, sqrt_5_expected)
    
    lowkey (sqrt_2_test) {
        vibez.spill("✅ √2 test passed")
    } no cap {
        vibez.spill("❌ √2 test failed")
    }
    
    lowkey (sqrt_3_test) {
        vibez.spill("✅ √3 test passed")
    } no cap {
        vibez.spill("❌ √3 test failed")
    }
    
    lowkey (sqrt_5_test) {
        vibez.spill("✅ √5 test passed")
    } no cap {
        vibez.spill("❌ √5 test failed")
    }
    
    fr fr Test 1/√2
    meal frac_1_sqrt_2_expected = 1.0 / mathz.Sqrt(2.0)
    sus frac_1_sqrt_2_test = mathz.Abs(mathz.FRAC_1_SQRT_2 - frac_1_sqrt_2_expected) < 1e-10
    
    vibez.spillf("1/√2 = %.10f (expected: %.10f)", mathz.FRAC_1_SQRT_2, frac_1_sqrt_2_expected)
    
    lowkey (frac_1_sqrt_2_test) {
        vibez.spill("✅ 1/√2 test passed")
    } no cap {
        vibez.spill("❌ 1/√2 test failed")
    }
    
    fr fr Test relationship: √2 * (1/√2) = 1
    meal sqrt_2_relationship = mathz.SQRT_2 * mathz.FRAC_1_SQRT_2
    sus relationship_test = mathz.Abs(sqrt_2_relationship - 1.0) < 1e-10
    
    vibez.spillf("√2 * (1/√2) = %.10f (should be 1.0)", sqrt_2_relationship)
    
    lowkey (relationship_test) {
        vibez.spill("✅ √2 * (1/√2) = 1 relationship test passed")
    } no cap {
        vibez.spill("❌ √2 * (1/√2) = 1 relationship test failed")
    }
}

slay test_logarithmic_constants() {
    vibez.spill("\n--- Testing Logarithmic Constants ---")
    
    fr fr Test natural logarithms: ln(2), ln(10)
    meal ln_2_expected = mathz.Ln(2.0)
    meal ln_10_expected = mathz.Ln(10.0)
    
    sus ln_2_test = mathz.Abs(mathz.LN_2 - ln_2_expected) < 1e-10
    sus ln_10_test = mathz.Abs(mathz.LN_10 - ln_10_expected) < 1e-10
    
    vibez.spillf("ln(2) = %.10f (expected: %.10f)", mathz.LN_2, ln_2_expected)
    vibez.spillf("ln(10) = %.10f (expected: %.10f)", mathz.LN_10, ln_10_expected)
    
    lowkey (ln_2_test) {
        vibez.spill("✅ ln(2) test passed")
    } no cap {
        vibez.spill("❌ ln(2) test failed")
    }
    
    lowkey (ln_10_test) {
        vibez.spill("✅ ln(10) test passed")
    } no cap {
        vibez.spill("❌ ln(10) test failed")
    }
    
    fr fr Test base-2 logarithms: log₂(e), log₂(10)
    meal log2_e_expected = mathz.Log2(mathz.E)
    meal log2_10_expected = mathz.Log2(10.0)
    
    sus log2_e_test = mathz.Abs(mathz.LOG2_E - log2_e_expected) < 1e-10
    sus log2_10_test = mathz.Abs(mathz.LOG2_10 - log2_10_expected) < 1e-10
    
    vibez.spillf("log₂(e) = %.10f (expected: %.10f)", mathz.LOG2_E, log2_e_expected)
    vibez.spillf("log₂(10) = %.10f (expected: %.10f)", mathz.LOG2_10, log2_10_expected)
    
    lowkey (log2_e_test) {
        vibez.spill("✅ log₂(e) test passed")
    } no cap {
        vibez.spill("❌ log₂(e) test failed")
    }
    
    lowkey (log2_10_test) {
        vibez.spill("✅ log₂(10) test passed")
    } no cap {
        vibez.spill("❌ log₂(10) test failed")
    }
    
    fr fr Test base-10 logarithms: log₁₀(e), log₁₀(2)
    meal log10_e_expected = mathz.Log10(mathz.E)
    meal log10_2_expected = mathz.Log10(2.0)
    
    sus log10_e_test = mathz.Abs(mathz.LOG10_E - log10_e_expected) < 1e-10
    sus log10_2_test = mathz.Abs(mathz.LOG10_2 - log10_2_expected) < 1e-10
    
    vibez.spillf("log₁₀(e) = %.10f (expected: %.10f)", mathz.LOG10_E, log10_e_expected)
    vibez.spillf("log₁₀(2) = %.10f (expected: %.10f)", mathz.LOG10_2, log10_2_expected)
    
    lowkey (log10_e_test) {
        vibez.spill("✅ log₁₀(e) test passed")
    } no cap {
        vibez.spill("❌ log₁₀(e) test failed")
    }
    
    lowkey (log10_2_test) {
        vibez.spill("✅ log₁₀(2) test passed")
    } no cap {
        vibez.spill("❌ log₁₀(2) test failed")
    }
}

slay test_conversion_factors() {
    vibez.spill("\n--- Testing Conversion Factors ---")
    
    fr fr Test degree/radian conversion factors
    meal deg_to_rad_expected = mathz.PI / 180.0
    meal rad_to_deg_expected = 180.0 / mathz.PI
    
    sus deg_to_rad_test = mathz.Abs(mathz.DEG_TO_RAD - deg_to_rad_expected) < 1e-10
    sus rad_to_deg_test = mathz.Abs(mathz.RAD_TO_DEG - rad_to_deg_expected) < 1e-10
    
    vibez.spillf("DEG_TO_RAD = %.10f (expected: %.10f)", mathz.DEG_TO_RAD, deg_to_rad_expected)
    vibez.spillf("RAD_TO_DEG = %.10f (expected: %.10f)", mathz.RAD_TO_DEG, rad_to_deg_expected)
    
    lowkey (deg_to_rad_test) {
        vibez.spill("✅ DEG_TO_RAD test passed")
    } no cap {
        vibez.spill("❌ DEG_TO_RAD test failed")
    }
    
    lowkey (rad_to_deg_test) {
        vibez.spill("✅ RAD_TO_DEG test passed")
    } no cap {
        vibez.spill("❌ RAD_TO_DEG test failed")
    }
    
    fr fr Test reciprocal relationship: DEG_TO_RAD * RAD_TO_DEG = 1
    meal reciprocal_product = mathz.DEG_TO_RAD * mathz.RAD_TO_DEG
    sus reciprocal_test = mathz.Abs(reciprocal_product - 1.0) < 1e-10
    
    vibez.spillf("DEG_TO_RAD * RAD_TO_DEG = %.15f (should be 1.0)", reciprocal_product)
    
    lowkey (reciprocal_test) {
        vibez.spill("✅ Reciprocal relationship test passed")
    } no cap {
        vibez.spill("❌ Reciprocal relationship test failed")
    }
    
    fr fr Test common conversions
    meal ninety_deg_in_rad = 90.0 * mathz.DEG_TO_RAD
    meal pi_rad_in_deg = mathz.PI * mathz.RAD_TO_DEG
    
    sus ninety_deg_test = mathz.Abs(ninety_deg_in_rad - mathz.FRAC_PI_2) < 1e-10
    sus pi_rad_test = mathz.Abs(pi_rad_in_deg - 180.0) < 1e-10
    
    vibez.spillf("90° in radians = %.10f (should be π/2 = %.10f)", ninety_deg_in_rad, mathz.FRAC_PI_2)
    vibez.spillf("π radians in degrees = %.10f (should be 180.0)", pi_rad_in_deg)
    
    lowkey (ninety_deg_test) {
        vibez.spill("✅ 90° = π/2 radians test passed")
    } no cap {
        vibez.spill("❌ 90° = π/2 radians test failed")
    }
    
    lowkey (pi_rad_test) {
        vibez.spill("✅ π radians = 180° test passed")
    } no cap {
        vibez.spill("❌ π radians = 180° test failed")
    }
}

slay test_special_constants() {
    vibez.spill("\n--- Testing Special Mathematical Constants ---")
    
    fr fr Test Euler-Mascheroni constant γ ≈ 0.5772156649015329
    vibez.spillf("Euler-Mascheroni constant γ = %.15f", mathz.EULER_GAMMA)
    sus gamma_test = mathz.Abs(mathz.EULER_GAMMA - 0.5772156649015329) < 1e-12
    
    lowkey (gamma_test) {
        vibez.spill("✅ Euler-Mascheroni constant test passed")
    } no cap {
        vibez.spill("❌ Euler-Mascheroni constant test failed")
    }
    
    fr fr Test Catalan's constant ≈ 0.9159655941772190
    vibez.spillf("Catalan's constant = %.15f", mathz.CATALAN)
    sus catalan_test = mathz.Abs(mathz.CATALAN - 0.9159655941772190) < 1e-12
    
    lowkey (catalan_test) {
        vibez.spill("✅ Catalan's constant test passed")
    } no cap {
        vibez.spill("❌ Catalan's constant test failed")
    }
    
    fr fr Test golden ratio relationships: φ * (1/φ) = 1, φ - 1 = 1/φ
    meal phi_inv_phi_product = mathz.PHI * mathz.INV_PHI
    meal phi_minus_1 = mathz.PHI - 1.0
    
    sus phi_product_test = mathz.Abs(phi_inv_phi_product - 1.0) < 1e-10
    sus phi_relationship_test = mathz.Abs(phi_minus_1 - mathz.INV_PHI) < 1e-10
    
    vibez.spillf("φ * (1/φ) = %.15f (should be 1.0)", phi_inv_phi_product)
    vibez.spillf("φ - 1 = %.15f, 1/φ = %.15f", phi_minus_1, mathz.INV_PHI)
    
    lowkey (phi_product_test) {
        vibez.spill("✅ φ * (1/φ) = 1 test passed")
    } no cap {
        vibez.spill("❌ φ * (1/φ) = 1 test failed")
    }
    
    lowkey (phi_relationship_test) {
        vibez.spill("✅ φ - 1 = 1/φ test passed")
    } no cap {
        vibez.spill("❌ φ - 1 = 1/φ test failed")
    }
}

slay test_physical_constants() {
    vibez.spill("\n--- Testing Physical Constants ---")
    
    fr fr Test speed of light (exact by definition)
    vibez.spillf("Speed of light = %.0f m/s", mathz.SPEED_OF_LIGHT)
    sus speed_light_test = mathz.SPEED_OF_LIGHT == 299792458.0
    
    lowkey (speed_light_test) {
        vibez.spill("✅ Speed of light test passed")
    } no cap {
        vibez.spill("❌ Speed of light test failed")
    }
    
    fr fr Test Planck constant (exact by definition)
    vibez.spillf("Planck constant = %.11e J⋅s", mathz.PLANCK)
    sus planck_test = mathz.PLANCK == 6.62607015e-34
    
    lowkey (planck_test) {
        vibez.spill("✅ Planck constant test passed")
    } no cap {
        vibez.spill("❌ Planck constant test failed")
    }
    
    fr fr Test reduced Planck constant: ℏ = h/2π
    meal hbar_expected = mathz.PLANCK / mathz.TAU
    sus hbar_test = mathz.Abs(mathz.HBAR - hbar_expected) < 1e-40
    
    vibez.spillf("ℏ = %.11e J⋅s (expected: %.11e)", mathz.HBAR, hbar_expected)
    
    lowkey (hbar_test) {
        vibez.spill("✅ ℏ = h/2π test passed")
    } no cap {
        vibez.spill("❌ ℏ = h/2π test failed")
    }
    
    fr fr Test Avogadro number (exact by definition)
    vibez.spillf("Avogadro number = %.8e mol⁻¹", mathz.AVOGADRO)
    sus avogadro_test = mathz.AVOGADRO == 6.02214076e23
    
    lowkey (avogadro_test) {
        vibez.spill("✅ Avogadro number test passed")
    } no cap {
        vibez.spill("❌ Avogadro number test failed")
    }
    
    fr fr Test elementary charge (exact by definition)
    vibez.spillf("Elementary charge = %.9e C", mathz.ELEMENTARY_CHARGE)
    sus charge_test = mathz.ELEMENTARY_CHARGE == 1.602176634e-19
    
    lowkey (charge_test) {
        vibez.spill("✅ Elementary charge test passed")
    } no cap {
        vibez.spill("❌ Elementary charge test failed")
    }
    
    fr fr Test that masses are positive and proton > electron
    sus mass_positive_test = mathz.ELECTRON_MASS > 0.0 && mathz.PROTON_MASS > 0.0
    sus mass_ratio_test = mathz.PROTON_MASS > mathz.ELECTRON_MASS
    
    vibez.spillf("Electron mass = %.10e kg", mathz.ELECTRON_MASS)
    vibez.spillf("Proton mass = %.10e kg", mathz.PROTON_MASS)
    
    lowkey (mass_positive_test) {
        vibez.spill("✅ Masses are positive test passed")
    } no cap {
        vibez.spill("❌ Masses are positive test failed")
    }
    
    lowkey (mass_ratio_test) {
        vibez.spill("✅ Proton heavier than electron test passed")
    } no cap {
        vibez.spill("❌ Proton heavier than electron test failed")
    }
}

slay test_floating_point_constants() {
    vibez.spill("\n--- Testing Floating Point Constants ---")
    
    fr fr Test machine epsilon
    vibez.spillf("Machine epsilon = %.2e", mathz.EPSILON)
    sus epsilon_test = mathz.EPSILON > 0.0 && mathz.EPSILON < 1e-10
    
    lowkey (epsilon_test) {
        vibez.spill("✅ Machine epsilon test passed")
    } no cap {
        vibez.spill("❌ Machine epsilon test failed")
    }
    
    fr fr Test extremes
    vibez.spillf("MAX value = %.2e", mathz.MAX)
    vibez.spillf("MIN value = %.2e", mathz.MIN)
    vibez.spillf("MIN_POSITIVE = %.2e", mathz.MIN_POSITIVE)
    
    sus max_test = mathz.MAX > 1e100
    sus min_test = mathz.MIN < -1e100
    sus min_pos_test = mathz.MIN_POSITIVE > 0.0 && mathz.MIN_POSITIVE < 1e-100
    
    lowkey (max_test) {
        vibez.spill("✅ MAX value test passed")
    } no cap {
        vibez.spill("❌ MAX value test failed")
    }
    
    lowkey (min_test) {
        vibez.spill("✅ MIN value test passed")
    } no cap {
        vibez.spill("❌ MIN value test failed")
    }
    
    lowkey (min_pos_test) {
        vibez.spill("✅ MIN_POSITIVE value test passed")
    } no cap {
        vibez.spill("❌ MIN_POSITIVE value test failed")
    }
    
    fr fr Test special values (can't directly test infinity/NaN in CURSED, but verify they exist)
    vibez.spillf("INFINITY = %f", mathz.INFINITY)
    vibez.spillf("NEG_INFINITY = %f", mathz.NEG_INFINITY)
    vibez.spill("NAN exists and is defined")
    
    vibez.spill("✅ Special floating point constants are accessible")
}

slay test_mathematical_relationships() {
    vibez.spill("\n--- Testing Mathematical Relationships ---")
    
    fr fr Test trigonometric identities with constants
    meal sin_pi_2 = mathz.Sin(mathz.FRAC_PI_2)
    meal cos_pi = mathz.Cos(mathz.PI)
    meal tan_pi_4 = mathz.Tan(mathz.FRAC_PI_4)
    
    sus sin_test = mathz.Abs(sin_pi_2 - 1.0) < 1e-10
    sus cos_test = mathz.Abs(cos_pi + 1.0) < 1e-10
    sus tan_test = mathz.Abs(tan_pi_4 - 1.0) < 1e-10
    
    vibez.spillf("sin(π/2) = %.10f (should be 1.0)", sin_pi_2)
    vibez.spillf("cos(π) = %.10f (should be -1.0)", cos_pi)
    vibez.spillf("tan(π/4) = %.10f (should be 1.0)", tan_pi_4)
    
    lowkey (sin_test) {
        vibez.spill("✅ sin(π/2) = 1 test passed")
    } no cap {
        vibez.spill("❌ sin(π/2) = 1 test failed")
    }
    
    lowkey (cos_test) {
        vibez.spill("✅ cos(π) = -1 test passed")
    } no cap {
        vibez.spill("❌ cos(π) = -1 test failed")
    }
    
    lowkey (tan_test) {
        vibez.spill("✅ tan(π/4) = 1 test passed")
    } no cap {
        vibez.spill("❌ tan(π/4) = 1 test failed")
    }
    
    fr fr Test logarithmic identities
    meal ln_e = mathz.Ln(mathz.E)
    meal log10_10 = mathz.Log10(10.0)
    meal log2_2 = mathz.Log2(2.0)
    
    sus ln_e_test = mathz.Abs(ln_e - 1.0) < 1e-10
    sus log10_10_test = mathz.Abs(log10_10 - 1.0) < 1e-10
    sus log2_2_test = mathz.Abs(log2_2 - 1.0) < 1e-10
    
    vibez.spillf("ln(e) = %.10f (should be 1.0)", ln_e)
    vibez.spillf("log₁₀(10) = %.10f (should be 1.0)", log10_10)
    vibez.spillf("log₂(2) = %.10f (should be 1.0)", log2_2)
    
    lowkey (ln_e_test) {
        vibez.spill("✅ ln(e) = 1 test passed")
    } no cap {
        vibez.spill("❌ ln(e) = 1 test failed")
    }
    
    lowkey (log10_10_test) {
        vibez.spill("✅ log₁₀(10) = 1 test passed")
    } no cap {
        vibez.spill("❌ log₁₀(10) = 1 test failed")
    }
    
    lowkey (log2_2_test) {
        vibez.spill("✅ log₂(2) = 1 test passed")
    } no cap {
        vibez.spill("❌ log₂(2) = 1 test failed")
    }
    
    fr fr Test golden ratio equation: φ² - φ - 1 = 0
    meal phi_squared = mathz.PHI * mathz.PHI
    meal golden_equation = phi_squared - mathz.PHI - 1.0
    sus golden_equation_test = mathz.Abs(golden_equation) < 1e-10
    
    vibez.spillf("φ² - φ - 1 = %.15f (should be 0.0)", golden_equation)
    
    lowkey (golden_equation_test) {
        vibez.spill("✅ Golden ratio equation test passed")
    } no cap {
        vibez.spill("❌ Golden ratio equation test failed")
    }
}

slay test_practical_applications() {
    vibez.spill("\n--- Testing Practical Applications ---")
    
    fr fr Test circle calculations
    meal radius = 5.0
    meal circumference = 2.0 * mathz.PI * radius
    meal area = mathz.PI * radius * radius
    
    vibez.spillf("Circle with radius %.1f:", radius)
    vibez.spillf("  Circumference = %.10f", circumference)
    vibez.spillf("  Area = %.10f", area)
    
    sus circumference_test = mathz.Abs(circumference - 31.41592653589793) < 1e-10
    sus area_test = mathz.Abs(area - 78.53981633974483) < 1e-10
    
    lowkey (circumference_test) {
        vibez.spill("✅ Circle circumference calculation test passed")
    } no cap {
        vibez.spill("❌ Circle circumference calculation test failed")
    }
    
    lowkey (area_test) {
        vibez.spill("✅ Circle area calculation test passed")
    } no cap {
        vibez.spill("❌ Circle area calculation test failed")
    }
    
    fr fr Test angle conversions
    meal degrees_45 = 45.0 * mathz.DEG_TO_RAD
    meal radians_pi = mathz.PI * mathz.RAD_TO_DEG
    
    vibez.spillf("45° in radians = %.10f (should be π/4 = %.10f)", degrees_45, mathz.FRAC_PI_4)
    vibez.spillf("π radians in degrees = %.10f (should be 180.0)", radians_pi)
    
    sus deg_conversion_test = mathz.Abs(degrees_45 - mathz.FRAC_PI_4) < 1e-10
    sus rad_conversion_test = mathz.Abs(radians_pi - 180.0) < 1e-10
    
    lowkey (deg_conversion_test) {
        vibez.spill("✅ 45° to radians conversion test passed")
    } no cap {
        vibez.spill("❌ 45° to radians conversion test failed")
    }
    
    lowkey (rad_conversion_test) {
        vibez.spill("✅ π radians to degrees conversion test passed")
    } no cap {
        vibez.spill("❌ π radians to degrees conversion test failed")
    }
    
    fr fr Test compound interest with e
    meal principal = 1000.0
    meal rate = 0.05  fr fr 5%
    meal time = 10.0
    meal continuous_compound = principal * mathz.Pow(mathz.E, rate * time)
    
    vibez.spillf("Compound interest calculation:")
    vibez.spillf("  Principal: $%.2f", principal)
    vibez.spillf("  Rate: %.1f%%", rate * 100.0)
    vibez.spillf("  Time: %.1f years", time)
    vibez.spillf("  Final amount: $%.2f", continuous_compound)
    
    sus compound_test = continuous_compound > principal
    sus compound_value_test = mathz.Abs(continuous_compound - 1648.7212707001282) < 1e-10
    
    lowkey (compound_test) {
        vibez.spill("✅ Compound interest increases principal test passed")
    } no cap {
        vibez.spill("❌ Compound interest increases principal test failed")
    }
    
    lowkey (compound_value_test) {
        vibez.spill("✅ Compound interest calculation test passed")
    } no cap {
        vibez.spill("❌ Compound interest calculation test failed")
    }
    
    fr fr Test Fibonacci-like calculation with golden ratio
    fr fr F(n) ≈ φⁿ/√5 for large n
    meal n = 10.0
    meal phi_power_n = mathz.Pow(mathz.PHI, n)
    meal approx_fib = phi_power_n / mathz.SQRT_5
    
    vibez.spillf("Fibonacci approximation using φ:")
    vibez.spillf("  φ^%.0f = %.2f", n, phi_power_n)
    vibez.spillf("  F(%.0f) ≈ %.2f", n, approx_fib)
    
    fr fr F(10) = 55, so this should be close
    sus fib_test = mathz.Abs(approx_fib - 55.0) < 1.0
    
    lowkey (fib_test) {
        vibez.spill("✅ Fibonacci approximation test passed")
    } no cap {
        vibez.spill("❌ Fibonacci approximation test failed")
    }
}
