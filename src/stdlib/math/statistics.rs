use crate::error::CursedError;
/// Statistical functions module for CURSED programming language
/// 
/// Provides comprehensive statistical analysis including descriptive statistics,
/// probability distributions, correlation analysis, and data validation utilities.

use super::{MathError, MathResult, validate_float, domain_error, division_by_zero_error};
use std::collections::HashMap;
use std::f64;

// ===================== DESCRIPTIVE STATISTICS =====================

/// Calculate the arithmetic mean (average) of a dataset
pub fn mean(data: &[f64]) -> MathResult<f64> {
    if data.is_empty() {
        return Err(domain_error("mean", 0.0, "empty dataset"));
    }
    
    for &value in data {
        validate_float("mean", "data_point", value)?;
    }
    
    let sum: f64 = data.iter().sum();
    Ok(sum / data.len() as f64)
}

/// Calculate the median of a dataset
pub fn median(data: &[f64]) -> MathResult<f64> {
    if data.is_empty() {
        return Err(domain_error("median", 0.0, "empty dataset"));
    }
    
    for &value in data {
        validate_float("median", "data_point", value)?;
    }
    
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let n = sorted.len();
    if n % 2 == 0 {
        Ok((sorted[n/2 - 1] + sorted[n/2]) / 2.0)
    } else {
        Ok(sorted[n/2])
    }
}

/// Calculate the mode of a dataset (most frequent value)
/// Returns the first mode found in case of ties
pub fn mode(data: &[f64]) -> MathResult<f64> {
    if data.is_empty() {
        return Err(domain_error("mode", 0.0, "empty dataset"));
    }
    
    for &value in data {
        validate_float("mode", "data_point", value)?;
    }
    
    let mut frequency_map = HashMap::new();
    for &value in data {
        *frequency_map.entry(value.to_bits()).or_insert(0) += 1;
    }
    
    let max_frequency = frequency_map.values().max().unwrap();
    let mode_bits = frequency_map
        .iter()
        .find(|(_, &freq)| freq == *max_frequency)
        .map(|(&bits, _)| bits)
        .unwrap();
    
    Ok(f64::from_bits(mode_bits))
}

/// Calculate the population variance of a dataset
pub fn variance(data: &[f64]) -> MathResult<f64> {
    if data.is_empty() {
        return Err(domain_error("variance", 0.0, "empty dataset"));
    }
    
    if data.len() == 1 {
        return Ok(0.0);
    }
    
    let mean_val = mean(data)?;
    let sum_of_squares: f64 = data.iter()
        .map(|&x| (x - mean_val).powi(2))
        .sum();
    
    Ok(sum_of_squares / data.len() as f64)
}

/// Calculate the sample variance of a dataset (Bessel's correction)
pub fn sample_variance(data: &[f64]) -> MathResult<f64> {
    if data.len() < 2 {
        return Err(domain_error("sample_variance", data.len() as f64, "need at least 2 data points"));
    }
    
    let mean_val = mean(data)?;
    let sum_of_squares: f64 = data.iter()
        .map(|&x| (x - mean_val).powi(2))
        .sum();
    
    Ok(sum_of_squares / (data.len() - 1) as f64)
}

/// Calculate the population standard deviation
pub fn standard_deviation(data: &[f64]) -> MathResult<f64> {
    let var = variance(data)?;
    Ok(var.sqrt())
}

/// Calculate the sample standard deviation
pub fn sample_standard_deviation(data: &[f64]) -> MathResult<f64> {
    let var = sample_variance(data)?;
    Ok(var.sqrt())
}

/// Calculate the skewness of a dataset (measure of asymmetry)
pub fn skewness(data: &[f64]) -> MathResult<f64> {
    if data.len() < 3 {
        return Err(domain_error("skewness", data.len() as f64, "need at least 3 data points"));
    }
    
    let mean_val = mean(data)?;
    let std_dev = standard_deviation(data)?;
    
    if std_dev == 0.0 {
        return Ok(0.0); // No skewness if no variation
    }
    
    let sum_cubed: f64 = data.iter()
        .map(|&x| ((x - mean_val) / std_dev).powi(3))
        .sum();
    
    Ok(sum_cubed / data.len() as f64)
}

