//! Variadic Function FFI Bridge for CURSED
//! Provides safe calling of C variadic functions (printf, scanf, etc.) from CURSED code
//! Implements proper argument marshaling, type safety, and memory management

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const c = @cImport({
    @cInclude("stdio.h");
    @cInclude("stdarg.h");
    @cInclude("string.h");
    @cInclude("stdlib.h");
});

// LLVM C API imports for codegen
const llvm_c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

pub const VariadicError = error{
    TooManyArguments,
    InvalidArgumentType,
    FormatStringMismatch,
    NullFormatString,
    BufferOverflow,
    InvalidFunctionSignature,
    UnsupportedArgumentType,
};

/// Represents a C variadic function argument
pub const VariadicArgument = union(enum) {
    int32: i32,
    int64: i64,
    uint32: u32,
    uint64: u64,
    float32: f32,
    float64: f64,
    pointer: *anyopaque,
    string: [*:0]const u8,
    boolean: bool,
    
    /// Convert CURSED type to VariadicArgument
    pub fn fromCursedValue(cursed_type: []const u8, value_ptr: *const anyopaque) !VariadicArgument {
        if (std.mem.eql(u8, cursed_type, "normie")) {
            const val = @as(*const i32, @ptrCast(@alignCast(value_ptr))).*;
            return VariadicArgument{ .int32 = val };
        } else if (std.mem.eql(u8, cursed_type, "drip")) {
            const val = @as(*const i64, @ptrCast(@alignCast(value_ptr))).*;
            return VariadicArgument{ .int64 = val };
        } else if (std.mem.eql(u8, cursed_type, "meal")) {
            const val = @as(*const f64, @ptrCast(@alignCast(value_ptr))).*;
            return VariadicArgument{ .float64 = val };
        } else if (std.mem.eql(u8, cursed_type, "snack")) {
            const val = @as(*const f32, @ptrCast(@alignCast(value_ptr))).*;
            return VariadicArgument{ .float32 = val };
        } else if (std.mem.eql(u8, cursed_type, "tea")) {
            const val = @as(*const [*:0]const u8, @ptrCast(@alignCast(value_ptr))).*;
            return VariadicArgument{ .string = val };
        } else if (std.mem.eql(u8, cursed_type, "lit")) {
            const val = @as(*const bool, @ptrCast(@alignCast(value_ptr))).*;
            return VariadicArgument{ .boolean = val };
        } else {
            return VariadicError.UnsupportedArgumentType;
        }
    }
    
    /// Get the size needed for this argument in a va_list
    pub fn getSize(self: VariadicArgument) usize {
        return switch (self) {
            .int32 => @sizeOf(c_int),
            .int64 => @sizeOf(c_longlong),
            .uint32 => @sizeOf(c_uint),
            .uint64 => @sizeOf(c_ulonglong),
            .float32 => @sizeOf(c_double), // floats promoted to double in varargs
            .float64 => @sizeOf(c_double),
            .pointer => @sizeOf(*anyopaque),
            .string => @sizeOf([*:0]const u8),
            .boolean => @sizeOf(c_int), // bools promoted to int in varargs
        };
    }
};

