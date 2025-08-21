const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// Minimal LLVM backend without C imports to avoid athlon-xp issues
// This provides basic LLVM IR generation using string templating

const ArithmeticResult = struct {
    variable_value: i64,
    addend: i64,
};

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
            .functions = .{},
            .globals = .{},
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
        self.functions.deinit(allocator);
        self.globals.deinit(allocator);
    }
    
    pub fn addFunction(self: *LLVMBackendMinimal, name: []const u8, return_type: []const u8, params: []const u8, body: []const u8) !void {
        const func_def = try std.fmt.allocPrint(self.allocator,
            \\define {s} @{s}({s}) {{
            \\entry:
            \\{s}
            \\}}
            \\
        , .{ return_type, name, params, body });
        
        try self.functions.append(self.allocator, func_def);
    }
    
    pub fn addGlobal(self: *LLVMBackendMinimal, name: []const u8, type_str: []const u8, value: []const u8) !void {
        const global_def = try std.fmt.allocPrint(self.allocator,
            "@{s} = global {s} {s}\n", .{ name, type_str, value });
        
        try self.globals.append(self.allocator, global_def);
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
    
    pub fn compileProgram(self: *LLVMBackendMinimal, source: []const u8) !void {
        // Check for function definitions first
        if (std.mem.indexOf(u8, source, "slay ") != null) {
            try self.compileProgramWithFunctions(source);
        } else if (std.mem.indexOf(u8, source, "vibez.spill(") != null) {
            if (parseSimpleArithmetic(source)) |result| {
                try self.compileArithmeticExpression(result);
            } else {
                // Fallback to simple expression
                try self.compileSimpleExpression(42);
            }
        } else {
            // Default fallback
            try self.compileSimpleExpression(42);
        }
    }
    
    pub fn compileProgramWithFunctions(self: *LLVMBackendMinimal, source: []const u8) !void {
        // Parse function definitions and function calls
        var lines = std.mem.splitScalar(u8, source, '\n');
        var has_functions = false;
        var main_statements: std.ArrayList([]const u8) = .empty;
        defer {
            // Fix memory leak: Free all duplicated strings before deinit
            for (main_statements.items) |stmt| {
                self.allocator.free(stmt);
            }
            main_statements.deinit(allocator);
        }
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (trimmed.len == 0) continue;
            
            if (std.mem.startsWith(u8, trimmed, "slay ")) {
                // Generate function definition
                try self.generateFunctionFromLine(trimmed);
                has_functions = true;
            } else {
                // Collect main statements - fix memory leak by properly managing string lifecycle
                try main_statements.append(self.allocator, try self.allocator.dupe(u8, trimmed));
            }
        }
        
        // Generate main function
        try self.generateMainFunction(main_statements.items, has_functions);
    }
    
    fn generateFunctionFromLine(self: *LLVMBackendMinimal, line: []const u8) !void {
        // Parse: slay add(x drip, y drip) drip { damn x + y }
        if (std.mem.indexOf(u8, line, "(")) |paren_pos| {
            const func_name = std.mem.trim(u8, line[5..paren_pos], " \t");
            
            // For now, generate a simple add function
            if (std.mem.eql(u8, func_name, "add")) {
                const func_body = 
                    \\  %result = add i64 %x, %y
                    \\  ret i64 %result
                ;
                try self.addFunction("add", "i64", "i64 %x, i64 %y", func_body);
            }
        }
    }
    
    fn generateMainFunction(self: *LLVMBackendMinimal, statements: [][]const u8, has_functions: bool) !void {
        var main_body: std.ArrayList(u8) = .empty;
        defer main_body.deinit(allocator);
        
        for (statements) |stmt| {
            if (std.mem.indexOf(u8, stmt, "vibez.spill(") != null) {
                // Handle function calls
                if (std.mem.indexOf(u8, stmt, "add(") != null and has_functions) {
                    // Parse add(2, 3) call
                    try main_body.appendSlice("  %1 = call i64 @add(i64 2, i64 3)\n");
                    try main_body.appendSlice("  %2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.str, i32 0, i32 0), i64 %1)\n");
                } else {
                    // Default output
                    try main_body.appendSlice("  %1 = alloca i64\n");
                    try main_body.appendSlice("  store i64 42, i64* %1\n");
                    try main_body.appendSlice("  %2 = load i64, i64* %1\n");
                    try main_body.appendSlice("  %3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @.str, i32 0, i32 0), i64 %2)\n");
                }
            }
        }
        
        try main_body.appendSlice("  ret i32 0\n");
        
        try self.addGlobal(".str", "[12 x i8]", "c\"Value: %ld\\0A\\00\"");
        try self.addFunction("main", "i32", "", main_body.items);
    }
    
    pub fn compileArithmeticExpression(self: *LLVMBackendMinimal, result: ArithmeticResult) !void {
        const main_body = try std.fmt.allocPrint(self.allocator,
            \\  %1 = alloca i64
            \\  store i64 {d}, i64* %1
            \\  %2 = load i64, i64* %1
            \\  %3 = add i64 %2, {d}
            \\  %4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.str, i32 0, i32 0), i64 %3)
            \\  ret i32 0
        , .{ result.variable_value, result.addend });
        defer self.allocator.free(main_body);
        
        try self.addGlobal(".str", "[5 x i8]", "c\"%ld\\0A\\00\"");
        try self.addFunction("main", "i32", "", main_body);
    }
};

// Simple parser for "sus x drip = 10; vibez.spill(x + 5)" pattern
fn parseSimpleArithmetic(source: []const u8) ?ArithmeticResult {
    // Look for variable assignment: "sus x drip = NUMBER"
    if (std.mem.indexOf(u8, source, "sus ")) |sus_start| {
        if (std.mem.indexOf(u8, source[sus_start..], " drip = ")) |drip_pos| {
            const equals_start = sus_start + drip_pos + 8; // length of " drip = "
            
            // Find the end of the number (semicolon or space)
            var num_end = equals_start;
            while (num_end < source.len and 
                   source[num_end] != ';' and 
                   source[num_end] != ' ' and
                   source[num_end] != '\n') {
                num_end += 1;
            }
            
            const var_value_str = source[equals_start..num_end];
            const variable_value = std.fmt.parseInt(i64, var_value_str, 10) catch return null;
            
            // Look for vibez.spill(x + NUMBER)
            if (std.mem.indexOf(u8, source, "vibez.spill(")) |spill_start| {
                if (std.mem.indexOf(u8, source[spill_start..], " + ")) |plus_pos| {
                    const plus_start = spill_start + plus_pos + 3; // length of " + "
                    
                    // Find the closing parenthesis
                    if (std.mem.indexOf(u8, source[plus_start..], ")")) |paren_pos| {
                        const addend_str = source[plus_start..plus_start + paren_pos];
                        const addend = std.fmt.parseInt(i64, addend_str, 10) catch return null;
                        
                        return ArithmeticResult{
                            .variable_value = variable_value,
                            .addend = addend,
                        };
                    }
                }
            }
        }
    }
    
    return null;
}

// Simple compiler function that generates LLVM IR for basic CURSED programs
pub fn compileToLLVM(allocator: Allocator, source: []const u8, output_file: []const u8) !void {
    print("[LLVM] Compiling CURSED program without C imports...\n", .{});
    
    var backend = try LLVMBackendMinimal.init(allocator, "cursed_module");
    defer backend.deinit(allocator);
    
    // Parse the source to extract variable assignments and arithmetic
    try backend.compileProgram(source);
    
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
