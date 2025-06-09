// Build script for Stage 2 CURSED compiler
// Compiles .csd files to executable using the Rust Stage 1 compiler

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let stage2_dir = Path::new("src/bootstrap/stage2");
    
    println!("cargo:rerun-if-changed=src/bootstrap/stage2/");
    
    // Check if Stage 2 compiler should be built
    if env::var("CURSED_BUILD_STAGE2").is_ok() {
        build_stage2_compiler(&out_dir, stage2_dir);
    }
}

fn build_stage2_compiler(out_dir: &str, stage2_dir: &Path) {
    let cursed_files = [
        "main.csd",
        "lexer.csd", 
        "ast.csd",
        "parser.csd",
        "codegen.csd",
    ];
    
    // Create combined CURSED source file
    let mut combined_source = String::new();
    
    for file in &cursed_files {
        let file_path = stage2_dir.join(file);
        if file_path.exists() {
            let content = fs::read_to_string(&file_path).unwrap();
            combined_source.push_str(&format!("// === {} ===\n", file));
            combined_source.push_str(&content);
            combined_source.push_str("\n\n");
        }
    }
    
    // Write combined source
    let combined_path = Path::new(out_dir).join("stage2_compiler.csd");
    fs::write(&combined_path, combined_source).unwrap();
    
    // Compile using Stage 1 compiler
    let stage1_binary = env::var("CARGO_BIN_EXE_cursed")
        .unwrap_or_else(|_| "target/debug/cursed".to_string());
    
    let stage2_binary = Path::new(out_dir).join("cursed-stage2");
    
    let output = Command::new(&stage1_binary)
        .arg("compile")
        .arg(&combined_path)
        .arg("-o")
        .arg(&stage2_binary)
        .output()
        .expect("Failed to execute Stage 1 compiler");
    
    if !output.status.success() {
        println!("cargo:warning=Stage 2 compilation failed:");
        println!("cargo:warning={}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("cargo:warning=Stage 2 compiler built successfully: {:?}", stage2_binary);
        
        // Make the binary available to the main build
        let final_path = Path::new("target").join(env::var("PROFILE").unwrap()).join("cursed-stage2");
        if let Some(parent) = final_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::copy(&stage2_binary, &final_path).ok();
    }
}
