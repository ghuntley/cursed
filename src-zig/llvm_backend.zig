const std = @import("std");
const print = std.debug.print;
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

const lexer = @import("lexer.zig");

pub const LLVMBackendError = error{
    LLVMError,
    OutOfMemory,
    InvalidType,
    UndefinedSymbol,
    TypeMismatch,
    CompilationFailed,
};

pub const LLVMBackend = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    
    // Symbol tables
    functions: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Current state
    current_function: ?c.LLVMValueRef,
    string_counter: u32,
    
    pub fn init(allocator: Allocator, module_name: []const u8) LLVMBackendError!LLVMBackend {
        // Initialize LLVM
        _ = c.LLVMInitializeNativeTarget();
        _ = c.LLVMInitializeNativeAsmPrinter();
        _ = c.LLVMInitializeNativeAsmParser();
        
        const context = c.LLVMContextCreate();
        const module = c.LLVMModuleCreateWithNameInContext(module_name.ptr, context);
        const builder = c.LLVMCreateBuilderInContext(context);
        
        // Set target triple
        const target_triple = c.LLVMGetDefaultTargetTriple();
        c.LLVMSetTarget(module, target_triple);
        c.LLVMDisposeMessage(target_triple);
        
        return LLVMBackend{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .functions = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
            .string_counter = 0,
        };
    }
    
    pub fn deinit(self: *LLVMBackend) void {
        self.functions.deinit();
        self.variables.deinit();
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
    }
    
    pub fn compileSource(self: *LLVMBackend, source: []const u8, optimize_level: u8, verbose: bool) LLVMBackendError!void {
        if (verbose) print("🔥 Starting LLVM IR generation...\n", .{});
        
        // 1. Setup external declarations
        try self.setupExternalDeclarations();
        
        // 2. Parse and generate IR from source
        try self.generateFromSource(source, verbose);
        
        // 3. Create main function wrapper if needed
        try self.ensureMainFunction();
        
        // 4. Verify module
        try self.verifyModule();
        
        // 5. Apply optimizations
        if (optimize_level > 0) {
            try self.applyOptimizations(optimize_level, verbose);
        }
        
        if (verbose) print("✅ LLVM IR generation complete\n", .{});
    }
    
    pub fn writeToFile(self: *LLVMBackend, filename: []const u8) LLVMBackendError!void {
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMPrintModuleToFile(self.module, filename.ptr, &error_msg) != 0) {
            std.debug.print("Failed to write LLVM IR to file: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return LLVMBackendError.LLVMError;
        }
    }
    
    pub fn compileToNative(self: *LLVMBackend, output_filename: []const u8, verbose: bool) LLVMBackendError!void {
        if (verbose) print("🚀 Compiling to native executable...\n", .{});
        
        // Generate temporary LLVM IR file
        const ir_filename = try std.fmt.allocPrint(self.allocator, "{s}.ll", .{output_filename});
        defer self.allocator.free(ir_filename);
        
        try self.writeToFile(ir_filename);
        
        // Use clang to compile LLVM IR to native executable
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();
        
        var child = std.process.Child.init(&[_][]const u8{
            "clang",
            "-o", output_filename,
            ir_filename,
            "-lm", // Link math library
        }, arena_allocator);
        
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;
        
        const result = child.spawnAndWait() catch |err| {
            print("❌ Failed to spawn clang: {}\n", .{err});
            return LLVMBackendError.CompilationFailed;
        };
        
        switch (result) {
            .Exited => |code| {
                if (code != 0) {
                    print("❌ clang compilation failed with exit code: {}\n", .{code});
                    return LLVMBackendError.CompilationFailed;
                }
            },
            else => {
                print("❌ clang process terminated abnormally\n", .{});
                return LLVMBackendError.CompilationFailed;
            },
        }
        
        // Cleanup temporary IR file
        std.fs.cwd().deleteFile(ir_filename) catch {};
        
        if (verbose) print("✅ Native compilation successful: {s}\n", .{output_filename});
    }
    
    fn setupExternalDeclarations(self: *LLVMBackend) LLVMBackendError!void {
        // Declare printf for vibez.spill
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return i32
            @as([*c]c.LLVMTypeRef, @ptrCast(@constCast(&[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)
            }))),
            1, // parameter count
            1  // is variadic
        );
        const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        try self.functions.put("printf", printf_func);
        
        // Declare puts for string output
        const puts_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return i32
            @as([*c]c.LLVMTypeRef, @ptrCast(@constCast(&[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)
            }))),
            1, // parameter count
            0  // not variadic
        );
        const puts_func = c.LLVMAddFunction(self.module, "puts", puts_type);
        try self.functions.put("puts", puts_func);
    }
    
    fn generateFromSource(self: *LLVMBackend, source: []const u8, verbose: bool) LLVMBackendError!void {
        // First pass: collect string literals for global constants
        var string_literals = std.ArrayList([]const u8).init(self.allocator);
        defer {
            for (string_literals.items) |str| {
                self.allocator.free(str);
            }
            string_literals.deinit();
        }
        
        try self.collectStringLiterals(source, &string_literals);
        
        // Generate global string constants
        for (string_literals.items, 0..) |str_content, i| {
            try self.createGlobalString(str_content, i);
        }
        
        // Create main function
        const main_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return i32
            null, // no parameters
            0, // parameter count
            0  // not variadic
        );
        const main_func = c.LLVMAddFunction(self.module, "main", main_type);
        try self.functions.put("main", main_func);
        self.current_function = main_func;
        
        // Create entry basic block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Generate code from source
        try self.generateStatementsFromSource(source, &string_literals, verbose);
        
        // Return 0 from main
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
    }
    
    fn collectStringLiterals(self: *LLVMBackend, source: []const u8, string_literals: *std.ArrayList([]const u8)) LLVMBackendError!void {
        var lines = std.mem.splitScalar(u8, source, '\n');
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            
            // Look for vibez.spill() with string literals
            if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
                if (std.mem.indexOf(u8, trimmed[start..], "(")) |paren_start| {
                    if (std.mem.lastIndexOf(u8, trimmed, ")")) |paren_end| {
                        const content_start = start + paren_start + 1;
                        const content = trimmed[content_start..paren_end];
                        
                        if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                            const string_content = content[1..content.len - 1];
                            const string_copy = try self.allocator.dupe(u8, string_content);
                            try string_literals.append(string_copy);
                        }
                    }
                }
            }
        }
    }
    
    fn createGlobalString(self: *LLVMBackend, content: []const u8, index: usize) LLVMBackendError!void {
        // Create string constant
        const string_value = c.LLVMConstStringInContext(self.context, content.ptr, @intCast(content.len), 0);
        const string_type = c.LLVMTypeOf(string_value);
        
        // Create global variable
        const global_name = try std.fmt.allocPrint(self.allocator, "str_{}", .{index});
        defer self.allocator.free(global_name);
        
        const global_var = c.LLVMAddGlobal(self.module, string_type, global_name.ptr);
        c.LLVMSetInitializer(global_var, string_value);
        c.LLVMSetLinkage(global_var, c.LLVMPrivateLinkage);
        c.LLVMSetUnnamedAddr(global_var, 1);
    }
    
    fn generateStatementsFromSource(self: *LLVMBackend, source: []const u8, string_literals: *std.ArrayList([]const u8), verbose: bool) LLVMBackendError!void {
        var lines = std.mem.splitScalar(u8, source, '\n');
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            
            // Skip empty lines and comments
            if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
                continue;
            }
            
            // Skip imports
            if (std.mem.startsWith(u8, trimmed, "yeet ")) {
                continue;
            }
            
            // Handle vibez.spill() statements
            if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |_| {
                try self.generateVibesSpill(trimmed, string_literals, verbose);
            }
            
            // Handle variable declarations
            if (std.mem.startsWith(u8, trimmed, "sus ")) {
                try self.generateVariableDeclaration(trimmed, verbose);
            }
        }
    }
    
    fn generateVibesSpill(self: *LLVMBackend, line: []const u8, string_literals: *std.ArrayList([]const u8), verbose: bool) LLVMBackendError!void {
        if (std.mem.indexOf(u8, line, "vibez.spill(")) |start| {
            if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
                if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
                    const content_start = start + paren_start + 1;
                    const content = line[content_start..paren_end];
                    
                    if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                        // String literal
                        const string_content = content[1..content.len - 1];
                        
                        // Find the string in our global constants
                        var string_index: ?usize = null;
                        for (string_literals.items, 0..) |str_const, i| {
                            if (std.mem.eql(u8, str_const, string_content)) {
                                string_index = i;
                                break;
                            }
                        }
                        
                        if (string_index) |index| {
                            try self.generateStringPrint(index, verbose);
                        }
                    } else {
                        // Variable or numeric literal
                        if (std.fmt.parseInt(i64, content, 10)) |num| {
                            try self.generateNumberPrint(num, verbose);
                        } else |_| {
                            // Variable reference
                            if (self.variables.get(content)) |var_ptr| {
                                try self.generateVariablePrint(var_ptr, content, verbose);
                            } else {
                                if (verbose) print("⚠️  Unknown variable: {s}\n", .{content});
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn generateStringPrint(self: *LLVMBackend, string_index: usize, verbose: bool) LLVMBackendError!void {
        if (verbose) print("  Generating string print for index {}\n", .{string_index});
        
        // Get global string
        const global_name = try std.fmt.allocPrint(self.allocator, "str_{}", .{string_index});
        defer self.allocator.free(global_name);
        
        const global_var = c.LLVMGetNamedGlobal(self.module, global_name.ptr);
        if (global_var == null) {
            return LLVMBackendError.UndefinedSymbol;
        }
        
        // Get pointer to first character
        const indices = [_]c.LLVMValueRef{
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
        };
        
        const str_ptr = c.LLVMBuildGEP2(
            self.builder,
            c.LLVMGlobalGetValueType(global_var.?),
            global_var.?,
            @as([*c]c.LLVMValueRef, @ptrCast(@constCast(&indices))),
            2,
            "str_ptr"
        );
        
        // Call puts
        const puts_func = self.functions.get("puts").?;
        const args = [_]c.LLVMValueRef{str_ptr};
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGlobalGetValueType(puts_func),
            puts_func,
            @as([*c]c.LLVMValueRef, @ptrCast(@constCast(&args))),
            1,
            "puts_call"
        );
    }
    
    fn generateNumberPrint(self: *LLVMBackend, number: i64, verbose: bool) LLVMBackendError!void {
        if (verbose) print("  Generating number print: {}\n", .{number});
        
        // Create format string for printf
        const format_str = "%lld\n";
        const format_value = c.LLVMConstStringInContext(self.context, format_str.ptr, format_str.len, 0);
        const format_type = c.LLVMTypeOf(format_value);
        
        const format_global = c.LLVMAddGlobal(self.module, format_type, "int_fmt");
        c.LLVMSetInitializer(format_global, format_value);
        c.LLVMSetLinkage(format_global, c.LLVMPrivateLinkage);
        
        // Get pointer to format string
        const indices = [_]c.LLVMValueRef{
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
        };
        
        const fmt_ptr = c.LLVMBuildGEP2(
            self.builder,
            format_type,
            format_global,
            @as([*c]c.LLVMValueRef, @ptrCast(@constCast(&indices))),
            2,
            "fmt_ptr"
        );
        
        // Create number value
        const num_value = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @intCast(number), 0);
        
        // Call printf
        const printf_func = self.functions.get("printf").?;
        const args = [_]c.LLVMValueRef{ fmt_ptr, num_value };
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGlobalGetValueType(printf_func),
            printf_func,
            @as([*c]c.LLVMValueRef, @ptrCast(@constCast(&args))),
            2,
            "printf_call"
        );
    }
    
    fn generateVariablePrint(self: *LLVMBackend, var_ptr: c.LLVMValueRef, var_name: []const u8, verbose: bool) LLVMBackendError!void {
        if (verbose) print("  Generating variable print: {s}\n", .{var_name});
        
        // Load variable value
        const var_type = c.LLVMGetElementType(c.LLVMTypeOf(var_ptr));
        const loaded_value = c.LLVMBuildLoad2(self.builder, var_type, var_ptr, "loaded_val");
        
        // For now, assume integer type and print accordingly
        const format_str = "%lld\n";
        const format_value = c.LLVMConstStringInContext(self.context, format_str.ptr, format_str.len, 0);
        const format_type = c.LLVMTypeOf(format_value);
        
        const format_global = c.LLVMAddGlobal(self.module, format_type, "var_fmt");
        c.LLVMSetInitializer(format_global, format_value);
        c.LLVMSetLinkage(format_global, c.LLVMPrivateLinkage);
        
        const indices = [_]c.LLVMValueRef{
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
        };
        
        const fmt_ptr = c.LLVMBuildGEP2(
            self.builder,
            format_type,
            format_global,
            @as([*c]c.LLVMValueRef, @ptrCast(@constCast(&indices))),
            2,
            "fmt_ptr"
        );
        
        const printf_func = self.functions.get("printf").?;
        const args = [_]c.LLVMValueRef{ fmt_ptr, loaded_value };
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGlobalGetValueType(printf_func),
            printf_func,
            @as([*c]c.LLVMValueRef, @ptrCast(@constCast(&args))),
            2,
            "printf_call"
        );
    }
    
    fn generateVariableDeclaration(self: *LLVMBackend, line: []const u8, verbose: bool) LLVMBackendError!void {
        var parts = std.mem.tokenizeScalar(u8, line, ' ');
        _ = parts.next(); // skip "sus"
        
        const var_name = parts.next() orelse return;
        const var_type = parts.next() orelse return;
        const equals = parts.next() orelse return;
        
        if (!std.mem.eql(u8, equals, "=")) return;
        
        const value_str = parts.rest();
        
        if (verbose) print("  Declaring variable: {s} {s} = {s}\n", .{ var_name, var_type, value_str });
        
        // Determine LLVM type
        const llvm_type = if (std.mem.eql(u8, var_type, "drip"))
            c.LLVMInt64TypeInContext(self.context)
        else if (std.mem.eql(u8, var_type, "lit"))
            c.LLVMInt1TypeInContext(self.context)
        else if (std.mem.eql(u8, var_type, "meal"))
            c.LLVMDoubleTypeInContext(self.context)
        else
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0); // Default to string
        
        // Allocate stack space
        const var_ptr = c.LLVMBuildAlloca(self.builder, llvm_type, var_name.ptr);
        
        // Parse and store initial value
        if (std.mem.eql(u8, var_type, "drip")) {
            if (std.fmt.parseInt(i64, value_str, 10)) |num| {
                const num_value = c.LLVMConstInt(llvm_type, @intCast(num), 0);
                _ = c.LLVMBuildStore(self.builder, num_value, var_ptr);
            } else |_| {
                // Default to 0
                const zero_value = c.LLVMConstInt(llvm_type, 0, 0);
                _ = c.LLVMBuildStore(self.builder, zero_value, var_ptr);
            }
        } else if (std.mem.eql(u8, var_type, "lit")) {
            const bool_value = if (std.mem.eql(u8, std.mem.trim(u8, value_str, " \t"), "based"))
                c.LLVMConstInt(llvm_type, 1, 0)
            else
                c.LLVMConstInt(llvm_type, 0, 0);
            _ = c.LLVMBuildStore(self.builder, bool_value, var_ptr);
        } else if (std.mem.eql(u8, var_type, "meal")) {
            if (std.fmt.parseFloat(f64, value_str)) |num| {
                const num_value = c.LLVMConstReal(llvm_type, num);
                _ = c.LLVMBuildStore(self.builder, num_value, var_ptr);
            } else |_| {
                const zero_value = c.LLVMConstReal(llvm_type, 0.0);
                _ = c.LLVMBuildStore(self.builder, zero_value, var_ptr);
            }
        }
        
        // Store in symbol table
        try self.variables.put(var_name, var_ptr);
    }
    
    fn ensureMainFunction(self: *LLVMBackend) LLVMBackendError!void {
        // Main function should already exist, just verify it's complete
        if (self.functions.get("main") == null) {
            return LLVMBackendError.LLVMError;
        }
    }
    
    fn verifyModule(self: *LLVMBackend) LLVMBackendError!void {
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module, c.LLVMPrintMessageAction, &error_msg) != 0) {
            std.debug.print("LLVM module verification failed: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return LLVMBackendError.LLVMError;
        }
    }
    
    fn applyOptimizations(self: *LLVMBackend, level: u8, verbose: bool) LLVMBackendError!void {
        if (verbose) print("🔧 Applying O{} optimizations...\n", .{level});
        
        // Create pass manager
        const pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(pass_manager);
        
        // Add optimization passes based on level
        switch (level) {
            1 => {
                c.LLVMAddInstructionCombiningPass(pass_manager);
                c.LLVMAddReassociatePass(pass_manager);
            },
            2 => {
                c.LLVMAddInstructionCombiningPass(pass_manager);
                c.LLVMAddReassociatePass(pass_manager);
                c.LLVMAddGVNPass(pass_manager);
                c.LLVMAddCFGSimplificationPass(pass_manager);
            },
            3 => {
                c.LLVMAddInstructionCombiningPass(pass_manager);
                c.LLVMAddReassociatePass(pass_manager);
                c.LLVMAddGVNPass(pass_manager);
                c.LLVMAddCFGSimplificationPass(pass_manager);
                c.LLVMAddAggressiveDCEPass(pass_manager);
            },
            else => {},
        }
        
        // Run optimizations
        _ = c.LLVMRunPassManager(pass_manager, self.module);
        
        if (verbose) print("✅ Optimizations applied\n", .{});
    }
};