/// Calculate the kurtosis of a dataset (measure of tail heaviness)
pub fn kurtosis(data: &[f64]) -> MathResult<f64> {
    if data.len() < 4 {
        return Err(domain_error("kurtosis", data.len() as f64, "need at least 4 data points"));
    }
    
    let mean_val = mean(data)?;
    let std_dev = standard_deviation(data)?;
    
    if std_dev == 0.0 {
        return Ok(0.0); // No kurtosis if no variation
    }
    
    let sum_fourth: f64 = data.iter()
        .map(|&x| ((x - mean_val) / std_dev).powi(4))
        .sum();
    
    Ok(sum_fourth / data.len() as f64 - 3.0) // Excess kurtosis
}

// ===================== STATISTICAL MEASURES =====================

/// Calculate a specific percentile of a dataset
pub fn percentile(data: &[f64], p: f64) -> MathResult<f64> {
    if data.is_empty() {
        return Err(domain_error("percentile", 0.0, "empty dataset"));
    }
    
    if p < 0.0 || p > 100.0 {
        return Err(domain_error("percentile", p, "percentile must be between 0 and 100"));
    }
    
    for &value in data {
        validate_float("percentile", "data_point", value)?;
    }
    
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let index = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;
    
    if lower == upper {
        Ok(sorted[lower])
    } else {
        let weight = index - lower as f64;
        Ok(sorted[lower] * (1.0 - weight) + sorted[upper] * weight)
    }
}

/// Calculate the first quartile (25th percentile)
pub fn q1(data: &[f64]) -> MathResult<f64> {
    percentile(data, 25.0)
}

/// Calculate the third quartile (75th percentile)
pub fn q3(data: &[f64]) -> MathResult<f64> {
    percentile(data, 75.0)
}

/// Calculate the range (max - min) of a dataset
pub fn range(data: &[f64]) -> MathResult<f64> {
    if data.is_empty() {
        return Err(domain_error("range", 0.0, "empty dataset"));
    }
    
    for &value in data {
        validate_float("range", "data_point", value)?;
    }
    
    let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    Ok(max_val - min_val)
}

/// Calculate the interquartile range (Q3 - Q1)
pub fn interquartile_range(data: &[f64]) -> MathResult<f64> {
    let q1_val = q1(data)?;
    let q3_val = q3(data)?;
    Ok(q3_val - q1_val)
}

/// Calculate the five-number summary (min, Q1, median, Q3, max)
pub fn five_number_summary(data: &[f64]) -> MathResult<(f64, f64, f64, f64, f64)> {
    if data.is_empty() {
        return Err(domain_error("five_number_summary", 0.0, "empty dataset"));
    }
    
    for &value in data {
        validate_float("five_number_summary", "data_point", value)?;
    }
    
    let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let q1_val = q1(data)?;
    let median_val = median(data)?;
    let q3_val = q3(data)?;
    
    Ok((min_val, q1_val, median_val, q3_val, max_val))
}

// ===================== PROBABILITY DISTRIBUTIONS =====================

/// Normal (Gaussian) distribution probability density function
pub fn normal_pdf(x: f64, mean: f64, std_dev: f64) -> MathResult<f64> {
    validate_float("normal_pdf", "x", x)?;
    validate_float("normal_pdf", "mean", mean)?;
    validate_float("normal_pdf", "std_dev", std_dev)?;
    
    if std_dev <= 0.0 {
        return Err(domain_error("normal_pdf", std_dev, "standard deviation must be positive"));
    }
    
    let coefficient = 1.0 / (std_dev * (2.0 * f64::consts::PI).sqrt());
    let exponent = -0.5 * ((x - mean) / std_dev).powi(2);
    
    Ok(coefficient * exponent.exp())
}

/// Standard normal distribution CDF approximation (using error function)
pub fn standard_normal_cdf(z: f64) -> MathResult<f64> {
    validate_float("standard_normal_cdf", "z", z)?;
    
    // Using Abramowitz and Stegun approximation
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;
    
    let sign = if z < 0.0 { -1.0 } else { 1.0 };
    let z_abs = z.abs();
    
    let t = 1.0 / (1.0 + p * z_abs);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-z_abs * z_abs).exp();
    
    Ok(0.5 * (1.0 + sign * y))
}

