/// High-level secure random number generation interface
use std::sync::{Arc, Mutex, OnceLock};
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use super::csprng::{Csprng, CsprngConfig, CsprngAlgorithm};
use super::entropy_collection::{EntropyCollector, EntropyCollectionConfig};
use super::hardware_entropy::HardwareEntropyCollector;
use super::randomness_tests::RandomnessTestSuite;

/// Global secure random number generator instance
static GLOBAL_SECURE_RNG: OnceLock<Arc<Mutex<SecureRandom>>> = OnceLock::new();

/// Secure random number generator with automatic entropy management
pub struct SecureRandom {
    csprng: Csprng,
    entropy_collector: Option<Arc<Mutex<EntropyCollector>>>,
    hardware_collector: Option<Arc<HardwareEntropyCollector>>,
    test_suite: RandomnessTestSuite,
    auto_test: bool,
    initialized: bool,
}

impl SecureRandom {
    /// Create new secure random number generator
    pub fn new() -> AdvancedCryptoResult<Self> {
        let csprng_config = CsprngConfig {
            algorithm: CsprngAlgorithm::ChaCha20,
            auto_reseed: true,
            ..Default::default()
        };
        
        let csprng = Csprng::with_config(csprng_config)?;
        
        let mut secure_rng = Self {
            csprng,
            entropy_collector: None,
            hardware_collector: None,
            test_suite: RandomnessTestSuite::new(),
            auto_test: true,
            initialized: false,
        };
        
        secure_rng.initialize()?;
        Ok(secure_rng)
    }
    
    /// Initialize the secure random number generator with entropy sources
    fn initialize(&mut self) -> AdvancedCryptoResult<()> {
        // Set up hardware entropy collector
        let hardware_collector = Arc::new(HardwareEntropyCollector::new());
        if hardware_collector.has_hardware_rng() {
            self.hardware_collector = Some(hardware_collector);
        }
        
        // Set up entropy collector with system sources
        let entropy_config = EntropyCollectionConfig::default();
        let mut entropy_collector = EntropyCollector::new(entropy_config);
        entropy_collector.start()?;
        
        let entropy_arc = Arc::new(Mutex::new(entropy_collector));
        self.entropy_collector = Some(entropy_arc.clone());
        
        // Create new CSPRNG with entropy collector
        let csprng_config = CsprngConfig {
            algorithm: CsprngAlgorithm::ChaCha20,
            auto_reseed: true,
            ..Default::default()
        };
        
        self.csprng = Csprng::with_entropy_collector(csprng_config, entropy_arc)?;
        self.initialized = true;
        
        Ok(())
    }
    
    /// Generate cryptographically secure random bytes
    pub fn bytes(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        if !self.initialized {
            return Err("SecureRandom not initialized".into());
        }
        
        let bytes = self.csprng.generate(size)?;
        
        // Optionally test randomness quality
        if self.auto_test && size >= 256 {
            let _test_results = self.test_suite.quick_test(&bytes);
            // In production, you might want to handle test failures
        }
        
        Ok(bytes)
    }
    
    /// Generate a random u8
    pub fn u8(&self) -> AdvancedCryptoResult<u8> {
        let bytes = self.bytes(1)?;
        Ok(bytes[0])
    }
    
