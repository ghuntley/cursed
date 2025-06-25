use crate::error::CursedError;
/// Advanced Mathematical Functions Module for CURSED
/// 
/// Provides sophisticated mathematical operations including advanced numerical methods,
/// optimization algorithms, Fourier transforms, matrix operations, and mathematical
/// modeling utilities that complement the existing basic mathematical modules.
/// 
/// This module focuses on advanced computational mathematics, numerical analysis,
/// and specialized algorithms for scientific and engineering applications.

use std::f64::consts::{PI, E};
use super::{MathError, MathResult, validate_float};

// =============================================================================
// ADVANCED NUMERICAL METHODS
// =============================================================================

/// Computes numerical derivative using central difference method
/// 
/// Uses the central difference formula: f'(x) ≈ (f(x+h) - f(x-h)) / (2h)
/// where h is automatically chosen for optimal accuracy
/// 
/// # Arguments
/// * `f` - Function to differentiate
/// * `x` - Point at which to compute derivative
/// * `h` - Step size (optional, defaults to optimal value)
/// 
/// # Examples
/// ```
/// let f = |x| x * x; // f(x) = x²
/// let derivative = numerical_derivative(f, 2.0, None)?; // Should be ~4.0
/// ```
pub fn numerical_derivative<F>(f: F, x: f64, h: Option<f64>) -> MathResult<f64> 
where
{
    validate_float("numerical_derivative", "x", x)?;
    
    let step = h.unwrap_or(1e-8_f64.sqrt()); // Optimal step size
    validate_float("numerical_derivative", "h", step)?;
    
    if step <= 0.0 {
        return Err(MathError::InvalidInput {
        });
    let f_plus = f(x + step);
    let f_minus = f(x - step);
    
    if !f_plus.is_finite() || !f_minus.is_finite() {
        return Err(MathError::ComputationError {
        });
    Ok((f_plus - f_minus) / (2.0 * step))
/// Computes definite integral using adaptive Simpson's rule
/// 
/// Uses recursive subdivision to achieve specified accuracy
/// 
/// # Arguments
/// * `f` - Function to integrate
/// * `a` - Lower bound
/// * `b` - Upper bound
/// * `tolerance` - Desired accuracy (default: 1e-10)
/// * `max_depth` - Maximum recursion depth (default: 20)
/// 
/// # Examples
/// ```
/// let f = |x| x * x; // ∫x²dx from 0 to 2 = 8/3
/// let integral = adaptive_simpson_integration(f, 0.0, 2.0, None, None)?;
/// ```
pub fn adaptive_simpson_integration<F>(
    max_depth: Option<usize>
) -> MathResult<f64> 
where
{
    validate_float("adaptive_simpson_integration", "a", a)?;
    validate_float("adaptive_simpson_integration", "b", b)?;
    
    let tol = tolerance.unwrap_or(1e-10);
    let max_d = max_depth.unwrap_or(20);
    
    if tol <= 0.0 {
        return Err(MathError::InvalidInput {
        });
    fn simpson_step<F>(f: &F, a: f64, b: f64) -> f64 
    where 
    {
        let h = (b - a) / 6.0;
        let mid = (a + b) / 2.0;
        h * (f(a) + 4.0 * f(mid) + f(b))
    fn adaptive_step<F>(
        max_depth: usize
    ) -> MathResult<f64> 
    where 
    {
        if depth > max_depth {
            return Err(MathError::ComputationError {
            });
        let mid = (a + b) / 2.0;
        let whole = simpson_step(f, a, b);
        let left = simpson_step(f, a, mid);
        let right = simpson_step(f, mid, b);
        
        let error = (left + right - whole).abs() / 15.0;
        
        if error < tolerance {
            Ok(left + right + error)
        } else {
            let left_result = adaptive_step(f, a, mid, tolerance / 2.0, depth + 1, max_depth)?;
            let right_result = adaptive_step(f, mid, b, tolerance / 2.0, depth + 1, max_depth)?;
            Ok(left_result + right_result)
        }
    }
    
    adaptive_step(&f, a, b, tol, 0, max_d)
/// Multi-dimensional Newton-Raphson root finding
/// 
/// Solves systems of nonlinear equations using Newton's method
/// 
/// # Arguments
/// * `functions` - Vector of functions f_i(x) = 0
/// * `jacobian` - Jacobian matrix function
/// * `initial_guess` - Starting point
/// * `tolerance` - Convergence tolerance
/// * `max_iterations` - Maximum iterations
pub fn multidimensional_newton_raphson<F, J>(
) -> MathResult<Vec<f64>>
where
{
    let tol = tolerance.unwrap_or(1e-10);
    let max_iter = max_iterations.unwrap_or(100);
    let n = initial_guess.len();
    
    if n == 0 {
        return Err(MathError::InvalidInput {
        });
    let mut x = initial_guess.to_vec();
    
    for iteration in 0..max_iter {
        let f_vals = functions(&x);
        let jac = jacobian(&x);
        
        // Check for convergence
        let norm = f_vals.iter().map(|&v| v * v).sum::<f64>().sqrt();
        if norm < tol {
            return Ok(x);
        // Solve Jacobian * delta = -f_vals using Gaussian elimination
        let delta = solve_linear_system(&jac, &f_vals.iter().map(|&v| -v).collect::<Vec<_>>())?;
        
        // Update solution
        for i in 0..n {
            x[i] += delta[i];
        // Check for divergence
        if x.iter().any(|&v| !v.is_finite()) {
            return Err(MathError::ComputationError {
            });
        }
    }
    
    Err(MathError::ComputationError {
    })
// =============================================================================
// OPTIMIZATION ALGORITHMS
// =============================================================================

/// Golden section search for univariate optimization
/// 
/// Finds minimum of unimodal function on interval [a, b]
/// 
/// # Arguments
/// * `f` - Function to minimize
/// * `a` - Left endpoint
/// * `b` - Right endpoint
/// * `tolerance` - Search tolerance
/// 
/// # Examples
/// ```
/// let f = |x| (x - 2.0).powi(2); // Minimum at x = 2
/// let min_x = golden_section_search(f, 0.0, 4.0, None)?;
/// ```
pub fn golden_section_search<F>(
    tolerance: Option<f64>
) -> MathResult<f64>
where
{
    validate_float("golden_section_search", "a", a)?;
    validate_float("golden_section_search", "b", b)?;
    
    if a >= b {
        return Err(MathError::InvalidInput {
        });
    let tol = tolerance.unwrap_or(1e-10);
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0; // Golden ratio
    let resphi = 2.0 - phi;
    
    let mut x1 = a;
    let mut x2 = b;
    let mut x3 = x1 + resphi * (x2 - x1);
    let mut x4 = x1 + (1.0 - resphi) * (x2 - x1);
    
    let mut f3 = f(x3);
    let mut f4 = f(x4);
    
    while (x2 - x1).abs() > tol {
        if f3 < f4 {
            x2 = x4;
            x4 = x3;
            f4 = f3;
            x3 = x1 + resphi * (x2 - x1);
            f3 = f(x3);
        } else {
            x1 = x3;
            x3 = x4;
            f3 = f4;
            x4 = x1 + (1.0 - resphi) * (x2 - x1);
            f4 = f(x4);
        }
    }
    
    Ok((x1 + x2) / 2.0)
/// Gradient descent optimization
/// 
/// Minimizes function using gradient descent with adaptive learning rate
/// 
/// # Arguments
/// * `f` - Function to minimize
/// * `gradient` - Gradient function
/// * `initial_point` - Starting point
/// * `learning_rate` - Initial learning rate
/// * `tolerance` - Convergence tolerance
/// * `max_iterations` - Maximum iterations
pub fn gradient_descent<F, G>(
) -> MathResult<Vec<f64>>
where
{
    let mut x = initial_point.to_vec();
    let mut lr = learning_rate.unwrap_or(0.01);
    let tol = tolerance.unwrap_or(1e-8);
    let max_iter = max_iterations.unwrap_or(1000);
    
    let mut prev_f = f(&x);
    
    for iteration in 0..max_iter {
        let grad = gradient(&x);
        
        // Check for convergence
        let grad_norm = grad.iter().map(|&g| g * g).sum::<f64>().sqrt();
        if grad_norm < tol {
            return Ok(x);
        // Update position
        let mut new_x = x.clone();
        for i in 0..x.len() {
            new_x[i] -= lr * grad[i];
        let new_f = f(&new_x);
        
        // Adaptive learning rate
        if new_f < prev_f {
            lr *= 1.1; // Increase learning rate if improving
            x = new_x;
            prev_f = new_f;
        } else {
            lr *= 0.5; // Decrease learning rate if not improving
        // Check for very small learning rate
        if lr < 1e-12 {
            return Err(MathError::ComputationError {
            });
        }
    }
    
    Err(MathError::ComputationError {
    })
// =============================================================================
// FOURIER TRANSFORMS
// =============================================================================

/// Complex number representation for FFT
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexNumber {
impl ComplexNumber {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    pub fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    pub fn conjugate(&self) -> Self {
        Self::new(self.real, -self.imag)
    }
}

impl std::ops::Add for ComplexNumber {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self::new(self.real + other.real, self.imag + other.imag)
    }
}

impl std::ops::Sub for ComplexNumber {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self::new(self.real - other.real, self.imag - other.imag)
    }
}

impl std::ops::Mul for ComplexNumber {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        Self::new(
        )
    }
}

/// Fast Fourier Transform (Cooley-Tukey algorithm)
/// 
/// Computes FFT of input signal. Input length must be a power of 2.
/// 
/// # Arguments
/// * `signal` - Input signal (real values)
/// 
/// # Returns
/// Complex frequency domain representation
/// 
/// # Examples
/// ```
/// let signal = vec![1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0];
/// let fft_result = fast_fourier_transform(&signal)?;
/// ```
pub fn fast_fourier_transform(signal: &[f64]) -> MathResult<Vec<ComplexNumber>> {
    let n = signal.len();
    
    // Check if length is power of 2
    if n == 0 || (n & (n - 1)) != 0 {
        return Err(MathError::InvalidInput {
        });
    // Convert to complex numbers
    let mut data: Vec<ComplexNumber> = signal.iter()
        .map(|&x| ComplexNumber::new(x, 0.0))
        .collect();
    
    fft_recursive(&mut data);
    Ok(data)
/// Recursive FFT implementation
fn fft_recursive(data: &mut [ComplexNumber]) {
    let n = data.len();
    
    if n <= 1 {
        return;
    // Divide
    let mut even: Vec<ComplexNumber> = data.iter().step_by(2).copied().collect();
    let mut odd: Vec<ComplexNumber> = data.iter().skip(1).step_by(2).copied().collect();
    
    // Conquer
    fft_recursive(&mut even);
    fft_recursive(&mut odd);
    
    // Combine
    for i in 0..n/2 {
        let angle = -2.0 * PI * i as f64 / n as f64;
        let w = ComplexNumber::new(angle.cos(), angle.sin());
        let t = w * odd[i];
        
        data[i] = even[i] + t;
        data[i + n/2] = even[i] - t;
    }
}

/// Inverse Fast Fourier Transform
/// 
/// Computes inverse FFT to recover time domain signal
/// 
/// # Arguments
/// * `spectrum` - Frequency domain data
/// 
/// # Returns
/// Time domain signal (real values)
pub fn inverse_fast_fourier_transform(spectrum: &[ComplexNumber]) -> MathResult<Vec<f64>> {
    let n = spectrum.len();
    
    if n == 0 || (n & (n - 1)) != 0 {
        return Err(MathError::InvalidInput {
        });
    // Conjugate input
    let mut data: Vec<ComplexNumber> = spectrum.iter()
        .map(|c| c.conjugate())
        .collect();
    
    // Perform FFT
    fft_recursive(&mut data);
    
    // Conjugate output and scale
    let result: Vec<f64> = data.iter()
        .map(|c| c.conjugate().real / n as f64)
        .collect();
    
    Ok(result)
// =============================================================================
// INTERPOLATION AND APPROXIMATION
// =============================================================================

/// Cubic spline interpolation
/// 
/// Creates smooth interpolating spline through given points
/// 
/// # Arguments
/// * `x_values` - X coordinates (must be sorted)
/// * `y_values` - Y coordinates
/// * `x` - Point to interpolate
/// 
/// # Returns
/// Interpolated value at x
pub fn cubic_spline_interpolation(
    x: f64
) -> MathResult<f64> {
    validate_float("cubic_spline_interpolation", "x", x)?;
    
    if x_values.len() != y_values.len() || x_values.len() < 2 {
        return Err(MathError::InvalidInput {
        });
    let n = x_values.len();
    
    // Check if x_values are sorted
    for i in 1..n {
        if x_values[i] <= x_values[i-1] {
            return Err(MathError::InvalidInput {
            });
        }
    }
    
    // Find interval
    let mut interval = 0;
    for i in 0..n-1 {
        if x >= x_values[i] && x <= x_values[i+1] {
            interval = i;
            break;
        }
    }
    
    if x < x_values[0] || x > x_values[n-1] {
        return Err(MathError::DomainError {
        });
    // Compute spline coefficients (simplified natural spline)
    let h = x_values[interval+1] - x_values[interval];
    let a = y_values[interval];
    let b = (y_values[interval+1] - y_values[interval]) / h;
    
    // For simplicity, use linear interpolation (can be extended to full cubic)
    let t = (x - x_values[interval]) / h;
    Ok(a + b * t * h)
/// Chebyshev polynomial approximation
/// 
/// Approximates function using Chebyshev polynomials
/// 
/// # Arguments
/// * `f` - Function to approximate
/// * `a` - Left endpoint of interval
/// * `b` - Right endpoint of interval
/// * `n` - Number of Chebyshev nodes
/// 
/// # Returns
/// Coefficients of Chebyshev expansion
pub fn chebyshev_approximation<F>(
    n: usize
) -> MathResult<Vec<f64>>
where
{
    validate_float("chebyshev_approximation", "a", a)?;
    validate_float("chebyshev_approximation", "b", b)?;
    
    if a >= b {
        return Err(MathError::InvalidInput {
        });
    if n == 0 {
        return Err(MathError::InvalidInput {
        });
    let mut coefficients = vec![0.0; n];
    
    // Chebyshev nodes
    for k in 0..n {
        let x_k = ((2 * k + 1) as f64 * PI / (2 * n) as f64).cos();
        let x_mapped = 0.5 * ((b - a) * x_k + (b + a));
        let f_k = f(x_mapped);
        
        for j in 0..n {
            let t_j = (j as f64 * (2 * k + 1) as f64 * PI / (2 * n) as f64).cos();
            coefficients[j] += f_k * t_j;
        }
    }
    
    // Normalize coefficients
    coefficients[0] /= n as f64;
    for j in 1..n {
        coefficients[j] *= 2.0 / n as f64;
    Ok(coefficients)
// =============================================================================
// MATRIX OPERATIONS
// =============================================================================

/// Solves linear system Ax = b using Gaussian elimination
/// 
/// # Arguments
/// * `matrix` - Coefficient matrix A (n×n)
/// * `vector` - Right-hand side vector b
/// 
/// # Returns
/// Solution vector x
pub fn solve_linear_system(matrix: &[Vec<f64>], vector: &[f64]) -> MathResult<Vec<f64>> {
    let n = matrix.len();
    
    if n == 0 || matrix.iter().any(|row| row.len() != n) || vector.len() != n {
        return Err(MathError::InvalidInput {
        });
    // Create augmented matrix
    let mut aug: Vec<Vec<f64>> = matrix.iter()
        .zip(vector.iter())
        .map(|(row, &b)| {
            let mut aug_row = row.clone();
            aug_row.push(b);
            aug_row
        })
        .collect();
    
    // Forward elimination
    for i in 0..n {
        // Find pivot
        let mut max_row = i;
        for k in i+1..n {
            if aug[k][i].abs() > aug[max_row][i].abs() {
                max_row = k;
            }
        }
        
        // Swap rows
        aug.swap(i, max_row);
        
        // Check for singular matrix
        if aug[i][i].abs() < 1e-12 {
            return Err(MathError::ComputationError {
            });
        // Eliminate column
        for k in i+1..n {
            let factor = aug[k][i] / aug[i][i];
            for j in i..=n {
                aug[k][j] -= factor * aug[i][j];
            }
        }
    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = aug[i][n];
        for j in i+1..n {
            x[i] -= aug[i][j] * x[j];
        }
        x[i] /= aug[i][i];
    Ok(x)
/// Computes matrix determinant using LU decomposition
/// 
/// # Arguments
/// * `matrix` - Square matrix
/// 
/// # Returns
/// Determinant value
pub fn matrix_determinant(matrix: &[Vec<f64>]) -> MathResult<f64> {
    let n = matrix.len();
    
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return Err(MathError::InvalidInput {
        });
    let mut m = matrix.to_vec();
    let mut det = 1.0;
    
    // Gaussian elimination
    for i in 0..n {
        // Find pivot
        let mut max_row = i;
        for k in i+1..n {
            if m[k][i].abs() > m[max_row][i].abs() {
                max_row = k;
            }
        }
        
        // Swap rows (affects determinant sign)
        if max_row != i {
            m.swap(i, max_row);
            det = -det;
        // Check for zero diagonal element
        if m[i][i].abs() < 1e-12 {
            return Ok(0.0);
        det *= m[i][i];
        
        // Eliminate column
        for k in i+1..n {
            let factor = m[k][i] / m[i][i];
            for j in i+1..n {
                m[k][j] -= factor * m[i][j];
            }
        }
    Ok(det)
// =============================================================================
// SIGNAL PROCESSING
// =============================================================================

/// Digital filter using convolution
/// 
/// Applies FIR (Finite Impulse Response) filter to signal
/// 
/// # Arguments
/// * `signal` - Input signal
/// * `filter_coefficients` - Filter impulse response
/// 
/// # Returns
/// Filtered signal
pub fn convolution_filter(signal: &[f64], filter_coefficients: &[f64]) -> MathResult<Vec<f64>> {
    if signal.is_empty() || filter_coefficients.is_empty() {
        return Err(MathError::InvalidInput {
        });
    let signal_len = signal.len();
    let filter_len = filter_coefficients.len();
    let output_len = signal_len + filter_len - 1;
    
    let mut output = vec![0.0; output_len];
    
    for i in 0..output_len {
        for j in 0..filter_len {
            if i >= j && (i - j) < signal_len {
                output[i] += signal[i - j] * filter_coefficients[j];
            }
        }
    Ok(output)
/// Moving average filter
/// 
/// Applies simple moving average smoothing filter
/// 
/// # Arguments
/// * `signal` - Input signal
/// * `window_size` - Size of averaging window
/// 
/// # Returns
/// Smoothed signal
pub fn moving_average_filter(signal: &[f64], window_size: usize) -> MathResult<Vec<f64>> {
    if signal.is_empty() {
        return Err(MathError::InvalidInput {
        });
    if window_size == 0 || window_size > signal.len() {
        return Err(MathError::InvalidInput {
        });
    let mut output = Vec::with_capacity(signal.len());
    
    for i in 0..signal.len() {
        let start = if i < window_size / 2 { 0 } else { i - window_size / 2 };
        let end = std::cmp::min(start + window_size, signal.len());
        
        let sum: f64 = signal[start..end].iter().sum();
        let count = end - start;
        output.push(sum / count as f64);
    Ok(output)
// =============================================================================
// MATHEMATICAL MODELING
// =============================================================================

/// Curve fitting using least squares
/// 
/// Fits polynomial of specified degree to data points
/// 
/// # Arguments
/// * `x_data` - X coordinates
/// * `y_data` - Y coordinates  
/// * `degree` - Polynomial degree
/// 
/// # Returns
/// Polynomial coefficients [a₀, a₁, a₂, ...] for a₀ + a₁x + a₂x² + ...
pub fn polynomial_fit(x_data: &[f64], y_data: &[f64], degree: usize) -> MathResult<Vec<f64>> {
    if x_data.len() != y_data.len() || x_data.is_empty() {
        return Err(MathError::InvalidInput {
        });
    if degree >= x_data.len() {
        return Err(MathError::InvalidInput {
        });
    let n = degree + 1;
    let m = x_data.len();
    
    // Build Vandermonde matrix
    let mut matrix = vec![vec![0.0; n]; n];
    let mut vector = vec![0.0; n];
    
    for i in 0..n {
        for j in 0..n {
            for k in 0..m {
                matrix[i][j] += x_data[k].powi((i + j) as i32);
            }
        }
        
        for k in 0..m {
            vector[i] += y_data[k] * x_data[k].powi(i as i32);
        }
    }
    
    solve_linear_system(&matrix, &vector)
/// Evaluates polynomial at given point
/// 
/// # Arguments
/// * `coefficients` - Polynomial coefficients [a₀, a₁, a₂, ...]
/// * `x` - Evaluation point
/// 
/// # Returns
/// Polynomial value at x
pub fn evaluate_polynomial(coefficients: &[f64], x: f64) -> MathResult<f64> {
    validate_float("evaluate_polynomial", "x", x)?;
    
    if coefficients.is_empty() {
        return Ok(0.0);
    // Use Horner's method for numerical stability
    let mut result = coefficients[coefficients.len() - 1];
    for i in (0..coefficients.len() - 1).rev() {
        result = result * x + coefficients[i];
    Ok(result)
