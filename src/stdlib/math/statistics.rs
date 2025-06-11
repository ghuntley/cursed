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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_descriptive_statistics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        assert_eq!(mean(&data).unwrap(), 3.0);
        assert_eq!(median(&data).unwrap(), 3.0);
        assert_eq!(variance(&data).unwrap(), 2.0);
        assert!((standard_deviation(&data).unwrap() - 1.4142135623730951).abs() < 1e-10);
    }
    
    #[test]
    fn test_empty_dataset_errors() {
        let empty_data: Vec<f64> = vec![];
        
        assert!(mean(&empty_data).is_err());
        assert!(median(&empty_data).is_err());
        assert!(variance(&empty_data).is_err());
    }
    
    #[test]
    fn test_percentiles() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        
        assert_eq!(percentile(&data, 0.0).unwrap(), 1.0);
        assert_eq!(percentile(&data, 50.0).unwrap(), 5.5);
        assert_eq!(percentile(&data, 100.0).unwrap(), 10.0);
    }
    
    #[test]
    fn test_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        
        assert!((correlation(&x, &y).unwrap() - 1.0).abs() < 1e-10);
    }
}
