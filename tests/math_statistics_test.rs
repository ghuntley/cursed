/// Comprehensive test suite for CURSED math statistics module
/// 
/// Tests all statistical functions including descriptive statistics,
/// probability distributions, statistical tests, and regression analysis.
/// 
/// Validates mathematical accuracy, error handling, and edge cases.

use cursed::stdlib::math::statistics::*;
use cursed::stdlib::math::{MathError, MathResult};

const TOLERANCE: f64 = 1e-10;

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() < tolerance,
        "Expected {}, got {}, difference: {}",
        expected, actual, (actual - expected).abs()
    );
}

// ===================== DESCRIPTIVE STATISTICS TESTS =====================

#[test]
fn test_mean_basic() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_close(mean(&data).unwrap(), 3.0, TOLERANCE);
    
    let data2 = vec![10.0, 20.0, 30.0];
    assert_close(mean(&data2).unwrap(), 20.0, TOLERANCE);
}

#[test]
fn test_mean_edge_cases() {
    // Empty dataset
    let empty: Vec<f64> = vec![];
    assert!(mean(&empty).is_err());
    
    // Single value
    let single = vec![42.0];
    assert_close(mean(&single).unwrap(), 42.0, TOLERANCE);
    
    // Negative values
    let negative = vec![-1.0, -2.0, -3.0];
    assert_close(mean(&negative).unwrap(), -2.0, TOLERANCE);
}

#[test]
fn test_median_basic() {
    // Odd number of elements
    let odd_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_close(median(&odd_data).unwrap(), 3.0, TOLERANCE);
    
    // Even number of elements
    let even_data = vec![1.0, 2.0, 3.0, 4.0];
    assert_close(median(&even_data).unwrap(), 2.5, TOLERANCE);
}

#[test]
fn test_median_unsorted() {
    let data = vec![5.0, 1.0, 3.0, 2.0, 4.0];
    assert_close(median(&data).unwrap(), 3.0, TOLERANCE);
}

#[test]
fn test_mode_basic() {
    let data = vec![1.0, 2.0, 2.0, 3.0, 4.0];
    assert_close(mode(&data).unwrap(), 2.0, TOLERANCE);
    
    // All values same frequency (returns first occurrence)
    let data2 = vec![1.0, 2.0, 3.0];
    let result = mode(&data2).unwrap();
    assert!(result == 1.0 || result == 2.0 || result == 3.0);
}

#[test]
fn test_variance_and_standard_deviation() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    
    // Population variance: ((1-3)² + (2-3)² + (3-3)² + (4-3)² + (5-3)²) / 5 = 10/5 = 2
    assert_close(variance(&data).unwrap(), 2.0, TOLERANCE);
    assert_close(standard_deviation(&data).unwrap(), 2.0_f64.sqrt(), TOLERANCE);
    
    // Sample variance: 10/4 = 2.5
    assert_close(sample_variance(&data).unwrap(), 2.5, TOLERANCE);
    assert_close(sample_standard_deviation(&data).unwrap(), 2.5_f64.sqrt(), TOLERANCE);
}

#[test]
fn test_skewness_and_kurtosis() {
    let symmetric_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let skew = skewness(&symmetric_data).unwrap();
    assert_close(skew, 0.0, 1e-6); // Symmetric distribution should have skewness near 0
    
    let kurt = kurtosis(&symmetric_data).unwrap();
    // Uniform distribution has excess kurtosis of -1.2
    assert!(kurt < 0.0);
}

#[test]
fn test_percentiles() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    
    assert_close(percentile(&data, 0.0).unwrap(), 1.0, TOLERANCE);
    assert_close(percentile(&data, 25.0).unwrap(), 3.25, TOLERANCE);
    assert_close(percentile(&data, 50.0).unwrap(), 5.5, TOLERANCE);
    assert_close(percentile(&data, 75.0).unwrap(), 7.75, TOLERANCE);
    assert_close(percentile(&data, 100.0).unwrap(), 10.0, TOLERANCE);
    
    assert_close(q1(&data).unwrap(), 3.25, TOLERANCE);
    assert_close(q3(&data).unwrap(), 7.75, TOLERANCE);
}

#[test]
fn test_range_measures() {
    let data = vec![1.0, 3.0, 5.0, 7.0, 9.0];
    
    assert_close(range(&data).unwrap(), 8.0, TOLERANCE);
    let iqr = interquartile_range(&data).unwrap();
    assert!(iqr > 0.0);
    
    let summary = five_number_summary(&data).unwrap();
    assert_eq!(summary.0, 1.0); // min
    assert_eq!(summary.4, 9.0); // max
    assert_close(summary.2, 5.0, TOLERANCE); // median
}

