const std = @import("std");
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
const ast = @import("ast.zig");
const CodeGenError = @import("codegen.zig").CodeGenError;

/// Array metadata structure for LLVM
pub const ArrayMetadata = struct {
    /// LLVM struct type: { i64 length, [0 x T] data }
    pub fn createArrayStructType(context: c.LLVMContextRef, element_type: c.LLVMTypeRef) c.LLVMTypeRef {
        const length_type = c.LLVMInt64TypeInContext(context);
        const data_array_type = c.LLVMArrayType(element_type, 0); // Variable size array
        
        const struct_elements = [_]c.LLVMTypeRef{ length_type, data_array_type };
        return c.LLVMStructTypeInContext(context, &struct_elements, 2, 0);
    }
    
    /// Create array with length metadata in LLVM IR
    pub fn createArrayWithLength(
        context: c.LLVMContextRef,
        builder: c.LLVMBuilderRef,
        module: c.LLVMModuleRef,
        element_type: c.LLVMTypeRef,
        length: c.LLVMValueRef,
        _: std.mem.Allocator,
    ) CodeGenError!c.LLVMValueRef {
        // Create array struct type
        const array_struct_type = createArrayStructType(context, element_type);
        
        // Calculate size needed: sizeof(struct) + length * sizeof(element)
        const element_size = c.LLVMSizeOf(element_type);
        const data_size = c.LLVMBuildMul(builder, length, element_size, "data_size");
        const struct_size = c.LLVMSizeOf(array_struct_type);
        const total_size = c.LLVMBuildAdd(builder, struct_size, data_size, "total_size");
        
        // Allocate memory
        const malloc_func = c.LLVMGetNamedFunction(module, "malloc");
        if (malloc_func == null) {
            return CodeGenError.UndefinedSymbol;
        }
        
        const raw_ptr = c.LLVMBuildCall2(
            builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(malloc_func)),
            malloc_func,
            &[_]c.LLVMValueRef{total_size},
            1,
            "array_alloc"
        );
        
        // Cast to array struct pointer
        const array_ptr = c.LLVMBuildBitCast(
            builder,
            raw_ptr,
            c.LLVMPointerType(array_struct_type, 0),
            "array_struct_ptr"
        );
        
        // Store length in metadata
        const length_ptr = c.LLVMBuildStructGEP2(
            builder,
            array_struct_type,
            array_ptr,
            0,
            "length_ptr"
        );
        _ = c.LLVMBuildStore(builder, length, length_ptr);
        
        return array_ptr;
    }
    
    /// Get array length from array pointer
    pub fn getArrayLength(
        context: c.LLVMContextRef,
        builder: c.LLVMBuilderRef,
        array_ptr: c.LLVMValueRef,
        element_type: c.LLVMTypeRef,
    ) c.LLVMValueRef {
        const array_struct_type = createArrayStructType(context, element_type);
        
        // Get pointer to length field
        const length_ptr = c.LLVMBuildStructGEP2(
            builder,
            array_struct_type,
            array_ptr,
            0,
            "length_ptr"
        );
        
        // Load length value
        return c.LLVMBuildLoad2(
            builder,
            c.LLVMInt64TypeInContext(context),
            length_ptr,
            "array_length"
        );
    }
    
    /// Get pointer to array data
    pub fn getArrayDataPtr(
        context: c.LLVMContextRef,
        builder: c.LLVMBuilderRef,
        array_ptr: c.LLVMValueRef,
        element_type: c.LLVMTypeRef,
    ) c.LLVMValueRef {
        const array_struct_type = createArrayStructType(context, element_type);
        
        // Get pointer to data field
        const data_ptr = c.LLVMBuildStructGEP2(
            builder,
            array_struct_type,
            array_ptr,
            1,
            "data_ptr"
        );
        
        // Cast to element pointer
        return c.LLVMBuildBitCast(
            builder,
            data_ptr,
            c.LLVMPointerType(element_type, 0),
            "array_data"
        );
    }
    
    /// Generate array element access with bounds checking
    pub fn generateArrayAccess(
        context: c.LLVMContextRef,
        builder: c.LLVMBuilderRef,
        array_ptr: c.LLVMValueRef,
        index: c.LLVMValueRef,
        element_type: c.LLVMTypeRef,
        bounds_check: bool,
    ) CodeGenError!c.LLVMValueRef {
        if (bounds_check) {
            // Get array length for bounds check
            const length = getArrayLength(context, builder, array_ptr, element_type);
            
            // Check bounds: index >= 0 && index < length
            const zero = c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0, 0);
            const index_valid = c.LLVMBuildAnd(
                builder,
                c.LLVMBuildICmp(builder, c.LLVMIntSGE, index, zero, "index_ge_zero"),
                c.LLVMBuildICmp(builder, c.LLVMIntSLT, index, length, "index_lt_length"),
                "bounds_check"
            );
            
            // Generate bounds check trap for out-of-bounds access
            const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(builder));
            const bounds_error_block = c.LLVMAppendBasicBlockInContext(context, current_func, "bounds_error");
            const bounds_ok_block = c.LLVMAppendBasicBlockInContext(context, current_func, "bounds_ok");
            
            // Branch based on bounds check
            _ = c.LLVMBuildCondBr(builder, index_valid, bounds_ok_block, bounds_error_block);
            
            // Bounds error block - generate trap instruction
            c.LLVMPositionBuilderAtEnd(builder, bounds_error_block);
            
            // Create bounds error function call
            const error_func_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0);
            const bounds_error_func = c.LLVMAddFunction(c.LLVMGetModuleFromFunction(current_func), 
                "cursed_bounds_error", error_func_type);
            _ = c.LLVMBuildCall2(builder, error_func_type, bounds_error_func, null, 0, "");
            
            // Generate trap instruction to immediately terminate
            const trap_func = c.LLVMAddFunction(c.LLVMGetModuleFromFunction(current_func),
                "llvm.trap", c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0));
            _ = c.LLVMBuildCall2(builder, c.LLVMGlobalGetValueType(trap_func), trap_func, null, 0, "bounds_trap");
            _ = c.LLVMBuildUnreachable(builder);
            
            // Continue with valid bounds
            c.LLVMPositionBuilderAtEnd(builder, bounds_ok_block);
        }
        
        // Get data pointer
        const data_ptr = getArrayDataPtr(context, builder, array_ptr, element_type);
        
        // Calculate element address
        const element_ptr = c.LLVMBuildGEP2(
            builder,
            element_type,
            data_ptr,
            &[_]c.LLVMValueRef{index},
            1,
            "element_ptr"
        );
        
        // Load element value
        return c.LLVMBuildLoad2(builder, element_type, element_ptr, "element_value");
    }
};

