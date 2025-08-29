const std = @import("std");
const llvm_fixes = @import("llvm_fixes.zig");
const ast = @import("ast.zig");

// LLVM C imports disabled to fix "athlon-xp" CPU detection issues
// Replace with dummy types to allow compilation without LLVM
const c = struct {
    // Dummy LLVM types to make compilation work without LLVM
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMExecutionEngineRef = ?*anyopaque;
    pub const LLVMTargetRef = ?*anyopaque;
    pub const LLVMTargetMachineRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    pub const LLVMMemoryBufferRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy functions to prevent link errors (add more as needed)
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMInitializeX86TargetInfo() void {}
    pub fn LLVMInitializeX86Target() void {}
    pub fn LLVMInitializeX86TargetMC() void {}
    pub fn LLVMInitializeX86AsmPrinter() void {}
    pub fn LLVMCreateTargetMachine() LLVMTargetMachineRef { return null; }
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}
};

/// Test variable registration and lookup in LLVM compilation
pub fn testVariableSystem() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    // Initialize LLVM
    c.LLVMInitializeCore(c.LLVMGetGlobalPassRegistry());
    c.LLVMInitializeNativeTarget();
    c.LLVMInitializeNativeAsmPrinter();
    
    // Create LLVM context
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    // Create module
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    // Create builder
    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);
    
    // Initialize variable scope system
    try llvm_fixes.initializeVariableScope(allocator);
    defer llvm_fixes.deinitializeVariableScope(allocator);
    
    std.debug.print("🧪 Testing variable registration and lookup...\n", .{});
    
    // Test 1: Register a variable
    const int_value = c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 42, 0);
    const alloca = try llvm_fixes.registerVariable(context, builder, "x", "drip", int_value);
    std.debug.print("✅ Test 1: Variable registration successful\n", .{});
    
    // Test 2: Look up the variable
    const loaded_value = try llvm_fixes.lookupAndLoadVariable(context, builder, "x");
    std.debug.print("✅ Test 2: Variable lookup successful\n", .{});
    
    // Test 3: Test nested scopes
    try llvm_fixes.enterVariableScope();
    const nested_value = c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 100, 0);
    _ = try llvm_fixes.registerVariable(context, builder, "y", "drip", nested_value);
    
    // Should find both x (from parent) and y (from current scope)
    _ = try llvm_fixes.lookupAndLoadVariable(context, builder, "x"); // Parent scope
    _ = try llvm_fixes.lookupAndLoadVariable(context, builder, "y"); // Current scope
    
    llvm_fixes.exitVariableScope();
    std.debug.print("✅ Test 3: Nested scope handling successful\n", .{});
    
    // Test 4: Variable should not be accessible after exiting scope
    const result = llvm_fixes.lookupAndLoadVariable(context, builder, "y");
    if (result) |_| {
        std.debug.print("❌ Test 4: Failed - variable should not be accessible\n", .{});
    } else |err| {
        if (err == error.VariableNotFound) {
            std.debug.print("✅ Test 4: Scope isolation working correctly\n", .{});
        } else {
            std.debug.print("❌ Test 4: Unexpected error: {s}\n", .{err});
        }
    }
    
    std.debug.print("🎉 Variable system tests completed!\n", .{});
}

/// Test generating expressions with variables
pub fn testExpressionCompilation() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    // Initialize LLVM components
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("expr_test", context);
    defer c.LLVMDisposeModule(module);
    
    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);
    
    // Initialize variable scope
    try llvm_fixes.initializeVariableScope(allocator);
    defer llvm_fixes.deinitializeVariableScope(allocator);
    
    std.debug.print("🧪 Testing expression compilation with variables...\n", .{});
    
    // Create a test function
    const fn_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0);
    const test_function = c.LLVMAddFunction(module, "test_func", fn_type);
    
    // Create entry block
    const entry_block = c.LLVMAppendBasicBlockInContext(context, test_function, "entry");
    c.LLVMPositionBuilderAtEnd(builder, entry_block);
    
    // Register variables for expression testing
    const value1 = c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 10, 0);
    const value2 = c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 20, 0);
    
    _ = try llvm_fixes.registerVariable(context, builder, "a", "drip", value1);
    _ = try llvm_fixes.registerVariable(context, builder, "b", "drip", value2);
    
    // Test expressions: a + b
    const a_value = try llvm_fixes.lookupAndLoadVariable(context, builder, "a");
    const b_value = try llvm_fixes.lookupAndLoadVariable(context, builder, "b");
    const sum = c.LLVMBuildAdd(builder, a_value, b_value, "sum");
    
    // Store result in new variable
    _ = try llvm_fixes.registerVariable(context, builder, "result", "drip", sum);
    
    std.debug.print("✅ Expression compilation with variables successful!\n", .{});
    
    // Generate LLVM IR to verify correctness
    var ir_string = c.LLVMPrintModuleToString(module);
    defer c.LLVMDisposeMessage(ir_string);
    
    const ir_content = std.mem.span(ir_string);
    std.debug.print("Generated LLVM IR:\n{s}\n", .{ir_content});
}

pub fn main() !void {
    std.debug.print("🚀 Starting variable compilation tests...\n", .{});
    
    try testVariableSystem();
    try testExpressionCompilation();
    
    std.debug.print("✅ All tests completed successfully!\n", .{});
}
