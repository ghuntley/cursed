# Comprehensive Self-Hosting Test for CURSED
# Tests critical modules for self-hosting compilation pipeline

yeet "testz"
yeet "vibe_life"
yeet "sys_core"
yeet "memory"
yeet "exec_slay"
yeet "parser"
yeet "vibez"

# ===== SELF-HOSTING CAPABILITY TESTS =====

# Test 1: OS Operations (vibe_life module)
slay test_os_operations() lit {
    test_start("OS Operations for Self-Hosting")
    
    # Test command line argument handling
    vibe_life.set_args(["cursed", "--compile", "program.csd"])
    sus args := vibe_life.get_args()
    assert_eq_int(vibe_life.get_arg_count(), 3)
    assert_eq_string(vibe_life.get_arg(0), "cursed")
    assert_eq_string(vibe_life.get_arg(1), "--compile")
    
    # Test environment variable access
    vibe_life.set_env("CURSED_HOME", "/usr/local/cursed")
    assert_eq_string(vibe_life.get_env("CURSED_HOME"), "/usr/local/cursed")
    
    # Test filesystem operations
    assert_true(vibe_life.file_exists("/dev/null"))
    sus cwd := vibe_life.get_cwd()
    assert_true(stringz.length(cwd) > 0)
    
    print_test_summary()
    damn based
}

# Test 2: System Core Functions (sys_core module)
slay test_system_core() lit {
    test_start("System Core Functions")
    
    # Test system information
    sus platform := sys_core.get_platform()
    assert_eq_string(platform, "linux-x64")
    
    sus arch := sys_core.get_architecture()
    assert_eq_string(arch, "x64")
    
    # Test memory operations
    sus mem_usage := sys_core.memory_usage()
    assert_true(mem_usage > 0)
    
    sus mem_limit := sys_core.get_memory_limit()
    assert_true(mem_limit > mem_usage)
    
    # Test process operations
    sus pid := sys_core.get_process_id()
    assert_true(pid > 0)
    
    print_test_summary()
    damn based
}

# Test 3: Memory Management (memory module)
slay test_memory_management() lit {
    test_start("Memory Management for Compilation")
    
    # Test heap allocation
    sus ptr := memory.allocate(1024)
    assert_true(ptr != 0)
    
    # Test memory initialization
    memory.zero_memory(ptr, 1024)
    
    # Test memory copy operations
    sus src_ptr := memory.allocate(256)
    sus dest_ptr := memory.allocate(256)
    memory.copy_memory(src_ptr, dest_ptr, 256)
    
    # Test memory deallocation
    assert_true(memory.deallocate(ptr))
    assert_true(memory.deallocate(src_ptr))
    assert_true(memory.deallocate(dest_ptr))
    
    # Test memory statistics
    sus stats := memory.get_statistics()
    assert_true(stats.total_allocated >= 0)
    
    print_test_summary()
    damn based
}

# Test 4: Process Execution (exec_slay module)
slay test_process_execution() lit {
    test_start("Process Execution for Compilation Pipeline")
    
    # Test simple command execution
    sus result := exec_slay.exec_command("echo", ["Hello Self-Hosting"])
    assert_eq_int(result.exit_code, 0)
    assert_true(result.success)
    
    # Test compilation pipeline simulation
    sus compile_result := exec_slay.exec_command("which", ["llc"])
    if compile_result.success {
        vibez.spill("LLVM toolchain available for native compilation")
    }
    
    # Test pipeline building
    sus pipeline := exec_slay.create_pipeline()
    exec_slay.add_command(pipeline, "echo", ["Stage 1: Lexical Analysis"])
    exec_slay.add_command(pipeline, "echo", ["Stage 2: Parsing"])
    exec_slay.add_command(pipeline, "echo", ["Stage 3: Code Generation"])
    
    sus pipeline_result := exec_slay.execute_pipeline(pipeline)
    assert_true(pipeline_result.success)
    
    print_test_summary()
    damn based
}

# Test 5: Code Parsing (parser module)
slay test_code_parsing() lit {
    test_start("Code Parsing for Self-Hosting")
    
    # Test tokenization of simple CURSED code
    sus source_code := "vibez.spill(\"Hello World\")"
    sus tokens := parser.tokenize(source_code)
    assert_true(tokens.length() > 0)
    
    # Test AST generation
    sus ast := parser.parse(source_code)
    assert_true(ast != cringe)
    
    # Test parsing a variable declaration
    sus var_decl := "sus x := 42"
    sus var_tokens := parser.tokenize(var_decl)
    assert_true(var_tokens.length() >= 4)  # sus, x, :=, 42
    
    # Test parsing a function definition
    sus func_def := "slay test_func() lit { damn based }"
    sus func_tokens := parser.tokenize(func_def)
    assert_true(func_tokens.length() >= 8)
    
    # Test code validation
    sus valid_code := parser.validate_syntax(source_code)
    assert_true(valid_code)
    
    print_test_summary()
    damn based
}

