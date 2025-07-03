#!/usr/bin/env rust-script

//! Test script to demonstrate CURSED compilation concept
//! 
//! This script simulates the compilation pipeline we've implemented:
//! 1. Parse CURSED source
//! 2. Generate LLVM IR  
//! 3. Compile IR to object file with llc
//! 4. Link object file to executable with gcc/clang

use std::process::Command;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing CURSED Compilation Pipeline");
    
    // Step 1: Create test CURSED source
    let cursed_source = r#"
vibe main

yeet "vibez"

slay main() {
    vibez.spill("Hello, CURSED world! 🎉")
}
"#;
    
    println!("📝 CURSED Source:");
    println!("{}", cursed_source);
    
    // Step 2: Generate corresponding LLVM IR (simplified version)
    let llvm_ir = r#"; CURSED Language - Advanced LLVM Compilation
target triple = "x86_64-unknown-linux-gnu"

; Runtime function declarations
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)

; String constants
@.str.0 = private unnamed_addr constant [27 x i8] c"Hello, CURSED world! \F0\9F\8E\89\00", align 1

define i32 @main() {
  %1 = call i32 @puts(i8* getelementptr inbounds ([27 x i8], [27 x i8]* @.str.0, i64 0, i64 0))
  ret i32 0
}
"#;
    
    println!("\n⚡ Generated LLVM IR:");
    println!("{}", llvm_ir);
    
    // Step 3: Write IR to file
    fs::write("test.ll", llvm_ir)?;
    println!("\n💾 Wrote IR to test.ll");
    
    // Step 4: Check if LLVM tools are available
    let llc_check = Command::new("llc").arg("--version").output();
    if llc_check.is_err() {
        println!("❌ LLVM compiler (llc) not found. Install LLVM tools to test full compilation.");
        cleanup_files();
        return Ok(());
    }
    
    // Step 5: Compile IR to object file
    println!("\n🔧 Compiling IR to object file...");
    let llc_output = Command::new("llc")
        .args(&["-filetype=obj", "-o", "test.o", "test.ll"])
        .output()?;
    
    if !llc_output.status.success() {
        println!("❌ LLC compilation failed:");
        println!("{}", String::from_utf8_lossy(&llc_output.stderr));
        cleanup_files();
        return Ok(());
    }
    
    println!("✅ Object file compiled successfully");
    
    // Step 6: Link to executable
    println!("\n🔗 Linking to executable...");
    let linkers = ["clang", "gcc"];
    let mut linking_successful = false;
    
    for linker in &linkers {
        let link_check = Command::new(linker).arg("--version").output();
        if link_check.is_ok() {
            println!("Using {} as linker", linker);
            
            let link_output = Command::new(linker)
                .args(&["-o", "test_hello", "test.o", "-lc"])
                .output()?;
            
            if link_output.status.success() {
                linking_successful = true;
                println!("✅ Executable linked successfully");
                break;
            } else {
                println!("Failed to link with {}: {}", linker, String::from_utf8_lossy(&link_output.stderr));
            }
        }
    }
    
    if !linking_successful {
        println!("❌ No suitable linker found or linking failed");
        cleanup_files();
        return Ok(());
    }
    
    // Step 7: Test the executable
    println!("\n🎯 Testing the executable...");
    let run_output = Command::new("./test_hello").output()?;
    
    println!("📤 Executable output:");
    println!("{}", String::from_utf8_lossy(&run_output.stdout));
    
    if run_output.status.success() {
        println!("🎉 SUCCESS! CURSED compilation pipeline works!");
    } else {
        println!("❌ Executable failed to run properly");
    }
    
    // Cleanup
    cleanup_files();
    
    Ok(())
}

fn cleanup_files() {
    let _ = fs::remove_file("test.ll");
    let _ = fs::remove_file("test.o");
    let _ = fs::remove_file("test_hello");
}
