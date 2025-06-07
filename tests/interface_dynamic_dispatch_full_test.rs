use std::collections::HashMap;
use std::path::PathBuf;
use inkwell::context::Context;
use inkwell::types::BasicType;
use inkwell::values::BasicValue;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::InterfaceImplementation;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::error::Error;

// Full tests for interface dynamic dispatch in LLVM code generation


#[path = "tracing_setup.rs"]
pub mod tracing_setup;

/// Test interface implementation and dynamic dispatch with a Reader interface
#[test]
fn test_reader_interface_dynamic_dispatch() -> Result<(), Error> {
    // Set up tracing
    tracing_setup::init_test_tracing();
    
    // Create LLVM context and code generator
    let context = Context::create();
    let module_path = PathBuf::from("reader_interface_test.bc");
    let mut codegen = LlvmCodeGenerator::new(&context, "reader_interface_test", module_path);
    
    // Create type checker
    let mut type_checker = TypeChecker::new();
    
    // 1. Register Reader interface with a read method that takes a buffer and returns bytes read
    type_checker.register_interface(
        "Reader",
        vec![("read".to_string(), 
             vec![Type::Pointer(Box::new(Type::Tea)),  // buffer
                 Type::Normie],                       // offset
             Some(Type::Normie))],                    // returns bytes read
        Vec::new()
    );
    
    // Register interface with code generator
    codegen.register_interface(
        "Reader",
        vec![("read".to_string(), 
              vec![Type::Pointer(Box::new(Type::Tea)),
                  Type::Normie],
              Some(Type::Normie))],
        Vec::new()
    )?;
    
    // 2. Register FileReader struct
    let file_reader_fields = HashMap::from([
        ("path".to_string(), Type::Tea),
        ("position".to_string(), Type::Normie),
    ]);
    
    type_checker.register_struct("FileReader", file_reader_fields, Vec::new();
    
    // 3. Register read method for FileReader
    type_checker.register_struct_method(
        "FileReader",
        "read",
        vec![Type::Pointer(Box::new(Type::Tea)), Type::Normie],
        Some(Type::Normie)
    )?;
    
    // 4. Verify FileReader implements Reader
    let file_reader_type = Type::Struct("FileReader".to_string(), Vec::new();
    let reader_interface_type = Type::Interface("Reader".to_string(), Vec::new();
    
    let implements = type_checker.check_interface_implementation(
        &file_reader_type,
        &reader_interface_type
    )?;
    
    assert!(implements, "FileReader should implement Reader");
    
    // 5. Define LLVM struct type for FileReader
    let file_reader_llvm_type = context.struct_type(
        &[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // path
            context.i32_type().into(), // position
        ],
        false
    );
    
    // 6. Create read function for FileReader
    // Function signature: fn read(self: *FileReader, buffer: *i8, offset: i32) -> i32
    let read_fn_type = context.i32_type().fn_type(
        &[
            file_reader_llvm_type.ptr_type(inkwell::AddressSpace::default()).into(),
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
            context.i32_type().into(),
        ],
        false
    );
    
    let read_function = codegen.module().add_function(
        "FileReader.read",
        read_fn_type,
        None
    );
    
    // Implement the read function that returns 42 (bytes read)
    let entry_block = context.append_basic_block(read_function, "entry");
    codegen.builder().position_at_end(entry_block);
    
    // Return a constant value of 42 (simulating bytes read)
    let return_value = context.i32_type().const_int(42, false);
    codegen.builder().build_return(Some(&return_value)).unwrap());
    
    // 7. Register FileReader as implementing Reader
    let mut reader_methods = HashMap::new();
    reader_methods.insert("read".to_string(, read_function);
    
    codegen.register_interface_implementation(
        "FileReader",
        "Reader",
        reader_methods
    )?;
    
    // 8. Create a test function to verify dynamic dispatch
    // This function:
    // - Creates a FileReader
    // - Converts it to a Reader interface
    // - Calls the read method through the interface
    // - Returns the result
    let test_fn_type = context.i32_type().fn_type(&[], false);
    let test_function = codegen.module().add_function(
        "test_reader_dispatch",
        test_fn_type,
        None
    );
    
    let test_entry = context.append_basic_block(test_function, "entry");
    codegen.builder().position_at_end(test_entry);
    
    // Allocate FileReader
    let file_reader_ptr = codegen.builder()
        .build_alloca(file_reader_llvm_type, "file_reader")
        .unwrap();
    
    // Initialize FileReader fields
    // Get pointer to path field
    let path_ptr = unsafe {
        codegen.builder().build_struct_gep(
            file_reader_llvm_type,
            file_reader_ptr,
            0,
            "path_ptr"
        ).unwrap()
    };
    
    // Store empty string as path
    let empty_string = codegen.create_string_constant("test.txt");
    // Need to handle the result
    codegen.builder().build_store(path_ptr, empty_string.unwrap()).unwrap());
    
    // Get pointer to position field
    let position_ptr = unsafe {
        codegen.builder().build_struct_gep(
            file_reader_llvm_type,
            file_reader_ptr,
            1,
            "position_ptr"
        ).unwrap()
    };
    
    // Store 0 as position
    let zero = context.i32_type().const_int(0, false);
    codegen.builder().build_store(position_ptr, zero).unwrap());
    
    // Convert FileReader to Reader interface
    let reader_interface = codegen.create_interface_value(
        file_reader_ptr,
        &file_reader_type,
        "Reader"
    )?;
    
    // Allocate a buffer for reading
    let buffer_type = context.i8_type().array_type(100);
    let buffer_ptr = codegen.builder()
        .build_alloca(buffer_type, "buffer")
        .unwrap();
    
    // Cast array to pointer
    let buffer_i8_ptr = codegen.builder()
        .build_bitcast(
            buffer_ptr,
            context.i8_type().ptr_type(inkwell::AddressSpace::default()),
            "buffer_i8_ptr"
        )
        .unwrap()
        .into_pointer_value();
    
    // Call read method on the interface
    let args = [
        buffer_i8_ptr.into(),
        zero.into(),
    ];
    
    let result = codegen.call_interface_method(
        reader_interface,
        "Reader",
        "read",
        &args
    )?;
    
    // Return the result
    if let Some(read_result) = result {
        codegen.builder().build_return(Some(&read_result)).unwrap());
    } else {
        let default = context.i32_type().const_int(0, false);
        codegen.builder().build_return(Some(&default)).unwrap());
    }
    
    // Verify the module
    if let Err(message) = codegen.module().verify() {
        return Err(Error::from_str(&format!("Module verification error: {}", message.to_string());
    }
    
    Ok(())
}

/// Test interface type assertions and conversions
#[test]
fn test_interface_type_assertion() -> Result<(), Error> {
    // Set up tracing
    tracing_setup::init_test_tracing();
    
    // Create LLVM context and code generator
    let context = Context::create();
    let module_path = PathBuf::from("type_assertion_test.bc");
    let mut codegen = LlvmCodeGenerator::new(&context, "type_assertion_test", module_path);
    
    // Create type checker
    let mut type_checker = TypeChecker::new();
    
    // 1. Register Stringer interface
    type_checker.register_interface(
        "Stringer",
        vec![("to_string".to_string(), vec![], Some(Type::Tea))],
        Vec::new()
    );
    
    codegen.register_interface(
        "Stringer",
        vec![("to_string".to_string(), vec![], Some(Type::Tea))],
        Vec::new()
    )?;
    
    // 2. Register Person struct
    let person_fields = HashMap::from([
        ("name".to_string(), Type::Tea),
        ("age".to_string(), Type::Normie),
    ]);
    
    type_checker.register_struct("Person", person_fields, Vec::new();
    
    // 3. Register to_string method for Person
    type_checker.register_struct_method(
        "Person",
        "to_string",
        vec![],
        Some(Type::Tea)
    )?;
    
    // 4. Define LLVM struct type for Person
    let person_llvm_type = context.struct_type(
        &[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // name
            context.i32_type().into(), // age
        ],
        false
    );
    
    // 5. Create to_string function for Person
    let to_string_fn_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(
            &[person_llvm_type.ptr_type(inkwell::AddressSpace::default()).into()],
            false
        );
    
    let to_string_function = codegen.module().add_function(
        "Person.to_string",
        to_string_fn_type,
        None
    );
    
    // Implement the to_string function to return the name field
    let entry_block = context.append_basic_block(to_string_function, "entry");
    codegen.builder().position_at_end(entry_block);
    
    // Get the self parameter
    let self_param = to_string_function.get_first_param().unwrap()
        .into_pointer_value();
    
    // Get pointer to name field
    let name_ptr = unsafe {
        codegen.builder().build_struct_gep(
            person_llvm_type,
            self_param,
            0,
            "name_ptr"
        ).unwrap()
    };
    
    // Load name and return it
    let name = codegen.builder()
        .build_load(
            context.i8_type().ptr_type(inkwell::AddressSpace::default()),
            name_ptr,
            "name"
        )
        .unwrap();
    
    codegen.builder().build_return(Some(&name)).unwrap());
    
    // 6. Register Person as implementing Stringer
    let mut stringer_methods = HashMap::new();
    stringer_methods.insert("to_string".to_string(, to_string_function);
    
    codegen.register_interface_implementation(
        "Person",
        "Stringer",
        stringer_methods
    )?;
    
    // 7. Create test function for type assertion
    let test_fn_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[], false);
    
    let test_function = codegen.module().add_function(
        "test_type_assertion",
        test_fn_type,
        None
    );
    
    let test_entry = context.append_basic_block(test_function, "entry");
    codegen.builder().position_at_end(test_entry);
    
    // Create a Person
    let person_ptr = codegen.builder()
        .build_alloca(person_llvm_type, "person")
        .unwrap();
    
    // Initialize Person fields
    // Get pointer to name field
    let name_ptr = unsafe {
        codegen.builder().build_struct_gep(
            person_llvm_type,
            person_ptr,
            0,
            "name_ptr"
        ).unwrap()
    };
    
    // Store "Alice" as name
    let name_str = codegen.create_string_constant("Alice");
    // Need to handle the result
    codegen.builder().build_store(name_ptr, name_str.unwrap()).unwrap());
    
    // Get pointer to age field
    let age_ptr = unsafe {
        codegen.builder().build_struct_gep(
            person_llvm_type,
            person_ptr,
            1,
            "age_ptr"
        ).unwrap()
    };
    
    // Store 30 as age
    let age = context.i32_type().const_int(30, false);
    codegen.builder().build_store(age_ptr, age).unwrap());
    
    // Convert Person to Stringer interface
    let person_type = Type::Struct("Person".to_string(), Vec::new();
    let stringer_interface = codegen.create_interface_value(
        person_ptr,
        &person_type,
        "Stringer"
    )?;
    
    // Type assertion: Check if stringer is a Person
    let is_person = codegen.check_instance_of(
        stringer_interface,
        "Person"
    )?;
    
    // Branches for type assertion result
    let success_block = context.append_basic_block(test_function, "success");
    let failure_block = context.append_basic_block(test_function, "failure");
    
    codegen.builder().build_conditional_branch(
        is_person.into_int_value(),
        success_block,
        failure_block
    ).unwrap();
    
    // Success case - cast back to Person and return its name
    codegen.builder().position_at_end(success_block);
    
    // Cast interface to Person
    let person_from_interface = codegen.compile_interface_type_assertion(
        stringer_interface,
        &person_type
    )?;
    
    // Get name field from the Person
    let name_ptr = unsafe {
        codegen.builder().build_struct_gep(
            person_llvm_type,
            person_from_interface,
            0,
            "name_ptr_from_interface"
        ).unwrap()
    };
    
    // Load and return name
    let name = codegen.builder()
        .build_load(
            context.i8_type().ptr_type(inkwell::AddressSpace::default()),
            name_ptr,
            "name_from_interface"
        )
        .unwrap();
    
    codegen.builder().build_return(Some(&name)).unwrap());
    
    // Failure case - return "not a Person"
    codegen.builder().position_at_end(failure_block);
    let error_str = codegen.create_string_constant("not a Person");
    // Need to handle the result
    codegen.builder().build_return(Some(&error_str.unwrap()).unwrap());
    
    // Verify the module
    if let Err(message) = codegen.module().verify() {
        return Err(Error::from_str(&format!("Module verification error: {}", message.to_string());
    }
    
    Ok(())
}