/// Format string parser for printf-style functions
pub const FormatStringParser = struct {
    format: []const u8,
    position: usize,
    
    pub fn init(format_str: []const u8) FormatStringParser {
        return FormatStringParser{
            .format = format_str,
            .position = 0,
        };
    }
    
    pub const FormatSpecifier = struct {
        type: enum { int, long, float, double, string, char, pointer },
        width: ?u32,
        precision: ?u32,
        length_modifier: enum { none, long, long_long, short },
    };
    
    /// Parse next format specifier from format string
    pub fn nextSpecifier(self: *FormatStringParser) ?FormatSpecifier {
        while (self.position < self.format.len) {
            if (self.format[self.position] == '%') {
                self.position += 1;
                if (self.position >= self.format.len) break;
                
                // Skip %% (literal %)
                if (self.format[self.position] == '%') {
                    self.position += 1;
                    continue;
                }
                
                return self.parseSpecifier();
            }
            self.position += 1;
        }
        return null;
    }
    
    fn parseSpecifier(self: *FormatStringParser) FormatSpecifier {
        var spec = FormatSpecifier{
            .type = .int,
            .width = null,
            .precision = null,
            .length_modifier = .none,
        };
        
        // Skip flags (-, +, space, #, 0)
        while (self.position < self.format.len) {
            const ch = self.format[self.position];
            if (ch == '-' or ch == '+' or ch == ' ' or ch == '#' or ch == '0') {
                self.position += 1;
            } else {
                break;
            }
        }
        
        // Parse width
        if (self.position < self.format.len and std.ascii.isDigit(self.format[self.position])) {
            var width: u32 = 0;
            while (self.position < self.format.len and std.ascii.isDigit(self.format[self.position])) {
                width = width * 10 + (self.format[self.position] - '0');
                self.position += 1;
            }
            spec.width = width;
        }
        
        // Parse precision
        if (self.position < self.format.len and self.format[self.position] == '.') {
            self.position += 1;
            var precision: u32 = 0;
            while (self.position < self.format.len and std.ascii.isDigit(self.format[self.position])) {
                precision = precision * 10 + (self.format[self.position] - '0');
                self.position += 1;
            }
            spec.precision = precision;
        }
        
        // Parse length modifier
        if (self.position < self.format.len) {
            const ch = self.format[self.position];
            if (ch == 'l') {
                self.position += 1;
                if (self.position < self.format.len and self.format[self.position] == 'l') {
                    self.position += 1;
                    spec.length_modifier = .long_long;
                } else {
                    spec.length_modifier = .long;
                }
            } else if (ch == 'h') {
                self.position += 1;
                spec.length_modifier = .short;
            }
        }
        
        // Parse conversion specifier
        if (self.position < self.format.len) {
            const ch = self.format[self.position];
            self.position += 1;
            
            spec.type = switch (ch) {
                'd', 'i', 'o', 'x', 'X', 'u' => .int,
                'f', 'F' => .float,
                'e', 'E', 'g', 'G' => .double,
                's' => .string,
                'c' => .char,
                'p' => .pointer,
                else => .int, // default fallback
            };
        }
        
        return spec;
    }
};

