const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// Fixed LLVM backend using C wrapper to avoid athlon-xp CPU detection issue
// This replaces the minimal backend to use full LLVM functionality

// Use external C wrapper functions to avoid @cImport athlon-xp issue
extern fn llvm_initialize_core() void;
extern fn llvm_create_context() ?*anyopaque;
extern fn llvm_dispose_context(context: ?*anyopaque) void;
extern fn llvm_create_module(context: ?*anyopaque, name: [*c]const u8) ?*anyopaque;
extern fn llvm_dispose_module(module: ?*anyopaque) void;
extern fn llvm_create_builder(context: ?*anyopaque) ?*anyopaque;
extern fn llvm_dispose_builder(builder: ?*anyopaque) void;
extern fn llvm_int32_type(context: ?*anyopaque) ?*anyopaque;
extern fn llvm_int8_type(context: ?*anyopaque) ?*anyopaque;
extern fn llvm_pointer_type(element_type: ?*anyopaque) ?*anyopaque;
extern fn llvm_function_type(return_type: ?*anyopaque, param_types: [*]?*anyopaque, param_count: c_int, is_var_arg: c_int) ?*anyopaque;
extern fn llvm_add_function(module: ?*anyopaque, name: [*c]const u8, function_type: ?*anyopaque) ?*anyopaque;
extern fn llvm_append_basic_block(context: ?*anyopaque, function: ?*anyopaque, name: [*c]const u8) ?*anyopaque;
extern fn llvm_position_builder_at_end(builder: ?*anyopaque, block: ?*anyopaque) void;
extern fn llvm_build_global_string_ptr(builder: ?*anyopaque, str: [*c]const u8, name: [*c]const u8) ?*anyopaque;
extern fn llvm_const_int(int_type: ?*anyopaque, value: c_ulonglong) ?*anyopaque;
extern fn llvm_build_ret(builder: ?*anyopaque, value: ?*anyopaque) ?*anyopaque;
extern fn llvm_get_named_function(module: ?*anyopaque, name: [*c]const u8) ?*anyopaque;
extern fn llvm_get_function_type(function: ?*anyopaque) ?*anyopaque;
extern fn llvm_build_call2(builder: ?*anyopaque, function_type: ?*anyopaque, function: ?*anyopaque, args: [*]?*anyopaque, arg_count: c_int, name: [*c]const u8) ?*anyopaque;
extern fn llvm_verify_module(module: ?*anyopaque) c_int;
extern fn llvm_print_module_to_string(module: ?*anyopaque) [*c]u8;
extern fn llvm_dispose_message(message: [*c]u8) void;
extern fn llvm_write_bitcode_to_file(module: ?*anyopaque, path: [*c]const u8) c_int;
extern fn llvm_build_add(builder: ?*anyopaque, lhs: ?*anyopaque, rhs: ?*anyopaque, name: [*c]const u8) ?*anyopaque;
extern fn llvm_build_mul(builder: ?*anyopaque, lhs: ?*anyopaque, rhs: ?*anyopaque, name: [*c]const u8) ?*anyopaque;
extern fn llvm_build_sub(builder: ?*anyopaque, lhs: ?*anyopaque, rhs: ?*anyopaque, name: [*c]const u8) ?*anyopaque;

// Type aliases for compatibility
const LLVMContextRef = ?*anyopaque;
const LLVMModuleRef = ?*anyopaque;  
const LLVMBuilderRef = ?*anyopaque;
const LLVMValueRef = ?*anyopaque;
const LLVMTypeRef = ?*anyopaque;
const LLVMBasicBlockRef = ?*anyopaque;
const LLVMTargetMachineRef = ?*anyopaque;

