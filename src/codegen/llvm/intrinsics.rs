//! LLVM intrinsics support

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::AddressSpace;

/// Register standard LLVM intrinsics with the module.
pub fn register_intrinsics<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    // Implement registration of any needed LLVM intrinsics
    Ok(())
}

/// Register external functions for the Cursed standard library.
pub fn register_stdlib_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), String> {
    // Register 'puts' function used by many tests
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    module.add_function("puts", puts_type, Some(inkwell::module::Linkage::External));
    
    // TODO: Add more stdlib functions as needed
    
    Ok(())
}