    /// Generate a random u16
    pub fn u16(&self) -> AdvancedCryptoResult<u16> {
        let bytes = self.bytes(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }
    
    /// Generate a random u32
    pub fn u32(&self) -> AdvancedCryptoResult<u32> {
        let bytes = self.bytes(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    
    /// Generate a random u64
    pub fn u64(&self) -> AdvancedCryptoResult<u64> {
        let bytes = self.bytes(8)?;
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
    
    /// Generate a random u128
    pub fn u128(&self) -> AdvancedCryptoResult<u128> {
        let bytes = self.bytes(16)?;
        let mut array = [0u8; 16];
        array.copy_from_slice(&bytes);
        Ok(u128::from_le_bytes(array))
    }
    
    /// Generate a random i8
    pub fn i8(&self) -> AdvancedCryptoResult<i8> {
        Ok(self.u8()? as i8)
    }
    
    /// Generate a random i16
    pub fn i16(&self) -> AdvancedCryptoResult<i16> {
        Ok(self.u16()? as i16)
    }
    
    /// Generate a random i32
    pub fn i32(&self) -> AdvancedCryptoResult<i32> {
        Ok(self.u32()? as i32)
    }
    
    /// Generate a random i64
    pub fn i64(&self) -> AdvancedCryptoResult<i64> {
        Ok(self.u64()? as i64)
    }
    
    /// Generate a random i128
    pub fn i128(&self) -> AdvancedCryptoResult<i128> {
        Ok(self.u128()? as i128)
    }
    
    /// Generate a random f32 in range [0.0, 1.0)
    pub fn f32(&self) -> AdvancedCryptoResult<f32> {
        let bytes = self.u32()?;
        // Use upper 23 bits for mantissa, ensuring [0.0, 1.0)
        Ok((bytes >> 9) as f32 / (1u32 << 23) as f32)
    }
    
    /// Generate a random f64 in range [0.0, 1.0)
    pub fn f64(&self) -> AdvancedCryptoResult<f64> {
        let bytes = self.u64()?;
        // Use upper 52 bits for mantissa, ensuring [0.0, 1.0)
        Ok((bytes >> 12) as f64 / (1u64 << 52) as f64)
    }
    
    /// Generate a random bool
    pub fn bool(&self) -> AdvancedCryptoResult<bool> {
        Ok(self.u8()? & 1 == 1)
    }
    
    /// Generate a random value in range [min, max] (inclusive)
    pub fn range_u32(&self, min: u32, max: u32) -> AdvancedCryptoResult<u32> {
        if min > max {
            return Err("Invalid range: min > max".into());
        }
        
        if min == max {
            return Ok(min);
        }
        
        let range = max - min + 1;
        
        // Use rejection sampling to avoid bias
        let limit = u32::MAX - (u32::MAX % range);
        
        loop {
            let value = self.u32()?;
            if value < limit {
                return Ok(min + (value % range));
            }
        }
    }
    
    /// Generate a random value in range [min, max] (inclusive)
    pub fn range_u64(&self, min: u64, max: u64) -> AdvancedCryptoResult<u64> {
        if min > max {
            return Err("Invalid range: min > max".into());
        }
        
        if min == max {
            return Ok(min);
        }
        
        let range = max - min + 1;
        
        // Use rejection sampling to avoid bias
        let limit = u64::MAX - (u64::MAX % range);
        
        loop {
            let value = self.u64()?;
            if value < limit {
                return Ok(min + (value % range));
            }
        }
    }
    
    /// Generate a random value in range [min, max] (inclusive)
    pub fn range_i32(&self, min: i32, max: i32) -> AdvancedCryptoResult<i32> {
        if min > max {
            return Err("Invalid range: min > max".into());
        }
        
        let min_u = min as u32;
        let max_u = max as u32;
        let range = max_u.wrapping_sub(min_u).wrapping_add(1);
        
        // Use rejection sampling
        let limit = u32::MAX - (u32::MAX % range);
        
        loop {
            let value = self.u32()?;
            if value < limit {
                return Ok(min_u.wrapping_add(value % range) as i32);
            }
        }
    }
    
    /// Generate a random value in range [min, max] (inclusive)
    pub fn range_i64(&self, min: i64, max: i64) -> AdvancedCryptoResult<i64> {
        if min > max {
            return Err("Invalid range: min > max".into());
        }
        
        let min_u = min as u64;
        let max_u = max as u64;
        let range = max_u.wrapping_sub(min_u).wrapping_add(1);
        
        // Use rejection sampling
        let limit = u64::MAX - (u64::MAX % range);
        
        loop {
            let value = self.u64()?;
            if value < limit {
                return Ok(min_u.wrapping_add(value % range) as i64);
            }
        }
    }
    
    /// Generate a random f32 in range [min, max)
    pub fn range_f32(&self, min: f32, max: f32) -> AdvancedCryptoResult<f32> {
        if min >= max {
            return Err("Invalid range: min >= max".into());
        }
        
        let rand_val = self.f32()?;
        Ok(min + rand_val * (max - min))
    }
    
    /// Generate a random f64 in range [min, max)
    pub fn range_f64(&self, min: f64, max: f64) -> AdvancedCryptoResult<f64> {
        if min >= max {
            return Err("Invalid range: min >= max".into());
        }
        
        let rand_val = self.f64()?;
        Ok(min + rand_val * (max - min))
    }
    
    /// Choose a random element from a slice
    pub fn choose<T>(&self, items: &[T]) -> AdvancedCryptoResult<Option<&T>> {
        if items.is_empty() {
            return Ok(None);
        }
        
        let index = self.range_u64(0, items.len() as u64 - 1)?;
        Ok(Some(&items[index as usize]))
    }
    
    /// Shuffle a slice in place using Fisher-Yates algorithm
    pub fn shuffle<T>(&self, items: &mut [T]) -> AdvancedCryptoResult<()> {
        for i in (1..items.len()).rev() {
            let j = self.range_u64(0, i as u64)? as usize;
            items.swap(i, j);
        }
        Ok(())
    }
    
    /// Fill a slice with random bytes
    pub fn fill_bytes(&self, dest: &mut [u8]) -> AdvancedCryptoResult<()> {
        let bytes = self.bytes(dest.len())?;
        dest.copy_from_slice(&bytes);
        Ok(())
    }
    
    /// Force reseed of the underlying CSPRNG
    pub fn reseed(&self) -> AdvancedCryptoResult<()> {
        self.csprng.force_reseed()
    }
    
    /// Check if hardware entropy is available
    pub fn has_hardware_entropy(&self) -> bool {
        self.hardware_collector.as_ref()
            .map(|hc| hc.has_hardware_rng())
            .unwrap_or(false)
    }
    
    /// Get entropy source information
    pub fn get_entropy_info(&self) -> String {
        let mut info = String::new();
        
        info.push_str(&format!("CSPRNG Algorithm: {:?}\n", self.csprng.get_algorithm()));
        info.push_str(&format!("Initialized: {}\n", self.initialized));
        info.push_str(&format!("Auto-test enabled: {}\n", self.auto_test));
        
        if let Some(ref hw) = self.hardware_collector {
            let capabilities = hw.get_capabilities();
            info.push_str(&format!("Hardware RNGs available: {}\n", capabilities.len()));
            for cap in capabilities {
                info.push_str(&format!("  - {:?}: {}\n", cap.rng_type, if cap.available { "Available" } else { "Unavailable" }));
            }
        } else {
            info.push_str("Hardware RNGs: None available\n");
        }
        
        if let Some(ref ec) = self.entropy_collector {
            let stats = ec.lock().unwrap().get_stats();
            info.push_str(&format!("Entropy collected: {} bytes\n", stats.total_entropy_collected));
            info.push_str(&format!("Fast reseeds: {}\n", stats.fast_reseeds));
            info.push_str(&format!("Slow reseeds: {}\n", stats.slow_reseeds));
        }
        
        info
    }
    
    /// Set automatic testing on/off
    pub fn set_auto_test(&mut self, enabled: bool) {
        self.auto_test = enabled;
    }
    
    /// Test randomness quality of generated data
    pub fn test_quality(&self, sample_size: usize) -> AdvancedCryptoResult<String> {
        let sample = self.bytes(sample_size)?;
        let results = self.test_suite.comprehensive_test(&sample)?;
        
        let mut report = String::new();
        report.push_str(&format!("Randomness Quality Test Report\n"));
        report.push_str(&format!("Sample size: {} bytes\n\n", sample_size));
        
        for result in results {
            report.push_str(&format!("Test: {}\n", result.test_name));
            report.push_str(&format!("Result: {}\n", if result.passed { "PASS" } else { "FAIL" }));
            report.push_str(&format!("P-value: {:.6}\n", result.p_value));
            report.push_str(&format!("Details: {}\n\n", result.details));
        }
        
        Ok(report)
    }
}

/// Global functions for convenient access to secure random generation
impl SecureRandom {
    /// Get or initialize the global secure random instance
    fn global() -> &'static Arc<Mutex<SecureRandom>> {
        GLOBAL_SECURE_RNG.get_or_init(|| {
            Arc::new(Mutex::new(
                SecureRandom::new().expect("Failed to initialize global SecureRandom")
            ))
        })
    }
}

/// Generate cryptographically secure random bytes
pub fn secure_bytes(size: usize) -> AdvancedCryptoResult<Vec<u8>> {
    SecureRandom::global().lock().unwrap().bytes(size)
}

/// Generate a cryptographically secure random u32
pub fn secure_u32() -> AdvancedCryptoResult<u32> {
    SecureRandom::global().lock().unwrap().u32()
}

/// Generate a cryptographically secure random u64
pub fn secure_u64() -> AdvancedCryptoResult<u64> {
    SecureRandom::global().lock().unwrap().u64()
}

/// Generate a cryptographically secure random f64 in range [0.0, 1.0)
pub fn secure_f64() -> AdvancedCryptoResult<f64> {
    SecureRandom::global().lock().unwrap().f64()
}

/// Generate a cryptographically secure random bool
pub fn secure_bool() -> AdvancedCryptoResult<bool> {
    SecureRandom::global().lock().unwrap().bool()
}

/// Generate a cryptographically secure random value in range [min, max] (inclusive)
pub fn secure_range_u32(min: u32, max: u32) -> AdvancedCryptoResult<u32> {
    SecureRandom::global().lock().unwrap().range_u32(min, max)
}

/// Generate a cryptographically secure random value in range [min, max] (inclusive)
pub fn secure_range_u64(min: u64, max: u64) -> AdvancedCryptoResult<u64> {
    SecureRandom::global().lock().unwrap().range_u64(min, max)
}

/// Choose a random element from a slice using cryptographically secure random
pub fn secure_choose<T>(items: &[T]) -> AdvancedCryptoResult<Option<&T>> {
    SecureRandom::global().lock().unwrap().choose(items)
}

/// Shuffle a slice in place using cryptographically secure random
pub fn secure_shuffle<T>(items: &mut [T]) -> AdvancedCryptoResult<()> {
    SecureRandom::global().lock().unwrap().shuffle(items)
}

/// Fill a slice with cryptographically secure random bytes
pub fn secure_fill_bytes(dest: &mut [u8]) -> AdvancedCryptoResult<()> {
    SecureRandom::global().lock().unwrap().fill_bytes(dest)
}

/// Force reseed of the global secure random generator
pub fn secure_reseed() -> AdvancedCryptoResult<()> {
    SecureRandom::global().lock().unwrap().reseed()
}

/// Get information about available entropy sources
pub fn secure_entropy_info() -> String {
    SecureRandom::global().lock().unwrap().get_entropy_info()
}

/// Test the quality of the secure random number generator
pub fn secure_test_quality(sample_size: usize) -> AdvancedCryptoResult<String> {
    SecureRandom::global().lock().unwrap().test_quality(sample_size)
}

impl Default for SecureRandom {
    fn default() -> Self {
        Self::new().expect("Failed to create default SecureRandom")
    }
}
