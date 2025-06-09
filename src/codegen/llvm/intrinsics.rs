//! LLVM intrinsics and runtime library support
//!
//! This module provides registration of LLVM intrinsics and external runtime
//! library functions needed for CURSED program execution.

use inkwell::context::Context;
use inkwell::module::{Module, Linkage};
use inkwell::types::BasicTypeEnum;
use inkwell::AddressSpace;
use tracing::{debug, info, instrument};

/// Register standard LLVM intrinsics with the module.
#[instrument(skip(context, module))]
pub fn register_intrinsics<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    debug!("Registering LLVM intrinsics");
    
    // Register memory management intrinsics
    register_memory_intrinsics(context, module)?;
    
    // Register GC intrinsics
    register_gc_intrinsics(context, module)?;
    
    // Register math intrinsics for performance
    register_math_intrinsics(context, module)?;
    
    info!("LLVM intrinsics registered successfully");
    Ok(())
}

/// Register memory management intrinsics
#[instrument(skip(context, module))]
fn register_memory_intrinsics<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    
    // memcpy intrinsic
    let memcpy_type = context.void_type().fn_type(&[
        i8_ptr_type.into(),  // dest
        i8_ptr_type.into(),  // src
        i64_type.into(),     // len
        context.bool_type().into(), // is_volatile
    ], false);
    module.add_function("llvm.memcpy.p0i8.p0i8.i64", memcpy_type, Some(Linkage::External));
    
    // memset intrinsic
    let memset_type = context.void_type().fn_type(&[
        i8_ptr_type.into(),  // dest
        context.i8_type().into(), // val
        i64_type.into(),     // len
        context.bool_type().into(), // is_volatile
    ], false);
    module.add_function("llvm.memset.p0i8.i64", memset_type, Some(Linkage::External));
    
    debug!("Memory management intrinsics registered");
    Ok(())
}

/// Register garbage collection intrinsics
#[instrument(skip(context, module))]
fn register_gc_intrinsics<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = context.i64_type();
    
    // GC root registration
    let gcroot_type = context.void_type().fn_type(&[
        i8_ptr_type.ptr_type(AddressSpace::default()).into(), // ptr to root
        i8_ptr_type.into(), // metadata
    ], false);
    module.add_function("llvm.gcroot", gcroot_type, Some(Linkage::External));
    
    // GC write barrier
    let gcwrite_type = context.void_type().fn_type(&[
        i8_ptr_type.into(), // ptr
        i8_ptr_type.into(), // value
    ], false);
    module.add_function("llvm.gcwrite", gcwrite_type, Some(Linkage::External));
    
    debug!("GC intrinsics registered");
    Ok(())
}

/// Register math intrinsics for performance
#[instrument(skip(context, module))]
fn register_math_intrinsics<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let f64_type = context.f64_type();
    let f32_type = context.f32_type();
    
    // Double precision math functions
    let f64_unary_type = f64_type.fn_type(&[f64_type.into()], false);
    let f64_binary_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
    
    module.add_function("llvm.sin.f64", f64_unary_type, Some(Linkage::External));
    module.add_function("llvm.cos.f64", f64_unary_type, Some(Linkage::External));
    module.add_function("llvm.sqrt.f64", f64_unary_type, Some(Linkage::External));
    module.add_function("llvm.pow.f64", f64_binary_type, Some(Linkage::External));
    module.add_function("llvm.log.f64", f64_unary_type, Some(Linkage::External));
    module.add_function("llvm.exp.f64", f64_unary_type, Some(Linkage::External));
    module.add_function("llvm.floor.f64", f64_unary_type, Some(Linkage::External));
    module.add_function("llvm.ceil.f64", f64_unary_type, Some(Linkage::External));
    module.add_function("llvm.round.f64", f64_unary_type, Some(Linkage::External));
    module.add_function("llvm.fabs.f64", f64_unary_type, Some(Linkage::External));
    
    // Single precision math functions
    let f32_unary_type = f32_type.fn_type(&[f32_type.into()], false);
    let f32_binary_type = f32_type.fn_type(&[f32_type.into(), f32_type.into()], false);
    
    module.add_function("llvm.sin.f32", f32_unary_type, Some(Linkage::External));
    module.add_function("llvm.cos.f32", f32_unary_type, Some(Linkage::External));
    module.add_function("llvm.sqrt.f32", f32_unary_type, Some(Linkage::External));
    module.add_function("llvm.pow.f32", f32_binary_type, Some(Linkage::External));
    
    debug!("Math intrinsics registered");
    Ok(())
}

