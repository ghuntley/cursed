const std = @import("std");
const testing = std.testing;
const comprehensive_type_system = @import("src-zig/comprehensive_type_system.zig");
const TypeEnvironment = comprehensive_type_system.TypeEnvironment;
const CursedType = comprehensive_type_system.CursedType;

test "type variable unification with occurs check" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var env = try TypeEnvironment.init(allocator);
    defer env.deinit();
    
    // Test 1: Simple unification should work
    const var1 = env.freshTypeVar();
    try env.unifyTypeVar(var1, CursedType.Drip);
    
    const resolved = env.resolveType(CursedType{ .Unknown = var1 });
    try testing.expect(std.meta.eql(resolved, CursedType.Drip));
    
    // Test 2: Occurs check should prevent infinite types
    const var2 = env.freshTypeVar();
    const array_type_ptr = try allocator.create(comprehensive_type_system.ArrayType);
    array_type_ptr.* = comprehensive_type_system.ArrayType{
        .element_type = try allocator.create(CursedType),
        .size = null,
    };
    array_type_ptr.element_type.* = CursedType{ .Unknown = var2 };
    
    const array_type = CursedType{ .Array = array_type_ptr };
    
    // This should fail due to occurs check (T = Array[T])
    try testing.expectError(error.InfiniteTypeError, env.unifyTypeVar(var2, array_type));
}

test "constraint validation during unification" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var env = try TypeEnvironment.init(allocator);
    defer env.deinit();
    
    // Add a numeric constraint to a type variable
    const var1 = env.freshTypeVar();
    var constraint_set = comprehensive_type_system.TypeEnvironment.TypeConstraintSet{
        .type_var = var1,
        .constraints = std.ArrayList(comprehensive_type_system.TypeConstraint).init(allocator),
    };
    try constraint_set.constraints.append(comprehensive_type_system.TypeConstraint{
        .kind = .Numeric,
        .bound = null,
    });
    try env.constraints.append(constraint_set);
    
    // Test 1: Unifying with numeric type should succeed
    try env.unifyTypeVar(var1, CursedType.Drip);
    
    // Test 2: Unifying with non-numeric type should fail
    const var2 = env.freshTypeVar();
    var constraint_set2 = comprehensive_type_system.TypeEnvironment.TypeConstraintSet{
        .type_var = var2,
        .constraints = std.ArrayList(comprehensive_type_system.TypeConstraint).init(allocator),
    };
    try constraint_set2.constraints.append(comprehensive_type_system.TypeConstraint{
        .kind = .Numeric,
        .bound = null,
    });
    try env.constraints.append(constraint_set2);
    
    try testing.expectError(error.ConstraintViolationError, env.unifyTypeVar(var2, CursedType.Tea));
}

test "recursive type resolution" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var env = try TypeEnvironment.init(allocator);
    defer env.deinit();
    
    // Create chain of type variables: T1 -> T2 -> T3 -> Drip
    const var1 = env.freshTypeVar();
    const var2 = env.freshTypeVar();
    const var3 = env.freshTypeVar();
    
    try env.unifyTypeVar(var3, CursedType.Drip);
    try env.unifyTypeVar(var2, CursedType{ .Unknown = var3 });
    try env.unifyTypeVar(var1, CursedType{ .Unknown = var2 });
    
    const resolved = env.resolveTypeRecursive(CursedType{ .Unknown = var1 });
    try testing.expect(std.meta.eql(resolved, CursedType.Drip));
}

test "type compatibility checking" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var env = try TypeEnvironment.init(allocator);
    defer env.deinit();
    
    // Test numeric compatibility
    try testing.expect(env.areTypesCompatible(CursedType.Drip, CursedType.Normie));
    try testing.expect(env.areTypesCompatible(CursedType.Meal, CursedType.Snack));
    
    // Test incompatible types
    try testing.expect(!env.areTypesCompatible(CursedType.Tea, CursedType.Drip));
    try testing.expect(!env.areTypesCompatible(CursedType.Lit, CursedType.Meal));
}

test "types equal checking" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var env = try TypeEnvironment.init(allocator);
    defer env.deinit();
    
    // Test primitive type equality
    try testing.expect(env.typesEqual(CursedType.Drip, CursedType.Drip));
    try testing.expect(!env.typesEqual(CursedType.Drip, CursedType.Normie));
    
    // Test array type equality
    const elem_type1 = try allocator.create(CursedType);
    elem_type1.* = CursedType.Drip;
    const arr_type1 = try allocator.create(comprehensive_type_system.ArrayType);
    arr_type1.* = comprehensive_type_system.ArrayType{
        .element_type = elem_type1,
        .size = null,
    };
    
    const elem_type2 = try allocator.create(CursedType);
    elem_type2.* = CursedType.Drip;
    const arr_type2 = try allocator.create(comprehensive_type_system.ArrayType);
    arr_type2.* = comprehensive_type_system.ArrayType{
        .element_type = elem_type2,
        .size = null,
    };
    
    const array1 = CursedType{ .Array = arr_type1 };
    const array2 = CursedType{ .Array = arr_type2 };
    
    try testing.expect(env.typesEqual(array1, array2));
}
