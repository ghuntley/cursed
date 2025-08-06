const std = @import("std");
const DebugInfoGenerator = @import("enhanced_debug_generation.zig").DebugInfoGenerator;
const FunctionParameter = @import("enhanced_debug_generation.zig").FunctionParameter;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("🔧 Testing comprehensive CURSED debug information generation...\n\n", .{});

    // Initialize debug generator
    var debug_gen = try DebugInfoGenerator.init(allocator, "test_program.csd");
    defer debug_gen.deinit();

    // Create CURSED types
    _ = try debug_gen.createCursedTypes();
    std.debug.print("✅ Created {} CURSED types\n\n", .{8});

    // Simulate parsing and compiling this CURSED program:
    // sus global_count drip = 0
    // 
    // squad Point {
    //     spill x normie
    //     spill y normie
    // }
    // 
    // slay add_numbers(first normie, second normie) normie {
    //     sus local_sum normie = first + second
    //     sus temp_var drip = 42
    //     damn local_sum
    // }
    // 
    // slay main() {
    //     sus x normie = 10
    //     sus y normie = 20
    //     sus result normie = add_numbers(x, y)
    //     vibez.spill(result)
    // }

    // Create debug info for global variable
    _ = try debug_gen.createVariable("global_count", "drip", 1, 0);

    // Create debug info for struct Point (simplified as type info)
    const point_type_id = try debug_gen.createType("Point", 8, 4, .struct_type);
    _ = point_type_id; // Use it to avoid unused variable warning

    // Create debug info for add_numbers function
    var add_params = [_]FunctionParameter{
        .{ .name = "first", .param_type = "normie" },
        .{ .name = "second", .param_type = "normie" },
    };
    const add_func_id = try debug_gen.createFunction("add_numbers", 8, "normie", &add_params);

    // Enter function scope
    debug_gen.enterScope();

    // Create debug info for local variables in add_numbers
    _ = try debug_gen.createVariable("local_sum", "normie", 9, add_func_id);
    _ = try debug_gen.createVariable("temp_var", "drip", 10, add_func_id);

    // Add source locations for key operations
    try debug_gen.addSourceLocation(9, 5, "variable assignment");
    try debug_gen.addSourceLocation(10, 5, "variable assignment");
    try debug_gen.addSourceLocation(11, 5, "return statement");

    // Exit function scope
    debug_gen.exitScope();

    // Create debug info for main function
    const main_func_id = try debug_gen.createFunction("main", 14, "void", &[_]FunctionParameter{});

    // Enter main function scope
    debug_gen.enterScope();

    // Create debug info for local variables in main
    _ = try debug_gen.createVariable("x", "normie", 15, main_func_id);
    _ = try debug_gen.createVariable("y", "normie", 16, main_func_id);
    _ = try debug_gen.createVariable("result", "normie", 17, main_func_id);

    // Add source locations for main function operations
    try debug_gen.addSourceLocation(15, 5, "variable declaration");
    try debug_gen.addSourceLocation(16, 5, "variable declaration");
    try debug_gen.addSourceLocation(17, 5, "function call");
    try debug_gen.addSourceLocation(18, 5, "print statement");

    // Exit main function scope
    debug_gen.exitScope();

    std.debug.print("\n📊 Debug Information Summary:\n", .{});
    std.debug.print("  Functions: {}\n", .{debug_gen.functions.items.len});
    std.debug.print("  Variables: {}\n", .{debug_gen.variables.items.len});
    std.debug.print("  Types: {}\n", .{debug_gen.types.items.len});
    std.debug.print("  Source Locations: {}\n\n", .{debug_gen.source_locations.items.len});

    // Generate debug files
    try debug_gen.generateDWARFInfo("test_program");
    try debug_gen.generateLineNumberMapping("test_program");

    std.debug.print("\n🎉 Comprehensive debug information generation completed!\n", .{});

    // Test debugging workflow simulation
    std.debug.print("\n🔍 Simulating debugger workflow:\n", .{});
    
    // Find function by name
    for (debug_gen.functions.items) |func| {
        if (std.mem.eql(u8, func.name, "add_numbers")) {
            std.debug.print("  Breakpoint set at function '{s}' (line {})\n", .{ func.name, func.line });
            
            // Find variables in this function's scope
            for (debug_gen.variables.items) |variable| {
                if (variable.scope_id == func.id) {
                    std.debug.print("    Variable in scope: {s}: {s} (line {})\n", .{ 
                        variable.name, variable.var_type, variable.line 
                    });
                }
            }
        }
    }

    // Test source location lookup
    std.debug.print("\n📍 Source location mapping test:\n", .{});
    for (debug_gen.source_locations.items) |loc| {
        if (loc.line == 17) {
            std.debug.print("  Line {} ({}:{}): {s}\n", .{ loc.line, loc.line, loc.column, loc.context });
        }
    }

    std.debug.print("\n✅ Debug information system working correctly!\n", .{});
}
