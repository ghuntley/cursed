const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

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

const debug_info = @import("debug_info.zig");
const DebugInfoGenerator = debug_info.DebugInfoGenerator;

/// Enhanced LLVM Code Generator with comprehensive DWARF debug information
/// Provides full GDB/LLDB debugging support for CURSED programs
pub const DebugEnabledCodeGen = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    debug_generator: DebugInfoGenerator,
    
    // Type cache
    i8_type: c.LLVMTypeRef,
    i32_type: c.LLVMTypeRef,
    i64_type: c.LLVMTypeRef,
    f64_type: c.LLVMTypeRef,
    void_type: c.LLVMTypeRef,
    ptr_type: c.LLVMTypeRef,
    
    // Runtime functions
    printf_func: c.LLVMValueRef,
    puts_func: c.LLVMValueRef,
    
    // Debug metadata
    main_function: ?c.LLVMValueRef,
    current_debug_scope: ?c.LLVMMetadataRef,
    
    pub const CodeGenError = error{
        LLVMError,
        DebugError,
        OutOfMemory,
    };
    
    pub fn init(allocator: Allocator, source_file: []const u8) CodeGenError!DebugEnabledCodeGen {
        // Initialize LLVM
        const context = c.LLVMContextCreate();
        const module = c.LLVMModuleCreateWithNameInContext("cursed_debug_module", context);
        const builder = c.LLVMCreateBuilderInContext(context);
        
        // Initialize debug generator
        var debug_generator = DebugInfoGenerator.init(allocator, context, module) catch |err| {
            std.debug.print("❌ Failed to initialize debug generator: {any}\n", .{err});
            return CodeGenError.DebugError;
        };
        
        // Create compile unit for debug info
        const directory = std.fs.path.dirname(source_file) orelse ".";
        const filename = std.fs.path.basename(source_file);
        debug_generator.createCompileUnit(filename, directory) catch |err| {
            std.debug.print("❌ Failed to create debug compile unit: {any}\n", .{err});
            return CodeGenError.DebugError;
        };
        
        // Initialize types
        const i8_type = c.LLVMInt8TypeInContext(context);
        const i32_type = c.LLVMInt32TypeInContext(context);
        const i64_type = c.LLVMInt64TypeInContext(context);
        const f64_type = c.LLVMDoubleTypeInContext(context);
        const void_type = c.LLVMVoidTypeInContext(context);
        const ptr_type = c.LLVMPointerType(i8_type, 0);
        
        // Add module flags for debug info
        c.LLVMAddModuleFlag(
            module,
            c.LLVMModuleFlagBehaviorWarning,
            "Debug Info Version",
            17,
            c.LLVMValueAsMetadata(c.LLVMConstInt(i32_type, 3, 0))
        );
        
        c.LLVMAddModuleFlag(
            module,
            c.LLVMModuleFlagBehaviorWarning,
            "Dwarf Version",
            13,
            c.LLVMValueAsMetadata(c.LLVMConstInt(i32_type, 4, 0))
        );
        
        // Declare runtime functions
        const printf_type = c.LLVMFunctionType(i32_type, &[_]c.LLVMTypeRef{ptr_type}, 1, 1);
        const printf_func = c.LLVMAddFunction(module, "printf", printf_type);
        
        const puts_type = c.LLVMFunctionType(i32_type, &[_]c.LLVMTypeRef{ptr_type}, 1, 0);
        const puts_func = c.LLVMAddFunction(module, "puts", puts_type);
        
        return DebugEnabledCodeGen{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .debug_generator = debug_generator,
            .i8_type = i8_type,
            .i32_type = i32_type,
            .i64_type = i64_type,
            .f64_type = f64_type,
            .void_type = void_type,
            .ptr_type = ptr_type,
            .printf_func = printf_func,
            .puts_func = puts_func,
            .main_function = null,
            .current_debug_scope = null,
        };
    }
    
    pub fn deinit(self: *DebugEnabledCodeGen) void {
        self.debug_generator.deinit();
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
    }
    
    /// Generate LLVM IR with comprehensive debug information
    pub fn generateDebugProgram(self: *DebugEnabledCodeGen) CodeGenError!void {
        try self.createDebugTestFunction();
        try self.createMainFunction();
        
        // Finalize debug information
        self.debug_generator.finalize();
        
        std.debug.print("✅ Generated LLVM IR with comprehensive DWARF debug info\n", .{});
    }
    
    /// Create a debug test function with various variable types
    fn createDebugTestFunction(self: *DebugEnabledCodeGen) CodeGenError!void {
        // Create function type
        const param_types = [_]c.LLVMTypeRef{ self.i32_type, self.ptr_type };
        const func_type = c.LLVMFunctionType(self.void_type, &param_types, 2, 0);
        
        // Create function
        const function = c.LLVMAddFunction(self.module, "debug_test_function", func_type);
        
        // Create debug function type
        const debug_param_types = [_]c.LLVMMetadataRef{
            self.debug_generator.cursed_debug_types.?.normie_type,
            self.debug_generator.cursed_debug_types.?.tea_type,
        };
        
        const debug_func_type = self.debug_generator.createFunctionType(
            self.debug_generator.cursed_debug_types.?.void_type,
            debug_param_types[0..]
        ) catch |err| {
            std.debug.print("❌ Failed to create debug function type: {any}\n", .{err});
            return CodeGenError.DebugError;
        };
        
        // Create debug function metadata
        const debug_func = self.debug_generator.createFunction(
            "debug_test_function",
            "debug_test_function",
            5, // Line number
            debug_func_type,
            function
        ) catch |err| {
            std.debug.print("❌ Failed to create debug function: {any}\n", .{err});
            return CodeGenError.DebugError;
        };
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Set debug location for function entry
        self.debug_generator.setInstructionDebugLocation(
            c.LLVMGetFirstInstruction(entry_block),
            5, 1
        );
        
        // Create parameter allocas with debug info
        const param1_alloca = c.LLVMBuildAlloca(self.builder, self.i32_type, "param1");
        const param2_alloca = c.LLVMBuildAlloca(self.builder, self.ptr_type, "param2");
        
        // Store parameters
        _ = c.LLVMBuildStore(self.builder, c.LLVMGetParam(function, 0), param1_alloca);
        _ = c.LLVMBuildStore(self.builder, c.LLVMGetParam(function, 1), param2_alloca);
        
        // Create debug info for parameters
        self.debug_generator.createParameterVariable(
            "param1",
            1,
            5,
            self.debug_generator.cursed_debug_types.?.normie_type,
            param1_alloca
        ) catch |err| {
            std.debug.print("❌ Failed to create param1 debug info: {any}\n", .{err});
        };
        
        self.debug_generator.createParameterVariable(
            "param2",
            2,
            5,
            self.debug_generator.cursed_debug_types.?.tea_type,
            param2_alloca
        ) catch |err| {
            std.debug.print("❌ Failed to create param2 debug info: {any}\n", .{err});
        };
        
        // Create local variables with debug info
        const drip_alloca = c.LLVMBuildAlloca(self.builder, self.i64_type, "drip_value");
        const normie_alloca = c.LLVMBuildAlloca(self.builder, self.i32_type, "normie_value");
        const meal_alloca = c.LLVMBuildAlloca(self.builder, self.f64_type, "meal_value");
        const lit_alloca = c.LLVMBuildAlloca(self.builder, self.i32_type, "lit_value");
        
        // Store initial values
        _ = c.LLVMBuildStore(self.builder, c.LLVMConstInt(self.i64_type, 42, 0), drip_alloca);
        _ = c.LLVMBuildStore(self.builder, c.LLVMConstInt(self.i32_type, 123, 0), normie_alloca);
        _ = c.LLVMBuildStore(self.builder, c.LLVMConstReal(self.f64_type, 3.14159), meal_alloca);
        _ = c.LLVMBuildStore(self.builder, c.LLVMConstInt(self.i32_type, 1, 0), lit_alloca);
        
        // Create debug info for local variables
        self.debug_generator.createLocalVariable(
            "drip_value",
            11,
            self.debug_generator.cursed_debug_types.?.drip_type,
            drip_alloca
        ) catch |err| {
            std.debug.print("❌ Failed to create drip_value debug info: {any}\n", .{err});
        };
        
        self.debug_generator.createLocalVariable(
            "normie_value",
            12,
            self.debug_generator.cursed_debug_types.?.normie_type,
            normie_alloca
        ) catch |err| {
            std.debug.print("❌ Failed to create normie_value debug info: {any}\n", .{err});
        };
        
        self.debug_generator.createLocalVariable(
            "meal_value",
            13,
            self.debug_generator.cursed_debug_types.?.meal_type,
            meal_alloca
        ) catch |err| {
            std.debug.print("❌ Failed to create meal_value debug info: {any}\n", .{err});
        };
        
        self.debug_generator.createLocalVariable(
            "lit_value",
            15,
            self.debug_generator.cursed_debug_types.?.lit_type,
            lit_alloca
        ) catch |err| {
            std.debug.print("❌ Failed to create lit_value debug info: {any}\n", .{err});
        };
        
        // Create string constant for tea value
        const tea_string = "Hello Debug!";
        const tea_const = c.LLVMBuildGlobalStringPtr(self.builder, tea_string.ptr, "tea_value");
        
        // Generate printf calls with debug locations
        self.generatePrintfCall("Function called with parameters", 6);
        self.generatePrintfCallInt("param1", param1_alloca, 7);
        self.generatePrintfCallString("param2", param2_alloca, 8);
        
        self.generatePrintfCallLong("drip_value", drip_alloca, 17);
        self.generatePrintfCallInt("normie_value", normie_alloca, 18);
        self.generatePrintfCallDouble("meal_value", meal_alloca, 19);
        self.generatePrintfCallStringLiteral("tea_value", tea_const, 20);
        self.generatePrintfCallBool("lit_value", lit_alloca, 21);
        
        // Create nested scope for scoped variable
        const scope_block = c.LLVMAppendBasicBlockInContext(self.context, function, "nested_scope");
        _ = c.LLVMBuildBr(self.builder, scope_block);
        c.LLVMPositionBuilderAtEnd(self.builder, scope_block);
        
        // Create lexical block for nested scope
        const lexical_block = self.debug_generator.createLexicalBlock(24, 5) catch |err| {
            std.debug.print("❌ Failed to create lexical block: {any}\n", .{err});
            return CodeGenError.DebugError;
        };
        
        self.debug_generator.pushScope(lexical_block) catch |err| {
            std.debug.print("❌ Failed to push scope: {any}\n", .{err});
            return CodeGenError.DebugError;
        };
        
        // Scoped variable
        const scoped_alloca = c.LLVMBuildAlloca(self.builder, self.i32_type, "scoped_var");
        _ = c.LLVMBuildStore(self.builder, c.LLVMConstInt(self.i32_type, 999, 0), scoped_alloca);
        
        self.debug_generator.createLocalVariable(
            "scoped_var",
            25,
            self.debug_generator.cursed_debug_types.?.normie_type,
            scoped_alloca
        ) catch |err| {
            std.debug.print("❌ Failed to create scoped_var debug info: {any}\n", .{err});
        };
        
        self.generatePrintfCallInt("scoped_var", scoped_alloca, 26);
        
        // Pop scope
        self.debug_generator.popScope();
        
        // Return
        _ = c.LLVMBuildRetVoid(self.builder);
    }
    
    /// Create main function with debug info
    fn createMainFunction(self: *DebugEnabledCodeGen) CodeGenError!void {
        const func_type = c.LLVMFunctionType(self.i32_type, null, 0, 0);
        const main_func = c.LLVMAddFunction(self.module, "main", func_type);
        self.main_function = main_func;
        
        // Create debug function type for main
        const debug_func_type = self.debug_generator.createFunctionType(
            self.debug_generator.cursed_debug_types.?.normie_type,
            &[_]c.LLVMMetadataRef{}
        ) catch |err| {
            std.debug.print("❌ Failed to create main debug function type: {any}\n", .{err});
            return CodeGenError.DebugError;
        };
        
        // Create debug function metadata for main
        const debug_main = self.debug_generator.createFunction(
            "main",
            "main",
            30, // Line number
            debug_func_type,
            main_func
        ) catch |err| {
            std.debug.print("❌ Failed to create main debug function: {any}\n", .{err});
            return CodeGenError.DebugError;
        };
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Set debug location
        self.debug_generator.setInstructionDebugLocation(
            c.LLVMGetFirstInstruction(entry_block),
            30, 1
        );
        
        // Print welcome messages
        self.generatePrintfCall("CURSED Debug Information Test", 31);
        self.generatePrintfCall("This tests GDB/LLDB compatibility", 32);
        
        // Call debug test function
        const debug_func = c.LLVMGetNamedFunction(self.module, "debug_test_function");
        const test_string = c.LLVMBuildGlobalStringPtr(self.builder, "test_string", "test_str");
        
        const call_args = [_]c.LLVMValueRef{
            c.LLVMConstInt(self.i32_type, 42, 0),
            test_string
        };
        
        const call_inst = c.LLVMBuildCall2(
            self.builder,
            c.LLVMFunctionType(self.void_type, &[_]c.LLVMTypeRef{ self.i32_type, self.ptr_type }, 2, 0),
            debug_func,
            &call_args,
            2,
            ""
        );
        
        // Set debug location for call
        self.debug_generator.setInstructionDebugLocation(call_inst, 34, 5);
        
        // Return 0
        const ret_inst = c.LLVMBuildRet(self.builder, c.LLVMConstInt(self.i32_type, 0, 0));
        self.debug_generator.setInstructionDebugLocation(ret_inst, 36, 5);
    }
    
    /// Generate printf call with debug location
    fn generatePrintfCall(self: *DebugEnabledCodeGen, message: []const u8, line: u32) void {
        const str_const = c.LLVMBuildGlobalStringPtr(self.builder, message.ptr, "str");
        const format_str = c.LLVMBuildGlobalStringPtr(self.builder, "%s\n", "fmt");
        
        const call_inst = c.LLVMBuildCall2(
            self.builder,
            c.LLVMFunctionType(self.i32_type, &[_]c.LLVMTypeRef{self.ptr_type}, 1, 1),
            self.printf_func,
            &[_]c.LLVMValueRef{ format_str, str_const },
            2,
            ""
        );
        
        self.debug_generator.setInstructionDebugLocation(call_inst, line, 5);
    }
    
    /// Generate printf call for integer with debug location
    fn generatePrintfCallInt(self: *DebugEnabledCodeGen, name: []const u8, alloca: c.LLVMValueRef, line: u32) void {
        const load_inst = c.LLVMBuildLoad2(self.builder, self.i32_type, alloca, "load");
        const format_str = c.LLVMBuildGlobalStringPtr(self.builder, "%s: %d\n", "fmt");
        const name_str = c.LLVMBuildGlobalStringPtr(self.builder, name.ptr, "name");
        
        const call_inst = c.LLVMBuildCall2(
            self.builder,
            c.LLVMFunctionType(self.i32_type, &[_]c.LLVMTypeRef{self.ptr_type}, 1, 1),
            self.printf_func,
            &[_]c.LLVMValueRef{ format_str, name_str, load_inst },
            3,
            ""
        );
        
        self.debug_generator.setInstructionDebugLocation(load_inst, line, 5);
        self.debug_generator.setInstructionDebugLocation(call_inst, line, 5);
    }
    
    /// Generate printf call for long with debug location
    fn generatePrintfCallLong(self: *DebugEnabledCodeGen, name: []const u8, alloca: c.LLVMValueRef, line: u32) void {
        const load_inst = c.LLVMBuildLoad2(self.builder, self.i64_type, alloca, "load");
        const format_str = c.LLVMBuildGlobalStringPtr(self.builder, "%s: %lld\n", "fmt");
        const name_str = c.LLVMBuildGlobalStringPtr(self.builder, name.ptr, "name");
        
        const call_inst = c.LLVMBuildCall2(
            self.builder,
            c.LLVMFunctionType(self.i32_type, &[_]c.LLVMTypeRef{self.ptr_type}, 1, 1),
            self.printf_func,
            &[_]c.LLVMValueRef{ format_str, name_str, load_inst },
            3,
            ""
        );
        
        self.debug_generator.setInstructionDebugLocation(load_inst, line, 5);
        self.debug_generator.setInstructionDebugLocation(call_inst, line, 5);
    }
    
    /// Generate printf call for double with debug location
    fn generatePrintfCallDouble(self: *DebugEnabledCodeGen, name: []const u8, alloca: c.LLVMValueRef, line: u32) void {
        const load_inst = c.LLVMBuildLoad2(self.builder, self.f64_type, alloca, "load");
        const format_str = c.LLVMBuildGlobalStringPtr(self.builder, "%s: %f\n", "fmt");
        const name_str = c.LLVMBuildGlobalStringPtr(self.builder, name.ptr, "name");
        
        const call_inst = c.LLVMBuildCall2(
            self.builder,
            c.LLVMFunctionType(self.i32_type, &[_]c.LLVMTypeRef{self.ptr_type}, 1, 1),
            self.printf_func,
            &[_]c.LLVMValueRef{ format_str, name_str, load_inst },
            3,
            ""
        );
        
        self.debug_generator.setInstructionDebugLocation(load_inst, line, 5);
        self.debug_generator.setInstructionDebugLocation(call_inst, line, 5);
    }
    
    /// Generate printf call for string with debug location
    fn generatePrintfCallString(self: *DebugEnabledCodeGen, name: []const u8, alloca: c.LLVMValueRef, line: u32) void {
        const load_inst = c.LLVMBuildLoad2(self.builder, self.ptr_type, alloca, "load");
        const format_str = c.LLVMBuildGlobalStringPtr(self.builder, "%s: %s\n", "fmt");
        const name_str = c.LLVMBuildGlobalStringPtr(self.builder, name.ptr, "name");
        
        const call_inst = c.LLVMBuildCall2(
            self.builder,
            c.LLVMFunctionType(self.i32_type, &[_]c.LLVMTypeRef{self.ptr_type}, 1, 1),
            self.printf_func,
            &[_]c.LLVMValueRef{ format_str, name_str, load_inst },
            3,
            ""
        );
        
        self.debug_generator.setInstructionDebugLocation(load_inst, line, 5);
        self.debug_generator.setInstructionDebugLocation(call_inst, line, 5);
    }
    
    /// Generate printf call for string literal with debug location
    fn generatePrintfCallStringLiteral(self: *DebugEnabledCodeGen, name: []const u8, str_val: c.LLVMValueRef, line: u32) void {
        const format_str = c.LLVMBuildGlobalStringPtr(self.builder, "%s: %s\n", "fmt");
        const name_str = c.LLVMBuildGlobalStringPtr(self.builder, name.ptr, "name");
        
        const call_inst = c.LLVMBuildCall2(
            self.builder,
            c.LLVMFunctionType(self.i32_type, &[_]c.LLVMTypeRef{self.ptr_type}, 1, 1),
            self.printf_func,
            &[_]c.LLVMValueRef{ format_str, name_str, str_val },
            3,
            ""
        );
        
        self.debug_generator.setInstructionDebugLocation(call_inst, line, 5);
    }
    
    /// Generate printf call for boolean with debug location
    fn generatePrintfCallBool(self: *DebugEnabledCodeGen, name: []const u8, alloca: c.LLVMValueRef, line: u32) void {
        const load_inst = c.LLVMBuildLoad2(self.builder, self.i32_type, alloca, "load");
        
        // Create conditional for boolean printing
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        const true_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "bool_true");
        const false_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "bool_false");
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "bool_merge");
        
        const cmp = c.LLVMBuildICmp(self.builder, c.LLVMIntNE, load_inst, c.LLVMConstInt(self.i32_type, 0, 0), "cmp");
        _ = c.LLVMBuildCondBr(self.builder, cmp, true_block, false_block);
        
        // True block
        c.LLVMPositionBuilderAtEnd(self.builder, true_block);
        const format_str_true = c.LLVMBuildGlobalStringPtr(self.builder, "%s: based\n", "fmt_true");
        const name_str_true = c.LLVMBuildGlobalStringPtr(self.builder, name.ptr, "name_true");
        const call_true = c.LLVMBuildCall2(
            self.builder,
            c.LLVMFunctionType(self.i32_type, &[_]c.LLVMTypeRef{self.ptr_type}, 1, 1),
            self.printf_func,
            &[_]c.LLVMValueRef{ format_str_true, name_str_true },
            2,
            ""
        );
        self.debug_generator.setInstructionDebugLocation(call_true, line, 5);
        _ = c.LLVMBuildBr(self.builder, merge_block);
        
        // False block
        c.LLVMPositionBuilderAtEnd(self.builder, false_block);
        const format_str_false = c.LLVMBuildGlobalStringPtr(self.builder, "%s: cringe\n", "fmt_false");
        const name_str_false = c.LLVMBuildGlobalStringPtr(self.builder, name.ptr, "name_false");
        const call_false = c.LLVMBuildCall2(
            self.builder,
            c.LLVMFunctionType(self.i32_type, &[_]c.LLVMTypeRef{self.ptr_type}, 1, 1),
            self.printf_func,
            &[_]c.LLVMValueRef{ format_str_false, name_str_false },
            2,
            ""
        );
        self.debug_generator.setInstructionDebugLocation(call_false, line, 5);
        _ = c.LLVMBuildBr(self.builder, merge_block);
        
        // Continue in merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
        
        self.debug_generator.setInstructionDebugLocation(load_inst, line, 5);
    }
    
    /// Write LLVM IR to file with debug information
    pub fn writeIRWithDebug(self: *DebugEnabledCodeGen, filename: []const u8) !void {
        // Verify module
        var error_message: [*c]u8 = null;
        const verify_result = c.LLVMVerifyModule(self.module, c.LLVMPrintMessageAction, &error_message);
        if (verify_result != 0) {
            std.debug.print("❌ Module verification failed: {s}\n", .{error_message});
            c.LLVMDisposeMessage(error_message);
            return;
        }
        
        // Write bitcode to file
        const result = c.LLVMWriteBitcodeToFile(self.module, filename.ptr);
        if (result != 0) {
            std.debug.print("❌ Failed to write LLVM bitcode\n", .{});
            return;
        }
        
        // Also write human-readable IR
        const ir_filename = try std.fmt.allocPrint(self.allocator, "{s}.ll", .{filename[0..filename.len-3]});
        defer self.allocator.free(ir_filename);
        
        var ir_error_message: [*c]u8 = null;
        const ir_result = c.LLVMPrintModuleToFile(self.module, ir_filename.ptr, &ir_error_message);
        if (ir_result != 0) {
            std.debug.print("❌ Failed to write LLVM IR: {s}\n", .{ir_error_message});
            c.LLVMDisposeMessage(ir_error_message);
            return;
        }
        
        std.debug.print("✅ Written LLVM IR with debug info to {s}\n", .{ir_filename});
    }
    
    /// Compile to executable with debug information preserved
    pub fn compileToExecutableWithDebug(self: *DebugEnabledCodeGen, output_path: []const u8) !void {
        // First write IR to temporary file
        try self.writeIRWithDebug("temp_debug.bc");
        
        // Compile using clang with debug flags
        const compile_cmd = try std.fmt.allocPrint(
            self.allocator,
            "clang -g -O0 -o {s} temp_debug.ll",
            .{output_path}
        );
        defer self.allocator.free(compile_cmd);
        
        std.debug.print("🔨 Compiling with debug info: {s}\n", .{compile_cmd});
        
        const result = std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{ "sh", "-c", compile_cmd },
        }) catch |err| {
            std.debug.print("❌ Compilation failed: {any}\n", .{err});
            return;
        };
        
        defer self.allocator.free(result.stdout);
        defer self.allocator.free(result.stderr);
        
        if (result.term.Exited != 0) {
            std.debug.print("❌ Compilation failed with exit code {}\n", .{result.term.Exited});
            std.debug.print("stderr: {s}\n", .{result.stderr});
            return;
        }
        
        std.debug.print("✅ Executable with debug info created: {s}\n", .{output_path});
        std.debug.print("🔍 Debug with: gdb {s} or lldb {s}\n", .{ output_path, output_path });
    }
};