pub const LLVMBackendFixed = struct {
    allocator: Allocator,
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    target_machine: LLVMTargetMachineRef,
    
    pub fn init(allocator: Allocator, module_name: []const u8) !LLVMBackendFixed {
        // Initialize LLVM with proper target detection
        llvm_initialize_core();
        
        const context = llvm_create_context();
        if (context == null) return error.LLVMContextCreationFailed;
        
        const module_name_cstr = try allocator.dupeZ(u8, module_name);
        defer allocator.free(module_name_cstr);
        
        const module = llvm_create_module(context, module_name_cstr.ptr);
        if (module == null) return error.LLVMModuleCreationFailed;
        
        const builder = llvm_create_builder(context);
        if (builder == null) return error.LLVMBuilderCreationFailed;
        
        // Skip target machine setup for now (causes athlon-xp issues)
        // Use null target machine - LLVM will use defaults
        const target_machine: LLVMTargetMachineRef = null;
        
        return LLVMBackendFixed{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .target_machine = target_machine,
        };
    }
    
    pub fn deinit(self: *LLVMBackendFixed) void {
        // Target machine not used in simplified version
        if (self.builder) |b| llvm_dispose_builder(b);
        if (self.module) |m| llvm_dispose_module(m);
        if (self.context) |ctx| llvm_dispose_context(ctx);
    }
    
    pub fn createFunction(self: *LLVMBackendFixed, name: []const u8, return_type: LLVMTypeRef, param_types: []LLVMTypeRef, is_var_args: bool) !LLVMValueRef {
        const name_cstr = try self.allocator.dupeZ(u8, name);
        defer self.allocator.free(name_cstr);
        
        const func_type = llvm_function_type(
            return_type,
            param_types.ptr,
            @intCast(param_types.len),
            if (is_var_args) 1 else 0
        );
        
        const func = llvm_add_function(self.module, name_cstr.ptr, func_type);
        return func;
    }
    
    pub fn createBasicBlock(self: *LLVMBackendFixed, function: LLVMValueRef, name: []const u8) !LLVMBasicBlockRef {
        const name_cstr = try self.allocator.dupeZ(u8, name);
        defer self.allocator.free(name_cstr);
        
        return llvm_append_basic_block(self.context, function, name_cstr.ptr);
    }
    
    pub fn positionBuilder(self: *LLVMBackendFixed, block: LLVMBasicBlockRef) void {
        llvm_position_builder_at_end(self.builder, block);
    }
    
    pub fn buildConstInt(self: *LLVMBackendFixed, int_type: LLVMTypeRef, value: u64) LLVMValueRef {
        _ = self; // Mark parameter as used
        return llvm_const_int(int_type, value);
    }
    
    pub fn buildConstString(self: *LLVMBackendFixed, str: []const u8, name: []const u8) !LLVMValueRef {
        const str_cstr = try self.allocator.dupeZ(u8, str);
        defer self.allocator.free(str_cstr);
        
        const name_cstr = try self.allocator.dupeZ(u8, name);
        defer self.allocator.free(name_cstr);
        
        return llvm_build_global_string_ptr(self.builder, str_cstr.ptr, name_cstr.ptr);
    }
    
    pub fn buildCall(self: *LLVMBackendFixed, function: LLVMValueRef, args: []LLVMValueRef, name: []const u8) !LLVMValueRef {
        // Validate input parameters
        if (self.builder == null) return error.InvalidBuilder;
        if (function == null) return error.InvalidFunction;
        
        const name_cstr = try self.allocator.dupeZ(u8, name);
        defer self.allocator.free(name_cstr);
        
        // Get the function type from the function value - this is critical for LLVM
        const function_type = llvm_get_function_type(function);
        if (function_type == null) return error.InvalidFunctionType;
        
        // Validate args - no need to check ptr directly since it's part of the slice
        
        const call_result = llvm_build_call2(self.builder, function_type, function, args.ptr, @intCast(args.len), name_cstr.ptr);
        if (call_result == null) return error.CallBuildFailed;
        
        return call_result;
    }
    
    pub fn buildRet(self: *LLVMBackendFixed, value: ?LLVMValueRef) LLVMValueRef {
        if (value) |v| {
            return llvm_build_ret(self.builder, v);
        } else {
            return llvm_build_ret(self.builder, null);
        }
    }
    
    pub fn buildAdd(self: *LLVMBackendFixed, lhs: LLVMValueRef, rhs: LLVMValueRef, name: []const u8) !LLVMValueRef {
        const name_cstr = try std.fmt.allocPrintZ(self.allocator, "{s}", .{name});
        defer self.allocator.free(name_cstr);
        return llvm_build_add(self.builder, lhs, rhs, name_cstr.ptr) orelse error.LLVMError;
    }
    
    pub fn buildMul(self: *LLVMBackendFixed, lhs: LLVMValueRef, rhs: LLVMValueRef, name: []const u8) !LLVMValueRef {
        const name_cstr = try std.fmt.allocPrintZ(self.allocator, "{s}", .{name});
        defer self.allocator.free(name_cstr);
        return llvm_build_mul(self.builder, lhs, rhs, name_cstr.ptr) orelse error.LLVMError;
    }
    
    pub fn buildSub(self: *LLVMBackendFixed, lhs: LLVMValueRef, rhs: LLVMValueRef, name: []const u8) !LLVMValueRef {
        const name_cstr = try std.fmt.allocPrintZ(self.allocator, "{s}", .{name});
        defer self.allocator.free(name_cstr);
        return llvm_build_sub(self.builder, lhs, rhs, name_cstr.ptr) orelse error.LLVMError;
    }
    
    pub fn getInt32Type(self: *LLVMBackendFixed) LLVMTypeRef {
        return llvm_int32_type(self.context);
    }
    
    pub fn getVoidType(self: *LLVMBackendFixed) LLVMTypeRef {
        // Void type not in wrapper yet, return int32 for now
        return llvm_int32_type(self.context);
    }
    
    pub fn getInt8PtrType(self: *LLVMBackendFixed) LLVMTypeRef {
        const int8_type = llvm_int8_type(self.context);
        return llvm_pointer_type(int8_type);
    }
    
    pub fn verifyModule(self: *LLVMBackendFixed) !void {
        if (llvm_verify_module(self.module) != 0) {
            print("LLVM module verification failed\n", .{});
            return error.LLVMModuleVerificationFailed;
        }
    }
    
    pub fn printModuleToFile(self: *LLVMBackendFixed, filename: []const u8) !void {
        // For now, just print to string and write to file manually
        const module_str = llvm_print_module_to_string(self.module);
        defer llvm_dispose_message(module_str);
        
        const file = std.fs.cwd().createFile(filename, .{}) catch |err| {
            print("Failed to create file {s}: {}\n", .{ filename, err });
            return error.LLVMWriteFailed;
        };
        defer file.close();
        
        _ = file.writeAll(std.mem.span(module_str)) catch |err| {
            print("Failed to write to file {s}: {}\n", .{ filename, err });
            return error.LLVMWriteFailed;
        };
    }
    
    pub fn writeBitcodeToFile(self: *LLVMBackendFixed, filename: []const u8) !void {
        const filename_cstr = try self.allocator.dupeZ(u8, filename);
        defer self.allocator.free(filename_cstr);
        
        if (llvm_write_bitcode_to_file(self.module, filename_cstr.ptr) != 0) {
            return error.LLVMBitcodeWriteFailed;
        }
    }
    
    pub fn optimizeModule(self: *LLVMBackendFixed) !void {
        // Optimization not implemented in simplified wrapper
        _ = self;
        // Skip optimization for now
    }
};

