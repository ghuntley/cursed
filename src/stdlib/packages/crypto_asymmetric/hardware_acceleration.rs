//! Hardware acceleration for cryptography
//! 
//! Provides comprehensive hardware acceleration detection and capabilities enumeration.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{__cpuid, __cpuid_count};
#[cfg(target_arch = "x86")]
use std::arch::x86::{__cpuid, __cpuid_count};

/// CPU instruction set features for cryptography
#[derive(Debug, Clone, PartialEq)]
pub struct CpuFeatures {
    pub aes_ni: bool,
    pub sha_extensions: bool,
    pub pclmulqdq: bool,
    pub rdrand: bool,
    pub rdseed: bool,
    pub avx2: bool,
    pub avx512f: bool,
    pub vaes: bool,
    pub vpclmulqdq: bool,
    pub sha512_extensions: bool,
    pub sm3_extensions: bool,
    pub sm4_extensions: bool,
}

impl Default for CpuFeatures {
    fn default() -> Self {
        Self {
            aes_ni: false,
            sha_extensions: false,
            pclmulqdq: false,
            rdrand: false,
            rdseed: false,
            avx2: false,
            avx512f: false,
            vaes: false,
            vpclmulqdq: false,
            sha512_extensions: false,
            sm3_extensions: false,
            sm4_extensions: false,
        }
    }
}

/// Hardware Security Module information
#[derive(Debug, Clone)]
pub struct HsmInfo {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub available: bool,
}

/// GPU acceleration information
#[derive(Debug, Clone)]
pub struct GpuAcceleration {
    pub opencl_available: bool,
    pub cuda_available: bool,
    pub vulkan_compute: bool,
    pub devices: Vec<GpuDevice>,
}

/// GPU device information
#[derive(Debug, Clone)]
pub struct GpuDevice {
    pub name: String,
    pub vendor: String,
    pub compute_units: u32,
    pub memory_mb: u64,
    pub supports_crypto: bool,
}

/// Cryptographic coprocessor information
#[derive(Debug, Clone)]
pub struct CryptoCoprocessor {
    pub name: String,
    pub vendor: String,
    pub capabilities: Vec<String>,
    pub performance_rating: u32,
}

/// Complete hardware acceleration capabilities
#[derive(Debug, Clone)]
pub struct HardwareCapabilities {
    pub cpu_features: CpuFeatures,
    pub hsms: Vec<HsmInfo>,
    pub gpu_acceleration: GpuAcceleration,
    pub crypto_coprocessors: Vec<CryptoCoprocessor>,
    pub detection_time: Duration,
    pub platform: String,
    pub architecture: String,
}

/// Hardware acceleration detector
pub struct HardwareAccelerationDetector {
    cached_capabilities: Arc<Mutex<Option<HardwareCapabilities>>>,
    cache_timestamp: Arc<Mutex<Option<Instant>>>,
    cache_duration: Duration,
}

impl Default for HardwareAccelerationDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareAccelerationDetector {
    /// Create a new hardware acceleration detector
    pub fn new() -> Self {
        Self {
            cached_capabilities: Arc::new(Mutex::new(None)),
            cache_timestamp: Arc::new(Mutex::new(None)),
            cache_duration: Duration::from_secs(300), // 5 minutes cache
        }
    }

    /// Set cache duration
    pub fn set_cache_duration(&mut self, duration: Duration) {
        self.cache_duration = duration;
    }

    /// Get cached capabilities or detect if cache is expired
    pub fn get_capabilities(&self) -> Result<HardwareCapabilities, CursedError> {
        let now = Instant::now();
        
        // Check if we have valid cached data
        {
            let cache_time = self.cache_timestamp.lock().unwrap();
            let cached_caps = self.cached_capabilities.lock().unwrap();
            
            if let (Some(timestamp), Some(capabilities)) = (cache_time.as_ref(), cached_caps.as_ref()) {
                if now.duration_since(*timestamp) < self.cache_duration {
                    return Ok(capabilities.clone());
                }
            }
        }

        // Detect capabilities
        let capabilities = self.detect_capabilities()?;
        
        // Update cache
        {
            let mut cache_time = self.cache_timestamp.lock().unwrap();
            let mut cached_caps = self.cached_capabilities.lock().unwrap();
            
            *cache_time = Some(now);
            *cached_caps = Some(capabilities.clone());
        }

        Ok(capabilities)
    }

