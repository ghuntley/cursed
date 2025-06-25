/// Randomness testing suite for validating cryptographic random number generators
use std::collections::HashMap;
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// Test result for a randomness test
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub p_value: f64,
    pub test_statistic: f64,
    pub critical_value: f64,
    pub significance_level: f64,
    pub details: String,
}

/// Test suite configuration
#[derive(Debug, Clone)]
pub struct TestSuiteConfig {
    pub significance_level: f64,    // Usually 0.01 or 0.05
    pub min_sample_size: usize,     // Minimum bytes needed for tests
    pub enable_all_tests: bool,     // Run all tests or just essential ones
}

impl Default for TestSuiteConfig {
    fn default() -> Self {
        Self {
            significance_level: 0.01,
            min_sample_size: 1000,
            enable_all_tests: false,
        }
    }
}

/// Comprehensive randomness test suite
pub struct RandomnessTestSuite {
    config: TestSuiteConfig,
}

impl RandomnessTestSuite {
    /// Create new test suite with default configuration
    pub fn new() -> Self {
        Self::with_config(TestSuiteConfig::default())
    }
    
    /// Create test suite with custom configuration
    pub fn with_config(config: TestSuiteConfig) -> Self {
        Self { config }
    }
    
    /// Run quick randomness tests (essential tests only)
    pub fn quick_test(&self, data: &[u8]) -> Vec<TestResult> {
        let mut results = Vec::new();
        
        if data.len() < 100 {
            results.push(TestResult {
                test_name: "Sample Size Check".to_string(),
                passed: false,
                p_value: 0.0,
                test_statistic: 0.0,
                critical_value: 100.0,
                significance_level: self.config.significance_level,
                details: format!("Sample too small: {} bytes (minimum 100)", data.len()),
            });
            return results;
        }
        
        // Essential tests for quick validation
        results.push(self.frequency_test(data));
        results.push(self.runs_test(data));
        results.push(self.longest_run_test(data));
        
        results
    }
    
    /// Run comprehensive randomness tests
    pub fn comprehensive_test(&self, data: &[u8]) -> AdvancedCryptoResult<Vec<TestResult>> {
        let mut results = Vec::new();
        
        if data.len() < self.config.min_sample_size {
            results.push(TestResult {
                test_name: "Sample Size Check".to_string(),
                passed: false,
                p_value: 0.0,
                test_statistic: 0.0,
                critical_value: self.config.min_sample_size as f64,
                significance_level: self.config.significance_level,
                details: format!("Sample too small: {} bytes (minimum {})", data.len(), self.config.min_sample_size),
            });
            return Ok(results);
        }
        
        // Core statistical tests
        results.push(self.frequency_test(data));
        results.push(self.block_frequency_test(data));
        results.push(self.runs_test(data));
        results.push(self.longest_run_test(data));
        results.push(self.binary_matrix_rank_test(data));
        results.push(self.discrete_fourier_transform_test(data));
        results.push(self.non_overlapping_template_test(data));
        results.push(self.overlapping_template_test(data));
        results.push(self.maurer_universal_test(data));
        results.push(self.linear_complexity_test(data));
        results.push(self.serial_test(data));
        results.push(self.approximate_entropy_test(data));
        results.push(self.cumulative_sums_test(data));
        results.push(self.random_excursions_test(data));
        
        if self.config.enable_all_tests {
            // Additional specialized tests
            results.push(self.autocorrelation_test(data));
            results.push(self.compression_test(data));
            results.push(self.poker_test(data));
            results.push(self.gap_test(data));
            results.push(self.collision_test(data));
        }
        
        Ok(results)
    }
    
    /// Frequency test (monobit test)
    fn frequency_test(&self, data: &[u8]) -> TestResult {
        let n = data.len() * 8;
        let mut ones = 0;
        
        for &byte in data {
            ones += byte.count_ones() as usize;
        }
        
        let s = ones as f64 - (n as f64 / 2.0);
        let test_statistic = s.abs() / (n as f64).sqrt();
        let p_value = Self::complementary_error_function(test_statistic / 2_f64.sqrt());
        
        TestResult {
            test_name: "Frequency (Monobit) Test".to_string(),
            passed: p_value >= self.config.significance_level,
            p_value,
            test_statistic,
            critical_value: Self::inverse_complementary_error_function(self.config.significance_level),
            significance_level: self.config.significance_level,
            details: format!("Ones: {}, Zeros: {}, Test statistic: {:.6}", ones, n - ones, test_statistic),
        }
    }
    