/// Register external functions for the CURSED standard library runtime.
#[instrument(skip(context, module))]
pub fn register_stdlib_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    debug!("Registering stdlib runtime functions");
    
    // Register C library functions that stdlib depends on
    register_c_library_functions(context, module)?;
    
    // Register CURSED runtime functions
    register_cursed_runtime_functions(context, module)?;
    
    info!("Stdlib runtime functions registered successfully");
    Ok(())
}

/// Register C library functions that the stdlib depends on
#[instrument(skip(context, module))]
fn register_c_library_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    
    // Standard I/O functions
    let puts_type = i32_type.fn_type(&[i8_ptr_type.into()], false);
    module.add_function("puts", puts_type, Some(Linkage::External));
    
    let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true); // variadic
    module.add_function("printf", printf_type, Some(Linkage::External));
    
    // Memory management functions
    let malloc_type = i8_ptr_type.fn_type(&[i64_type.into()], false);
    module.add_function("malloc", malloc_type, Some(Linkage::External));
    
    let free_type = context.void_type().fn_type(&[i8_ptr_type.into()], false);
    module.add_function("free", free_type, Some(Linkage::External));
    
    let realloc_type = i8_ptr_type.fn_type(&[i8_ptr_type.into(), i64_type.into()], false);
    module.add_function("realloc", realloc_type, Some(Linkage::External));
    
    // String functions
    let strlen_type = i64_type.fn_type(&[i8_ptr_type.into()], false);
    module.add_function("strlen", strlen_type, Some(Linkage::External));
    
    let strcmp_type = i32_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
    module.add_function("strcmp", strcmp_type, Some(Linkage::External));
    
    let strcpy_type = i8_ptr_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
    module.add_function("strcpy", strcpy_type, Some(Linkage::External));
    
    debug!("C library functions registered");
    Ok(())
}

/// Register CURSED runtime functions
#[instrument(skip(context, module))]
fn register_cursed_runtime_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let bool_type = context.bool_type();
    
    // String runtime functions
    register_string_runtime_functions(context, module)?;
    
    // Array/slice runtime functions
    register_array_runtime_functions(context, module)?;
    
    // Map runtime functions
    register_map_runtime_functions(context, module)?;
    
    // Concurrency runtime functions
    register_concurrency_runtime_functions(context, module)?;
    
    // GC runtime functions
    register_gc_runtime_functions(context, module)?;
    
    // Error handling runtime functions
    register_error_runtime_functions(context, module)?;
    
    debug!("CURSED runtime functions registered");
    Ok(())
}

/// Register string runtime functions
#[instrument(skip(context, module))]
fn register_string_runtime_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = context.i64_type();
    let bool_type = context.bool_type();
    
    // String type: {i64 length, i8* data}
    let string_type = context.struct_type(&[i64_type.into(), i8_ptr_type.into()], false);
    
    // cursed_string_create(const char* data, i64 length) -> string
    let string_create_type = string_type.fn_type(&[i8_ptr_type.into(), i64_type.into()], false);
    module.add_function("cursed_string_create", string_create_type, Some(Linkage::External));
    
    // cursed_string_concat(string a, string b) -> string
    let string_concat_type = string_type.fn_type(&[string_type.into(), string_type.into()], false);
    module.add_function("cursed_string_concat", string_concat_type, Some(Linkage::External));
    
    // cursed_string_contains(string haystack, string needle) -> bool
    let string_contains_type = bool_type.fn_type(&[string_type.into(), string_type.into()], false);
    module.add_function("cursed_string_contains", string_contains_type, Some(Linkage::External));
    
    debug!("String runtime functions registered");
    Ok(())
}

