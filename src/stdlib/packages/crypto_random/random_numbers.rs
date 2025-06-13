/// Random number generation with various distributions and utilities
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use super::secure_random::SecureRandom;
use std::f64::consts::{PI, E};

/// Random number generator with statistical distributions
pub struct RandomNumbers {
    secure_rng: SecureRandom,
}

impl RandomNumbers {
    /// Create new random number generator
    pub fn new() -> AdvancedCryptoResult<Self> {
        Ok(Self {
            secure_rng: SecureRandom::new()?,
        })
    }
    
    /// Generate random number with normal (Gaussian) distribution
    /// Uses Box-Muller transform
    pub fn normal(&self, mean: f64, std_dev: f64) -> AdvancedCryptoResult<f64> {
        // Box-Muller transform to generate normal distribution
        let u1 = self.secure_rng.f64()?;
        let u2 = self.secure_rng.f64()?;
        
        // Ensure u1 is not zero to avoid log(0)
        let u1 = if u1 == 0.0 { f64::EPSILON } else { u1 };
        
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
        Ok(mean + std_dev * z0)
    }
    
    /// Generate pair of normal random numbers (Box-Muller gives two)
    pub fn normal_pair(&self, mean: f64, std_dev: f64) -> AdvancedCryptoResult<(f64, f64)> {
        let u1 = self.secure_rng.f64()?;
        let u2 = self.secure_rng.f64()?;
        
        let u1 = if u1 == 0.0 { f64::EPSILON } else { u1 };
        
        let factor = (-2.0 * u1.ln()).sqrt();
        let z0 = factor * (2.0 * PI * u2).cos();
        let z1 = factor * (2.0 * PI * u2).sin();
        
        Ok((mean + std_dev * z0, mean + std_dev * z1))
    }
    
    /// Generate random number with exponential distribution
    pub fn exponential(&self, lambda: f64) -> AdvancedCryptoResult<f64> {
        if lambda <= 0.0 {
            return Err("Lambda must be positive for exponential distribution".into());
        }
        
        let u = self.secure_rng.f64()?;
        let u = if u == 0.0 { f64::EPSILON } else { u };
        
        Ok(-u.ln() / lambda)
    }
    
    /// Generate random number with gamma distribution
    /// Uses Marsaglia and Tsang's method
    pub fn gamma(&self, alpha: f64, beta: f64) -> AdvancedCryptoResult<f64> {
        if alpha <= 0.0 || beta <= 0.0 {
            return Err("Alpha and beta must be positive for gamma distribution".into());
        }
        
        if alpha < 1.0 {
            // For alpha < 1, use transformation
            let gamma_1_plus_alpha = self.gamma(alpha + 1.0, beta)?;
            let u = self.secure_rng.f64()?;
            let u = if u == 0.0 { f64::EPSILON } else { u };
            
            Ok(gamma_1_plus_alpha * u.powf(1.0 / alpha))
        } else {
            // Marsaglia and Tsang's method for alpha >= 1
            let d = alpha - 1.0 / 3.0;
            let c = 1.0 / (9.0 * d).sqrt();
            
            loop {
                let x = self.normal(0.0, 1.0)?;
                let v = (1.0 + c * x).powi(3);
                
                if v > 0.0 {
                    let u = self.secure_rng.f64()?;
                    let x_squared = x * x;
                    
                    if u < 1.0 - 0.0331 * x_squared * x_squared {
                        return Ok(d * v / beta);
                    }
                    
                    if u.ln() < 0.5 * x_squared + d * (1.0 - v + v.ln()) {
                        return Ok(d * v / beta);
                    }
                }
            }
        }
    }
    
    /// Generate random number with beta distribution
    pub fn beta(&self, alpha: f64, beta: f64) -> AdvancedCryptoResult<f64> {
        if alpha <= 0.0 || beta <= 0.0 {
            return Err("Alpha and beta must be positive for beta distribution".into());
        }
        
        let x = self.gamma(alpha, 1.0)?;
        let y = self.gamma(beta, 1.0)?;
        
        Ok(x / (x + y))
    }
    
    /// Generate random number with Poisson distribution
    /// Uses Knuth's algorithm for small lambda, rejection method for large lambda
    pub fn poisson(&self, lambda: f64) -> AdvancedCryptoResult<u32> {
        if lambda <= 0.0 {
            return Err("Lambda must be positive for Poisson distribution".into());
        }
        
        if lambda < 30.0 {
            // Knuth's algorithm for small lambda
            let l = (-lambda).exp();
            let mut k = 0;
            let mut p = 1.0;
            
            loop {
                k += 1;
                let u = self.secure_rng.f64()?;
                p *= u;
                
                if p <= l {
                    return Ok((k - 1) as u32);
                }
            }
        } else {
            // Atkinson's algorithm for large lambda
            let c = 0.445 * lambda - 0.08;
            let d = 6.0 / lambda / lambda + 6.0 / lambda + 1.0;
            let m = (lambda + 0.5).max(0.0);
            let l = (lambda - 1.1459 * lambda.ln()).sqrt();
            let g = lambda + l * self.normal(0.0, 1.0)?;
            
            loop {
                let u = self.secure_rng.f64()?;
                let x = (alpha: f64) - 0.5;
                let y = 0.5 * (u - 0.5).abs() / l;
                let lhs = 0.83 + 0.65 / lambda + 0.5 / (lambda * lambda);
                let rhs = 0.25 * y * (y + 1.0) + (lambda - m) / l;
                
                if rhs >= lhs {
                    continue;
                }
                
                let k = (g + 0.5).floor();
                if k >= 0.0 {
                    return Ok(k as u32);
                }
            }
        }
    }
    
