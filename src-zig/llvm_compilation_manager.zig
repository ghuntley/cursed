const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const SimpleLLVMIRGenerator = @import("simple_llvm_ir_generator.zig").SimpleLLVMIRGenerator;
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

/// LLVM Compilation Manager
/// Manages the complete compilation pipeline from source to executable
pub const LLVMCompilationManager = struct {
    allocator: Allocator,
    verbose: bool,
    optimization_level: u32,
    debug_info: bool,
    
    pub fn init(allocator: Allocator) LLVMCompilationManager {
        return LLVMCompilationManager{
            .allocator = allocator,
            .verbose = false,
            .optimization_level = 2,
            .debug_info = false,
        };
    }
    
    pub fn setVerbose(self: *LLVMCompilationManager, verbose: bool) void {
        self.verbose = verbose;
    }
    
    pub fn setOptimizationLevel(self: *LLVMCompilationManager, level: u32) void {
        self.optimization_level = level;
    }
    
    pub fn setDebugInfo(self: *LLVMCompilationManager, debug: bool) void {
        self.debug_info = debug;
    }
    
    /// Compile CURSED source file to executable
    pub fn compileFile(self: *LLVMCompilationManager, input_file: []const u8, output_file: []const u8) !void {
        print("📁 Compiling file: {s} -> {s}\n", .{ input_file, output_file });
        
        // Read source file
        const source = try std.fs.cwd().readFileAlloc(self.allocator, input_file, 1024 * 1024); // 1MB max
        defer self.allocator.free(source);
        
        try self.compileSource(source, output_file);
    }
    
    /// Compile CURSED source code to executable
    pub fn compileSource(self: *LLVMCompilationManager, source: []const u8, output_file: []const u8) !void {
        print("🚀 Starting LLVM compilation...\n", .{});
        if (self.verbose) {
            print("Source code ({d} bytes):\n{s}\n", .{ source.len, source });
        }
        
        // Create LLVM IR generator
        var generator = SimpleLLVMIRGenerator.init(self.allocator);
        defer generator.deinit();
        
        generator.setVerbose(self.verbose);
        
        // Generate LLVM IR from source
        try generator.generateFromSource(source);
        
        // Write IR to temporary file
        const ir_file = try std.fmt.allocPrint(self.allocator, "{s}.ll", .{output_file});
        defer self.allocator.free(ir_file);
        
        try generator.writeToFile(ir_file);
        
        // Compile IR to executable
        try generator.compileToExecutable(ir_file, output_file);
        
        // Clean up IR file unless verbose
        if (!self.verbose) {
            std.fs.cwd().deleteFile(ir_file) catch {};
        }
        
        print("✅ Compilation completed successfully!\n", .{});
    }
    
    /// Generate LLVM IR only (for debugging)
    pub fn generateIROnly(self: *LLVMCompilationManager, source: []const u8, ir_file: []const u8) !void {
        print("⚡ Generating LLVM IR only...\n", .{});
        
        // Create LLVM IR generator
        var generator = SimpleLLVMIRGenerator.init(self.allocator);
        defer generator.deinit();
        
        generator.setVerbose(self.verbose);
        
        // Generate LLVM IR from source
        try generator.generateFromSource(source);
        
        // Write IR to file
        try generator.writeToFile(ir_file);
        
        print("✅ LLVM IR written to: {s}\n", .{ir_file});
    }
    
    /// Check if LLVM backend is available
    pub fn checkLLVMAvailability(self: *LLVMCompilationManager) bool {
        
        // Check if clang is available as a fallback
        const result = std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{"clang", "--version"},
        }) catch {
            print("❌ Neither LLVM backend nor clang available\n", .{});
            return false;
        };
        
        defer self.allocator.free(result.stdout);
        defer self.allocator.free(result.stderr);
        
        if (result.term == .Exited and result.term.Exited == 0) {
            print("✅ LLVM backend available via clang\n", .{});
            return true;
        } else {
            print("❌ LLVM backend not available\n", .{});
            return false;
        }
    }
    
    /// Run LLVM compilation tests
    pub fn runTests(self: *LLVMCompilationManager) !void {
        print("🧪 Running LLVM compilation tests...\n");
        
        // Test 1: Basic arithmetic
        const test1_source = 
            \\slay main_character() {
            \\    sus x drip = 42;
            \\    sus y drip = 13;
            \\    sus result drip = x + y;
            \\    vibez.spill(result);
            \\}
        ;
        
        print("Test 1: Basic arithmetic\n");
        try self.compileSource(test1_source, "test1_arithmetic");
        
        // Test 2: String output
        const test2_source = 
            \\slay main_character() {
            \\    vibez.spill("Hello from CURSED LLVM!");
            \\}
        ;
        
        print("Test 2: String output\n");
        try self.compileSource(test2_source, "test2_string");
        
        // Test 3: Function calls
        const test3_source = 
            \\slay add(a drip, b drip) drip {
            \\    damn a + b;
            \\}
            \\
            \\slay main_character() {
            \\    sus result drip = add(10, 20);
            \\    vibez.spill(result);
            \\}
        ;
        
        print("Test 3: Function calls\n");
        try self.compileSource(test3_source, "test3_functions");
        
        print("✅ All LLVM compilation tests passed!\n");
    }
};

// Test function for the compilation manager
pub fn testLLVMCompilation() !void {
    print("🔥 Testing LLVM IR Pipeline Integration...\n");
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var manager = LLVMCompilationManager.init(allocator);
    manager.setVerbose(true);
    manager.setOptimizationLevel(2);
    
    // Check LLVM availability
    if (!manager.checkLLVMAvailability()) {
        print("❌ LLVM not available, skipping tests\n");
        return;
    }
    
    // Run compilation tests
    try manager.runTests();
    
    print("🎉 LLVM IR Pipeline integration test completed!\n");
}
