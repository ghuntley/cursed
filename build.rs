// Minimal Build script for the CURSED programming language
// Simplified to avoid hanging issues

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    
    // Basic linking without complex runtime building
    let target = env::var("TARGET").unwrap_or_default();
    
    if target.contains("linux") {
        println!("cargo:rustc-link-arg=-Wl,--as-needed");
    }
    
    // Skip all complex runtime library building for now
    println!("cargo:warning=Using minimal build script - complex runtime building disabled");
}