    /// Generate random number with binomial distribution
    pub fn binomial(&self, n: u32, p: f64) -> AdvancedCryptoResult<u32> {
        if p < 0.0 || p > 1.0 {
            return Err("Probability must be between 0 and 1 for binomial distribution".into());
        }
        
        if n == 0 {
            return Ok(0);
        }
        
        // For large n, use normal approximation
        if n > 50 && n as f64 * p * (1.0 - p) > 5.0 {
            let mean = n as f64 * p;
            let variance = n as f64 * p * (1.0 - p);
            let std_dev = variance.sqrt();
            
            let x = self.normal(mean, std_dev)?;
            let result = x.round() as i64;
            
            Ok(result.max(0).min(n as i64) as u32)
        } else {
            // Direct method for small n
            let mut count = 0;
            for _ in 0..n {
                if self.secure_rng.f64()? < p {
                    count += 1;
                }
            }
            Ok(count)
        }
    }
    
    /// Generate random number with geometric distribution
    pub fn geometric(&self, p: f64) -> AdvancedCryptoResult<u32> {
        if p <= 0.0 || p > 1.0 {
            return Err("Probability must be between 0 and 1 for geometric distribution".into());
        }
        
        let u = self.secure_rng.f64()?;
        let u = if u == 0.0 { f64::EPSILON } else { u };
        
        let result = (u.ln() / (1.0 - p).ln()).ceil() as u32;
        Ok(result.max(1))
    }
    
    /// Generate random number with log-normal distribution
    pub fn log_normal(&self, mu: f64, sigma: f64) -> AdvancedCryptoResult<f64> {
        if sigma <= 0.0 {
            return Err("Sigma must be positive for log-normal distribution".into());
        }
        
        let normal_sample = self.normal(mu, sigma)?;
        Ok(normal_sample.exp())
    }
    
    /// Generate random number with Weibull distribution
    pub fn weibull(&self, lambda: f64, k: f64) -> AdvancedCryptoResult<f64> {
        if lambda <= 0.0 || k <= 0.0 {
            return Err("Lambda and k must be positive for Weibull distribution".into());
        }
        
        let u = self.secure_rng.f64()?;
        let u = if u == 0.0 { f64::EPSILON } else { u };
        
        Ok(lambda * (-u.ln()).powf(1.0 / k))
    }
    
    /// Generate random number with chi-squared distribution
    pub fn chi_squared(&self, df: f64) -> AdvancedCryptoResult<f64> {
        if df <= 0.0 {
            return Err("Degrees of freedom must be positive for chi-squared distribution".into());
        }
        
        // Chi-squared is a special case of gamma distribution
        self.gamma(df / 2.0, 2.0)
    }
    
    /// Generate random number with Student's t-distribution
    pub fn student_t(&self, df: f64) -> AdvancedCryptoResult<f64> {
        if df <= 0.0 {
            return Err("Degrees of freedom must be positive for Student's t-distribution".into());
        }
        
        let z = self.normal(0.0, 1.0)?;
        let v = self.chi_squared(df)?;
        
        Ok(z / (v / df).sqrt())
    }
    
    /// Generate random number with F-distribution
    pub fn f_distribution(&self, df1: f64, df2: f64) -> AdvancedCryptoResult<f64> {
        if df1 <= 0.0 || df2 <= 0.0 {
            return Err("Degrees of freedom must be positive for F-distribution".into());
        }
        
        let chi1 = self.chi_squared(df1)?;
        let chi2 = self.chi_squared(df2)?;
        
        Ok((chi1 / df1) / (chi2 / df2))
    }
    
    /// Generate random number with triangular distribution
    pub fn triangular(&self, a: f64, b: f64, c: f64) -> AdvancedCryptoResult<f64> {
        if a >= b || c < a || c > b {
            return Err("Invalid parameters for triangular distribution: a < c < b required".into());
        }
        
        let u = self.secure_rng.f64()?;
        let fc = (c - a) / (b - a);
        
        if u < fc {
            Ok(a + ((b - a) * (c - a) * u).sqrt())
        } else {
            Ok(b - ((b - a) * (b - c) * (1.0 - u)).sqrt())
        }
    }
    
