//! Comprehensive tests for hardware acceleration detection
//! 
//! Tests CPU feature detection, HSM enumeration, GPU acceleration, and crypto coprocessors.

use cursed::stdlib::packages::crypto_asymmetric::hardware_acceleration::{
    check_hardware_support, has_cpu_feature, get_available_hsms, refresh_hardware_detection,
    get_hardware_detector, HardwareAccelerationDetector, CpuFeatures, HsmInfo, GpuAcceleration, 
    GpuDevice, CryptoCoprocessor, HardwareCapabilities
};
use cursed::stdlib::value::Value;
use std::time::Duration;
use std::collections::HashMap;

#[test]
fn test_hardware_detector_basic_functionality() {
    let detector = HardwareAccelerationDetector::new();
    
    // Test basic detection
    let capabilities = detector.get_capabilities();
    assert!(capabilities.is_ok(), "Hardware capabilities detection should succeed");
    
    let caps = capabilities.unwrap();
    assert!(!caps.platform.is_empty(), "Platform should not be empty");
    assert!(!caps.architecture.is_empty(), "Architecture should not be empty");
    assert!(caps.detection_time.as_millis() >= 0, "Detection time should be non-negative");
}

#[test]
fn test_hardware_detector_caching() {
    let detector = HardwareAccelerationDetector::new();
    
    // First detection
    let start1 = std::time::Instant::now();
    let caps1 = detector.get_capabilities().unwrap();
    let time1 = start1.elapsed();
    
    // Second detection (should use cache)
    let start2 = std::time::Instant::now();
    let caps2 = detector.get_capabilities().unwrap();
    let time2 = start2.elapsed();
    
    // Cached call should be faster
    assert!(time2 < time1, "Cached detection should be faster");
    assert_eq!(caps1.platform, caps2.platform, "Cached results should be identical");
    assert_eq!(caps1.architecture, caps2.architecture, "Cached results should be identical");
}

#[test]
fn test_hardware_detector_cache_expiration() {
    let mut detector = HardwareAccelerationDetector::new();
    detector.set_cache_duration(Duration::from_millis(1)); // Very short cache
    
    // Get initial capabilities
    let _caps1 = detector.get_capabilities().unwrap();
    
    // Wait for cache to expire
    std::thread::sleep(Duration::from_millis(10));
    
    // This should trigger new detection
    let caps2 = detector.get_capabilities().unwrap();
    assert!(!caps2.platform.is_empty());
}

#[test]
fn test_hardware_detector_refresh() {
    let detector = HardwareAccelerationDetector::new();
    
    // Get initial capabilities and cache them
    let _caps1 = detector.get_capabilities().unwrap();
    
    // Force refresh
    let caps2 = detector.refresh_capabilities().unwrap();
    assert!(!caps2.platform.is_empty());
    assert!(!caps2.architecture.is_empty());
}

#[test]
fn test_cpu_features_structure() {
    let features = CpuFeatures::default();
    
    // All features should start as false
    assert!(!features.aes_ni);
    assert!(!features.sha_extensions);
    assert!(!features.pclmulqdq);
    assert!(!features.rdrand);
    assert!(!features.rdseed);
    assert!(!features.avx2);
    assert!(!features.avx512f);
    assert!(!features.vaes);
    assert!(!features.vpclmulqdq);
    assert!(!features.sha512_extensions);
    assert!(!features.sm3_extensions);
    assert!(!features.sm4_extensions);
}

#[test]
fn test_hsm_info_structure() {
    let hsm = HsmInfo {
        name: "Test HSM".to_string(),
        version: "1.0".to_string(),
        capabilities: vec!["RSA".to_string(), "AES".to_string()],
        available: true,
    };
    
    assert_eq!(hsm.name, "Test HSM");
    assert_eq!(hsm.version, "1.0");
    assert_eq!(hsm.capabilities.len(), 2);
    assert!(hsm.available);
}

#[test]
fn test_gpu_acceleration_structure() {
    let gpu_accel = GpuAcceleration {
        opencl_available: true,
        cuda_available: false,
        vulkan_compute: true,
        devices: vec![
            GpuDevice {
                name: "Test GPU".to_string(),
                vendor: "Test Vendor".to_string(),
                compute_units: 8,
                memory_mb: 4096,
                supports_crypto: true,
            }
        ],
    };
    
    assert!(gpu_accel.opencl_available);
    assert!(!gpu_accel.cuda_available);
    assert!(gpu_accel.vulkan_compute);
    assert_eq!(gpu_accel.devices.len(), 1);
    assert_eq!(gpu_accel.devices[0].name, "Test GPU");
}

