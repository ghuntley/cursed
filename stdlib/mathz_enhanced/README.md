# mathz_enhanced - Enhanced Mathematical Operations Module

## Overview

The `mathz_enhanced` module provides advanced mathematical functions, algorithms, and specialized computational capabilities for CURSED programs. **Why enhanced math?** Because beyond basic arithmetic lies a world of statistical analysis, linear algebra, signal processing, and computational mathematics that modern applications demand. This module exists to bring scientific computing power to CURSED without sacrificing performance or precision.

**Design Philosophy**: IEEE 754 compliance, vectorized operations where possible, and numerically stable algorithms with automatic precision selection for optimal accuracy.

## Quick Start

```cursed
yeet "mathz_enhanced"

// Statistical analysis
sus dataset []float = [1.2, 3.4, 2.1, 5.7, 4.3, 6.8, 3.9, 2.5, 4.1, 5.2]
sus stats mathz_enhanced.Statistics = mathz_enhanced.analyze(dataset)

vibez.spill("Mean:", stats.mean)
vibez.spill("Median:", stats.median) 
vibez.spill("Std Dev:", stats.standard_deviation)
vibez.spill("95% Confidence Interval:", stats.confidence_interval(0.95))

// Linear algebra
sus matrix_a mathz_enhanced.Matrix = mathz_enhanced.Matrix.from_array([
    [2.0, 3.0],
    [1.0, 4.0]
])
sus matrix_b mathz_enhanced.Matrix = mathz_enhanced.Matrix.from_array([
    [5.0, 6.0],
    [7.0, 8.0]
])

sus product mathz_enhanced.Matrix = matrix_a.multiply(matrix_b)
sus determinant float = matrix_a.determinant()
sus inverse mathz_enhanced.Matrix = matrix_a.invert() fam {
    when "singular_matrix" -> {
        vibez.spill_error("Matrix is not invertible")
        damn mathz_enhanced.Matrix.identity(2)
    }
}

// Advanced mathematical functions
sus complex_result mathz_enhanced.Complex = mathz_enhanced.Complex.from_polar(5.0, mathz.pi / 4.0)
sus fourier_transform []mathz_enhanced.Complex = mathz_enhanced.fft(signal_data)
sus prime_factors []drip = mathz_enhanced.factor(1234567)

vibez.spill("Complex result:", complex_result.to_string())
vibez.spill("Prime factors of 1234567:", prime_factors.join(" × "))
```

## Why This Design?

### Numerical Stability First
**Problem**: Naive mathematical algorithms suffer from numerical instability, precision loss, and catastrophic cancellation that make results unreliable.

**Solution**: Implement numerically stable algorithms (Kahan summation, Givens rotations, iterative refinement) that maintain precision even with extreme inputs or ill-conditioned problems.

### Vectorized Operations
**Problem**: Element-wise operations on large arrays are slow when implemented with scalar code, especially on modern SIMD-capable processors.

**Solution**: Automatic vectorization using platform-specific SIMD instructions (AVX2, NEON) with transparent fallbacks to scalar implementations.

### Arbitrary Precision Support
**Problem**: IEEE 754 float/double precision is insufficient for cryptographic, financial, or scientific applications requiring exact arithmetic.

**Solution**: Seamless integration between fixed-precision (fast) and arbitrary-precision (exact) arithmetic based on context and requirements.

## API Reference

### Core Types

#### `mathz_enhanced.Complex`
Complex number type with comprehensive mathematical operations.

```cursed
squad mathz_enhanced.Complex {
    real float
    imaginary float
    
    slay from_cartesian(real float, imag float) mathz_enhanced.Complex
    slay from_polar(magnitude float, angle float) mathz_enhanced.Complex
    
    slay magnitude() float
    slay argument() float  // Phase angle
    slay conjugate() mathz_enhanced.Complex
    
    slay add(other mathz_enhanced.Complex) mathz_enhanced.Complex
    slay multiply(other mathz_enhanced.Complex) mathz_enhanced.Complex
    slay divide(other mathz_enhanced.Complex) yikes<mathz_enhanced.Complex>
    slay power(exponent mathz_enhanced.Complex) mathz_enhanced.Complex
    
    slay exp() mathz_enhanced.Complex
    slay log() yikes<mathz_enhanced.Complex>
    slay sqrt() mathz_enhanced.Complex
    slay sin() mathz_enhanced.Complex
    slay cos() mathz_enhanced.Complex
}
```

