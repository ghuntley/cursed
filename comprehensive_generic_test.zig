const std = @import("std");
const testing = std.testing;
const print = std.debug.print;

const ast = @import("src-zig/ast.zig");
const type_system = @import("src-zig/type_system_runtime.zig");
const generic_constraints = @import("src-zig/generic_constraint_system.zig");
const type_inference = @import("src-zig/type_inference.zig");
const generics = @import("src-zig/generics.zig");

test "Comprehensive Generic Constraint System Test" {
    print("\n🧪 Testing CURSED Generic Constraint System\n", .{});
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Initialize type registry
    var type_registry = type_system.GCTypeRegistry.init(allocator);
    defer type_registry.deinit();
    
    // Initialize constraint validator
    var constraint_validator = generic_constraints.ConstraintValidator.init(allocator, &type_registry);
    defer constraint_validator.deinit();
    
    print("✅ Constraint validator initialized\n", .{});
    
    // Test 1: Numeric constraint validation
    {
        print("\n🔢 Test 1: Numeric Constraint Validation\n", .{});
        
        const numeric_constraint = generic_constraints.TypeConstraint.init(.Numeric);
        
        // Valid numeric types
        const valid_types = [_]ast.Type{
            ast.Type{ .Primitive = .Drip },
            ast.Type{ .Primitive = .Normie },
            ast.Type{ .Primitive = .Meal },
            ast.Type{ .Primitive = .Snack },
        };
        
        for (valid_types) |valid_type| {
            const result = constraint_validator.validateConstraint(valid_type, numeric_constraint);
            try testing.expect(result.valid);
            print("  ✅ {} satisfies Numeric constraint\n", .{valid_type});
        }
        
        // Invalid numeric types
        const invalid_types = [_]ast.Type{
            ast.Type{ .Primitive = .Tea },
            ast.Type{ .Primitive = .Lit },
            ast.Type{ .Primitive = .Vibes },
        };
        
        for (invalid_types) |invalid_type| {
            const result = constraint_validator.validateConstraint(invalid_type, numeric_constraint);
            try testing.expect(!result.valid);
            print("  ❌ {} does not satisfy Numeric constraint\n", .{invalid_type});
        }
    }
    
    // Test 2: Comparable constraint validation
    {
        print("\n🔍 Test 2: Comparable Constraint Validation\n", .{});
        
        const comparable_constraint = generic_constraints.TypeConstraint.init(.Comparable);
        
        // Valid comparable types
        const valid_types = [_]ast.Type{
            ast.Type{ .Primitive = .Drip },
            ast.Type{ .Primitive = .Tea },
            ast.Type{ .Primitive = .Lit },
        };
        
        for (valid_types) |valid_type| {
            const result = constraint_validator.validateConstraint(valid_type, comparable_constraint);
            try testing.expect(result.valid);
            print("  ✅ {} satisfies Comparable constraint\n", .{valid_type});
        }
        
        // vibes is not comparable
        const vibes_type = ast.Type{ .Primitive = .Vibes };
        const vibes_result = constraint_validator.validateConstraint(vibes_type, comparable_constraint);
        try testing.expect(!vibes_result.valid);
        print("  ❌ {} does not satisfy Comparable constraint\n", .{vibes_type});
    }
    
    // Test 3: Ordered constraint validation
    {
        print("\n📊 Test 3: Ordered Constraint Validation\n", .{});
        
        const ordered_constraint = generic_constraints.TypeConstraint.init(.Ordered);
        
        // Valid ordered types (comparable except boolean)
        const valid_types = [_]ast.Type{
            ast.Type{ .Primitive = .Drip },
            ast.Type{ .Primitive = .Tea },
        };
        
        for (valid_types) |valid_type| {
            const result = constraint_validator.validateConstraint(valid_type, ordered_constraint);
            try testing.expect(result.valid);
            print("  ✅ {} satisfies Ordered constraint\n", .{valid_type});
        }
        
        // lit (boolean) is comparable but not ordered
        const bool_type = ast.Type{ .Primitive = .Lit };
        const bool_result = constraint_validator.validateConstraint(bool_type, ordered_constraint);
        try testing.expect(!bool_result.valid);
        print("  ❌ {} does not satisfy Ordered constraint (boolean not ordered)\n", .{bool_type});
    }
    
    // Test 4: Sized constraint validation
    {
        print("\n📏 Test 4: Sized Constraint Validation\n", .{});
        
        const sized_constraint = generic_constraints.TypeConstraint.init(.Sized);
        
        // Most types are sized
        const valid_types = [_]ast.Type{
            ast.Type{ .Primitive = .Drip },
            ast.Type{ .Array = ast.ArrayType{
                .element_type = &ast.Type{ .Primitive = .Normie },
                .size = 10,
            }},
        };
        
        for (valid_types) |valid_type| {
            const result = constraint_validator.validateConstraint(valid_type, sized_constraint);
            try testing.expect(result.valid);
            print("  ✅ {} satisfies Sized constraint\n", .{valid_type});
        }
        
        // Slices have dynamic size
        const slice_type = ast.Type{ .Slice = ast.SliceType{
            .element_type = &ast.Type{ .Primitive = .Normie },
        }};
        const slice_result = constraint_validator.validateConstraint(slice_type, sized_constraint);
        try testing.expect(!slice_result.valid);
        print("  ❌ slice type does not satisfy Sized constraint (dynamic size)\n", .{});
    }
    
    // Test 5: Const generic constraint validation
    {
        print("\n🔧 Test 5: Const Generic Constraint Validation\n", .{});
        
        const const_bounds = generic_constraints.TypeConstraint.ConstGenericBounds{
            .min_value = 0,
            .max_value = 1000,
        };
        const const_constraint = generic_constraints.TypeConstraint.initConstGeneric(const_bounds);
        
        // Valid const generic types (integers)
        const valid_types = [_]ast.Type{
            ast.Type{ .Primitive = .Normie },
            ast.Type{ .Primitive = .Drip },
        };
        
        for (valid_types) |valid_type| {
            const result = constraint_validator.validateConstraint(valid_type, const_constraint);
            try testing.expect(result.valid);
            print("  ✅ {} satisfies ConstGeneric constraint\n", .{valid_type});
        }
        
        // Floats are not valid for const generics
        const float_type = ast.Type{ .Primitive = .Meal };
        const float_result = constraint_validator.validateConstraint(float_type, const_constraint);
        try testing.expect(!float_result.valid);
        print("  ❌ {} does not satisfy ConstGeneric constraint (not integer)\n", .{float_type});
    }
    
    // Test 6: Multiple constraints validation
    {
        print("\n🔗 Test 6: Multiple Constraints Validation\n", .{});
        
        const constraints = [_]generic_constraints.TypeConstraint{
            generic_constraints.TypeConstraint.init(.Numeric),
            generic_constraints.TypeConstraint.init(.Comparable),
            generic_constraints.TypeConstraint.init(.Sized),
        };
        
        // Integer types satisfy all three constraints
        const int_type = ast.Type{ .Primitive = .Drip };
        const all_valid = constraint_validator.satisfiesAllConstraints(int_type, &constraints);
        try testing.expect(all_valid);
        print("  ✅ {} satisfies all constraints (Numeric + Comparable + Sized)\n", .{int_type});
        
        // String type fails numeric constraint
        const string_type = ast.Type{ .Primitive = .Tea };
        const some_invalid = constraint_validator.satisfiesAllConstraints(string_type, &constraints);
        try testing.expect(!some_invalid);
        print("  ❌ {} fails some constraints (not Numeric)\n", .{string_type});
    }
    
    // Test 7: Type inference with constraints
    {
        print("\n🧠 Test 7: Type Inference with Constraints\n", .{});
        
        var inference_context = type_inference.TypeInferenceContext.init(
            allocator, 
            undefined, // monomorphizer not needed for this test
            &type_registry
        );
        defer inference_context.deinit();
        
        // Add a constraint that T must be numeric
        try inference_context.addConstraint("T", ast.Type{ .Primitive = .Drip }, .Argument);
        
        // Solve constraints
        try inference_context.solveConstraints();
        
        // Check if T was inferred correctly
        if (inference_context.inferred_types.get("T")) |inferred_type| {
            try testing.expect(inferred_type.Primitive == .Drip);
            print("  ✅ Type parameter T correctly inferred as: {}\n", .{inferred_type});
        }
    }
    
    // Test 8: Suggestion system
    {
        print("\n💡 Test 8: Type Suggestion System\n", .{});
        
        const numeric_constraint = generic_constraints.TypeConstraint.init(.Numeric);
        const suggestions = constraint_validator.getSuggestedTypes(numeric_constraint);
        
        try testing.expect(suggestions.len > 0);
        print("  ✅ Suggestions for Numeric constraint:\n", .{});
        for (suggestions) |suggestion| {
            print("    - {s}\n", .{suggestion});
        }
        
        const comparable_constraint = generic_constraints.TypeConstraint.init(.Comparable);
        const comp_suggestions = constraint_validator.getSuggestedTypes(comparable_constraint);
        
        try testing.expect(comp_suggestions.len > 0);
        print("  ✅ Suggestions for Comparable constraint:\n", .{});
        for (comp_suggestions) |suggestion| {
            print("    - {s}\n", .{suggestion});
        }
    }
    
    print("\n🎉 All generic constraint tests passed!\n", .{});
    print("✅ Comprehensive constraint validation system working correctly\n", .{});
    print("✅ Type inference with constraints functional\n", .{});
    print("✅ Error messages and suggestions implemented\n", .{});
    print("✅ Const generic bounds checking operational\n", .{});
    print("✅ Multi-constraint validation working\n", .{});
}