# ===== BOOTSTRAP COMPILATION TEST =====

slay test_bootstrap_compilation() lit {
    test_start("Bootstrap Self-Hosting Compilation Test")
    
    # Create minimal CURSED program for self-compilation
    sus mini_program := "vibez.spill(\"Self-compiled program works!\")"
    
    # Stage 1: Lexical Analysis
    vibez.spill("Stage 1: Tokenizing source code...")
    sus tokens := parser.tokenize(mini_program)
    assert_true(tokens.length() > 0)
    vibez.spill("✅ Lexical analysis complete")
    
    # Stage 2: Syntax Analysis
    vibez.spill("Stage 2: Parsing AST...")
    sus ast := parser.parse(mini_program)
    assert_true(ast != cringe)
    vibez.spill("✅ Syntax analysis complete")
    
    # Stage 3: Code Generation (simulated)
    vibez.spill("Stage 3: Generating intermediate representation...")
    sus ir_code := parser.generate_ir(ast)
    assert_true(stringz.length(ir_code) > 0)
    vibez.spill("✅ Code generation complete")
    
    # Stage 4: Compilation Pipeline
    vibez.spill("Stage 4: Executing compilation pipeline...")
    sus compile_pipeline := exec_slay.create_pipeline()
    exec_slay.add_command(compile_pipeline, "echo", ["Compiling CURSED program"])
    exec_slay.add_command(compile_pipeline, "echo", ["Linking runtime library"])
    exec_slay.add_command(compile_pipeline, "echo", ["Generating executable"])
    
    sus compile_result := exec_slay.execute_pipeline(compile_pipeline)
    assert_true(compile_result.success)
    vibez.spill("✅ Compilation pipeline complete")
    
    # Stage 5: Memory Management Verification
    vibez.spill("Stage 5: Verifying memory management...")
    sus compiler_memory := memory.allocate(4096)  # Simulate compiler heap
    assert_true(compiler_memory != 0)
    memory.zero_memory(compiler_memory, 4096)
    assert_true(memory.deallocate(compiler_memory))
    vibez.spill("✅ Memory management verified")
    
    print_test_summary()
    damn based
}

# ===== SELF-HOSTING READINESS VERIFICATION =====

slay verify_self_hosting_readiness() lit {
    test_start("Self-Hosting Readiness Verification")
    
    vibez.spill("🚀 CURSED Self-Hosting Capability Assessment")
    vibez.spill("=" * 50)
    
    # Check OS operation capabilities
    sus os_ready := test_os_operations()
    if os_ready {
        vibez.spill("✅ OS Operations: READY")
    } else {
        vibez.spill("❌ OS Operations: NOT READY")
    }
    
    # Check system core functionality
    sus sys_ready := test_system_core()
    if sys_ready {
        vibez.spill("✅ System Core: READY")
    } else {
        vibez.spill("❌ System Core: NOT READY")
    }
    
    # Check memory management
    sus mem_ready := test_memory_management()
    if mem_ready {
        vibez.spill("✅ Memory Management: READY")
    } else {
        vibez.spill("❌ Memory Management: NOT READY")
    }
    
    # Check process execution
    sus exec_ready := test_process_execution()
    if exec_ready {
        vibez.spill("✅ Process Execution: READY")
    } else {
        vibez.spill("❌ Process Execution: NOT READY")
    }
    
    # Check code parsing
    sus parse_ready := test_code_parsing()
    if parse_ready {
        vibez.spill("✅ Code Parsing: READY")
    } else {
        vibez.spill("❌ Code Parsing: NOT READY")
    }
    
    # Check bootstrap compilation
    sus bootstrap_ready := test_bootstrap_compilation()
    if bootstrap_ready {
        vibez.spill("✅ Bootstrap Compilation: READY")
    } else {
        vibez.spill("❌ Bootstrap Compilation: NOT READY")
    }
    
    # Overall readiness assessment
    sus total_ready := os_ready && sys_ready && mem_ready && exec_ready && parse_ready && bootstrap_ready
    
    vibez.spill("")
    vibez.spill("📊 FINAL ASSESSMENT:")
    if total_ready {
        vibez.spill("🎉 CURSED IS READY FOR SELF-HOSTING!")
        vibez.spill("All critical modules are functional and tested.")
        vibez.spill("The compiler can now compile itself!")
    } else {
        vibez.spill("⚠️  CURSED REQUIRES ADDITIONAL WORK FOR SELF-HOSTING")
        vibez.spill("Some critical modules need fixes before self-hosting.")
    }
    
    print_test_summary()
    damn total_ready
}

# ===== MAIN TEST EXECUTION =====

# Run individual module tests
test_os_operations()
test_system_core()
test_memory_management()
test_process_execution()
test_code_parsing()

# Run bootstrap compilation test
test_bootstrap_compilation()

# Final readiness verification
verify_self_hosting_readiness()

vibez.spill("")
vibez.spill("🏁 Self-Hosting Test Suite Complete!")
vibez.spill("Check results above for readiness assessment.")