#### `mathz_enhanced.Matrix`
Dense matrix type with linear algebra operations.

```cursed
squad mathz_enhanced.Matrix {
    rows drip
    cols drip
    data []float  // Row-major storage
    
    slay from_array(data [][]float) mathz_enhanced.Matrix
    slay identity(size drip) mathz_enhanced.Matrix
    slay zeros(rows drip, cols drip) mathz_enhanced.Matrix
    slay random(rows drip, cols drip, distribution tea) mathz_enhanced.Matrix
    
    slay get(row drip, col drip) yikes<float>
    slay set(row drip, col drip, value float) yikes<>
    
    slay multiply(other mathz_enhanced.Matrix) yikes<mathz_enhanced.Matrix>
    slay transpose() mathz_enhanced.Matrix
    slay determinant() yikes<float>
    slay invert() yikes<mathz_enhanced.Matrix>
    
    slay eigenvalues() yikes<[]mathz_enhanced.Complex>
    slay svd() yikes<mathz_enhanced.SVDResult>
    slay qr_decomposition() yikes<mathz_enhanced.QRResult>
}
```

#### `mathz_enhanced.Vector`
Vector type optimized for numerical computations.

```cursed
squad mathz_enhanced.Vector {
    data []float
    
    slay from_array(values []float) mathz_enhanced.Vector
    slay zeros(size drip) mathz_enhanced.Vector
    slay linspace(start float, stop float, count drip) mathz_enhanced.Vector
    
    slay dot(other mathz_enhanced.Vector) yikes<float>
    slay cross(other mathz_enhanced.Vector) yikes<mathz_enhanced.Vector>  // 3D only
    slay norm() float
    slay normalize() mathz_enhanced.Vector
    
    slay add(other mathz_enhanced.Vector) yikes<mathz_enhanced.Vector>
    slay subtract(other mathz_enhanced.Vector) yikes<mathz_enhanced.Vector>
    slay scale(factor float) mathz_enhanced.Vector
    
    slay convolve(kernel mathz_enhanced.Vector) mathz_enhanced.Vector
    slay correlate(other mathz_enhanced.Vector) mathz_enhanced.Vector
}
```

### Statistical Functions

#### Descriptive Statistics
```cursed
slay compute_comprehensive_statistics(data []float) mathz_enhanced.Statistics {
    sus stats mathz_enhanced.Statistics = mathz_enhanced.analyze(data)
    
    // Central tendency
    vibez.spill("Mean:", stats.mean)
    vibez.spill("Median:", stats.median)
    vibez.spill("Mode:", stats.mode)
    vibez.spill("Geometric mean:", stats.geometric_mean())
    vibez.spill("Harmonic mean:", stats.harmonic_mean())
    
    // Variability
    vibez.spill("Variance:", stats.variance)
    vibez.spill("Standard deviation:", stats.standard_deviation)
    vibez.spill("Range:", stats.max - stats.min)
    vibez.spill("Interquartile range:", stats.q75 - stats.q25)
    vibez.spill("Mean absolute deviation:", stats.mean_absolute_deviation())
    
    // Shape
    vibez.spill("Skewness:", stats.skewness())
    vibez.spill("Kurtosis:", stats.kurtosis())
    
    // Confidence intervals
    vibez.spill("95% CI for mean:", stats.confidence_interval(0.95))
    vibez.spill("90% CI for median:", stats.median_confidence_interval(0.90))
    
    damn stats
}

// Hypothesis testing
slay perform_t_test(sample1 []float, sample2 []float) mathz_enhanced.TTestResult {
    sus result mathz_enhanced.TTestResult = mathz_enhanced.two_sample_t_test(
        sample1, sample2,
        mathz_enhanced.TTestOptions{
            equal_variances: false,  // Welch's t-test
            alpha: 0.05,
            alternative: "two_sided"
        }
    )
    
    vibez.spill("t-statistic:", result.statistic)
    vibez.spill("p-value:", result.p_value) 
    vibez.spill("Degrees of freedom:", result.degrees_of_freedom)
    vibez.spill("95% CI for difference:", result.confidence_interval)
    
    ready (result.p_value < 0.05) {
        vibez.spill("Result: Significant difference detected (p <", result.p_value, ")")
    } otherwise {
        vibez.spill("Result: No significant difference (p =", result.p_value, ")")
    }
    
    damn result
}
```

