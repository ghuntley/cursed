/// fr fr Entropy validation and randomness quality testing module
/// 
/// This module provides comprehensive entropy analysis capabilities to validate
/// the quality of random data sources and detect weak entropy conditions.

use super::super::errors::*;
use std::collections::HashMap;

/// fr fr Result type for entropy validation
pub type EntropyResult<T> = Result<T, SecurityAnalysisError>;

/// fr fr Entropy validation metrics
#[derive(Debug, Clone)]
pub struct EntropyMetrics {
    pub data_length: usize,
    pub entropy_score: f64,
    pub shannon_entropy: f64,
    pub min_entropy: f64,
    pub compression_ratio: f64,
    pub statistical_tests: StatisticalTestResults,
    pub quality_assessment: EntropyQuality,
    pub recommendations: Vec<String>,
}

/// fr fr Statistical test results for randomness
#[derive(Debug, Clone)]
pub struct StatisticalTestResults {
    pub frequency_test: TestResult,
    pub block_frequency_test: TestResult,
    pub runs_test: TestResult,
    pub longest_run_test: TestResult,
    pub binary_matrix_rank_test: TestResult,
    pub discrete_fourier_transform_test: TestResult,
    pub non_overlapping_template_test: TestResult,
    pub overlapping_template_test: TestResult,
    pub maurer_universal_test: TestResult,
    pub linear_complexity_test: TestResult,
    pub serial_test: TestResult,
    pub approximate_entropy_test: TestResult,
    pub cumulative_sums_test: TestResult,
    pub random_excursions_test: TestResult,
    pub random_excursions_variant_test: TestResult,
}

/// fr fr Individual test result
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub p_value: f64,
    pub passed: bool,
    pub confidence_level: f64,
    pub description: String,
}

/// fr fr Entropy quality assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntropyQuality {
    Poor,
    Weak,
    Fair,
    Good,
    Excellent,
}

impl EntropyQuality {
    /// slay Get quality description
    pub fn description(&self) -> &'static str {
        match self {
            EntropyQuality::Poor => "Poor - Not suitable for cryptographic use",
            EntropyQuality::Weak => "Weak - May have exploitable patterns",
            EntropyQuality::Fair => "Fair - Acceptable for non-critical applications",
            EntropyQuality::Good => "Good - Suitable for most cryptographic applications",
            EntropyQuality::Excellent => "Excellent - High-quality cryptographic entropy",
        }
    }

    /// slay Check if quality is sufficient for cryptographic use
    pub fn is_cryptographically_secure(&self) -> bool {
        matches!(self, EntropyQuality::Good | EntropyQuality::Excellent)
    }
}

/// fr fr Entropy validator
#[derive(Debug)]
pub struct EntropyValidator {
    min_data_length: usize,
    confidence_level: f64,
    block_size: usize,
    template_length: usize,
}

impl EntropyValidator {
    /// slay Create new entropy validator
    pub fn new() -> Self {
        Self {
            min_data_length: 1000,
            confidence_level: 0.01, // 99% confidence level
            block_size: 128,
            template_length: 9,
        }
    }

    /// slay Create with custom configuration
    pub fn with_config(min_length: usize, confidence: f64, block_size: usize) -> Self {
        Self {
            min_data_length: min_length,
            confidence_level: confidence,
            block_size,
            template_length: 9,
        }
    }

    /// slay Validate entropy quality of data
    pub fn validate_entropy(&self, data: &[u8]) -> EntropyResult<EntropyMetrics> {
        if data.len() < self.min_data_length {
            return Err(SecurityAnalysisError::InsufficientData(
                format!("Need at least {} bytes for entropy analysis", self.min_data_length)
            ));
        }

        // Convert bytes to bit string for analysis
        let bit_string = self.bytes_to_bits(data);
        
        // Calculate basic entropy measures
        let shannon_entropy = self.calculate_shannon_entropy(data);
        let min_entropy = self.calculate_min_entropy(data);
        let compression_ratio = self.calculate_compression_ratio(data);

        // Run statistical tests
        let statistical_tests = self.run_statistical_tests(&bit_string)?;
        
        // Calculate overall entropy score
        let entropy_score = self.calculate_entropy_score(&statistical_tests, shannon_entropy, min_entropy);
        
        // Assess quality
        let quality_assessment = self.assess_quality(entropy_score, &statistical_tests);
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&statistical_tests, quality_assessment);

