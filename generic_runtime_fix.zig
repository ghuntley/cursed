/// Enhanced Generic Runtime System Fix for CURSED
/// Fixes critical issues with generic function calls causing runtime errors
/// Addresses the issue: "Undefined variable in drip assignment: 'generic_function<drip>(100)'"

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const ast = @import("ast.zig");
const interpreter = @import("interpreter.zig");
const generics = @import("generics.zig");

/// Enhanced generic function call resolver
pub const GenericCallResolver = struct {
    allocator: Allocator,
    functions: *HashMap([]const u8, interpreter.Function, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    generic_instances: HashMap([]const u8, interpreter.Function, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator, functions: *HashMap([]const u8, interpreter.Function, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) GenericCallResolver {
        return GenericCallResolver{
            .allocator = allocator,
            .functions = functions,
            .generic_instances = HashMap([]const u8, interpreter.Function, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *GenericCallResolver) void {
        var iterator = self.generic_instances.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.generic_instances.deinit();
    }
    
    /// Resolve generic function call with type arguments
    /// Handles both <T> and [T] syntax
    pub fn resolveGenericCall(self: *GenericCallResolver, function_name: []const u8) !?interpreter.Function {
        // Check if it's a generic call with angle brackets <T> or square brackets [T]
        const generic_call = self.parseGenericCall(function_name) catch return null;
        if (generic_call == null) return null;
        
        const call_info = generic_call.?;
        defer self.allocator.free(call_info.base_name);
        defer {
            for (call_info.type_args) |arg| {
                self.allocator.free(arg);
            }
            self.allocator.free(call_info.type_args);
        }
        
        // Check if we already have this instantiation cached
        if (self.generic_instances.get(function_name)) |cached| {
            return cached;
        }
        
        // Find the generic template function
        const template_func = self.findGenericTemplate(call_info.base_name) orelse {
            std.log.err("Generic template function not found: {s}", .{call_info.base_name});
            return null;
        };
        
        // Validate type argument count
        if (call_info.type_args.len != template_func.declaration.type_parameters.items.len) {
            std.log.err("Type argument count mismatch for {s}: expected {d}, got {d}", 
                .{call_info.base_name, template_func.declaration.type_parameters.items.len, call_info.type_args.len});
            return null;
        }
        
        // Create specialized instance
        const specialized_func = try self.createSpecializedFunction(template_func, call_info);
        
        // Cache the instance
        const cached_name = try self.allocator.dupe(u8, function_name);
        try self.generic_instances.put(cached_name, specialized_func);
        
        return specialized_func;
    }
    
    /// Parse generic function call to extract base name and type arguments
    const GenericCallInfo = struct {
        base_name: []const u8,
        type_args: [][]const u8,
    };
    
    fn parseGenericCall(self: *GenericCallResolver, function_name: []const u8) !?GenericCallInfo {
        // Try angle bracket syntax first: function_name<type1, type2>
        if (std.mem.indexOf(u8, function_name, "<")) |start| {
            if (std.mem.lastIndexOf(u8, function_name, ">")) |end| {
                const base_name = try self.allocator.dupe(u8, function_name[0..start]);
                const type_args_str = function_name[start + 1..end];
                const type_args = try self.parseTypeArguments(type_args_str);
                
                return GenericCallInfo{
                    .base_name = base_name,
                    .type_args = type_args,
                };
            }
        }
        
        // Try square bracket syntax: function_name[type1, type2]
        if (std.mem.indexOf(u8, function_name, "[")) |start| {
            if (std.mem.lastIndexOf(u8, function_name, "]")) |end| {
                const base_name = try self.allocator.dupe(u8, function_name[0..start]);
                const type_args_str = function_name[start + 1..end];
                const type_args = try self.parseTypeArguments(type_args_str);
                
                return GenericCallInfo{
                    .base_name = base_name,
                    .type_args = type_args,
                };
            }
        }
        
        return null;
    }
    
    /// Parse comma-separated type arguments
    fn parseTypeArguments(self: *GenericCallResolver, type_args_str: []const u8) ![][]const u8 {
        var type_args = ArrayList([]const u8).init(self.allocator);
        defer type_args.deinit();
        
        var iterator = std.mem.split(u8, type_args_str, ",");
        while (iterator.next()) |type_arg| {
            const trimmed = std.mem.trim(u8, type_arg, " \t\n");
            if (trimmed.len > 0) {
                try type_args.append(try self.allocator.dupe(u8, trimmed));
            }
        }
        
        return type_args.toOwnedSlice();
    }
    
    /// Find generic template function by base name
    fn findGenericTemplate(self: *GenericCallResolver, base_name: []const u8) ?interpreter.Function {
        // First, try exact match
        if (self.functions.get(base_name)) |func| {
            if (func.declaration.type_parameters.items.len > 0) {
                return func;
            }
        }
        
        // Search for functions that start with base_name and have type parameters
        var iterator = self.functions.iterator();
        while (iterator.next()) |entry| {
            const func_name = entry.key_ptr.*;
            const func = entry.value_ptr.*;
            
            if (std.mem.startsWith(u8, func_name, base_name) and 
                func.declaration.type_parameters.items.len > 0) {
                return func;
            }
        }
        
        return null;
    }
    
    /// Create specialized function instance
    fn createSpecializedFunction(self: *GenericCallResolver, template_func: interpreter.Function, call_info: GenericCallInfo) !interpreter.Function {
        // Create type substitution map
        var type_substitutions = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer type_substitutions.deinit();
        
        for (template_func.declaration.type_parameters.items, 0..) |param, i| {
            if (i < call_info.type_args.len) {
                try type_substitutions.put(param.name, call_info.type_args[i]);
            }
        }
        
        // Create specialized function declaration
        var specialized_decl = ast.FunctionStatement{
            .name = try self.allocator.dupe(u8, template_func.declaration.name),
            .parameters = ArrayList(ast.Parameter).init(self.allocator),
            .return_type = null, // Will be set if needed
            .body = ArrayList(ast.Statement).init(self.allocator),
            .type_parameters = ArrayList(ast.TypeParameter).init(self.allocator), // No type params for instance
            .is_async = template_func.declaration.is_async,
        };
        
        // Copy and substitute parameters
        for (template_func.declaration.parameters.items) |param| {
            const substituted_type = try self.substituteType(param.param_type, &type_substitutions);
            
            try specialized_decl.parameters.append(ast.Parameter{
                .name = try self.allocator.dupe(u8, param.name),
                .param_type = substituted_type,
            });
        }
        
        // Copy and substitute return type
        if (template_func.declaration.return_type) |ret_type| {
            specialized_decl.return_type = try self.substituteType(ret_type, &type_substitutions);
        }
        
        // Copy function body (would need deep substitution for complex cases)
        for (template_func.declaration.body.items) |stmt| {
            try specialized_decl.body.append(stmt); // Shallow copy for now
        }
        
        return interpreter.Function{
            .declaration = specialized_decl,
            .environment = template_func.environment, // Share environment for now
        };
    }
    
    /// Substitute type parameters in a type
    fn substituteType(self: *GenericCallResolver, original_type: ast.Type, substitutions: *HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !ast.Type {
        return switch (original_type) {
            .Identifier => |name| {
                if (substitutions.get(name)) |concrete_type| {
                    // Convert string to appropriate type
                    return self.stringToType(concrete_type);
                } else {
                    return original_type;
                }
            },
            .Array => |array_type| {
                const new_element_type = try self.allocator.create(ast.Type);
                new_element_type.* = try self.substituteType(array_type.element_type.*, substitutions);
                return ast.Type{ .Array = ast.ArrayType{
                    .element_type = new_element_type,
                    .size = array_type.size,
                }};
            },
            .Slice => |slice_type| {
                const new_element_type = try self.allocator.create(ast.Type);
                new_element_type.* = try self.substituteType(slice_type.element_type.*, substitutions);
                return ast.Type{ .Slice = ast.SliceType{
                    .element_type = new_element_type,
                }};
            },
            else => original_type,
        };
    }
    
    /// Convert type string to AST type
    fn stringToType(self: *GenericCallResolver, type_str: []const u8) ast.Type {
        _ = self;
        
        // Map CURSED type names to AST types
        if (std.mem.eql(u8, type_str, "drip")) {
            return ast.Type{ .Primitive = .Drip };
        } else if (std.mem.eql(u8, type_str, "normie")) {
            return ast.Type{ .Primitive = .Normie };
        } else if (std.mem.eql(u8, type_str, "tea")) {
            return ast.Type{ .Primitive = .Tea };
        } else if (std.mem.eql(u8, type_str, "lit")) {
            return ast.Type{ .Primitive = .Lit };
        } else if (std.mem.eql(u8, type_str, "smol")) {
            return ast.Type{ .Primitive = .Smol };
        } else if (std.mem.eql(u8, type_str, "thicc")) {
            return ast.Type{ .Primitive = .Thicc };
        } else if (std.mem.eql(u8, type_str, "meal")) {
            return ast.Type{ .Primitive = .Meal };
        } else if (std.mem.eql(u8, type_str, "snack")) {
            return ast.Type{ .Primitive = .Snack };
        } else if (std.mem.eql(u8, type_str, "vibes")) {
            return ast.Type{ .Primitive = .Vibes };
        } else {
            // User-defined type
            return ast.Type{ .Identifier = type_str };
        }
    }
};

/// Enhanced evaluateCall function for interpreter
pub fn enhancedEvaluateCall(interpreter_self: *interpreter.Interpreter, call: ast.CallExpression, resolver: *GenericCallResolver) !interpreter.Value {
    switch (call.function.*) {
        .Identifier => |name| {
            // Try direct function lookup first
            if (interpreter_self.functions.get(name)) |func| {
                std.log.info("Calling direct function: {s}", .{name});
                
                // Evaluate arguments
                var args = ArrayList(interpreter.Value).init(interpreter_self.allocator);
                defer args.deinit();
                
                for (call.arguments.items) |arg_expr| {
                    const arg = try interpreter_self.evaluateExpression(arg_expr.*);
                    try args.append(arg);
                }
                
                return try interpreter_self.callFunction(func, args.items);
            }
            
            // Try generic function resolution
            if (try resolver.resolveGenericCall(name)) |generic_func| {
                std.log.info("Calling generic function: {s}", .{name});
                
                // Evaluate arguments
                var args = ArrayList(interpreter.Value).init(interpreter_self.allocator);
                defer args.deinit();
                
                for (call.arguments.items) |arg_expr| {
                    const arg = try interpreter_self.evaluateExpression(arg_expr.*);
                    try args.append(arg);
                }
                
                return try interpreter_self.callFunction(generic_func, args.items);
            }
            
            std.log.err("Function not found: {s}", .{name});
            return interpreter.InterpreterError.UndefinedFunction;
        },
        else => {
            // Handle other call types (member access, etc.)
            return interpreter.InterpreterError.UndefinedFunction;
        },
    }
}

/// Integration helper to patch existing interpreter
pub fn patchInterpreterForGenerics(interp: *interpreter.Interpreter) !GenericCallResolver {
    return GenericCallResolver.init(interp.allocator, &interp.functions);
}