#### Probability Distributions
```cursed
// Normal distribution
sus normal mathz_enhanced.NormalDistribution = mathz_enhanced.NormalDistribution{
    mean: 0.0,
    std_dev: 1.0
}

sus probability float = normal.pdf(1.96)  // Probability density
sus cumulative float = normal.cdf(1.96)   // Cumulative probability  
sus quantile float = normal.quantile(0.975)  // Inverse CDF

// Other distributions
sus exponential mathz_enhanced.ExponentialDistribution = mathz_enhanced.ExponentialDistribution{rate: 0.5}
sus chi_square mathz_enhanced.ChiSquareDistribution = mathz_enhanced.ChiSquareDistribution{df: 5}
sus beta mathz_enhanced.BetaDistribution = mathz_enhanced.BetaDistribution{alpha: 2.0, beta: 5.0}

// Distribution fitting
sus data []float = load_experimental_data()
sus fitted_dist mathz_enhanced.Distribution = mathz_enhanced.fit_distribution(data, [
    "normal", "exponential", "gamma", "weibull"
]) fam {
    when "no_convergence" -> {
        vibez.spill_error("Could not fit any distribution to data")
        damn mathz_enhanced.UniformDistribution{min: data.min(), max: data.max()}
    }
}

vibez.spill("Best fit:", fitted_dist.name(), "AIC:", fitted_dist.aic())
```

### Linear Algebra Operations

#### Matrix Decompositions
**Why matrix decompositions?** They're the foundation of numerical linear algebra, enabling efficient solutions to systems of equations, eigenvalue problems, and least-squares fitting.

```cursed
// LU decomposition for solving linear systems
slay solve_linear_system(A mathz_enhanced.Matrix, b mathz_enhanced.Vector) yikes<mathz_enhanced.Vector> {
    sus lu_result mathz_enhanced.LUResult = A.lu_decompose() fam {
        when "singular_matrix" -> yikes "system_has_no_unique_solution"
        when _ -> yikes error
    }
    
    // Solve Ly = Pb (forward substitution)
    sus y mathz_enhanced.Vector = lu_result.solve_forward(lu_result.P.multiply_vector(b))
    
    // Solve Ux = y (backward substitution)
    sus x mathz_enhanced.Vector = lu_result.solve_backward(y)
    
    // Iterative refinement for improved accuracy
    sus residual mathz_enhanced.Vector = A.multiply_vector(x).subtract(b)
    ready (residual.norm() > 1e-12) {
        sus correction mathz_enhanced.Vector = lu_result.solve(residual)
        x = x.subtract(correction)
    }
    
    damn x
}

// Singular Value Decomposition (SVD)
slay analyze_data_matrix(data mathz_enhanced.Matrix) mathz_enhanced.DataAnalysis {
    sus svd mathz_enhanced.SVDResult = data.svd() fam {
        when "convergence_failed" -> {
            vibez.spill_error("SVD did not converge, using eigenvalue decomposition")
            damn analyze_with_eigendecomposition(data)
        }
    }
    
    // Principal components analysis
    sus explained_variance []float = []float{}
    sus total_variance float = svd.singular_values.map(slay(s float) float { damn s * s }).sum()
    
    bestie (sus i drip = 0; i < svd.singular_values.length; i++) {
        sus variance float = (svd.singular_values[i] * svd.singular_values[i]) / total_variance
        explained_variance.push(variance)
        vibez.spill("PC" + (i+1).(tea) + " explains", (variance * 100), "% of variance")
    }
    
    // Dimensionality reduction (keep 95% of variance)
    sus cumulative_variance float = 0.0
    sus components_needed drip = 0
    bestie (sus variance float : explained_variance) {
        cumulative_variance += variance
        components_needed++
        ready (cumulative_variance >= 0.95) break
    }
    
    vibez.spill("Need", components_needed, "components to explain 95% of variance")
    
    // Reduced representation
    sus reduced_data mathz_enhanced.Matrix = data.multiply(svd.V.submatrix(0, 0, svd.V.rows, components_needed))
    
    damn mathz_enhanced.DataAnalysis{
        original: data,
        reduced: reduced_data,
        explained_variance: explained_variance,
        components_needed: components_needed
    }
}

// Eigenvalue problems
slay find_dominant_eigenvalues(matrix mathz_enhanced.Matrix, count drip) []mathz_enhanced.Complex {
    // Power iteration for largest eigenvalue
    sus eigenvalues []mathz_enhanced.Complex = []mathz_enhanced.Complex{}
    sus working_matrix mathz_enhanced.Matrix = matrix.copy()
    
    bestie (sus i drip = 0; i < count; i++) {
        sus eigenval mathz_enhanced.Complex = working_matrix.power_iteration() fam {
            when "no_convergence" -> {
                vibez.spill_error("Power iteration failed for eigenvalue", i)
                continue
            }
        }
        
        eigenvalues.push(eigenval)
        
        // Deflation: remove found eigenvalue
        working_matrix = working_matrix.deflate_eigenvalue(eigenval)
    }
    
    damn eigenvalues
}
```

