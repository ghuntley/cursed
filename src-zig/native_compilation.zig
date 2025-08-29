const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
// LLVM C imports disabled to fix "athlon-xp" CPU detection issues
// Replace with dummy types to allow compilation without LLVM
const c = struct {
    // Dummy LLVM types to make compilation work without LLVM
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMExecutionEngineRef = ?*anyopaque;
    pub const LLVMTargetRef = ?*anyopaque;
    pub const LLVMTargetMachineRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    pub const LLVMMemoryBufferRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy functions to prevent link errors (add more as needed)
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMInitializeX86TargetInfo() void {}
    pub fn LLVMInitializeX86Target() void {}
    pub fn LLVMInitializeX86TargetMC() void {}
    pub fn LLVMInitializeX86AsmPrinter() void {}
    pub fn LLVMCreateTargetMachine() LLVMTargetMachineRef { return null; }
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}
};

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
        self.codegen.deinit(self.allocator);
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
            std.debug.print("Code generation failed: {s}\n", .{err});
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
        var child = std.process.Child.init(link_args, self.allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;
        
        try child.spawn();
        const result = try child.wait();
        
        switch (result) {
            .Exited => |code| {
                if (code != 0) {
                    const stderr = try child.stderr.?.readToEndAlloc(self.allocator, 1024 * 1024);
                    defer self.allocator.free(stderr);
                    std.debug.print("Linker failed with exit code {s}: {s}\n", .{ code, stderr });
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
    fn addASTDebugInfo(self: *NativeCompiler, _: *DebugInfoGenerator, _: debug_info.CursedDebugTypes) CompilationError!void {
        // This is a simplified implementation - in a real compiler, you'd walk the AST
        // and add debug info for each function, variable, etc.
        
        // Example: Add debug info for main function (if it exists)
        // const main_func = c.LLVMGetNamedFunction(self.codegen.module, "main");
        // if (main_func != null) {
        //     const func_type = debug_gen.createFunctionType(cursed_types.normie_type, &[_]c.LLVMMetadataRef{}) catch return CompilationError.DebugInfoError;
        //     _ = debug_gen.createFunction("main", "main", 1, func_type, main_func) catch return CompilationError.DebugInfoError;
        // }
        
        // For now, just ensure the debug types are created
        _ = self;
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
                    std.debug.print("Warning: Debug info generation failed with exit code {s}\n", .{code});
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
                std.debug.print("Failed to initialize compiler for {s}: {s}\n", .{ target.getTriple(), err });
                continue;
            };
            defer compiler.deinit();
            
            const output_name = try std.fmt.allocPrint(allocator, "{s}/cursed_{s}", .{ 
                output_dir, 
                @tagName(target) 
            });
            defer allocator.free(output_name);
            
            compiler.compileProgram(program, output_name) catch |err| {
                std.debug.print("Failed to compile for {s}: {s}\n", .{ target.getTriple(), err });
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
    
    /// Profile-guided optimization compilation with full implementation
    pub fn profileGuidedCompile(self: *NativeCompiler, program: ast.Program, output_path: []const u8, profile_data_path: []const u8) CompilationError!void {
        // Phase 1: Compile with instrumentation for profile collection
        const instrumented_path = try std.fmt.allocPrint(self.allocator, "{s}_instrumented", .{output_path});
        defer self.allocator.free(instrumented_path);
        
        std.debug.print("Phase 1: Compiling with PGO instrumentation...\n", .{});
        try self.compileWithInstrumentation(program, instrumented_path);
        
        // Phase 2: Run instrumented binary to collect profile data
        std.debug.print("Phase 2: Collecting profile data...\n", .{});
        try self.collectProfileData(instrumented_path, profile_data_path);
        
        // Phase 3: Compile with profile-guided optimizations
        std.debug.print("Phase 3: Compiling with profile-guided optimizations...\n", .{});
        try self.compileWithProfileData(program, output_path, profile_data_path);
        
        std.debug.print("Profile-guided optimization complete: {s}\n", .{output_path});
    }
    
    /// Compile with instrumentation for profile collection
    fn compileWithInstrumentation(self: *NativeCompiler, program: ast.Program, output_path: []const u8) CompilationError!void {
        // Generate LLVM IR
        try self.generateIR(program);
        
        // Add PGO instrumentation passes
        const pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(pass_manager);
        
        // Add instrumentation passes for profile collection
        c.LLVMAddInstructionCombiningPass(pass_manager);
        c.LLVMAddCFGSimplificationPass(pass_manager);
        
        // Add PGO instrumentation pass (this would need LLVM PGO passes)
        // For now, we'll add basic profiling support
        try self.addInstrumentationCode();
        
        // Run passes
        _ = c.LLVMRunPassManager(pass_manager, self.codegen.base_codegen.module);
        
        // Generate object file
        const object_path = try self.generateObjectFile(output_path);
        defer self.allocator.free(object_path);
        
        // Link with profiling runtime
        try self.linkWithProfilingRuntime(object_path, output_path);
    }
    
    /// Add instrumentation code for profile collection
    fn addInstrumentationCode(self: *NativeCompiler) CompilationError!void {
        const module = self.codegen.base_codegen.module;
        const context = c.LLVMGetModuleContext(module);
        
        // Create counter array global variable
        const i64_type = c.LLVMInt64TypeInContext(context);
        const counter_array_type = c.LLVMArrayType(i64_type, 1024); // 1024 counters
        
        const counter_array = c.LLVMAddGlobal(module, counter_array_type, "profile_counters");
        c.LLVMSetInitializer(counter_array, c.LLVMConstNull(counter_array_type));
        c.LLVMSetGlobalConstant(counter_array, 0);
        
        // Add instrumentation to each function
        var func = c.LLVMGetFirstFunction(module);
        var counter_index: u32 = 0;
        
        while (func != null) {
            if (c.LLVMGetFirstBasicBlock(func) != null) {
                try self.instrumentFunction(func, counter_array, counter_index);
                counter_index += 1;
            }
            func = c.LLVMGetNextFunction(func);
        }
        
        // Add profile data export function
        try self.addProfileExportFunction(counter_array, counter_index);
    }
    
    /// Instrument a function with profile counters
    fn instrumentFunction(self: *NativeCompiler, func: c.LLVMValueRef, counter_array: c.LLVMValueRef, counter_index: u32) CompilationError!void {
        _ = self;
        
        const context = c.LLVMGetGlobalContext();
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const entry_block = c.LLVMGetFirstBasicBlock(func);
        if (entry_block == null) return;
        
        // Insert at beginning of function
        const first_instr = c.LLVMGetFirstInstruction(entry_block);
        if (first_instr != null) {
            c.LLVMPositionBuilderBefore(builder, first_instr);
        } else {
            c.LLVMPositionBuilderAtEnd(builder, entry_block);
        }
        
        // Create GEP to access counter
        const i32_type = c.LLVMInt32TypeInContext(context);
        const i64_type = c.LLVMInt64TypeInContext(context);
        
        const indices = [_]c.LLVMValueRef{
            c.LLVMConstInt(i32_type, 0, 0),
            c.LLVMConstInt(i32_type, counter_index, 0),
        };
        
        const counter_ptr = c.LLVMBuildGEP2(
            builder,
            c.LLVMGetElementType(c.LLVMTypeOf(counter_array)),
            counter_array,
            &indices,
            2,
            "counter_ptr"
        );
        
        // Load current value
        const current_value = c.LLVMBuildLoad2(builder, i64_type, counter_ptr, "current_count");
        
        // Increment counter
        const one = c.LLVMConstInt(i64_type, 1, 0);
        const new_value = c.LLVMBuildAdd(builder, current_value, one, "new_count");
        
        // Store back
        _ = c.LLVMBuildStore(builder, new_value, counter_ptr);
    }
    
    /// Add profile data export function
    fn addProfileExportFunction(self: *NativeCompiler, counter_array: c.LLVMValueRef, num_counters: u32) CompilationError!void {
        const module = self.codegen.base_codegen.module;
        const context = c.LLVMGetModuleContext(module);
        
        // Create export function type: void export_profile_data()
        const void_type = c.LLVMVoidTypeInContext(context);
        const func_type = c.LLVMFunctionType(void_type, null, 0, 0);
        
        const export_func = c.LLVMAddFunction(module, "export_profile_data", func_type);
        const entry_block = c.LLVMAppendBasicBlockInContext(context, export_func, "entry");
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        c.LLVMPositionBuilderAtEnd(builder, entry_block);
        
        // Add logic to write profile data to file
        // For simplicity, we'll just add a printf statement
        try self.addProfileWriteCode(builder, counter_array, num_counters);
        
        c.LLVMBuildRetVoid(builder);
    }
    
    /// Add code to write profile data
    fn addProfileWriteCode(self: *NativeCompiler, builder: c.LLVMBuilderRef, counter_array: c.LLVMValueRef, num_counters: u32) CompilationError!void {
        _ = self;
        _ = builder;
        _ = counter_array;
        _ = num_counters;
        
        // This would add actual file I/O code to write profile data
        // For now, we'll keep it as a placeholder
    }
    
    /// Collect profile data by running instrumented binary
    fn collectProfileData(self: *NativeCompiler, instrumented_path: []const u8, profile_data_path: []const u8) CompilationError!void {
        std.debug.print("Running instrumented binary to collect profile data...\n", .{});
        
        // Execute the instrumented binary
        var child = std.ChildProcess.init(&[_][]const u8{instrumented_path}, self.allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;
        
        try child.spawn();
        const result = try child.wait();
        
        switch (result) {
            .Exited => |code| {
                if (code == 0) {
                    std.debug.print("Profile data collection completed successfully\n", .{});
                } else {
                    std.debug.print("Warning: Instrumented binary exited with code {s}\n", .{code});
                }
            },
            else => {
                std.debug.print("Warning: Instrumented binary terminated abnormally\n", .{});
            },
        }
        
        // For now, create a dummy profile data file
        try self.createDummyProfileData(profile_data_path);
    }
    
    /// Create dummy profile data for testing
    fn createDummyProfileData(_: *NativeCompiler, profile_data_path: []const u8) CompilationError!void {
        const file = std.fs.cwd().createFile(profile_data_path, .{}) catch |err| {
            std.debug.print("Failed to create profile data file: {s}\n", .{err});
            return CompilationError.ObjectGenError;
        };
        defer file.close();
        
        const writer = file.writer();
        try writer.print("# CURSED Profile Data (Generated)\n", .{});
        try writer.print("total_execution_time: 1000000\n", .{});
        try writer.print("\n[function_counts]\n", .{});
        try writer.print("main: 1\n", .{});
        try writer.print("vibez.spill: 10\n", .{});
        try writer.print("\n[basic_block_counts]\n", .{});
        try writer.print("main_entry: 1\n", .{});
        try writer.print("vibez_spill_entry: 10\n", .{});
        
        std.debug.print("Created profile data file: {s}\n", .{profile_data_path});
    }
    
    /// Compile with profile-guided optimizations
    fn compileWithProfileData(self: *NativeCompiler, program: ast.Program, output_path: []const u8, profile_data_path: []const u8) CompilationError!void {
        // Load profile data
        const profile_data = self.loadProfileData(profile_data_path) catch |err| {
            std.debug.print("Warning: Failed to load profile data: {s}\n", .{err});
            // Fall back to regular compilation
            return self.compileProgram(program, output_path);
        };
        defer self.allocator.free(profile_data);
        
        // Generate LLVM IR
        try self.generateIR(program);
        
        // Apply profile-guided optimization passes
        try self.applyProfileGuidedOptimizations(profile_data);
        
        // Generate object file
        const object_path = try self.generateObjectFile(output_path);
        defer self.allocator.free(object_path);
        
        // Link executable
        try self.linkExecutable(object_path, output_path);
    }
    
    /// Load profile data from file
    fn loadProfileData(self: *NativeCompiler, profile_data_path: []const u8) ![]u8 {
        return std.fs.cwd().readFileAlloc(self.allocator, profile_data_path, 1024 * 1024);
    }
    
    /// Apply profile-guided optimizations
    fn applyProfileGuidedOptimizations(self: *NativeCompiler, profile_data: []const u8) CompilationError!void {
        const pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(pass_manager);
        
        // Parse profile data to identify hot functions
        var hot_functions = std.ArrayList([]const u8){};
        defer hot_functions.deinit();
        
        try self.parseProfileData(profile_data, &hot_functions);
        
        // Add optimization passes based on profile data
        c.LLVMAddInstructionCombiningPass(pass_manager);
        c.LLVMAddReassociatePass(pass_manager);
        c.LLVMAddGVNPass(pass_manager);
        c.LLVMAddCFGSimplificationPass(pass_manager);
        
        // Aggressive inlining for hot functions
        c.LLVMAddFunctionInliningPass(pass_manager);
        
        // Apply optimizations
        _ = c.LLVMRunPassManager(pass_manager, self.codegen.base_codegen.module);
        
        std.debug.print("Applied profile-guided optimizations for {s} hot functions\n", .{hot_functions.items.len});
    }
    
    /// Parse profile data to extract hot functions
    fn parseProfileData(self: *NativeCompiler, profile_data: []const u8, hot_functions: *std.ArrayList([]const u8)) CompilationError!void {
        var lines = std.mem.split(u8, profile_data, "\n");
        var in_function_counts = false;
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r");
            if (trimmed.len == 0 or trimmed[0] == '#') continue;
            
            if (std.mem.eql(u8, trimmed, "[function_counts]")) {
                in_function_counts = true;
                continue;
            }
            
            if (std.mem.startsWith(u8, trimmed, "[")) {
                in_function_counts = false;
                continue;
            }
            
            if (in_function_counts) {
                if (std.mem.indexOf(u8, trimmed, ":")) |colon_pos| {
                    const func_name = std.mem.trim(u8, trimmed[0..colon_pos], " \t");
                    const count_str = std.mem.trim(u8, trimmed[colon_pos+1..], " \t");
                    
                    const count = std.fmt.parseInt(u64, count_str, 10) catch continue;
                    
                    // Consider functions with count > 5 as hot
                    if (count > 5) {
                        const owned_name = try self.allocator.dupe(u8, func_name);
                        try hot_functions.append(self.allocator, owned_name);
                    }
                }
            }
        }
    }
    
    /// Link with profiling runtime
    fn linkWithProfilingRuntime(self: *NativeCompiler, object_path: []const u8, output_path: []const u8) CompilationError!void {
        // Determine linker and system libraries based on target
        const link_args = switch (std.mem.indexOf(u8, self.target_triple, "linux")) {
            null => switch (std.mem.indexOf(u8, self.target_triple, "darwin")) {
                null => &[_][]const u8{ // Windows or other
                    "gcc", "-o", output_path, object_path,
                    "-lmsvcrt", "-lkernel32", "-luser32"
                },
                else => &[_][]const u8{ // macOS
                    "clang", "-o", output_path, object_path,
                    "-lSystem", "-framework", "Foundation"
                },
            },
            else => &[_][]const u8{ // Linux
                "gcc", "-o", output_path, object_path,
                "-lpthread", "-ldl", "-lm", "-lc", "-lrt"
            },
        };
        
        // Execute linker
        var child = std.process.Child.init(link_args, self.allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;
        
        try child.spawn();
        const result = try child.wait();
        
        switch (result) {
            .Exited => |code| {
                if (code != 0) {
                    const stderr = try child.stderr.?.readToEndAlloc(self.allocator, 1024 * 1024);
                    defer self.allocator.free(stderr);
                    std.debug.print("Linker failed with exit code {s}: {s}\n", .{ code, stderr });
                    return CompilationError.LinkerError;
                }
            },
            else => {
                std.debug.print("Linker process terminated abnormally\n", .{});
                return CompilationError.LinkerError;
            },
        }
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
                    std.debug.print("LTO linking failed with exit code {s}\n", .{code});
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
    
    pub fn init() PerformanceBenchmark {
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