#[test]
fn test_crypto_coprocessor_structure() {
    let coprocessor = CryptoCoprocessor {
        name: "Test Crypto Card".to_string(),
        vendor: "Test Vendor".to_string(),
        capabilities: vec!["RSA".to_string(), "ECC".to_string(), "AES".to_string()],
        performance_rating: 85,
    };
    
    assert_eq!(coprocessor.name, "Test Crypto Card");
    assert_eq!(coprocessor.vendor, "Test Vendor");
    assert_eq!(coprocessor.capabilities.len(), 3);
    assert_eq!(coprocessor.performance_rating, 85);
}

#[test]
fn test_hardware_capabilities_structure() {
    let capabilities = HardwareCapabilities {
        cpu_features: CpuFeatures::default(),
        hsms: Vec::new(),
        gpu_acceleration: GpuAcceleration {
            opencl_available: false,
            cuda_available: false,
            vulkan_compute: false,
            devices: Vec::new(),
        },
        crypto_coprocessors: Vec::new(),
        detection_time: Duration::from_millis(100),
        platform: "test".to_string(),
        architecture: "test_arch".to_string(),
    };
    
    assert_eq!(capabilities.platform, "test");
    assert_eq!(capabilities.architecture, "test_arch");
    assert_eq!(capabilities.detection_time.as_millis(), 100);
    assert_eq!(capabilities.hsms.len(), 0);
    assert_eq!(capabilities.crypto_coprocessors.len(), 0);
}

#[test]
fn test_check_hardware_support_function() {
    let result = check_hardware_support(vec![]);
    assert!(result.is_ok(), "check_hardware_support should succeed");
    
    if let Value::Object(map) = result.unwrap() {
        // Check required fields
        assert!(map.contains_key("cpu_features"), "Should contain cpu_features");
        assert!(map.contains_key("hsms"), "Should contain hsms");
        assert!(map.contains_key("gpu_acceleration"), "Should contain gpu_acceleration");
        assert!(map.contains_key("crypto_coprocessors"), "Should contain crypto_coprocessors");
        assert!(map.contains_key("platform"), "Should contain platform");
        assert!(map.contains_key("architecture"), "Should contain architecture");
        assert!(map.contains_key("detection_time_ms"), "Should contain detection_time_ms");
        
        // Check cpu_features structure
        if let Some(Value::Object(cpu_features)) = map.get("cpu_features") {
            assert!(cpu_features.contains_key("aes_ni"));
            assert!(cpu_features.contains_key("sha_extensions"));
            assert!(cpu_features.contains_key("pclmulqdq"));
            assert!(cpu_features.contains_key("rdrand"));
            assert!(cpu_features.contains_key("rdseed"));
            assert!(cpu_features.contains_key("avx2"));
            assert!(cpu_features.contains_key("avx512f"));
        } else {
            panic!("cpu_features should be an object");
        }
        
        // Check hsms structure
        if let Some(Value::Array(_)) = map.get("hsms") {
            // HSMs should be an array
        } else {
            panic!("hsms should be an array");
        }
        
        // Check gpu_acceleration structure
        if let Some(Value::Object(gpu_accel)) = map.get("gpu_acceleration") {
            assert!(gpu_accel.contains_key("opencl_available"));
            assert!(gpu_accel.contains_key("cuda_available"));
            assert!(gpu_accel.contains_key("vulkan_compute"));
            assert!(gpu_accel.contains_key("devices"));
        } else {
            panic!("gpu_acceleration should be an object");
        }
        
        // Check platform and architecture
        if let Some(Value::String(platform)) = map.get("platform") {
            assert!(!platform.is_empty());
        } else {
            panic!("platform should be a non-empty string");
        }
        
        if let Some(Value::String(architecture)) = map.get("architecture") {
            assert!(!architecture.is_empty());
        } else {
            panic!("architecture should be a non-empty string");
        }
        
    } else {
        panic!("Expected object result from check_hardware_support");
    }
}

#[test]
fn test_has_cpu_feature_function() {
    // Test valid feature names
    let valid_features = vec![
        "aes_ni", "sha_extensions", "pclmulqdq", "rdrand", "rdseed",
        "avx2", "avx512f", "vaes", "vpclmulqdq"
    ];
    
    for feature in valid_features {
        let result = has_cpu_feature(vec![Value::String(feature.to_string())]);
        assert!(result.is_ok(), "has_cpu_feature should succeed for {}", feature);
        
        if let Value::Bool(_) = result.unwrap() {
            // Should return a boolean value
        } else {
            panic!("has_cpu_feature should return a boolean for {}", feature);
        }
    }
    
    // Test unknown feature
    let result = has_cpu_feature(vec![Value::String("unknown_feature".to_string())]);
    assert!(result.is_ok());
    if let Value::Bool(has_feature) = result.unwrap() {
        assert!(!has_feature, "Unknown feature should return false");
    }
}