/// Normal distribution cumulative distribution function
pub fn normal_cdf(x: f64, mean: f64, std_dev: f64) -> MathResult<f64> {
    validate_float("normal_cdf", "x", x)?;
    validate_float("normal_cdf", "mean", mean)?;
    validate_float("normal_cdf", "std_dev", std_dev)?;
    
    if std_dev <= 0.0 {
        return Err(domain_error("normal_cdf", std_dev, "standard deviation must be positive"));
    }
    
    let z = (x - mean) / std_dev;
    standard_normal_cdf(z)
}

/// Uniform distribution probability density function
pub fn uniform_pdf(x: f64, a: f64, b: f64) -> MathResult<f64> {
    validate_float("uniform_pdf", "x", x)?;
    validate_float("uniform_pdf", "a", a)?;
    validate_float("uniform_pdf", "b", b)?;
    
    if a >= b {
        return Err(domain_error("uniform_pdf", b - a, "b must be greater than a"));
    }
    
    if x >= a && x <= b {
        Ok(1.0 / (b - a))
    } else {
        Ok(0.0)
    }
}

/// Uniform distribution cumulative distribution function
pub fn uniform_cdf(x: f64, a: f64, b: f64) -> MathResult<f64> {
    validate_float("uniform_cdf", "x", x)?;
    validate_float("uniform_cdf", "a", a)?;
    validate_float("uniform_cdf", "b", b)?;
    
    if a >= b {
        return Err(domain_error("uniform_cdf", b - a, "b must be greater than a"));
    }
    
    if x < a {
        Ok(0.0)
    } else if x > b {
        Ok(1.0)
    } else {
        Ok((x - a) / (b - a))
    }
}

/// Exponential distribution probability density function
pub fn exponential_pdf(x: f64, lambda: f64) -> MathResult<f64> {
    validate_float("exponential_pdf", "x", x)?;
    validate_float("exponential_pdf", "lambda", lambda)?;
    
    if lambda <= 0.0 {
        return Err(domain_error("exponential_pdf", lambda, "lambda must be positive"));
    }
    
    if x < 0.0 {
        Ok(0.0)
    } else {
        Ok(lambda * (-lambda * x).exp())
    }
}

/// Exponential distribution cumulative distribution function
pub fn exponential_cdf(x: f64, lambda: f64) -> MathResult<f64> {
    validate_float("exponential_cdf", "x", x)?;
    validate_float("exponential_cdf", "lambda", lambda)?;
    
    if lambda <= 0.0 {
        return Err(domain_error("exponential_cdf", lambda, "lambda must be positive"));
    }
    
    if x < 0.0 {
        Ok(0.0)
    } else {
        Ok(1.0 - (-lambda * x).exp())
    }
}

/// Binomial distribution probability mass function
pub fn binomial_pmf(k: i32, n: i32, p: f64) -> MathResult<f64> {
    validate_float("binomial_pmf", "p", p)?;
    
    if n < 0 {
        return Err(domain_error("binomial_pmf", n as f64, "n must be non-negative"));
    }
    
    if k < 0 || k > n {
        return Ok(0.0);
    }
    
    if p < 0.0 || p > 1.0 {
        return Err(domain_error("binomial_pmf", p, "p must be between 0 and 1"));
    }
    
    if p == 0.0 {
        return Ok(if k == 0 { 1.0 } else { 0.0 });
    }
    
    if p == 1.0 {
        return Ok(if k == n { 1.0 } else { 0.0 });
    }
    
    // Calculate binomial coefficient C(n, k)
    let mut coeff = 1.0;
    for i in 0..k {
        coeff *= (n - i) as f64 / (i + 1) as f64;
    }
    
    Ok(coeff * p.powi(k) * (1.0 - p).powi(n - k))
}

/// Poisson distribution probability mass function
pub fn poisson_pmf(k: i32, lambda: f64) -> MathResult<f64> {
    validate_float("poisson_pmf", "lambda", lambda)?;
    
    if lambda <= 0.0 {
        return Err(domain_error("poisson_pmf", lambda, "lambda must be positive"));
    }
    
    if k < 0 {
        return Ok(0.0);
    }
    
    // For large k, use Stirling's approximation
    if k > 170 {
        return Ok(0.0); // Probability is essentially zero
    }
    
    // Calculate k! iteratively to avoid overflow
    let mut factorial = 1.0;
    for i in 1..=k {
        factorial *= i as f64;
    }
    
    Ok((-lambda).exp() * lambda.powi(k) / factorial)
}

// ===================== CORRELATION AND COVARIANCE =====================

