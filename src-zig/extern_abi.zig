//! P33: Simplified C ABI Support for Extern Functions
//! Provides easy extern "C" function declarations and calling

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast_advanced.zig");

// Simple C bindings (fallback when LLVM not available)
const c = struct {
    // Function pointer types for common C signatures
    pub const VoidFunc = *const fn () callconv(.C) void;
    pub const IntFunc = *const fn () callconv(.C) i32;
    pub const FloatFunc = *const fn () callconv(.C) f64;
    pub const StringFunc = *const fn () callconv(.C) [*:0]const u8;
    pub const IntIntFunc = *const fn (i32) callconv(.C) i32;
    pub const FloatFloatFunc = *const fn (f64) callconv(.C) f64;
    pub const StringStringFunc = *const fn ([*:0]const u8) callconv(.C) [*:0]const u8;
    
    // Library loading functions (platform-specific)
    pub fn loadLibrary(name: [*:0]const u8) ?*anyopaque {
        if (@import("builtin").os.tag == .windows) {
            // Windows LoadLibrary
            return null; // Stub
        } else {
            // Unix dlopen
            return null; // Stub
        }
    }
    
    pub fn getSymbol(lib: *anyopaque, name: [*:0]const u8) ?*anyopaque {
        _ = lib;
        _ = name;
        // Platform-specific symbol lookup
        return null; // Stub
    }
    
    pub fn closeLibrary(lib: *anyopaque) void {
        _ = lib;
        // Platform-specific cleanup
    }
};

/// C ABI function signature
pub const CABISignature = struct {
    name: []const u8,
    return_type: CABIType,
    parameters: ArrayList(CABIParameter),
    calling_convention: CallingConvention,
    
    pub const CABIType = enum {
        Void,
        Int8,
        Int16,
        Int32,
        Int64,
        UInt8,
        UInt16,
        UInt32,
        UInt64,
        Float32,
        Float64,
        Pointer,
        String,
        
        pub fn toZigType(self: CABIType) []const u8 {
            return switch (self) {
                .Void => "void",
                .Int8 => "i8",
                .Int16 => "i16",
                .Int32 => "i32",
                .Int64 => "i64",
                .UInt8 => "u8",
                .UInt16 => "u16",
                .UInt32 => "u32",
                .UInt64 => "u64",
                .Float32 => "f32",
                .Float64 => "f64",
                .Pointer => "*anyopaque",
                .String => "[*:0]const u8",
            };
        }
        
        pub fn toCType(self: CABIType) []const u8 {
            return switch (self) {
                .Void => "void",
                .Int8 => "char",
                .Int16 => "short",
                .Int32 => "int",
                .Int64 => "long long",
                .UInt8 => "unsigned char",
                .UInt16 => "unsigned short",
                .UInt32 => "unsigned int",
                .UInt64 => "unsigned long long",
                .Float32 => "float",
                .Float64 => "double",
                .Pointer => "void*",
                .String => "const char*",
            };
        }
    };
    
    pub const CABIParameter = struct {
        name: []const u8,
        param_type: CABIType,
    };
    
    pub const CallingConvention = enum {
        C,
        Stdcall,
        Fastcall,
        
        pub fn toZigConvention(self: CallingConvention) []const u8 {
            return switch (self) {
                .C => ".C",
                .Stdcall => ".Stdcall",
                .Fastcall => ".Fastcall",
            };
        }
    };
    
    pub fn init(allocator: Allocator, name: []const u8) CABISignature {
        return CABISignature{
            .name = name,
            .return_type = .Void,
            .parameters = ArrayList(CABIParameter).init(allocator),
            .calling_convention = .C,
        };
    }
    
    pub fn deinit(self: *CABISignature) void {
        self.parameters.deinit();
    }
};

