/// Entropy estimation algorithms for cryptographic random number generation
use std::collections::HashMap;
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// Entropy estimation method
#[derive(Debug, Clone, PartialEq)]
pub enum EntropyEstimationMethod {
    Shannon,           // Shannon entropy
    MinEntropy,        // Min-entropy (most conservative)
    RenyiEntropy(f64), // Rényi entropy with parameter α
    Collision,         // Collision entropy
    Compression,       // Compression-based estimation
    SpectralTest,      // Spectral test
    SerialTest,        // Serial correlation test
    ApproximateEntropy, // Approximate entropy (ApEn)
    SampleEntropy,     // Sample entropy (SampEn)
    PermutationEntropy, // Permutation entropy
}

/// Entropy estimation result
#[derive(Debug, Clone)]
pub struct EntropyEstimate {
    pub method: EntropyEstimationMethod,
    pub estimated_bits: f64,
    pub bits_per_byte: f64,
    pub confidence: f64,        // Confidence in estimate (0.0 to 1.0)
    pub sample_size: usize,
    pub statistical_significance: f64, // P-value or similar
}

/// Comprehensive entropy estimator
pub struct EntropyEstimator {
    // Configuration parameters
    pub pattern_length: usize,      // Pattern length for tests
    pub tolerance: f64,             // Tolerance for approximate entropy
    pub embedding_dimension: usize, // Embedding dimension for complexity measures
}

impl EntropyEstimator {
    /// Create new entropy estimator with default parameters
    pub fn new() -> Self {
        Self {
            pattern_length: 2,
            tolerance: 0.1,
            embedding_dimension: 3,
        }
    }
    
    /// Create entropy estimator with custom parameters
    pub fn with_config(pattern_length: usize, tolerance: f64, embedding_dimension: usize) -> Self {
        Self {
            pattern_length,
            tolerance,
            embedding_dimension,
        }
    }
    
    /// Estimate entropy using all available methods
    pub fn estimate_all(&self, data: &[u8]) -> AdvancedCryptoResult<Vec<EntropyEstimate>> {
        if data.is_empty() {
            return Err("Cannot estimate entropy of empty data".into());
        }
        
        let mut estimates = Vec::new();
        
        // Shannon entropy
        estimates.push(self.estimate_shannon(data)?);
        
        // Min-entropy
        estimates.push(self.estimate_min_entropy(data)?);
        
        // Rényi entropy (α = 2, collision entropy)
        estimates.push(self.estimate_renyi(data, 2.0)?);
        
        // Compression-based
        estimates.push(self.estimate_compression(data)?);
        
        // Spectral test
        estimates.push(self.estimate_spectral(data)?);
        
        // Serial correlation test
        estimates.push(self.estimate_serial(data)?);
        
        // Approximate entropy
        estimates.push(self.estimate_approximate_entropy(data)?);
        
        // Sample entropy
        estimates.push(self.estimate_sample_entropy(data)?);
        
        // Permutation entropy
        estimates.push(self.estimate_permutation_entropy(data)?);
        
        Ok(estimates)
    }
    
