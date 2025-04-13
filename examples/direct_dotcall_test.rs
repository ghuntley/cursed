use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() {
    // Create an LLVM context and module
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    
    // Define the puts function from C
    let i32_type = context.i32_type();
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let puts_type = i32_type.fn_type(&[i8_ptr_type.into()], false);
    module.add_function("puts", puts_type, None);
    
    // Create a global string
    let str_val = context.const_string("Hello from direct test\0".as_bytes(), false);
    let str_type = context.i8_type().array_type("Hello from direct test\0".len() as u32);
    let global = module.add_global(str_type, None, "hello_str");
    global.set_initializer(&str_val);
    
    // Create main function
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_fn = module.add_function("main", main_fn_type, None);
    let entry = context.append_basic_block(main_fn, "entry");
    
    builder.position_at_end(entry);
    
    // Get a pointer to the global string
    let str_ptr = unsafe {
        builder.build_pointer_cast(
            global.as_pointer_value(),
            i8_ptr_type,
            "str_ptr"
        ).unwrap()
    };
    
    // Call puts with the string
    if let Some(puts_fn) = module.get_function("puts") {
        builder.build_call(puts_fn, &[str_ptr.into()], "puts_call").unwrap();
    }
    
    // Return 0
    builder.build_return(Some(&i32_type.const_int(0, false))).unwrap();
    
    // Print the generated IR
    println!("{}", module.print_to_string().to_string());
    
    // JIT execution
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    
    // Define the external function that will be used
    extern "C" fn puts_impl(str_ptr: *const i8) -> i32 {
        unsafe {
            // Convert the pointer to a C string and print it
            let c_str = std::ffi::CStr::from_ptr(str_ptr);
            println!("{}", c_str.to_string_lossy());
        }
        0
    }
    
    // Map the external function
    if let Some(puts_fn) = module.get_function("puts") {
        unsafe {
            execution_engine.add_global_mapping(&puts_fn, puts_impl as usize);
        }
    }
    
    // Execute the main function
    unsafe {
        let main_fn = execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main").unwrap();
        let result = main_fn.call();
        println!("Main returned: {}", result);
    }
}