// Test function to verify the fixed backend works
pub fn testLLVMBackendFixed(allocator: Allocator) !void {
    print("Testing fixed LLVM backend...\n", .{});
    
    var backend = try LLVMBackendFixed.init(allocator, "test_module");
    defer backend.deinit();
    
    // Create a simple function: int main() { return 42; }
    const int32_type = backend.getInt32Type();
    var empty_params2: [0]LLVMTypeRef = undefined;
    const main_func = try backend.createFunction("main", int32_type, empty_params2[0..], false);
    
    const entry_block = try backend.createBasicBlock(main_func, "entry");
    backend.positionBuilder(entry_block);
    
    const return_value = backend.buildConstInt(int32_type, 42);
    _ = backend.buildRet(return_value);
    
    try backend.verifyModule();
    try backend.printModuleToFile("test_fixed_output.ll");
    
    print("✅ Fixed LLVM backend working correctly!\n", .{});
}

// Import reliable IR generator
const reliable_ir = @import("reliable_llvm_ir_generator.zig");

// Interface functions for compatibility with enhanced_compiler.zig
pub fn compileToLLVM(allocator: Allocator, source: []const u8, output_file: []const u8) !void {
    print("[LLVM] Compiling CURSED program without C imports...\n", .{});
    
    // Use reliable IR generator first
    reliable_ir.generateReliableLLVMIR(allocator, source, output_file) catch |err| {
        print("⚠️ Reliable IR generation failed: {any}, falling back to simplified backend\n", .{err});
        return compileToLLVMFallback(allocator, source, output_file);
    };
    
    return; // Success with reliable generator
}

// Fallback implementation using original backend
fn compileToLLVMFallback(allocator: Allocator, source: []const u8, output_file: []const u8) !void {
    print("[LLVM] Using fallback LLVM backend...\n", .{});
    
    // Parse the source code (simplified for now)
    var backend = try LLVMBackendFixed.init(allocator, "cursed_program");
    defer backend.deinit();
    
    // Create main function
    const int32_type = backend.getInt32Type();
    var empty_params: [0]LLVMTypeRef = undefined;
    const main_func = try backend.createFunction("main", int32_type, empty_params[0..], false);
    
    const entry_block = try backend.createBasicBlock(main_func, "entry");
    backend.positionBuilder(entry_block);
    
    // Generate printf call for output
    const int8_ptr_type = backend.getInt8PtrType();
    var param_types = [_]LLVMTypeRef{int8_ptr_type};
    const printf_func = try backend.createFunction("printf", int32_type, &param_types, true);
    
    // Store variables for reference with LLVM values
    var variables = std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    var llvm_variables = std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    defer {
        var iterator = variables.iterator();
        while (iterator.next()) |entry| {
            allocator.free(entry.key_ptr.*);
        }
        variables.deinit();
        
        var llvm_iterator = llvm_variables.iterator();
        while (llvm_iterator.next()) |entry| {
            allocator.free(entry.key_ptr.*);
        }
        llvm_variables.deinit();
    }
    
    // Store user-defined functions
    var user_functions = std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    defer {
        var func_iterator = user_functions.iterator();
        while (func_iterator.next()) |entry| {
            allocator.free(entry.key_ptr.*);
        }
        user_functions.deinit();
    }
    
    // Two-pass compilation: first collect functions, then generate code
    try parseAndCollectFunctions(allocator, source, &backend, &user_functions);
    try generateFunctionBodies(allocator, source, &backend, &variables, &llvm_variables, &user_functions, printf_func);
    
    // Return 0 from main
    const return_value = backend.buildConstInt(int32_type, 0);
    _ = backend.buildRet(return_value);
    
    // Verify and save module
    try backend.verifyModule();
    try backend.printModuleToFile(output_file);
    
    print("[LLVM] Generated IR: {s}\n", .{output_file});
}

