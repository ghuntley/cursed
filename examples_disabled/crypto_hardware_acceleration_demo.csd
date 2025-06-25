#!/usr/bin/env cursed

// Hardware Acceleration Detection Demo
// Demonstrates comprehensive hardware acceleration capabilities detection

import "stdlib::io";
import "stdlib::packages::crypto_asymmetric::hardware_acceleration";

slay main() -> Result<(), Error> {
    println("🔧 CURSED Hardware Acceleration Detection Demo")?;
    println("==============================================")?;
    println("")?;

    // Basic hardware support check
    println("📊 Detecting Hardware Acceleration Support...")?;
    facts hardware_info = check_hardware_support()?;
    println("")?;

    // Display platform information
    println("🖥️  Platform Information:")?;
    printf("   Platform: {}\n", &[hardware_info.platform])?;
    printf("   Architecture: {}\n", &[hardware_info.architecture])?;
    printf("   Detection Time: {} ms\n", &[hardware_info.detection_time_ms])?;
    println("")?;

    // CPU Cryptographic Features
    println("🔒 CPU Cryptographic Features:")?;
    facts cpu_features = hardware_info.cpu_features;
    
    // Check individual features
    facts aes_ni = has_cpu_feature("aes_ni")?;
    facts sha_ext = has_cpu_feature("sha_extensions")?;
    facts pclmul = has_cpu_feature("pclmulqdq")?;
    facts rdrand = has_cpu_feature("rdrand")?;
    facts rdseed = has_cpu_feature("rdseed")?;
    facts avx2 = has_cpu_feature("avx2")?;
    facts avx512 = has_cpu_feature("avx512f")?;
    
    printf("   AES-NI (AES acceleration):     {}\n", &[format_bool(aes_ni)])?;
    printf("   SHA Extensions:                {}\n", &[format_bool(sha_ext)])?;
    printf("   PCLMULQDQ (GCM acceleration):  {}\n", &[format_bool(pclmul)])?;
    printf("   RDRAND (Hardware RNG):         {}\n", &[format_bool(rdrand)])?;
    printf("   RDSEED (Seed generator):       {}\n", &[format_bool(rdseed)])?;
    printf("   AVX2 (Vector operations):      {}\n", &[format_bool(avx2)])?;
    printf("   AVX-512F (Advanced vectors):   {}\n", &[format_bool(avx512)])?;
    
    // Additional features
    facts vaes = has_cpu_feature("vaes")?;
    facts vpclmul = has_cpu_feature("vpclmulqdq")?;
    
    lowkey (vaes || vpclmul) {
        println("   Advanced Vector Crypto:")?;
        printf("     VAES (Vector AES):          {}\n", &[format_bool(vaes)])?;
        printf("     VPCLMULQDQ (Vector GCM):    {}\n", &[format_bool(vpclmul)])?;
    }
    println("")?;

    // Hardware Security Modules
    println("🛡️  Hardware Security Modules:")?;
    facts available_hsms = get_available_hsms()?;
    
    lowkey (available_hsms.length() > 0) {
        bestie hsm_name in available_hsms {
            printf("   ✓ {}\n", &[hsm_name])?;
        }
    } flex {
        println("   No Hardware Security Modules detected")?;
    }
    
    // Display detailed HSM information
    facts hsms = hardware_info.hsms;
    lowkey (hsms.length() > 0) {
        println("   Detailed HSM Information:")?;
        bestie hsm in hsms {
            printf("     Name: {} (Version: {})\n", &[hsm.name, hsm.version])?;
            printf("     Available: {}\n", &[format_bool(hsm.available)])?;
            printf("     Capabilities: {}\n", &[hsm.capabilities.join(", ")])?;
        }
    }
    println("")?;

    // GPU Acceleration
    println("🚀 GPU Acceleration Support:")?;
    facts gpu_accel = hardware_info.gpu_acceleration;
    
    printf("   OpenCL Support:     {}\n", &[format_bool(gpu_accel.opencl_available)])?;
    printf("   CUDA Support:       {}\n", &[format_bool(gpu_accel.cuda_available)])?;
    printf("   Vulkan Compute:     {}\n", &[format_bool(gpu_accel.vulkan_compute)])?;
    
    lowkey (gpu_accel.devices.length() > 0) {
        println("   GPU Devices:")?;
        bestie device in gpu_accel.devices {
            printf("     {} ({}) - {} MB VRAM\n", &[
                device.name, 
                device.vendor, 
                device.memory_mb
            ])?;
            printf("       Compute Units: {}\n", &[device.compute_units])?;
            printf("       Crypto Support: {}\n", &[format_bool(device.supports_crypto)])?;
        }
    } flex {
        println("   No GPU devices detected")?;
    }
    println("")?;

    // Cryptographic Coprocessors
    println("⚡ Cryptographic Coprocessors:")?;
    facts crypto_coprocessors = hardware_info.crypto_coprocessors;
    
    lowkey (crypto_coprocessors.length() > 0) {
        bestie cp in crypto_coprocessors {
            printf("   {} ({})\n", &[cp.name, cp.vendor])?;
            printf("     Performance Rating: {}/100\n", &[cp.performance_rating])?;
            printf("     Capabilities: {}\n", &[cp.capabilities.join(", ")])?;
        }
    } flex {
        println("   No cryptographic coprocessors detected")?;
    }
    println("")?;

    // Performance Recommendations
    println("💡 Performance Recommendations:")?;
    show_performance_recommendations(hardware_info)?;
    println("")?;

    // Security Recommendations
    println("🔐 Security Recommendations:")?;
    show_security_recommendations(hardware_info)?;
    println("")?;

    // Test refresh functionality
    println("🔄 Testing Hardware Detection Refresh...")?;
    facts refresh_result = refresh_hardware_detection()?;
    printf("   {}\n", &[refresh_result])?;
    println("")?;

    println("✅ Hardware acceleration detection completed successfully!")?;
    return Ok(());
}