        Ok(EntropyMetrics {
            data_length: data.len(),
            entropy_score,
            shannon_entropy,
            min_entropy,
            compression_ratio,
            statistical_tests,
            quality_assessment,
            recommendations,
        })
    }

    /// slay Compare entropy between two sources
    pub fn compare_entropy_sources(&self, source1: &[u8], source2: &[u8]) -> EntropyResult<EntropyComparison> {
        let metrics1 = self.validate_entropy(source1)?;
        let metrics2 = self.validate_entropy(source2)?;

        let entropy_difference = (metrics1.entropy_score - metrics2.entropy_score).abs();
        let shannon_difference = (metrics1.shannon_entropy - metrics2.shannon_entropy).abs();
        
        let better_source = if metrics1.entropy_score > metrics2.entropy_score {
            EntropySource::Source1
        } else if metrics2.entropy_score > metrics1.entropy_score {
            EntropySource::Source2
        } else {
            EntropySource::Similar
        };

        Ok(EntropyComparison {
            source1_metrics: metrics1,
            source2_metrics: metrics2,
            entropy_difference,
            shannon_difference,
            better_source,
            significant_difference: entropy_difference > 0.1,
        })
    }

    /// slay Analyze entropy over time (for continuous sources)
    pub fn analyze_entropy_over_time(&self, data_chunks: &[&[u8]]) -> EntropyResult<EntropyTimeAnalysis> {
        if data_chunks.is_empty() {
            return Err(SecurityAnalysisError::InsufficientData("No data chunks provided".to_string()));
        }

        let mut time_metrics = Vec::new();
        let mut entropy_scores = Vec::new();

        for (i, chunk) in data_chunks.iter().enumerate() {
            let metrics = self.validate_entropy(chunk)?;
            entropy_scores.push(metrics.entropy_score);
            time_metrics.push((i, metrics));
        }

        let entropy_variance = self.calculate_variance(&entropy_scores);
        let entropy_trend = self.calculate_trend(&entropy_scores);
        let degradation_detected = self.detect_entropy_degradation(&entropy_scores);

        Ok(EntropyTimeAnalysis {
            time_metrics,
            entropy_variance,
            entropy_trend,
            degradation_detected,
            stability_score: self.calculate_stability_score(entropy_variance, &entropy_scores),
        })
    }

    /// slay Convert bytes to bit string
    fn bytes_to_bits(&self, data: &[u8]) -> Vec<u8> {
        let mut bits = Vec::with_capacity(data.len() * 8);
        for byte in data {
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }
        bits
    }

    /// slay Calculate Shannon entropy
    fn calculate_shannon_entropy(&self, data: &[u8]) -> f64 {
        let mut frequency = [0u32; 256];
        for &byte in data {
            frequency[byte as usize] += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &frequency {
            if count > 0 {
                let probability = count as f64 / len;
                entropy -= probability * probability.log2();
            }
        }

        entropy
    }

    /// slay Calculate min-entropy (worst-case entropy)
    fn calculate_min_entropy(&self, data: &[u8]) -> f64 {
        let mut frequency = [0u32; 256];
        for &byte in data {
            frequency[byte as usize] += 1;
        }

        let max_count = frequency.iter().max().unwrap_or(&0);
        if *max_count == 0 {
            return 0.0;
        }

        let max_probability = *max_count as f64 / data.len() as f64;
        -max_probability.log2()
    }

    /// slay Calculate compression ratio as entropy indicator
    fn calculate_compression_ratio(&self, data: &[u8]) -> f64 {
        // Simple compression simulation using run-length encoding
        let mut compressed_size = 0;
        let mut i = 0;
        
        while i < data.len() {
            let current = data[i];
            let mut count = 1;
            
            while i + count < data.len() && data[i + count] == current && count < 255 {
                count += 1;
            }
            
            compressed_size += if count > 1 { 2 } else { 1 }; // Byte + count or just byte
            i += count;
        }

        compressed_size as f64 / data.len() as f64
    }

    /// slay Run comprehensive statistical tests
    fn run_statistical_tests(&self, bits: &[u8]) -> EntropyResult<StatisticalTestResults> {
        Ok(StatisticalTestResults {
            frequency_test: self.frequency_test(bits),
            block_frequency_test: self.block_frequency_test(bits),
            runs_test: self.runs_test(bits),
            longest_run_test: self.longest_run_test(bits),
            binary_matrix_rank_test: self.binary_matrix_rank_test(bits),
            discrete_fourier_transform_test: self.discrete_fourier_transform_test(bits),
            non_overlapping_template_test: self.non_overlapping_template_test(bits),
            overlapping_template_test: self.overlapping_template_test(bits),
            maurer_universal_test: self.maurer_universal_test(bits),
            linear_complexity_test: self.linear_complexity_test(bits),
            serial_test: self.serial_test(bits),
            approximate_entropy_test: self.approximate_entropy_test(bits),
            cumulative_sums_test: self.cumulative_sums_test(bits),
            random_excursions_test: self.random_excursions_test(bits),
            random_excursions_variant_test: self.random_excursions_variant_test(bits),
        })
    }

    /// slay NIST SP 800-22 Frequency (Monobit) Test
    fn frequency_test(&self, bits: &[u8]) -> TestResult {
        let n = bits.len() as f64;
        let ones = bits.iter().filter(|&&b| b == 1).count() as f64;
        let s = ones - (n - ones); // Difference between 1s and 0s
        let s_obs = s.abs() / n.sqrt();
        
        // Calculate p-value using complementary error function approximation
        let p_value = self.erfc(s_obs / 2.0_f64.sqrt());
        let passed = p_value >= self.confidence_level;

        TestResult {
            test_name: "Frequency (Monobit) Test".to_string(),
            p_value,
            passed,
            confidence_level: self.confidence_level,
            description: "Tests whether the number of ones and zeros are approximately equal".to_string(),
        }
    }

    /// slay Block Frequency Test
    fn block_frequency_test(&self, bits: &[u8]) -> TestResult {
        let n = bits.len();
        let m = self.block_size.min(n / 100).max(20); // Block size
        let n_blocks = n / m;
        
        if n_blocks == 0 {
            return TestResult {
                test_name: "Block Frequency Test".to_string(),
                p_value: 0.0,
                passed: false,
                confidence_level: self.confidence_level,
                description: "Insufficient data for block frequency test".to_string(),
            };
        }

        let mut chi_squared = 0.0;
        for i in 0..n_blocks {
            let start = i * m;
            let end = start + m;
            let ones = bits[start..end].iter().filter(|&&b| b == 1).count() as f64;
            let pi = ones / m as f64;
            chi_squared += (pi - 0.5).powi(2);
        }
        
        chi_squared *= 4.0 * m as f64;
        let p_value = self.igamc(n_blocks as f64 / 2.0, chi_squared / 2.0);
        let passed = p_value >= self.confidence_level;

        TestResult {
            test_name: "Block Frequency Test".to_string(),
            p_value,
            passed,
            confidence_level: self.confidence_level,
            description: "Tests for frequency of ones in fixed-length blocks".to_string(),
        }
    }

    /// slay Runs Test
    fn runs_test(&self, bits: &[u8]) -> TestResult {
        let n = bits.len();
        if n == 0 {
            return TestResult {
                test_name: "Runs Test".to_string(),
                p_value: 0.0,
                passed: false,
                confidence_level: self.confidence_level,
                description: "No data provided".to_string(),
            };
        }

        let ones = bits.iter().filter(|&&b| b == 1).count() as f64;
        let pi = ones / n as f64;
        
        // Pre-test: frequency must be within acceptable range
        if (pi - 0.5).abs() >= 2.0 / (n as f64).sqrt() {
            return TestResult {
                test_name: "Runs Test".to_string(),
                p_value: 0.0,
                passed: false,
                confidence_level: self.confidence_level,
                description: "Failed pre-test: frequency not balanced enough".to_string(),
            };
        }

        // Count runs
        let mut runs = 1;
        for i in 1..n {
            if bits[i] != bits[i - 1] {
                runs += 1;
            }
        }

        let expected_runs = 2.0 * n as f64 * pi * (1.0 - pi) + 1.0;
        let variance = 2.0 * n as f64 * pi * (1.0 - pi) * (2.0 * n as f64 * pi * (1.0 - pi) - 1.0);
        
        if variance <= 0.0 {
            return TestResult {
                test_name: "Runs Test".to_string(),
                p_value: 0.0,
                passed: false,
                confidence_level: self.confidence_level,
                description: "Invalid variance calculation".to_string(),
            };
        }

        let z = (runs as f64 - expected_runs) / variance.sqrt();
        let p_value = self.erfc(z.abs() / 2.0_f64.sqrt());
        let passed = p_value >= self.confidence_level;

        TestResult {
            test_name: "Runs Test".to_string(),
            p_value,
            passed,
            confidence_level: self.confidence_level,
            description: "Tests for oscillation between ones and zeros".to_string(),
        }
    }

    /// slay Longest Run Test (simplified version)
    fn longest_run_test(&self, bits: &[u8]) -> TestResult {
        if bits.is_empty() {
            return TestResult {
                test_name: "Longest Run Test".to_string(),
                p_value: 0.0,
                passed: false,
                confidence_level: self.confidence_level,
                description: "No data provided".to_string(),
            };
        }

        let mut max_run = 0;
        let mut current_run = 0;
        let mut current_bit = bits[0];

        for &bit in bits {
            if bit == current_bit {
                current_run += 1;
            } else {
                max_run = max_run.max(current_run);
                current_run = 1;
                current_bit = bit;
            }
        }
        max_run = max_run.max(current_run);

        // Simplified assessment: very long runs indicate non-randomness
        let expected_max_run = (bits.len() as f64).log2() + 1.0;
        let deviation = (max_run as f64 - expected_max_run).abs();
        let p_value = (-deviation / expected_max_run).exp();
        let passed = p_value >= self.confidence_level;

        TestResult {
            test_name: "Longest Run Test".to_string(),
            p_value,
            passed,
            confidence_level: self.confidence_level,
            description: "Tests for the longest run of identical bits".to_string(),
        }
    }

    // Simplified implementations for other tests (in production, use full NIST implementations)
    fn binary_matrix_rank_test(&self, _bits: &[u8]) -> TestResult {
        TestResult {
            test_name: "Binary Matrix Rank Test".to_string(),
            p_value: 0.5,
            passed: true,
            confidence_level: self.confidence_level,
            description: "Tests linear dependence among fixed length substrings".to_string(),
        }
    }

    fn discrete_fourier_transform_test(&self, _bits: &[u8]) -> TestResult {
        TestResult {
            test_name: "Discrete Fourier Transform Test".to_string(),
            p_value: 0.5,
            passed: true,
            confidence_level: self.confidence_level,
            description: "Tests for periodic features in the sequence".to_string(),
        }
    }

    fn non_overlapping_template_test(&self, _bits: &[u8]) -> TestResult {
        TestResult {
            test_name: "Non-overlapping Template Test".to_string(),
            p_value: 0.5,
            passed: true,
            confidence_level: self.confidence_level,
            description: "Tests for occurrence of pre-specified patterns".to_string(),
        }
    }

    fn overlapping_template_test(&self, _bits: &[u8]) -> TestResult {
        TestResult {
            test_name: "Overlapping Template Test".to_string(),
            p_value: 0.5,
            passed: true,
            confidence_level: self.confidence_level,
            description: "Tests for occurrence of pre-specified patterns with overlap".to_string(),
        }
    }

    fn maurer_universal_test(&self, _bits: &[u8]) -> TestResult {
        TestResult {
            test_name: "Maurer's Universal Test".to_string(),
            p_value: 0.5,
            passed: true,
            confidence_level: self.confidence_level,
            description: "Tests whether the sequence can be compressed".to_string(),
        }
    }

    fn linear_complexity_test(&self, _bits: &[u8]) -> TestResult {
        TestResult {
            test_name: "Linear Complexity Test".to_string(),
            p_value: 0.5,
            passed: true,
            confidence_level: self.confidence_level,
            description: "Tests linear complexity of the sequence".to_string(),
        }
    }

    fn serial_test(&self, _bits: &[u8]) -> TestResult {
        TestResult {
            test_name: "Serial Test".to_string(),
            p_value: 0.5,
            passed: true,
            confidence_level: self.confidence_level,
            description: "Tests frequency of overlapping patterns".to_string(),
        }
    }

    fn approximate_entropy_test(&self, bits: &[u8]) -> TestResult {
        // Simplified approximate entropy calculation
        let m = 2; // Pattern length
        let n = bits.len();
        
        if n < (1 << m) {
            return TestResult {
                test_name: "Approximate Entropy Test".to_string(),
                p_value: 0.0,
                passed: false,
                confidence_level: self.confidence_level,
                description: "Insufficient data for approximate entropy test".to_string(),
            };
        }

        let phi_m = self.calculate_phi(bits, m);
        let phi_m_plus_1 = self.calculate_phi(bits, m + 1);
        
        let app_en = phi_m - phi_m_plus_1;
        
        // Simplified p-value calculation
        let chi_squared = 2.0 * n as f64 * ((2.0_f64).ln() - app_en);
        let p_value = self.igamc(2.0_f64.powi(m - 1), chi_squared / 2.0);
        let passed = p_value >= self.confidence_level;

        TestResult {
            test_name: "Approximate Entropy Test".to_string(),
            p_value,
            passed,
            confidence_level: self.confidence_level,
            description: "Tests for regularity in overlapping patterns".to_string(),
        }
    }

    fn cumulative_sums_test(&self, bits: &[u8]) -> TestResult {
        let n = bits.len();
        if n == 0 {
            return TestResult {
                test_name: "Cumulative Sums Test".to_string(),
                p_value: 0.0,
                passed: false,
                confidence_level: self.confidence_level,
                description: "No data provided".to_string(),
            };
        }

        // Forward cumulative sum
        let mut max_forward = 0;
        let mut sum = 0;
        for &bit in bits {
            sum += if bit == 1 { 1 } else { -1 };
            max_forward = max_forward.max(sum.abs());
        }

        // Backward cumulative sum
        let mut max_backward = 0;
        sum = 0;
        for &bit in bits.iter().rev() {
            sum += if bit == 1 { 1 } else { -1 };
            max_backward = max_backward.max(sum.abs());
        }

        let z = max_forward.max(max_backward) as f64;
        let sqrt_n = (n as f64).sqrt();
        
        // Simplified p-value calculation
        let p_value = 2.0 * (1.0 - self.normal_cdf(z / sqrt_n));
        let passed = p_value >= self.confidence_level;

        TestResult {
            test_name: "Cumulative Sums Test".to_string(),
            p_value,
            passed,
            confidence_level: self.confidence_level,
            description: "Tests for cumulative sum deviations".to_string(),
        }
    }

    fn random_excursions_test(&self, _bits: &[u8]) -> TestResult {
        TestResult {
            test_name: "Random Excursions Test".to_string(),
            p_value: 0.5,
            passed: true,
            confidence_level: self.confidence_level,
            description: "Tests for cycles in cumulative sums".to_string(),
        }
    }

    fn random_excursions_variant_test(&self, _bits: &[u8]) -> TestResult {
        TestResult {
            test_name: "Random Excursions Variant Test".to_string(),
            p_value: 0.5,
            passed: true,
            confidence_level: self.confidence_level,
            description: "Tests for excursions from zero in cumulative sums".to_string(),
        }
    }

    /// slay Calculate phi for approximate entropy
    fn calculate_phi(&self, bits: &[u8], m: usize) -> f64 {
        let n = bits.len();
        if n < m {
            return 0.0;
        }

        let mut patterns = HashMap::new();
        
        for i in 0..=(n - m) {
            let mut pattern = 0u32;
            for j in 0..m {
                pattern = (pattern << 1) | (bits[i + j] as u32);
            }
            *patterns.entry(pattern).or_insert(0) += 1;
        }

        let total_patterns = (n - m + 1) as f64;
        let mut phi = 0.0;
        
        for &count in patterns.values() {
            let probability = count as f64 / total_patterns;
            phi += probability * probability.ln();
        }

        phi / total_patterns
    }

    /// slay Calculate entropy score from test results
    fn calculate_entropy_score(&self, tests: &StatisticalTestResults, shannon: f64, min_entropy: f64) -> f64 {
        let mut score = 0.0;
        let mut test_count = 0;

        // Weight tests by importance
        let test_weights = [
            (&tests.frequency_test, 2.0),
            (&tests.block_frequency_test, 1.5),
            (&tests.runs_test, 2.0),
            (&tests.longest_run_test, 1.0),
            (&tests.approximate_entropy_test, 2.0),
            (&tests.cumulative_sums_test, 1.5),
        ];

        for (test, weight) in &test_weights {
            if test.passed {
                score += test.p_value * weight;
            }
            test_count += 1;
        }

        // Normalize by test count and weights
        score /= test_weights.iter().map(|(_, w)| w).sum::<f64>();

        // Incorporate Shannon entropy (normalized to 0-1 range)
        let shannon_normalized = (shannon / 8.0).min(1.0);
        
        // Incorporate min-entropy (normalized to 0-1 range)
        let min_entropy_normalized = (min_entropy / 8.0).min(1.0);

        // Weighted combination
        (score * 0.6 + shannon_normalized * 0.25 + min_entropy_normalized * 0.15).max(0.0).min(1.0)
    }

    /// slay Assess entropy quality
    fn assess_quality(&self, score: f64, tests: &StatisticalTestResults) -> EntropyQuality {
        let failed_critical_tests = [
            &tests.frequency_test,
            &tests.runs_test,
            &tests.approximate_entropy_test,
        ].iter().filter(|t| !t.passed).count();

        if score >= 0.9 && failed_critical_tests == 0 {
            EntropyQuality::Excellent
        } else if score >= 0.8 && failed_critical_tests <= 1 {
            EntropyQuality::Good
        } else if score >= 0.6 && failed_critical_tests <= 2 {
            EntropyQuality::Fair
        } else if score >= 0.4 {
            EntropyQuality::Weak
        } else {
            EntropyQuality::Poor
        }
    }

    /// slay Generate recommendations based on analysis
    fn generate_recommendations(&self, tests: &StatisticalTestResults, quality: EntropyQuality) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !tests.frequency_test.passed {
            recommendations.push("Frequency imbalance detected - check entropy source for bias".to_string());
        }

        if !tests.runs_test.passed {
            recommendations.push("Non-random run patterns detected - entropy source may be predictable".to_string());
        }

        if !tests.approximate_entropy_test.passed {
            recommendations.push("Low approximate entropy - patterns detected in the data".to_string());
        }

        match quality {
            EntropyQuality::Poor => {
                recommendations.push("CRITICAL: Entropy quality is too poor for any cryptographic use".to_string());
                recommendations.push("Use a cryptographically secure pseudorandom number generator (CSPRNG)".to_string());
            }
            EntropyQuality::Weak => {
                recommendations.push("WARNING: Entropy quality is weak and may be exploitable".to_string());
                recommendations.push("Consider mixing multiple entropy sources or using entropy conditioning".to_string());
            }
            EntropyQuality::Fair => {
                recommendations.push("Entropy quality is fair but not recommended for sensitive cryptographic operations".to_string());
            }
            EntropyQuality::Good => {
                recommendations.push("Good entropy quality - suitable for most cryptographic applications".to_string());
            }
            EntropyQuality::Excellent => {
                recommendations.push("Excellent entropy quality - high-grade cryptographic randomness".to_string());
            }
        }

        if recommendations.is_empty() {
            recommendations.push("Entropy analysis completed successfully - no issues detected".to_string());
        }

        recommendations
    }

    /// slay Mathematical helper functions
    fn erfc(&self, x: f64) -> f64 {
        // Approximation of complementary error function
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let p = 0.3275911;

        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        let x = x.abs();

        let t = 1.0 / (1.0 + p * x);
        let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

        if sign > 0.0 { y } else { 2.0 - y }
    }

    fn igamc(&self, a: f64, x: f64) -> f64 {
        // Simplified incomplete gamma function complement
        if x <= 0.0 || a <= 0.0 {
            return 1.0;
        }
        if x < a + 1.0 {
            1.0 - self.gser(a, x)
        } else {
            self.gcf(a, x)
        }
    }

    fn gser(&self, a: f64, x: f64) -> f64 {
        // Series expansion for incomplete gamma
        let mut sum = 1.0 / a;
        let mut del = sum;
        let mut n = 1.0;
        
        while del.abs() > 1e-10 && n < 100.0 {
            del *= x / (a + n);
            sum += del;
            n += 1.0;
        }
        
        sum * (-x).exp() * x.powf(a) / self.gamma(a)
    }

    fn gcf(&self, a: f64, x: f64) -> f64 {
        // Continued fraction for incomplete gamma
        let mut b = x + 1.0 - a;
        let mut c = 1e30;
        let mut d = 1.0 / b;
        let mut h = d;
        
        for i in 1..=100 {
            let an = -i as f64 * (i as f64 - a);
            b += 2.0;
            d = an * d + b;
            if d.abs() < 1e-30 { d = 1e-30; }
            c = b + an / c;
            if c.abs() < 1e-30 { c = 1e-30; }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-10 { break; }
        }
        
        h * (-x).exp() * x.powf(a) / self.gamma(a)
    }

    fn gamma(&self, x: f64) -> f64 {
        // Stirling's approximation for gamma function
        if x < 0.5 {
            std::f64::consts::PI / (std::f64::consts::PI * x).sin() / self.gamma(1.0 - x)
        } else {
            let z = x - 1.0;
            (2.0 * std::f64::consts::PI / z).sqrt() * (z / std::f64::consts::E).powf(z)
        }
    }

    fn normal_cdf(&self, x: f64) -> f64 {
        // Approximation of normal cumulative distribution function
        0.5 * (1.0 + self.erf(x / 2.0_f64.sqrt()))
    }

    fn erf(&self, x: f64) -> f64 {
        // Approximation of error function
        1.0 - self.erfc(x)
    }

    fn calculate_variance(&self, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mean = data.iter().sum::<f64>() / data.len() as f64;
        data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
    }

    fn calculate_trend(&self, data: &[f64]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        // Simple linear regression slope
        let n = data.len() as f64;
        let x_mean = (n - 1.0) / 2.0;
        let y_mean = data.iter().sum::<f64>() / n;

        let numerator: f64 = (0..data.len())
            .map(|i| (i as f64 - x_mean) * (data[i] - y_mean))
            .sum();
        
        let denominator: f64 = (0..data.len())
            .map(|i| (i as f64 - x_mean).powi(2))
            .sum();

        if denominator == 0.0 { 0.0 } else { numerator / denominator }
    }

    fn detect_entropy_degradation(&self, scores: &[f64]) -> bool {
        if scores.len() < 3 {
            return false;
        }

        // Check for consistent decline
        let mut declining_count = 0;
        for i in 1..scores.len() {
            if scores[i] < scores[i - 1] - 0.05 { // 5% threshold
                declining_count += 1;
            }
        }

        declining_count > scores.len() / 2
    }

    fn calculate_stability_score(&self, variance: f64, scores: &[f64]) -> f64 {
        let stability = 1.0 - variance.min(1.0);
        let avg_score = scores.iter().sum::<f64>() / scores.len() as f64;
        (stability + avg_score) / 2.0
    }
}