// Helper function to compile variable declarations
fn compileVariableDeclaration(allocator: Allocator, line: []const u8, variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !void {
    // Parse: sus x drip = 10 OR sus x = 10 (simplified version)
    var parts = std.mem.tokenizeScalar(u8, line, ' ');
    _ = parts.next(); // skip "sus"
    
    const var_name = parts.next() orelse return error.InvalidSyntax;
    
    // Check if type is present (optional in simple cases)
    var has_type = false;
    const next_part = parts.next() orelse return error.InvalidSyntax;
    var equals_part: ?[]const u8 = null;
    
    if (std.mem.eql(u8, next_part, "=")) {
        // No type specified: sus x = 10
        equals_part = next_part;
    } else {
        // Type specified: sus x drip = 10
        has_type = true;
        equals_part = parts.next();
    }
    
    if (equals_part == null or !std.mem.eql(u8, equals_part.?, "=")) {
        return error.InvalidSyntax;
    }
    
    const value_str = parts.rest();
    const trimmed_value = std.mem.trim(u8, value_str, " \t\r\n");
    
    // Handle different types of values
    if (std.fmt.parseInt(i64, trimmed_value, 10)) |value| {
        // Simple integer literal
        const var_name_copy = try allocator.dupe(u8, var_name);
        try variables.put(var_name_copy, value);
    } else |_| {
        // Try to evaluate arithmetic expressions
        const evaluated_value = evaluateArithmeticExpression(trimmed_value, variables) catch 0;
        const var_name_copy = try allocator.dupe(u8, var_name);
        try variables.put(var_name_copy, evaluated_value);
    }
    
    // Use has_type flag for future type system integration
    if (has_type) {
        // Type information available for future use
    }
}

// Helper function to compile vibez.spill() statements
fn compileVibesSpillStatement(
    allocator: Allocator, 
    line: []const u8, 
    start: usize, 
    backend: *LLVMBackendFixed, 
    printf_func: LLVMValueRef,
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !void {
    // Find the content inside vibez.spill(...)
    if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
        if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
            const content_start = start + paren_start + 1;
            const content = line[content_start..paren_end];
            const trimmed_content = std.mem.trim(u8, content, " \t");
            
            // Handle different types of content
            if (trimmed_content.len >= 2 and trimmed_content[0] == '"' and trimmed_content[trimmed_content.len - 1] == '"') {
                // String literal: vibez.spill("Hello")
                const string_content = trimmed_content[1..trimmed_content.len - 1];
                const format_str = try backend.buildConstString(try std.fmt.allocPrint(allocator, "{s}\n", .{string_content}), "fmt_str");
                
                var args = [_]LLVMValueRef{format_str};
                _ = try backend.buildCall(printf_func, &args, "printf_call");
                
            } else if (std.mem.indexOf(u8, trimmed_content, ",")) |_| {
                // Multiple arguments: vibez.spill("Value:", x)
                try compileMultiArgumentPrint(allocator, trimmed_content, backend, printf_func, variables);
                
            } else if (variables.get(trimmed_content)) |value| {
                // Single variable: vibez.spill(x)
                const format_str = try backend.buildConstString("%ld\n", "fmt_str");
                const value_const = backend.buildConstInt(backend.getInt32Type(), @intCast(value));
                
                var args = [_]LLVMValueRef{ format_str, value_const };
                _ = try backend.buildCall(printf_func, &args, "printf_call");
                
            } else {
                // Try to parse as integer literal
                if (std.fmt.parseInt(i64, trimmed_content, 10)) |value| {
                    const format_str = try backend.buildConstString("%ld\n", "fmt_str");
                    const value_const = backend.buildConstInt(backend.getInt32Type(), @intCast(value));
                    
                    var args = [_]LLVMValueRef{ format_str, value_const };
                    _ = try backend.buildCall(printf_func, &args, "printf_call");
                } else |_| {
                    // Default: print the raw content
                    const format_str = try backend.buildConstString(try std.fmt.allocPrint(allocator, "%s\n", .{}), "fmt_str");
                    const string_value = try backend.buildConstString(trimmed_content, "str_val");
                    
                    var args = [_]LLVMValueRef{ format_str, string_value };
                    _ = try backend.buildCall(printf_func, &args, "printf_call");
                }
            }
        }
    }
}