    /// Force refresh of capabilities detection
    pub fn refresh_capabilities(&self) -> Result<HardwareCapabilities, CursedError> {
        // Clear cache
        {
            let mut cache_time = self.cache_timestamp.lock().unwrap();
            let mut cached_caps = self.cached_capabilities.lock().unwrap();
            
            *cache_time = None;
            *cached_caps = None;
        }

        self.get_capabilities()
    }

    /// Detect all hardware acceleration capabilities
    fn detect_capabilities(&self) -> Result<HardwareCapabilities, CursedError> {
        let start_time = Instant::now();

        let cpu_features = self.detect_cpu_features()?;
        let hsms = self.detect_hsms()?;
        let gpu_acceleration = self.detect_gpu_acceleration()?;
        let crypto_coprocessors = self.detect_crypto_coprocessors()?;

        let detection_time = start_time.elapsed();

        Ok(HardwareCapabilities {
            cpu_features,
            hsms,
            gpu_acceleration,
            crypto_coprocessors,
            detection_time,
            platform: self.get_platform(),
            architecture: self.get_architecture(),
        })
    }

    /// Detect CPU cryptographic features
    fn detect_cpu_features(&self) -> Result<CpuFeatures, CursedError> {
        let mut features = CpuFeatures::default();

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            features = self.detect_x86_features()?;
        }

        #[cfg(target_arch = "aarch64")]
        {
            features = self.detect_aarch64_features()?;
        }

        #[cfg(target_arch = "arm")]
        {
            features = self.detect_arm_features()?;
        }