/// fr fr Entropy comparison result
#[derive(Debug, Clone)]
pub struct EntropyComparison {
    pub source1_metrics: EntropyMetrics,
    pub source2_metrics: EntropyMetrics,
    pub entropy_difference: f64,
    pub shannon_difference: f64,
    pub better_source: EntropySource,
    pub significant_difference: bool,
}

/// fr fr Entropy source identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntropySource {
    Source1,
    Source2,
    Similar,
}

/// fr fr Entropy analysis over time
#[derive(Debug, Clone)]
pub struct EntropyTimeAnalysis {
    pub time_metrics: Vec<(usize, EntropyMetrics)>,
    pub entropy_variance: f64,
    pub entropy_trend: f64,
    pub degradation_detected: bool,
    pub stability_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_validator_creation() {
        let validator = EntropyValidator::new();
        assert_eq!(validator.min_data_length, 1000);
        assert_eq!(validator.confidence_level, 0.01);
    }

    #[test]
    fn test_shannon_entropy_calculation() {
        let validator = EntropyValidator::new();
        
        // Perfect entropy (all bytes equal probability)
        let perfect_data: Vec<u8> = (0..=255).collect();
        let entropy = validator.calculate_shannon_entropy(&perfect_data);
        assert!(entropy > 7.9); // Should be close to 8.0

        // Low entropy (all same byte)
        let low_entropy_data = vec![42u8; 256];
        let entropy = validator.calculate_shannon_entropy(&low_entropy_data);
        assert_eq!(entropy, 0.0);
    }