/// Enhanced array literal generation with proper length tracking
pub const ArrayLiteralGenerator = struct {
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    module: c.LLVMModuleRef,
    allocator: std.mem.Allocator,
    
    pub fn init(
        context: c.LLVMContextRef,
        builder: c.LLVMBuilderRef,
        module: c.LLVMModuleRef,
        allocator: std.mem.Allocator,
    ) ArrayLiteralGenerator {
        return ArrayLiteralGenerator{
            .context = context,
            .builder = builder,
            .module = module,
            .allocator = allocator,
        };
    }
    
    /// Generate array literal with dynamic length calculation
    pub fn generateArrayLiteral(
        self: *ArrayLiteralGenerator,
        elements: []c.LLVMValueRef,
        element_type: c.LLVMTypeRef,
    ) CodeGenError!c.LLVMValueRef {
        if (elements.len == 0) {
            // Empty array
            const zero_length = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
            return ArrayMetadata.createArrayWithLength(
                self.context,
                self.builder,
                self.module,
                element_type,
                zero_length,
                self.allocator,
            );
        }
        
        // Create array with known length
        const length = c.LLVMConstInt(
            c.LLVMInt64TypeInContext(self.context),
            @as(u64, @intCast(elements.len)),
            0
        );
        
        const array_ptr = try ArrayMetadata.createArrayWithLength(
            self.context,
            self.builder,
            self.module,
            element_type,
            length,
            self.allocator,
        );
        
        // Initialize elements
        const data_ptr = ArrayMetadata.getArrayDataPtr(
            self.context,
            self.builder,
            array_ptr,
            element_type,
        );
        
        for (elements, 0..) |element, i| {
            const index = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @as(u64, @intCast(i)), 0);
            const element_ptr = c.LLVMBuildGEP2(
                self.builder,
                element_type,
                data_ptr,
                &[_]c.LLVMValueRef{index},
                1,
                "init_element_ptr"
            );
            _ = c.LLVMBuildStore(self.builder, element, element_ptr);
        }
        
        return array_ptr;
    }
    
    /// Generate length function call for arrays
    pub fn generateLengthCall(
        self: *ArrayLiteralGenerator,
        array_value: c.LLVMValueRef,
        element_type: c.LLVMTypeRef,
    ) c.LLVMValueRef {
        return ArrayMetadata.getArrayLength(
            self.context,
            self.builder,
            array_value,
            element_type,
        );
    }
};

/// Array type utilities
pub const ArrayTypeUtils = struct {
    /// Check if a type is an array type in CURSED
    pub fn isCursedArrayType(type_name: []const u8) bool {
        return std.mem.startsWith(u8, type_name, "[]");
    }
    
    /// Extract element type from array type string (e.g., "[]drip" -> "drip")
    pub fn getElementTypeName(array_type: []const u8) ?[]const u8 {
        if (!isCursedArrayType(array_type)) return null;
        return array_type[2..];
    }
    
    /// Convert CURSED array type to LLVM type
    pub fn cursedArrayTypeToLLVM(
        context: c.LLVMContextRef,
        cursed_type: []const u8,
    ) ?c.LLVMTypeRef {
        const element_type_name = getElementTypeName(cursed_type) orelse return null;
        
        // Map CURSED types to LLVM types
        const element_type = if (std.mem.eql(u8, element_type_name, "drip"))
            c.LLVMInt64TypeInContext(context)
        else if (std.mem.eql(u8, element_type_name, "normie"))
            c.LLVMInt32TypeInContext(context)
        else if (std.mem.eql(u8, element_type_name, "tea"))
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)
        else if (std.mem.eql(u8, element_type_name, "lit"))
            c.LLVMInt1TypeInContext(context)
        else
            return null;
            
        return ArrayMetadata.createArrayStructType(context, element_type);
    }
};
