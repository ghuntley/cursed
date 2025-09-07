const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Import AST and components
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

// Use Zig's built-in LLVM IR builder (cross-platform, no C dependencies)
const llvm = std.zig.llvm;

/// Minimal Cross-platform LLVM IR Generation Pipeline using Zig's native LLVM builder
/// Works on Linux, Windows, macOS without external LLVM library dependencies
pub const LLVMIRPipeline = struct {
    allocator: Allocator,
    
    // Zig LLVM IR Builder (cross-platform)
    builder: llvm.Builder,
    
    // Optimization settings
    optimization_level: u8,
    debug_info: bool,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, module_name: []const u8) !Self {
        // Initialize Zig's LLVM IR builder (no external dependencies)
        var builder = try llvm.Builder.init(.{
            .allocator = allocator,
            .strip = false,
        });
        
        // Set module metadata
        const source_filename = try builder.string(module_name);
        builder.source_filename = source_filename;
        
        return Self{
            .allocator = allocator,
            .builder = builder,
            .optimization_level = 0,
            .debug_info = false,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.builder.deinit();
    }
    
    /// Main compilation entry point - generates minimal LLVM bitcode
    pub fn compileSource(self: *Self, source: []const u8, output_file: []const u8, verbose: bool) !void {
        if (verbose) print("🔥 Starting minimal Zig-native LLVM compilation...\n", .{});
        
        // Step 1: Parse CURSED source
        if (verbose) print("🔍 Parsing CURSED source...\n", .{});
        var lex = lexer.Lexer.init(self.allocator, source);
        var tokens_list = try lex.tokenize();
        defer tokens_list.deinit(self.allocator);
        
        var cursed_parser = parser.Parser.initWithFile(self.allocator, tokens_list.items, "source.💀");
        defer cursed_parser.deinit();
        
        const program = try cursed_parser.parseProgram();
        
        // Step 2: Generate minimal LLVM IR
        if (verbose) print("⚡ Generating minimal LLVM IR...\n", .{});
        try self.generateMinimalIR(&program);
        
        // Step 3: Write LLVM bitcode to file
        if (verbose) print("📝 Writing LLVM bitcode...\n", .{});
        try self.writeBitcode(output_file);
        
        // Step 4: Generate native executable (stub)
        if (verbose) print("🏗️ Generated bitcode for {s}\n", .{output_file});
        
        if (verbose) print("✅ Minimal Zig-native LLVM compilation complete!\n", .{});
    }
    
    /// Generate minimal LLVM IR with a main function
    fn generateMinimalIR(self: *Self, program: *const ast.Program) !void {
        // Create main function type: void main()
        var param_types: [0]llvm.Builder.Type = .{};
        const main_func_type = try self.builder.fnType(llvm.Builder.Type.void, &param_types, .normal);
        
        // Create main function
        const main_name = try self.builder.strtabString("main");
        const main_function = try self.builder.addFunction(main_func_type, main_name, .default);
        
        // Create entry basic block
        const entry_block = try self.builder.appendBasicBlock(main_function, "entry");
        
        // Position builder at entry
        try self.builder.positionAtEnd(entry_block);
        
        if (program.statements.items.len > 0) {
            if (@import("builtin").mode == .Debug) {
                print("🎯 Processing {d} statements\n", .{program.statements.items.len});
            }
        }
        
        // Generate simple return
        try self.builder.ret(null);
    }
    
    /// Write LLVM bitcode to file
    fn writeBitcode(self: *Self, output_file: []const u8) !void {
        const bitcode_file = try std.fmt.allocPrint(self.allocator, "{s}.bc", .{output_file});
        defer self.allocator.free(bitcode_file);
        
        var file = try std.fs.cwd().createFile(bitcode_file, .{});
        defer file.close();
        
        try llvm.bitcode_writer.writeBitcode(file.writer(), &self.builder);
        print("🎉 Generated LLVM bitcode: {s}\n", .{bitcode_file});
    }
};