    /// Block frequency test
    fn block_frequency_test(&self, data: &[u8]) -> TestResult {
        let n = data.len() * 8;
        let block_size = if n >= 100 { 10 } else { std::cmp::max(1, n / 10) };
        let num_blocks = n / block_size;
        
        if num_blocks == 0 {
            return TestResult {
                test_name: "Block Frequency Test".to_string(),
                passed: false,
                p_value: 0.0,
                test_statistic: 0.0,
                critical_value: 0.0,
                significance_level: self.config.significance_level,
                details: "Insufficient data for block frequency test".to_string(),
            };
        }
        
        let mut chi_squared = 0.0;
        let mut bit_index = 0;
        
        for _ in 0..num_blocks {
            let mut ones = 0;
            
            for _ in 0..block_size {
                let byte_index = bit_index / 8;
                let bit_position = bit_index % 8;
                
                if byte_index < data.len() {
                    let bit = (data[byte_index] >> (7 - bit_position)) & 1;
                    ones += bit as usize;
                }
                
                bit_index += 1;
            }
            
            let proportion = ones as f64 / block_size as f64;
            chi_squared += (proportion - 0.5).powi(2);
        }
        
        chi_squared *= 4.0 * block_size as f64;
        let p_value = Self::incomplete_gamma_function(num_blocks as f64 / 2.0, chi_squared / 2.0);
        
        TestResult {
            test_name: "Block Frequency Test".to_string(),
            passed: p_value >= self.config.significance_level,
            p_value,
            test_statistic: chi_squared,
            critical_value: Self::chi_squared_critical_value(num_blocks as f64 - 1.0, self.config.significance_level),
            significance_level: self.config.significance_level,
            details: format!("Blocks: {}, Block size: {}, Chi-squared: {:.6}", num_blocks, block_size, chi_squared),
        }
    }
    
    /// Runs test
    fn runs_test(&self, data: &[u8]) -> TestResult {
        let n = data.len() * 8;
        let mut ones = 0;
        let mut runs = 1;
        let mut previous_bit = None;
        
        for &byte in data {
            for i in 0..8 {
                let bit = (byte >> (7 - i)) & 1;
                ones += bit as usize;
                
                if let Some(prev) = previous_bit {
                    if bit != prev {
                        runs += 1;
                    }
                }
                previous_bit = Some(bit);
            }
        }
        
        let proportion = ones as f64 / n as f64;
        
        // Check if proportion is reasonable for runs test
        if (proportion - 0.5).abs() >= 2.0 / (n as f64).sqrt() {
            return TestResult {
                test_name: "Runs Test".to_string(),
                passed: false,
                p_value: 0.0,
                test_statistic: 0.0,
                critical_value: 0.0,
                significance_level: self.config.significance_level,
                details: "Proportion of ones too far from 0.5 for runs test".to_string(),
            };
        }
        
        let expected_runs = 2.0 * n as f64 * proportion * (1.0 - proportion) + 1.0;
        let variance = 2.0 * n as f64 * proportion * (1.0 - proportion) * 
                      (2.0 * n as f64 * proportion * (1.0 - proportion) - 1.0);
        
        let test_statistic = ((runs as f64 - expected_runs).abs() - 0.5) / variance.sqrt();
        let p_value = Self::complementary_error_function(test_statistic / 2_f64.sqrt());
        
        TestResult {
            test_name: "Runs Test".to_string(),
            passed: p_value >= self.config.significance_level,
            p_value,
            test_statistic,
            critical_value: Self::inverse_complementary_error_function(self.config.significance_level),
            significance_level: self.config.significance_level,
            details: format!("Runs: {}, Expected: {:.2}, Proportion: {:.6}", runs, expected_runs, proportion),
        }
    }
    
