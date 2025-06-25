/// Random number generation utilities

use std::sync::Mutex;
use rand::{Rng, SeedableRng};
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand_chacha::ChaCha20Rng;
use once_cell::sync::Lazy;

use super::{MathError, MathResult, validate_float, domain_error};
use crate::error::CursedError;

/// Thread-safe random number generator
static RNG: Lazy<Mutex<ChaCha20Rng>> = Lazy::new(|| Mutex::new(ChaCha20Rng::from_entropy()));

/// Generate a random f64 in the range [0.0, 1.0)
pub fn random() -> f64 {
    let mut rng = RNG.lock().unwrap();
    rng.gen()
/// Generate a random f64 in the range [min, max)
pub fn random_range(min: f64, max: f64) -> MathResult<f64> {
    validate_float("random_range", "min", min)?;
    validate_float("random_range", "max", max)?;
    
    if min >= max {
        return Err(domain_error("random_range", min, "min must be less than max"));
    let mut rng = RNG.lock().unwrap();
    let dist = Uniform::new(min, max);
    Ok(dist.sample(&mut *rng))
/// Generate a random integer in the range [min, max] (inclusive)
pub fn random_int(min: i64, max: i64) -> MathResult<i64> {
    if min > max {
        return Err(MathError::InvalidInput {
        });
    let mut rng = RNG.lock().unwrap();
    let dist = Uniform::new_inclusive(min, max);
    Ok(dist.sample(&mut *rng))
/// Generate a random u64 in the range [min, max] (inclusive)
pub fn random_u64(min: u64, max: u64) -> MathResult<u64> {
    if min > max {
        return Err(MathError::InvalidInput {
        });
    let mut rng = RNG.lock().unwrap();
    let dist = Uniform::new_inclusive(min, max);
    Ok(dist.sample(&mut *rng))
/// Generate a random boolean with given probability of being true
pub fn random_bool(probability: f64) -> MathResult<bool> {
    validate_float("random_bool", "probability", probability)?;
    
    if probability < 0.0 || probability > 1.0 {
        return Err(domain_error("random_bool", probability, "probability must be between 0 and 1"));
    Ok(random() < probability)
/// Generate a random element from a slice
pub fn choice<T: Clone>(items: &[T]) -> MathResult<T> {
    if items.is_empty() {
        return Err(MathError::InvalidInput {
        });
    let index = random_u64(0, items.len() as u64 - 1)?;
    Ok(items[index as usize].clone())
/// Generate multiple random elements from a slice (with replacement)
pub fn choices<T: Clone>(items: &[T], count: usize) -> MathResult<Vec<T>> {
    if items.is_empty() {
        return Err(MathError::InvalidInput {
        });
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        let item = choice(items)?;
        result.push(item);
    Ok(result)
/// Generate a weighted random choice
pub fn weighted_choice<T: Clone>(items: &[T], weights: &[f64]) -> MathResult<T> {
    if items.is_empty() || weights.is_empty() {
        return Err(MathError::InvalidInput {
            parameter: "items/weights".to_string(),
        });
    if items.len() != weights.len() {
        return Err(MathError::InvalidInput {
        });
    for &weight in weights {
        validate_float("weighted_choice", "weight", weight)?;
        if weight < 0.0 {
            return Err(domain_error("weighted_choice", weight, "weights must be non-negative"));
        }
    }
    
    let dist = WeightedIndex::new(weights).map_err(|_| {
        MathError::InvalidInput {
        }
    })?;
    
    let mut rng = RNG.lock().unwrap();
    let index = dist.sample(&mut *rng);
    
    Ok(items[index].clone())
/// Shuffle a vector in place using Fisher-Yates algorithm
pub fn shuffle<T>(items: &mut [T]) -> MathResult<()> {
    let mut rng = RNG.lock().unwrap();
    
    for i in (1..items.len()).rev() {
        let j = random_u64(0, i as u64)? as usize;
        items.swap(i, j);
    Ok(())
/// Generate a shuffled copy of a slice
pub fn shuffled<T: Clone>(items: &[T]) -> MathResult<Vec<T>> {
    let mut result = items.to_vec();
    shuffle(&mut result)?;
    Ok(result)
/// Sample without replacement from a slice
pub fn sample<T: Clone>(items: &[T], count: usize) -> MathResult<Vec<T>> {
    if count > items.len() {
        return Err(MathError::InvalidInput {
        });
    let mut indices: Vec<usize> = (0..items.len()).collect();
    shuffle(&mut indices)?;
    
    let mut result = Vec::with_capacity(count);
    for &i in indices.iter().take(count) {
        result.push(items[i].clone());
    Ok(result)
/// Generate random bytes
pub fn random_bytes(count: usize) -> Vec<u8> {
    let mut rng = RNG.lock().unwrap();
    let mut bytes = vec![0u8; count];
    rng.fill(&mut bytes[..]);
    bytes
/// Generate a random string of given length using specified alphabet
pub fn random_string(length: usize, alphabet: &str) -> MathResult<String> {
    if alphabet.is_empty() {
        return Err(MathError::InvalidInput {
        });
    let chars: Vec<char> = alphabet.chars().collect();
    let mut result = String::with_capacity(length);
    
    for _ in 0..length {
        let c = choice(&chars)?;
        result.push(c);
    Ok(result)
/// Generate a random alphanumeric string
pub fn random_alphanumeric(length: usize) -> MathResult<String> {
    const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    random_string(length, ALPHABET)
/// Generate a random hexadecimal string
pub fn random_hex(length: usize) -> MathResult<String> {
    const HEX_CHARS: &str = "0123456789abcdef";
    random_string(length, HEX_CHARS)
/// Seed the random number generator
pub fn set_seed(seed: u64) {
    let mut rng = RNG.lock().unwrap();
    *rng = ChaCha20Rng::seed_from_u64(seed);
/// Generate random numbers from a normal (Gaussian) distribution
pub fn random_normal(mean: f64, std_dev: f64) -> MathResult<f64> {
    validate_float("random_normal", "mean", mean)?;
    validate_float("random_normal", "std_dev", std_dev)?;
    
    if std_dev <= 0.0 {
        return Err(domain_error("random_normal", std_dev, "standard deviation must be positive"));
    use rand_distr::Normal;
    
    let mut rng = RNG.lock().unwrap();
    let normal = Normal::new(mean, std_dev).map_err(|_| {
        MathError::InvalidInput {
        }
    })?;
    
    Ok(normal.sample(&mut *rng))
/// Generate random numbers from an exponential distribution
pub fn random_exponential(lambda: f64) -> MathResult<f64> {
    validate_float("random_exponential", "lambda", lambda)?;
    
    if lambda <= 0.0 {
        return Err(domain_error("random_exponential", lambda, "lambda must be positive"));
    use rand_distr::Exp;
    
    let mut rng = RNG.lock().unwrap();
    let exp = Exp::new(lambda).map_err(|_| {
        MathError::InvalidInput {
        }
    })?;
    
    Ok(exp.sample(&mut *rng))
/// Generate random numbers from a uniform distribution
pub fn random_uniform(min: f64, max: f64) -> MathResult<f64> {
    random_range(min, max)
/// Generate random integers from a Poisson distribution
pub fn random_poisson(lambda: f64) -> MathResult<u64> {
    validate_float("random_poisson", "lambda", lambda)?;
    
    if lambda <= 0.0 {
        return Err(domain_error("random_poisson", lambda, "lambda must be positive"));
    use rand_distr::Poisson;
    
    let mut rng = RNG.lock().unwrap();
    let poisson = Poisson::new(lambda).map_err(|_| {
        MathError::InvalidInput {
        }
    })?;
    
    Ok(poisson.sample(&mut *rng) as u64)
/// Generate random numbers from a beta distribution
pub fn random_beta(alpha: f64, beta: f64) -> MathResult<f64> {
    validate_float("random_beta", "alpha", alpha)?;
    validate_float("random_beta", "beta", beta)?;
    
    if alpha <= 0.0 || beta <= 0.0 {
        return Err(domain_error("random_beta", alpha.min(beta), "alpha and beta must be positive"));
    use rand_distr::Beta;
    
    let mut rng = RNG.lock().unwrap();
    let beta_dist = Beta::new(alpha, beta).map_err(|_| {
        MathError::InvalidInput {
        }
    })?;
    
    Ok(beta_dist.sample(&mut *rng))
/// Generate random numbers from a gamma distribution
pub fn random_gamma(shape: f64, scale: f64) -> MathResult<f64> {
    validate_float("random_gamma", "shape", shape)?;
    validate_float("random_gamma", "scale", scale)?;
    
    if shape <= 0.0 || scale <= 0.0 {
        return Err(domain_error("random_gamma", shape.min(scale), "shape and scale must be positive"));
    use rand_distr::Gamma;
    
    let mut rng = RNG.lock().unwrap();
    let gamma = Gamma::new(shape, scale).map_err(|_| {
        MathError::InvalidInput {
        }
    })?;
    
    Ok(gamma.sample(&mut *rng))
}
