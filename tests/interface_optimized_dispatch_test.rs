use std::collections::HashMap;
use std::path::PathBuf;
use inkwell::context::Context;
use inkwell::types::BasicType;
use inkwell::values::BasicValue;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::InterfaceImplementation;
use cursed::codegen::llvm::OptimizedDynamicDispatch;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::error::Error;

// Test for optimized dynamic dispatch for interfaces


#[path = "common/mod.rs"]
mod common;

/// Test fixture with shape interfaces and implementing types
fn setup_shape_hierarchy() -> Result<TypeChecker, Error> {
    let mut type_checker = TypeChecker::new();
    
    // Register a Shape interface
    type_checker.register_interface(
        "Shape",
        vec![
            ("area".to_string(), vec![], Some(Type::Meal)),
            ("perimeter".to_string(), vec![], Some(Type::Meal)),
        ],
        Vec::new(),
    );
    
    // Register Circle struct
    let circle_fields = HashMap::from([
        ("radius".to_string(), Type::Meal),
    ]);
    
    type_checker.register_struct("Circle", circle_fields, Vec::new());
    
    // Register Circle methods
    let circle_methods = vec![
        ("area".to_string(), vec![], Some(Type::Meal)),
        ("perimeter".to_string(), vec![], Some(Type::Meal)),
    ];
    
    for (method_name, param_types, return_type) in circle_methods {
        type_checker.register_struct_method("Circle", &method_name, param_types, return_type)?;
    }
    
    // Register Rectangle struct
    let rectangle_fields = HashMap::from([
        ("width".to_string(), Type::Meal),
        ("height".to_string(), Type::Meal),
    ]);
    
    type_checker.register_struct("Rectangle", rectangle_fields, Vec::new());
    
    // Register Rectangle methods
    let rectangle_methods = vec![
        ("area".to_string(), vec![], Some(Type::Meal)),
        ("perimeter".to_string(), vec![], Some(Type::Meal)),
    ];
    
    for (method_name, param_types, return_type) in rectangle_methods {
        type_checker.register_struct_method("Rectangle", &method_name, param_types, return_type)?;
    }
    
    Ok(type_checker)
}