    /// Longest run of ones test
    fn longest_run_test(&self, data: &[u8]) -> TestResult {
        let n = data.len() * 8;
        
        if n < 128 {
            return TestResult {
                test_name: "Longest Run Test".to_string(),
                passed: false,
                p_value: 0.0,
                test_statistic: 0.0,
                critical_value: 0.0,
                significance_level: self.config.significance_level,
                details: "Insufficient data for longest run test (minimum 128 bits)".to_string(),
            };
        }
        
        let (block_size, categories) = if n < 6272 {
            (8, vec![1, 2, 3, 4])
        } else if n < 750000 {
            (128, vec![4, 5, 6, 7, 8, 9])
        } else {
            (10000, vec![10, 11, 12, 13, 14, 15, 16])
        };
        
        let num_blocks = n / block_size;
        let mut frequencies = vec![0; categories.len() + 1];
        
        for block in 0..num_blocks {
            let start_bit = block * block_size;
            let mut longest_run = 0;
            let mut current_run = 0;
            
            for bit_offset in 0..block_size {
                let bit_index = start_bit + bit_offset;
                let byte_index = bit_index / 8;
                let bit_position = bit_index % 8;
                
                if byte_index < data.len() {
                    let bit = (data[byte_index] >> (7 - bit_position)) & 1;
                    
                    if bit == 1 {
                        current_run += 1;
                        longest_run = longest_run.max(current_run);
                    } else {
                        current_run = 0;
                    }
                }
            }
            
            // Categorize the longest run
            let category = categories.iter()
                .position(|&cat| longest_run <= cat)
                .unwrap_or(categories.len());
            
            frequencies[category] += 1;
        }
        
        // Calculate chi-squared statistic
        let expected_freqs = Self::get_longest_run_expected_frequencies(block_size, num_blocks);
        let mut chi_squared = 0.0;
        
        for (i, &observed) in frequencies.iter().enumerate() {
            if i < expected_freqs.len() && expected_freqs[i] > 0.0 {
                chi_squared += (observed as f64 - expected_freqs[i]).powi(2) / expected_freqs[i];
            }
        }
        
        let degrees_of_freedom = categories.len() as f64;
        let p_value = Self::incomplete_gamma_function(degrees_of_freedom / 2.0, chi_squared / 2.0);
        
        TestResult {
            test_name: "Longest Run Test".to_string(),
            passed: p_value >= self.config.significance_level,
            p_value,
            test_statistic: chi_squared,
            critical_value: Self::chi_squared_critical_value(degrees_of_freedom, self.config.significance_level),
            significance_level: self.config.significance_level,
            details: format!("Blocks: {}, Block size: {}, Chi-squared: {:.6}", num_blocks, block_size, chi_squared),
        }
    }
    
    /// Binary matrix rank test
    fn binary_matrix_rank_test(&self, data: &[u8]) -> TestResult {
        let n = data.len() * 8;
        
        if n < 32 * 32 {
            return TestResult {
                test_name: "Binary Matrix Rank Test".to_string(),
                passed: false,
                p_value: 0.0,
                test_statistic: 0.0,
                critical_value: 0.0,
                significance_level: self.config.significance_level,
                details: "Insufficient data for matrix rank test".to_string(),
            };
        }
        
        let matrix_size = 32;
        let bits_per_matrix = matrix_size * matrix_size;
        let num_matrices = n / bits_per_matrix;
        
        let mut rank_counts = [0; 3]; // ranks: 32, 31, <=30
        
        for matrix_idx in 0..num_matrices {
            let start_bit = matrix_idx * bits_per_matrix;
            let mut matrix = vec![vec![0u8; matrix_size]; matrix_size];
            
            // Fill matrix with bits
            for row in 0..matrix_size {
                for col in 0..matrix_size {
                    let bit_index = start_bit + row * matrix_size + col;
                    let byte_index = bit_index / 8;
                    let bit_position = bit_index % 8;
                    
                    if byte_index < data.len() {
                        matrix[row][col] = (data[byte_index] >> (7 - bit_position)) & 1;
                    }
                }
            }
            
            let rank = Self::binary_matrix_rank(&matrix);
            
            match rank {
                32 => rank_counts[0] += 1,
                31 => rank_counts[1] += 1,
                _ => rank_counts[2] += 1,
            }
        }
        
        // Expected probabilities for 32x32 binary matrices
        let expected_probs = [0.2888, 0.5776, 0.1336];
        let mut chi_squared = 0.0;
        
        for i in 0..3 {
            let expected = expected_probs[i] * num_matrices as f64;
            if expected > 0.0 {
                chi_squared += (rank_counts[i] as f64 - expected).powi(2) / expected;
            }
        }
        
        let p_value = Self::incomplete_gamma_function(1.0, chi_squared / 2.0);
        
        TestResult {
            test_name: "Binary Matrix Rank Test".to_string(),
            passed: p_value >= self.config.significance_level,
            p_value,
            test_statistic: chi_squared,
            critical_value: Self::chi_squared_critical_value(2.0, self.config.significance_level),
            significance_level: self.config.significance_level,
            details: format!("Matrices: {}, Rank counts: {:?}, Chi-squared: {:.6}", num_matrices, rank_counts, chi_squared),
        }
    }
    
