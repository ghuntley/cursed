/// Test integration of generics system with CURSED compiler
const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const testing = std.testing;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
});

const generics = @import("src-zig/generics.zig");
const ast = @import("src-zig/ast_fixed.zig");

test "generics monomorphization basic functionality" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Initialize LLVM
    c.LLVMInitializeCore(c.LLVMGetGlobalPassRegistry());
    c.LLVMInitializeNativeTarget();
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    // Initialize monomorphizer
    var monomorphizer = generics.Monomorphizer.init(allocator, context, module);
    defer monomorphizer.deinit();
    
    // Test type parameter creation
    var type_param = generics.TypeParameter.init(allocator, "T");
    defer type_param.deinit(allocator);
    
    try type_param.constraints.append(generics.Constraint.init(.Any));
    try testing.expect(std.mem.eql(u8, type_param.name, "T"));
    try testing.expect(type_param.constraints.items.len == 1);
    
    std.log.info("Basic generics functionality test passed", .{});
}

test "constraint validation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    var monomorphizer = generics.Monomorphizer.init(allocator, context, module);
    defer monomorphizer.deinit();
    
    // Test numeric constraint validation
    const int_type = ast.Type{ .Primitive = .Normie };
    const string_type = ast.Type{ .Primitive = .Tea };
    
    try testing.expect(try monomorphizer.isNumeric(int_type));
    try testing.expect(!(try monomorphizer.isNumeric(string_type)));
    
    // Test comparable constraint validation
    try testing.expect(try monomorphizer.isComparable(int_type));
    try testing.expect(try monomorphizer.isComparable(string_type));
    
    std.log.info("Constraint validation test passed", .{});
}

test "type substitution" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    var monomorphizer = generics.Monomorphizer.init(allocator, context, module);
    defer monomorphizer.deinit();
    
    // Create substitution map
    var substitutions = std.HashMap([]const u8, ast.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    defer substitutions.deinit();
    
    try substitutions.put("T", ast.Type{ .Primitive = .Normie });
    
    // Test type substitution
    const generic_type = ast.Type{ .Identifier = "T" };
    const substituted = try monomorphizer.substituteType(generic_type, &substitutions);
    
    try testing.expect(substituted == .Primitive);
    try testing.expect(substituted.Primitive == .Normie);
    
    std.log.info("Type substitution test passed", .{});
}

test "specialized name generation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    var monomorphizer = generics.Monomorphizer.init(allocator, context, module);
    defer monomorphizer.deinit();
    
    // Test name generation
    const type_args = [_]ast.Type{
        ast.Type{ .Primitive = .Normie },
        ast.Type{ .Primitive = .Tea },
    };
    
    const specialized_name = try monomorphizer.generateSpecializedName("Box", &type_args);
    defer allocator.free(specialized_name);
    
    try testing.expect(std.mem.eql(u8, specialized_name, "Box_i32_string"));
    
    std.log.info("Specialized name generation test passed", .{});
}

test "LLVM type conversion" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    var monomorphizer = generics.Monomorphizer.init(allocator, context, module);
    defer monomorphizer.deinit();
    
    // Test CURSED type to LLVM type conversion
    const int_type = ast.Type{ .Primitive = .Normie };
    const llvm_int_type = try monomorphizer.typeToLLVMType(int_type);
    
    try testing.expect(c.LLVMGetTypeKind(llvm_int_type) == c.LLVMIntegerTypeKind);
    try testing.expect(c.LLVMGetIntTypeWidth(llvm_int_type) == 32);
    
    const bool_type = ast.Type{ .Primitive = .Lit };
    const llvm_bool_type = try monomorphizer.typeToLLVMType(bool_type);
    
    try testing.expect(c.LLVMGetTypeKind(llvm_bool_type) == c.LLVMIntegerTypeKind);
    try testing.expect(c.LLVMGetIntTypeWidth(llvm_bool_type) == 1);
    
    std.log.info("LLVM type conversion test passed", .{});
}

test "generic function registration and instantiation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    var monomorphizer = generics.Monomorphizer.init(allocator, context, module);
    defer monomorphizer.deinit();
    
    // Create generic function declaration
    var generic_decl = generics.GenericDeclaration.init(allocator, "identity", .Function);
    defer generic_decl.deinit(allocator);
    
    var type_param = generics.TypeParameter.init(allocator, "T");
    defer type_param.deinit(allocator);
    try type_param.constraints.append(generics.Constraint.init(.Any));
    try generic_decl.type_parameters.append(type_param);
    
    // Create dummy function declaration
    var func_decl = ast.FunctionDeclaration{
        .name = "identity",
        .parameters = ArrayList(ast.Parameter).init(allocator),
        .return_type = ast.Type{ .Identifier = "T" },
        .body = ArrayList(ast.Statement).init(allocator),
        .is_async = false,
    };
    defer {
        func_decl.parameters.deinit();
        func_decl.body.deinit();
    }
    
    try func_decl.parameters.append(ast.Parameter{
        .name = "x",
        .param_type = ast.Type{ .Identifier = "T" },
    });
    
    generic_decl.ast_node = generics.GenericDeclaration.ASTNode{ .Function = &func_decl };
    
    // Register generic
    try monomorphizer.registerGeneric(generic_decl);
    
    // Request instantiation
    const type_args = [_]ast.Type{ast.Type{ .Primitive = .Normie }};
    const specialized_name = try monomorphizer.requestInstantiation("identity", &type_args, "test_location");
    defer allocator.free(specialized_name);
    
    try testing.expect(std.mem.eql(u8, specialized_name, "identity_i32"));
    
    // Process instantiations
    try monomorphizer.processInstantiations();
    
    // Check if instantiated
    try testing.expect(monomorphizer.isInstantiated(specialized_name));
    
    std.log.info("Generic function registration and instantiation test passed", .{});
}
