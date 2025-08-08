const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// Minimal LLVM backend without C imports to avoid athlon-xp issues
// This provides basic LLVM IR generation using string templating

pub const LLVMBackendMinimal = struct {
    allocator: Allocator,
    module_name: []const u8,
    functions: std.ArrayList([]const u8),
    globals: std.ArrayList([]const u8),
    target_triple: []const u8,
    
    pub fn init(allocator: Allocator, module_name: []const u8) !LLVMBackendMinimal {
        return LLVMBackendMinimal{
            .allocator = allocator,
            .module_name = module_name,
            .functions = std.ArrayList([]const u8).init(allocator),
            .globals = std.ArrayList([]const u8).init(allocator),
            .target_triple = "x86_64-unknown-linux-gnu",
        };
    }
    
    pub fn deinit(self: *LLVMBackendMinimal) void {
        for (self.functions.items) |func| {
            self.allocator.free(func);
        }
        for (self.globals.items) |global| {
            self.allocator.free(global);
        }
        self.functions.deinit();
        self.globals.deinit();
    }
    
    pub fn addFunction(self: *LLVMBackendMinimal, name: []const u8, return_type: []const u8, params: []const u8, body: []const u8) !void {
        const func_def = try std.fmt.allocPrint(self.allocator,
            \\define {s} @{s}({s}) {{
            \\entry:
            \\{s}
            \\}}
            \\
        , .{ return_type, name, params, body });
        
        try self.functions.append(func_def);
    }
    
    pub fn addGlobal(self: *LLVMBackendMinimal, name: []const u8, type_str: []const u8, value: []const u8) !void {
        const global_def = try std.fmt.allocPrint(self.allocator,
            "@{s} = global {s} {s}\n", .{ name, type_str, value });
        
        try self.globals.append(global_def);
    }
    
    pub fn generateIR(self: *LLVMBackendMinimal, writer: anytype) !void {
        // Write module header
        try writer.print("target triple = \"{s}\"\n\n", .{self.target_triple});
        
        // Write global declarations
        for (self.globals.items) |global| {
            try writer.writeAll(global);
        }
        
        // Write function declarations for printf
        try writer.writeAll("declare i32 @printf(i8*, ...)\n\n");
        
        // Write functions
        for (self.functions.items) |func| {
            try writer.writeAll(func);
        }
    }
    
    pub fn compileSimpleExpression(self: *LLVMBackendMinimal, expr_value: i64) !void {
        const main_body = try std.fmt.allocPrint(self.allocator,
            \\  %1 = alloca i64
            \\  store i64 {d}, i64* %1
            \\  %2 = load i64, i64* %1
            \\  %3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.str, i32 0, i32 0), i64 %2)
            \\  ret i32 0
        , .{expr_value});
        defer self.allocator.free(main_body);
        
        try self.addGlobal(".str", "[12 x i8]", "c\"Value: %ld\\0A\\00\"");
        try self.addFunction("main", "i32", "", main_body);
    }
};

// Simple compiler function that generates LLVM IR for basic CURSED programs
pub fn compileToLLVM(allocator: Allocator, _: []const u8, output_file: []const u8) !void {
    print("[LLVM] Compiling CURSED program without C imports...\n", .{});
    
    var backend = try LLVMBackendMinimal.init(allocator, "cursed_module");
    defer backend.deinit();
    
    // For now, generate a simple main function that prints a value
    try backend.compileSimpleExpression(42);
    
    // Write IR to file
    const file = try std.fs.cwd().createFile(output_file, .{});
    defer file.close();
    
    const writer = file.writer();
    try backend.generateIR(writer);
    
    print("[LLVM] Generated IR: {s}\n", .{output_file});
}

// Compile LLVM IR to native executable using llc and clang
pub fn compileIRToNative(allocator: Allocator, ir_file: []const u8, output_file: []const u8) !void {
    print("[LLVM] Compiling IR to native executable...\n", .{});
    
    // Generate object file using llc
    const obj_file = try std.fmt.allocPrint(allocator, "{s}.o", .{output_file});
    defer allocator.free(obj_file);
    
    const llc_args = [_][]const u8{ "llc-18", "-filetype=obj", ir_file, "-o", obj_file };
    
    var llc_process = std.process.Child.init(&llc_args, allocator);
    llc_process.stdout_behavior = .Pipe;
    llc_process.stderr_behavior = .Pipe;
    
    const llc_result = llc_process.spawnAndWait() catch |err| {
        print("❌ Failed to run llc: {}\n", .{err});
        return;
    };
    
    if (llc_result != .Exited or llc_result.Exited != 0) {
        print("❌ llc compilation failed\n", .{});
        return;
    }
    
    // Link with gcc
    const gcc_args = [_][]const u8{ "gcc", obj_file, "-o", output_file };
    
    var gcc_process = std.process.Child.init(&gcc_args, allocator);
    gcc_process.stdout_behavior = .Pipe;
    gcc_process.stderr_behavior = .Pipe;
    
    const gcc_result = gcc_process.spawnAndWait() catch |err| {
        print("❌ Failed to run gcc: {}\n", .{err});
        return;
    };
    
    if (gcc_result != .Exited or gcc_result.Exited != 0) {
        print("❌ gcc linking failed\n", .{});
        return;
    }
    
    print("✅ Native executable created: {s}\n", .{output_file});
    
    // Clean up object file
    std.fs.cwd().deleteFile(obj_file) catch {};
}