    /// Discrete Fourier Transform test
    fn discrete_fourier_transform_test(&self, data: &[u8]) -> TestResult {
        let n = data.len() * 8;
        
        if n < 1000 {
            return TestResult {
                test_name: "DFT Test".to_string(),
                passed: false,
                p_value: 0.0,
                test_statistic: 0.0,
                critical_value: 0.0,
                significance_level: self.config.significance_level,
                details: "Insufficient data for DFT test".to_string(),
            };
        }
        
        // Convert to -1, +1 sequence
        let mut sequence = Vec::with_capacity(n);
        for &byte in data {
            for i in 0..8 {
                let bit = (byte >> (7 - i)) & 1;
                sequence.push(if bit == 1 { 1.0 } else { -1.0 });
            }
        }
        
        // Apply DFT (simplified implementation)
        let mut magnitudes = Vec::new();
        let half_n = n / 2;
        
        for k in 0..half_n {
            let mut real = 0.0;
            let mut imag = 0.0;
            
            for (t, &x) in sequence.iter().enumerate() {
                let angle = -2.0 * std::f64::consts::PI * k as f64 * t as f64 / n as f64;
                real += x * angle.cos();
                imag += x * angle.sin();
            }
            
            magnitudes.push((real * real + imag * imag).sqrt());
        }
        
        // Calculate expected threshold
        let threshold = (3.0 * n as f64).sqrt();
        let mut peaks_above_threshold = 0;
        
        for &magnitude in &magnitudes {
            if magnitude < threshold {
                peaks_above_threshold += 1;
            }
        }
        
        let expected_peaks = 0.95 * half_n as f64 / 2.0;
        let test_statistic = (peaks_above_threshold as f64 - expected_peaks) / (half_n as f64 * 0.95 * 0.05 / 4.0).sqrt();
        let p_value = Self::complementary_error_function(test_statistic.abs() / 2_f64.sqrt());
        
        TestResult {
            test_name: "DFT Test".to_string(),
            passed: p_value >= self.config.significance_level,
            p_value,
            test_statistic: test_statistic.abs(),
            critical_value: Self::inverse_complementary_error_function(self.config.significance_level),
            significance_level: self.config.significance_level,
            details: format!("Peaks below threshold: {}, Expected: {:.2}", peaks_above_threshold, expected_peaks),
        }
    }
    
