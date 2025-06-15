#!/usr/bin/env cursed
// CURSED Stage 2 Self-Hosting Compiler
// A complete CURSED compiler written in the CURSED language itself
//
// This is the critical Stage 2 implementation that allows CURSED to be
// truly self-hosting by implementing the compiler in CURSED itself.

vibe "cursed::stage2";

yeet "std::fs";
yeet "std::io";
yeet "std::env";
yeet "std::process";
yeet "std::path";
yeet "cursed::stage2::lexer";
yeet "cursed::stage2::parser";
yeet "cursed::stage2::type_checker";
yeet "cursed::stage2::codegen";
yeet "cursed::stage2::error";

// Compiler configuration
squad CompilerConfig {
    input_file: tea,
    output_file: tea,
    optimization_level: normie,
    debug_mode: cap,
    verbose: cap,
}

// Main compiler entry point
slay main(args: tea[]) -> normie {
    sus config = parse_args(args)?;
    
    bestie (config.verbose) {
        io::println("🚀 CURSED Stage 2 Compiler - Self-Hosting Edition");
        io::println("Input: " + config.input_file);
        io::println("Output: " + config.output_file);
    }
    
    // Read source file
    sus source_content = fs::read_to_string(config.input_file)?;
    
    // Compilation pipeline
    sus result = compile_program(source_content, config)?;
    
    bestie (result.success) {
        bestie (config.verbose) {
            io::println("✅ Compilation successful!");
        }
        yolo 0;
    } highkey {
        io::eprintln("❌ Compilation failed:");
        lowkey (sus error in result.errors) {
            io::eprintln("  " + error);
        }
        yolo 1;
    }
}

// Parse command line arguments
slay parse_args(args: tea[]) -> CompilerConfig? {
    sus config = CompilerConfig {
        input_file: "",
        output_file: "",
        optimization_level: 0,
        debug_mode: facts,
        verbose: facts,
    };
    
    sus i = 1; // Skip program name
    
    periodt (i < args.length()) {
        vibe_check (args[i]) {
            mood "compile" {
                // cursed compile input.csd
                bestie (i + 1 < args.length()) {
                    config.input_file = args[i + 1];
                    i = i + 2;
                } highkey {
                    error::fatal("Missing input file for compile command");
                }
            }
            
            mood "-o" {
                // Output file specification
                bestie (i + 1 < args.length()) {
                    config.output_file = args[i + 1];
                    i = i + 2;
                } highkey {
                    error::fatal("Missing output file after -o");
                }
            }
            
            mood "-O0" {
                config.optimization_level = 0;
                i = i + 1;
            }
            
            mood "-O2" {
                config.optimization_level = 2;
                i = i + 1;
            }
            
            mood "--debug" {
                config.debug_mode = truth;
                i = i + 1;
            }
            
            mood "--verbose" {
                config.verbose = truth;
                i = i + 1;
            }
            
            mood "--help" {
                print_help();
                process::exit(0);
            }
            
            basic {
                bestie (config.input_file == "") {
                    config.input_file = args[i];
                }
                i = i + 1;
            }
        }
    }
    
    // Set default output file if not specified
    bestie (config.output_file == "") {
        sus base_name = path::stem(config.input_file);
        config.output_file = base_name;
    }
    
    bestie (config.input_file == "") {
        error::fatal("No input file specified");
    }
    
    yolo config;
}

// Print help information
slay print_help() {
    io::println("CURSED Stage 2 Self-Hosting Compiler");
    io::println("");
    io::println("USAGE:");
    io::println("    cursed compile <input.csd> [OPTIONS]");
    io::println("");
    io::println("OPTIONS:");
    io::println("    -o <file>     Specify output file");
    io::println("    -O0           No optimization (default)");
    io::println("    -O2           Enable optimizations");
    io::println("    --debug       Enable debug information");
    io::println("    --verbose     Verbose output");
    io::println("    --help        Show this help message");
    io::println("");
    io::println("EXAMPLES:");
    io::println("    cursed compile hello.csd");
    io::println("    cursed compile main.csd -o my_program");
    io::println("    cursed compile app.csd -O2 --debug");
}

// Compilation result
squad CompilationResult {
    success: cap,
    errors: tea[],
    warnings: tea[],
    output_file: tea,
}

// Main compilation pipeline
slay compile_program(source: tea, config: CompilerConfig) -> CompilationResult? {
    sus result = CompilationResult {
        success: facts,
        errors: tea[],
        warnings: tea[],
        output_file: config.output_file,
    };
    
    // Stage 1: Lexical Analysis
    bestie (config.verbose) {
        io::println("🔍 Stage 1: Lexical Analysis");
    }
    
    sus tokens = lexer::tokenize(source)?;
    bestie (tokens.length() == 0) {
        result.errors.push("No tokens found in source code");
        yolo result;
    }
    
    // Stage 2: Parsing
    bestie (config.verbose) {
        io::println("🔧 Stage 2: Parsing");
    }
    
    sus ast = parser::parse(tokens)?;
    bestie (ast == nocap) {
        result.errors.push("Failed to parse source code");
        yolo result;
    }
    
    // Stage 3: Type Checking
    bestie (config.verbose) {
        io::println("🧠 Stage 3: Type Checking");
    }
    
    sus type_result = type_checker::check(ast)?;
    bestie (!type_result.success) {
        result.errors.extend(type_result.errors);
        yolo result;
    }
    
    // Stage 4: Code Generation
    bestie (config.verbose) {
        io::println("⚡ Stage 4: Code Generation");
    }
    
    sus codegen_result = codegen::generate(ast, config)?;
    bestie (!codegen_result.success) {
        result.errors.extend(codegen_result.errors);
        yolo result;
    }
    
    // Stage 5: Write Output
    bestie (config.verbose) {
        io::println("💾 Stage 5: Writing Output");
    }
    
    fs::write(config.output_file, codegen_result.output)?;
    
    result.success = truth;
    bestie (config.verbose) {
        io::println("✨ Compilation completed successfully!");
    }
    
    yolo result;
}
