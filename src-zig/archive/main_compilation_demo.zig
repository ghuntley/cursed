const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");

// LLVM C bindings
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
});

const CompilerError = error{
    LexerError,
    ParserError,
    CodeGenError,
    LinkerError,
    OutOfMemory,
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Compilation Pipeline Demo v1.0.0\n", .{});
        return;
    }

    const filename = args[1];
    var compile_mode = false;
    
    // Check for --compile flag
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        }
    }

    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        print("Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    print("📄 Read source file: {s} ({} bytes)\n", .{ filename, source.len });

    if (compile_mode) {
        // LLVM Compilation Pipeline Demo
        print("🔧 Starting LLVM compilation pipeline...\n", .{});
        
        try demonstrateCompilationPipeline(allocator, source, filename);
        
        print("✅ Compilation pipeline demonstration complete!\n", .{});
    } else {
        // Simple interpretation
        print("🚀 Running in interpretation mode...\n", .{});
        print("Source: {s}\n", .{source});
        print("Note: Use --compile flag to test compilation pipeline\n", .{});
    }
}

fn demonstrateCompilationPipeline(allocator: Allocator, source: []const u8, filename: []const u8) !void {
    // Step 1: Tokenization
    print("1️⃣ Tokenization...\n", .{});
    var l = lexer.Lexer.init(allocator, source);
    const tokens = try l.tokenize();
    defer tokens.deinit();
    print("   Generated {} tokens\n", .{tokens.items.len});

    // Step 2: Parsing
    print("2️⃣ Parsing...\n", .{});
    var p = parser.Parser.init(allocator, tokens.items);
    defer p.deinit();
    const program = try p.parseProgram();
    print("   Parsed {} statements\n", .{program.statements.items.len});

    // Step 3: LLVM IR Generation
    print("3️⃣ LLVM IR Generation...\n", .{});
    var codegen = SimpleCodeGen.init(allocator);
    defer codegen.deinit();
    
    try codegen.generateProgram(program);
    print("   Generated LLVM module\n", .{});

    // Step 4: Write LLVM IR to file
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);
    
    var ir_filename = ArrayList(u8).init(allocator);
    defer ir_filename.deinit();
    try ir_filename.appendSlice(output_name);
    try ir_filename.appendSlice(".ll");
    
    try codegen.writeIR(ir_filename.items);
    print("   Wrote LLVM IR to: {s}\n", .{ir_filename.items});

    // Step 5: Compile to Object File
    print("4️⃣ Compiling to object file...\n", .{});
    try codegen.compileToObject(output_name);
    
    // Step 6: Link to Executable
    print("5️⃣ Linking to executable...\n", .{});
    try linkToExecutable(allocator, output_name);
    
    print("✅ Generated executable: {s}\n", .{output_name});
    
    // Step 7: Test execution
    print("6️⃣ Testing executable...\n", .{});
    try testExecutable(allocator, output_name);
}

