const std = @import("std");

// Real LLVM C API integration using the C wrapper functions
extern fn llvm_initialize_core() void;
extern fn llvm_create_context() ?*anyopaque;
extern fn llvm_dispose_context(?*anyopaque) void;
extern fn llvm_create_module(?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_dispose_module(?*anyopaque) void;
extern fn llvm_create_builder(?*anyopaque) ?*anyopaque;
extern fn llvm_dispose_builder(?*anyopaque) void;
extern fn llvm_int32_type(?*anyopaque) ?*anyopaque;
extern fn llvm_int8_type(?*anyopaque) ?*anyopaque;
extern fn llvm_pointer_type(?*anyopaque) ?*anyopaque;
extern fn llvm_function_type(?*anyopaque, ?[*]?*anyopaque, c_int, c_int) ?*anyopaque;
extern fn llvm_add_function(?*anyopaque, [*c]const u8, ?*anyopaque) ?*anyopaque;
extern fn llvm_append_basic_block(?*anyopaque, ?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_position_builder_at_end(?*anyopaque, ?*anyopaque) void;
extern fn llvm_build_global_string_ptr(?*anyopaque, [*c]const u8, [*c]const u8) ?*anyopaque;
extern fn llvm_const_int(?*anyopaque, c_ulonglong) ?*anyopaque;
extern fn llvm_build_ret(?*anyopaque, ?*anyopaque) ?*anyopaque;
extern fn llvm_get_named_function(?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_get_function_type(?*anyopaque) ?*anyopaque;
extern fn llvm_build_call2(?*anyopaque, ?*anyopaque, ?*anyopaque, ?[*]?*anyopaque, c_int, [*c]const u8) ?*anyopaque;
extern fn llvm_verify_module(?*anyopaque) c_int;
extern fn llvm_print_module_to_string(?*anyopaque) [*c]u8;
extern fn llvm_dispose_message([*c]u8) void;
extern fn llvm_write_bitcode_to_file(?*anyopaque, [*c]const u8) c_int;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    std.debug.print("🚀 Starting LLVM Backend Test\n", .{});

    // Initialize LLVM core
    llvm_initialize_core();
    std.debug.print("✅ LLVM core initialized\n", .{});

    const context = llvm_create_context() orelse {
        std.debug.print("❌ Failed to create LLVM context\n", .{});
        return;
    };
    defer llvm_dispose_context(context);
    std.debug.print("✅ LLVM context created\n", .{});

    const module = llvm_create_module(context, "cursed_test_module") orelse {
        std.debug.print("❌ Failed to create LLVM module\n", .{});
        return;
    };
    defer llvm_dispose_module(module);
    std.debug.print("✅ LLVM module created\n", .{});

    const builder = llvm_create_builder(context) orelse {
        std.debug.print("❌ Failed to create LLVM builder\n", .{});
        return;
    };
    defer llvm_dispose_builder(builder);
    std.debug.print("✅ LLVM builder created\n", .{});

    // Create basic types
    const i32_type = llvm_int32_type(context);
    const i8_type = llvm_int8_type(context);
    const i8_ptr_type = llvm_pointer_type(i8_type);
    std.debug.print("✅ Basic types created\n", .{});

    // Create printf function declaration
    var printf_params = [_]?*anyopaque{i8_ptr_type};
    const printf_type = llvm_function_type(i32_type, &printf_params, 1, 1); // variadic
    const printf_func = llvm_add_function(module, "printf", printf_type);
    std.debug.print("✅ printf function declared\n", .{});

    // Create main function
    const main_type = llvm_function_type(i32_type, null, 0, 0);
    const main_func = llvm_add_function(module, "main", main_type);
    std.debug.print("✅ main function created\n", .{});

    // Create entry block
    const entry_block = llvm_append_basic_block(context, main_func, "entry");
    llvm_position_builder_at_end(builder, entry_block);
    std.debug.print("✅ Entry block created\n", .{});

    // Create a simple CURSED program: vibez.spill("Answer:", 42)
    const format_str = llvm_build_global_string_ptr(builder, "Answer: %d\n", "fmt");
    const answer_value = llvm_const_int(i32_type, 42);
    
    var args = [_]?*anyopaque{ format_str, answer_value };
    const printf_func_type = llvm_get_function_type(printf_func);
    _ = llvm_build_call2(builder, printf_func_type, printf_func, &args, 2, "printf_call");
    std.debug.print("✅ printf call generated\n", .{});

    // Return 0
    const return_value = llvm_const_int(i32_type, 0);
    _ = llvm_build_ret(builder, return_value);
    std.debug.print("✅ Return statement generated\n", .{});

    // Verify the module
    if (llvm_verify_module(module) != 0) {
        std.debug.print("❌ LLVM module verification failed\n", .{});
        return;
    }
    std.debug.print("✅ LLVM module verified successfully\n", .{});

    // Print the generated LLVM IR
    const module_str = llvm_print_module_to_string(module);
    if (module_str) |str| {
        std.debug.print("\n📄 Generated LLVM IR:\n{s}\n", .{str});
        llvm_dispose_message(str);
    }

    // Write bitcode to file
    const output_file = "simple_test.bc";
    const filename_c = try allocator.dupeZ(u8, output_file);
    defer allocator.free(filename_c);
    
    if (llvm_write_bitcode_to_file(module, filename_c.ptr) != 0) {
        std.debug.print("❌ Failed to write bitcode file\n", .{});
        return;
    }

    std.debug.print("✅ Bitcode written to {s}\n", .{output_file});
    std.debug.print("🎉 LLVM Backend Test Completed Successfully!\n", .{});
    
    // Show how to compile and run the generated code
    std.debug.print("\n📋 To compile and run the generated code:\n", .{});
    std.debug.print("   llc simple_test.bc -o simple_test.s\n", .{});
    std.debug.print("   gcc simple_test.s -o simple_test\n", .{});
    std.debug.print("   ./simple_test\n", .{});
}
