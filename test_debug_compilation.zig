const std = @import("std");
const debug_info = @import("src-zig/debug_info.zig");
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/DebugInfo.h");
});

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize LLVM
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);

    const module = c.LLVMModuleCreateWithNameInContext("test_debug", context);
    defer c.LLVMDisposeModule(module);

    // Create debug info generator
    var debug_gen = try debug_info.DebugInfoGenerator.init(allocator, context, module);
    defer debug_gen.deinit();

    // Create compilation unit
    try debug_gen.createCompileUnit("test.csd", ".");

    // Create basic types
    const cursed_types = try debug_gen.createCursedTypes();

    // Create a simple function with debug info
    const func_type = c.LLVMFunctionType(c.LLVMInt32Type(), null, 0, 0);
    const func = c.LLVMAddFunction(module, "test_function", func_type);

    // Create function debug info
    const func_debug_type = try debug_gen.createFunctionType(cursed_types.normie_type, &[_]c.LLVMMetadataRef{});
    _ = try debug_gen.createFunction("test_function", "test_function", 1, func_debug_type, func);

    // Finalize debug info
    debug_gen.finalize();

    // Dump the module to see if debug info is present
    c.LLVMDumpModule(module);

    std.debug.print("Debug info test completed!\n");
}
