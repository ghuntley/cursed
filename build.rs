// Build script for the CURSED programming language
// Handles linking with system libraries including SQLite3

use std::env;

fn main() {
    // Link with SQLite3 library
    println!("cargo:rustc-link-lib=sqlite3");
    
    // Link with libffi library
    println!("cargo:rustc-link-lib=ffi");
    
    // Link with terminfo/ncurses library
    println!("cargo:rustc-link-lib=tinfo");
    
    // Link with libxml2 library
    println!("cargo:rustc-link-lib=xml2");
    
    // For the Nix environment, add the specific library path
    if let Ok(library_path) = env::var("LIBRARY_PATH") {
        for path in library_path.split(':') {
            if !path.is_empty() {
                println!("cargo:rustc-link-search=native={}", path);
            }
        }
    }
    
    // Add libffi library search paths for Nix environment
    let possible_libffi_paths = vec![
        "/nix/store/09b5m303v4d52wjry30xsabj65vnhkni-libffi-3.4.7/lib",
        "/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib",
        "/nix/store/n0lzbpbc5dwq03s1vjr985b28cjbp2gs-libffi-3.4.7/lib",
        "/nix/store/paqdsvmj4fwhc2w6rr884c3kymxl69k0-libffi-3.4.8/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/local/lib",
    ];
    
    // Add ncurses/tinfo library search paths for Nix environment
    let possible_ncurses_paths = vec![
        "/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/local/lib",
    ];
    
    // Add libxml2 library search paths for Nix environment
    let possible_xml2_paths = vec![
        "/nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/local/lib",
    ];
    
    for path in possible_libffi_paths {
        if std::path::Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }
    
    for path in possible_ncurses_paths {
        if std::path::Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }
    
    for path in possible_xml2_paths {
        if std::path::Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
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
    
    // macOS-specific configuration for dylib loading
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    }
    
    // Add LLVM bin paths to environment for compilation
    let possible_llvm_bin_paths = vec![
        "/nix/store/013b6qj9g2n2pmxcllnch9drrf9m0zwf-llvm-17.0.6/bin",
        "/nix/store/s5a4igx64mngxrz3d4s2mxz6764mdv47-llvm-17.0.6/bin",
        "/nix/store/8qpf7pp0a71psdngm5nxc64jahw0vlwl-llvm-19.1.7/bin",
        "/nix/store/vnxd8nqfibccfbczxwd9li5hw42k5kmw-llvm-19.1.6/bin",
        "/usr/bin",
        "/usr/local/bin",
    ];
    
    for path in possible_llvm_bin_paths {
        if std::path::Path::new(path).exists() {
            if let Ok(current_path) = env::var("PATH") {
                println!("cargo:rustc-env=PATH={}:{}", path, current_path);
                break;
            } else {
                println!("cargo:rustc-env=PATH={}", path);
                break;
            }
        }
    }
    
    // Tell cargo to rerun build script if environment changes
    println!("cargo:rerun-if-env-changed=LIBRARY_PATH");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PATH");
}