### Signal Processing

#### Fourier Transforms
**Why FFT?** Fast Fourier Transform is fundamental to signal processing, enabling frequency domain analysis, filtering, and convolution operations.

```cursed
// Fast Fourier Transform
slay analyze_signal_spectrum(signal []float, sampling_rate float) mathz_enhanced.SpectrumAnalysis {
    // Convert to complex for FFT
    sus complex_signal []mathz_enhanced.Complex = signal.map(slay(x float) mathz_enhanced.Complex {
        damn mathz_enhanced.Complex.from_cartesian(x, 0.0)
    })
    
    // Apply window function to reduce spectral leakage
    sus windowed []mathz_enhanced.Complex = mathz_enhanced.apply_hamming_window(complex_signal)
    
    // Compute FFT
    sus spectrum []mathz_enhanced.Complex = mathz_enhanced.fft(windowed) fam {
        when "invalid_size" -> {
            vibez.spill_error("Signal length must be power of 2 for FFT")
            // Zero-pad to next power of 2
            sus padded_size drip = mathz_enhanced.next_power_of_2(windowed.length)
            sus padded []mathz_enhanced.Complex = mathz_enhanced.zero_pad(windowed, padded_size)
            damn mathz_enhanced.fft(padded)
        }
    }
    
    // Compute magnitude and phase
    sus magnitudes []float = spectrum.map(slay(c mathz_enhanced.Complex) float { damn c.magnitude() })
    sus phases []float = spectrum.map(slay(c mathz_enhanced.Complex) float { damn c.argument() })
    
    // Frequency bins
    sus frequencies []float = mathz_enhanced.fft_frequencies(spectrum.length, sampling_rate)
    
    // Find dominant frequencies
    sus peaks []drip = mathz_enhanced.find_peaks(magnitudes, mathz_enhanced.PeakOptions{
        min_height: magnitudes.max() * 0.1,  // 10% of max magnitude
        min_distance: 5  // Minimum distance between peaks
    })
    
    vibez.spill("Found", peaks.length, "dominant frequencies:")
    bestie (sus peak_idx drip : peaks) {
        vibez.spill("  Frequency:", frequencies[peak_idx], "Hz, Magnitude:", magnitudes[peak_idx])
    }
    
    damn mathz_enhanced.SpectrumAnalysis{
        frequencies: frequencies,
        magnitudes: magnitudes,
        phases: phases,
        dominant_peaks: peaks
    }
}

// Digital filtering
slay apply_butterworth_filter(signal []float, cutoff_freq float, sampling_rate float, order drip) []float {
    // Design Butterworth low-pass filter
    sus filter mathz_enhanced.ButterworthFilter = mathz_enhanced.butterworth_lowpass(
        order, cutoff_freq, sampling_rate
    )
    
    // Apply filter (forward and backward to eliminate phase distortion)
    sus filtered []float = mathz_enhanced.filtfilt(filter, signal)
    
    damn filtered
}

// Convolution and correlation
slay detect_pattern_in_signal(signal []float, pattern []float) []drip {
    // Cross-correlation to find pattern occurrences
    sus correlation []float = mathz_enhanced.correlate(signal, pattern)
    
    // Find peaks in correlation (pattern matches)
    sus matches []drip = mathz_enhanced.find_peaks(correlation, mathz_enhanced.PeakOptions{
        min_height: correlation.max() * 0.8,  // 80% of maximum correlation
        min_distance: pattern.length  // Patterns can't overlap
    })
    
    vibez.spill("Pattern found at positions:", matches.join(", "))
    
    damn matches
}
```

### Numerical Integration and Differentiation

#### Adaptive Integration
**Why adaptive integration?** Fixed-step integration methods waste computation on smooth regions and miss features in complex regions.

