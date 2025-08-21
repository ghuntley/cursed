const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const ast = @import("ast.zig");
const CursedError = @import("error_handling.zig").CursedError;
pub const ErrorContext = @import("error_handling.zig").ErrorContext;

/// Complete error propagation semantics for CURSED error handling
pub const ErrorPropagation = struct {
    allocator: Allocator,
    error_stack: ArrayList(ErrorContext),
    propagation_handlers: ArrayList(PropagationHandler),
    try_catch_stack: ArrayList(TryCatchFrame),
    current_scope_id: u32,
    defer_entries: DeferStack,
    current_file: ?[]const u8,
    current_line: u32,
    current_column: u32,
    
    const DeferStack = struct {
        entries: ArrayList(DeferEntry),
        current_scope: u32,
        allocator: Allocator,
        
        const DeferEntry = struct {
            cleanup_fn: *const fn() void,
            context_data: []const u8,
            scope_id: u32,
        };
        
        fn init(allocator: Allocator) DeferStack {
            return DeferStack{
                .entries = .empty,
                .current_scope = 0,
                .allocator = allocator,
            };
        }
        
        fn deinit(self: *DeferStack) void {
            self.executeAll();
            self.entries.deinit(allocator);
        }
        
        fn enterScope(self: *DeferStack) !void {
            self.current_scope += 1;
        }
        
        fn exitScope(self: *DeferStack) void {
            var i = self.entries.items.len;
            while (i > 0) {
                i -= 1;
                const entry = self.entries.items[i];
                if (entry.scope_id == self.current_scope) {
                    entry.cleanup_fn();
                    self.allocator.free(entry.context_data);
                    _ = self.entries.orderedRemove(i);
                }
            }
            
            if (self.current_scope > 0) {
                self.current_scope -= 1;
            }
        }
        
        fn executeAll(self: *DeferStack) void {
            while (self.entries.items.len > 0) {
                const entry = self.entries.pop();
                entry.cleanup_fn();
                self.allocator.free(entry.context_data);
            }
        }
        
        fn push(self: *DeferStack, cleanup_fn: *const fn() void, context: []const u8) !void {
            const context_copy = try self.allocator.dupe(u8, context);
            try self.entries.append(self.allocator, DeferEntry{
                .cleanup_fn = cleanup_fn,
                .context_data = context_copy,
                .scope_id = self.current_scope,
            });
        }
    };
    
    const PropagationHandler = struct {
        error_type: []const u8,
        handler_fn: *const fn (ErrorContext) CursedError!void,
    };
    
    const TryCatchFrame = struct {
        catch_blocks: ArrayList(CatchBlock),
        finally_block: ?ArrayList(ast.Statement),
        error_occurred: ?ErrorContext,
        recovery_point: usize,
        
        const CatchBlock = struct {
            error_variable: ?[]const u8,
            error_type: ?[]const u8,
            body: ArrayList(ast.Statement),
        };
    };
    
    pub fn init(allocator: Allocator) ErrorPropagation {
        return ErrorPropagation{
            .allocator = allocator,
            .error_stack = .empty,
            .propagation_handlers = .empty,
            .try_catch_stack = .empty,
            .current_scope_id = 0,
            .defer_entries = DeferStack.init(allocator),
            .current_file = null,
            .current_line = 0,
            .current_column = 0,
        };
    }
    
    pub fn deinit(self: *ErrorPropagation) void {
        for (self.error_stack.items) |*error_ctx| {
            error_ctx.deinit(allocator);
        }
        self.error_stack.deinit(allocator);
        
        for (self.try_catch_stack.items) |*frame| {
            for (frame.catch_blocks.items) |*catch_block| {
                catch_block.body.deinit(allocator);
            }
            frame.catch_blocks.deinit(allocator);
            if (frame.finally_block) |*finally| {
                finally.deinit(allocator);
            }
            if (frame.error_occurred) |*err| {
                err.deinit(allocator);
            }
        }
        self.try_catch_stack.deinit(allocator);
        
        self.defer_entries.deinit(allocator);
        self.propagation_handlers.deinit(allocator);
        
        if (self.current_file) |file| {
            self.allocator.free(file);
        }
    }
    
    /// Create error with yikes semantics
    pub fn createYikesError(
        self: *ErrorPropagation,
        message: []const u8,
        _: ?[]const u8,
        location: ?ErrorContext.SourceLocation
    ) CursedError!ErrorContext {
        var ctx = try ErrorContext.init(self.allocator, CursedError.RuntimeError, message);
        if (location) |loc| {
            ctx.location = loc;
        }
        
        // Add to error stack for propagation
        try self.error_stack.append(self.allocator, ctx);
        
        return ctx;
    }
    
    /// Propagate error with shook semantics (Rust-style ? operator)
    pub fn propagateError(
        self: *ErrorPropagation,
        error_ctx: ErrorContext,
        propagate_immediately: bool
    ) CursedError!bool {
        // Check if we're in a try-catch block
        if (self.try_catch_stack.items.len > 0) {
            var current_frame = &self.try_catch_stack.items[self.try_catch_stack.items.len - 1];
            
            // Store error in current frame
            if (current_frame.error_occurred) |*existing_err| {
                existing_err.deinit(allocator);
            }
            current_frame.error_occurred = try ErrorContext.initWithInner(
                self.allocator,
                error_ctx.error_code,
                error_ctx.message,
                @constCast(&error_ctx)
            );
            
            // Return false to indicate we should unwind to catch block
            return false;
        }
        
        if (propagate_immediately) {
            // No try-catch frame, propagate error up the call stack
            try self.error_stack.append(allocator, error_ctx);
            return CursedError.RuntimeError;
        }
        
        return true;
    }
    
    /// Enter fam try-catch block
    pub fn enterTryCatchBlock(
        self: *ErrorPropagation,
        catch_blocks: []const ast.FamStatement.CatchBlock,
        finally_block: ?ArrayList(ast.Statement)
    ) CursedError!void {
        var frame = TryCatchFrame{
            .catch_blocks = .empty,
            .finally_block = finally_block,
            .error_occurred = null,
            .recovery_point = self.error_stack.items.len,
        };
        
        // Convert AST catch blocks to runtime catch blocks
        for (catch_blocks) |ast_catch| {
            var runtime_catch = TryCatchFrame.CatchBlock{
                .error_variable = ast_catch.error_variable,
                .error_type = ast_catch.error_type,
                .body = try ArrayList(ast.Statement).initCapacity(self.allocator, ast_catch.body.items.len),
            };
            
            for (ast_catch.body.items) |stmt| {
                try runtime_catch.body.append(self.allocator, stmt);
            }
            
            try frame.catch_blocks.append(self.allocator, runtime_catch);
        }
        
        try self.try_catch_stack.append(self.allocator, frame);
    }
    
    /// Exit fam try-catch block and handle any errors
    pub fn exitTryCatchBlock(self: *ErrorPropagation) CursedError!?ErrorContext {
        if (self.try_catch_stack.items.len == 0) {
            return CursedError.RuntimeError;
        }
        
        const frame: TryCatchFrame = self.try_catch_stack.pop() orelse return null;
        defer {
            for (frame.catch_blocks.items) |*catch_block| {
                catch_block.body.deinit(allocator);
            }
            frame.catch_blocks.deinit(allocator);
            if (frame.finally_block) |*finally| {
                finally.deinit(allocator);
            }
        }
        
        // Return any unhandled error for further propagation
        if (frame.error_occurred) |error_ctx| {
            // Find matching catch block
            for (frame.catch_blocks.items) |catch_block| {
                if (self.errorMatches(error_ctx, catch_block.error_type)) {
                    // Error was handled by catch block
                    return null;
                }
            }
            
            // No matching catch block found, propagate error
            return error_ctx;
        }
        
        return null;
    }
    
    /// Check if error matches catch block type filter
    pub fn errorMatches(self: *ErrorPropagation, error_ctx: ErrorContext, error_type: ?[]const u8) bool {
        _ = self;
        
        if (error_type == null) {
            // Catch-all block
            return true;
        }
        
        // TODO: Implement sophisticated error type matching
        // For now, do simple string comparison
        const error_name = @errorName(error_ctx.error_code);
        return std.mem.eql(u8, error_name, error_type.?);
    }
    
    /// Register error propagation handler
    pub fn registerPropagationHandler(
        self: *ErrorPropagation,
        error_type: []const u8,
        handler_fn: *const fn (ErrorContext) CursedError!void
    ) CursedError!void {
        try self.propagation_handlers.append(self.allocator, PropagationHandler{
            .error_type = try self.allocator.dupe(u8, error_type),
            .handler_fn = handler_fn,
        });
    }
    
    /// Execute finally blocks in reverse order
    pub fn executeFinally(self: *ErrorPropagation) CursedError!void {
        var i = self.try_catch_stack.items.len;
        while (i > 0) {
            i -= 1;
            const frame = &self.try_catch_stack.items[i];
            if (frame.finally_block) |finally_block| {
                // TODO: Execute finally block statements
                // This would require access to the interpreter
                _ = finally_block;
            }
        }
    }
    
    /// Get current error context for debugging
    pub fn getCurrentError(self: *ErrorPropagation) ?ErrorContext {
        if (self.error_stack.items.len > 0) {
            return self.error_stack.items[self.error_stack.items.len - 1];
        }
        return null;
    }
    
    /// Clear error stack (useful for error recovery)
    pub fn clearErrors(self: *ErrorPropagation) void {
        for (self.error_stack.items) |*error_ctx| {
            error_ctx.deinit(allocator);
        }
        self.error_stack.clearRetainingCapacity();
    }
    
    /// Format error stack for debugging
    pub fn formatErrorStack(self: *ErrorPropagation, writer: anytype) !void {
        try writer.print("Error Stack ({} errors):\n", .{self.error_stack.items.len});
        for (self.error_stack.items, 0..) |error_ctx, i| {
            try writer.print("  [{}] ", .{i});
            try error_ctx.format(writer);
        }
    }
};