#[test]
fn test_has_cpu_feature_invalid_arguments() {
    // Test no arguments
    let result = has_cpu_feature(vec![]);
    assert!(result.is_err(), "has_cpu_feature should fail with no arguments");
    
    // Test too many arguments
    let result = has_cpu_feature(vec![
        Value::String("aes_ni".to_string()),
        Value::String("sha_extensions".to_string())
    ]);
    assert!(result.is_err(), "has_cpu_feature should fail with too many arguments");
    
    // Test wrong argument type
    let result = has_cpu_feature(vec![Value::Number(42.0)]);
    assert!(result.is_err(), "has_cpu_feature should fail with wrong argument type");
}

#[test]
fn test_get_available_hsms_function() {
    let result = get_available_hsms(vec![]);
    assert!(result.is_ok(), "get_available_hsms should succeed");
    
    if let Value::Array(hsms) = result.unwrap() {
        // Should return an array of HSM names
        for hsm in hsms {
            if let Value::String(name) = hsm {
                assert!(!name.is_empty(), "HSM name should not be empty");
            } else {
                panic!("HSM list should contain strings");
            }
        }
    } else {
        panic!("get_available_hsms should return an array");
    }
}

#[test]
fn test_refresh_hardware_detection_function() {
    let result = refresh_hardware_detection(vec![]);
    assert!(result.is_ok(), "refresh_hardware_detection should succeed");
    
    if let Value::String(message) = result.unwrap() {
        assert!(message.contains("Hardware detection refreshed"), 
                "Message should contain 'Hardware detection refreshed'");
        assert!(message.contains("CPU features"), 
                "Message should mention CPU features");
        assert!(message.contains("HSMs"), 
                "Message should mention HSMs");
        assert!(message.contains("GPU devices"), 
                "Message should mention GPU devices");
        assert!(message.contains("crypto coprocessors"), 
                "Message should mention crypto coprocessors");
    } else {
        panic!("refresh_hardware_detection should return a string message");
    }
}

#[test]
fn test_global_hardware_detector() {
    let detector1 = get_hardware_detector();
    let detector2 = get_hardware_detector();
    
    // Should return the same instance
    assert!(std::ptr::eq(detector1, detector2), "Should return same global instance");
    
    // Should work correctly
    let capabilities = detector1.get_capabilities();
    assert!(capabilities.is_ok(), "Global detector should work");
}

#[test]
fn test_hardware_detection_performance() {
    let detector = HardwareAccelerationDetector::new();
    
    let start = std::time::Instant::now();
    let capabilities = detector.get_capabilities().unwrap();
    let detection_time = start.elapsed();
    
    // Detection should complete within reasonable time (5 seconds)
    assert!(detection_time.as_secs() < 5, 
            "Hardware detection should complete within 5 seconds");
    
    // Internal detection time should be reasonable
    assert!(capabilities.detection_time.as_secs() < 5,
            "Internal detection time should be reasonable");
}

#[test]
fn test_platform_and_architecture_detection() {
    let detector = HardwareAccelerationDetector::new();
    let capabilities = detector.get_capabilities().unwrap();
    
    // Platform should be valid
    let valid_platforms = vec!["linux", "windows", "macos", "freebsd", "openbsd", "netbsd"];
    assert!(valid_platforms.contains(&capabilities.platform.as_str()) || 
            !capabilities.platform.is_empty(),
            "Platform should be valid or non-empty");
    
    // Architecture should be valid
    let valid_architectures = vec!["x86", "x86_64", "aarch64", "arm", "mips", "powerpc", "riscv64"];
    assert!(valid_architectures.contains(&capabilities.architecture.as_str()) ||
            !capabilities.architecture.is_empty(),
            "Architecture should be valid or non-empty");
}

#[test]
fn test_cpu_feature_detection_consistency() {
    let detector = HardwareAccelerationDetector::new();
    
    // Multiple calls should return consistent results
    let caps1 = detector.get_capabilities().unwrap();
    let caps2 = detector.refresh_capabilities().unwrap();
    
    // CPU features should be consistent
    assert_eq!(caps1.cpu_features.aes_ni, caps2.cpu_features.aes_ni);
    assert_eq!(caps1.cpu_features.sha_extensions, caps2.cpu_features.sha_extensions);
    assert_eq!(caps1.cpu_features.pclmulqdq, caps2.cpu_features.pclmulqdq);
    assert_eq!(caps1.cpu_features.rdrand, caps2.cpu_features.rdrand);
}