/// Register array/slice runtime functions
#[instrument(skip(context, module))]
fn register_array_runtime_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = context.i64_type();
    
    // Slice type: {i64 length, i64 capacity, i8* data}
    let slice_type = context.struct_type(&[i64_type.into(), i64_type.into(), i8_ptr_type.into()], false);
    
    // cursed_slice_create(i64 element_size, i64 length, i64 capacity) -> slice
    let slice_create_type = slice_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
    module.add_function("cursed_slice_create", slice_create_type, Some(Linkage::External));
    
    // cursed_slice_append(slice s, void* element, i64 element_size) -> slice
    let slice_append_type = slice_type.fn_type(&[slice_type.into(), i8_ptr_type.into(), i64_type.into()], false);
    module.add_function("cursed_slice_append", slice_append_type, Some(Linkage::External));
    
    debug!("Array runtime functions registered");
    Ok(())
}

/// Register map runtime functions
#[instrument(skip(context, module))]
fn register_map_runtime_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = context.i64_type();
    let bool_type = context.bool_type();
    
    // Map is opaque pointer to runtime structure
    let map_type = i8_ptr_type;
    
    // cursed_map_create(i64 key_size, i64 value_size) -> map
    let map_create_type = map_type.fn_type(&[i64_type.into(), i64_type.into()], false);
    module.add_function("cursed_map_create", map_create_type, Some(Linkage::External));
    
    // cursed_map_set(map m, void* key, void* value) -> void
    let map_set_type = context.void_type().fn_type(&[map_type.into(), i8_ptr_type.into(), i8_ptr_type.into()], false);
    module.add_function("cursed_map_set", map_set_type, Some(Linkage::External));
    
    // cursed_map_get(map m, void* key, void* out_value) -> bool
    let map_get_type = bool_type.fn_type(&[map_type.into(), i8_ptr_type.into(), i8_ptr_type.into()], false);
    module.add_function("cursed_map_get", map_get_type, Some(Linkage::External));
    
    debug!("Map runtime functions registered");
    Ok(())
}

/// Register concurrency runtime functions
#[instrument(skip(context, module))]
fn register_concurrency_runtime_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = context.i64_type();
    
    // Goroutine spawn
    let goroutine_spawn_type = context.void_type().fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
    module.add_function("cursed_goroutine_spawn", goroutine_spawn_type, Some(Linkage::External));
    
    // Channel operations
    let channel_create_type = i8_ptr_type.fn_type(&[i64_type.into()], false);
    module.add_function("cursed_channel_create", channel_create_type, Some(Linkage::External));
    
    let channel_send_type = context.void_type().fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
    module.add_function("cursed_channel_send", channel_send_type, Some(Linkage::External));
    
    let channel_recv_type = i8_ptr_type.fn_type(&[i8_ptr_type.into()], false);
    module.add_function("cursed_channel_recv", channel_recv_type, Some(Linkage::External));
    
    debug!("Concurrency runtime functions registered");
    Ok(())
}

/// Register garbage collection runtime functions
#[instrument(skip(context, module))]
fn register_gc_runtime_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    let i64_type = context.i64_type();
    
    // GC allocation
    let gc_alloc_type = i8_ptr_type.fn_type(&[i64_type.into()], false);
    module.add_function("cursed_gc_alloc", gc_alloc_type, Some(Linkage::External));
    
    // GC collection trigger
    let gc_collect_type = context.void_type().fn_type(&[], false);
    module.add_function("cursed_gc_collect", gc_collect_type, Some(Linkage::External));
    
    // GC root registration for stack frames
    let gc_register_root_type = context.void_type().fn_type(&[i8_ptr_type.into()], false);
    module.add_function("cursed_gc_register_root", gc_register_root_type, Some(Linkage::External));
    
    debug!("GC runtime functions registered");
    Ok(())
}

/// Register error handling runtime functions
#[instrument(skip(context, module))]
fn register_error_runtime_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
    
    // Panic function
    let panic_type = context.void_type().fn_type(&[i8_ptr_type.into()], false);
    module.add_function("cursed_panic", panic_type, Some(Linkage::External));
    
    // Recover function
    let recover_type = i8_ptr_type.fn_type(&[], false);
    module.add_function("cursed_recover", recover_type, Some(Linkage::External));
    
    debug!("Error runtime functions registered");
    Ok(())
}