/// Test for optimized dynamic dispatch with shape interfaces
#[test]
fn test_optimized_dynamic_dispatch() -> Result<(), Error> {
    // Set up tracing
    common::tracing::setup();
    tracing::info!("Starting optimized dynamic dispatch test");
    
    // Create LLVM context and code generator
    let context = Context::create();
    let module_path = PathBuf::from("optimized_dispatch_test.bc");
    let mut codegen = LlvmCodeGenerator::new(&context, "optimized_dispatch_test", module_path);
    
    // Initialize the optimized dispatch system
    codegen.init_optimized_dynamic_dispatch()?;
    
    // Create type checker and register shapes
    let _type_checker = setup_shape_hierarchy()?;
    
    // Register Shape interface with code generator
    codegen.register_interface(
        "Shape",
        vec![
            ("area".to_string(), vec![], Some(Type::Meal)),
            ("perimeter".to_string(), vec![], Some(Type::Meal)),
        ],
        Vec::new(),
    )?;
    
    // Define LLVM struct type for Circle
    let circle_llvm_type = context.struct_type(
        &[context.f64_type().into()], // radius
        false
    );
    
    // Define LLVM struct type for Rectangle
    let rectangle_llvm_type = context.struct_type(
        &[
            context.f64_type().into(), // width
            context.f64_type().into(), // height
        ],
        false
    );
    
    // Create area function for Circle
    let area_fn_type = context.f64_type().fn_type(
        &[circle_llvm_type.ptr_type(inkwell::AddressSpace::default()).into()],
        false
    );
    
    let circle_area_function = codegen.module().add_function(
        "Circle.area",
        area_fn_type,
        None
    );
    
    // Implement area function for Circle (πr²)
    let entry_block = context.append_basic_block(circle_area_function, "entry");
    codegen.builder().position_at_end(entry_block);
    
    // Get self parameter
    let self_param = circle_area_function.get_first_param().unwrap()
        .into_pointer_value();
    
    // Get radius field
    let radius_ptr = unsafe {
        codegen.builder().build_struct_gep(
            circle_llvm_type,
            self_param,
            0,
            "radius_ptr"
        ).unwrap()
    };
    
    // Load radius
    let radius = codegen.builder()
        .build_load(
            context.f64_type(),
            radius_ptr,
            "radius"
        )
        .unwrap();
    
    // Calculate area = PI * r * r
    let pi = context.f64_type().const_float(std::f64::consts::PI);
    let radius_squared = codegen.builder()
        .build_float_mul(
            radius.into_float_value(),
            radius.into_float_value(),
            "radius_squared"
        )
        .unwrap();
    
    let area = codegen.builder()
        .build_float_mul(
            pi,
            radius_squared,
            "area"
        )
        .unwrap();
    
    // Return area
    codegen.builder().build_return(Some(&area)).unwrap();
    
    // Create perimeter function for Circle
    let perimeter_fn_type = context.f64_type().fn_type(
        &[circle_llvm_type.ptr_type(inkwell::AddressSpace::default()).into()],
        false
    );
    
    let circle_perimeter_function = codegen.module().add_function(
        "Circle.perimeter",
        perimeter_fn_type,
        None
    );
    
    // Implement perimeter function for Circle (2πr)
    let entry_block = context.append_basic_block(circle_perimeter_function, "entry");
    codegen.builder().position_at_end(entry_block);
    
    // Get self parameter
    let self_param = circle_perimeter_function.get_first_param().unwrap()
        .into_pointer_value();
    
    // Get radius field
    let radius_ptr = unsafe {
        codegen.builder().build_struct_gep(
            circle_llvm_type,
            self_param,
            0,
            "radius_ptr"
        ).unwrap()
    };
    
    // Load radius
    let radius = codegen.builder()
        .build_load(
            context.f64_type(),
            radius_ptr,
            "radius"
        )
        .unwrap();
    
    // Calculate perimeter = 2 * PI * r
    let pi = context.f64_type().const_float(std::f64::consts::PI);
    let two = context.f64_type().const_float(2.0);
    let two_pi = codegen.builder()
        .build_float_mul(
            two,
            pi,
            "two_pi"
        )
        .unwrap();
    
    let perimeter = codegen.builder()
        .build_float_mul(
            two_pi,
            radius.into_float_value(),
            "perimeter"
        )
        .unwrap();
    
    // Return perimeter
    codegen.builder().build_return(Some(&perimeter)).unwrap();
    
    // Register Circle as implementing Shape
    let mut circle_methods = HashMap::new();
    circle_methods.insert("area".to_string(), circle_area_function);
    circle_methods.insert("perimeter".to_string(), circle_perimeter_function);
    
    codegen.register_interface_implementation(
        "Circle",
        "Shape",
        circle_methods
    )?;
    
    // Create a test function to verify dynamic dispatch
    let test_fn_type = context.f64_type().fn_type(&[], false);
    let test_function = codegen.module().add_function(
        "test_optimized_dispatch",
        test_fn_type,
        None
    );
    
    let test_entry = context.append_basic_block(test_function, "entry");
    codegen.builder().position_at_end(test_entry);
    
    // Allocate Circle with radius 5.0
    let circle_ptr = codegen.builder()
        .build_alloca(circle_llvm_type, "circle")
        .unwrap();
    
    // Initialize Circle radius field
    let radius_ptr = unsafe {
        codegen.builder().build_struct_gep(
            circle_llvm_type,
            circle_ptr,
            0,
            "radius_ptr"
        ).unwrap()
    };
    
    // Store 5.0 as radius
    let radius_value = context.f64_type().const_float(5.0);
    codegen.builder().build_store(radius_ptr, radius_value).unwrap();
    
    // Convert Circle to Shape interface
    let circle_type = Type::Struct("Circle".to_string(), Vec::new());
    let shape_interface = codegen.create_interface_value(
        circle_ptr,
        &circle_type,
        "Shape"
    )?;
    
    // Call area method on the interface using optimized dispatch
    let area_result = codegen.call_interface_method_optimized(
        shape_interface,
        "Shape",
        "area",
        &[]
    )?;
    
    // Call perimeter method on the interface using optimized dispatch
    let perimeter_result = codegen.call_interface_method_optimized(
        shape_interface,
        "Shape",
        "perimeter",
        &[]
    )?;
    
    // Calculate the sum of area and perimeter
    let area_value = area_result.unwrap();
    let perimeter_value = perimeter_result.unwrap();
    
    let sum = codegen.builder()
        .build_float_add(
            area_value.into_float_value(),
            perimeter_value.into_float_value(),
            "sum"
        )
        .unwrap();
    
    // Return the result
    codegen.builder().build_return(Some(&sum)).unwrap();
    
    // Get dispatch statistics
    let stats = codegen.get_dispatch_statistics()?;
    tracing::info!("Dispatch statistics: {:?}", stats);
    
    // Reset statistics
    codegen.reset_dispatch_statistics()?;
    
    // Verify the module
    if let Err(message) = codegen.module().verify() {
        return Err(Error::from_str(&format!("Module verification error: {}", message.to_string())));
    }
    
    tracing::info!("Completed optimized dynamic dispatch test");
    Ok(())
}

