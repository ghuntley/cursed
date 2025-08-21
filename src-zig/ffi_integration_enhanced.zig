//! Enhanced FFI Integration with Variadic Function Support
//! Integrates the variadic function bridge with the existing CURSED FFI system

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const extern_abi = @import("extern_abi.zig");
const variadic_bridge = @import("variadic_ffi_bridge.zig");
const ast = @import("ast_advanced.zig");

// LLVM imports
const llvm_c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

/// Enhanced FFI manager with variadic function support
pub const EnhancedFFIManager = struct {
    allocator: Allocator,
    cabi_bridge: extern_abi.CABIBridge,
    variadic_integration: variadic_bridge.CursedVariadicIntegration,
    standard_functions: HashMap([]const u8, StandardFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const StandardFunction = struct {
        name: []const u8,
        is_variadic: bool,
        min_args: usize,
        max_args: usize,
        arg_types: []const []const u8,
        return_type: []const u8,
        library: []const u8,
        wrapper_generated: bool,
    };
    
    pub fn init() EnhancedFFIManager {
        var manager = EnhancedFFIManager{
            .allocator = allocator,
            .cabi_bridge = extern_abi.CABIBridge.init(allocator),
            .variadic_integration = variadic_bridge.CursedVariadicIntegration.init(allocator),
            .standard_functions = HashMap([]const u8, StandardFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
        
        // Initialize standard library functions
        manager.initializeStandardFunctions() catch {};
        
        return manager;
    }
    
    pub fn deinit(self: *EnhancedFFIManager) void {
        self.cabi_bridge.deinit();
        self.variadic_integration.deinit();
        self.standard_functions.deinit();
    }
    
    /// Initialize common C standard library functions
    fn initializeStandardFunctions(self: *EnhancedFFIManager) !void {
        // Variadic functions
        try self.standard_functions.put("printf", StandardFunction{
            .name = "printf",
            .is_variadic = true,
            .min_args = 1,
            .max_args = 32,
            .arg_types = &[_][]const u8{"tea"}, // Format string + varargs
            .return_type = "normie",
            .library = "libc",
            .wrapper_generated = false,
        });
        
        try self.standard_functions.put("fprintf", StandardFunction{
            .name = "fprintf",
            .is_variadic = true,
            .min_args = 2,
            .max_args = 32,
            .arg_types = &[_][]const u8{ "*vibes", "tea" }, // FILE*, format + varargs
            .return_type = "normie",
            .library = "libc",
            .wrapper_generated = false,
        });
        
        try self.standard_functions.put("sprintf", StandardFunction{
            .name = "sprintf",
            .is_variadic = true,
            .min_args = 2,
            .max_args = 32,
            .arg_types = &[_][]const u8{ "tea", "tea" }, // buffer, format + varargs
            .return_type = "normie",
            .library = "libc",
            .wrapper_generated = false,
        });
        
        try self.standard_functions.put("snprintf", StandardFunction{
            .name = "snprintf",
            .is_variadic = true,
            .min_args = 3,
            .max_args = 32,
            .arg_types = &[_][]const u8{ "tea", "normie", "tea" }, // buffer, size, format + varargs
            .return_type = "normie",
            .library = "libc",
            .wrapper_generated = false,
        });
        
        try self.standard_functions.put("scanf", StandardFunction{
            .name = "scanf",
            .is_variadic = true,
            .min_args = 1,
            .max_args = 32,
            .arg_types = &[_][]const u8{"tea"}, // Format string + varargs
            .return_type = "normie",
            .library = "libc",
            .wrapper_generated = false,
        });
        
        try self.standard_functions.put("sscanf", StandardFunction{
            .name = "sscanf",
            .is_variadic = true,
            .min_args = 2,
            .max_args = 32,
            .arg_types = &[_][]const u8{ "tea", "tea" }, // input, format + varargs
            .return_type = "normie",
            .library = "libc",
            .wrapper_generated = false,
        });
        
        // Non-variadic functions for comparison
        try self.standard_functions.put("strlen", StandardFunction{
            .name = "strlen",
            .is_variadic = false,
            .min_args = 1,
            .max_args = 1,
            .arg_types = &[_][]const u8{"tea"},
            .return_type = "normie",
            .library = "libc",
            .wrapper_generated = false,
        });
        
        try self.standard_functions.put("strcmp", StandardFunction{
            .name = "strcmp",
            .is_variadic = false,
            .min_args = 2,
            .max_args = 2,
            .arg_types = &[_][]const u8{ "tea", "tea" },
            .return_type = "normie",
            .library = "libc",
            .wrapper_generated = false,
        });
        
        try self.standard_functions.put("malloc", StandardFunction{
            .name = "malloc",
            .is_variadic = false,
            .min_args = 1,
            .max_args = 1,
            .arg_types = &[_][]const u8{"normie"},
            .return_type = "*vibes",
            .library = "libc",
            .wrapper_generated = false,
        });
        
        try self.standard_functions.put("free", StandardFunction{
            .name = "free",
            .is_variadic = false,
            .min_args = 1,
            .max_args = 1,
            .arg_types = &[_][]const u8{"*vibes"},
            .return_type = "vibes",
            .library = "libc",
            .wrapper_generated = false,
        });
    }
    
    /// Generate comprehensive FFI module for CURSED
    pub fn generateFFIModule(self: *EnhancedFFIManager) ![]const u8 {
        var module = .empty;
        defer module.deinit();
        
        try module.writer().print("//! Auto-generated CURSED FFI Module with Variadic Support\n", .{});
        try module.writer().print("//! Provides safe wrappers for C standard library functions\n\n", .{});
        
        // Generate extern declarations
        try module.writer().print("extern \"C\" {{\n");
        try module.writer().print("    library \"libc\"\n\n");
        
        var iterator = self.standard_functions.iterator();
        while (iterator.next()) |entry| {
            const func = entry.value_ptr.*;
            
            if (func.is_variadic) {
                try module.writer().print("    // Variadic function: {s}\n", .{func.name});
                try module.writer().print("    slay {s}(", .{func.name});
                
                // Fixed parameters
                for (func.arg_types, 0..) |arg_type, i| {
                    if (i > 0) try module.writer().print(", ", .{});
                    try module.writer().print("arg{} {s}", .{ i, arg_type });
                }
                
                try module.writer().print(", ...varargs) {s}\n\n", .{func.return_type});
            } else {
                try module.writer().print("    // Regular function: {s}\n", .{func.name});
                try module.writer().print("    slay {s}(", .{func.name});
                
                for (func.arg_types, 0..) |arg_type, i| {
                    if (i > 0) try module.writer().print(", ", .{});
                    try module.writer().print("arg{} {s}", .{ i, arg_type });
                }
                
                try module.writer().print(") {s}\n\n", .{func.return_type});
            }
        }
        
        try module.writer().print("}}\n\n", .{});
        
        // Generate safe wrappers
        try module.writer().print("// Safe wrapper functions with error handling\n\n", .{});
        
        iterator = self.standard_functions.iterator();
        while (iterator.next()) |entry| {
            const func = entry.value_ptr.*;
            
            if (func.is_variadic) {
                const wrapper = try self.variadic_integration.generateCursedWrapper(func.name);
                defer self.allocator.free(wrapper);
                try module.appendSlice(wrapper);
            } else {
                try self.generateRegularWrapper(&module, func);
            }
        }
        
        // Generate utility functions
        try self.generateUtilityFunctions(&module);
        
        return module.toOwnedSlice(self.allocator);
    }
    
    /// Generate wrapper for non-variadic function
    fn generateRegularWrapper(self: *EnhancedFFIManager, module: *ArrayList(u8), func: StandardFunction) !void {
        _ = self;
        try module.writer().print("// Safe wrapper for {s}\n", .{func.name});
        try module.writer().print("slay safe_{s}(", .{func.name});
        
        for (func.arg_types, 0..) |arg_type, i| {
            if (i > 0) try module.writer().print(", ", .{});
            try module.writer().print("arg{} {s}", .{ i, arg_type });
        }
        
        try module.writer().print(") {s} {{\n", .{func.return_type});
        
        // Add basic validation
        for (func.arg_types, 0..) |arg_type, i| {
            if (std.mem.eql(u8, arg_type, "tea") or std.mem.eql(u8, arg_type, "*vibes")) {
                try module.writer().print("    ready (arg{} == null) {{\n", .{i});
                try module.writer().print("        yikes \"Argument {} cannot be null\"\n", .{i});
                try module.writer().print("    }}\n", .{});
            }
        }
        
        try module.writer().print("\n    // Call native function\n", .{});
        
        if (!std.mem.eql(u8, func.return_type, "vibes")) {
            try module.writer().print("    sus result {s} = ", .{func.return_type});
        } else {
            try module.writer().print("    ", .{});
        }
        
        try module.writer().print("{s}(", .{func.name});
        for (func.arg_types, 0..) |_, i| {
            if (i > 0) try module.writer().print(", ", .{});
            try module.writer().print("arg{}", .{i});
        }
        try module.writer().print(")\n", .{});
        
        if (!std.mem.eql(u8, func.return_type, "vibes")) {
            try module.writer().print("    damn result\n", .{});
        }
        
        try module.writer().print("}}\n\n", .{});
    }
    
    /// Generate utility functions for FFI
    fn generateUtilityFunctions(self: *EnhancedFFIManager, module: *ArrayList(u8)) !void {
        _ = self;
        try module.writer().print("// Utility functions for FFI operations\n\n", .{});
        
        // String conversion utilities
        try module.writer().print("// Convert CURSED string to C string (null-terminated)\n", .{});
        try module.writer().print("slay cursed_to_c_string(str tea) tea {{\n", .{});
        try module.writer().print("    ready (str == null) {{ damn null }}\n", .{});
        try module.writer().print("    // Ensure null termination\n", .{});
        try module.writer().print("    sus len normie = string_length(str)\n", .{});
        try module.writer().print("    sus c_str tea = allocate_string(len + 1)\n", .{});
        try module.writer().print("    copy_string(c_str, str, len)\n", .{});
        try module.writer().print("    set_char_at(c_str, len, 0)  // Null terminator\n", .{});
        try module.writer().print("    damn c_str\n", .{});
        try module.writer().print("}}\n\n", .{});
        
        // Buffer management
        try module.writer().print("// Safe buffer allocation with size tracking\n", .{});
        try module.writer().print("slay allocate_safe_buffer(size normie) *vibes {{\n", .{});
        try module.writer().print("    ready (size <= 0 || size > 1048576) {{  // 1MB limit\n", .{});
        try module.writer().print("        yikes \"Invalid buffer size\"\n");
        try module.writer().print("    }}\n", .{});
        try module.writer().print("    sus buffer *vibes = safe_malloc(size)\n", .{});
        try module.writer().print("    ready (buffer == null) {{\n", .{});
        try module.writer().print("        yikes \"Memory allocation failed\"\n");
        try module.writer().print("    }}\n", .{});
        try module.writer().print("    zero_memory(buffer, size)\n", .{});
        try module.writer().print("    damn buffer\n", .{});
        try module.writer().print("}}\n\n", .{});
        
        // Error handling utilities
        try module.writer().print("// Get last FFI error message\n", .{});
        try module.writer().print("slay get_ffi_error() tea {{\n", .{});
        try module.writer().print("    damn get_last_error_message()\n", .{});
        try module.writer().print("}}\n\n", .{});
        
        // Type validation
        try module.writer().print("// Validate variadic argument types\n", .{});
        try module.writer().print("slay validate_varargs(format tea, args []vibes) lit {{\n", .{});
        try module.writer().print("    ready (format == null) {{ damn cringe }}\n", .{});
        try module.writer().print("    \n", .{});
        try module.writer().print("    sus expected_count normie = count_format_specifiers(format)\n", .{});
        try module.writer().print("    ready (args.len != expected_count) {{ damn cringe }}\n", .{});
        try module.writer().print("    \n", .{});
        try module.writer().print("    damn based\n", .{});
        try module.writer().print("}}\n\n", .{});
    }
    
    /// Generate LLVM backend integration
    pub fn generateLLVMIntegration(self: *EnhancedFFIManager) ![]const u8 {
        var code = .empty;
        defer code.deinit();
        
        try code.writer().print("//! LLVM Backend Integration for Enhanced FFI\n\n", .{});
        try code.writer().print("const std = @import(\"std\");\n");
        try code.writer().print("const llvm_c = @cImport({{\n", .{});
        try code.writer().print("    @cInclude(\"llvm-c/Core.h\");\n");
        try code.writer().print("    @cInclude(\"llvm-c/Target.h\");\n");
        try code.writer().print("}})\n\n", .{});
        
        try code.writer().print("pub const EnhancedFFIBackend = struct {{\n", .{});
        try code.writer().print("    context: llvm_c.LLVMContextRef,\n", .{});
        try code.writer().print("    module: llvm_c.LLVMModuleRef,\n", .{});
        try code.writer().print("    builder: llvm_c.LLVMBuilderRef,\n", .{});
        try code.writer().print("    declared_functions: std.HashMap([]const u8, llvm_c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),\n\n", .{});
        
        try code.writer().print("    pub fn init(allocator: std.mem.Allocator, context: llvm_c.LLVMContextRef, module: llvm_c.LLVMModuleRef, builder: llvm_c.LLVMBuilderRef) EnhancedFFIBackend {{\n", .{});
        try code.writer().print("        return EnhancedFFIBackend{{\n", .{});
        try code.writer().print("            .context = context,\n", .{});
        try code.writer().print("            .module = module,\n", .{});
        try code.writer().print("            .builder = builder,\n", .{});
        try code.writer().print("            .declared_functions = std.HashMap([]const u8, llvm_c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),\n", .{});
        try code.writer().print("        }};\n", .{});
        try code.writer().print("    }}\n\n", .{});
        
        try code.writer().print("    pub fn deinit(self: *EnhancedFFIBackend) void {{\n", .{});
        try code.writer().print("        self.declared_functions.deinit();\n", .{});
        try code.writer().print("    }}\n\n", .{});
        
        // Generate function declaration methods
        try code.writer().print("    /// Declare all standard C library functions\n", .{});
        try code.writer().print("    pub fn declareStandardFunctions(self: *EnhancedFFIBackend) !void {{\n", .{});
        
        var iterator = self.standard_functions.iterator();
        while (iterator.next()) |entry| {
            const func = entry.value_ptr.*;
            try code.writer().print("        try self.declare{s}Function();\n", .{capitalizeFirst(func.name)});
        }
        
        try code.writer().print("    }}\n\n", .{});
        
        // Generate individual declaration methods
        iterator = self.standard_functions.iterator();
        while (iterator.next()) |entry| {
            const func = entry.value_ptr.*;
            try self.generateLLVMFunctionDeclaration(&code, func);
        }
        
        // Generate call methods
        try code.writer().print("    /// Generate call to variadic function\n", .{});
        try code.writer().print("    pub fn callVariadicFunction(self: *EnhancedFFIBackend, func_name: []const u8, args: []llvm_c.LLVMValueRef) !llvm_c.LLVMValueRef {{\n", .{});
        try code.writer().print("        const func = self.declared_functions.get(func_name) orelse return error.FunctionNotDeclared;\n", .{});
        try code.writer().print("        \n", .{});
        try code.writer().print("        return llvm_c.LLVMBuildCall2(\n", .{});
        try code.writer().print("            self.builder,\n", .{});
        try code.writer().print("            llvm_c.LLVMGlobalGetValueType(func),\n", .{});
        try code.writer().print("            func,\n", .{});
        try code.writer().print("            args.ptr,\n", .{});
        try code.writer().print("            @intCast(args.len),\n", .{});
        try code.writer().print("            \"variadic_call\"\n");
        try code.writer().print("        );\n", .{});
        try code.writer().print("    }}\n", .{});
        
        try code.writer().print("}};\n", .{});
        
        return code.toOwnedSlice();
    }
    
    /// Generate LLVM function declaration
    fn generateLLVMFunctionDeclaration(self: *EnhancedFFIManager, code: *ArrayList(u8), func: StandardFunction) !void {
        _ = self;
        try code.writer().print("    fn declare{s}Function(self: *EnhancedFFIBackend) !void {{\n", .{capitalizeFirst(func.name)});
        
        // Generate return type
        try code.writer().print("        const return_type = ", .{});
        try code.writer().print("{s};\n", .{self.cursedTypeToLLVMType(func.return_type)});
        
        // Generate parameter types
        try code.writer().print("        const param_types = [_]llvm_c.LLVMTypeRef{{\n", .{});
        for (func.arg_types) |arg_type| {
            try code.writer().print("            {s},\n", .{self.cursedTypeToLLVMType(arg_type)});
        }
        try code.writer().print("        }};\n", .{});
        
        // Create function type
        try code.writer().print("        const func_type = llvm_c.LLVMFunctionType(\n", .{});
        try code.writer().print("            return_type,\n", .{});
        try code.writer().print("            @ptrCast(&param_types),\n", .{});
        try code.writer().print("            {},\n", .{func.arg_types.len});
        try code.writer().print("            {} // is_variadic\n", .{if (func.is_variadic) 1 else 0});
        try code.writer().print("        );\n", .{});
        
        // Add function to module
        try code.writer().print("        const func = llvm_c.LLVMAddFunction(self.module, \"{s}\", func_type);\n", .{func.name});
        try code.writer().print("        try self.declared_functions.put(\"{s}\", func);\n", .{func.name});
        
        try code.writer().print("    }}\n\n", .{});
    }
    
    /// Convert CURSED type to LLVM type expression
    fn cursedTypeToLLVMType(self: *EnhancedFFIManager, cursed_type: []const u8) []const u8 {
        _ = self;
        if (std.mem.eql(u8, cursed_type, "vibes")) {
            return "llvm_c.LLVMVoidTypeInContext(self.context)";
        } else if (std.mem.eql(u8, cursed_type, "smol")) {
            return "llvm_c.LLVMInt8TypeInContext(self.context)";
        } else if (std.mem.eql(u8, cursed_type, "normie")) {
            return "llvm_c.LLVMInt32TypeInContext(self.context)";
        } else if (std.mem.eql(u8, cursed_type, "drip")) {
            return "llvm_c.LLVMInt64TypeInContext(self.context)";
        } else if (std.mem.eql(u8, cursed_type, "snack")) {
            return "llvm_c.LLVMFloatTypeInContext(self.context)";
        } else if (std.mem.eql(u8, cursed_type, "meal")) {
            return "llvm_c.LLVMDoubleTypeInContext(self.context)";
        } else if (std.mem.eql(u8, cursed_type, "tea")) {
            return "llvm_c.LLVMPointerType(llvm_c.LLVMInt8TypeInContext(self.context), 0)";
        } else if (std.mem.eql(u8, cursed_type, "*vibes")) {
            return "llvm_c.LLVMPointerType(llvm_c.LLVMInt8TypeInContext(self.context), 0)";
        } else if (std.mem.eql(u8, cursed_type, "lit")) {
            return "llvm_c.LLVMInt1TypeInContext(self.context)";
        } else {
            return "llvm_c.LLVMPointerType(llvm_c.LLVMInt8TypeInContext(self.context), 0)"; // Default to pointer
        }
    }
    
    /// Capitalize first letter of string
    fn capitalizeFirst(input: []const u8) []const u8 {
        if (input.len == 0) return input;
        
        // This is a simplified version - in practice you'd want proper memory management
        var result = std.ArrayList(u8).init(self.allocator);
        result.append(std.ascii.toUpper(input[0])) catch return input;
        result.appendSlice(input[1..]) catch return input;
        return result.toOwnedSlice() catch input;
    }
    
    /// Check if function is variadic
    pub fn isVariadicFunction(self: *EnhancedFFIManager, func_name: []const u8) bool {
        if (self.standard_functions.get(func_name)) |func| {
            return func.is_variadic;
        }
        return false;
    }
    
    /// Get function info
    pub fn getFunctionInfo(self: *EnhancedFFIManager, func_name: []const u8) ?StandardFunction {
        return self.standard_functions.get(func_name);
    }
    
    /// Validate function call arguments
    pub fn validateFunctionCall(self: *EnhancedFFIManager, func_name: []const u8, arg_count: usize) bool {
        if (self.standard_functions.get(func_name)) |func| {
            return arg_count >= func.min_args and arg_count <= func.max_args;
        }
        return false;
    }
};

// Test integration
test "enhanced FFI manager initialization" {
    var manager = EnhancedFFIManager.init(std.testing.allocator);
    defer manager.deinit();
    
    // Test that standard functions are registered
    try std.testing.expect(manager.isVariadicFunction("printf"));
    try std.testing.expect(manager.isVariadicFunction("sprintf"));
    try std.testing.expect(!manager.isVariadicFunction("strlen"));
    
    // Test function validation
    try std.testing.expect(manager.validateFunctionCall("printf", 1));
    try std.testing.expect(manager.validateFunctionCall("printf", 5));
    try std.testing.expect(!manager.validateFunctionCall("printf", 0));
    
    try std.testing.expect(manager.validateFunctionCall("strlen", 1));
    try std.testing.expect(!manager.validateFunctionCall("strlen", 2));
}

test "FFI module generation" {
    var manager = EnhancedFFIManager.init(std.testing.allocator);
    defer manager.deinit();
    
    const module = try manager.generateFFIModule();
    defer std.testing.allocator.free(module);
    
    // Check that module contains expected elements
    try std.testing.expect(std.mem.indexOf(u8, module, "extern \"C\"") != null);
    try std.testing.expect(std.mem.indexOf(u8, module, "safe_printf") != null);
    try std.testing.expect(std.mem.indexOf(u8, module, "safe_strlen") != null);
    try std.testing.expect(std.mem.indexOf(u8, module, "...varargs") != null);
}

test "LLVM integration generation" {
    var manager = EnhancedFFIManager.init(std.testing.allocator);
    defer manager.deinit();
    
    const llvm_code = try manager.generateLLVMIntegration();
    defer std.testing.allocator.free(llvm_code);
    
    // Check that LLVM code contains expected elements
    try std.testing.expect(std.mem.indexOf(u8, llvm_code, "EnhancedFFIBackend") != null);
    try std.testing.expect(std.mem.indexOf(u8, llvm_code, "declareStandardFunctions") != null);
    try std.testing.expect(std.mem.indexOf(u8, llvm_code, "callVariadicFunction") != null);
}