```cursed
// Adaptive quadrature with error control
slay integrate_function(f slay(float) float, a float, b float, tolerance float) yikes<float> {
    sus result mathz_enhanced.IntegrationResult = mathz_enhanced.adaptive_quadrature(
        f, a, b,
        mathz_enhanced.IntegrationOptions{
            absolute_tolerance: tolerance,
            relative_tolerance: tolerance,
            max_intervals: 10000
        }
    ) fam {
        when "no_convergence" -> {
            vibez.spill_error("Integration did not converge within tolerance")
            yikes "integration_failed"
        }
        when "singularity_detected" -> {
            vibez.spill_error("Function appears to have singularity in interval")
            yikes "singular_integrand"
        }
    }
    
    vibez.spill("Integration result:", result.value)
    vibez.spill("Estimated error:", result.error_estimate)
    vibez.spill("Function evaluations:", result.function_calls)
    
    damn result.value
}

// Monte Carlo integration for high-dimensional integrals
slay monte_carlo_integration(f slay([]float) float, bounds []mathz_enhanced.Interval, samples drip) float {
    sus total float = 0.0
    sus volume float = bounds.map(slay(interval mathz_enhanced.Interval) float {
        damn interval.width()
    }).product()
    
    bestie (sus i drip = 0; i < samples; i++) {
        sus point []float = []float{}
        bestie (sus bound mathz_enhanced.Interval : bounds) {
            point.push(mathz_enhanced.random_uniform(bound.min, bound.max))
        }
        
        total += f(point)
    }
    
    sus result float = volume * total / (samples as float)
    vibez.spill("Monte Carlo result:", result, "from", samples, "samples")
    
    damn result
}

// Numerical differentiation with automatic step size selection
slay numerical_derivative(f slay(float) float, x float, order drip) yikes<float> {
    sick (order) {
        when 1 -> {
            // Central difference with Richardson extrapolation
            damn mathz_enhanced.central_difference_richardson(f, x, 5) fam {
                when "step_size_too_small" -> yikes "numerical_instability"
            }
        }
        when 2 -> {
            // Second derivative using central difference
            damn mathz_enhanced.second_derivative_central(f, x)
        }
        when _ -> {
            yikes "unsupported_derivative_order"
        }
    }
}
```

## Advanced Features

### Arbitrary Precision Arithmetic

**Why arbitrary precision?** Financial calculations, cryptographic operations, and some scientific computations require exact arithmetic without rounding errors.

```cursed
// High-precision calculations
sus pi_precise mathz_enhanced.BigFloat = mathz_enhanced.compute_pi(1000)  // 1000 decimal places
vibez.spill("π to 50 digits:", pi_precise.to_string(50))

// Exact rational arithmetic
sus fraction1 mathz_enhanced.Rational = mathz_enhanced.Rational.from_fraction(22, 7)
sus fraction2 mathz_enhanced.Rational = mathz_enhanced.Rational.from_fraction(355, 113)

sus pi_approx1 float = fraction1.to_float()
sus pi_approx2 float = fraction2.to_float()

vibez.spill("22/7 ≈", pi_approx1, "error:", mathz.abs(pi_approx1 - mathz.pi))
vibez.spill("355/113 ≈", pi_approx2, "error:", mathz.abs(pi_approx2 - mathz.pi))

// Financial calculations with exact decimal arithmetic
sus price mathz_enhanced.Decimal = mathz_enhanced.Decimal.from_string("19.99")
sus tax_rate mathz_enhanced.Decimal = mathz_enhanced.Decimal.from_string("0.08")
sus quantity mathz_enhanced.Decimal = mathz_enhanced.Decimal.from_string("3")

sus subtotal mathz_enhanced.Decimal = price.multiply(quantity)
sus tax mathz_enhanced.Decimal = subtotal.multiply(tax_rate)
sus total mathz_enhanced.Decimal = subtotal.add(tax)

vibez.spill("Subtotal: $", subtotal.to_string(2))
vibez.spill("Tax: $", tax.to_string(2))
vibez.spill("Total: $", total.to_string(2))
```

### Optimization and Root Finding

**Why optimization?** Many real-world problems reduce to finding maxima, minima, or zeros of functions.