slay format_bool(value: Bool) -> String {
    lowkey (value) {
        return "✅ Available";
    } flex {
        return "❌ Not Available";
    }
}

slay show_performance_recommendations(hardware_info: Object) -> Result<(), Error> {
    facts cpu_features = hardware_info.cpu_features;
    facts gpu_accel = hardware_info.gpu_acceleration;
    facts crypto_coprocessors = hardware_info.crypto_coprocessors;
    
    // AES recommendations
    lowkey (cpu_features.aes_ni) {
        println("   ✓ Use AES-NI accelerated implementations for symmetric encryption")?;
    } flex {
        println("   ⚠ Consider software AES implementations or hardware upgrades")?;
    }
    
    // SHA recommendations
    lowkey (cpu_features.sha_extensions) {
        println("   ✓ Use SHA hardware acceleration for hashing operations")?;
    } flex {
        println("   ⚠ SHA operations will use software implementations")?;
    }
    
    // Random number generation
    lowkey (cpu_features.rdrand || cpu_features.rdseed) {
        println("   ✓ Hardware random number generation available")?;
    } flex {
        println("   ⚠ Use additional entropy sources for cryptographic randomness")?;
    }
    
    // GPU recommendations
    lowkey (gpu_accel.opencl_available || gpu_accel.cuda_available) {
        println("   ✓ GPU acceleration available for parallel crypto operations")?;
        lowkey (gpu_accel.devices.length() > 0) {
            printf("   ✓ {} GPU device(s) available for crypto workloads\n", 
                   &[gpu_accel.devices.length()])?;
        }
    } flex {
        println("   ⚠ No GPU acceleration available")?;
    }
    
    // Coprocessor recommendations
    lowkey (crypto_coprocessors.length() > 0) {
        printf("   ✓ {} dedicated crypto coprocessor(s) available\n", 
               &[crypto_coprocessors.length()])?;
        println("   ✓ Consider offloading intensive crypto operations")?;
    }
    
    return Ok(());
}

slay show_security_recommendations(hardware_info: Object) -> Result<(), Error> {
    facts cpu_features = hardware_info.cpu_features;
    facts hsms = hardware_info.hsms;
    facts platform = hardware_info.platform;
    
    // HSM recommendations
    lowkey (hsms.length() > 0) {
        printf("   ✓ {} HSM(s) available for secure key storage\n", &[hsms.length()])?;
        println("   ✓ Consider using HSMs for high-value cryptographic keys")?;
    } flex {
        println("   ⚠ No HSMs detected - use software key protection")?;
    }
    
    // Secure random recommendations
    lowkey (cpu_features.rdrand && cpu_features.rdseed) {
        println("   ✓ Hardware entropy sources available (RDRAND + RDSEED)")?;
    } flex lowkey (cpu_features.rdrand) {
        println("   ⚠ Only RDRAND available - supplement with additional entropy")?;
    } flex {
        println("   ⚠ No hardware entropy - use OS entropy sources carefully")?;
    }
    
    // Platform-specific recommendations
    vibe_check platform {
        mood "linux" => {
            println("   ✓ Linux: Consider using /dev/urandom for cryptographic randomness")?;
            println("   ✓ Check for TPM 2.0 support via tpm2-tools")?;
        }
        mood "windows" => {
            println("   ✓ Windows: Use CryptGenRandom or BCrypt for secure randomness")?;
            println("   ✓ Consider Windows Hello for TPM-based authentication")?;
        }
        mood "macos" => {
            println("   ✓ macOS: Use Secure Enclave for key storage when available")?;
            println("   ✓ Consider Keychain Services for secure credential storage")?;
        }
        basic => {
            println("   ⚠ Platform-specific security features may be available")?;
        }
    }
    
    return Ok(());
}