/// Test for speculative dispatch with multiple shape types
#[test]
fn test_speculative_dispatch() -> Result<(), Error> {
    // Set up tracing
    common::tracing::setup();
    tracing::info!("Starting speculative dispatch test");
    
    // Create LLVM context and code generator
    let context = Context::create();
    let module_path = PathBuf::from("speculative_dispatch_test.bc");
    let mut codegen = LlvmCodeGenerator::new(&context, "speculative_dispatch_test", module_path);
    
    // Initialize the optimized dispatch system
    codegen.init_optimized_dynamic_dispatch()?;
    
    // Create type checker and register shapes
    let _type_checker = setup_shape_hierarchy()?;
    
    // Register Shape interface with code generator
    codegen.register_interface(
        "Shape",
        vec![
            ("area".to_string(), vec![], Some(Type::Meal)),
            ("perimeter".to_string(), vec![], Some(Type::Meal)),
        ],
        Vec::new(),
    )?;
    
    // Define LLVM struct type for Circle
    let circle_llvm_type = context.struct_type(
        &[context.f64_type().into()], // radius
        false
    );
    
    // Define LLVM struct type for Rectangle
    let rectangle_llvm_type = context.struct_type(
        &[
            context.f64_type().into(), // width
            context.f64_type().into(), // height
        ],
        false
    );
    
    // Create area function for Circle
    let area_fn_type = context.f64_type().fn_type(
        &[circle_llvm_type.ptr_type(inkwell::AddressSpace::default()).into()],
        false
    );
    
    let circle_area_function = codegen.module().add_function(
        "Circle.area",
        area_fn_type,
        None
    );
    
    // Implement area function for Circle (πr²)
    let entry_block = context.append_basic_block(circle_area_function, "entry");
    codegen.builder().position_at_end(entry_block);
    
    // Get self parameter
    let self_param = circle_area_function.get_first_param().unwrap()
        .into_pointer_value();
    
    // Get radius field
    let radius_ptr = unsafe {
        codegen.builder().build_struct_gep(
            circle_llvm_type,
            self_param,
            0,
            "radius_ptr"
        ).unwrap()
    };
    
    // Load radius
    let radius = codegen.builder()
        .build_load(
            context.f64_type(),
            radius_ptr,
            "radius"
        )
        .unwrap();
    
    // Calculate area = PI * r * r
    let pi = context.f64_type().const_float(std::f64::consts::PI);
    let radius_squared = codegen.builder()
        .build_float_mul(
            radius.into_float_value(),
            radius.into_float_value(),
            "radius_squared"
        )
        .unwrap();
    
    let area = codegen.builder()
        .build_float_mul(
            pi,
            radius_squared,
            "area"
        )
        .unwrap();
    
    // Return area
    codegen.builder().build_return(Some(&area)).unwrap();
    
    // Create perimeter function for Circle
    let perimeter_fn_type = context.f64_type().fn_type(
        &[circle_llvm_type.ptr_type(inkwell::AddressSpace::default()).into()],
        false
    );
    
    let circle_perimeter_function = codegen.module().add_function(
        "Circle.perimeter",
        perimeter_fn_type,
        None
    );
    
    // Implement perimeter function for Circle (2πr)
    let entry_block = context.append_basic_block(circle_perimeter_function, "entry");
    codegen.builder().position_at_end(entry_block);
    
    // Get self parameter
    let self_param = circle_perimeter_function.get_first_param().unwrap()
        .into_pointer_value();
    
    // Get radius field
    let radius_ptr = unsafe {
        codegen.builder().build_struct_gep(
            circle_llvm_type,
            self_param,
            0,
            "radius_ptr"
        ).unwrap()
    };
    
    // Load radius
    let radius = codegen.builder()
        .build_load(
            context.f64_type(),
            radius_ptr,
            "radius"
        )
        .unwrap();
    
    // Calculate perimeter = 2 * PI * r
    let pi = context.f64_type().const_float(std::f64::consts::PI);
    let two = context.f64_type().const_float(2.0);
    let two_pi = codegen.builder()
        .build_float_mul(
            two,
            pi,
            "two_pi"
        )
        .unwrap();
    
    let perimeter = codegen.builder()
        .build_float_mul(
            two_pi,
            radius.into_float_value(),
            "perimeter"
        )
        .unwrap();
    
    // Return perimeter
    codegen.builder().build_return(Some(&perimeter)).unwrap();
    
    // Create area function for Rectangle
    let rect_area_fn_type = context.f64_type().fn_type(
        &[rectangle_llvm_type.ptr_type(inkwell::AddressSpace::default()).into()],
        false
    );
    
    let rectangle_area_function = codegen.module().add_function(
        "Rectangle.area",
        rect_area_fn_type,
        None
    );
    
    // Implement area function for Rectangle (width * height)
    let entry_block = context.append_basic_block(rectangle_area_function, "entry");
    codegen.builder().position_at_end(entry_block);
    
    // Get self parameter
    let self_param = rectangle_area_function.get_first_param().unwrap()
        .into_pointer_value();
    
    // Get width field
    let width_ptr = unsafe {
        codegen.builder().build_struct_gep(
            rectangle_llvm_type,
            self_param,
            0,
            "width_ptr"
        ).unwrap()
    };
    
    // Get height field
    let height_ptr = unsafe {
        codegen.builder().build_struct_gep(
            rectangle_llvm_type,
            self_param,
            1,
            "height_ptr"
        ).unwrap()
    };
    
    // Load width and height
    let width = codegen.builder()
        .build_load(
            context.f64_type(),
            width_ptr,
            "width"
        )
        .unwrap();
    
    let height = codegen.builder()
        .build_load(
            context.f64_type(),
            height_ptr,
            "height"
        )
        .unwrap();
    
    // Calculate area = width * height
    let area = codegen.builder()
        .build_float_mul(
            width.into_float_value(),
            height.into_float_value(),
            "area"
        )
        .unwrap();
    
    // Return area
    codegen.builder().build_return(Some(&area)).unwrap();
    
    // Create perimeter function for Rectangle
    let rect_perimeter_fn_type = context.f64_type().fn_type(
        &[rectangle_llvm_type.ptr_type(inkwell::AddressSpace::default()).into()],
        false
    );
    
    let rectangle_perimeter_function = codegen.module().add_function(
        "Rectangle.perimeter",
        rect_perimeter_fn_type,
        None
    );
    
    // Implement perimeter function for Rectangle (2 * (width + height))
    let entry_block = context.append_basic_block(rectangle_perimeter_function, "entry");
    codegen.builder().position_at_end(entry_block);
    
    // Get self parameter
    let self_param = rectangle_perimeter_function.get_first_param().unwrap()
        .into_pointer_value();
    
    // Get width field
    let width_ptr = unsafe {
        codegen.builder().build_struct_gep(
            rectangle_llvm_type,
            self_param,
            0,
            "width_ptr"
        ).unwrap()
    };
    
    // Get height field
    let height_ptr = unsafe {
        codegen.builder().build_struct_gep(
            rectangle_llvm_type,
            self_param,
            1,
            "height_ptr"
        ).unwrap()
    };
    
    // Load width and height
    let width = codegen.builder()
        .build_load(
            context.f64_type(),
            width_ptr,
            "width"
        )
        .unwrap();
    
    let height = codegen.builder()
        .build_load(
            context.f64_type(),
            height_ptr,
            "height"
        )
        .unwrap();
    
    // Calculate perimeter = 2 * (width + height)
    let width_plus_height = codegen.builder()
        .build_float_add(
            width.into_float_value(),
            height.into_float_value(),
            "width_plus_height"
        )
        .unwrap();
    
    let two = context.f64_type().const_float(2.0);
    let perimeter = codegen.builder()
        .build_float_mul(
            two,
            width_plus_height,
            "perimeter"
        )
        .unwrap();
    
    // Return perimeter
    codegen.builder().build_return(Some(&perimeter)).unwrap();
    
    // Register Circle as implementing Shape
    let mut circle_methods = HashMap::new();
    circle_methods.insert("area".to_string(), circle_area_function);
    circle_methods.insert("perimeter".to_string(), circle_perimeter_function);
    
    codegen.register_interface_implementation(
        "Circle",
        "Shape",
        circle_methods
    )?;
    
    // Register Rectangle as implementing Shape
    let mut rectangle_methods = HashMap::new();
    rectangle_methods.insert("area".to_string(), rectangle_area_function);
    rectangle_methods.insert("perimeter".to_string(), rectangle_perimeter_function);
    
    codegen.register_interface_implementation(
        "Rectangle",
        "Shape",
        rectangle_methods
    )?;
    
    // Create a test function to verify speculative dispatch
    let test_fn_type = context.f64_type().fn_type(&[], false);
    let test_function = codegen.module().add_function(
        "test_speculative_dispatch",
        test_fn_type,
        None
    );
    
    let test_entry = context.append_basic_block(test_function, "entry");
    codegen.builder().position_at_end(test_entry);
    
    // Allocate Circle with radius 5.0
    let circle_ptr = codegen.builder()
        .build_alloca(circle_llvm_type, "circle")
        .unwrap();
    
    // Initialize Circle radius field
    let radius_ptr = unsafe {
        codegen.builder().build_struct_gep(
            circle_llvm_type,
            circle_ptr,
            0,
            "radius_ptr"
        ).unwrap()
    };
    
    // Store 5.0 as radius
    let radius_value = context.f64_type().const_float(5.0);
    codegen.builder().build_store(radius_ptr, radius_value).unwrap();
    
    // Allocate Rectangle with width 4.0 and height 6.0
    let rectangle_ptr = codegen.builder()
        .build_alloca(rectangle_llvm_type, "rectangle")
        .unwrap();
    
    // Initialize Rectangle width field
    let width_ptr = unsafe {
        codegen.builder().build_struct_gep(
            rectangle_llvm_type,
            rectangle_ptr,
            0,
            "width_ptr"
        ).unwrap()
    };
    
    // Store 4.0 as width
    let width_value = context.f64_type().const_float(4.0);
    codegen.builder().build_store(width_ptr, width_value).unwrap();
    
    // Initialize Rectangle height field
    let height_ptr = unsafe {
        codegen.builder().build_struct_gep(
            rectangle_llvm_type,
            rectangle_ptr,
            1,
            "height_ptr"
        ).unwrap()
    };
    
    // Store 6.0 as height
    let height_value = context.f64_type().const_float(6.0);
    codegen.builder().build_store(height_ptr, height_value).unwrap();
    
    // Convert Circle to Shape interface
    let circle_type = Type::Struct("Circle".to_string(), Vec::new());
    let circle_shape = codegen.create_interface_value(
        circle_ptr,
        &circle_type,
        "Shape"
    )?;
    
    // Convert Rectangle to Shape interface
    let rectangle_type = Type::Struct("Rectangle".to_string(), Vec::new());
    let rectangle_shape = codegen.create_interface_value(
        rectangle_ptr,
        &rectangle_type,
        "Shape"
    )?;
    
    // Create an array of shape interfaces
    let shapes_array_type = context.struct_type(
        &[
            context.struct_type(&[
                context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
                context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
            ], false).into(),
            context.struct_type(&[
                context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
                context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
            ], false).into(),
        ],
        false
    );
    
    // Allocate the shapes array
    let shapes_array_ptr = codegen.builder()
        .build_alloca(shapes_array_type, "shapes_array")
        .unwrap();
    
    // Store Circle shape in the array
    let circle_slot_ptr = unsafe {
        codegen.builder().build_struct_gep(
            shapes_array_type,
            shapes_array_ptr,
            0,
            "circle_slot_ptr"
        ).unwrap()
    };
    
    codegen.builder().build_store(circle_slot_ptr, circle_shape).unwrap();
    
    // Store Rectangle shape in the array
    let rectangle_slot_ptr = unsafe {
        codegen.builder().build_struct_gep(
            shapes_array_type,
            shapes_array_ptr,
            1,
            "rectangle_slot_ptr"
        ).unwrap()
    };
    
    codegen.builder().build_store(rectangle_slot_ptr, rectangle_shape).unwrap();
    
    // Create a loop to calculate total area of all shapes
    // Loop initialization - total area
    let total_area = codegen.builder().build_alloca(context.f64_type(), "total_area").unwrap();
    let zero = context.f64_type().const_float(0.0);
    codegen.builder().build_store(total_area, zero).unwrap();
    
    // Load shape instances
    let circle_shape = codegen.builder()
        .build_load(
            circle_shape.get_type(),
            circle_slot_ptr,
            "loaded_circle"
        )
        .unwrap()
        .into_pointer_value();
    
    let rectangle_shape = codegen.builder()
        .build_load(
            rectangle_shape.get_type(),
            rectangle_slot_ptr,
            "loaded_rectangle"
        )
        .unwrap()
        .into_pointer_value();
    
    // The following code would be in a loop in a real implementation
    // For simplicity, we're manually calling area on each shape
    
    // Call area for circle several times to collect statistics
    for _ in 0..15 {
        // Call area method on circle
        let circle_area = codegen.call_interface_method_optimized(
            circle_shape,
            "Shape",
            "area",
            &[]
        )?;
        
        // Add to total
        let current_total = codegen.builder()
            .build_load(
                context.f64_type(),
                total_area,
                "current_total"
            )
            .unwrap();
        
        let new_total = codegen.builder()
            .build_float_add(
                current_total.into_float_value(),
                circle_area.unwrap().into_float_value(),
                "new_total"
            )
            .unwrap();
        
        codegen.builder().build_store(total_area, new_total).unwrap();
    }
    
    // Call area for rectangle several times
    for _ in 0..15 {
        // Call area method on rectangle
        let rectangle_area = codegen.call_interface_method_optimized(
            rectangle_shape,
            "Shape",
            "area",
            &[]
        )?;
        
        // Add to total
        let current_total = codegen.builder()
            .build_load(
                context.f64_type(),
                total_area,
                "current_total"
            )
            .unwrap();
        
        let new_total = codegen.builder()
            .build_float_add(
                current_total.into_float_value(),
                rectangle_area.unwrap().into_float_value(),
                "new_total"
            )
            .unwrap();
        
        codegen.builder().build_store(total_area, new_total).unwrap();
    }
    
    // After enough calls, the system should use speculative dispatch
    // Get the final result
    let result = codegen.builder()
        .build_load(
            context.f64_type(),
            total_area,
            "result"
        )
        .unwrap();
    
    // Return the result
    codegen.builder().build_return(Some(&result)).unwrap();
    
    // Get dispatch statistics
    let stats = codegen.get_dispatch_statistics()?;
    tracing::info!("Dispatch statistics: {:?}", stats);
    
    // Verify the module
    if let Err(message) = codegen.module().verify() {
        return Err(Error::from_str(&format!("Module verification error: {}", message.to_string())));
    }
    
    tracing::info!("Completed speculative dispatch test");
    Ok(())
}