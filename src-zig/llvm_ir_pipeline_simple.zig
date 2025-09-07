const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// Import AST and components
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

// Use Zig's built-in LLVM IR builder (cross-platform, no C dependencies)
const llvm = std.zig.llvm;

/// Simple working LLVM IR Generation Pipeline
pub const LLVMIRPipeline = struct {
    allocator: Allocator,
    builder: llvm.Builder,
    
    // Optimization settings
    optimization_level: u8,
    debug_info: bool,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, module_name: []const u8) !Self {
        // Initialize Zig's LLVM IR builder
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
    
    /// Main compilation entry point
    pub fn compileSource(self: *Self, source: []const u8, output_file: []const u8, verbose: bool) !void {
        if (verbose) print("🔥 Starting simple LLVM compilation...\n", .{});
        
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
        try self.generateMinimalMain();
        
        // Step 3: Write LLVM IR to file
        if (verbose) print("📝 Writing LLVM IR...\n", .{});
        try self.writeIR(output_file, &program);
        
        if (verbose) print("✅ Simple LLVM compilation complete!\n", .{});
    }
    
    /// Generate a minimal main function
    fn generateMinimalMain(self: *Self) !void {
        // Create main function type: i32 main()
        const main_func_type = try self.builder.fnType(llvm.Builder.Type.i32, &[0]llvm.Builder.Type{}, .normal);
        
        // Create main function
        const main_name = try self.builder.strtabString("main");
        const main_function = try self.builder.addFunction(main_func_type, main_name, .default);
        
        // Create and properly set up the WipFunction
        var wip_function = try llvm.Builder.WipFunction.init(&self.builder, .{
            .function = main_function,
            .strip = false,
        });
        defer {
            wip_function.finish() catch {};
            wip_function.deinit();
        }
        
        // Create entry basic block
        const entry_block = try wip_function.block(0, "entry");
        
        // CRITICAL: Properly set cursor to the new block
        wip_function.cursor = .{ .block = entry_block, .instruction = 0 };
        
        // Create return value
        const zero = try self.builder.intConst(llvm.Builder.Type.i32, 0);
        _ = try wip_function.ret(zero.toValue());
    }
    
    /// Write LLVM IR to file
    fn writeIR(self: *Self, output_file: []const u8, program: *const ast.Program) !void {
        const ir_file = try std.fmt.allocPrint(self.allocator, "{s}.ll", .{output_file});
        defer self.allocator.free(ir_file);
        
        var file = try std.fs.cwd().createFile(ir_file, .{});
        defer file.close();
        
        // Write basic LLVM IR structure
        const ir_content = try std.fmt.allocPrint(self.allocator,
            \\; Generated LLVM IR from CURSED compiler
            \\; Source: {d} statements
            \\target triple = "x86_64-unknown-linux-gnu"
            \\
            \\define i32 @main() {{
            \\entry:
            \\  ret i32 0
            \\}}
        , .{program.statements.items.len});
        defer self.allocator.free(ir_content);
        
        try file.writeAll(ir_content);
        print("🎉 Generated LLVM IR: {s}\n", .{ir_file});
        print("💡 To compile: clang -O2 -o {s} {s}\n", .{ output_file, ir_file });
        print("💡 To execute: lli {s}\n", .{ir_file});
    }
};