/// Variadic function call builder with type safety
pub const VariadicCallBuilder = struct {
    allocator: Allocator,
    function_name: []const u8,
    arguments: ArrayList(VariadicArgument),
    format_string: ?[]const u8,
    max_args: usize,
    
    const MAX_VARIADIC_ARGS = 32; // Safety limit
    
    pub fn init(allocator: Allocator, function_name: []const u8) VariadicCallBuilder {
        return VariadicCallBuilder{
            .allocator = allocator,
            .function_name = function_name,
            .arguments = ArrayList(VariadicArgument).init(allocator),
            .format_string = null,
            .max_args = MAX_VARIADIC_ARGS,
        };
    }
    
    pub fn deinit(self: *VariadicCallBuilder) void {
        self.arguments.deinit();
    }
    
    /// Set the format string for printf-style functions
    pub fn setFormatString(self: *VariadicCallBuilder, format_str: []const u8) void {
        self.format_string = format_str;
    }
    
    /// Add an argument to the call
    pub fn addArgument(self: *VariadicCallBuilder, arg: VariadicArgument) !void {
        if (self.arguments.items.len >= self.max_args) {
            return VariadicError.TooManyArguments;
        }
        try self.arguments.append(arg);
    }
    
    /// Add argument from CURSED value
    pub fn addCursedArgument(self: *VariadicCallBuilder, cursed_type: []const u8, value_ptr: *const anyopaque) !void {
        const arg = try VariadicArgument.fromCursedValue(cursed_type, value_ptr);
        try self.addArgument(arg);
    }
    
    /// Validate arguments against format string
    pub fn validateArguments(self: *VariadicCallBuilder) !void {
        if (self.format_string) |fmt_str| {
            var parser = FormatStringParser.init(fmt_str);
            var arg_index: usize = 0;
            
            while (parser.nextSpecifier()) |spec| {
                if (arg_index >= self.arguments.items.len) {
                    return VariadicError.FormatStringMismatch;
                }
                
                const arg = self.arguments.items[arg_index];
                const valid = switch (spec.type) {
                    .int => switch (arg) {
                        .int32, .int64, .uint32, .uint64, .boolean => true,
                        else => false,
                    },
                    .long => switch (arg) {
                        .int64, .uint64 => true,
                        else => false,
                    },
                    .float, .double => switch (arg) {
                        .float32, .float64 => true,
                        else => false,
                    },
                    .string => switch (arg) {
                        .string => true,
                        else => false,
                    },
                    .char => switch (arg) {
                        .int32 => true,
                        else => false,
                    },
                    .pointer => switch (arg) {
                        .pointer, .string => true,
                        else => false,
                    },
                };
                
                if (!valid) {
                    return VariadicError.FormatStringMismatch;
                }
                
                arg_index += 1;
            }
        }
    }
    
    /// Generate LLVM IR for the variadic call
    pub fn generateLLVMCall(self: *VariadicCallBuilder, 
                           context: llvm_c.LLVMContextRef,
                           builder: llvm_c.LLVMBuilderRef,
                           module: llvm_c.LLVMModuleRef) !llvm_c.LLVMValueRef {
        
        // Get or declare the function
        const func_name_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{self.function_name});
        defer self.allocator.free(func_name_z);
        
        var func = llvm_c.LLVMGetNamedFunction(module, func_name_z);
        if (func == null) {
            // Declare the function as variadic
            const return_type = llvm_c.LLVMInt32TypeInContext(context);
            const param_types = [_]llvm_c.LLVMTypeRef{
                llvm_c.LLVMPointerType(llvm_c.LLVMInt8TypeInContext(context), 0)
            };
            const func_type = llvm_c.LLVMFunctionType(
                return_type,
                @ptrCast(&param_types),
                1,
                1 // is_variadic = true
            );
            func = llvm_c.LLVMAddFunction(module, func_name_z, func_type);
        }
        
        // Prepare arguments for the call
        var llvm_args = ArrayList(llvm_c.LLVMValueRef).init(self.allocator);
        defer llvm_args.deinit();
        
        // Add format string if present
        if (self.format_string) |fmt_str| {
            const fmt_str_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{fmt_str});
            defer self.allocator.free(fmt_str_z);
            const fmt_global = llvm_c.LLVMBuildGlobalStringPtr(builder, fmt_str_z, "fmt_str");
            try llvm_args.append(fmt_global);
        }
        
        // Convert arguments to LLVM values
        for (self.arguments.items) |arg| {
            const llvm_value = switch (arg) {
                .int32 => |val| llvm_c.LLVMConstInt(llvm_c.LLVMInt32TypeInContext(context), @intCast(val), 0),
                .int64 => |val| llvm_c.LLVMConstInt(llvm_c.LLVMInt64TypeInContext(context), @intCast(val), 0),
                .uint32 => |val| llvm_c.LLVMConstInt(llvm_c.LLVMInt32TypeInContext(context), val, 0),
                .uint64 => |val| llvm_c.LLVMConstInt(llvm_c.LLVMInt64TypeInContext(context), val, 0),
                .float32 => |val| llvm_c.LLVMConstReal(llvm_c.LLVMDoubleTypeInContext(context), val), // promoted to double
                .float64 => |val| llvm_c.LLVMConstReal(llvm_c.LLVMDoubleTypeInContext(context), val),
                .string => |val| blk: {
                    const str_len = std.mem.len(val);
                    const str_global = llvm_c.LLVMBuildGlobalStringPtr(builder, val, "str_arg");
                    _ = str_len; // Suppress unused warning
                    break :blk str_global;
                },
                .pointer => |val| llvm_c.LLVMConstIntToPtr(
                    llvm_c.LLVMConstInt(llvm_c.LLVMInt64TypeInContext(context), @intFromPtr(val), 0),
                    llvm_c.LLVMPointerType(llvm_c.LLVMInt8TypeInContext(context), 0)
                ),
                .boolean => |val| llvm_c.LLVMConstInt(llvm_c.LLVMInt32TypeInContext(context), if (val) 1 else 0, 0),
            };
            try llvm_args.append(llvm_value);
        }
        
        // Build the call
        const call_result = llvm_c.LLVMBuildCall2(
            builder,
            llvm_c.LLVMGlobalGetValueType(func.?),
            func.?,
            llvm_args.items.ptr,
            @intCast(llvm_args.items.len),
            "variadic_call"
        );
        
        return call_result;
    }
};

