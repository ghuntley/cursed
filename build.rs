// Build script for the CURSED programming language
// Handles linking with system libraries including SQLite3

use std::env;

fn main() {
    // Link with SQLite3 library
    println!("cargo:rustc-link-lib=sqlite3");
    
    // For the Nix environment, add the specific library path
    if let Ok(library_path) = env::var("LIBRARY_PATH") {
        for path in library_path.split(':') {
            if !path.is_empty() {
                println!("cargo:rustc-link-search=native={}", path);
            }
        }
    }
    
    // Also check for pkg-config for SQLite3
    if let Ok(pkg_config_path) = env::var("PKG_CONFIG_PATH") {
        for path in pkg_config_path.split(':') {
            if !path.is_empty() {
                println!("cargo:rustc-link-search=native={}", path.replace("/pkgconfig", ""));
            }
        }
    }
    
    // Tell cargo to rerun build script if environment changes
    println!("cargo:rerun-if-env-changed=LIBRARY_PATH");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
}