```cursed
// Nonlinear optimization
slay optimize_portfolio(returns []float, risks [][]float, target_return float) yikes<[]float> {
    // Minimize portfolio risk subject to target return constraint
    sus objective slay(weights []float) float = {
        // Calculate portfolio risk (w^T * Σ * w)
        sus risk float = 0.0
        bestie (sus i drip = 0; i < weights.length; i++) {
            bestie (sus j drip = 0; j < weights.length; j++) {
                risk += weights[i] * risks[i][j] * weights[j]
            }
        }
        damn risk
    }
    
    sus constraints []mathz_enhanced.Constraint = [
        // Weights sum to 1
        mathz_enhanced.Constraint{
            function: slay(w []float) float { damn w.sum() - 1.0 },
            type: "equality"
        },
        // Target return constraint
        mathz_enhanced.Constraint{
            function: slay(w []float) float {
                sus portfolio_return float = 0.0
                bestie (sus i drip = 0; i < w.length; i++) {
                    portfolio_return += w[i] * returns[i]
                }
                damn portfolio_return - target_return
            },
            type: "equality"
        }
    ]
    
    sus initial_weights []float = mathz_enhanced.Vector.ones(returns.length).scale(1.0 / returns.length).data
    
    sus result mathz_enhanced.OptimizationResult = mathz_enhanced.minimize(
        objective, initial_weights, constraints,
        mathz_enhanced.OptimizationOptions{
            method: "sequential_quadratic_programming",
            tolerance: 1e-8,
            max_iterations: 1000
        }
    ) fam {
        when "no_convergence" -> yikes "optimization_failed"
        when "infeasible_constraints" -> yikes "no_feasible_solution"
    }
    
    vibez.spill("Optimal portfolio risk:", result.objective_value)
    vibez.spill("Portfolio weights:", result.x.map(slay(w float) tea { damn (w * 100).(tea) + "%" }).join(", "))
    
    damn result.x
}

// Root finding with multiple methods
slay find_equation_roots(f slay(float) float, interval mathz_enhanced.Interval) []float {
    sus roots []float = []float{}
    
    // Try bracketing methods first (robust)
    sus bracket_roots []float = mathz_enhanced.find_roots_bracketing(f, interval, 100) fam {
        when _ -> []float{}
    }
    roots.extend(bracket_roots)
    
    // Try Newton's method for additional roots
    sus newton_roots []float = mathz_enhanced.find_roots_newton(f, interval, 50) fam {
        when _ -> []float{}
    }
    
    // Remove duplicates
    sus all_roots []float = roots.extend(newton_roots).unique(1e-10)
    
    vibez.spill("Found", all_roots.length, "roots:")
    bestie (sus root float : all_roots) {
        vibez.spill("  x =", root, ", f(x) =", f(root))
    }
    
    damn all_roots
}
```

## Performance Characteristics

### SIMD Vectorization
- **Vector Operations**: 4-8x speedup on AVX2/AVX-512 processors
- **Matrix Multiplication**: 10-20x speedup with BLAS integration
- **FFT**: Optimized radix-4 implementation with vector instructions
- **Automatic Fallback**: Scalar implementations for unsupported architectures

### Memory Optimization
```cursed
// Memory-efficient operations for large datasets
slay process_large_dataset(data_file tea) {
    // Stream processing to avoid loading entire dataset
    sus data_stream mathz_enhanced.DataStream = mathz_enhanced.open_numeric_stream(data_file) fam {
        when _ -> handle_file_error()
    }
    
    // Compute statistics incrementally
    sus running_stats mathz_enhanced.RunningStatistics = mathz_enhanced.create_running_stats()
    sus chunk_size drip = 100_000  // Process 100k values at a time
    
    bestie (sus chunk []float : data_stream.chunks(chunk_size)) {
        running_stats.update(chunk)  // Update statistics without storing all data
        
        ready (running_stats.count % 1_000_000 == 0) {
            vibez.spill("Processed", running_stats.count, "values, mean =", running_stats.mean)
        }
    }
    
    vibez.spill("Final statistics:")
    vibez.spill("Count:", running_stats.count)
    vibez.spill("Mean:", running_stats.mean)
    vibez.spill("Std Dev:", running_stats.standard_deviation())
}

// Memory pool for frequent allocations
sus math_pool mathz_enhanced.MathPool = mathz_enhanced.create_math_pool(100_000_000)  // 100MB

slay matrix_multiply_optimized(A mathz_enhanced.Matrix, B mathz_enhanced.Matrix) mathz_enhanced.Matrix {
    // Use memory pool for temporary allocations
    sus result mathz_enhanced.Matrix = math_pool.create_matrix(A.rows, B.cols)
    
    // Cache-friendly blocked matrix multiplication
    mathz_enhanced.gemm_blocked(A, B, result, math_pool)
    
    damn result
} // Memory automatically returned to pool
```

