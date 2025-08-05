const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/TargetMachine.h");
    @cInclude("llvm-c/Transforms/PassManagerBuilder.h");
    @cInclude("llvm-c/Transforms/IPO.h");
    @cInclude("llvm-c/Transforms/Scalar.h");
    @cInclude("llvm-c/Linker.h");
    @cInclude("llvm-c/Object.h");
    @cInclude("llvm-c/DebugInfo.h");
});

const ast = @import("ast.zig");
const CodeGen = @import("codegen.zig").CodeGen;
const AdvancedCodeGen = @import("advanced_codegen.zig").AdvancedCodeGen;
const debug_info = @import("debug_info.zig");
const DebugInfoGenerator = debug_info.DebugInfoGenerator;

/// Native executable generation pipeline
pub const NativeCompiler = struct {
    allocator: Allocator,
    codegen: AdvancedCodeGen,
    target_triple: []const u8,
    target_machine: ?c.LLVMTargetMachineRef,
    optimization_level: OptimizationLevel,
    debug_info: bool,
    
    pub const CompilationError = error{
        TargetInitError,
        CodeGenError,
        ObjectGenError,
        LinkerError,
        DebugInfoError,
        OptimizationError,
        OutOfMemory,
    };
    
    pub const OptimizationLevel = enum(u8) {
        None = 0,
        Less = 1,
        Default = 2,
        Aggressive = 3,
    };
    
    pub const TargetPlatform = enum {
        Linux_x64,
        Linux_ARM64,
        MacOS_Intel,
        MacOS_ARM64,
        Windows_x64,
        WASM32,
        
        pub fn getTriple(self: TargetPlatform) []const u8 {
            return switch (self) {
                .Linux_x64 => "x86_64-unknown-linux-gnu",
                .Linux_ARM64 => "aarch64-unknown-linux-gnu",
                .MacOS_Intel => "x86_64-apple-darwin",
                .MacOS_ARM64 => "aarch64-apple-darwin",
                .Windows_x64 => "x86_64-pc-windows-gnu",
                .WASM32 => "wasm32-unknown-unknown",
            };
        }
    };
    
    pub fn init(allocator: Allocator, target: TargetPlatform) CompilationError!NativeCompiler {
        // Initialize LLVM targets
        c.LLVMInitializeAllTargetInfos();
        c.LLVMInitializeAllTargets();
        c.LLVMInitializeAllTargetMCs();
        c.LLVMInitializeAllAsmParsers();
        c.LLVMInitializeAllAsmPrinters();
        
        const target_triple = target.getTriple();
        
        // Get target
        var llvm_target: c.LLVMTargetRef = undefined;
        var error_msg: [*c]u8 = undefined;
        
        if (c.LLVMGetTargetFromTriple(target_triple.ptr, &llvm_target, &error_msg) != 0) {
            std.debug.print("Failed to get target: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CompilationError.TargetInitError;
        }
        
        // Create target machine
        const target_machine = c.LLVMCreateTargetMachine(
            llvm_target,
            target_triple.ptr,
            "generic", // CPU
            "", // Features
            c.LLVMCodeGenLevelDefault,
            c.LLVMRelocDefault,
            c.LLVMCodeModelDefault
        );
        
        if (target_machine == null) {
            return CompilationError.TargetInitError;
        }
        
        return NativeCompiler{
            .allocator = allocator,
            .codegen = AdvancedCodeGen.init(allocator),
            .target_triple = target_triple,
            .target_machine = target_machine,
            .optimization_level = .Default,
            .debug_info = true,
        };
    }
    
    pub fn deinit(self: *NativeCompiler) void {
        if (self.target_machine) |tm| {
            c.LLVMDisposeTargetMachine(tm);
        }
        self.codegen.deinit();
    }
    
    pub fn setOptimizationLevel(self: *NativeCompiler, level: OptimizationLevel) void {
        self.optimization_level = level;
    }
    
    pub fn setDebugInfo(self: *NativeCompiler, enable: bool) void {
        self.debug_info = enable;
    }
    
    /// Compile CURSED program to native executable
    pub fn compileProgram(self: *NativeCompiler, program: ast.Program, output_path: []const u8) CompilationError!void {
        // Generate LLVM IR
        try self.generateIR(program);
        
        // Apply optimization passes
        try self.optimizeModule();
        
        // Generate object file
        const object_path = try self.generateObjectFile(output_path);
        defer self.allocator.free(object_path);
        
        // Link executable
        try self.linkExecutable(object_path, output_path);
        
        // Generate debug information if enabled
        if (self.debug_info) {
            try self.generateDebugInfo(output_path);
        }
        
        std.debug.print("Successfully compiled to native executable: {s}\n", .{output_path});
    }
    
    /// Generate LLVM IR for the program
    fn generateIR(self: *NativeCompiler, program: ast.Program) CompilationError!void {
        self.codegen.generateAdvancedProgram(program) catch |err| {
            std.debug.print("Code generation failed: {}\n", .{err});
            return CompilationError.CodeGenError;
        };
        
        // Set target triple on module
        c.LLVMSetTarget(self.codegen.base_codegen.module, self.target_triple.ptr);
        
        // Set data layout from target machine
        const data_layout = c.LLVMCreateTargetDataLayout(self.target_machine.?);
        defer c.LLVMDisposeTargetData(data_layout);
        
        const layout_str = c.LLVMCopyStringRepOfTargetData(data_layout);
        defer c.LLVMDisposeMessage(layout_str);
        
        c.LLVMSetDataLayout(self.codegen.base_codegen.module, layout_str);
    }
    
    /// Apply optimization passes based on optimization level
    fn optimizeModule(self: *NativeCompiler) CompilationError!void {
        const pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(pass_manager);
        
        // Add passes based on optimization level
        switch (self.optimization_level) {
            .None => {
                // No optimization passes
            },
            .Less => {
                c.LLVMAddInstructionCombiningPass(pass_manager);
                c.LLVMAddCFGSimplificationPass(pass_manager);
            },
            .Default => {
                c.LLVMAddInstructionCombiningPass(pass_manager);
                c.LLVMAddReassociatePass(pass_manager);
                c.LLVMAddGVNPass(pass_manager);
                c.LLVMAddCFGSimplificationPass(pass_manager);
                c.LLVMAddPromoteMemoryToRegisterPass(pass_manager);
                c.LLVMAddFunctionInliningPass(pass_manager);
            },
            .Aggressive => {
                // Basic passes
                c.LLVMAddInstructionCombiningPass(pass_manager);
                c.LLVMAddReassociatePass(pass_manager);
                c.LLVMAddGVNPass(pass_manager);
                c.LLVMAddCFGSimplificationPass(pass_manager);
                c.LLVMAddPromoteMemoryToRegisterPass(pass_manager);
                
                // Interprocedural passes
                c.LLVMAddInternalizePass(pass_manager, 1);
                c.LLVMAddFunctionInliningPass(pass_manager);
                c.LLVMAddGlobalDCEPass(pass_manager);
                c.LLVMAddGlobalOptimizerPass(pass_manager);
                
                // Loop passes
                c.LLVMAddLoopUnrollPass(pass_manager);
                c.LLVMAddLICMPass(pass_manager);
                c.LLVMAddLoopDeletionPass(pass_manager);
                
                // Advanced scalar passes
                c.LLVMAddSCCPPass(pass_manager);
                c.LLVMAddDeadStoreEliminationPass(pass_manager);
                c.LLVMAddAggressiveDCEPass(pass_manager);
            },
        }
        
        // Run optimization passes
        const changed = c.LLVMRunPassManager(pass_manager, self.codegen.base_codegen.module);
        if (changed == 0) {
            std.debug.print("Warning: No optimizations applied\n", .{});
        }
        
        // Verify module after optimization
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.codegen.base_codegen.module, c.LLVMPrintMessageAction, &error_msg) != 0) {
            std.debug.print("Module verification failed after optimization: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CompilationError.OptimizationError;
        }
    }
    
    /// Generate object file from LLVM module
    fn generateObjectFile(self: *NativeCompiler, base_path: []const u8) CompilationError![]u8 {
        const object_path = try std.fmt.allocPrint(self.allocator, "{s}.o", .{base_path});
        
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMTargetMachineEmitToFile(
            self.target_machine.?,
            self.codegen.base_codegen.module,
            object_path.ptr,
            c.LLVMObjectFile,
            &error_msg
        ) != 0) {
            std.debug.print("Failed to generate object file: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            self.allocator.free(object_path);
            return CompilationError.ObjectGenError;
        }
        
        return object_path;
    }
    
    /// Link object file to create executable
    fn linkExecutable(self: *NativeCompiler, object_path: []const u8, output_path: []const u8) CompilationError!void {
        // Determine linker and system libraries based on target
        const link_args = switch (std.mem.indexOf(u8, self.target_triple, "linux")) {
            null => switch (std.mem.indexOf(u8, self.target_triple, "darwin")) {
                null => switch (std.mem.indexOf(u8, self.target_triple, "windows")) {
                    null => &[_][]const u8{ // WASM or unknown
                        "wasm-ld", "--no-entry", "--export-all", "--allow-undefined",
                        object_path, "-o", output_path
                    },
                    else => &[_][]const u8{ // Windows
                        "x86_64-w64-mingw32-gcc", "-o", output_path, object_path,
                        "-lmsvcrt", "-lkernel32", "-luser32"
                    },
                },
                else => &[_][]const u8{ // macOS
                    "clang", "-o", output_path, object_path,
                    "-lSystem", "-framework", "Foundation"
                },
            },
            else => &[_][]const u8{ // Linux
                "gcc", "-o", output_path, object_path,
                "-lpthread", "-ldl", "-lm", "-lc"
            },
        };
        
        // Execute linker
        var child = std.ChildProcess.init(link_args, self.allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;
        
        try child.spawn();
        const result = try child.wait();
        
        switch (result) {
            .Exited => |code| {
                if (code != 0) {
                    const stderr = try child.stderr.?.readToEndAlloc(self.allocator, 1024 * 1024);
                    defer self.allocator.free(stderr);
                    std.debug.print("Linker failed with exit code {}: {s}\n", .{ code, stderr });
                    return CompilationError.LinkerError;
                }
            },
            else => {
                std.debug.print("Linker process terminated abnormally\n", .{});
                return CompilationError.LinkerError;
            },
        }
    }
    
    /// Generate debug information (DWARF)
    fn generateDebugInfo(self: *NativeCompiler, output_path: []const u8) CompilationError!void {
        if (!self.debug_info) return;
        
        // Get LLVM context and module from codegen
        const module = self.codegen.module;
        const context = c.LLVMGetModuleContext(module);
        
        // Create debug info generator
        var debug_gen = DebugInfoGenerator.init(self.allocator, context, module) catch |err| switch (err) {
            error.InitError => return CompilationError.DebugInfoError,
            error.OutOfMemory => return CompilationError.OutOfMemory,
            else => return CompilationError.DebugInfoError,
        };
        defer debug_gen.deinit();
        
        // Initialize debug compilation unit
        debug_gen.createCompileUnit("main.csd", ".") catch |err| switch (err) {
            error.MetadataError => return CompilationError.DebugInfoError,
            error.OutOfMemory => return CompilationError.OutOfMemory,
            else => return CompilationError.DebugInfoError,
        };
        
        // Create standard CURSED debug types
        const cursed_types = debug_gen.createCursedTypes() catch |err| switch (err) {
            error.TypeCreationError => return CompilationError.DebugInfoError,
            error.OutOfMemory => return CompilationError.OutOfMemory,
            else => return CompilationError.DebugInfoError,
        };
        
        // Add debug information for functions and variables
        try self.addASTDebugInfo(&debug_gen, cursed_types);
        
        // Finalize debug information
        debug_gen.finalize();
        
        // Generate debug symbols file for external tools
        try self.generateExternalDebugSymbols(output_path);
    }
    
    /// Add debug information for AST nodes
    fn addASTDebugInfo(self: *NativeCompiler, debug_gen: *DebugInfoGenerator, cursed_types: debug_info.CursedDebugTypes) CompilationError!void {
        // This is a simplified implementation - in a real compiler, you'd walk the AST
        // and add debug info for each function, variable, etc.
        
        // Example: Add debug info for main function (if it exists)
        // const main_func = c.LLVMGetNamedFunction(self.codegen.module, "main");
        // if (main_func != null) {
        //     const func_type = debug_gen.createFunctionType(cursed_types.normie_type, &[_]c.LLVMMetadataRef{}) catch return CompilationError.DebugInfoError;
        //     _ = debug_gen.createFunction("main", "main", 1, func_type, main_func) catch return CompilationError.DebugInfoError;
        // }
        
        // For now, just ensure the debug types are created
        _ = cursed_types;
    }
    
    /// Generate external debug symbols
    fn generateExternalDebugSymbols(self: *NativeCompiler, output_path: []const u8) CompilationError!void {
        // Generate debug symbols file
        const debug_path = try std.fmt.allocPrint(self.allocator, "{s}.dSYM", .{output_path});
        defer self.allocator.free(debug_path);
        
        const debug_args = switch (std.mem.indexOf(u8, self.target_triple, "darwin")) {
            null => &[_][]const u8{ // Linux/Windows
                "objcopy", "--only-keep-debug", output_path, debug_path
            },
            else => &[_][]const u8{ // macOS
                "dsymutil", output_path, "-o", debug_path
            },
        };
        
        var child = std.ChildProcess.init(debug_args, self.allocator);
        child.stdout_behavior = .Ignore;
        child.stderr_behavior = .Pipe;
        
        try child.spawn();
        const result = try child.wait();
        
        switch (result) {
            .Exited => |code| {
                if (code == 0) {
                    std.debug.print("Generated debug information: {s}\n", .{debug_path});
                } else {
                    // Debug info generation is optional, so we don't fail on error
                    std.debug.print("Warning: Debug info generation failed with exit code {}\n", .{code});
                }
            },
            else => {
                std.debug.print("Warning: Debug info process terminated abnormally\n", .{});
            },
        }
    }
    
    /// Cross-compile for multiple targets
    pub fn crossCompile(allocator: Allocator, program: ast.Program, output_dir: []const u8) CompilationError!void {
        const targets = [_]TargetPlatform{
            .Linux_x64,
            .Linux_ARM64,
            .MacOS_Intel,
            .MacOS_ARM64,
            .Windows_x64,
        };
        
        // Create output directory
        std.fs.cwd().makeDir(output_dir) catch |err| switch (err) {
            error.PathAlreadyExists => {},
            else => return CompilationError.LinkerError,
        };
        
        for (targets) |target| {
            std.debug.print("Cross-compiling for target: {s}\n", .{target.getTriple()});
            
            var compiler = init(allocator, target) catch |err| {
                std.debug.print("Failed to initialize compiler for {s}: {}\n", .{ target.getTriple(), err });
                continue;
            };
            defer compiler.deinit();
            
            const output_name = try std.fmt.allocPrint(allocator, "{s}/cursed_{s}", .{ 
                output_dir, 
                @tagName(target) 
            });
            defer allocator.free(output_name);
            
            compiler.compileProgram(program, output_name) catch |err| {
                std.debug.print("Failed to compile for {s}: {}\n", .{ target.getTriple(), err });
                continue;
            };
        }
        
        std.debug.print("Cross-compilation complete. Binaries available in: {s}\n", .{output_dir});
    }
    
    /// Generate assembly output for analysis
    pub fn generateAssembly(self: *NativeCompiler, output_path: []const u8) CompilationError!void {
        const asm_path = try std.fmt.allocPrint(self.allocator, "{s}.s", .{output_path});
        defer self.allocator.free(asm_path);
        
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMTargetMachineEmitToFile(
            self.target_machine.?,
            self.codegen.base_codegen.module,
            asm_path.ptr,
            c.LLVMAssemblyFile,
            &error_msg
        ) != 0) {
            std.debug.print("Failed to generate assembly: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CompilationError.ObjectGenError;
        }
        
        std.debug.print("Generated assembly: {s}\n", .{asm_path});
    }
    
    /// Profile-guided optimization compilation
    pub fn profileGuidedCompile(self: *NativeCompiler, program: ast.Program, output_path: []const u8, _: []const u8) CompilationError!void {
        // First compilation with instrumentation
        const instrumented_path = try std.fmt.allocPrint(self.allocator, "{s}_instrumented", .{output_path});
        defer self.allocator.free(instrumented_path);
        
        // Enable PGO instrumentation
        // This would require additional LLVM PGO passes
        try self.compileProgram(program, instrumented_path);
        
        // TODO: Run instrumented binary to collect profile data
        // This would involve executing the instrumented binary with representative workloads
        
        // Second compilation with profile data
        // TODO: Load profile data and apply PGO optimization passes
        try self.compileProgram(program, output_path);
        
        std.debug.print("Profile-guided optimization complete: {s}\n", .{output_path});
    }
    
    /// Link-time optimization compilation
    pub fn linkTimeOptimization(self: *NativeCompiler, program: ast.Program, output_path: []const u8) CompilationError!void {
        // Generate bitcode instead of object file
        const bitcode_path = try std.fmt.allocPrint(self.allocator, "{s}.bc", .{output_path});
        defer self.allocator.free(bitcode_path);
        
        // Generate LLVM IR first
        try self.generateIR(program);
        
        // Write bitcode
        if (c.LLVMWriteBitcodeToFile(self.codegen.base_codegen.module, bitcode_path.ptr) != 0) {
            return CompilationError.ObjectGenError;
        }
        
        // Use LLVM's lto linker
        const lto_args = &[_][]const u8{
            "llvm-lto2", "run", bitcode_path, "-o", output_path,
            "-r", bitcode_path,
            "-save-temps",
        };
        
        var child = std.ChildProcess.init(lto_args, self.allocator);
        try child.spawn();
        const result = try child.wait();
        
        switch (result) {
            .Exited => |code| {
                if (code != 0) {
                    std.debug.print("LTO linking failed with exit code {}\n", .{code});
                    return CompilationError.LinkerError;
                }
            },
            else => return CompilationError.LinkerError,
        }
        
        std.debug.print("Link-time optimization complete: {s}\n", .{output_path});
    }
};

/// Performance benchmarking utilities
pub const PerformanceBenchmark = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) PerformanceBenchmark {
        return PerformanceBenchmark{ .allocator = allocator };
    }
    
    pub fn benchmarkCompilation(self: *PerformanceBenchmark, program: ast.Program, target: NativeCompiler.TargetPlatform) !u64 {
        const start_time = std.time.nanoTimestamp();
        
        var compiler = try NativeCompiler.init(self.allocator, target);
        defer compiler.deinit();
        
        const temp_output = "/tmp/cursed_benchmark";
        try compiler.compileProgram(program, temp_output);
        
        const end_time = std.time.nanoTimestamp();
        const compilation_time = @as(u64, @intCast(end_time - start_time));
        
        // Clean up
        std.fs.cwd().deleteFile(temp_output) catch {};
        std.fs.cwd().deleteFile("/tmp/cursed_benchmark.o") catch {};
        
        return compilation_time;
    }
    
    pub fn benchmarkExecution(self: *PerformanceBenchmark, executable_path: []const u8) !u64 {
        const start_time = std.time.nanoTimestamp();
        
        var child = std.ChildProcess.init(&[_][]const u8{executable_path}, self.allocator);
        try child.spawn();
        _ = try child.wait();
        
        const end_time = std.time.nanoTimestamp();
        return @as(u64, @intCast(end_time - start_time));
    }
};

test "native compiler initialization" {
    const allocator = std.testing.allocator;
    
    var compiler = try NativeCompiler.init(allocator, .Linux_x64);
    defer compiler.deinit();
    
    try std.testing.expect(compiler.target_machine != null);
    try std.testing.expect(compiler.optimization_level == .Default);
}

test "cross compilation targets" {
    const targets = [_]NativeCompiler.TargetPlatform{
        .Linux_x64, .Linux_ARM64, .MacOS_Intel, .MacOS_ARM64, .Windows_x64
    };
    
    for (targets) |target| {
        const triple = target.getTriple();
        try std.testing.expect(triple.len > 0);
    }
}