        Ok(features)
    }

    /// Detect x86/x86_64 CPU features using CPUID
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn detect_x86_features(&self) -> Result<CpuFeatures, CursedError> {
        let mut features = CpuFeatures::default();

        unsafe {
            // Check for CPUID support
            let cpuid_result = __cpuid(0);
            if cpuid_result.eax == 0 {
                return Ok(features);
            }

            // Check feature flags in CPUID leaf 1
            let leaf1 = __cpuid(1);
            
            // ECX features
            features.aes_ni = (leaf1.ecx & (1 << 25)) != 0;
            features.pclmulqdq = (leaf1.ecx & (1 << 1)) != 0;
            features.rdrand = (leaf1.ecx & (1 << 30)) != 0;

            // Check extended features in CPUID leaf 7
            if cpuid_result.eax >= 7 {
                let leaf7 = __cpuid_count(7, 0);
                
                // EBX features
                features.sha_extensions = (leaf7.ebx & (1 << 29)) != 0;
                features.avx2 = (leaf7.ebx & (1 << 5)) != 0;
                features.avx512f = (leaf7.ebx & (1 << 16)) != 0;
                features.rdseed = (leaf7.ebx & (1 << 18)) != 0;

                // ECX features
                features.vaes = (leaf7.ecx & (1 << 9)) != 0;
                features.vpclmulqdq = (leaf7.ecx & (1 << 10)) != 0;

                // Check for SHA-512 extensions (hypothetical future feature)
                features.sha512_extensions = (leaf7.ecx & (1 << 31)) != 0;

                // Check for Chinese cryptographic extensions
                features.sm3_extensions = (leaf7.edx & (1 << 8)) != 0;
                features.sm4_extensions = (leaf7.edx & (1 << 9)) != 0;
            }
        }

        Ok(features)
    }

    /// Detect AArch64 CPU features
    #[cfg(target_arch = "aarch64")]
    fn detect_aarch64_features(&self) -> Result<CpuFeatures, CursedError> {
        let mut features = CpuFeatures::default();

        // Check for AES and SHA extensions via system registers or /proc/cpuinfo
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            features.aes_ni = cpuinfo.contains("aes");
            features.sha_extensions = cpuinfo.contains("sha1") || cpuinfo.contains("sha2");
            features.rdrand = cpuinfo.contains("rng");
        }

        Ok(features)
    }

    /// Detect ARM CPU features
    #[cfg(target_arch = "arm")]
    fn detect_arm_features(&self) -> Result<CpuFeatures, CursedError> {
        let mut features = CpuFeatures::default();

        // Check for cryptographic extensions via /proc/cpuinfo
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            features.aes_ni = cpuinfo.contains("aes");
            features.sha_extensions = cpuinfo.contains("sha1") || cpuinfo.contains("sha2");
        }

        Ok(features)
    }

    /// Detect Hardware Security Modules
    fn detect_hsms(&self) -> Result<Vec<HsmInfo>, CursedError> {
        let mut hsms = Vec::new();

        // Check for common HSM interfaces
        self.check_pkcs11_hsms(&mut hsms)?;
        self.check_tpm_modules(&mut hsms)?;
        self.check_secure_enclaves(&mut hsms)?;

        Ok(hsms)
    }

    /// Check for PKCS#11 HSMs
    fn check_pkcs11_hsms(&self, hsms: &mut Vec<HsmInfo>) -> Result<(), CursedError> {
        // Common PKCS#11 library paths
        let pkcs11_paths = vec![
            "/usr/lib/pkcs11/",
            "/usr/local/lib/pkcs11/",
            "/opt/pkcs11/lib/",
            "/lib/pkcs11/",
        ];

        for path in pkcs11_paths {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".so") || name.ends_with(".dylib") {
                            hsms.push(HsmInfo {
                                name: name.to_string(),
                                version: "Unknown".to_string(),
                                capabilities: vec!["PKCS#11".to_string()],
                                available: true,
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check for TPM modules
    fn check_tpm_modules(&self, hsms: &mut Vec<HsmInfo>) -> Result<(), CursedError> {
        // Check for TPM 2.0 device
        if std::path::Path::new("/dev/tpm0").exists() {
            hsms.push(HsmInfo {
                name: "TPM 2.0".to_string(),
                version: "2.0".to_string(),
                capabilities: vec!["RSA".to_string(), "ECC".to_string(), "HMAC".to_string()],
                available: true,
            });
        }

        // Check for TPM 1.2 device
        if std::path::Path::new("/dev/tpm1").exists() {
            hsms.push(HsmInfo {
                name: "TPM 1.2".to_string(),
                version: "1.2".to_string(),
                capabilities: vec!["RSA".to_string(), "SHA-1".to_string()],
                available: true,
            });
        }

        Ok(())
    }

    /// Check for secure enclaves
    fn check_secure_enclaves(&self, hsms: &mut Vec<HsmInfo>) -> Result<(), CursedError> {
        // Intel SGX
        if std::path::Path::new("/dev/isgx").exists() || 
           std::path::Path::new("/dev/sgx").exists() {
            hsms.push(HsmInfo {
                name: "Intel SGX".to_string(),
                version: "Unknown".to_string(),
                capabilities: vec!["Secure Enclave".to_string(), "Attestation".to_string()],
                available: true,
            });
        }

        // ARM TrustZone
        if std::path::Path::new("/dev/tee0").exists() {
            hsms.push(HsmInfo {
                name: "ARM TrustZone".to_string(),
                version: "Unknown".to_string(),
                capabilities: vec!["Secure World".to_string(), "Trusted Applications".to_string()],
                available: true,
            });
        }

        Ok(())
    }

    /// Detect GPU acceleration capabilities
    fn detect_gpu_acceleration(&self) -> Result<GpuAcceleration, CursedError> {
        let mut gpu_accel = GpuAcceleration {
            opencl_available: false,
            cuda_available: false,
            vulkan_compute: false,
            devices: Vec::new(),
        };

        // Check for OpenCL
        gpu_accel.opencl_available = self.check_opencl_support()?;
        
        // Check for CUDA
        gpu_accel.cuda_available = self.check_cuda_support()?;
        
        // Check for Vulkan Compute
        gpu_accel.vulkan_compute = self.check_vulkan_compute()?;

        // Enumerate GPU devices
        gpu_accel.devices = self.enumerate_gpu_devices()?;

        Ok(gpu_accel)
    }

    /// Check OpenCL support
    fn check_opencl_support(&self) -> Result<bool, CursedError> {
        // Check for OpenCL libraries
        let opencl_libs = vec![
            "libOpenCL.so.1",
            "libOpenCL.so",
            "OpenCL.dll",
            "/System/Library/Frameworks/OpenCL.framework/OpenCL",
        ];

        for lib in opencl_libs {
            if std::path::Path::new(lib).exists() {
                return Ok(true);
            }
        }

        // Check common library paths
        let lib_paths = vec![
            "/usr/lib/x86_64-linux-gnu/",
            "/usr/lib/",
            "/usr/local/lib/",
            "/opt/cuda/lib64/",
        ];

        for path in lib_paths {
            let opencl_path = format!("{}libOpenCL.so", path);
            if std::path::Path::new(&opencl_path).exists() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Check CUDA support
    fn check_cuda_support(&self) -> Result<bool, CursedError> {
        // Check for CUDA runtime libraries
        let cuda_libs = vec![
            "/usr/local/cuda/lib64/libcudart.so",
            "/opt/cuda/lib64/libcudart.so",
            "/usr/lib/x86_64-linux-gnu/libcudart.so",
        ];

        for lib in cuda_libs {
            if std::path::Path::new(lib).exists() {
                return Ok(true);
            }
        }

        // Check for nvidia-smi
        if let Ok(output) = std::process::Command::new("nvidia-smi").output() {
            return Ok(output.status.success());
        }

        Ok(false)
    }

    /// Check Vulkan Compute support
    fn check_vulkan_compute(&self) -> Result<bool, CursedError> {
        // Check for Vulkan libraries
        let vulkan_libs = vec![
            "libvulkan.so.1",
            "libvulkan.so",
            "vulkan-1.dll",
            "/usr/lib/libvulkan.so",
        ];

        for lib in vulkan_libs {
            if std::path::Path::new(lib).exists() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Enumerate GPU devices
    fn enumerate_gpu_devices(&self) -> Result<Vec<GpuDevice>, CursedError> {
        let mut devices = Vec::new();

        // Try to get NVIDIA GPU information
        if let Ok(output) = std::process::Command::new("nvidia-smi")
            .args(&["--query-gpu=name,memory.total", "--format=csv,noheader,nounits"])
            .output() 
        {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 2 {
                        devices.push(GpuDevice {
                            name: parts[0].trim().to_string(),
                            vendor: "NVIDIA".to_string(),
                            compute_units: 0, // Would need more detailed query
                            memory_mb: parts[1].trim().parse().unwrap_or(0),
                            supports_crypto: true,
                        });
                    }
                }
            }
        }

        // Try to get AMD GPU information from /sys/class/drm
        if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("card") && !name.contains("-") {
                        let device_path = path.join("device");
                        if let Ok(vendor) = std::fs::read_to_string(device_path.join("vendor")) {
                            if vendor.trim() == "0x1002" { // AMD vendor ID
                                devices.push(GpuDevice {
                                    name: "AMD GPU".to_string(),
                                    vendor: "AMD".to_string(),
                                    compute_units: 0,
                                    memory_mb: 0,
                                    supports_crypto: true,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(devices)
    }

    /// Detect cryptographic coprocessors
    fn detect_crypto_coprocessors(&self) -> Result<Vec<CryptoCoprocessor>, CursedError> {
        let mut coprocessors = Vec::new();

        // Check for IBM Crypto Express cards
        self.check_ibm_crypto_express(&mut coprocessors)?;
        
        // Check for Cavium/Marvell crypto cards
        self.check_cavium_crypto(&mut coprocessors)?;
        
        // Check for other PCIe crypto cards
        self.check_pcie_crypto_cards(&mut coprocessors)?;

        Ok(coprocessors)
    }

    /// Check for IBM Crypto Express cards
    fn check_ibm_crypto_express(&self, coprocessors: &mut Vec<CryptoCoprocessor>) -> Result<(), CursedError> {
        // Check for IBM crypto devices in /sys/bus/ap/devices
        if let Ok(entries) = std::fs::read_dir("/sys/bus/ap/devices") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.contains("crypto") {
                        coprocessors.push(CryptoCoprocessor {
                            name: format!("IBM Crypto Express ({})", name),
                            vendor: "IBM".to_string(),
                            capabilities: vec![
                                "RSA".to_string(),
                                "ECC".to_string(),
                                "AES".to_string(),
                                "SHA".to_string(),
                            ],
                            performance_rating: 95,
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Check for Cavium/Marvell crypto cards
    fn check_cavium_crypto(&self, coprocessors: &mut Vec<CryptoCoprocessor>) -> Result<(), CursedError> {
        // Check lspci output for Cavium devices
        if let Ok(output) = std::process::Command::new("lspci").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("Cavium") && (line.contains("crypto") || line.contains("security")) {
                    coprocessors.push(CryptoCoprocessor {
                        name: "Cavium Crypto Accelerator".to_string(),
                        vendor: "Cavium/Marvell".to_string(),
                        capabilities: vec![
                            "RSA".to_string(),
                            "ECC".to_string(),
                            "AES".to_string(),
                            "3DES".to_string(),
                            "SHA".to_string(),
                        ],
                        performance_rating: 90,
                    });
                }
            }
        }

        Ok(())
    }

    /// Check for other PCIe crypto cards
    fn check_pcie_crypto_cards(&self, coprocessors: &mut Vec<CryptoCoprocessor>) -> Result<(), CursedError> {
        // Check for various crypto accelerator vendors
        if let Ok(output) = std::process::Command::new("lspci").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                // Check for various crypto accelerator keywords
                if line.contains("crypto") || line.contains("security") || line.contains("accelerator") {
                    if line.contains("Intel") {
                        coprocessors.push(CryptoCoprocessor {
                            name: "Intel Crypto Accelerator".to_string(),
                            vendor: "Intel".to_string(),
                            capabilities: vec!["AES".to_string(), "SHA".to_string()],
                            performance_rating: 85,
                        });
                    } else if line.contains("Broadcom") {
                        coprocessors.push(CryptoCoprocessor {
                            name: "Broadcom Crypto Accelerator".to_string(),
                            vendor: "Broadcom".to_string(),
                            capabilities: vec!["AES".to_string(), "RSA".to_string()],
                            performance_rating: 80,
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Get platform string
    fn get_platform(&self) -> String {
        std::env::consts::OS.to_string()
    }

    /// Get architecture string
    fn get_architecture(&self) -> String {
        std::env::consts::ARCH.to_string()
    }
}

/// Global hardware acceleration detector instance
static HARDWARE_DETECTOR: OnceLock<HardwareAccelerationDetector> = OnceLock::new();

/// Get the global hardware detector instance
pub fn get_hardware_detector() -> &'static HardwareAccelerationDetector {
    HARDWARE_DETECTOR.get_or_init(|| HardwareAccelerationDetector::new())
}

/// Check hardware acceleration support
pub fn check_hardware_support(_args: Vec<Value>) -> Result<Value, CursedError> {
    let detector = get_hardware_detector();
    let capabilities = detector.get_capabilities()?;
    
    // Convert capabilities to CURSED Value
    let mut result = HashMap::new();
    
    // CPU features
    let mut cpu_features = HashMap::new();
    cpu_features.insert("aes_ni".to_string(), Value::Bool(capabilities.cpu_features.aes_ni));
    cpu_features.insert("sha_extensions".to_string(), Value::Bool(capabilities.cpu_features.sha_extensions));
    cpu_features.insert("pclmulqdq".to_string(), Value::Bool(capabilities.cpu_features.pclmulqdq));
    cpu_features.insert("rdrand".to_string(), Value::Bool(capabilities.cpu_features.rdrand));
    cpu_features.insert("rdseed".to_string(), Value::Bool(capabilities.cpu_features.rdseed));
    cpu_features.insert("avx2".to_string(), Value::Bool(capabilities.cpu_features.avx2));
    cpu_features.insert("avx512f".to_string(), Value::Bool(capabilities.cpu_features.avx512f));
    cpu_features.insert("vaes".to_string(), Value::Bool(capabilities.cpu_features.vaes));
    cpu_features.insert("vpclmulqdq".to_string(), Value::Bool(capabilities.cpu_features.vpclmulqdq));
    result.insert("cpu_features".to_string(), Value::Object(cpu_features));

    // HSMs
    let hsm_list: Vec<Value> = capabilities.hsms.iter().map(|hsm| {
        let mut hsm_map = HashMap::new();
        hsm_map.insert("name".to_string(), Value::String(hsm.name.clone()));
        hsm_map.insert("version".to_string(), Value::String(hsm.version.clone()));
        hsm_map.insert("available".to_string(), Value::Bool(hsm.available));
        hsm_map.insert("capabilities".to_string(), Value::Array(
            hsm.capabilities.iter().map(|c| Value::String(c.clone())).collect()
        ));
        Value::Object(hsm_map)
    }).collect();
    result.insert("hsms".to_string(), Value::Array(hsm_list));

    // GPU acceleration
    let mut gpu_accel = HashMap::new();
    gpu_accel.insert("opencl_available".to_string(), Value::Bool(capabilities.gpu_acceleration.opencl_available));
    gpu_accel.insert("cuda_available".to_string(), Value::Bool(capabilities.gpu_acceleration.cuda_available));
    gpu_accel.insert("vulkan_compute".to_string(), Value::Bool(capabilities.gpu_acceleration.vulkan_compute));
    
    let gpu_devices: Vec<Value> = capabilities.gpu_acceleration.devices.iter().map(|device| {
        let mut device_map = HashMap::new();
        device_map.insert("name".to_string(), Value::String(device.name.clone()));
        device_map.insert("vendor".to_string(), Value::String(device.vendor.clone()));
        device_map.insert("compute_units".to_string(), Value::Number(device.compute_units as f64));
        device_map.insert("memory_mb".to_string(), Value::Number(device.memory_mb as f64));
        device_map.insert("supports_crypto".to_string(), Value::Bool(device.supports_crypto));
        Value::Object(device_map)
    }).collect();
    gpu_accel.insert("devices".to_string(), Value::Array(gpu_devices));
    result.insert("gpu_acceleration".to_string(), Value::Object(gpu_accel));

    // Crypto coprocessors
    let coprocessor_list: Vec<Value> = capabilities.crypto_coprocessors.iter().map(|cp| {
        let mut cp_map = HashMap::new();
        cp_map.insert("name".to_string(), Value::String(cp.name.clone()));
        cp_map.insert("vendor".to_string(), Value::String(cp.vendor.clone()));
        cp_map.insert("performance_rating".to_string(), Value::Number(cp.performance_rating as f64));
        cp_map.insert("capabilities".to_string(), Value::Array(
            cp.capabilities.iter().map(|c| Value::String(c.clone())).collect()
        ));
        Value::Object(cp_map)
    }).collect();
    result.insert("crypto_coprocessors".to_string(), Value::Array(coprocessor_list));

    // Metadata
    result.insert("platform".to_string(), Value::String(capabilities.platform));
    result.insert("architecture".to_string(), Value::String(capabilities.architecture));
    result.insert("detection_time_ms".to_string(), Value::Number(capabilities.detection_time.as_millis() as f64));

    Ok(Value::Object(result))
}

/// Get specific CPU feature availability
pub fn has_cpu_feature(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() != 1 {
        return Err(CursedError::InvalidArguments("Expected 1 argument (feature_name)".to_string()));
    }

    let feature_name = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::InvalidArguments("Feature name must be a string".to_string())),
    };

    let detector = get_hardware_detector();
    let capabilities = detector.get_capabilities()?;
    
    let has_feature = match feature_name.as_str() {
        "aes_ni" => capabilities.cpu_features.aes_ni,
        "sha_extensions" => capabilities.cpu_features.sha_extensions,
        "pclmulqdq" => capabilities.cpu_features.pclmulqdq,
        "rdrand" => capabilities.cpu_features.rdrand,
        "rdseed" => capabilities.cpu_features.rdseed,
        "avx2" => capabilities.cpu_features.avx2,
        "avx512f" => capabilities.cpu_features.avx512f,
        "vaes" => capabilities.cpu_features.vaes,
        "vpclmulqdq" => capabilities.cpu_features.vpclmulqdq,
        _ => false,
    };

    Ok(Value::Bool(has_feature))
}

/// Get available HSMs
pub fn get_available_hsms(_args: Vec<Value>) -> Result<Value, CursedError> {
    let detector = get_hardware_detector();
    let capabilities = detector.get_capabilities()?;
    
    let hsm_list: Vec<Value> = capabilities.hsms.iter()
        .filter(|hsm| hsm.available)
        .map(|hsm| Value::String(hsm.name.clone()))
        .collect();
    
    Ok(Value::Array(hsm_list))
}

/// Refresh hardware detection cache
pub fn refresh_hardware_detection(_args: Vec<Value>) -> Result<Value, CursedError> {
    let detector = get_hardware_detector();
    let capabilities = detector.refresh_capabilities()?;
    
    Ok(Value::String(format!(
        "Hardware detection refreshed. Found {} CPU features, {} HSMs, {} GPU devices, {} crypto coprocessors", 
        if capabilities.cpu_features.aes_ni { 1 } else { 0 } + 
        if capabilities.cpu_features.sha_extensions { 1 } else { 0 } +
        if capabilities.cpu_features.pclmulqdq { 1 } else { 0 },
        capabilities.hsms.len(),
        capabilities.gpu_acceleration.devices.len(),
        capabilities.crypto_coprocessors.len()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_detector_creation() {
        let detector = HardwareAccelerationDetector::new();
        assert!(detector.cached_capabilities.lock().unwrap().is_none());
    }

    #[test]
    fn test_cpu_features_default() {
        let features = CpuFeatures::default();
        assert!(!features.aes_ni);
        assert!(!features.sha_extensions);
        assert!(!features.pclmulqdq);
    }

    #[test]
    fn test_get_platform_and_architecture() {
        let detector = HardwareAccelerationDetector::new();
        let platform = detector.get_platform();
        let architecture = detector.get_architecture();
        
        assert!(!platform.is_empty());
        assert!(!architecture.is_empty());
    }

    #[test]
    fn test_capabilities_detection() {
        let detector = HardwareAccelerationDetector::new();
        let result = detector.get_capabilities();
        
        // Should succeed even if no acceleration is available
        assert!(result.is_ok());
        
        let capabilities = result.unwrap();
        assert!(!capabilities.platform.is_empty());
        assert!(!capabilities.architecture.is_empty());
    }

    #[test]
    fn test_cache_functionality() {
        let detector = HardwareAccelerationDetector::new();
        
        // First call should detect
        let caps1 = detector.get_capabilities().unwrap();
        
        // Second call should use cache
        let caps2 = detector.get_capabilities().unwrap();
        
        // Should be the same (cached)
        assert_eq!(caps1.platform, caps2.platform);
        assert_eq!(caps1.architecture, caps2.architecture);
    }

    #[test]
    fn test_refresh_capabilities() {
        let detector = HardwareAccelerationDetector::new();
        
        // Get initial capabilities
        let _caps1 = detector.get_capabilities().unwrap();
        
        // Refresh should work
        let caps2 = detector.refresh_capabilities().unwrap();
        assert!(!caps2.platform.is_empty());
    }

    #[test]
    fn test_check_hardware_support_function() {
        let result = check_hardware_support(vec![]);
        assert!(result.is_ok());
        
        if let Value::Object(map) = result.unwrap() {
            assert!(map.contains_key("cpu_features"));
            assert!(map.contains_key("hsms"));
            assert!(map.contains_key("gpu_acceleration"));
            assert!(map.contains_key("crypto_coprocessors"));
            assert!(map.contains_key("platform"));
            assert!(map.contains_key("architecture"));
        } else {
            panic!("Expected object result");
        }
    }

    #[test]
    fn test_has_cpu_feature_function() {
        let result = has_cpu_feature(vec![Value::String("aes_ni".to_string())]);
        assert!(result.is_ok());
        
        if let Value::Bool(_) = result.unwrap() {
            // Result should be a boolean
        } else {
            panic!("Expected boolean result");
        }
    }

    #[test]
    fn test_has_cpu_feature_invalid_args() {
        let result = has_cpu_feature(vec![]);
        assert!(result.is_err());
        
        let result = has_cpu_feature(vec![Value::Number(42.0)]);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_available_hsms_function() {
        let result = get_available_hsms(vec![]);
        assert!(result.is_ok());
        
        if let Value::Array(_) = result.unwrap() {
            // Result should be an array
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_refresh_hardware_detection_function() {
        let result = refresh_hardware_detection(vec![]);
        assert!(result.is_ok());
        
        if let Value::String(msg) = result.unwrap() {
            assert!(msg.contains("Hardware detection refreshed"));
        } else {
            panic!("Expected string result");
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[test]
    fn test_x86_feature_detection() {
        let detector = HardwareAccelerationDetector::new();
        let result = detector.detect_x86_features();
        
        // Should succeed on x86 platforms
        assert!(result.is_ok());
    }

    #[test]
    fn test_hsm_detection() {
        let detector = HardwareAccelerationDetector::new();
        let result = detector.detect_hsms();
        
        // Should succeed even if no HSMs are found
        assert!(result.is_ok());
    }

    #[test]
    fn test_gpu_acceleration_detection() {
        let detector = HardwareAccelerationDetector::new();
        let result = detector.detect_gpu_acceleration();
        
        // Should succeed even if no GPU acceleration is available
        assert!(result.is_ok());
    }

    #[test]
    fn test_crypto_coprocessor_detection() {
        let detector = HardwareAccelerationDetector::new();
        let result = detector.detect_crypto_coprocessors();
        
        // Should succeed even if no coprocessors are found
        assert!(result.is_ok());
    }
}
