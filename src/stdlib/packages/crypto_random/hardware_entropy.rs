/// Hardware entropy integration for cryptographically secure random number generation
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// Hardware RNG types
#[derive(Debug, Clone, PartialEq)]
pub enum HardwareRngType {
    IntelRdrand,    // Intel RDRAND instruction
    IntelRdseed,    // Intel RDSEED instruction
    ArmTrng,        // ARM True Random Number Generator
    TpmRng,         // TPM (Trusted Platform Module) RNG
    ChaChaTng,      // ChaCha-based hardware RNG
    Custom(String), // Custom hardware RNG
/// Hardware RNG capabilities
#[derive(Debug, Clone)]
pub struct HardwareCapabilities {
    pub estimated_entropy_rate: f64, // bits per byte
/// Hardware RNG statistics
#[derive(Debug, Clone, Default)]
pub struct HardwareStats {
/// Hardware entropy collector
pub struct HardwareEntropyCollector {
impl HardwareEntropyCollector {
    /// Create new hardware entropy collector
    pub fn new() -> Self {
        let mut collector = Self {
        
        collector.detect_hardware_rngs();
        collector
    /// Detect available hardware RNGs
    fn detect_hardware_rngs(&mut self) {
        // Detect Intel RDRAND
        if self.detect_intel_rdrand() {
            self.capabilities.push(HardwareCapabilities {
                estimated_entropy_rate: 7.5, // Conservative estimate
            });
        // Detect Intel RDSEED
        if self.detect_intel_rdseed() {
            self.capabilities.push(HardwareCapabilities {
                estimated_entropy_rate: 7.8, // Higher entropy than RDRAND
            });
        // Detect ARM TRNG
        if self.detect_arm_trng() {
            self.capabilities.push(HardwareCapabilities {
            });
        // Detect TPM RNG
        if self.detect_tpm_rng() {
            self.capabilities.push(HardwareCapabilities {
            });
        // Set preferred RNG (prioritize RDSEED > RDRAND > ARM TRNG > TPM)
        if self.has_capability(&HardwareRngType::IntelRdseed) {
            self.preferred_rng = Some(HardwareRngType::IntelRdseed);
        } else if self.has_capability(&HardwareRngType::IntelRdrand) {
            self.preferred_rng = Some(HardwareRngType::IntelRdrand);
        } else if self.has_capability(&HardwareRngType::ArmTrng) {
            self.preferred_rng = Some(HardwareRngType::ArmTrng);
        } else if self.has_capability(&HardwareRngType::TpmRng) {
            self.preferred_rng = Some(HardwareRngType::TpmRng);
        }
    }
    
    /// Detect Intel RDRAND support
    fn detect_intel_rdrand(&self) -> bool {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            // Check CPUID for RDRAND support
            #[cfg(target_arch = "x86_64")]
            {
                use std::arch::x86_64::__cpuid;
                let cpuid = unsafe { __cpuid(1) };
                (cpuid.ecx & (1 << 30)) != 0
            }
            #[cfg(target_arch = "x86")]
            {
                use std::arch::x86::__cpuid;
                let cpuid = unsafe { __cpuid(1) };
                (cpuid.ecx & (1 << 30)) != 0
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            false
        }
    }
    
    /// Detect Intel RDSEED support
    fn detect_intel_rdseed(&self) -> bool {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            // Check CPUID for RDSEED support
            #[cfg(target_arch = "x86_64")]
            {
                use std::arch::x86_64::__cpuid;
                let cpuid = unsafe { __cpuid(7) };
                (cpuid.ebx & (1 << 18)) != 0
            }
            #[cfg(target_arch = "x86")]
            {
                use std::arch::x86::__cpuid;
                let cpuid = unsafe { __cpuid(7) };
                (cpuid.ebx & (1 << 18)) != 0
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            false
        }
    }
    
    /// Detect ARM TRNG support
    fn detect_arm_trng(&self) -> bool {
        #[cfg(target_arch = "aarch64")]
        {
            // Check for ARM TrustZone TRNG support
            // This is a simplified check - real implementation would probe hardware
            std::path::Path::new("/dev/hwrng").exists()
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            false
        }
    }
    
    /// Detect TPM RNG support
    fn detect_tpm_rng(&self) -> bool {
        // Check for TPM device
        std::path::Path::new("/dev/tpm0").exists() || 
        std::path::Path::new("/dev/tpmrm0").exists()
    /// Check if hardware capability is available
    pub fn has_capability(&self, rng_type: &HardwareRngType) -> bool {
        self.capabilities.iter()
            .any(|cap| &cap.rng_type == rng_type && cap.available)
    /// Get hardware capabilities
    pub fn get_capabilities(&self) -> &[HardwareCapabilities] {
        &self.capabilities
    /// Generate random bytes using hardware RNG
    pub fn generate_bytes(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        if let Some(ref preferred) = self.preferred_rng {
            match self.generate_bytes_with_rng(preferred, size) {
                Err(_) if self.fallback_enabled => {
                    // Try other available RNGs
                    for capability in &self.capabilities {
                        if &capability.rng_type != preferred && capability.available {
                            if let Ok(bytes) = self.generate_bytes_with_rng(&capability.rng_type, size) {
                                return Ok(bytes);
                            }
                        }
                    }
                }
            }
        }
        
        Err("No hardware RNG available".into())
    /// Generate random bytes using specific RNG type
    pub fn generate_bytes_with_rng(&self, rng_type: &HardwareRngType, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        let start_time = SystemTime::now();
        
        let result = match rng_type {
            HardwareRngType::Custom(name) => {
                return Err(format!("Custom hardware RNG '{}' not implemented", name).into());
            }
        
        // Update statistics
        let call_time = start_time.elapsed().unwrap_or(Duration::from_millis(1));
        self.update_stats(&result, call_time);
        
        result
    /// Generate random bytes using Intel RDRAND
    fn generate_intel_rdrand(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if !self.detect_intel_rdrand() {
                return Err("Intel RDRAND not available".into());
            let mut buffer = vec![0u8; size];
            let mut offset = 0;
            
            #[cfg(target_arch = "x86_64")]
            {
                use std::arch::x86_64::_rdrand64_step;
                
                while offset < size {
                    let mut value = 0u64;
                    let success = unsafe { _rdrand64_step(&mut value) };
                    
                    if success == 0 {
                        return Err("Intel RDRAND failed to generate random value".into());
                    let bytes = value.to_le_bytes();
                    let copy_len = std::cmp::min(8, size - offset);
                    buffer[offset..offset + copy_len].copy_from_slice(&bytes[..copy_len]);
                    offset += copy_len;
                }
            }
            
            #[cfg(target_arch = "x86")]
            {
                use std::arch::x86::_rdrand32_step;
                
                while offset < size {
                    let mut value = 0u32;
                    let success = unsafe { _rdrand32_step(&mut value) };
                    
                    if success == 0 {
                        return Err("Intel RDRAND failed to generate random value".into());
                    let bytes = value.to_le_bytes();
                    let copy_len = std::cmp::min(4, size - offset);
                    buffer[offset..offset + copy_len].copy_from_slice(&bytes[..copy_len]);
                    offset += copy_len;
                }
            }
            
            Ok(buffer)
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            Err("Intel RDRAND not supported on this architecture".into())
        }
    }
    
    /// Generate random bytes using Intel RDSEED
    fn generate_intel_rdseed(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if !self.detect_intel_rdseed() {
                return Err("Intel RDSEED not available".into());
            let mut buffer = vec![0u8; size];
            let mut offset = 0;
            
            #[cfg(target_arch = "x86_64")]
            {
                use std::arch::x86_64::_rdseed64_step;
                
                while offset < size {
                    let mut value = 0u64;
                    
                    // RDSEED may take time to accumulate entropy, retry if needed
                    let mut retries = 10;
                    let mut success = 0;
                    
                    while retries > 0 && success == 0 {
                        success = unsafe { _rdseed64_step(&mut value) };
                        if success == 0 {
                            retries -= 1;
                            std::hint::spin_loop(); // Brief pause
                        }
                    }
                    
                    if success == 0 {
                        return Err("Intel RDSEED failed to generate random value after retries".into());
                    let bytes = value.to_le_bytes();
                    let copy_len = std::cmp::min(8, size - offset);
                    buffer[offset..offset + copy_len].copy_from_slice(&bytes[..copy_len]);
                    offset += copy_len;
                }
            }
            
            #[cfg(target_arch = "x86")]
            {
                use std::arch::x86::_rdseed32_step;
                
                while offset < size {
                    let mut value = 0u32;
                    
                    let mut retries = 10;
                    let mut success = 0;
                    
                    while retries > 0 && success == 0 {
                        success = unsafe { _rdseed32_step(&mut value) };
                        if success == 0 {
                            retries -= 1;
                            std::hint::spin_loop();
                        }
                    }
                    
                    if success == 0 {
                        return Err("Intel RDSEED failed to generate random value after retries".into());
                    let bytes = value.to_le_bytes();
                    let copy_len = std::cmp::min(4, size - offset);
                    buffer[offset..offset + copy_len].copy_from_slice(&bytes[..copy_len]);
                    offset += copy_len;
                }
            }
            
            Ok(buffer)
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            Err("Intel RDSEED not supported on this architecture".into())
        }
    }
    
    /// Generate random bytes using ARM TRNG
    fn generate_arm_trng(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        #[cfg(target_arch = "aarch64")]
        {
            use std::fs::File;
            use std::io::Read;
            
            // Try to read from hardware RNG device
            if let Ok(mut file) = File::open("/dev/hwrng") {
                let mut buffer = vec![0u8; size];
                match file.read_exact(&mut buffer) {
                }
            }
            
            Err("ARM TRNG device not available".into())
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            Err("ARM TRNG not supported on this architecture".into())
        }
    }
    
    /// Generate random bytes using TPM RNG
    fn generate_tpm_rng(&self, size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        use std::fs::File;
        use std::io::Read;
        
        // Try TPM device files
        let tpm_devices = ["/dev/tpmrm0", "/dev/tpm0"];
        
        for device in &tpm_devices {
            if let Ok(mut file) = File::open(device) {
                let mut buffer = vec![0u8; size];
                match file.read_exact(&mut buffer) {
                }
            }
        Err("TPM RNG device not available".into())
    /// Generate random bytes using ChaCha TRNG (software fallback)
    fn generate_chacha_trng(&self, _size: usize) -> AdvancedCryptoResult<Vec<u8>> {
        // This would implement a ChaCha-based TRNG
        // For now, return error as not implemented
        Err("ChaCha TRNG not yet implemented".into())
    /// Update hardware RNG statistics
    fn update_stats(&self, result: &AdvancedCryptoResult<Vec<u8>>, call_time: Duration) {
        let mut stats = self.stats.lock().unwrap();
        
        stats.total_calls += 1;
        
        match result {
            Ok(data) => {
                stats.total_bytes_generated += data.len() as u64;
                stats.last_successful_call = Some(SystemTime::now());
                stats.consecutive_failures = 0;
            }
            Err(_) => {
                stats.failed_calls += 1;
                stats.last_failure = Some(SystemTime::now());
                stats.consecutive_failures += 1;
            }
        }
        
        // Update average call time
        let total_time = stats.average_call_time.as_nanos() as f64 * (stats.total_calls - 1) as f64;
        let new_total_time = total_time + call_time.as_nanos() as f64;
        stats.average_call_time = Duration::from_nanos((new_total_time / stats.total_calls as f64) as u64);
    /// Get hardware RNG statistics
    pub fn get_stats(&self) -> HardwareStats {
        self.stats.lock().unwrap().clone()
    /// Test hardware RNG availability
    pub fn test_rng(&self, rng_type: &HardwareRngType) -> bool {
        match self.generate_bytes_with_rng(rng_type, 32) {
        }
    }
    
    /// Test all available hardware RNGs
    pub fn test_all_rngs(&self) -> Vec<(HardwareRngType, bool)> {
        self.capabilities.iter()
            .map(|cap| (cap.rng_type.clone(), self.test_rng(&cap.rng_type)))
            .collect()
    /// Set preferred RNG type
    pub fn set_preferred_rng(&mut self, rng_type: Option<HardwareRngType>) {
        if let Some(ref rng) = rng_type {
            if self.has_capability(rng) {
                self.preferred_rng = rng_type;
            }
        } else {
            self.preferred_rng = None;
        }
    }
    
    /// Enable or disable fallback to other RNGs
    pub fn set_fallback_enabled(&mut self, enabled: bool) {
        self.fallback_enabled = enabled;
    /// Get preferred RNG type
    pub fn get_preferred_rng(&self) -> Option<&HardwareRngType> {
        self.preferred_rng.as_ref()
    /// Check if any hardware RNG is available
    pub fn has_hardware_rng(&self) -> bool {
        !self.capabilities.is_empty()
    /// Get best available hardware RNG (by entropy rate)
    pub fn get_best_rng(&self) -> Option<&HardwareCapabilities> {
        self.capabilities.iter()
            .filter(|cap| cap.available)
            .max_by(|a, b| a.estimated_entropy_rate.partial_cmp(&b.estimated_entropy_rate).unwrap())
    /// Refresh hardware detection
    pub fn refresh_detection(&mut self) {
        self.capabilities.clear();
        self.detect_hardware_rngs();
    }
}

impl Default for HardwareEntropyCollector {
    fn default() -> Self {
        Self::new()
    }
}
