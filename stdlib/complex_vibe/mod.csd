// complex_vibe module - Pure CURSED complex number operations
// Provides basic mathematical functions for complex numbers

// Basic operations on complex numbers using separate real and imaginary parts
slay ComplexAdd(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    // Return real part of addition
    damn r1 + r2
}

slay ComplexAddImag(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    // Return imaginary part of addition
    damn i1 + i2
}

slay ComplexSub(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    // Return real part of subtraction
    damn r1 - r2
}

slay ComplexSubImag(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    // Return imaginary part of subtraction
    damn i1 - i2
}

slay ComplexMul(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    // Return real part of multiplication
    damn r1 * r2 - i1 * i2
}

slay ComplexMulImag(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    // Return imaginary part of multiplication
    damn r1 * i2 + i1 * r2
}

slay ComplexDiv(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    // Return real part of division
    sus denom meal = r2 * r2 + i2 * i2
    damn (r1 * r2 + i1 * i2) / denom
}

slay ComplexDivImag(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    // Return imaginary part of division
    sus denom meal = r2 * r2 + i2 * i2
    damn (i1 * r2 - r1 * i2) / denom
}

slay ComplexAbs(r meal, i meal) meal {
    // Compute absolute value (modulus) of complex number
    damn mathz.Sqrt(r * r + i * i)
}

slay ComplexConj(r meal, i meal) meal {
    // Return real part of conjugate (same as original)
    damn r
}

slay ComplexConjImag(r meal, i meal) meal {
    // Return imaginary part of conjugate (negated)
    damn 0.0 - i
}

// Basic trigonometric functions using Taylor series
slay Sin(x meal) meal {
    // Taylor series for sin(x) = x - x^3/3! + x^5/5! - x^7/7! + ...
    sus result meal = x
    sus term meal = x
    sus x_squared meal = x * x
    
    // Add a few terms for reasonable accuracy
    term = term * (0.0 - x_squared) / (2.0 * 3.0)
    result = result + term
    
    term = term * (0.0 - x_squared) / (4.0 * 5.0)
    result = result + term
    
    term = term * (0.0 - x_squared) / (6.0 * 7.0)
    result = result + term
    
    damn result
}

slay Cos(x meal) meal {
    // Taylor series for cos(x) = 1 - x^2/2! + x^4/4! - x^6/6! + ...
    sus result meal = 1.0
    sus term meal = 1.0
    sus x_squared meal = x * x
    
    // Add a few terms for reasonable accuracy
    term = term * (0.0 - x_squared) / (1.0 * 2.0)
    result = result + term
    
    term = term * (0.0 - x_squared) / (3.0 * 4.0)
    result = result + term
    
    term = term * (0.0 - x_squared) / (5.0 * 6.0)
    result = result + term
    
    damn result
}

slay Exp(x meal) meal {
    // Taylor series for exp(x) = 1 + x + x^2/2! + x^3/3! + ...
    sus result meal = 1.0
    sus term meal = 1.0
    
    // Add terms for reasonable accuracy
    term = term * x / 1.0
    result = result + term
    
    term = term * x / 2.0
    result = result + term
    
    term = term * x / 3.0
    result = result + term
    
    term = term * x / 4.0
    result = result + term
    
    term = term * x / 5.0
    result = result + term
    
    damn result
}

// Complex exponential functions
slay ComplexExp(r meal, i meal) meal {
    // Return real part of e^(r + i*i) = e^r * cos(i)
    sus exp_r meal = Exp(r)
    sus cos_i meal = Cos(i)
    damn exp_r * cos_i
}

slay ComplexExpImag(r meal, i meal) meal {
    // Return imaginary part of e^(r + i*i) = e^r * sin(i)
    sus exp_r meal = Exp(r)
    sus sin_i meal = Sin(i)
    damn exp_r * sin_i
}

slay ComplexSin(r meal, i meal) meal {
    // Return real part of sin(r + i*i)
    sus sin_r meal = Sin(r)
    sus exp_i meal = Exp(i)
    sus exp_neg_i meal = Exp(0.0 - i)
    sus cosh_i meal = (exp_i + exp_neg_i) / 2.0
    damn sin_r * cosh_i
}

slay ComplexSinImag(r meal, i meal) meal {
    // Return imaginary part of sin(r + i*i)
    sus cos_r meal = Cos(r)
    sus exp_i meal = Exp(i)
    sus exp_neg_i meal = Exp(0.0 - i)
    sus sinh_i meal = (exp_i - exp_neg_i) / 2.0
    damn cos_r * sinh_i
}

slay ComplexCos(r meal, i meal) meal {
    // Return real part of cos(r + i*i)
    sus cos_r meal = Cos(r)
    sus exp_i meal = Exp(i)
    sus exp_neg_i meal = Exp(0.0 - i)
    sus cosh_i meal = (exp_i + exp_neg_i) / 2.0
    damn cos_r * cosh_i
}

slay ComplexCosImag(r meal, i meal) meal {
    // Return imaginary part of cos(r + i*i)
    sus sin_r meal = Sin(r)
    sus exp_i meal = Exp(i)
    sus exp_neg_i meal = Exp(0.0 - i)
    sus sinh_i meal = (exp_i - exp_neg_i) / 2.0
    damn 0.0 - sin_r * sinh_i
}

// Utility functions
slay IsComplexZero(r meal, i meal) lit {
    // Check if complex number is zero
    damn mathz.Abs(r) < 1e-10 && mathz.Abs(i) < 1e-10
}

slay IsComplexReal(r meal, i meal) lit {
    // Check if complex number is real (imaginary part is zero)
    damn mathz.Abs(i) < 1e-10
}

slay IsComplexImag(r meal, i meal) lit {
    // Check if complex number is purely imaginary (real part is zero)
    damn mathz.Abs(r) < 1e-10
}

slay ComplexEqual(r1 meal, i1 meal, r2 meal, i2 meal) lit {
    // Check if two complex numbers are equal
    sus real_diff meal = r1 - r2
    sus imag_diff meal = i1 - i2
    damn mathz.Abs(real_diff) < 1e-10 && mathz.Abs(imag_diff) < 1e-10
}

// Phase calculation
slay ComplexPhase(r meal, i meal) meal {
    // Calculate phase using simplified approach
    bestie r == 0.0 && i == 0.0 {
        damn 0.0
    }
    bestie r == 0.0 {
        bestie i > 0.0 {
            damn 1.5708  // π/2
        }
        damn -1.5708  // -π/2
    }
    bestie i == 0.0 {
        bestie r > 0.0 {
            damn 0.0
        }
        damn 3.14159  // π
    }
    
    // Simplified arctangent using linear approximation
    sus ratio meal = i / r
    sus atan_approx meal = ratio / (1.0 + 0.28 * ratio * ratio)
    
    bestie r < 0.0 {
        bestie i >= 0.0 {
            damn atan_approx + 3.14159  // π
        }
        damn atan_approx - 3.14159  // -π
    }
    
    damn atan_approx
}

// Polar to rectangular conversion
slay PolarToRect(r meal, theta meal) meal {
    // Return real part: r * cos(theta)
    damn r * Cos(theta)
}

slay PolarToRectImag(r meal, theta meal) meal {
    // Return imaginary part: r * sin(theta)
    damn r * Sin(theta)
}