/// Simple LLVM Code Generator for demonstration
const SimpleCodeGen = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,

    pub fn init(allocator: Allocator) SimpleCodeGen {
        const context = c.LLVMContextCreate();
        const module = c.LLVMModuleCreateWithNameInContext("cursed_module", context);
        const builder = c.LLVMCreateBuilderInContext(context);
        
        return SimpleCodeGen{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
        };
    }

    pub fn deinit(self: *SimpleCodeGen) void {
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
    }

    pub fn generateProgram(self: *SimpleCodeGen, program: ast.Program) !void {
        _ = program; // Unused for demo
        // Set up runtime functions
        self.setupRuntimeFunctions();
        
        // Create main function
        const i32_type = c.LLVMInt32TypeInContext(self.context);
        const main_type = c.LLVMFunctionType(i32_type, null, 0, 0);
        const main_function = c.LLVMAddFunction(self.module, "main", main_type);
        
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Generate a simple "Hello, World!" call for demonstration
        self.generateHelloWorld();
        
        // Return 0 from main
        const return_value = c.LLVMConstInt(i32_type, 0, 0);
        _ = c.LLVMBuildRet(self.builder, return_value);
    }

    fn setupRuntimeFunctions(self: *SimpleCodeGen) void {
        // puts function declaration
        const i32_type = c.LLVMInt32TypeInContext(self.context);
        const i8_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        
        var puts_params = [_]c.LLVMTypeRef{i8_ptr_type};
        const puts_type = c.LLVMFunctionType(i32_type, &puts_params, 1, 0);
        _ = c.LLVMAddFunction(self.module, "puts", puts_type);
    }

    fn generateHelloWorld(self: *SimpleCodeGen) void {
        // Create "Hello from CURSED!" string
        const hello_str = c.LLVMBuildGlobalStringPtr(self.builder, "Hello from CURSED compilation pipeline!", "hello_str");
        
        // Call puts function
        const puts_func = c.LLVMGetNamedFunction(self.module, "puts");
        if (puts_func != null) {
            var args = [_]c.LLVMValueRef{hello_str};
            const puts_type = c.LLVMFunctionType(c.LLVMInt32TypeInContext(self.context), 
                             &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}, 1, 0);
            _ = c.LLVMBuildCall2(self.builder, puts_type, puts_func, &args, 1, "puts_call");
        }
    }

    pub fn writeIR(self: *SimpleCodeGen, filename: []const u8) !void {
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMPrintModuleToFile(self.module, filename.ptr, &error_msg) != 0) {
            print("Failed to write LLVM IR: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CompilerError.CodeGenError;
        }
    }

    pub fn compileToObject(self: *SimpleCodeGen, output_base: []const u8) !void {
        // Initialize LLVM targets
        c.LLVMInitializeAllTargetInfos();
        c.LLVMInitializeAllTargets();
        c.LLVMInitializeAllTargetMCs();
        c.LLVMInitializeAllAsmParsers();
        c.LLVMInitializeAllAsmPrinters();

        // Get target triple
        const target_triple = c.LLVMGetDefaultTargetTriple();
        defer c.LLVMDisposeMessage(target_triple);

        // Get target
        var llvm_target: c.LLVMTargetRef = undefined;
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMGetTargetFromTriple(target_triple, &llvm_target, &error_msg) != 0) {
            print("Failed to get target: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CompilerError.CodeGenError;
        }

        // Create target machine
        const target_machine = c.LLVMCreateTargetMachine(
            llvm_target,
            target_triple,
            "generic",
            "",
            c.LLVMCodeGenLevelDefault,
            c.LLVMRelocDefault,
            c.LLVMCodeModelDefault
        );
        defer c.LLVMDisposeTargetMachine(target_machine);

        // Generate object file
        var obj_filename = ArrayList(u8).init(self.allocator);
        defer obj_filename.deinit();
        try obj_filename.appendSlice(output_base);
        try obj_filename.appendSlice(".o");

        if (c.LLVMTargetMachineEmitToFile(target_machine, self.module, obj_filename.items.ptr, c.LLVMObjectFile, &error_msg) != 0) {
            print("Failed to emit object file: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CompilerError.CodeGenError;
        }

        print("   Generated object file: {s}\n", .{obj_filename.items});
    }
};

fn linkToExecutable(allocator: Allocator, output_base: []const u8) !void {
    var obj_filename = ArrayList(u8).init(allocator);
    defer obj_filename.deinit();
    try obj_filename.appendSlice(output_base);
    try obj_filename.appendSlice(".o");

    // Link using gcc
    var link_args = ArrayList([]const u8).init(allocator);
    defer link_args.deinit();
    
    try link_args.append("gcc");
    try link_args.append("-o");
    try link_args.append(output_base);
    try link_args.append(obj_filename.items);
    try link_args.append("-no-pie");

    var child = std.ChildProcess.init(link_args.items, allocator);
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;
    
    const result = child.spawnAndWait() catch |err| {
        print("Failed to spawn linker: {}\n", .{err});
        return CompilerError.LinkerError;
    };
    
    switch (result) {
        .Exited => |code| {
            if (code != 0) {
                print("Linker failed with exit code: {}\n", .{code});
                return CompilerError.LinkerError;
            }
        },
        else => {
            print("Linker process terminated abnormally\n", .{});
            return CompilerError.LinkerError;
        }
    }
    
    print("   Linked executable: {s}\n", .{output_base});
}

fn testExecutable(allocator: Allocator, executable_path: []const u8) !void {
    var child = std.ChildProcess.init(&[_][]const u8{executable_path}, allocator);
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;
    
    const result = child.spawnAndWait() catch |err| {
        print("Failed to execute: {}\n", .{err});
        return;
    };
    
    switch (result) {
        .Exited => |code| {
            if (code == 0) {
                print("   ✅ Executable ran successfully!\n", .{});
            } else {
                print("   ❌ Executable failed with exit code: {}\n", .{code});
            }
        },
        else => {
            print("   ❌ Executable terminated abnormally\n", .{});
        }
    }
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}

fn printUsage() void {
    print("CURSED Compilation Pipeline Demo\n", .{});
    print("Usage: cursed-zig <file.csd> [--compile]\n", .{});
    print("       cursed-zig --version\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile          Demonstrate full compilation pipeline\n", .{});
    print("\nThis demo shows the complete pipeline:\n", .{});
    print("  1. Tokenization\n", .{});
    print("  2. Parsing\n", .{});
    print("  3. LLVM IR Generation\n", .{});
    print("  4. Object File Generation\n", .{});
    print("  5. Linking\n", .{});
    print("  6. Executable Testing\n", .{});
}
