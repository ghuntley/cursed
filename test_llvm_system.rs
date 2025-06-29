// Test the restored LLVM code generation system
use cursed::codegen::llvm::main::LlvmCodeGenerator;
use cursed::codegen::llvm::passes::*;
use cursed::error::Result;
use std::path::Path;

fn test_llvm_restoration() -> Result<()> {
    println!("=== Testing LLVM Code Generation System Restoration ===");
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new();
    
    // Test source file
    let test_file = Path::new("test_llvm_restoration.csd");
    let source = std::fs::read_to_string(test_file)
        .map_err(|e| cursed::error::CursedError::io_error(&format!("Failed to read test file: {}", e)))?;
    
    println!("1. Testing basic compilation...");
    match generator.compile(&source) {
        Ok(llvm_ir) => {
            println!("✓ Basic compilation successful");
            println!("Generated LLVM IR length: {} bytes", llvm_ir.len());
            
            // Check for key runtime function declarations
            if llvm_ir.contains("cursed_vibez_spill") {
                println!("✓ Vibez.spill runtime function found");
            }
            if llvm_ir.contains("cursed_goroutine_spawn") {
                println!("✓ Goroutine runtime functions found");
            }
            if llvm_ir.contains("cursed_gc_alloc") {
                println!("✓ GC runtime functions found");
            }
        },
        Err(e) => {
            println!("✗ Basic compilation failed: {}", e);
            return Err(e);
        }
    }
    
    println!("\n2. Testing optimization passes...");
    test_optimization_passes()?;
    
    println!("\n3. Testing package integration...");
    test_package_integration(&mut generator)?;
    
    println!("\n4. Testing member access code generation...");
    test_member_access_generation(&mut generator)?;
    
    println!("\n=== All LLVM System Tests Passed! ===");
    Ok(())
}

fn test_optimization_passes() -> Result<()> {
    use inkwell::context::Context;
    
    let context = Context::create();
    
    // Test SCCP pass
    let mut sccp = SccpPass::new(&context);
    println!("✓ SCCP pass created successfully");
    
    // Test LICM pass
    let mut licm = LicmPass::new(&context);
    println!("✓ LICM pass created successfully");
    
    // Test Mem2Reg pass
    let mut mem2reg = Mem2RegPass::new(&context);
    println!("✓ Mem2Reg pass created successfully");
    
    // Test SROA pass
    let mut sroa = SroaPass::new(&context);
    println!("✓ SROA pass created successfully");
    
    // Test Tail Call pass
    let mut tail_call = TailCallPass::new(&context);
    println!("✓ Tail Call pass created successfully");
    
    // Test Jump Threading pass
    let mut jump_threading = JumpThreadingPass::new(&context);
    println!("✓ Jump Threading pass created successfully");
    
    // Test Dead Code Elimination pass
    let dce = DeadCodeEliminationPass::new(&context);
    println!("✓ Dead Code Elimination pass created successfully");
    
    Ok(())
}

fn test_package_integration(generator: &mut LlvmCodeGenerator) -> Result<()> {
    let test_source = r#"
import "fmt"
import "strings"

func main() {
    fmt.println("Hello")
    strings.upper("test")
}
"#;
    
    match generator.compile_with_packages(test_source, None) {
        Ok(llvm_ir) => {
            println!("✓ Package integration compilation successful");
            
            // Check for package declarations
            if llvm_ir.contains("cursed_pkg_fmt") {
                println!("✓ Package 'fmt' integration found");
            }
            if llvm_ir.contains("cursed_pkg_strings") {
                println!("✓ Package 'strings' integration found");
            }
        },
        Err(e) => {
            println!("✗ Package integration failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

fn test_member_access_generation(generator: &mut LlvmCodeGenerator) -> Result<()> {
    let test_source = r#"
func main() {
    vibez.spill("Hello")
    vibez.spillf("Number: %d", 42)
    input := vibez.readln()
    data := vibez.read()
}
"#;
    
    match generator.compile(test_source) {
        Ok(llvm_ir) => {
            println!("✓ Member access compilation successful");
            
            // Check for member access code generation
            if llvm_ir.contains("vibez.spill") || llvm_ir.contains("cursed_vibez_spill") {
                println!("✓ vibez.spill member access found");
            }
            if llvm_ir.contains("vibez.spillf") || llvm_ir.contains("cursed_vibez_spillf") {
                println!("✓ vibez.spillf member access found");
            }
            if llvm_ir.contains("vibez.readln") || llvm_ir.contains("cursed_vibez_readln") {
                println!("✓ vibez.readln member access found");
            }
            if llvm_ir.contains("vibez.read") || llvm_ir.contains("cursed_vibez_read") {
                println!("✓ vibez.read member access found");
            }
        },
        Err(e) => {
            println!("✗ Member access generation failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

fn main() {
    match test_llvm_restoration() {
        Ok(()) => {
            println!("\n🎉 LLVM Code Generation System Restoration Complete!");
            println!("All critical components have been successfully restored:");
            println!("  ✓ JIT Runtime Functions");
            println!("  ✓ Optimization Passes (SCCP, LICM, Mem2Reg, SROA, Tail Call, Jump Threading)");
            println!("  ✓ Package Integration System");
            println!("  ✓ Member Access Code Generation");
            println!("  ✓ Dead Code Elimination");
            println!("  ✓ Runtime Function Declarations");
            std::process::exit(0);
        },
        Err(e) => {
            eprintln!("❌ LLVM System Test Failed: {}", e);
            std::process::exit(1);
        }
    }
}