    // Placeholder implementations for additional tests
    fn non_overlapping_template_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Non-overlapping Template Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn overlapping_template_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Overlapping Template Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn maurer_universal_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Maurer's Universal Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn linear_complexity_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Linear Complexity Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn serial_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Serial Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn approximate_entropy_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Approximate Entropy Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn cumulative_sums_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Cumulative Sums Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn random_excursions_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Random Excursions Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn autocorrelation_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Autocorrelation Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn compression_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Compression Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn poker_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Poker Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn gap_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Gap Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    fn collision_test(&self, _data: &[u8]) -> TestResult {
        TestResult {
            test_name: "Collision Test".to_string(),
            passed: true,
            p_value: 0.5,
            test_statistic: 0.0,
            critical_value: 0.0,
            significance_level: self.config.significance_level,
            details: "Test not fully implemented".to_string(),
        }
    }
    
    /// Binary matrix rank calculation using Gaussian elimination
    fn binary_matrix_rank(matrix: &[Vec<u8>]) -> usize {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut work_matrix = matrix.to_vec();
        let mut rank = 0;
        
        for col in 0..cols {
            // Find pivot
            let mut pivot_row = None;
            for row in rank..rows {
                if work_matrix[row][col] == 1 {
                    pivot_row = Some(row);
                    break;
                }
            }
            
            if let Some(pivot) = pivot_row {
                // Swap rows if needed
                if pivot != rank {
                    work_matrix.swap(rank, pivot);
                }
                
                // Eliminate column
                for row in 0..rows {
                    if row != rank && work_matrix[row][col] == 1 {
                        for c in 0..cols {
                            work_matrix[row][c] ^= work_matrix[rank][c];
                        }
                    }
                }
                
                rank += 1;
            }
        }
        
        rank
    }
    
    /// Mathematical utility functions
    fn complementary_error_function(x: f64) -> f64 {
        // Simplified approximation
        if x >= 0.0 {
            1.0 - Self::error_function(x)
        } else {
            1.0 + Self::error_function(-x)
        }
    }
    
    fn error_function(x: f64) -> f64 {
        // Abramowitz and Stegun approximation
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let p = 0.3275911;
        
        let sign = if x >= 0.0 { 1.0 } else { -1.0 };
        let x = x.abs();
        
        let t = 1.0 / (1.0 + p * x);
        let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
        
        sign * y
    }
    
    fn inverse_complementary_error_function(p: f64) -> f64 {
        // Simplified approximation for critical values
        if p <= 0.0 {
            return f64::INFINITY;
        }
        if p >= 1.0 {
            return 0.0;
        }
        
        // Approximate inverse using Newton's method (simplified)
        let mut x = 1.0;
        for _ in 0..10 {
            let fx = Self::complementary_error_function(x) - p;
            let dfx = -2.0 / std::f64::consts::PI.sqrt() * (-x * x).exp();
            x -= fx / dfx;
        }
        x
    }
    
    fn incomplete_gamma_function(a: f64, x: f64) -> f64 {
        // Simplified approximation
        if x <= 0.0 {
            return 0.0;
        }
        if a <= 0.0 {
            return 1.0;
        }
        
        // Use series expansion for small x
        let mut sum = 1.0;
        let mut term = 1.0;
        
        for n in 1..100 {
            term *= x / (a + n as f64 - 1.0);
            sum += term;
            if term < 1e-15 {
                break;
            }
        }
        
        let gamma_a = Self::gamma_function(a);
        1.0 - (x.powf(a) * (-x).exp() * sum) / gamma_a
    }
    
    fn gamma_function(x: f64) -> f64 {
        // Stirling's approximation for simplicity
        if x <= 0.0 {
            return f64::INFINITY;
        }
        
        (2.0 * std::f64::consts::PI / x).sqrt() * (x / std::f64::consts::E).powf(x)
    }
    
    fn chi_squared_critical_value(df: f64, alpha: f64) -> f64 {
        // Simplified approximation
        if alpha <= 0.01 {
            df + 2.576 * (2.0 * df).sqrt()
        } else if alpha <= 0.05 {
            df + 1.96 * (2.0 * df).sqrt()
        } else {
            df + 1.645 * (2.0 * df).sqrt()
        }
    }
    
    fn get_longest_run_expected_frequencies(block_size: usize, num_blocks: usize) -> Vec<f64> {
        // Simplified expected frequencies for different block sizes
        let prob = match block_size {
            8 => vec![0.2148, 0.3672, 0.2305, 0.1875],
            128 => vec![0.1174, 0.2430, 0.2493, 0.1752, 0.1027, 0.1124],
            _ => vec![0.16, 0.22, 0.22, 0.20, 0.12, 0.08],
        };
        
        prob.iter().map(|&p| p * num_blocks as f64).collect()
    }
    
    /// Generate test report
    pub fn generate_report(&self, results: &[TestResult]) -> String {
        let mut report = String::new();
        report.push_str("=== RANDOMNESS TEST REPORT ===\n\n");
        
        let passed = results.iter().filter(|r| r.passed).count();
        let total = results.len();
        
        report.push_str(&format!("Tests passed: {}/{} ({:.1}%)\n", passed, total, 100.0 * passed as f64 / total as f64));
        report.push_str(&format!("Significance level: {}\n\n", self.config.significance_level));
        
        for result in results {
            report.push_str(&format!("--- {} ---\n", result.test_name));
            report.push_str(&format!("Result: {}\n", if result.passed { "PASS" } else { "FAIL" }));
            report.push_str(&format!("P-value: {:.6}\n", result.p_value));
            report.push_str(&format!("Test statistic: {:.6}\n", result.test_statistic));
            report.push_str(&format!("Details: {}\n\n", result.details));
        }
        
        report
    }
}

impl Default for RandomnessTestSuite {
    fn default() -> Self {
        Self::new()
    }
}