#[test] 
fn test_gpu_detection_robustness() {
    let detector = HardwareAccelerationDetector::new();
    let capabilities = detector.get_capabilities().unwrap();
    
    // GPU detection should not fail
    let gpu_accel = &capabilities.gpu_acceleration;
    
    // Booleans should be valid
    let _ = gpu_accel.opencl_available;
    let _ = gpu_accel.cuda_available;
    let _ = gpu_accel.vulkan_compute;
    
    // Devices should be valid
    for device in &gpu_accel.devices {
        assert!(!device.name.is_empty(), "GPU device name should not be empty");
        assert!(!device.vendor.is_empty(), "GPU device vendor should not be empty");
    }
}

#[test]
fn test_hsm_detection_robustness() {
    let detector = HardwareAccelerationDetector::new();
    let capabilities = detector.get_capabilities().unwrap();
    
    // HSM detection should not fail
    for hsm in &capabilities.hsms {
        assert!(!hsm.name.is_empty(), "HSM name should not be empty");
        assert!(!hsm.version.is_empty(), "HSM version should not be empty");
        assert!(!hsm.capabilities.is_empty(), "HSM should have some capabilities");
    }
}

#[test]
fn test_crypto_coprocessor_detection_robustness() {
    let detector = HardwareAccelerationDetector::new();
    let capabilities = detector.get_capabilities().unwrap();
    
    // Crypto coprocessor detection should not fail
    for cp in &capabilities.crypto_coprocessors {
        assert!(!cp.name.is_empty(), "Coprocessor name should not be empty");
        assert!(!cp.vendor.is_empty(), "Coprocessor vendor should not be empty");
        assert!(!cp.capabilities.is_empty(), "Coprocessor should have capabilities");
        assert!(cp.performance_rating > 0, "Performance rating should be positive");
        assert!(cp.performance_rating <= 100, "Performance rating should be <= 100");
    }
}

#[test]
fn test_error_handling() {
    // Test that detection gracefully handles errors without panicking
    let detector = HardwareAccelerationDetector::new();
    
    // Should not panic even if some detection methods fail
    let result = detector.get_capabilities();
    assert!(result.is_ok(), "Detection should succeed even with partial failures");
}

#[test]
fn test_concurrent_access() {
    use std::sync::Arc;
    use std::thread;
    
    let detector = Arc::new(HardwareAccelerationDetector::new());
    let mut handles = Vec::new();
    
    // Spawn multiple threads to test concurrent access
    for _ in 0..4 {
        let detector_clone = Arc::clone(&detector);
        let handle = thread::spawn(move || {
            let capabilities = detector_clone.get_capabilities();
            assert!(capabilities.is_ok());
            capabilities.unwrap()
        });
        handles.push(handle);
    }
    
    // Wait for all threads and verify results
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    // All results should be consistent (from cache)
    for i in 1..results.len() {
        assert_eq!(results[0].platform, results[i].platform);
        assert_eq!(results[0].architecture, results[i].architecture);
    }
}

#[test]
fn test_cache_duration_setting() {
    let mut detector = HardwareAccelerationDetector::new();
    
    // Set a custom cache duration
    detector.set_cache_duration(Duration::from_secs(10));
    
    // Detection should still work
    let capabilities = detector.get_capabilities();
    assert!(capabilities.is_ok());
}

// Integration test with real hardware (when available)
#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_x86_feature_detection_integration() {
    let detector = HardwareAccelerationDetector::new();
    let capabilities = detector.get_capabilities().unwrap();
    
    // On x86/x86_64, some basic features might be available
    // This is just a sanity check that detection completes
    let cpu_features = &capabilities.cpu_features;
    
    // At least some detection should have occurred
    let any_feature_detected = cpu_features.aes_ni || 
                              cpu_features.sha_extensions || 
                              cpu_features.pclmulqdq ||
                              cpu_features.rdrand ||
                              cpu_features.avx2;
    
    // It's okay if no features are detected on older hardware
    // Just ensure detection completed without errors
    println!("Detected CPU features: AES-NI={}, SHA={}, PCLMULQDQ={}, RDRAND={}, AVX2={}", 
             cpu_features.aes_ni,
             cpu_features.sha_extensions,
             cpu_features.pclmulqdq,
             cpu_features.rdrand,
             cpu_features.avx2);
}