    /// Generate random number with uniform distribution in range [a, b)
    pub fn uniform(&self, a: f64, b: f64) -> AdvancedCryptoResult<f64> {
        if a >= b {
            return Err("Invalid range for uniform distribution: a must be less than b".into());
        }
        
        let u = self.secure_rng.f64()?;
        Ok(a + u * (b - a))
    }
    
    /// Generate random integer with uniform distribution in range [a, b] (inclusive)
    pub fn uniform_int(&self, a: i64, b: i64) -> AdvancedCryptoResult<i64> {
        if a > b {
            return Err("Invalid range for uniform integer distribution: a must be <= b".into());
        }
        
        self.secure_rng.range_i64(a, b)
    }
    
    /// Generate vector of random numbers with specified distribution
    pub fn normal_vector(&self, size: usize, mean: f64, std_dev: f64) -> AdvancedCryptoResult<Vec<f64>> {
        let mut values = Vec::with_capacity(size);
        for _ in 0..size {
            values.push(self.normal(mean, std_dev)?);
        }
        Ok(values)
    }
    
    /// Generate vector of random numbers with uniform distribution
    pub fn uniform_vector(&self, size: usize, a: f64, b: f64) -> AdvancedCryptoResult<Vec<f64>> {
        let mut values = Vec::with_capacity(size);
        for _ in 0..size {
            values.push(self.uniform(a, b)?);
        }
        Ok(values)
    }
    
    /// Sample from a discrete distribution given probabilities
    pub fn categorical(&self, probabilities: &[f64]) -> AdvancedCryptoResult<usize> {
        if probabilities.is_empty() {
            return Err("Probabilities vector cannot be empty".into());
        }
        
        // Normalize probabilities
        let sum: f64 = probabilities.iter().sum();
        if sum <= 0.0 {
            return Err("Sum of probabilities must be positive".into());
        }
        
        let u = self.secure_rng.f64()? * sum;
        let mut cumulative = 0.0;
        
        for (i, &prob) in probabilities.iter().enumerate() {
            cumulative += prob;
            if u <= cumulative {
                return Ok(i);
            }
        }
        
        // Fallback to last index due to floating point precision
        Ok(probabilities.len() - 1)
    }
    
    /// Sample without replacement from a collection
    pub fn sample_without_replacement<T: Clone>(&self, items: &[T], count: usize) -> AdvancedCryptoResult<Vec<T>> {
        if count > items.len() {
            return Err("Cannot sample more items than available".into());
        }
        
        let mut available: Vec<_> = items.iter().enumerate().collect();
        let mut result = Vec::with_capacity(count);
        
        for _ in 0..count {
            let index = self.secure_rng.range_u64(0, available.len() as u64 - 1)? as usize;
            let (_, item) = available.swap_remove(index);
            result.push(item.clone());
        }
        
        Ok(result)
    }
    
    /// Generate random walk (cumulative sum of random steps)
    pub fn random_walk(&self, steps: usize, step_size: f64) -> AdvancedCryptoResult<Vec<f64>> {
        let mut walk = Vec::with_capacity(steps + 1);
        walk.push(0.0); // Start at 0
        
        let mut current = 0.0;
        for _ in 0..steps {
            let step = if self.secure_rng.bool()? { step_size } else { -step_size };
            current += step;
            walk.push(current);
        }
        
        Ok(walk)
    }
    
    /// Generate Brownian motion (continuous random walk)
    pub fn brownian_motion(&self, steps: usize, dt: f64) -> AdvancedCryptoResult<Vec<f64>> {
        let mut motion = Vec::with_capacity(steps + 1);
        motion.push(0.0); // Start at 0
        
        let std_dev = dt.sqrt();
        let mut current = 0.0;
        
        for _ in 0..steps {
            let increment = self.normal(0.0, std_dev)?;
            current += increment;
            motion.push(current);
        }
        
        Ok(motion)
    }
}

impl Default for RandomNumbers {
    fn default() -> Self {
        Self::new().expect("Failed to create default RandomNumbers")
    }
}

/// Global functions for convenient access to random number distributions
pub fn random_normal(mean: f64, std_dev: f64) -> AdvancedCryptoResult<f64> {
    RandomNumbers::new()?.normal(mean, std_dev)
}

pub fn random_exponential(lambda: f64) -> AdvancedCryptoResult<f64> {
    RandomNumbers::new()?.exponential(lambda)
}

pub fn random_uniform(a: f64, b: f64) -> AdvancedCryptoResult<f64> {
    RandomNumbers::new()?.uniform(a, b)
}

pub fn random_uniform_int(a: i64, b: i64) -> AdvancedCryptoResult<i64> {
    RandomNumbers::new()?.uniform_int(a, b)
}

pub fn random_poisson(lambda: f64) -> AdvancedCryptoResult<u32> {
    RandomNumbers::new()?.poisson(lambda)
}

pub fn random_binomial(n: u32, p: f64) -> AdvancedCryptoResult<u32> {
    RandomNumbers::new()?.binomial(n, p)
}
