const std = @import("std");
const testing = std.testing;

const runtime_generics = @import("src-zig/runtime_generic_system.zig");

test "runtime generic type system basic functionality" {
    const allocator = testing.allocator;
    
    var type_env = runtime_generics.RuntimeTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    try runtime_generics.initializeBuiltinTypes(&type_env);
    
    // Test basic type lookup
    const drip_type = type_env.type_registry.get("drip").?;
    try testing.expect(type_env.isNumeric(drip_type));
    try testing.expect(type_env.isComparable(drip_type));
    
    // Test constraint checking
    const constraint = runtime_generics.RuntimeConstraint.init(allocator, .Numeric);
    try testing.expect(try constraint.check(drip_type, &type_env));
    
    std.log.info("✅ Basic runtime generic functionality working", .{});
}

test "runtime type parameter constraints" {
    const allocator = testing.allocator;
    
    var type_env = runtime_generics.RuntimeTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    try runtime_generics.initializeBuiltinTypes(&type_env);
    
    // Create type parameter with constraints
    var type_param = runtime_generics.RuntimeTypeParameter.init(allocator, "T", .Invariant);
    defer type_param.deinit();
    
    const numeric_constraint = runtime_generics.RuntimeConstraint.init(allocator, .Numeric);
    try type_param.addConstraint(numeric_constraint);
    
    // Test with valid type
    const drip_type = type_env.type_registry.get("drip").?;
    try testing.expect(try type_param.satisfiesConstraints(drip_type, &type_env));
    
    // Test with invalid type  
    const tea_type = type_env.type_registry.get("tea").?;
    try testing.expect(!try type_param.satisfiesConstraints(tea_type, &type_env));
    
    std.log.info("✅ Type parameter constraints working", .{});
}

test "runtime generic instantiation" {
    const allocator = testing.allocator;
    
    var type_env = runtime_generics.RuntimeTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    try runtime_generics.initializeBuiltinTypes(&type_env);
    
    // Create generic declaration
    var generic_decl = runtime_generics.RuntimeTypeEnvironment.GenericDeclaration.init(allocator, "Container");
    defer generic_decl.deinit(allocator);
    
    var type_param = runtime_generics.RuntimeTypeParameter.init(allocator, "T", .Covariant);
    defer type_param.deinit();
    
    try generic_decl.type_parameters.append(type_param);
    try type_env.registerGeneric(generic_decl);
    
    // Test instantiation
    const drip_type = type_env.type_registry.get("drip").?;
    const type_args = [_]runtime_generics.RuntimeType{drip_type};
    
    const instantiated = try type_env.instantiateGeneric("Container", &type_args);
    try testing.expect(instantiated.kind == .Instantiated);
    try testing.expect(std.mem.eql(u8, instantiated.name, "Container"));
    
    std.log.info("✅ Generic instantiation working", .{});
}

test "runtime variance checking" {
    const allocator = testing.allocator;
    
    var type_env = runtime_generics.RuntimeTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    try runtime_generics.initializeBuiltinTypes(&type_env);
    
    // Test type relationships
    const drip_type = type_env.type_registry.get("drip").?;
    const normie_type = type_env.type_registry.get("normie").?;
    
    // Test subtyping (numeric types should be compatible)
    try testing.expect(type_env.areTypesEqual(drip_type, drip_type));
    
    // Test coercion rules
    _ = normie_type; // Used for coercion testing
    try testing.expect(type_env.isPrimitiveCoercible("normie", "drip"));
    
    std.log.info("✅ Variance checking working", .{});
}

test "runtime type metadata" {
    const allocator = testing.allocator;
    
    var type_env = runtime_generics.RuntimeTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    try runtime_generics.initializeBuiltinTypes(&type_env);
    
    const drip_type = type_env.type_registry.get("drip").?;
    
    // Check metadata
    try testing.expect(drip_type.metadata.size == 8);  // 64-bit integer
    try testing.expect(drip_type.metadata.is_send);
    try testing.expect(drip_type.metadata.is_sync);
    try testing.expect(drip_type.metadata.is_copy);
    
    std.log.info("✅ Type metadata working", .{});
}
