const std = @import("std");
const print = std.debug.print;
const DebugInfoGenerator = @import("src-zig/debug_info.zig").DebugInfoGenerator;
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/DebugInfo.h");
    @cInclude("llvm-c/DIBuilder.h");
});

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize LLVM
    c.LLVMInitializeCore(c.LLVMGetGlobalPassRegistry());
    c.LLVMInitializeAllTargets();
    c.LLVMInitializeAllTargetInfos();
    c.LLVMInitializeAllAsmParsers();
    c.LLVMInitializeAllAsmPrinters();

    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);

    const module = c.LLVMModuleCreateWithNameInContext("debug_test", context);
    defer c.LLVMDisposeModule(module);

    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);

    print("🔧 Testing CURSED debug information generation...\n");

    // Create debug info generator
    var debug_gen = try DebugInfoGenerator.init(allocator, context, module);
    defer debug_gen.deinit();

    // Create compile unit
    try debug_gen.createCompileUnit("test_debug.csd", "/tmp");
    print("✅ Created debug compile unit\n");

    // Create CURSED types
    const cursed_types = try debug_gen.createCursedTypes();
    print("✅ Created CURSED debug types\n");

    // Create a simple function with debug info
    const int_type = c.LLVMInt64TypeInContext(context);
    var param_types = [_]c.LLVMTypeRef{int_type};
    const func_type = c.LLVMFunctionType(int_type, &param_types, 1, 0);
    const function = c.LLVMAddFunction(module, "test_function", func_type);

    // Create function debug info
    var debug_param_types = [_]c.LLVMMetadataRef{cursed_types.drip_type};
    const func_di_type = try debug_gen.createFunctionType(cursed_types.drip_type, &debug_param_types);
    _ = try debug_gen.createFunction("test_function", "test_function", 1, func_di_type, function);
    print("✅ Created function debug info\n");

    // Create basic block
    const entry_block = c.LLVMAppendBasicBlockInContext(context, function, "entry");
    c.LLVMPositionBuilderAtEnd(builder, entry_block);

    // Create local variable
    const alloca = c.LLVMBuildAlloca(builder, int_type, "local_var");
    try debug_gen.createLocalVariable("local_var", 2, cursed_types.drip_type, alloca);
    print("✅ Created local variable debug info\n");

    // Store value with debug location
    const store_inst = c.LLVMBuildStore(builder, c.LLVMConstInt(int_type, 42, 0), alloca);
    debug_gen.setDebugLocation(store_inst, 2, 5);

    // Load and return with debug location
    const load_inst = c.LLVMBuildLoad2(builder, int_type, alloca, "load_local");
    debug_gen.setDebugLocation(load_inst, 3, 5);

    const ret_inst = c.LLVMBuildRet(builder, load_inst);
    debug_gen.setDebugLocation(ret_inst, 3, 5);

    print("✅ Set debug locations on instructions\n");

    // Finalize debug info
    debug_gen.finalize();
    print("✅ Finalized debug information\n");

    // Verify module
    var error_message: [*c]u8 = null;
    if (c.LLVMVerifyModule(module, c.LLVMReturnStatusAction, &error_message) != 0) {
        if (error_message) |msg| {
            print("❌ Module verification failed: {s}\n", .{msg});
            c.LLVMDisposeMessage(msg);
            return;
        }
    }
    print("✅ Module verification passed\n");

    // Write LLVM IR to file with debug info
    if (c.LLVMWriteBitcodeToFile(module, "debug_test.bc") != 0) {
        print("❌ Failed to write bitcode\n");
        return;
    }
    print("✅ Wrote LLVM IR bitcode with debug info to debug_test.bc\n");

    // Print LLVM IR to console
    const ir_string = c.LLVMPrintModuleToString(module);
    defer c.LLVMDisposeMessage(ir_string);
    print("\n📄 Generated LLVM IR with debug information:\n");
    print("{s}\n", .{ir_string});

    print("🎉 Debug information generation test completed successfully!\n");
}