// ===================== PROBABILITY DISTRIBUTIONS TESTS =====================

#[test]
fn test_normal_distribution() {
    // Standard normal at mean
    let pdf_at_mean = normal_pdf(0.0, 0.0, 1.0).unwrap();
    assert_close(pdf_at_mean, 0.3989422804014327, 1e-10);
    
    // Standard normal CDF at mean should be 0.5
    let cdf_at_mean = normal_cdf(0.0, 0.0, 1.0).unwrap();
    assert_close(cdf_at_mean, 0.5, 1e-6);
    
    // CDF properties
    assert_close(normal_cdf(-1000.0, 0.0, 1.0).unwrap(), 0.0, 1e-6);
    assert_close(normal_cdf(1000.0, 0.0, 1.0).unwrap(), 1.0, 1e-6);
}

#[test]
fn test_uniform_distribution() {
    // Uniform PDF on [0, 1]
    assert_close(uniform_pdf(0.5, 0.0, 1.0).unwrap(), 1.0, TOLERANCE);
    assert_close(uniform_pdf(-0.5, 0.0, 1.0).unwrap(), 0.0, TOLERANCE);
    assert_close(uniform_pdf(1.5, 0.0, 1.0).unwrap(), 0.0, TOLERANCE);
    
    // Uniform CDF on [0, 1]
    assert_close(uniform_cdf(0.0, 0.0, 1.0).unwrap(), 0.0, TOLERANCE);
    assert_close(uniform_cdf(0.5, 0.0, 1.0).unwrap(), 0.5, TOLERANCE);
    assert_close(uniform_cdf(1.0, 0.0, 1.0).unwrap(), 1.0, TOLERANCE);
}

#[test]
fn test_exponential_distribution() {
    // Exponential with lambda = 1
    let pdf_at_zero = exponential_pdf(0.0, 1.0).unwrap();
    assert_close(pdf_at_zero, 1.0, TOLERANCE);
    
    let pdf_at_one = exponential_pdf(1.0, 1.0).unwrap();
    assert_close(pdf_at_one, 1.0_f64.exp().recip(), 1e-10);
    
    // CDF properties
    assert_close(exponential_cdf(0.0, 1.0).unwrap(), 0.0, TOLERANCE);
    let cdf_at_inf = exponential_cdf(1000.0, 1.0).unwrap();
    assert!(cdf_at_inf > 0.99);
}

#[test]
fn test_binomial_distribution() {
    // Binomial(10, 0.5) at k=5 should be maximum
    let pmf_at_mode = binomial_pmf(5, 10, 0.5).unwrap();
    assert!(pmf_at_mode > binomial_pmf(4, 10, 0.5).unwrap());
    assert!(pmf_at_mode > binomial_pmf(6, 10, 0.5).unwrap());
    
    // Edge cases
    assert_close(binomial_pmf(0, 10, 0.0).unwrap(), 1.0, TOLERANCE);
    assert_close(binomial_pmf(1, 10, 0.0).unwrap(), 0.0, TOLERANCE);
    assert_close(binomial_pmf(10, 10, 1.0).unwrap(), 1.0, TOLERANCE);
    assert_close(binomial_pmf(9, 10, 1.0).unwrap(), 0.0, TOLERANCE);
}

#[test]
fn test_poisson_distribution() {
    // Poisson(1) at k=1 should be approximately 0.368
    let pmf_at_one = poisson_pmf(1, 1.0).unwrap();
    assert_close(pmf_at_one, 1.0_f64.exp().recip(), 1e-10);
    
    // Poisson(0) at k=0 should be 1
    assert_close(poisson_pmf(0, 1.0).unwrap(), 1.0_f64.exp().recip(), 1e-10);
}

// ===================== CORRELATION AND COVARIANCE TESTS =====================

#[test]
fn test_covariance() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // y = 2x
    
    let cov = covariance(&x, &y).unwrap();
    assert!(cov > 0.0); // Positive covariance for positive relationship
    
    let sample_cov = sample_covariance(&x, &y).unwrap();
    assert!(sample_cov > cov); // Sample covariance should be larger
}

#[test]
fn test_correlation() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // Perfect positive correlation
    
    let corr = correlation(&x, &y).unwrap();
    assert_close(corr, 1.0, 1e-10); // Perfect positive correlation
    
    let y_neg = vec![-2.0, -4.0, -6.0, -8.0, -10.0]; // Perfect negative correlation
    let corr_neg = correlation(&x, &y_neg).unwrap();
    assert_close(corr_neg, -1.0, 1e-10);
}

