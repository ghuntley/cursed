yeet "stringz"
yeet "mathz"

# Self-hosting compiler simulation
squad Compiler {
    spill source tea
    spill tokens drip
    spill errors drip
}

slay new_compiler(source tea) Compiler {
    damn Compiler{source: source, tokens: 0, errors: 0}
}

slay compile_phase1(compiler Compiler) lit {
    vibez.spill("Phase 1: Lexical Analysis")
    vibez.spill("Source: " + compiler.source)
    
    # Simulate tokenization
    compiler.tokens = stringz.length(compiler.source) / 5
    vibez.spill("Generated tokens: " + compiler.tokens)
    
    damn based
}

slay compile_phase2(compiler Compiler) lit {
    vibez.spill("Phase 2: Syntax Analysis")
    
    # Simple validation
    ready (stringz.contains(compiler.source, "slay")) {
        vibez.spill("✅ Function declaration found")
        damn based
    } otherwise {
        vibez.spill("❌ No function declaration")
        compiler.errors = compiler.errors + 1
        damn cringe
    }
}

slay compile_phase3(compiler Compiler) lit {
    vibez.spill("Phase 3: Code Generation")
    vibez.spill("✅ Generated C code successfully")
    damn based
}

# Test the compiler
sus test_program tea = "slay main() { vibez.spill(\"Hello World\") }"
sus compiler Compiler = new_compiler(test_program)

vibez.spill("🚀 CURSED Self-Hosting Compiler Test")
vibez.spill("===================================")

sus phase1_ok lit = compile_phase1(compiler)
sus phase2_ok lit = compile_phase2(compiler) 
sus phase3_ok lit = compile_phase3(compiler)

ready (phase1_ok && phase2_ok && phase3_ok) {
    vibez.spill("🎉 Compilation successful!")
    vibez.spill("Errors: " + compiler.errors)
} otherwise {
    vibez.spill("❌ Compilation failed")
}