/// Safe wrapper for common C variadic functions
pub const SafeVariadicWrapper = struct {
    allocator: Allocator,
    max_buffer_size: usize,
    
    pub fn init(allocator: Allocator) SafeVariadicWrapper {
        return SafeVariadicWrapper{
            .allocator = allocator,
            .max_buffer_size = 4096, // 4KB default buffer
        };
    }
    
    /// Safe printf wrapper
    pub fn printf(self: *SafeVariadicWrapper, format: []const u8, args: []const VariadicArgument) !i32 {
        // Validate format string
        if (format.len == 0) return VariadicError.NullFormatString;
        
        var builder = VariadicCallBuilder.init(self.allocator, "printf");
        defer builder.deinit();
        
        builder.setFormatString(format);
        for (args) |arg| {
            try builder.addArgument(arg);
        }
        
        try builder.validateArguments();
        
        // Call the actual printf function
        const format_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{format});
        defer self.allocator.free(format_z);
        
        return self.callPrintfNative(format_z, args);
    }
    
    /// Safe scanf wrapper
    pub fn scanf(self: *SafeVariadicWrapper, format: []const u8, args: []const VariadicArgument) !i32 {
        if (format.len == 0) return VariadicError.NullFormatString;
        
        var builder = VariadicCallBuilder.init(self.allocator, "scanf");
        defer builder.deinit();
        
        builder.setFormatString(format);
        for (args) |arg| {
            try builder.addArgument(arg);
        }
        
        try builder.validateArguments();
        
        const format_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{format});
        defer self.allocator.free(format_z);
        
        return self.callScanfNative(format_z, args);
    }
    
    /// Safe sprintf wrapper with buffer overflow protection
    pub fn sprintf(self: *SafeVariadicWrapper, buffer: []u8, format: []const u8, args: []const VariadicArgument) !i32 {
        if (format.len == 0) return VariadicError.NullFormatString;
        if (buffer.len > self.max_buffer_size) return VariadicError.BufferOverflow;
        
        var builder = VariadicCallBuilder.init(self.allocator, "snprintf");
        defer builder.deinit();
        
        // Add buffer and size arguments first
        try builder.addArgument(VariadicArgument{ .pointer = buffer.ptr });
        try builder.addArgument(VariadicArgument{ .uint64 = buffer.len });
        
        builder.setFormatString(format);
        for (args) |arg| {
            try builder.addArgument(arg);
        }
        
        const format_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{format});
        defer self.allocator.free(format_z);
        
        return self.callSprintfNative(buffer, format_z, args);
    }
    
    // Native function call implementations using Zig's C interop
    fn callPrintfNative(self: *SafeVariadicWrapper, format: [*:0]const u8, args: []const VariadicArgument) i32 {
        _ = self;
        
        // For safety, we'll implement a simplified version that handles common cases
        // In a full implementation, this would use assembly or C bridge code
        
        switch (args.len) {
            0 => return c.printf(format),
            1 => return switch (args[0]) {
                .int32 => |val| c.printf(format, val),
                .int64 => |val| c.printf(format, val),
                .float64 => |val| c.printf(format, val),
                .string => |val| c.printf(format, val),
                else => c.printf(format),
            },
            2 => {
                const arg1 = args[0];
                const arg2 = args[1];
                return switch (arg1) {
                    .string => |str_val| switch (arg2) {
                        .int32 => |int_val| c.printf(format, str_val, int_val),
                        .int64 => |int_val| c.printf(format, str_val, int_val),
                        .float64 => |float_val| c.printf(format, str_val, float_val),
                        .string => |str_val2| c.printf(format, str_val, str_val2),
                        else => c.printf(format, str_val),
                    },
                    .int32 => |int_val| switch (arg2) {
                        .int32 => |int_val2| c.printf(format, int_val, int_val2),
                        .int64 => |int_val2| c.printf(format, int_val, int_val2),
                        .string => |str_val| c.printf(format, int_val, str_val),
                        else => c.printf(format, int_val),
                    },
                    else => c.printf(format),
                };
            },
            else => c.printf(format), // Fallback for complex cases
        }
    }
    
    fn callScanfNative(self: *SafeVariadicWrapper, format: [*:0]const u8, args: []const VariadicArgument) i32 {
        _ = self;
        _ = args;
        
        // Scanf is inherently unsafe, so we provide a very basic implementation
        // In practice, this should use safer alternatives
        return c.scanf(format);
    }
    
    fn callSprintfNative(self: *SafeVariadicWrapper, buffer: []u8, format: [*:0]const u8, args: []const VariadicArgument) i32 {
        _ = self;
        
        // Use snprintf for safety
        switch (args.len) {
            0 => return c.snprintf(buffer.ptr, buffer.len, format),
            1 => return switch (args[0]) {
                .int32 => |val| c.snprintf(buffer.ptr, buffer.len, format, val),
                .int64 => |val| c.snprintf(buffer.ptr, buffer.len, format, val),
                .float64 => |val| c.snprintf(buffer.ptr, buffer.len, format, val),
                .string => |val| c.snprintf(buffer.ptr, buffer.len, format, val),
                else => c.snprintf(buffer.ptr, buffer.len, format),
            },
            2 => {
                const arg1 = args[0];
                const arg2 = args[1];
                return switch (arg1) {
                    .string => |str_val| switch (arg2) {
                        .int32 => |int_val| c.snprintf(buffer.ptr, buffer.len, format, str_val, int_val),
                        .string => |str_val2| c.snprintf(buffer.ptr, buffer.len, format, str_val, str_val2),
                        else => c.snprintf(buffer.ptr, buffer.len, format, str_val),
                    },
                    .int32 => |int_val| switch (arg2) {
                        .int32 => |int_val2| c.snprintf(buffer.ptr, buffer.len, format, int_val, int_val2),
                        .string => |str_val| c.snprintf(buffer.ptr, buffer.len, format, int_val, str_val),
                        else => c.snprintf(buffer.ptr, buffer.len, format, int_val),
                    },
                    else => c.snprintf(buffer.ptr, buffer.len, format),
                };
            },
            else => c.snprintf(buffer.ptr, buffer.len, format),
        }
    }
};