/// Calculate the covariance between two datasets
pub fn covariance(x: &[f64], y: &[f64]) -> MathResult<f64> {
    if x.is_empty() || y.is_empty() {
        return Err(domain_error("covariance", 0.0, "empty dataset"));
    }
    
    if x.len() != y.len() {
        return Err(domain_error("covariance", (x.len() - y.len()) as f64, "datasets must have same length"));
    }
    
    for &value in x {
        validate_float("covariance", "x_data_point", value)?;
    }
    for &value in y {
        validate_float("covariance", "y_data_point", value)?;
    }
    
    let mean_x = mean(x)?;
    let mean_y = mean(y)?;
    
    let sum: f64 = x.iter().zip(y.iter())
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum();
    
    Ok(sum / x.len() as f64)
}

/// Calculate the sample covariance between two datasets
pub fn sample_covariance(x: &[f64], y: &[f64]) -> MathResult<f64> {
    if x.len() < 2 || y.len() < 2 {
        return Err(domain_error("sample_covariance", x.len().min(y.len()) as f64, "need at least 2 data points"));
    }
    
    if x.len() != y.len() {
        return Err(domain_error("sample_covariance", (x.len() - y.len()) as f64, "datasets must have same length"));
    }
    
    let mean_x = mean(x)?;
    let mean_y = mean(y)?;
    
    let sum: f64 = x.iter().zip(y.iter())
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum();
    
    Ok(sum / (x.len() - 1) as f64)
}

/// Calculate the Pearson correlation coefficient between two datasets
pub fn correlation(x: &[f64], y: &[f64]) -> MathResult<f64> {
    if x.is_empty() || y.is_empty() {
        return Err(domain_error("correlation", 0.0, "empty dataset"));
    }
    
    if x.len() != y.len() {
        return Err(domain_error("correlation", (x.len() - y.len()) as f64, "datasets must have same length"));
    }
    
    let std_x = standard_deviation(x)?;
    let std_y = standard_deviation(y)?;
    
    if std_x == 0.0 || std_y == 0.0 {
        return Err(division_by_zero_error("correlation"));
    }
    
    let cov = covariance(x, y)?;
    Ok(cov / (std_x * std_y))
}

/// Calculate the Spearman rank correlation coefficient
pub fn spearman_correlation(x: &[f64], y: &[f64]) -> MathResult<f64> {
    if x.is_empty() || y.is_empty() {
        return Err(domain_error("spearman_correlation", 0.0, "empty dataset"));
    }
    
    if x.len() != y.len() {
        return Err(domain_error("spearman_correlation", (x.len() - y.len()) as f64, "datasets must have same length"));
    }
    
    // Create ranks for x and y
    let ranks_x = create_ranks(x);
    let ranks_y = create_ranks(y);
    
    // Calculate Pearson correlation of ranks
    correlation(&ranks_x, &ranks_y)
}

/// Helper function to create ranks for Spearman correlation
fn create_ranks(data: &[f64]) -> Vec<f64> {
    let n = data.len();
    let mut indexed_data: Vec<(f64, usize)> = data.iter()
        .enumerate()
        .map(|(i, &val)| (val, i))
        .collect();
    
    // Sort by value
    indexed_data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    
    let mut ranks = vec![0.0; n];
    
    // Assign ranks, handling ties by averaging
    let mut i = 0;
    while i < n {
        let current_value = indexed_data[i].0;
        let mut j = i;
        
        // Find all values equal to current_value
        while j < n && indexed_data[j].0 == current_value {
            j += 1;
        }
        
        // Average rank for tied values
        let avg_rank = ((i + 1) + j) as f64 / 2.0;
        
        // Assign average rank to all tied values
        for k in i..j {
            ranks[indexed_data[k].1] = avg_rank;
        }
        
        i = j;
    }
    
    ranks
}

// ===================== DATA VALIDATION AND OUTLIER DETECTION =====================

/// Detect outliers using the IQR method
pub fn outliers_iqr(data: &[f64], factor: f64) -> MathResult<Vec<f64>> {
    if data.len() < 4 {
        return Err(domain_error("outliers_iqr", data.len() as f64, "need at least 4 data points"));
    }
    
    if factor <= 0.0 {
        return Err(domain_error("outliers_iqr", factor, "factor must be positive"));
    }
    
    let q1_val = q1(data)?;
    let q3_val = q3(data)?;
    let iqr = q3_val - q1_val;
    
    let lower_bound = q1_val - factor * iqr;
    let upper_bound = q3_val + factor * iqr;
    
    let outliers: Vec<f64> = data.iter()
        .cloned()
        .filter(|&x| x < lower_bound || x > upper_bound)
        .collect();
    
    Ok(outliers)
}