    /// Estimate Shannon entropy
    pub fn estimate_shannon(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyEstimate> {
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        let len = data.len() as f64;
        let mut entropy = 0.0;
        
        for &freq in &frequencies {
            if freq > 0 {
                let p = freq as f64 / len;
                entropy -= p * p.log2();
            }
        }
        
        let total_bits = entropy * len;
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::Shannon,
            estimated_bits: total_bits,
            bits_per_byte: entropy,
            confidence: 0.9, // Shannon is generally reliable
            sample_size: data.len(),
            statistical_significance: 0.0, // Not applicable for Shannon
        })
    }
    
    /// Estimate min-entropy (most conservative measure)
    pub fn estimate_min_entropy(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyEstimate> {
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        // Find the most frequent symbol
        let max_freq = frequencies.iter().max().unwrap_or(&0);
        
        if *max_freq == 0 {
            return Ok(EntropyEstimate {
                method: EntropyEstimationMethod::MinEntropy,
                estimated_bits: 0.0,
                bits_per_byte: 0.0,
                confidence: 1.0,
                sample_size: data.len(),
                statistical_significance: 0.0,
            });
        }
        
        let p_max = *max_freq as f64 / data.len() as f64;
        let min_entropy_per_symbol = -p_max.log2();
        let total_bits = min_entropy_per_symbol * data.len() as f64;
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::MinEntropy,
            estimated_bits: total_bits,
            bits_per_byte: min_entropy_per_symbol,
            confidence: 0.95, // Very conservative, high confidence
            sample_size: data.len(),
            statistical_significance: 0.0,
        })
    }
    
    /// Estimate Rényi entropy
    pub fn estimate_renyi(&self, data: &[u8], alpha: f64) -> AdvancedCryptoResult<EntropyEstimate> {
        if alpha == 1.0 {
            return self.estimate_shannon(data); // Rényi entropy approaches Shannon as α → 1
        }
        
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        let len = data.len() as f64;
        let mut sum = 0.0;
        
        for &freq in &frequencies {
            if freq > 0 {
                let p = freq as f64 / len;
                sum += p.powf(alpha);
            }
        }
        
        let renyi_entropy = if alpha > 1.0 && sum > 0.0 {
            (1.0 / (1.0 - alpha)) * sum.log2()
        } else {
            0.0
        };
        
        let total_bits = renyi_entropy * len;
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::RenyiEntropy(alpha),
            estimated_bits: total_bits,
            bits_per_byte: renyi_entropy,
            confidence: 0.85,
            sample_size: data.len(),
            statistical_significance: 0.0,
        })
    }
    
    /// Estimate entropy using compression ratio
    pub fn estimate_compression(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyEstimate> {
        if data.is_empty() {
            return Ok(EntropyEstimate {
                method: EntropyEstimationMethod::Compression,
                estimated_bits: 0.0,
                bits_per_byte: 0.0,
                confidence: 0.0,
                sample_size: 0,
                statistical_significance: 0.0,
            });
        }
        
        // Simple compression using run-length encoding
        let compressed_size = self.simple_compress(data);
        let compression_ratio = compressed_size as f64 / data.len() as f64;
        
        // Estimate entropy based on compression ratio
        // Better compression suggests lower entropy
        let estimated_entropy_per_byte = 8.0 * (1.0 - (1.0 - compression_ratio).min(1.0));
        let total_bits = estimated_entropy_per_byte * data.len() as f64;
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::Compression,
            estimated_bits: total_bits,
            bits_per_byte: estimated_entropy_per_byte,
            confidence: 0.7, // Compression-based estimates are less reliable
            sample_size: data.len(),
            statistical_significance: compression_ratio,
        })
    }
    
    /// Simple compression using run-length encoding
    fn simple_compress(&self, data: &[u8]) -> usize {
        if data.is_empty() {
            return 0;
        }
        
        let mut compressed_size = 0;
        let mut i = 0;
        
        while i < data.len() {
            let current = data[i];
            let mut run_length = 1;
            
            while i + run_length < data.len() && data[i + run_length] == current && run_length < 255 {
                run_length += 1;
            }
            
            // Each run takes 2 bytes (value + length)
            compressed_size += 2;
            i += run_length;
        }
        
        compressed_size
    }
    
    /// Estimate entropy using spectral test
    pub fn estimate_spectral(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyEstimate> {
        if data.len() < 32 {
            return Err("Insufficient data for spectral test".into());
        }
        
        // Convert bytes to binary sequence
        let mut binary_sequence = Vec::new();
        for &byte in data {
            for i in 0..8 {
                binary_sequence.push((byte >> i) & 1);
            }
        }
        
        // Simple spectral analysis - count frequency of transitions
        let mut transitions = 0;
        for i in 1..binary_sequence.len() {
            if binary_sequence[i] != binary_sequence[i - 1] {
                transitions += 1;
            }
        }
        
        let transition_rate = transitions as f64 / (binary_sequence.len() - 1) as f64;
        
        // Good randomness should have ~50% transition rate
        let quality = 1.0 - (transition_rate - 0.5).abs() * 2.0;
        let estimated_entropy_per_byte = 8.0 * quality;
        let total_bits = estimated_entropy_per_byte * data.len() as f64;
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::SpectralTest,
            estimated_bits: total_bits,
            bits_per_byte: estimated_entropy_per_byte,
            confidence: 0.75,
            sample_size: data.len(),
            statistical_significance: transition_rate,
        })
    }
    
    /// Estimate entropy using serial correlation test
    pub fn estimate_serial(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyEstimate> {
        if data.len() < self.pattern_length + 1 {
            return Err("Insufficient data for serial test".into());
        }
        
        // Count patterns of specified length
        let mut pattern_counts = HashMap::new();
        
        for i in 0..=data.len() - self.pattern_length {
            let pattern: Vec<u8> = data[i..i + self.pattern_length].to_vec();
            *pattern_counts.entry(pattern).or_insert(0) += 1;
        }
        
        // Calculate chi-square statistic
        let expected = (data.len() - self.pattern_length + 1) as f64 / (256_u32.pow(self.pattern_length as u32) as f64);
        let mut chi_square = 0.0;
        
        for &count in pattern_counts.values() {
            let observed = count as f64;
            let diff = observed - expected;
            chi_square += diff * diff / expected;
        }
        
        // Convert chi-square to quality metric (simplified)
        let degrees_of_freedom = 256_u32.pow(self.pattern_length as u32) - 1;
        let quality = if chi_square > degrees_of_freedom as f64 * 2.0 {
            0.0
        } else {
            1.0 - (chi_square / (degrees_of_freedom as f64 * 2.0))
        };
        
        let estimated_entropy_per_byte = 8.0 * quality;
        let total_bits = estimated_entropy_per_byte * data.len() as f64;
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::SerialTest,
            estimated_bits: total_bits,
            bits_per_byte: estimated_entropy_per_byte,
            confidence: 0.8,
            sample_size: data.len(),
            statistical_significance: chi_square,
        })
    }
    
    /// Estimate approximate entropy (ApEn)
    pub fn estimate_approximate_entropy(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyEstimate> {
        if data.len() < self.embedding_dimension + 1 {
            return Err("Insufficient data for approximate entropy".into());
        }
        
        let m = self.embedding_dimension;
        let r = self.tolerance;
        let n = data.len();
        
        let phi_m = self.calculate_phi(data, m, r);
        let phi_m_plus_1 = self.calculate_phi(data, m + 1, r);
        
        let apen = phi_m - phi_m_plus_1;
        
        // ApEn ranges from 0 (predictable) to ~log2(N) (random)
        let max_apen = (n as f64).log2();
        let normalized_apen = if max_apen > 0.0 { apen / max_apen } else { 0.0 };
        
        let estimated_entropy_per_byte = 8.0 * normalized_apen.min(1.0);
        let total_bits = estimated_entropy_per_byte * data.len() as f64;
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::ApproximateEntropy,
            estimated_bits: total_bits,
            bits_per_byte: estimated_entropy_per_byte,
            confidence: 0.75,
            sample_size: data.len(),
            statistical_significance: apen,
        })
    }
    
    /// Calculate phi function for approximate entropy
    fn calculate_phi(&self, data: &[u8], m: usize, r: f64) -> f64 {
        let n = data.len();
        if n < m {
            return 0.0;
        }
        
        let mut patterns = Vec::new();
        for i in 0..=n - m {
            patterns.push(&data[i..i + m]);
        }
        
        let mut sum = 0.0;
        for i in 0..patterns.len() {
            let mut count = 0;
            for j in 0..patterns.len() {
                if self.patterns_match(patterns[i], patterns[j], r) {
                    count += 1;
                }
            }
            if count > 0 {
                sum += (count as f64 / patterns.len() as f64).ln();
            }
        }
        
        sum / patterns.len() as f64
    }
    
    /// Check if patterns match within tolerance
    fn patterns_match(&self, pattern1: &[u8], pattern2: &[u8], tolerance: f64) -> bool {
        if pattern1.len() != pattern2.len() {
            return false;
        }
        
        let max_diff = pattern1.iter()
            .zip(pattern2.iter())
            .map(|(&a, &b)| (a as f64 - b as f64).abs())
            .fold(0.0, f64::max);
        
        max_diff <= tolerance * 255.0 // Scale tolerance to byte range
    }
    
    /// Estimate sample entropy (SampEn)
    pub fn estimate_sample_entropy(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyEstimate> {
        if data.len() < self.embedding_dimension + 2 {
            return Err("Insufficient data for sample entropy".into());
        }
        
        let m = self.embedding_dimension;
        let r = self.tolerance;
        
        let a = self.count_matches(data, m + 1, r);
        let b = self.count_matches(data, m, r);
        
        let sampen = if a > 0 && b > 0 {
            -(a as f64 / b as f64).ln()
        } else {
            0.0
        };
        
        // Normalize sample entropy
        let max_sampen = 2.0; // Typical maximum for sample entropy
        let normalized_sampen = (sampen / max_sampen).min(1.0);
        
        let estimated_entropy_per_byte = 8.0 * normalized_sampen;
        let total_bits = estimated_entropy_per_byte * data.len() as f64;
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::SampleEntropy,
            estimated_bits: total_bits,
            bits_per_byte: estimated_entropy_per_byte,
            confidence: 0.8,
            sample_size: data.len(),
            statistical_significance: sampen,
        })
    }
    
    /// Count pattern matches for sample entropy
    fn count_matches(&self, data: &[u8], m: usize, r: f64) -> usize {
        let n = data.len();
        if n < m + 1 {
            return 0;
        }
        
        let mut count = 0;
        for i in 0..n - m {
            for j in i + 1..n - m + 1 {
                if self.patterns_match(&data[i..i + m], &data[j..j + m], r) {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    /// Estimate permutation entropy
    pub fn estimate_permutation_entropy(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyEstimate> {
        if data.len() < self.embedding_dimension {
            return Err("Insufficient data for permutation entropy".into());
        }
        
        let m = self.embedding_dimension.min(7); // Limit to prevent factorial explosion
        let mut permutation_counts = HashMap::new();
        
        // Generate all permutations for windows of size m
        for i in 0..=data.len() - m {
            let window = &data[i..i + m];
            let permutation = self.get_ordinal_pattern(window);
            *permutation_counts.entry(permutation).or_insert(0) += 1;
        }
        
        // Calculate permutation entropy
        let total_patterns = data.len() - m + 1;
        let mut entropy = 0.0;
        
        for &count in permutation_counts.values() {
            if count > 0 {
                let p = count as f64 / total_patterns as f64;
                entropy -= p * p.log2();
            }
        }
        
        // Normalize by maximum possible permutation entropy
        let max_entropy = Self::factorial(m) as f64;
        let normalized_entropy = if max_entropy > 0.0 {
            entropy / max_entropy.log2()
        } else {
            0.0
        };
        
        let estimated_entropy_per_byte = 8.0 * normalized_entropy;
        let total_bits = estimated_entropy_per_byte * data.len() as f64;
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::PermutationEntropy,
            estimated_bits: total_bits,
            bits_per_byte: estimated_entropy_per_byte,
            confidence: 0.85,
            sample_size: data.len(),
            statistical_significance: entropy,
        })
    }
    
    /// Get ordinal pattern for permutation entropy
    fn get_ordinal_pattern(&self, window: &[u8]) -> Vec<usize> {
        let mut indexed_values: Vec<(usize, u8)> = window.iter()
            .enumerate()
            .map(|(i, &val)| (i, val))
            .collect();
        
        // Sort by value, keeping original indices
        indexed_values.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        
        // Extract the ordinal pattern
        indexed_values.iter().map(|(i, _)| *i).collect()
    }
    
    /// Calculate factorial
    fn factorial(n: usize) -> usize {
        if n <= 1 {
            1
        } else {
            n * Self::factorial(n - 1)
        }
    }
    
    /// Get best entropy estimate (most conservative reliable method)
    pub fn get_best_estimate(&self, estimates: &[EntropyEstimate]) -> Option<&EntropyEstimate> {
        // Prefer min-entropy for cryptographic applications (most conservative)
        // But also consider confidence levels
        estimates.iter()
            .filter(|est| est.confidence >= 0.8)
            .min_by(|a, b| a.bits_per_byte.partial_cmp(&b.bits_per_byte).unwrap())
    }
    
    /// Get consensus estimate from multiple methods
    pub fn get_consensus_estimate(&self, estimates: &[EntropyEstimate]) -> AdvancedCryptoResult<EntropyEstimate> {
        if estimates.is_empty() {
            return Err("No estimates provided for consensus".into());
        }
        
        // Weight estimates by confidence and conservativeness
        let mut weighted_sum = 0.0;
        let mut weight_sum = 0.0;
        let mut total_bits = 0.0;
        let sample_size = estimates[0].sample_size;
        
        for estimate in estimates {
            // Use inverse of bits_per_byte as weight (prefer conservative estimates)
            let conservativeness_weight = 8.0 / (estimate.bits_per_byte + 0.1);
            let total_weight = estimate.confidence * conservativeness_weight;
            
            weighted_sum += estimate.bits_per_byte * total_weight;
            weight_sum += total_weight;
            total_bits += estimate.estimated_bits * total_weight;
        }
        
        if weight_sum == 0.0 {
            return Err("Invalid weights in consensus calculation".into());
        }
        
        let consensus_bits_per_byte = weighted_sum / weight_sum;
        let consensus_total_bits = total_bits / weight_sum;
        
        // Calculate consensus confidence as average of reliable estimates
        let reliable_estimates: Vec<_> = estimates.iter()
            .filter(|est| est.confidence >= 0.7)
            .collect();
        
        let consensus_confidence = if reliable_estimates.is_empty() {
            0.5
        } else {
            reliable_estimates.iter()
                .map(|est| est.confidence)
                .sum::<f64>() / reliable_estimates.len() as f64
        };
        
        Ok(EntropyEstimate {
            method: EntropyEstimationMethod::Shannon, // Default for consensus
            estimated_bits: consensus_total_bits,
            bits_per_byte: consensus_bits_per_byte,
            confidence: consensus_confidence,
            sample_size,
            statistical_significance: 0.0,
        })
    }
}

impl Default for EntropyEstimator {
    fn default() -> Self {
        Self::new()
    }
}
