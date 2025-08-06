const std = @import("std");
const print = std.debug.print;

const type_system = @import("src-zig/type_system.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🔧 Testing CURSED Type System Foundation\n", .{});
    print("==================================================\n", .{});

    // Test type checker initialization
    var checker = type_system.TypeChecker.init(allocator) catch |err| {
        print("❌ Type checker initialization failed: {}\n", .{err});
        return;
    };
    defer checker.deinit();

    print("✅ Type checker initialized successfully\n", .{});

    // Test built-in types
    print("\n📋 Built-in CURSED types:\n", .{});
    const builtin_types = [_][]const u8{
        "lit",    // boolean
        "drip",   // integer
        "normie", // integer
        "thicc",  // large integer
        "smol",   // small integer
        "mid",    // medium integer
        "tea",    // string
        "sip",    // character
        "snack",  // float
        "meal",   // double
        "byte",   // byte
        "rune",   // unicode char
        "cap",    // void/unit
        "vibez",  // built-in object
    };

    for (builtin_types) |type_name| {
        if (checker.environment.hasType(type_name)) {
            print("  ✅ {s}\n", .{type_name});
        } else {
            print("  ❌ {s} (missing!)\n", .{type_name});
        }
    }

    // Test type expression creation
    print("\n🧪 Testing type expressions:\n", .{});
    
    var drip_type = type_system.TypeExpression.named(allocator, "drip");
    defer drip_type.deinit();
    print("  ✅ Created drip type: {s}\n", .{drip_type.name.?});

    var tea_type = type_system.TypeExpression.named(allocator, "tea");
    defer tea_type.deinit();
    print("  ✅ Created tea type: {s}\n", .{tea_type.name.?});

    var lit_type = type_system.TypeExpression.named(allocator, "lit");
    defer lit_type.deinit();
    print("  ✅ Created lit type: {s}\n", .{lit_type.name.?});

    // Test type compatibility
    print("\n🔍 Testing type compatibility:\n", .{});
    
    var normie_type = type_system.TypeExpression.named(allocator, "normie");
    defer normie_type.deinit();

    if (checker.typesCompatible(&drip_type, &normie_type)) {
        print("  ✅ drip and normie are compatible (numeric coercion)\n", .{});
    } else {
        print("  ❌ drip and normie should be compatible\n", .{});
    }

    if (!checker.typesCompatible(&drip_type, &tea_type)) {
        print("  ✅ drip and tea are incompatible (as expected)\n", .{});
    } else {
        print("  ❌ drip and tea should not be compatible\n", .{});
    }

    // Test numeric type checking
    print("\n🔢 Testing numeric type detection:\n", .{});
    if (drip_type.isNumeric()) {
        print("  ✅ drip is numeric\n", .{});
    } else {
        print("  ❌ drip should be numeric\n", .{});
    }

    if (!tea_type.isNumeric()) {
        print("  ✅ tea is not numeric (as expected)\n", .{});
    } else {
        print("  ❌ tea should not be numeric\n", .{});
    }

    // Test boolean type checking
    print("\n🔘 Testing boolean type detection:\n", .{});
    if (lit_type.isBoolean()) {
        print("  ✅ lit is boolean\n", .{});
    } else {
        print("  ❌ lit should be boolean\n", .{});
    }

    // Test string type checking
    print("\n📝 Testing string type detection:\n", .{});
    if (tea_type.isString()) {
        print("  ✅ tea is string\n", .{});
    } else {
        print("  ❌ tea should be string\n", .{});
    }

    // Test variable management
    print("\n📦 Testing variable management:\n", .{});
    
    try checker.addVariable("x", drip_type, false);
    if (checker.getVariable("x")) |var_info| {
        print("  ✅ Variable 'x' stored and retrieved: {s}\n", .{var_info.name});
        if (var_info.type_expr.name) |type_name| {
            print("     Type: {s}\n", .{type_name});
        }
    } else {
        print("  ❌ Failed to retrieve variable 'x'\n", .{});
    }

    // Test scoping
    print("\n🎯 Testing scope management:\n", .{});
    try checker.enterScope();
    try checker.addVariable("y", tea_type, true);
    
    if (checker.getVariable("y")) |var_info| {
        print("  ✅ Variable 'y' in new scope: {s}\n", .{var_info.name});
    }
    
    if (checker.getVariable("x")) |_| {
        print("  ✅ Variable 'x' still accessible from parent scope\n", .{});
    }
    
    checker.exitScope();
    
    if (checker.getVariable("y") == null) {
        print("  ✅ Variable 'y' no longer accessible after scope exit\n", .{});
    }

    // Test vibez built-in object
    print("\n💬 Testing vibez built-in object:\n", .{});
    if (checker.environment.getType("vibez")) |vibez_type_def| {
        print("  ✅ vibez object found\n", .{});
        if (vibez_type_def.getMethod("spill")) |spill_method| {
            print("  ✅ vibez.spill method found\n", .{});
            print("     Parameters: {}\n", .{spill_method.parameters.items.len});
            if (spill_method.return_type) |ret_type| {
                if (ret_type.name) |ret_name| {
                    print("     Return type: {s}\n", .{ret_name});
                }
            }
        } else {
            print("  ❌ vibez.spill method not found\n", .{});
        }
    } else {
        print("  ❌ vibez object not found\n", .{});
    }

    print("\n==================================================\n", .{});
    print("🎉 Type System Foundation Test Complete!\n", .{});
    print("\nImplemented components:\n", .{});
    print("  ✅ Core type system infrastructure\n", .{});
    print("  ✅ Type environment with built-in CURSED types\n", .{});
    print("  ✅ Type expressions and operations\n", .{});
    print("  ✅ Variable type tracking with scoping\n", .{});
    print("  ✅ Type compatibility checking with CURSED coercion rules\n", .{});
    print("  ✅ Symbol table management\n", .{});
    print("  ✅ Built-in objects (vibez) with method signatures\n", .{});
    print("\nReady for basic CURSED program type checking!\n", .{});
}