## Error Handling Patterns

### Numerical Error Detection
```cursed
slay robust_matrix_operations(matrix mathz_enhanced.Matrix) yikes<mathz_enhanced.ProcessingResult> {
    // Check condition number to detect numerical problems
    sus condition_number float = matrix.condition_number() fam {
        when "singular_matrix" -> {
            yikes "matrix_is_singular"
        }
    }
    
    ready (condition_number > 1e12) {
        vibez.spill_error("Matrix is ill-conditioned (condition number:", condition_number, ")")
        vibez.spill_error("Results may be numerically unstable")
        yikes "ill_conditioned_matrix"
    }
    
    // Attempt matrix inversion with iterative refinement
    sus inverse mathz_enhanced.Matrix = matrix.invert_with_refinement(3) fam {
        when "no_convergence" -> {
            vibez.spill_error("Iterative refinement did not converge")
            yikes "inversion_failed"
        }
        when "numerical_instability" -> {
            vibez.spill_error("Numerical instability detected during inversion")
            yikes "unstable_computation"
        }
    }
    
    // Verify inversion accuracy
    sus identity_check mathz_enhanced.Matrix = matrix.multiply(inverse)
    sus identity_expected mathz_enhanced.Matrix = mathz_enhanced.Matrix.identity(matrix.rows)
    sus inversion_error float = identity_check.subtract(identity_expected).frobenius_norm()
    
    ready (inversion_error > 1e-10) {
        vibez.spill_error("Large inversion error:", inversion_error)
        vibez.spill_error("Consider using regularization or different algorithm")
        yikes "inaccurate_inversion"
    }
    
    damn mathz_enhanced.ProcessingResult{
        matrix: matrix,
        inverse: inverse,
        condition_number: condition_number,
        inversion_error: inversion_error
    }
}
```

## Testing Strategy

### Numerical Accuracy Tests
```cursed
// stdlib/mathz_enhanced/test_mathz_enhanced.csd
yeet "testz"
yeet "mathz_enhanced"

slay test_complex_arithmetic() {
    sus z1 mathz_enhanced.Complex = mathz_enhanced.Complex.from_cartesian(3.0, 4.0)
    sus z2 mathz_enhanced.Complex = mathz_enhanced.Complex.from_cartesian(1.0, 2.0)
    
    sus sum mathz_enhanced.Complex = z1.add(z2)
    testz.assert_float_eq(sum.real, 4.0, 1e-15)
    testz.assert_float_eq(sum.imaginary, 6.0, 1e-15)
    
    sus product mathz_enhanced.Complex = z1.multiply(z2)
    testz.assert_float_eq(product.real, -5.0, 1e-15)  // (3+4i)(1+2i) = -5+10i
    testz.assert_float_eq(product.imaginary, 10.0, 1e-15)
    
    sus magnitude float = z1.magnitude()
    testz.assert_float_eq(magnitude, 5.0, 1e-15)  // sqrt(3²+4²) = 5
}

slay test_matrix_operations() {
    sus A mathz_enhanced.Matrix = mathz_enhanced.Matrix.from_array([
        [2.0, 3.0],
        [1.0, 4.0]
    ])
    
    sus det float = A.determinant() fam {
        when _ -> testz.fail("Determinant calculation should succeed")
    }
    testz.assert_float_eq(det, 5.0, 1e-15)  // 2*4 - 3*1 = 5
    
    sus inv mathz_enhanced.Matrix = A.invert() fam {
        when _ -> testz.fail("Matrix inversion should succeed")
    }
    
    // Verify A * A^(-1) = I
    sus identity mathz_enhanced.Matrix = A.multiply(inv) fam {
        when _ -> testz.fail("Matrix multiplication should succeed")
    }
    
    testz.assert_float_eq(identity.get(0, 0), 1.0, 1e-12)
    testz.assert_float_eq(identity.get(0, 1), 0.0, 1e-12)
    testz.assert_float_eq(identity.get(1, 0), 0.0, 1e-12)
    testz.assert_float_eq(identity.get(1, 1), 1.0, 1e-12)
}

slay test_numerical_stability() {
    // Test Kahan summation vs naive summation
    sus large_number float = 1e15
    sus small_numbers []float = []float{}
    bestie (sus i drip = 0; i < 1000; i++) {
        small_numbers.push(1e-5)
    }
    
    sus values []float = [large_number].extend(small_numbers)
    values.push(-large_number)
    
    // Naive summation loses precision
    sus naive_sum float = values.sum_naive()
    
    // Kahan summation maintains precision
    sus kahan_sum float = mathz_enhanced.kahan_sum(values)
    
    sus expected float = 1000 * 1e-5  // Should be 0.01
    
    // Kahan sum should be much more accurate
    testz.assert_true(mathz.abs(kahan_sum - expected) < 1e-10)
    testz.assert_true(mathz.abs(naive_sum - expected) > 1e-5)  // Naive sum has large error
}

slay test_statistical_functions() {
    sus data []float = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
    sus stats mathz_enhanced.Statistics = mathz_enhanced.analyze(data)
    
    testz.assert_float_eq(stats.mean, 5.5, 1e-15)
    testz.assert_float_eq(stats.median, 5.5, 1e-15)
    testz.assert_float_eq(stats.variance, 9.166666666666666, 1e-12)
    
    // Test confidence intervals
    sus ci mathz_enhanced.ConfidenceInterval = stats.confidence_interval(0.95)
    testz.assert_true(ci.lower < stats.mean)
    testz.assert_true(ci.upper > stats.mean)
}

slay main() {
    testz.start_suite("mathz_enhanced Tests")
    test_complex_arithmetic()
    test_matrix_operations()
    test_numerical_stability()
    test_statistical_functions()
    testz.print_summary()
}
```