/// Detect outliers using the Z-score method
pub fn outliers_z_score(data: &[f64], threshold: f64) -> MathResult<Vec<f64>> {
    if data.len() < 2 {
        return Err(domain_error("outliers_z_score", data.len() as f64, "need at least 2 data points"));
    }
    
    if threshold <= 0.0 {
        return Err(domain_error("outliers_z_score", threshold, "threshold must be positive"));
    }
    
    let mean_val = mean(data)?;
    let std_dev = standard_deviation(data)?;
    
    if std_dev == 0.0 {
        return Ok(Vec::new()); // No outliers if no variation
    }
    
    let outliers: Vec<f64> = data.iter()
        .cloned()
        .filter(|&x| ((x - mean_val) / std_dev).abs() > threshold)
        .collect();
    
    Ok(outliers)
}

/// Check if a dataset contains any invalid values (NaN or infinite)
pub fn has_invalid_values(data: &[f64]) -> bool {
    data.iter().any(|&x| !x.is_finite())
}

/// Remove invalid values (NaN and infinite) from a dataset
pub fn clean_data(data: &[f64]) -> Vec<f64> {
    data.iter()
        .cloned()
        .filter(|&x| x.is_finite())
        .collect()
}

/// Validate that a dataset is suitable for statistical analysis
pub fn validate_dataset(data: &[f64], function_name: &str) -> MathResult<()> {
    if data.is_empty() {
        return Err(domain_error(function_name, 0.0, "empty dataset"));
    }
    
    for &value in data {
        validate_float(function_name, "data_point", value)?;
    }
    
    Ok(())
}

// ===================== UTILITY FUNCTIONS =====================

/// Calculate the harmonic mean of a dataset
pub fn harmonic_mean(data: &[f64]) -> MathResult<f64> {
    validate_dataset(data, "harmonic_mean")?;
    
    // Check for zero or negative values
    if data.iter().any(|&x| x <= 0.0) {
        return Err(domain_error("harmonic_mean", 0.0, "all values must be positive"));
    }
    
    let sum_reciprocals: f64 = data.iter()
        .map(|&x| 1.0 / x)
        .sum();
    
    Ok(data.len() as f64 / sum_reciprocals)
}

/// Calculate the geometric mean of a dataset
pub fn geometric_mean(data: &[f64]) -> MathResult<f64> {
    validate_dataset(data, "geometric_mean")?;
    
    // Check for zero or negative values
    if data.iter().any(|&x| x <= 0.0) {
        return Err(domain_error("geometric_mean", 0.0, "all values must be positive"));
    }
    
    let product: f64 = data.iter()
        .map(|&x| x.ln())
        .sum();
    
    Ok((product / data.len() as f64).exp())
}

/// Calculate the root mean square (quadratic mean)
pub fn root_mean_square(data: &[f64]) -> MathResult<f64> {
    validate_dataset(data, "root_mean_square")?;
    
    let sum_squares: f64 = data.iter()
        .map(|&x| x * x)
        .sum();
    
    Ok((sum_squares / data.len() as f64).sqrt())
}

/// Calculate the coefficient of variation (relative standard deviation)
pub fn coefficient_of_variation(data: &[f64]) -> MathResult<f64> {
    let mean_val = mean(data)?;
    let std_dev = standard_deviation(data)?;
    
    if mean_val == 0.0 {
        return Err(division_by_zero_error("coefficient_of_variation"));
    }
    
    Ok(std_dev / mean_val.abs())
}

// ===================== STATISTICAL TESTS =====================

/// Perform a one-sample t-test
pub fn t_test_one_sample(data: &[f64], population_mean: f64) -> MathResult<(f64, f64)> {
    if data.len() < 2 {
        return Err(domain_error("t_test_one_sample", data.len() as f64, "need at least 2 data points"));
    }
    
    validate_float("t_test_one_sample", "population_mean", population_mean)?;
    
    let sample_mean = mean(data)?;
    let sample_std = sample_standard_deviation(data)?;
    let n = data.len() as f64;
    
    let t_statistic = (sample_mean - population_mean) / (sample_std / n.sqrt());
    let degrees_of_freedom = n - 1.0;
    
    Ok((t_statistic, degrees_of_freedom))
}