// Helper function to evaluate arithmetic expressions  
fn evaluateArithmeticExpression(
    expr: []const u8, 
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !i64 {
    const trimmed = std.mem.trim(u8, expr, " \t\r\n");
    
    // Handle simple variable references
    if (variables.get(trimmed)) |value| {
        return value;
    }
    
    // Handle arithmetic operations with precedence (basic implementation)
    // For now, handle simple cases: x + y * 2
    if (std.mem.indexOf(u8, trimmed, " + ")) |plus_pos| {
        const left_expr = std.mem.trim(u8, trimmed[0..plus_pos], " \t");
        const right_expr = std.mem.trim(u8, trimmed[plus_pos + 3..], " \t");
        
        const left_val = try evaluateSimpleExpression(left_expr, variables);
        const right_val = try evaluateSimpleExpression(right_expr, variables);
        
        return left_val + right_val;
    }
    
    // Handle multiplication
    if (std.mem.indexOf(u8, trimmed, " * ")) |mult_pos| {
        const left_expr = std.mem.trim(u8, trimmed[0..mult_pos], " \t");
        const right_expr = std.mem.trim(u8, trimmed[mult_pos + 3..], " \t");
        
        const left_val = try evaluateSimpleExpression(left_expr, variables);
        const right_val = try evaluateSimpleExpression(right_expr, variables);
        
        return left_val * right_val;
    }
    
    // Handle subtraction
    if (std.mem.indexOf(u8, trimmed, " - ")) |minus_pos| {
        const left_expr = std.mem.trim(u8, trimmed[0..minus_pos], " \t");
        const right_expr = std.mem.trim(u8, trimmed[minus_pos + 3..], " \t");
        
        const left_val = try evaluateSimpleExpression(left_expr, variables);
        const right_val = try evaluateSimpleExpression(right_expr, variables);
        
        return left_val - right_val;
    }
    
    // Try to parse as literal
    return std.fmt.parseInt(i64, trimmed, 10) catch 0;
}

// Helper function to evaluate simple expressions (literals and variables)
fn evaluateSimpleExpression(
    expr: []const u8,
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !i64 {
    const trimmed = std.mem.trim(u8, expr, " \t");
    
    // Check if it's a variable
    if (variables.get(trimmed)) |value| {
        return value;
    }
    
    // Handle y * 2 pattern specifically
    if (std.mem.indexOf(u8, trimmed, " * ")) |mult_pos| {
        const left_part = std.mem.trim(u8, trimmed[0..mult_pos], " \t");
        const right_part = std.mem.trim(u8, trimmed[mult_pos + 3..], " \t");
        
        var left_val: i64 = 0;
        var right_val: i64 = 0;
        
        // Get left value
        if (variables.get(left_part)) |val| {
            left_val = val;
        } else {
            left_val = std.fmt.parseInt(i64, left_part, 10) catch 0;
        }
        
        // Get right value
        if (variables.get(right_part)) |val| {
            right_val = val;
        } else {
            right_val = std.fmt.parseInt(i64, right_part, 10) catch 0;
        }
        
        return left_val * right_val;
    }
    
    // Try to parse as literal
    return std.fmt.parseInt(i64, trimmed, 10) catch 0;
}

// Helper function to compile multi-argument print statements
fn compileMultiArgumentPrint(
    allocator: Allocator,
    content: []const u8,
    backend: *LLVMBackendFixed,
    printf_func: LLVMValueRef,
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !void {
    // Parse arguments separated by commas
    var args_list = std.ArrayList(LLVMValueRef).init(self.allocator);
    defer args_list.deinit();
    
    var format_parts = std.ArrayList([]const u8).init(self.allocator);
    defer {
        for (format_parts.items) |part| {
            allocator.free(part);
        }
        format_parts.deinit();
    }
    
    var arg_iter = std.mem.tokenizeScalar(u8, content, ',');
    while (arg_iter.next()) |arg| {
        const trimmed_arg = std.mem.trim(u8, arg, " \t");
        
        if (trimmed_arg.len >= 2 and trimmed_arg[0] == '"' and trimmed_arg[trimmed_arg.len - 1] == '"') {
            // String argument
            const string_content = trimmed_arg[1..trimmed_arg.len - 1];
            try format_parts.append(try allocator.dupe(u8, string_content));
            
        } else if (variables.get(trimmed_arg)) |value| {
            // Variable argument
            try format_parts.append(try allocator.dupe(u8, " %ld"));
            const value_const = backend.buildConstInt(backend.getInt32Type(), @intCast(value));
            try args_list.append(value_const);
            
        } else if (std.fmt.parseInt(i64, trimmed_arg, 10)) |value| {
            // Integer literal
            try format_parts.append(try allocator.dupe(u8, " %ld"));
            const value_const = backend.buildConstInt(backend.getInt32Type(), @intCast(value));
            try args_list.append(value_const);
            
        } else |_| {
            // Unknown argument, treat as string
            try format_parts.append(try allocator.dupe(u8, " %s"));
            const string_value = try backend.buildConstString(trimmed_arg, "str_val");
            try args_list.append(string_value);
        }
    }
    
    // Build format string
    var format_string = std.ArrayList(u8).init(self.allocator);
    defer format_string.deinit();
    
    for (format_parts.items) |part| {
        try format_string.appendSlice(part);
    }
    try format_string.appendSlice("\n");
    
    const format_str = try backend.buildConstString(format_string.items, "fmt_str");
    
    // Create arguments array
    var final_args = std.ArrayList(LLVMValueRef).init(self.allocator);
    defer final_args.deinit();
    
    try final_args.append(format_str);
    try final_args.appendSlice(args_list.items);
    
    _ = try backend.buildCall(printf_func, final_args.items, "printf_call");
}

pub fn compileIRToNative(allocator: Allocator, ir_file: []const u8, output_file: []const u8) !void {
    print("[LLVM] Compiling IR to native executable...\n", .{});
    
    // Try multiple compilers in order of preference
    const compilers = [_][]const u8{ "clang", "clang-18", "gcc" };
    
    for (compilers) |compiler| {
        const compile_args = [_][]const u8{
            compiler,
            "-O2",
            "-o", output_file,
            ir_file,
        };
        
        var process = std.process.Child.init(&compile_args, allocator);
        process.stdout_behavior = .Ignore;
        process.stderr_behavior = .Ignore;
        
        if (process.spawnAndWait()) |result| {
            switch (result) {
                .Exited => |code| {
                    if (code == 0) {
                        print("✅ Native executable created with {s}: {s}\n", .{ compiler, output_file });
                        return;
                    } else {
                        print("⚠️  {s} compilation failed with code: {d}, trying next compiler...\n", .{ compiler, code });
                    }
                },
                else => {
                    print("⚠️  {s} process failed, trying next compiler...\n", .{compiler});
                },
            }
        } else |err| {
            if (err == error.FileNotFound) {
                print("⚠️  {s} not found, trying next compiler...\n", .{compiler});
            } else {
                print("⚠️  Error with {s}: {}, trying next compiler...\n", .{ compiler, err });
            }
        }
    }
    
    print("❌ All compilers failed or not found\n", .{});
    return error.CompilationFailed;
}

// Helper function to parse and collect function definitions
fn parseAndCollectFunctions(
    allocator: Allocator,
    source: []const u8,
    backend: *LLVMBackendFixed,
    user_functions: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !void {
    var lines = std.mem.splitScalar(u8, source, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        // Skip empty lines and comments
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        // Handle function definitions: slay function_name(params) return_type { ... }
        if (std.mem.startsWith(u8, trimmed, "slay ")) {
            try parseFunctionDefinition(allocator, trimmed, backend, user_functions);
        }
    }
}

// Helper function to generate function bodies and main program
fn generateFunctionBodies(
    allocator: Allocator,
    source: []const u8,
    backend: *LLVMBackendFixed,
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    llvm_variables: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    user_functions: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    printf_func: LLVMValueRef
) !void {
    var lines = std.mem.splitScalar(u8, source, '\n');
    var in_function = false;
    var current_function: ?LLVMValueRef = null;
    var function_body_lines = std.ArrayList([]const u8).init(self.allocator);
    defer function_body_lines.deinit();
    
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
        
        if (std.mem.startsWith(u8, trimmed, "slay ")) {
            // Start collecting function body
            in_function = true;
            const func_name = try extractFunctionName(allocator, trimmed);
            current_function = user_functions.get(func_name);
            allocator.free(func_name);
            continue;
        }
        
        if (in_function) {
            if (std.mem.indexOf(u8, trimmed, "}")) |_| {
                // End of function - generate body
                if (current_function) |func| {
                    try generateFunctionBodyLLVM(allocator, function_body_lines.items, backend, func, variables, llvm_variables, user_functions, printf_func);
                }
                in_function = false;
                current_function = null;
                function_body_lines.clearRetainingCapacity();
            } else {
                // Collect function body line
                const line_copy = try allocator.dupe(u8, trimmed);
                try function_body_lines.append(line_copy);
            }
            continue;
        }
        
        // Handle main program statements
        
        // Handle variable declarations
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            try compileVariableDeclarationLLVM(allocator, trimmed, backend, variables, llvm_variables);
        }
        
        // Handle vibez.spill() statements
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            compileVibesSpillStatement(allocator, trimmed, start, backend, printf_func, variables) catch |err| {
                print("Warning: Failed to compile print statement: {}\n", .{err});
            };
        }
        
        // Handle function calls
        if (containsFunctionCall(trimmed)) {
            try compileFunctionCall(allocator, trimmed, backend, variables, llvm_variables, user_functions);
        }
    }
    
    // Clean up function body lines
    for (function_body_lines.items) |body_line| {
        allocator.free(body_line);
    }
}

// Helper function to parse function definition syntax
fn parseFunctionDefinition(
    allocator: Allocator,
    line: []const u8,
    backend: *LLVMBackendFixed,
    user_functions: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !void {
    // Parse: slay function_name(param1 type1, param2 type2) return_type {
    const int32_type = backend.getInt32Type();
    
    // Extract function name (simplified parsing)
    var parts = std.mem.tokenizeScalar(u8, line, ' ');
    _ = parts.next(); // skip "slay"
    const func_part = parts.next() orelse return error.InvalidSyntax;
    
    // Extract function name before '('
    const paren_pos = std.mem.indexOf(u8, func_part, "(") orelse return error.InvalidSyntax;
    const func_name = func_part[0..paren_pos];
    
    // For now, create simple functions that return int32 and take int32 parameters
    // TODO: Parse actual parameter types
    var param_types = [_]LLVMTypeRef{ int32_type, int32_type }; // Assume 2 int parameters for now
    const func_type = try backend.createFunction(func_name, int32_type, param_types[0..2], false);
    
    const func_name_copy = try allocator.dupe(u8, func_name);
    try user_functions.put(func_name_copy, func_type);
    
    print("[LLVM] Created function: {s}\n", .{func_name});
}

// Helper function to extract function name from definition
fn extractFunctionName(allocator: Allocator, line: []const u8) ![]const u8 {
    var parts = std.mem.tokenizeScalar(u8, line, ' ');
    _ = parts.next(); // skip "slay"
    const func_part = parts.next() orelse return error.InvalidSyntax;
    
    const paren_pos = std.mem.indexOf(u8, func_part, "(") orelse return error.InvalidSyntax;
    return try allocator.dupe(u8, func_part[0..paren_pos]);
}

// Helper function to generate LLVM function body
fn generateFunctionBodyLLVM(
    allocator: Allocator,
    body_lines: []const []const u8,
    backend: *LLVMBackendFixed,
    function: LLVMValueRef,
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    llvm_variables: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    user_functions: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    printf_func: LLVMValueRef
) !void {
    // Create function entry block
    const entry_block = try backend.createBasicBlock(function, "entry");
    backend.positionBuilder(entry_block);
    
    for (body_lines) |line| {
        // Handle return statements: damn expression
        if (std.mem.startsWith(u8, line, "damn ")) {
            const expr = std.mem.trim(u8, line[5..], " \t");
            const return_value = try evaluateExpressionLLVM(allocator, expr, backend, variables, llvm_variables, user_functions);
            _ = backend.buildRet(return_value);
            break;
        }
        
        // Handle other statements in function body
        // For now, skip other statements in function bodies
    }
    
    _ = printf_func; // Mark as used to prevent warning
}

// Helper function to compile variable declarations with LLVM
fn compileVariableDeclarationLLVM(
    allocator: Allocator,
    line: []const u8,
    backend: *LLVMBackendFixed,
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    llvm_variables: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !void {
    // Parse and evaluate variable declaration
    try compileVariableDeclaration(allocator, line, variables);
    
    // Also create LLVM alloca for the variable
    var parts = std.mem.tokenizeScalar(u8, line, ' ');
    _ = parts.next(); // skip "sus"
    const var_name = parts.next() orelse return error.InvalidSyntax;
    
    // Skip optional type
    var next_part = parts.next() orelse return error.InvalidSyntax;
    if (!std.mem.eql(u8, next_part, "=")) {
        next_part = parts.next() orelse return error.InvalidSyntax;
    }
    
    if (variables.get(var_name)) |value| {
        const int32_type = backend.getInt32Type();
        const llvm_value = backend.buildConstInt(int32_type, @intCast(value));
        
        const var_name_copy = try allocator.dupe(u8, var_name);
        try llvm_variables.put(var_name_copy, llvm_value);
    }
}

// Helper function to evaluate expressions to LLVM values
fn evaluateExpressionLLVM(
    allocator: Allocator,
    expr: []const u8,
    backend: *LLVMBackendFixed,
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    llvm_variables: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    user_functions: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !LLVMValueRef {
    const trimmed = std.mem.trim(u8, expr, " \t");
    
    // Check if it's a variable
    if (llvm_variables.get(trimmed)) |llvm_value| {
        return llvm_value;
    }
    
    // Check if it's a literal integer
    if (std.fmt.parseInt(i64, trimmed, 10)) |value| {
        return backend.buildConstInt(backend.getInt32Type(), @intCast(value));
    } else |_| {}
    
    // Handle arithmetic expressions
    if (std.mem.indexOf(u8, trimmed, " + ")) |plus_pos| {
        const left_expr = std.mem.trim(u8, trimmed[0..plus_pos], " \t");
        const right_expr = std.mem.trim(u8, trimmed[plus_pos + 3..], " \t");
        
        const left_val = try evaluateExpressionLLVM(allocator, left_expr, backend, variables, llvm_variables, user_functions);
        const right_val = try evaluateExpressionLLVM(allocator, right_expr, backend, variables, llvm_variables, user_functions);
        
        return try backend.buildAdd(left_val, right_val, "add_result");
    }
    
    // Handle multiplication
    if (std.mem.indexOf(u8, trimmed, " * ")) |mult_pos| {
        const left_expr = std.mem.trim(u8, trimmed[0..mult_pos], " \t");
        const right_expr = std.mem.trim(u8, trimmed[mult_pos + 3..], " \t");
        
        const left_val = try evaluateExpressionLLVM(allocator, left_expr, backend, variables, llvm_variables, user_functions);
        const right_val = try evaluateExpressionLLVM(allocator, right_expr, backend, variables, llvm_variables, user_functions);
        
        return try backend.buildMul(left_val, right_val, "mul_result");
    }
    
    // Handle subtraction
    if (std.mem.indexOf(u8, trimmed, " - ")) |minus_pos| {
        const left_expr = std.mem.trim(u8, trimmed[0..minus_pos], " \t");
        const right_expr = std.mem.trim(u8, trimmed[minus_pos + 3..], " \t");
        
        const left_val = try evaluateExpressionLLVM(allocator, left_expr, backend, variables, llvm_variables, user_functions);
        const right_val = try evaluateExpressionLLVM(allocator, right_expr, backend, variables, llvm_variables, user_functions);
        
        return try backend.buildSub(left_val, right_val, "sub_result");
    }
    
    // Handle function calls
    if (std.mem.indexOf(u8, trimmed, "(")) |paren_pos| {
        const func_name = std.mem.trim(u8, trimmed[0..paren_pos], " \t");
        if (user_functions.get(func_name)) |func| {
            // Extract arguments (simplified)
            const args_start = paren_pos + 1;
            const args_end = std.mem.lastIndexOf(u8, trimmed, ")") orelse return error.InvalidSyntax;
            const args_str = trimmed[args_start..args_end];
            
            // Parse arguments (simplified - assume 2 integer arguments)
            var args_list = std.ArrayList(LLVMValueRef).init(self.allocator);
            defer args_list.deinit();
            
            if (args_str.len > 0) {
                var arg_parts = std.mem.tokenizeScalar(u8, args_str, ',');
                while (arg_parts.next()) |arg| {
                    const trimmed_arg = std.mem.trim(u8, arg, " \t");
                    const arg_value = try evaluateExpressionLLVM(allocator, trimmed_arg, backend, variables, llvm_variables, user_functions);
                    try args_list.append(arg_value);
                }
            }
            
            return try backend.buildCall(func, args_list.items, "call_result");
        }
    }
    
    // Default: return 0
    return backend.buildConstInt(backend.getInt32Type(), 0);
}

// Helper function to check if line contains function call
fn containsFunctionCall(line: []const u8) bool {
    // Simple heuristic: contains '(' and ')' but doesn't start with known keywords
    if (std.mem.indexOf(u8, line, "(") == null) return false;
    if (std.mem.indexOf(u8, line, ")") == null) return false;
    
    // Exclude known patterns
    if (std.mem.startsWith(u8, line, "sus ")) return false;
    if (std.mem.startsWith(u8, line, "vibez.spill(")) return false;
    if (std.mem.startsWith(u8, line, "slay ")) return false;
    if (std.mem.startsWith(u8, line, "ready (")) return false;
    if (std.mem.startsWith(u8, line, "bestie (")) return false;
    
    return true;
}

// Helper function to compile standalone function calls
fn compileFunctionCall(
    allocator: Allocator,
    line: []const u8,
    backend: *LLVMBackendFixed,
    variables: *std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    llvm_variables: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    user_functions: *std.HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
) !void {
    // Handle variable assignment with function call: sus result = func_call(args)
    if (std.mem.startsWith(u8, line, "sus ")) {
        const equals_pos = std.mem.indexOf(u8, line, "=") orelse return;
        const var_part = std.mem.trim(u8, line[4..equals_pos], " \t");
        const expr_part = std.mem.trim(u8, line[equals_pos + 1..], " \t");
        
        const result_value = try evaluateExpressionLLVM(allocator, expr_part, backend, variables, llvm_variables, user_functions);
        
        // Store variable name with result value
        const var_name_copy = try allocator.dupe(u8, var_part);
        try llvm_variables.put(var_name_copy, result_value);
        
        // Also store in regular variables for compatibility
        // Extract numeric value if possible
        const var_name_copy2 = try allocator.dupe(u8, var_part);
        try variables.put(var_name_copy2, 0); // Placeholder value
    }
}