## Implementation Choices Explained

### Why Numerically Stable Algorithms?
**Problem**: Naive implementations of mathematical algorithms often fail with real-world data due to rounding errors, catastrophic cancellation, and ill-conditioning.

**Solution**: Implement proven stable algorithms (Householder reflections, Givens rotations, iterative refinement) that maintain accuracy even in difficult cases.

### Why SIMD Vectorization?
**Problem**: Mathematical operations on arrays are embarrassingly parallel but scalar code can't utilize modern CPU vector units.

**Solution**: Automatic detection of vector instruction support with hand-optimized kernels for common operations, providing significant speedups without code complexity.

### Why Multiple Precision Support?
**Problem**: Different applications have different precision requirements - some need speed, others need exact results.

**Solution**: Seamless integration between IEEE 754 floating-point (fast), arbitrary precision floating-point (accurate), and rational arithmetic (exact).

## Migration Guide

### From NumPy (Python)
```python
# Python/NumPy
import numpy as np
data = np.array([1, 2, 3, 4, 5])
mean = np.mean(data)
std = np.std(data)

# CURSED
yeet "mathz_enhanced"
sus data []float = [1.0, 2.0, 3.0, 4.0, 5.0]
sus stats mathz_enhanced.Statistics = mathz_enhanced.analyze(data)
vibez.spill("Mean:", stats.mean, "Std:", stats.standard_deviation)
```

### From GSL (C)
```c
// C/GSL
#include <gsl/gsl_matrix.h>
gsl_matrix *m = gsl_matrix_alloc(2, 2);
gsl_matrix_set(m, 0, 0, 1.0);
double det = gsl_linalg_LU_det(m, perm);

// CURSED
sus matrix mathz_enhanced.Matrix = mathz_enhanced.Matrix.from_array([[1.0, 2.0], [3.0, 4.0]])
sus det float = matrix.determinant() fam { when _ -> handle_error() }
```

## Future Enhancements

### Planned Features
- **GPU Acceleration**: CUDA/OpenCL kernels for matrix operations
- **Distributed Computing**: MPI integration for large-scale computations  
- **Automatic Differentiation**: Forward and reverse mode AD
- **Symbolic Mathematics**: Computer algebra system integration

### Performance Improvements
- **JIT Compilation**: Runtime specialization of mathematical kernels
- **Memory-Mapped Large Matrices**: Handle matrices larger than RAM
- **Quantum Computing**: Basic quantum gate operations and simulation
- **Machine Learning Primitives**: Optimized neural network operations

---

The `mathz_enhanced` module brings scientific computing capabilities to CURSED with emphasis on numerical accuracy, performance optimization, and comprehensive mathematical functionality. Its design supports everything from simple statistical analysis to complex linear algebra operations while maintaining the safety and expressiveness of the CURSED language.