/// Perform a two-sample t-test (assuming equal variances)
pub fn t_test_two_sample(x: &[f64], y: &[f64]) -> MathResult<(f64, f64)> {
    if x.len() < 2 || y.len() < 2 {
        return Err(domain_error("t_test_two_sample", (x.len() + y.len()) as f64, "need at least 2 data points in each sample"));
    }
    
    let mean_x = mean(x)?;
    let mean_y = mean(y)?;
    let var_x = sample_variance(x)?;
    let var_y = sample_variance(y)?;
    let n_x = x.len() as f64;
    let n_y = y.len() as f64;
    
    // Pooled variance
    let pooled_var = ((n_x - 1.0) * var_x + (n_y - 1.0) * var_y) / (n_x + n_y - 2.0);
    let pooled_std = pooled_var.sqrt();
    
    let t_statistic = (mean_x - mean_y) / (pooled_std * (1.0 / n_x + 1.0 / n_y).sqrt());
    let degrees_of_freedom = n_x + n_y - 2.0;
    
    Ok((t_statistic, degrees_of_freedom))
}

/// Chi-square goodness of fit test
pub fn chi_square_test(observed: &[f64], expected: &[f64]) -> MathResult<(f64, f64)> {
    if observed.is_empty() || expected.is_empty() {
        return Err(domain_error("chi_square_test", 0.0, "empty dataset"));
    }
    
    if observed.len() != expected.len() {
        return Err(domain_error("chi_square_test", (observed.len() - expected.len()) as f64, "datasets must have same length"));
    }
    
    for &val in expected {
        if val <= 0.0 {
            return Err(domain_error("chi_square_test", val, "expected frequencies must be positive"));
        }
    }
    
    let mut chi_square = 0.0;
    for i in 0..observed.len() {
        let diff = observed[i] - expected[i];
        chi_square += (diff * diff) / expected[i];
    }
    
    let degrees_of_freedom = (observed.len() - 1) as f64;
    Ok((chi_square, degrees_of_freedom))
}

/// One-way ANOVA F-test
pub fn anova_one_way(groups: &[&[f64]]) -> MathResult<(f64, f64, f64)> {
    if groups.len() < 2 {
        return Err(domain_error("anova_one_way", groups.len() as f64, "need at least 2 groups"));
    }
    
    // Check each group has at least one observation
    for (i, group) in groups.iter().enumerate() {
        if group.is_empty() {
            return Err(domain_error("anova_one_way", i as f64, "group cannot be empty"));
        }
    }
    
    // Calculate overall mean and group means
    let all_data: Vec<f64> = groups.iter().flat_map(|&group| group.iter().cloned()).collect();
    let overall_mean = mean(&all_data)?;
    
    let group_means: Result<Vec<f64>, _> = groups.iter()
        .map(|&group| mean(group))
        .collect();
    let group_means = group_means?;
    
    // Calculate sum of squares
    let mut between_group_ss = 0.0;
    let mut within_group_ss = 0.0;
    
    for (i, &group) in groups.iter().enumerate() {
        let group_mean = group_means[i];
        let n_i = group.len() as f64;
        
        // Between-group sum of squares
        between_group_ss += n_i * (group_mean - overall_mean).powi(2);
        
        // Within-group sum of squares
        for &value in group {
            within_group_ss += (value - group_mean).powi(2);
        }
    }
    
    let k = groups.len() as f64; // number of groups
    let n = all_data.len() as f64; // total number of observations
    
    let between_df = k - 1.0;
    let within_df = n - k;
    
    let between_ms = between_group_ss / between_df;
    let within_ms = within_group_ss / within_df;
    
    let f_statistic = between_ms / within_ms;
    
    Ok((f_statistic, between_df, within_df))
}

// ===================== REGRESSION ANALYSIS =====================

