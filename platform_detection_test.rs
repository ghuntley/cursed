use std::env;

fn main() {
    println!("🧪 CURSED Platform Detection Test");
    println!("==================================");
    
    // Basic platform detection
    println!("Architecture: {}", env::consts::ARCH);
    println!("Operating System: {}", env::consts::OS);
    println!("Family: {}", env::consts::FAMILY);
    
    // Hardware concurrency
    let concurrency = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("Hardware Concurrency: {} cores", concurrency);
    
    // Basic feature detection for ARM64
    #[cfg(target_arch = "aarch64")]
    {
        println!("Platform: ARM64 {}", env::consts::OS);
        println!("Features detected:");
        
        #[cfg(target_feature = "neon")]
        println!("✅ NEON SIMD instructions");
        
        #[cfg(target_feature = "aes")]
        println!("✅ AES hardware acceleration");
        
        #[cfg(target_feature = "sha2")]
        println!("✅ SHA2 hardware acceleration");
        
        #[cfg(target_feature = "sha3")]
        println!("✅ SHA3 hardware acceleration");
        
        // Page size detection
        let page_size = if cfg!(target_os = "macos") {
            16 * 1024  // macOS ARM64 uses 16KB pages
        } else {
            4 * 1024   // Linux ARM64 typically uses 4KB pages
        };
        println!("Page Size: {}KB", page_size / 1024);
    }
    
    // Basic feature detection for x86_64
    #[cfg(target_arch = "x86_64")]
    {
        println!("Platform: x86_64 {}", env::consts::OS);
        println!("Features detected:");
        
        #[cfg(target_feature = "sse")]
        println!("✅ SSE instructions");
        
        #[cfg(target_feature = "sse2")]
        println!("✅ SSE2 instructions");
        
        #[cfg(target_feature = "avx")]
        println!("✅ AVX instructions");
        
        #[cfg(target_feature = "avx2")]
        println!("✅ AVX2 instructions");
        
        // Standard x86_64 page size
        println!("Page Size: 4KB");
    }
    
    // WASM detection
    #[cfg(target_arch = "wasm32")]
    {
        println!("Platform: WebAssembly");
        println!("Memory: Linear 64KB pages");
        println!("Scheduling: Cooperative");
    }
    
    println!("\n✅ Platform detection test completed successfully!");
}
