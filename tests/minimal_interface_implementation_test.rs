use inkwell::context::Context;
use std::path::PathBuf;
use inkwell::values::AnyValue;
use inkwell::AddressSpace;

// Test for a very basic interface-like implementation with structs


#[path = "tracing_setup.rs"]
pub mod tracing_setup;

#[test]
fn test_minimal_interface_implementation() {
    // Set up tracing
    tracing_setup::init_test_tracing();
    
    // Create a basic LLVM context
    let context = Context::create();
    let module = context.create_module("minimal_interface_test");
    let builder = context.create_builder();
    
    // Create a "Stringer" interface-like structure
    // It will have a single method: to_string() -> char*
    let stringer_vtable_type = context.struct_type(
        &[
            // First field is a function pointer: char* (*)(void*)
            context.i8_type().ptr_type(AddressSpace::default())
                .fn_type(&[context.i8_type().ptr_type(AddressSpace::default()).into()], false)
                .ptr_type(AddressSpace::default())
                .into()
        ],
        false
    );
    // stringer_vtable_type.set_name("Stringer_vtable"); // Not supported in this version of inkwell
    
    // Create a Person struct type
    let person_type = context.struct_type(
        &[
            // name: char*
            context.i8_type().ptr_type(AddressSpace::default()).into(),
            // age: int
            context.i32_type().into(),
        ],
        false
    );
    // person_type.set_name("Person"); // Not supported in this version of inkwell
    
    // Create Person.to_string implementation
    let person_to_string_type = context.i8_type().ptr_type(AddressSpace::default())
        .fn_type(&[context.i8_type().ptr_type(AddressSpace::default()).into()], false);
    let person_to_string = module.add_function(
        "Person_to_string",
        person_to_string_type,
        None
    );
    
    // Implement Person_to_string to return the name field
    let entry_block = context.append_basic_block(person_to_string, "entry");
    builder.position_at_end(entry_block);
    
    // Get the self parameter (void* that we'll cast to Person*)
    let self_param = person_to_string.get_first_param().unwrap()
        .into_pointer_value();
    
    // Cast void* to Person*
    let person_ptr = builder.build_bitcast(
        self_param,
        person_type.ptr_type(AddressSpace::default()),
        "person_ptr"
    ).unwrap().into_pointer_value());
    
    // Get pointer to name field
    let name_ptr = unsafe {
        builder.build_struct_gep(
            person_type,
            person_ptr,
            0,
            "name_ptr"
        ).unwrap()
    };
    
    // Load the name string
    let name = builder.build_load(
        context.i8_type().ptr_type(AddressSpace::default()),
        name_ptr,
        "name"
    ).unwrap();
    
    // Return the name string
    builder.build_return(Some(&name)).unwrap());
    
    // Create the Stringer vtable for Person
    let stringer_person_vtable = module.add_global(
        stringer_vtable_type,
        None,
        "Stringer_Person_vtable"
    );
    
    // Initialize the vtable with Person's to_string implementation
    let vtable_value = context.const_struct(
        &[
            person_to_string.as_global_value().as_pointer_value().into()
        ],
        false
    );
    stringer_person_vtable.set_initializer(&vtable_value);
    
    // Now create a test function that demonstrates using the vtable
    let test_fn_type = context.i8_type().ptr_type(AddressSpace::default())
        .fn_type(&[], false);
    let test_function = module.add_function("test_stringer", test_fn_type, None);
    
    let test_entry = context.append_basic_block(test_function, "entry");
    builder.position_at_end(test_entry);
    
    // Create a Person
    let person_ptr = builder.build_alloca(person_type, "person").unwrap());
    
    // Initialize name field with "Alice"
    let name_ptr = unsafe {
        builder.build_struct_gep(
            person_type,
            person_ptr,
            0,
            "name_ptr"
        ).unwrap()
    };
    
    // Create a global string constant for "Alice"
    let alice = builder.build_global_string_ptr("Alice", "alice_str").unwrap());
    builder.build_store(name_ptr, alice.as_pointer_value()).unwrap());
    
    // Initialize age field with 30
    let age_ptr = unsafe {
        builder.build_struct_gep(
            person_type,
            person_ptr,
            1,
            "age_ptr"
        ).unwrap()
    };
    
    let thirty = context.i32_type().const_int(30, false);
    builder.build_store(age_ptr, thirty).unwrap());
    
    // Get the vtable pointer
    let vtable_ptr = stringer_person_vtable.as_pointer_value();
    
    // Cast person to void*
    let person_void_ptr = builder.build_bitcast(
        person_ptr,
        context.i8_type().ptr_type(AddressSpace::default()),
        "person_void_ptr"
    ).unwrap().into_pointer_value());
    
    // Get the to_string function pointer from the vtable (first field)
    let to_string_ptr_ptr = unsafe {
        builder.build_struct_gep(
            stringer_vtable_type,
            vtable_ptr,
            0,
            "to_string_ptr_ptr"
        ).unwrap()
    };
    
    // Load the function pointer
    let to_string_ptr = builder.build_load(
        person_to_string_type.ptr_type(AddressSpace::default()),
        to_string_ptr_ptr,
        "to_string_ptr"
    ).unwrap().into_pointer_value());
    
    // Call the to_string function with the person object (dynamic dispatch)
    // Need to use build_indirect_call for function pointers
    let result = builder.build_indirect_call(
        person_to_string_type,
        to_string_ptr,
        &[person_void_ptr.into()],
        "to_string_result"
    ).unwrap();
    
    // Return the result
    builder.build_return(Some(&result.try_as_basic_value().left().unwrap()).unwrap());
    
    // Verify the module
    if let Err(err) = module.verify() {
        panic!("Module verification failed: {}", err);
    }
    
    // If we got here, the test passed
    println!("Successfully created and verified interface-like implementation");
}