/// Simple linear regression: y = a + b*x
pub fn linear_regression(x: &[f64], y: &[f64]) -> MathResult<(f64, f64, f64)> {
    if x.is_empty() || y.is_empty() {
        return Err(domain_error("linear_regression", 0.0, "empty dataset"));
    }
    
    if x.len() != y.len() {
        return Err(domain_error("linear_regression", (x.len() - y.len()) as f64, "datasets must have same length"));
    }
    
    if x.len() < 2 {
        return Err(domain_error("linear_regression", x.len() as f64, "need at least 2 data points"));
    }
    
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xx: f64 = x.iter().map(|&xi| xi * xi).sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(&xi, &yi)| xi * yi).sum();
    
    let mean_x = sum_x / n;
    let mean_y = sum_y / n;
    
    // Calculate slope (b) and intercept (a)
    let numerator = sum_xy - n * mean_x * mean_y;
    let denominator = sum_xx - n * mean_x * mean_x;
    
    if denominator.abs() < f64::EPSILON {
        return Err(division_by_zero_error("linear_regression"));
    }
    
    let slope = numerator / denominator;
    let intercept = mean_y - slope * mean_x;
    
    // Calculate R-squared
    let ss_tot: f64 = y.iter().map(|&yi| (yi - mean_y).powi(2)).sum();
    let ss_res: f64 = x.iter().zip(y.iter())
        .map(|(&xi, &yi)| {
            let predicted = intercept + slope * xi;
            (yi - predicted).powi(2)
        })
        .sum();
    
    let r_squared = if ss_tot.abs() < f64::EPSILON {
        1.0 // Perfect fit if no variation in y
    } else {
        1.0 - (ss_res / ss_tot)
    };
    
    Ok((intercept, slope, r_squared))
}

/// Multiple linear regression coefficients (basic implementation)
pub fn multiple_linear_regression(x_matrix: &[&[f64]], y: &[f64]) -> MathResult<Vec<f64>> {
    if x_matrix.is_empty() || y.is_empty() {
        return Err(domain_error("multiple_linear_regression", 0.0, "empty dataset"));
    }
    
    let n = y.len();
    let p = x_matrix.len(); // number of predictors
    
    // Check dimensions
    for (i, &x_col) in x_matrix.iter().enumerate() {
        if x_col.len() != n {
            return Err(domain_error("multiple_linear_regression", i as f64, "all predictor vectors must have same length as y"));
        }
    }
    
    if n <= p {
        return Err(domain_error("multiple_linear_regression", (n - p) as f64, "need more observations than predictors"));
    }
    
    // Create design matrix X with intercept column
    let mut x_design = vec![vec![0.0; p + 1]; n];
    for i in 0..n {
        x_design[i][0] = 1.0; // intercept
        for j in 0..p {
            x_design[i][j + 1] = x_matrix[j][i];
        }
    }
    
    // Solve normal equations: (X'X)β = X'y using basic matrix operations
    solve_normal_equations(&x_design, y)
}

/// Helper function to solve normal equations for multiple regression
fn solve_normal_equations(x: &[Vec<f64>], y: &[f64]) -> MathResult<Vec<f64>> {
    let n = x.len();
    let p = x[0].len();
    
    // Calculate X'X
    let mut xtx = vec![vec![0.0; p]; p];
    for i in 0..p {
        for j in 0..p {
            for k in 0..n {
                xtx[i][j] += x[k][i] * x[k][j];
            }
        }
    }
    
    // Calculate X'y
    let mut xty = vec![0.0; p];
    for i in 0..p {
        for k in 0..n {
            xty[i] += x[k][i] * y[k];
        }
    }
    
    // Solve using Gaussian elimination (simplified)
    gaussian_elimination(&mut xtx, &mut xty)
}

/// Simplified Gaussian elimination for solving linear systems
fn gaussian_elimination(a: &mut [Vec<f64>], b: &mut [f64]) -> MathResult<Vec<f64>> {
    let n = a.len();
    
    // Forward elimination
    for i in 0..n {
        // Find pivot
        let mut max_row = i;
        for k in (i + 1)..n {
            if a[k][i].abs() > a[max_row][i].abs() {
                max_row = k;
            }
        }
        
        // Swap rows
        a.swap(i, max_row);
        b.swap(i, max_row);
        
        // Check for singular matrix
        if a[i][i].abs() < f64::EPSILON {
            return Err(domain_error("gaussian_elimination", a[i][i], "singular matrix"));
        }
        
        // Make all rows below this one 0 in current column
        for k in (i + 1)..n {
            let factor = a[k][i] / a[i][i];
            for j in i..n {
                a[k][j] -= factor * a[i][j];
            }
            b[k] -= factor * b[i];
        }
    }
    
    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }
    
    Ok(x)
}