#[test]
fn test_spearman_correlation() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![1.0, 4.0, 9.0, 16.0, 25.0]; // y = x²
    
    // Spearman should be 1.0 for monotonic relationship
    let spearman = spearman_correlation(&x, &y).unwrap();
    assert_close(spearman, 1.0, 1e-10);
    
    // Pearson should be less than 1.0 for non-linear relationship
    let pearson = correlation(&x, &y).unwrap();
    assert!(pearson < 1.0);
    assert!(pearson > 0.9); // But still strong positive
}

// ===================== STATISTICAL TESTS =====================

#[test]
fn test_t_test_one_sample() {
    let data = vec![10.0, 12.0, 8.0, 11.0, 9.0, 13.0, 7.0, 14.0];
    let population_mean = 10.0;
    
    let (t_stat, df) = t_test_one_sample(&data, population_mean).unwrap();
    assert_eq!(df, 7.0); // n - 1
    assert!(t_stat.abs() < 10.0); // Reasonable t-statistic
}

#[test]
fn test_t_test_two_sample() {
    let group1 = vec![10.0, 12.0, 8.0, 11.0, 9.0];
    let group2 = vec![15.0, 17.0, 13.0, 16.0, 14.0];
    
    let (t_stat, df) = t_test_two_sample(&group1, &group2).unwrap();
    assert_eq!(df, 8.0); // n1 + n2 - 2
    assert!(t_stat < 0.0); // group1 mean < group2 mean, so negative t-stat
}

#[test]
fn test_chi_square_test() {
    let observed = vec![10.0, 15.0, 12.0, 8.0];
    let expected = vec![11.25, 11.25, 11.25, 11.25]; // Equal frequencies
    
    let (chi_sq, df) = chi_square_test(&observed, &expected).unwrap();
    assert_eq!(df, 3.0); // categories - 1
    assert!(chi_sq >= 0.0); // Chi-square is always non-negative
}

#[test]
fn test_anova_one_way() {
    let group1 = vec![10.0, 12.0, 8.0, 11.0];
    let group2 = vec![15.0, 17.0, 13.0, 16.0];
    let group3 = vec![20.0, 22.0, 18.0, 21.0];
    
    let groups = vec![&group1[..], &group2[..], &group3[..]];
    let (f_stat, between_df, within_df) = anova_one_way(&groups).unwrap();
    
    assert_eq!(between_df, 2.0); // groups - 1
    assert_eq!(within_df, 9.0); // total observations - groups
    assert!(f_stat > 0.0); // F-statistic should be positive
    assert!(f_stat > 10.0); // Should be large for clearly different groups
}

// ===================== REGRESSION ANALYSIS TESTS =====================

#[test]
fn test_linear_regression() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // y = 2x
    
    let (intercept, slope, r_squared) = linear_regression(&x, &y).unwrap();
    
    assert_close(intercept, 0.0, 1e-10); // Should be exactly 0
    assert_close(slope, 2.0, 1e-10); // Should be exactly 2
    assert_close(r_squared, 1.0, 1e-10); // Perfect fit
}

#[test]
fn test_linear_regression_with_noise() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.1, 3.9, 6.1, 7.9, 10.1]; // y ≈ 2x with small noise
    
    let (intercept, slope, r_squared) = linear_regression(&x, &y).unwrap();
    
    assert_close(intercept, 0.0, 0.5); // Close to 0
    assert_close(slope, 2.0, 0.5); // Close to 2
    assert!(r_squared > 0.95); // Very good fit
}

#[test]
fn test_multiple_linear_regression() {
    // Simple case: y = 1 + 2*x1 + 3*x2
    let x1 = vec![1.0, 2.0, 3.0, 4.0];
    let x2 = vec![2.0, 3.0, 4.0, 5.0];
    let y = vec![9.0, 16.0, 23.0, 30.0]; // 1 + 2*x1 + 3*x2
    
    let x_matrix = vec![&x1[..], &x2[..]];
    let coefficients = multiple_linear_regression(&x_matrix, &y).unwrap();
    
    assert_eq!(coefficients.len(), 3); // intercept + 2 predictors
    assert_close(coefficients[0], 1.0, 1e-10); // intercept
    assert_close(coefficients[1], 2.0, 1e-10); // coefficient for x1
    assert_close(coefficients[2], 3.0, 1e-10); // coefficient for x2
}

