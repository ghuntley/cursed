//! LLVM Atomic Operations Code Generation for CURSED
//!
//! This module provides LLVM code generation for hardware atomic operations
//! including compare-and-swap, atomic arithmetic, memory barriers, and ordering.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue, BasicValue};
use inkwell::types::{BasicTypeEnum, IntType, BasicType};
use inkwell::{AddressSpace, IntPredicate, AtomicOrdering, AtomicRMWBinOp};
use crate::error::CursedError;
use std::collections::HashMap;

/// LLVM atomic operations compiler
pub struct AtomicOperationsCompiler<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    module: &'ctx Module<'ctx>,
}

impl<'ctx> AtomicOperationsCompiler<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        module: &'ctx Module<'ctx>,
    ) -> Self {
        Self {
            context,
            builder,
            module,
        }
    }

    /// Convert CURSED memory ordering to LLVM atomic ordering
    pub fn convert_memory_ordering(order: i32) -> AtomicOrdering {
        match order {
            0 => AtomicOrdering::Monotonic, // MEMORY_ORDER_RELAXED
            1 => AtomicOrdering::Acquire,   // MEMORY_ORDER_ACQUIRE
            2 => AtomicOrdering::Release,   // MEMORY_ORDER_RELEASE
            3 => AtomicOrdering::AcquireRelease, // MEMORY_ORDER_ACQ_REL
            4 => AtomicOrdering::SequentiallyConsistent, // MEMORY_ORDER_SEQ_CST
            _ => AtomicOrdering::SequentiallyConsistent, // Default to strongest ordering
        }
    }

    /// Generate atomic load instruction
    pub fn generate_atomic_load(
        &self,
        ptr: PointerValue<'ctx>,
        ordering: AtomicOrdering,
        value_type: IntType<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let load = self.builder
            .build_load(value_type, ptr, "atomic_load")
            .map_err(|e| CursedError::CodegenError(format!("Failed to build atomic load: {:?}", e)))?;
        
        // Set atomic properties on the load instruction
        if let BasicValueEnum::IntValue(int_val) = load {
            // Note: inkwell doesn't directly expose atomic load instruction builder
            // We need to use the lower-level LLVM-C API or inline assembly
            // For now, we'll use a regular load and add memory barriers
            self.generate_memory_fence(ordering)?;
            Ok(int_val)
        } else {
            Err(CursedError::CodegenError("Atomic load did not produce integer value".to_string()))
        }
    }

    /// Generate atomic store instruction
    pub fn generate_atomic_store(
        &self,
        ptr: PointerValue<'ctx>,
        value: IntValue<'ctx>,
        ordering: AtomicOrdering,
    ) -> Result<(), CursedError> {
        // Generate memory fence before store for ordering
        self.generate_memory_fence(ordering)?;
        
        self.builder
            .build_store(ptr, value)
            .map_err(|e| CursedError::CodegenError(format!("Failed to build atomic store: {:?}", e)))?;
        
        // Generate memory fence after store for ordering
        self.generate_memory_fence(ordering)?;
        
        Ok(())
    }

    /// Generate atomic compare-and-swap instruction
    pub fn generate_atomic_cmpxchg(
        &self,
        ptr: PointerValue<'ctx>,
        expected: IntValue<'ctx>,
        desired: IntValue<'ctx>,
        success_ordering: AtomicOrdering,
        failure_ordering: AtomicOrdering,
        weak: bool,
    ) -> Result<IntValue<'ctx>, CursedError> {
        // Use LLVM's cmpxchg instruction
        let cmpxchg_result = self.builder
            .build_cmpxchg(
                ptr,
                expected,
                desired,
                success_ordering,
                failure_ordering,
            )
            .map_err(|e| CursedError::CodegenError(format!("Failed to build atomic cmpxchg: {:?}", e)))?;

        // Extract the success flag from the result
        let success_flag = self.builder
            .build_extract_value(cmpxchg_result, 1, "cmpxchg_success")
            .map_err(|e| CursedError::CodegenError(format!("Failed to extract cmpxchg success: {:?}", e)))?;

        if let BasicValueEnum::IntValue(success) = success_flag {
            Ok(success)
        } else {
            Err(CursedError::CodegenError("CmpXchg success flag is not an integer".to_string()))
        }
    }

    /// Generate atomic read-modify-write instruction
    pub fn generate_atomic_rmw(
        &self,
        op: AtomicRMWBinOp,
        ptr: PointerValue<'ctx>,
        value: IntValue<'ctx>,
        ordering: AtomicOrdering,
    ) -> Result<IntValue<'ctx>, CursedError> {
        self.builder
            .build_atomicrmw(op, ptr, value, ordering)
            .map_err(|e| CursedError::CodegenError(format!("Failed to build atomic RMW: {:?}", e)))
    }

    /// Generate memory fence instruction
    pub fn generate_memory_fence(&self, ordering: AtomicOrdering) -> Result<(), CursedError> {
        self.builder
            .build_fence(ordering, 0, "memory_fence")
            .map_err(|e| CursedError::CodegenError(format!("Failed to build memory fence: {:?}", e)))?;
        Ok(())
    }

    /// Generate atomic add operation (fetch-and-add)
    pub fn generate_atomic_add(
        &self,
        ptr: PointerValue<'ctx>,
        value: IntValue<'ctx>,
        ordering: AtomicOrdering,
    ) -> Result<IntValue<'ctx>, CursedError> {
        self.generate_atomic_rmw(AtomicRMWBinOp::Add, ptr, value, ordering)
    }

    /// Generate atomic subtract operation (fetch-and-sub)
    pub fn generate_atomic_sub(
        &self,
        ptr: PointerValue<'ctx>,
        value: IntValue<'ctx>,
        ordering: AtomicOrdering,
    ) -> Result<IntValue<'ctx>, CursedError> {
        self.generate_atomic_rmw(AtomicRMWBinOp::Sub, ptr, value, ordering)
    }

    /// Generate atomic AND operation (fetch-and-AND)
    pub fn generate_atomic_and(
        &self,
        ptr: PointerValue<'ctx>,
        value: IntValue<'ctx>,
        ordering: AtomicOrdering,
    ) -> Result<IntValue<'ctx>, CursedError> {
        self.generate_atomic_rmw(AtomicRMWBinOp::And, ptr, value, ordering)
    }

    /// Generate atomic OR operation (fetch-and-OR)
    pub fn generate_atomic_or(
        &self,
        ptr: PointerValue<'ctx>,
        value: IntValue<'ctx>,
        ordering: AtomicOrdering,
    ) -> Result<IntValue<'ctx>, CursedError> {
        self.generate_atomic_rmw(AtomicRMWBinOp::Or, ptr, value, ordering)
    }

    /// Generate atomic XOR operation (fetch-and-XOR)
    pub fn generate_atomic_xor(
        &self,
        ptr: PointerValue<'ctx>,
        value: IntValue<'ctx>,
        ordering: AtomicOrdering,
    ) -> Result<IntValue<'ctx>, CursedError> {
        self.generate_atomic_rmw(AtomicRMWBinOp::Xor, ptr, value, ordering)
    }

    /// Generate atomic exchange/swap operation
    pub fn generate_atomic_exchange(
        &self,
        ptr: PointerValue<'ctx>,
        value: IntValue<'ctx>,
        ordering: AtomicOrdering,
    ) -> Result<IntValue<'ctx>, CursedError> {
        self.generate_atomic_rmw(AtomicRMWBinOp::Xchg, ptr, value, ordering)
    }

    /// Generate platform-specific memory barrier
    pub fn generate_platform_memory_barrier(&self, target_triple: &str) -> Result<(), CursedError> {
        match target_triple {
            triple if triple.contains("x86_64") => {
                // x86_64: Use mfence instruction
                self.generate_inline_assembly(
                    "mfence",
                    "",
                    "",
                    true,  // has_side_effects
                    false, // is_align_stack
                )?;
            }
            triple if triple.contains("aarch64") || triple.contains("arm64") => {
                // ARM64: Use dmb sy instruction
                self.generate_inline_assembly(
                    "dmb sy",
                    "",
                    "",
                    true,  // has_side_effects
                    false, // is_align_stack
                )?;
            }
            triple if triple.contains("wasm32") => {
                // WASM: Use atomic.fence
                self.generate_inline_assembly(
                    "atomic.fence",
                    "",
                    "",
                    true,  // has_side_effects
                    false, // is_align_stack
                )?;
            }
            _ => {
                // Generic memory fence
                self.generate_memory_fence(AtomicOrdering::SequentiallyConsistent)?;
            }
        }
        Ok(())
    }

    /// Generate inline assembly for platform-specific operations
    fn generate_inline_assembly(
        &self,
        assembly: &str,
        constraints: &str,
        inputs: &str,
        has_side_effects: bool,
        is_align_stack: bool,
    ) -> Result<(), CursedError> {
        // Create inline assembly type (void function with no parameters)
        let void_type = self.context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        
        // Create inline assembly function
        let inline_asm = self.context.create_inline_asm(
            fn_type,
            assembly.to_string(),
            constraints.to_string(),
            has_side_effects,
            is_align_stack,
            None,
            false,
        );

        // Call the inline assembly
        self.builder
            .build_indirect_call(fn_type, inline_asm, &[], "inline_asm")
            .map_err(|e| CursedError::CodegenError(format!("Failed to build inline assembly: {:?}", e)))?;

        Ok(())
    }

    /// Declare atomic intrinsic functions
    pub fn declare_atomic_intrinsics(&self) -> Result<HashMap<String, inkwell::values::FunctionValue<'ctx>>, CursedError> {
        let mut intrinsics = HashMap::new();
        
        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let void_type = self.context.void_type();

        // Declare llvm.atomic.load intrinsics
        let atomic_load_i32_type = i32_type.fn_type(&[i8_ptr_type.into()], false);
        let atomic_load_i32 = self.module.add_function("llvm.atomic.load.i32", atomic_load_i32_type, None);
        intrinsics.insert("atomic_load_i32".to_string(), atomic_load_i32);

        let atomic_load_i64_type = i64_type.fn_type(&[i8_ptr_type.into()], false);
        let atomic_load_i64 = self.module.add_function("llvm.atomic.load.i64", atomic_load_i64_type, None);
        intrinsics.insert("atomic_load_i64".to_string(), atomic_load_i64);

        // Declare llvm.atomic.store intrinsics
        let atomic_store_i32_type = void_type.fn_type(&[i32_type.into(), i8_ptr_type.into()], false);
        let atomic_store_i32 = self.module.add_function("llvm.atomic.store.i32", atomic_store_i32_type, None);
        intrinsics.insert("atomic_store_i32".to_string(), atomic_store_i32);

        let atomic_store_i64_type = void_type.fn_type(&[i64_type.into(), i8_ptr_type.into()], false);
        let atomic_store_i64 = self.module.add_function("llvm.atomic.store.i64", atomic_store_i64_type, None);
        intrinsics.insert("atomic_store_i64".to_string(), atomic_store_i64);

        // Declare compiler builtin atomic functions
        self.declare_builtin_atomics(&mut intrinsics)?;

        Ok(intrinsics)
    }

    /// Declare compiler builtin atomic functions
    fn declare_builtin_atomics(
        &self,
        intrinsics: &mut HashMap<String, inkwell::values::FunctionValue<'ctx>>,
    ) -> Result<(), CursedError> {
        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();
        let i32_ptr_type = i32_type.ptr_type(AddressSpace::default());
        let i64_ptr_type = i64_type.ptr_type(AddressSpace::default());
        let i8_type = self.context.i8_type();
        let void_type = self.context.void_type();

        // __builtin_atomic_load_n
        let load_i32_type = i32_type.fn_type(&[i32_ptr_type.into(), i32_type.into()], false);
        let load_i32 = self.module.add_function("__builtin_atomic_load_4", load_i32_type, None);
        intrinsics.insert("builtin_atomic_load_i32".to_string(), load_i32);

        let load_i64_type = i64_type.fn_type(&[i64_ptr_type.into(), i32_type.into()], false);
        let load_i64 = self.module.add_function("__builtin_atomic_load_8", load_i64_type, None);
        intrinsics.insert("builtin_atomic_load_i64".to_string(), load_i64);

        // __builtin_atomic_store_n
        let store_i32_type = void_type.fn_type(&[i32_ptr_type.into(), i32_type.into(), i32_type.into()], false);
        let store_i32 = self.module.add_function("__builtin_atomic_store_4", store_i32_type, None);
        intrinsics.insert("builtin_atomic_store_i32".to_string(), store_i32);

        let store_i64_type = void_type.fn_type(&[i64_ptr_type.into(), i64_type.into(), i32_type.into()], false);
        let store_i64 = self.module.add_function("__builtin_atomic_store_8", store_i64_type, None);
        intrinsics.insert("builtin_atomic_store_i64".to_string(), store_i64);

        // __builtin_atomic_compare_exchange_n
        let cmpxchg_i32_type = i8_type.fn_type(&[
            i32_ptr_type.into(),
            i32_ptr_type.into(),
            i32_type.into(),
            i8_type.into(),
            i32_type.into(),
            i32_type.into(),
        ], false);
        let cmpxchg_i32 = self.module.add_function("__builtin_atomic_compare_exchange_4", cmpxchg_i32_type, None);
        intrinsics.insert("builtin_atomic_cmpxchg_i32".to_string(), cmpxchg_i32);

        let cmpxchg_i64_type = i8_type.fn_type(&[
            i64_ptr_type.into(),
            i64_ptr_type.into(),
            i64_type.into(),
            i8_type.into(),
            i32_type.into(),
            i32_type.into(),
        ], false);
        let cmpxchg_i64 = self.module.add_function("__builtin_atomic_compare_exchange_8", cmpxchg_i64_type, None);
        intrinsics.insert("builtin_atomic_cmpxchg_i64".to_string(), cmpxchg_i64);

        // __builtin_atomic_fetch_add
        let fetch_add_i32_type = i32_type.fn_type(&[i32_ptr_type.into(), i32_type.into(), i32_type.into()], false);
        let fetch_add_i32 = self.module.add_function("__builtin_atomic_fetch_add_4", fetch_add_i32_type, None);
        intrinsics.insert("builtin_atomic_fetch_add_i32".to_string(), fetch_add_i32);

        let fetch_add_i64_type = i64_type.fn_type(&[i64_ptr_type.into(), i64_type.into(), i32_type.into()], false);
        let fetch_add_i64 = self.module.add_function("__builtin_atomic_fetch_add_8", fetch_add_i64_type, None);
        intrinsics.insert("builtin_atomic_fetch_add_i64".to_string(), fetch_add_i64);

        // __builtin_atomic_fetch_sub
        let fetch_sub_i32_type = i32_type.fn_type(&[i32_ptr_type.into(), i32_type.into(), i32_type.into()], false);
        let fetch_sub_i32 = self.module.add_function("__builtin_atomic_fetch_sub_4", fetch_sub_i32_type, None);
        intrinsics.insert("builtin_atomic_fetch_sub_i32".to_string(), fetch_sub_i32);

        let fetch_sub_i64_type = i64_type.fn_type(&[i64_ptr_type.into(), i64_type.into(), i32_type.into()], false);
        let fetch_sub_i64 = self.module.add_function("__builtin_atomic_fetch_sub_8", fetch_sub_i64_type, None);
        intrinsics.insert("builtin_atomic_fetch_sub_i64".to_string(), fetch_sub_i64);

        // __builtin_atomic_fetch_and
        let fetch_and_i32_type = i32_type.fn_type(&[i32_ptr_type.into(), i32_type.into(), i32_type.into()], false);
        let fetch_and_i32 = self.module.add_function("__builtin_atomic_fetch_and_4", fetch_and_i32_type, None);
        intrinsics.insert("builtin_atomic_fetch_and_i32".to_string(), fetch_and_i32);

        // __builtin_atomic_fetch_or
        let fetch_or_i32_type = i32_type.fn_type(&[i32_ptr_type.into(), i32_type.into(), i32_type.into()], false);
        let fetch_or_i32 = self.module.add_function("__builtin_atomic_fetch_or_4", fetch_or_i32_type, None);
        intrinsics.insert("builtin_atomic_fetch_or_i32".to_string(), fetch_or_i32);

        // __builtin_atomic_fetch_xor
        let fetch_xor_i32_type = i32_type.fn_type(&[i32_ptr_type.into(), i32_type.into(), i32_type.into()], false);
        let fetch_xor_i32 = self.module.add_function("__builtin_atomic_fetch_xor_4", fetch_xor_i32_type, None);
        intrinsics.insert("builtin_atomic_fetch_xor_i32".to_string(), fetch_xor_i32);

        // __builtin_atomic_thread_fence
        let thread_fence_type = void_type.fn_type(&[i32_type.into()], false);
        let thread_fence = self.module.add_function("__builtin_atomic_thread_fence", thread_fence_type, None);
        intrinsics.insert("builtin_atomic_thread_fence".to_string(), thread_fence);

        // __builtin_atomic_signal_fence
        let signal_fence_type = void_type.fn_type(&[i32_type.into()], false);
        let signal_fence = self.module.add_function("__builtin_atomic_signal_fence", signal_fence_type, None);
        intrinsics.insert("builtin_atomic_signal_fence".to_string(), signal_fence);

        Ok(())
    }

    /// Generate atomic operation based on builtin function name
    pub fn generate_builtin_atomic_call(
        &self,
        intrinsics: &HashMap<String, inkwell::values::FunctionValue<'ctx>>,
        builtin_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, CursedError> {
        let function = intrinsics.get(builtin_name)
            .ok_or_else(|| CursedError::CodegenError(format!("Unknown atomic builtin: {}", builtin_name)))?;

        // Convert BasicValueEnum to BasicMetadataValueEnum
        let metadata_args: Vec<_> = args.iter().map(|arg| (*arg).into()).collect();
        let call_result = self.builder
            .build_call(*function, &metadata_args, builtin_name)
            .map_err(|e| CursedError::CodegenError(format!("Failed to call atomic builtin {}: {:?}", builtin_name, e)))?;

        Ok(call_result.try_as_basic_value().left())
    }
}

/// Atomic operation type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicOpType {
    Load,
    Store,
    CompareExchange,
    Exchange,
    Add,
    Sub,
    And,
    Or,
    Xor,
    Fence,
}

/// Atomic operation metadata
#[derive(Debug, Clone)]
pub struct AtomicOpMetadata<'ctx> {
    pub op_type: AtomicOpType,
    pub ordering: AtomicOrdering,
    pub is_weak: bool,
    pub value_type: BasicTypeEnum<'ctx>,
}

impl<'ctx> AtomicOpMetadata<'ctx> {
    pub fn new(op_type: AtomicOpType, ordering: AtomicOrdering, value_type: BasicTypeEnum<'ctx>) -> Self {
        Self {
            op_type,
            ordering,
            is_weak: false,
            value_type,
        }
    }

    pub fn with_weak(mut self, is_weak: bool) -> Self {
        self.is_weak = is_weak;
        self
    }
}