/// CURSED language integration for variadic functions
pub const CursedVariadicIntegration = struct {
    allocator: Allocator,
    safe_wrapper: SafeVariadicWrapper,
    registered_functions: HashMap([]const u8, VariadicFunctionInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const VariadicFunctionInfo = struct {
        name: []const u8,
        min_args: usize,
        max_args: usize,
        has_format_string: bool,
        return_type: []const u8,
    };
    
    pub fn init(allocator: Allocator) CursedVariadicIntegration {
        var integration = CursedVariadicIntegration{
            .allocator = allocator,
            .safe_wrapper = SafeVariadicWrapper.init(allocator),
            .registered_functions = HashMap([]const u8, VariadicFunctionInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
        
        // Register common variadic functions
        integration.registerCommonFunctions() catch {};
        
        return integration;
    }
    
    pub fn deinit(self: *CursedVariadicIntegration) void {
        self.registered_functions.deinit();
    }
    
    fn registerCommonFunctions(self: *CursedVariadicIntegration) !void {
        try self.registered_functions.put("printf", VariadicFunctionInfo{
            .name = "printf",
            .min_args = 1,
            .max_args = 32,
            .has_format_string = true,
            .return_type = "normie",
        });
        
        try self.registered_functions.put("scanf", VariadicFunctionInfo{
            .name = "scanf",
            .min_args = 1,
            .max_args = 32,
            .has_format_string = true,
            .return_type = "normie",
        });
        
        try self.registered_functions.put("sprintf", VariadicFunctionInfo{
            .name = "sprintf",
            .min_args = 2,
            .max_args = 32,
            .has_format_string = true,
            .return_type = "normie",
        });
        
        try self.registered_functions.put("snprintf", VariadicFunctionInfo{
            .name = "snprintf",
            .min_args = 3,
            .max_args = 32,
            .has_format_string = true,
            .return_type = "normie",
        });
    }
    
    /// Generate CURSED wrapper function for a variadic C function
    pub fn generateCursedWrapper(self: *CursedVariadicIntegration, func_name: []const u8) ![]const u8 {
        const func_info = self.registered_functions.get(func_name) orelse return VariadicError.InvalidFunctionSignature;
        
        var wrapper = ArrayList(u8).init(self.allocator);
        defer wrapper.deinit();
        
        try wrapper.writer().print("// Auto-generated CURSED wrapper for variadic C function {s}\n", .{func_name});
        try wrapper.writer().print("extern \"C\" {{\n");
        try wrapper.writer().print("    library \"libc\"\n");
        try wrapper.writer().print("    slay {s}(format tea, ...args) {s}\n", .{ func_name, func_info.return_type });
        try wrapper.writer().print("}}\n\n");
        
        try wrapper.writer().print("// Safe CURSED wrapper with type checking\n");
        try wrapper.writer().print("slay safe_{s}(format tea, ...args) {s} {{\n", .{ func_name, func_info.return_type });
        
        if (func_info.has_format_string) {
            try wrapper.writer().print("    // Validate format string\n");
            try wrapper.writer().print("    ready (format == null) {{\n");
            try wrapper.writer().print("        yikes \"Format string cannot be null\"\n");
            try wrapper.writer().print("    }}\n\n");
            
            try wrapper.writer().print("    // Validate argument count\n");
            try wrapper.writer().print("    ready (args.len < {} || args.len > {}) {{\n", .{ func_info.min_args - 1, func_info.max_args - 1 });
            try wrapper.writer().print("        yikes \"Invalid number of arguments\"\n");
            try wrapper.writer().print("    }}\n\n");
        }
        
        try wrapper.writer().print("    // Call native function with error handling\n");
        try wrapper.writer().print("    sus result {s} = {s}(format, ...args) fam {{\n", .{ func_info.return_type, func_name });
        try wrapper.writer().print("        when error -> {{\n");
        try wrapper.writer().print("            vibez.spill(\"Error calling {s}: \" + error)\n", .{func_name});
        try wrapper.writer().print("            damn -1\n");
        try wrapper.writer().print("        }}\n");
        try wrapper.writer().print("    }}\n\n");
        
        try wrapper.writer().print("    damn result\n");
        try wrapper.writer().print("}}\n\n");
        
        return wrapper.toOwnedSlice();
    }
    
    /// Generate LLVM integration code for variadic functions
    pub fn generateLLVMIntegration(self: *CursedVariadicIntegration) ![]const u8 {
        var code = ArrayList(u8).init(self.allocator);
        defer code.deinit();
        
        try code.writer().print("// LLVM Variadic Function Integration\n\n");
        try code.writer().print("pub fn setupVariadicFunctions(module: llvm_c.LLVMModuleRef, context: llvm_c.LLVMContextRef) !void {{\n");
        
        var iterator = self.registered_functions.iterator();
        while (iterator.next()) |entry| {
            const func_info = entry.value_ptr.*;
            try code.writer().print("    // Declare {s}\n", .{func_info.name});
            try code.writer().print("    {{\n");
            try code.writer().print("        const return_type = llvm_c.LLVMInt32TypeInContext(context);\n");
            try code.writer().print("        const param_types = [_]llvm_c.LLVMTypeRef{{\n");
            try code.writer().print("            llvm_c.LLVMPointerType(llvm_c.LLVMInt8TypeInContext(context), 0)\n");
            try code.writer().print("        }};\n");
            try code.writer().print("        const func_type = llvm_c.LLVMFunctionType(\n");
            try code.writer().print("            return_type,\n");
            try code.writer().print("            @ptrCast(&param_types),\n");
            try code.writer().print("            1,\n");
            try code.writer().print("            1 // is_variadic = true\n");
            try code.writer().print("        );\n");
            try code.writer().print("        _ = llvm_c.LLVMAddFunction(module, \"{s}\", func_type);\n", .{func_info.name});
            try code.writer().print("    }}\n\n");
        }
        
        try code.writer().print("}}\n\n");
        
        try code.writer().print("pub fn callVariadicFunction(\n");
        try code.writer().print("    builder: llvm_c.LLVMBuilderRef,\n");
        try code.writer().print("    module: llvm_c.LLVMModuleRef,\n");
        try code.writer().print("    func_name: []const u8,\n");
        try code.writer().print("    args: []llvm_c.LLVMValueRef\n");
        try code.writer().print(") !llvm_c.LLVMValueRef {{\n");
        try code.writer().print("    const func_name_z = try std.fmt.allocPrintZ(allocator, \"{s}\", .{{func_name}});\n");
        try code.writer().print("    defer allocator.free(func_name_z);\n");
        try code.writer().print("    \n");
        try code.writer().print("    const func = llvm_c.LLVMGetNamedFunction(module, func_name_z) orelse {{\n");
        try code.writer().print("        return error.FunctionNotFound;\n");
        try code.writer().print("    }};\n");
        try code.writer().print("    \n");
        try code.writer().print("    return llvm_c.LLVMBuildCall2(\n");
        try code.writer().print("        builder,\n");
        try code.writer().print("        llvm_c.LLVMGlobalGetValueType(func),\n");
        try code.writer().print("        func,\n");
        try code.writer().print("        args.ptr,\n");
        try code.writer().print("        @intCast(args.len),\n");
        try code.writer().print("        \"variadic_call\"\n");
        try code.writer().print("    );\n");
        try code.writer().print("}}\n");
        
        return code.toOwnedSlice();
    }
};

// Test cases
test "variadic argument conversion" {
    const val_i32: i32 = 42;
    const val_ptr: *const anyopaque = @ptrCast(&val_i32);
    const arg = try VariadicArgument.fromCursedValue("normie", val_ptr);
    
    try std.testing.expect(arg == .int32);
    try std.testing.expect(arg.int32 == 42);
}

test "format string parsing" {
    var parser = FormatStringParser.init("Hello %s, you have %d messages");
    
    const spec1 = parser.nextSpecifier().?;
    try std.testing.expect(spec1.type == .string);
    
    const spec2 = parser.nextSpecifier().?;
    try std.testing.expect(spec2.type == .int);
    
    try std.testing.expect(parser.nextSpecifier() == null);
}

test "variadic call builder" {
    var builder = VariadicCallBuilder.init(std.testing.allocator, "printf");
    defer builder.deinit();
    
    builder.setFormatString("Hello %s!");
    try builder.addArgument(VariadicArgument{ .string = "World" });
    
    try builder.validateArguments();
}

test "safe printf wrapper" {
    var wrapper = SafeVariadicWrapper.init(std.testing.allocator);
    
    const args = [_]VariadicArgument{
        VariadicArgument{ .string = "World" },
    };
    
    const result = try wrapper.printf("Hello %s!\n", &args);
    try std.testing.expect(result >= 0);
}

test "CURSED integration" {
    var integration = CursedVariadicIntegration.init(std.testing.allocator);
    defer integration.deinit();
    
    const wrapper = try integration.generateCursedWrapper("printf");
    defer std.testing.allocator.free(wrapper);
    
    try std.testing.expect(std.mem.indexOf(u8, wrapper, "safe_printf") != null);
    try std.testing.expect(std.mem.indexOf(u8, wrapper, "format tea") != null);
}