// ===================== OUTLIER DETECTION TESTS =====================

#[test]
fn test_outlier_detection_iqr() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100 is an outlier
    let outliers = outliers_iqr(&data, 1.5).unwrap();
    
    assert!(outliers.contains(&100.0));
    assert!(!outliers.contains(&3.0));
}

#[test]
fn test_outlier_detection_z_score() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100 is an outlier
    let outliers = outliers_z_score(&data, 2.0).unwrap();
    
    assert!(outliers.contains(&100.0));
    assert!(!outliers.contains(&3.0));
}

// ===================== DATA VALIDATION TESTS =====================

#[test]
fn test_data_validation() {
    let valid_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(!has_invalid_values(&valid_data));
    
    let invalid_data = vec![1.0, 2.0, f64::NAN, 4.0, 5.0];
    assert!(has_invalid_values(&invalid_data));
    
    let cleaned = clean_data(&invalid_data);
    assert_eq!(cleaned, vec![1.0, 2.0, 4.0, 5.0]);
    assert!(!has_invalid_values(&cleaned));
}

// ===================== ADVANCED STATISTICAL MEASURES TESTS =====================

#[test]
fn test_harmonic_mean() {
    let data = vec![1.0, 2.0, 4.0];
    let h_mean = harmonic_mean(&data).unwrap();
    // Harmonic mean of 1, 2, 4 = 3 / (1/1 + 1/2 + 1/4) = 3 / 1.75 ≈ 1.714
    assert_close(h_mean, 12.0 / 7.0, 1e-10);
}

#[test]
fn test_geometric_mean() {
    let data = vec![1.0, 2.0, 4.0];
    let g_mean = geometric_mean(&data).unwrap();
    // Geometric mean = (1 * 2 * 4)^(1/3) = 8^(1/3) = 2
    assert_close(g_mean, 2.0, 1e-10);
}

#[test]
fn test_root_mean_square() {
    let data = vec![1.0, 2.0, 3.0];
    let rms = root_mean_square(&data).unwrap();
    // RMS = sqrt((1² + 2² + 3²) / 3) = sqrt(14/3)
    assert_close(rms, (14.0 / 3.0).sqrt(), 1e-10);
}

#[test]
fn test_coefficient_of_variation() {
    let data = vec![10.0, 12.0, 8.0, 11.0, 9.0];
    let cv = coefficient_of_variation(&data).unwrap();
    
    let mean_val = mean(&data).unwrap();
    let std_val = standard_deviation(&data).unwrap();
    let expected_cv = std_val / mean_val;
    
    assert_close(cv, expected_cv, 1e-10);
}

// ===================== ERROR HANDLING TESTS =====================

#[test]
fn test_error_handling() {
    let empty: Vec<f64> = vec![];
    
    // Empty dataset errors
    assert!(mean(&empty).is_err());
    assert!(median(&empty).is_err());
    assert!(variance(&empty).is_err());
    
    // Invalid inputs
    let data_with_nan = vec![1.0, f64::NAN, 3.0];
    assert!(mean(&data_with_nan).is_err());
    
    // Domain errors for distributions
    assert!(normal_pdf(0.0, 0.0, -1.0).is_err()); // Negative std dev
    assert!(exponential_pdf(0.0, -1.0).is_err()); // Negative lambda
    assert!(binomial_pmf(5, 10, 1.5).is_err()); // p > 1
    
    // Insufficient data for statistical tests
    let small_data = vec![1.0];
    assert!(t_test_one_sample(&small_data, 0.0).is_err());
    assert!(sample_variance(&small_data).is_err());
}

#[test]
fn test_edge_cases() {
    // Single value datasets
    let single = vec![42.0];
    assert_close(mean(&single).unwrap(), 42.0, TOLERANCE);
    assert_close(median(&single).unwrap(), 42.0, TOLERANCE);
    assert_close(variance(&single).unwrap(), 0.0, TOLERANCE);
    
    // Identical values
    let identical = vec![5.0, 5.0, 5.0, 5.0];
    assert_close(mean(&identical).unwrap(), 5.0, TOLERANCE);
    assert_close(variance(&identical).unwrap(), 0.0, TOLERANCE);
    assert_close(standard_deviation(&identical).unwrap(), 0.0, TOLERANCE);
    
    // Large datasets (performance test)
    let large_data: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
    let large_mean = mean(&large_data).unwrap();
    assert_close(large_mean, 500.5, 1e-6);
}