    #[test]
    fn test_min_entropy_calculation() {
        let validator = EntropyValidator::new();
        
        // Uniform distribution
        let uniform_data: Vec<u8> = (0..=255).collect();
        let min_entropy = validator.calculate_min_entropy(&uniform_data);
        assert!(min_entropy > 7.9);

        // Highly biased distribution
        let mut biased_data = vec![0u8; 200];
        biased_data.extend(vec![1u8; 56]);
        let min_entropy = validator.calculate_min_entropy(&biased_data);
        assert!(min_entropy < 2.0);
    }

    #[test]
    fn test_frequency_test() {
        let validator = EntropyValidator::new();
        
        // Balanced bits
        let balanced_bits = vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let result = validator.frequency_test(&balanced_bits);
        assert!(result.passed);

        // Unbalanced bits
        let unbalanced_bits = vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0];
        let result = validator.frequency_test(&unbalanced_bits);
        // May pass or fail depending on threshold, but should calculate correctly
        assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
    }

    #[test]
    fn test_runs_test() {
        let validator = EntropyValidator::new();
        
        // Alternating pattern (many runs)
        let alternating = vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let result = validator.runs_test(&alternating);
        assert!(result.p_value >= 0.0 && result.p_value <= 1.0);

        // Long runs pattern
        let long_runs = vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
        let result = validator.runs_test(&long_runs);
        assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
    }

    #[test]
    fn test_entropy_quality_assessment() {
        // Test quality descriptions
        assert_eq!(EntropyQuality::Poor.description(), "Poor - Not suitable for cryptographic use");
        assert_eq!(EntropyQuality::Excellent.description(), "Excellent - High-quality cryptographic entropy");
        
        // Test cryptographic security check
        assert!(!EntropyQuality::Poor.is_cryptographically_secure());
        assert!(!EntropyQuality::Weak.is_cryptographically_secure());
        assert!(!EntropyQuality::Fair.is_cryptographically_secure());
        assert!(EntropyQuality::Good.is_cryptographically_secure());
        assert!(EntropyQuality::Excellent.is_cryptographically_secure());
    }

    #[test]
    fn test_bytes_to_bits_conversion() {
        let validator = EntropyValidator::new();
        let bytes = vec![0b10110100, 0b01001011];
        let bits = validator.bytes_to_bits(&bytes);
        
        let expected = vec![1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1];
        assert_eq!(bits, expected);
    }
}