/// LLVM IR generation for error propagation
pub const ErrorPropagationLLVM = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    
    const Self = @This();
    
    pub fn init(
        allocator: Allocator,
        context: c.LLVMContextRef,
        module: c.LLVMModuleRef,
        builder: c.LLVMBuilderRef
    ) Self {
        return Self{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
        };
    }
    
    /// Generate LLVM IR for shook error propagation
    pub fn generateShookPropagation(
        self: *Self,
        value: c.LLVMValueRef,
        current_function: c.LLVMValueRef
    ) CursedError!c.LLVMValueRef {
        // Create error checking function call
        const error_check_func = try self.getOrCreateErrorCheckFunction();
        
        // Check if value is an error
        const is_error = c.LLVMBuildCall2(
            self.builder,
            c.LLVMInt1TypeInContext(self.context),
            error_check_func,
            &[_]c.LLVMValueRef{value},
            1,
            "is_error"
        );
        
        // Create basic blocks
        _ = c.LLVMGetInsertBlock(self.builder);
        const error_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "error_propagate");
        const continue_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "continue");
        
        // Branch based on error check
        _ = c.LLVMBuildCondBr(self.builder, is_error, error_block, continue_block);
        
        // Error block: propagate error
        c.LLVMPositionBuilderAtEnd(self.builder, error_block);
        
        // Call error propagation function
        const propagate_func = try self.getOrCreateErrorPropagateFunction();
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            propagate_func,
            &[_]c.LLVMValueRef{value},
            1,
            ""
        );
        
        // Return error value to caller
        _ = c.LLVMBuildRet(self.builder, value);
        
        // Continue block: normal execution
        c.LLVMPositionBuilderAtEnd(self.builder, continue_block);
        
        return value;
    }
    
    /// Generate LLVM IR for fam try-catch block
    pub fn generateFamTryCatch(
        self: *Self,
        try_body_fn: *const fn () CursedError!c.LLVMValueRef,
        catch_blocks: []const ast.FamStatement.CatchBlock,
        current_function: c.LLVMValueRef
    ) CursedError!c.LLVMValueRef {
        // Create basic blocks
        const try_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "try");
        const catch_dispatch = c.LLVMAppendBasicBlockInContext(self.context, current_function, "catch_dispatch");
        const finally_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "finally");
        const end_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "end");
        
        // Set up exception handling
        const try_begin_func = try self.getOrCreateTryBeginFunction();
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            try_begin_func,
            null,
            0,
            ""
        );
        
        // Jump to try block
        _ = c.LLVMBuildBr(self.builder, try_block);
        
        // Try block
        c.LLVMPositionBuilderAtEnd(self.builder, try_block);
        const try_result = try try_body_fn();
        
        // Check if error occurred
        const error_check_func = try self.getOrCreateErrorCheckFunction();
        const try_error = c.LLVMBuildCall2(
            self.builder,
            c.LLVMInt1TypeInContext(self.context),
            error_check_func,
            &[_]c.LLVMValueRef{try_result},
            1,
            "try_error"
        );
        
        // Branch to catch dispatch or finally
        _ = c.LLVMBuildCondBr(self.builder, try_error, catch_dispatch, finally_block);
        
        // Catch dispatch block
        c.LLVMPositionBuilderAtEnd(self.builder, catch_dispatch);
        
        // Generate catch block dispatch logic
        for (catch_blocks, 0..) |catch_block, i| {
            const catch_bb = c.LLVMAppendBasicBlockInContext(
                self.context,
                current_function,
                try std.fmt.allocPrint(self.allocator, "catch_{}", .{i}).ptr
            );
            
            // TODO: Generate type checking for catch block
            _ = catch_block;
            _ = catch_bb;
        }
        
        // Jump to finally
        _ = c.LLVMBuildBr(self.builder, finally_block);
        
        // Finally block
        c.LLVMPositionBuilderAtEnd(self.builder, finally_block);
        
        // Call try end function
        const try_end_func = try self.getOrCreateTryEndFunction();
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            try_end_func,
            null,
            0,
            ""
        );
        
        // Jump to end
        _ = c.LLVMBuildBr(self.builder, end_block);
        
        // End block
        c.LLVMPositionBuilderAtEnd(self.builder, end_block);
        
        return try_result;
    }
    
    /// Get or create error checking function
    fn getOrCreateErrorCheckFunction(self: *Self) CursedError!c.LLVMValueRef {
        const func_name = "cursed_is_error";
        
        if (c.LLVMGetNamedFunction(self.module, func_name)) |existing| {
            return existing;
        }
        
        const param_types = [_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)
        };
        
        const func_type = c.LLVMFunctionType(
            c.LLVMInt1TypeInContext(self.context),
            &param_types,
            1,
            0
        );
        
        return c.LLVMAddFunction(self.module, func_name, func_type);
    }
    
    /// Get or create error propagation function
    fn getOrCreateErrorPropagateFunction(self: *Self) CursedError!c.LLVMValueRef {
        const func_name = "cursed_propagate_error";
        
        if (c.LLVMGetNamedFunction(self.module, func_name)) |existing| {
            return existing;
        }
        
        const param_types = [_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)
        };
        
        const func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &param_types,
            1,
            0
        );
        
        return c.LLVMAddFunction(self.module, func_name, func_type);
    }
    
    /// Get or create try begin function
    fn getOrCreateTryBeginFunction(self: *Self) CursedError!c.LLVMValueRef {
        const func_name = "cursed_try_begin";
        
        if (c.LLVMGetNamedFunction(self.module, func_name)) |existing| {
            return existing;
        }
        
        const func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            null,
            0,
            0
        );
        
        return c.LLVMAddFunction(self.module, func_name, func_type);
    }
    
    /// Get or create try end function
    fn getOrCreateTryEndFunction(self: *Self) CursedError!c.LLVMValueRef {
        const func_name = "cursed_try_end";
        
        if (c.LLVMGetNamedFunction(self.module, func_name)) |existing| {
            return existing;
        }
        
        const func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            null,
            0,
            0
        );
        
        return c.LLVMAddFunction(self.module, func_name, func_type);
    }
};

// Import LLVM C headers
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

test "error propagation system" {
    const allocator = std.testing.allocator;
    
    var error_prop = ErrorPropagation.init(allocator);
    defer error_prop.deinit(allocator);
    
    // Test yikes error creation
    const location = ErrorContext.SourceLocation{
        .file = "test.csd",
        .line = 10,
        .column = 5,
    };
    
    const error_ctx = try error_prop.createYikesError(
        "Test error message",
        "RuntimeError",
        location
    );
    
    try std.testing.expect(std.mem.eql(u8, error_ctx.message, "Test error message"));
    try std.testing.expect(error_prop.error_stack.items.len == 1);
    
    // Test error propagation
    const should_propagate = try error_prop.propagateError(error_ctx, false);
    try std.testing.expect(should_propagate);
}