/// External library interface
pub const ExternLibrary = struct {
    name: []const u8,
    handle: ?*anyopaque,
    functions: HashMap([]const u8, ExternFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    const ExternFunction = struct {
        signature: CABISignature,
        function_ptr: ?*anyopaque,
        wrapper_generated: bool,
    };
    
    pub fn init(allocator: Allocator, name: []const u8) ExternLibrary {
        return ExternLibrary{
            .name = name,
            .handle = null,
            .functions = HashMap([]const u8, ExternFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ExternLibrary) void {
        var iterator = self.functions.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.signature.deinit();
        }
        self.functions.deinit();
        
        if (self.handle) |handle| {
            c.closeLibrary(handle);
        }
    }
    
    /// Load the external library
    pub fn load(self: *ExternLibrary) !void {
        const lib_name_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{self.name});
        defer self.allocator.free(lib_name_z);
        
        self.handle = c.loadLibrary(lib_name_z) orelse {
            return error.LibraryNotFound;
        };
    }
    
    /// Declare an extern function
    pub fn declareFunction(self: *ExternLibrary, signature: CABISignature) !void {
        const function = ExternFunction{
            .signature = signature,
            .function_ptr = null,
            .wrapper_generated = false,
        };
        
        try self.functions.put(try self.allocator.dupe(u8, signature.name), function);
    }
    
    /// Resolve function pointer from library
    pub fn resolveFunction(self: *ExternLibrary, func_name: []const u8) !*anyopaque {
        if (self.handle == null) {
            try self.load();
        }
        
        const func_name_z = try std.fmt.allocPrintZ(self.allocator, "{s}", .{func_name});
        defer self.allocator.free(func_name_z);
        
        const func_ptr = c.getSymbol(self.handle.?, func_name_z) orelse {
            return error.FunctionNotFound;
        };
        
        // Update function entry
        if (self.functions.getPtr(func_name)) |function| {
            function.function_ptr = func_ptr;
        }
        
        return func_ptr;
    }
};

/// C ABI bridge for CURSED functions
pub const CABIBridge = struct {
    allocator: Allocator,
    libraries: HashMap([]const u8, ExternLibrary, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    type_mappings: HashMap([]const u8, CABISignature.CABIType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    generated_wrappers: ArrayList([]const u8),
    
    pub fn init(allocator: Allocator) CABIBridge {
        var bridge = CABIBridge{
            .allocator = allocator,
            .libraries = HashMap([]const u8, ExternLibrary, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .type_mappings = HashMap([]const u8, CABISignature.CABIType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .generated_wrappers = ArrayList([]const u8).init(allocator),
        };
        
        // Initialize default type mappings
        bridge.initializeTypeMappings() catch {};
        
        return bridge;
    }
    
    pub fn deinit(self: *CABIBridge) void {
        var lib_iterator = self.libraries.iterator();
        while (lib_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.libraries.deinit();
        
        self.type_mappings.deinit();
        
        for (self.generated_wrappers.items) |wrapper| {
            self.allocator.free(wrapper);
        }
        self.generated_wrappers.deinit();
    }
    
    /// Initialize CURSED to C type mappings
    fn initializeTypeMappings(self: *CABIBridge) !void {
        try self.type_mappings.put("lit", .Int8);        // boolean -> char
        try self.type_mappings.put("smol", .Int8);       // small int -> char
        try self.type_mappings.put("normie", .Int32);    // normal int -> int
        try self.type_mappings.put("drip", .Int64);      // big int -> long long
        try self.type_mappings.put("thicc", .Int64);     // huge int -> long long
        try self.type_mappings.put("snack", .Float32);   // small float -> float
        try self.type_mappings.put("meal", .Float64);    // big float -> double
        try self.type_mappings.put("tea", .String);      // string -> const char*
        try self.type_mappings.put("vibes", .Void);      // void -> void
    }
    
    /// Parse extern function declaration
    pub fn parseExternDeclaration(self: *CABIBridge, decl_text: []const u8) !CABISignature {
        var signature = CABISignature.init(self.allocator, "unknown");
        
        // Simple parser for extern declarations like:
        // extern "C" int add(int a, int b);
        
        var tokens = std.mem.tokenize(u8, decl_text, " \t\n();,");
        
        // Skip "extern" and "C"
        _ = tokens.next(); // extern
        const abi = tokens.next() orelse return error.InvalidDeclaration;
        
        if (std.mem.eql(u8, abi, "\"C\"")) {
            signature.calling_convention = .C;
        } else if (std.mem.eql(u8, abi, "\"stdcall\"")) {
            signature.calling_convention = .Stdcall;
        }
        
        // Parse return type
        const return_type_str = tokens.next() orelse return error.InvalidDeclaration;
        signature.return_type = self.parseCType(return_type_str);
        
        // Parse function name
        const func_name = tokens.next() orelse return error.InvalidDeclaration;
        signature.name = try self.allocator.dupe(u8, func_name);
        
        // Parse parameters
        while (tokens.next()) |param_type| {
            const param_name = tokens.next() orelse break;
            
            try signature.parameters.append(CABISignature.CABIParameter{
                .name = try self.allocator.dupe(u8, param_name),
                .param_type = self.parseCType(param_type),
            });
        }
        
        return signature;
    }
    
    /// Parse C type string to CABIType
    fn parseCType(self: *CABIBridge, type_str: []const u8) CABISignature.CABIType {
        _ = self;
        
        if (std.mem.eql(u8, type_str, "void")) return .Void;
        if (std.mem.eql(u8, type_str, "char")) return .Int8;
        if (std.mem.eql(u8, type_str, "short")) return .Int16;
        if (std.mem.eql(u8, type_str, "int")) return .Int32;
        if (std.mem.eql(u8, type_str, "long")) return .Int64;
        if (std.mem.eql(u8, type_str, "float")) return .Float32;
        if (std.mem.eql(u8, type_str, "double")) return .Float64;
        if (std.mem.indexOf(u8, type_str, "*") != null) return .Pointer;
        if (std.mem.indexOf(u8, type_str, "char*") != null) return .String;
        
        return .Int32; // Default fallback
    }
    
    /// Generate CURSED wrapper for extern function
    pub fn generateWrapper(self: *CABIBridge, signature: CABISignature, library_name: []const u8) ![]const u8 {
        var wrapper = ArrayList(u8).init(self.allocator);
        defer wrapper.deinit();
        
        try wrapper.writer().print("// Auto-generated wrapper for extern function {s}\n", .{signature.name});
        try wrapper.writer().print("slay {s}(", .{signature.name});
        
        // Generate parameters
        for (signature.parameters.items, 0..) |param, i| {
            if (i > 0) try wrapper.writer().print(", ");
            try wrapper.writer().print("{s} {s}", .{ self.cTypeToCA(param.param_type), param.name });
        }
        
        try wrapper.writer().print(") {s} {{\n", .{self.cTypeToCAbbreviationType(signature.return_type)});
        
        // Generate FFI call
        try wrapper.writer().print("    // FFI call to {s}.{s}\n", .{ library_name, signature.name });
        
        if (signature.return_type != .Void) {
            try wrapper.writer().print("    sus result {s} = ", .{self.cTypeToCAType(signature.return_type)});
        } else {
            try wrapper.writer().print("    ");
        }
        
        try wrapper.writer().print("cursed_ffi_call(\"{s}\", \"{s}\"", .{ library_name, signature.name });
        
        for (signature.parameters.items) |param| {
            try wrapper.writer().print(", {s}", .{param.name});
        }
        
        try wrapper.writer().print(")\n");
        
        if (signature.return_type != .Void) {
            try wrapper.writer().print("    damn result\n");
        }
        
        try wrapper.writer().print("}}\n\n");
        
        const result = try wrapper.toOwnedSlice();
        try self.generated_wrappers.append(result);
        return result;
    }
    
    /// Convert C type to CURSED type
    fn cTypeToCAType(self: *CABIBridge, c_type: CABISignature.CABIType) []const u8 {
        _ = self;
        return switch (c_type) {
            .Void => "vibes",
            .Int8 => "smol",
            .Int16 => "smol",
            .Int32 => "normie",
            .Int64 => "drip",
            .UInt8 => "smol",
            .UInt16 => "smol", 
            .UInt32 => "normie",
            .UInt64 => "drip",
            .Float32 => "snack",
            .Float64 => "meal",
            .Pointer => "*vibes",
            .String => "tea",
        };
    }
    
    /// Get CURSED type abbreviation
    fn cTypeToCAType(self: *CABIBridge, c_type: CABISignature.CABIType) []const u8 {
        return self.cTypeToCAType(c_type);
    }
    
    /// Register external library
    pub fn registerLibrary(self: *CABIBridge, library_name: []const u8) !*ExternLibrary {
        var library = ExternLibrary.init(self.allocator, library_name);
        try self.libraries.put(try self.allocator.dupe(u8, library_name), library);
        return self.libraries.getPtr(library_name).?;
    }
    
    /// Generate C header for CURSED functions
    pub fn generateCHeader(self: *CABIBridge, cursed_functions: []const ast.FunctionStatement) ![]const u8 {
        var header = ArrayList(u8).init(self.allocator);
        defer header.deinit();
        
        try header.writer().print("#ifndef CURSED_C_BINDINGS_H\n");
        try header.writer().print("#define CURSED_C_BINDINGS_H\n\n");
        try header.writer().print("#ifdef __cplusplus\n");
        try header.writer().print("extern \"C\" {\n");
        try header.writer().print("#endif\n\n");
        
        for (cursed_functions) |func| {
            // Generate C function declaration
            const c_return_type = self.cursedTypeToCType(func.return_type);
            try header.writer().print("{s} cursed_{s}(", .{ c_return_type, func.name });
            
            for (func.parameters.items, 0..) |param, i| {
                if (i > 0) try header.writer().print(", ");
                const c_param_type = self.cursedTypeToCType(param.param_type);
                try header.writer().print("{s} {s}", .{ c_param_type, param.name });
            }
            
            try header.writer().print(");\n");
        }
        
        try header.writer().print("\n#ifdef __cplusplus\n");
        try header.writer().print("}\n");
        try header.writer().print("#endif\n\n");
        try header.writer().print("#endif // CURSED_C_BINDINGS_H\n");
        
        return header.toOwnedSlice();
    }
    
    /// Convert CURSED type to C type
    fn cursedTypeToCType(self: *CABIBridge, cursed_type: ?ast.Type) []const u8 {
        if (cursed_type == null) return "void";
        
        return switch (cursed_type.?) {
            .Primitive => |primitive| switch (primitive) {
                .Lit => "int",
                .Smol => "char",
                .Normie => "int",
                .Drip => "long long",
                .Thicc => "long long",
                .Snack => "float",
                .Meal => "double",
                .Tea => "const char*",
                .Vibes => "void",
            },
            .Pointer => "void*",
            .Array => "void*", // Arrays passed as pointers
            .Slice => "void*",
            else => "void*",
        };
    }
    
    /// Generate FFI call implementation
    pub fn generateFFIRuntime(self: *CABIBridge) ![]const u8 {
        var runtime = ArrayList(u8).init(self.allocator);
        defer runtime.deinit();
        
        try runtime.writer().print("// Auto-generated FFI runtime for C interop\n\n");
        try runtime.writer().print("yeet \"cursed_ffi_internal\"\n\n");
        
        try runtime.writer().print("// FFI call dispatcher\n");
        try runtime.writer().print("slay cursed_ffi_call(library_name tea, function_name tea, ...args) tea {{\n");
        try runtime.writer().print("    // Load library if not already loaded\n");
        try runtime.writer().print("    sus lib_handle vibes = load_dynamic_library(library_name)\n");
        try runtime.writer().print("    lowkey lib_handle == null {{\n");
        try runtime.writer().print("        damn \"ERROR: Could not load library \" + library_name\n");
        try runtime.writer().print("    }}\n\n");
        
        try runtime.writer().print("    // Get function pointer\n");
        try runtime.writer().print("    sus func_ptr vibes = get_function_symbol(lib_handle, function_name)\n");
        try runtime.writer().print("    lowkey func_ptr == null {{\n");
        try runtime.writer().print("        damn \"ERROR: Could not find function \" + function_name\n");
        try runtime.writer().print("    }}\n\n");
        
        try runtime.writer().print("    // Call function with arguments\n");
        try runtime.writer().print("    damn call_c_function(func_ptr, args)\n");
        try runtime.writer().print("}}\n\n");
        
        return runtime.toOwnedSlice();
    }
};

/// Simple extern declaration parser
pub const ExternParser = struct {
    allocator: Allocator,
    bridge: *CABIBridge,
    
    pub fn init(allocator: Allocator, bridge: *CABIBridge) ExternParser {
        return ExternParser{
            .allocator = allocator,
            .bridge = bridge,
        };
    }
    
    /// Parse extern block
    pub fn parseExternBlock(self: *ExternParser, block_text: []const u8) !void {
        var lines = std.mem.split(u8, block_text, "\n");
        
        var current_library: ?[]const u8 = null;
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t");
            if (trimmed.len == 0 or trimmed[0] == '/') continue; // Skip empty lines and comments
            
            if (std.mem.startsWith(u8, trimmed, "library")) {
                // Parse library declaration: library "libname"
                current_library = try self.parseLibraryDeclaration(trimmed);
            } else if (std.mem.startsWith(u8, trimmed, "extern")) {
                // Parse extern function: extern "C" int func(int x);
                if (current_library) |lib_name| {
                    const signature = try self.bridge.parseExternDeclaration(trimmed);
                    var library = self.bridge.libraries.getPtr(lib_name) orelse {
                        var new_lib = try self.bridge.registerLibrary(lib_name);
                        new_lib
                    };
                    try library.declareFunction(signature);
                }
            }
        }
    }
    
    /// Parse library declaration
    fn parseLibraryDeclaration(self: *ExternParser, line: []const u8) ![]const u8 {
        // Parse: library "libname"
        const start = std.mem.indexOf(u8, line, "\"") orelse return error.InvalidLibraryDeclaration;
        const end = std.mem.lastIndexOf(u8, line, "\"") orelse return error.InvalidLibraryDeclaration;
        
        if (start >= end) return error.InvalidLibraryDeclaration;
        
        return try self.allocator.dupe(u8, line[start + 1 .. end]);
    }
};

// Test cases for extern ABI
test "basic extern function parsing" {
    var bridge = CABIBridge.init(std.testing.allocator);
    defer bridge.deinit();
    
    const decl = "extern \"C\" int add(int a, int b)";
    const signature = try bridge.parseExternDeclaration(decl);
    defer signature.deinit();
    
    try std.testing.expectEqualStrings("add", signature.name);
    try std.testing.expect(signature.return_type == .Int32);
    try std.testing.expect(signature.parameters.items.len == 2);
    try std.testing.expectEqualStrings("a", signature.parameters.items[0].name);
    try std.testing.expect(signature.parameters.items[0].param_type == .Int32);
}

test "wrapper generation" {
    var bridge = CABIBridge.init(std.testing.allocator);
    defer bridge.deinit();
    
    var signature = CABISignature.init(std.testing.allocator, "strlen");
    defer signature.deinit();
    
    signature.return_type = .Int32;
    try signature.parameters.append(CABISignature.CABIParameter{
        .name = "str",
        .param_type = .String,
    });
    
    const wrapper = try bridge.generateWrapper(signature, "libc");
    defer std.testing.allocator.free(wrapper);
    
    // Wrapper should contain the function declaration
    try std.testing.expect(std.mem.indexOf(u8, wrapper, "slay strlen") != null);
    try std.testing.expect(std.mem.indexOf(u8, wrapper, "cursed_ffi_call") != null);
}
