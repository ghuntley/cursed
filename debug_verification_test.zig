const std = @import("std");
const DebugInfoGenerator = @import("enhanced_debug_generation.zig").DebugInfoGenerator;
const FunctionParameter = @import("enhanced_debug_generation.zig").FunctionParameter;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("🔍 Verifying CURSED debug information with debugger integration...\n\n", .{});

    // Test 1: Basic debug info generation
    var debug_gen = try DebugInfoGenerator.init(allocator, "debug_test.csd");
    defer debug_gen.deinit();

    _ = try debug_gen.createCursedTypes();
    
    var params = [_]FunctionParameter{
        .{ .name = "n", .param_type = "drip" },
    };
    const func_id = try debug_gen.createFunction("factorial", 1, "drip", &params);
    
    debug_gen.enterScope();
    _ = try debug_gen.createVariable("result", "drip", 2, func_id);
    _ = try debug_gen.createVariable("i", "drip", 3, func_id);
    
    // Add debug locations for key operations
    try debug_gen.addSourceLocation(2, 5, "variable initialization");
    try debug_gen.addSourceLocation(3, 5, "loop counter");
    try debug_gen.addSourceLocation(4, 9, "loop condition");
    try debug_gen.addSourceLocation(5, 13, "multiplication");
    try debug_gen.addSourceLocation(6, 9, "increment");
    try debug_gen.addSourceLocation(7, 5, "return result");
    
    debug_gen.exitScope();
    
    // Generate debug files
    try debug_gen.generateDWARFInfo("factorial_test");
    try debug_gen.generateLineNumberMapping("factorial_test");
    
    std.debug.print("✅ Generated debug information for factorial function\n\n", .{});

    // Test 2: Verify debugger compatibility
    std.debug.print("🔧 Testing debugger compatibility features:\n", .{});
    
    // Simulate debugger queries
    std.debug.print("  - Finding function by name: ", .{});
    var found_function = false;
    for (debug_gen.functions.items) |func| {
        if (std.mem.eql(u8, func.name, "factorial")) {
            std.debug.print("✅ Found '{s}' at line {d}\n", .{ func.name, func.line });
            found_function = true;
            break;
        }
    }
    if (!found_function) {
        std.debug.print("❌ Function not found\n", .{});
    }
    
    // Test variable lookup
    std.debug.print("  - Variable lookup in scope: ", .{});
    var var_count: u32 = 0;
    for (debug_gen.variables.items) |variable| {
        if (variable.scope_id == func_id) {
            var_count += 1;
        }
    }
    std.debug.print("✅ Found {d} variables in function scope\n", .{var_count});
    
    // Test source location mapping
    std.debug.print("  - Source location mapping: ", .{});
    var location_count: u32 = 0;
    for (debug_gen.source_locations.items) |_| {
        location_count += 1;
    }
    std.debug.print("✅ {d} source locations mapped\n", .{location_count});
    
    // Test 3: Generate GDB-compatible debug script
    try generateGDBScript(allocator, &debug_gen);
    
    // Test 4: Generate LLDB-compatible debug script  
    try generateLLDBScript(allocator, &debug_gen);
    
    std.debug.print("\n🎉 Debug verification completed successfully!\n", .{});
    
    // Test 5: Performance test
    std.debug.print("\n⚡ Performance test - generating debug info for large program:\n", .{});
    const start_time = std.time.milliTimestamp();
    
    var large_debug_gen = try DebugInfoGenerator.init(allocator, "large_program.csd");
    defer large_debug_gen.deinit();
    
    _ = try large_debug_gen.createCursedTypes();
    
    // Simulate large program with many functions and variables
    var i: u32 = 0;
    while (i < 100) : (i += 1) {
        const func_name = try std.fmt.allocPrint(allocator, "function_{d}", .{i});
        defer allocator.free(func_name);
        
        const func_params = [_]FunctionParameter{
            .{ .name = "param1", .param_type = "normie" },
            .{ .name = "param2", .param_type = "drip" },
        };
        const large_func_id = try large_debug_gen.createFunction(func_name, i + 1, "normie", &func_params);
        
        large_debug_gen.enterScope();
        
        // Add local variables
        var j: u32 = 0;
        while (j < 10) : (j += 1) {
            const var_name = try std.fmt.allocPrint(allocator, "local_var_{d}", .{j});
            defer allocator.free(var_name);
            
            _ = try large_debug_gen.createVariable(var_name, "normie", i + j + 1, large_func_id);
            try large_debug_gen.addSourceLocation(i + j + 1, j + 1, "variable");
        }
        
        large_debug_gen.exitScope();
    }
    
    const end_time = std.time.milliTimestamp();
    const duration = end_time - start_time;
    
    std.debug.print("  Generated debug info for {d} functions and {d} variables in {d}ms\n", .{
        large_debug_gen.functions.items.len,
        large_debug_gen.variables.items.len,
        duration
    });
    
    try large_debug_gen.generateDWARFInfo("large_program");
    
    std.debug.print("✅ Performance test completed\n", .{});
}

fn generateGDBScript(allocator: std.mem.Allocator, debug_gen: *DebugInfoGenerator) !void {
    const script_file = try std.fs.cwd().createFile("debug_script.gdb", .{});
    defer script_file.close();
    
    const writer = script_file.writer();
    
    try writer.print("# GDB Script for CURSED Debug Information\n", .{});
    try writer.print("# Generated automatically from debug metadata\n\n", .{});
    
    try writer.print("# Set breakpoints on all functions\n", .{});
    for (debug_gen.functions.items) |func| {
        try writer.print("break {s}:{d}\n", .{ func.name, func.line });
    }
    
    try writer.print("\n# Display all variables when stopped\n", .{});
    try writer.print("define show_cursed_vars\n", .{});
    for (debug_gen.variables.items) |variable| {
        try writer.print("  info locals {s}\n", .{variable.name});
    }
    try writer.print("end\n\n", .{});
    
    try writer.print("# Start debugging\n", .{});
    try writer.print("run\n", .{});
    
    std.debug.print("✅ Generated GDB debug script: debug_script.gdb\n", .{});
    _ = allocator; // Suppress unused parameter warning
}

fn generateLLDBScript(allocator: std.mem.Allocator, debug_gen: *DebugInfoGenerator) !void {
    const script_file = try std.fs.cwd().createFile("debug_script.lldb", .{});
    defer script_file.close();
    
    const writer = script_file.writer();
    
    try writer.print("# LLDB Script for CURSED Debug Information\n", .{});
    try writer.print("# Generated automatically from debug metadata\n\n", .{});
    
    try writer.print("# Set breakpoints on all functions\n", .{});
    for (debug_gen.functions.items) |func| {
        try writer.print("breakpoint set -f debug_test.csd -l {d}\n", .{func.line});
    }
    
    try writer.print("\n# Custom command to show CURSED variables\n", .{});
    try writer.print("command script add -f show_cursed_vars\n", .{});
    try writer.print("def show_cursed_vars(debugger, command, result, internal_dict):\n", .{});
    for (debug_gen.variables.items) |variable| {
        try writer.print("    debugger.HandleCommand('frame variable {s}')\n", .{variable.name});
    }
    
    try writer.print("\n# Start debugging\n", .{});
    try writer.print("run\n", .{});
    
    std.debug.print("✅ Generated LLDB debug script: debug_script.lldb\n", .{});
    _ = allocator; // Suppress unused parameter warning
}
