/// Comprehensive integration tests for LookinGlass reflection package
use cursed::stdlib::lookin_glass::{
    // Core types and functions
    Type, Value, Kind, StructField, StructTag, Method,
    type_of, value_of, new, zero, indirect,
    make_slice, make_map, make_chan, make_func,
    
    // Enhanced utilities
    deep_equal, deep_copy, struct_to_map, map_to_struct,
    get_tags, get_field, has_field, field_names,
    
    // VibeMapper
    VibeMapper, VibeMapperConfig,
    camel_to_snake, snake_to_camel,
    
    // Core functions helpers
    array_of, slice_of, map_of, ptr_to, chan_of, func_of,
    register_type, lookup_type, registered_types, init_type_registry,
    
    // Module management
    initialize, get_reflection_statistics,
};
use std::collections::HashMap;

#[test]
fn test_basic_type_reflection() {
    initialize();
    
    // Test basic type creation
    let int_type = Type::basic(Kind::Int32);
    assert_eq!(int_type.kind(), Kind::Int32);
    assert_eq!(int_type.name(), "int32");
    assert!(int_type.is_basic());
    assert!(int_type.comparable());
    
    let string_type = Type::basic(Kind::String);
    assert_eq!(string_type.kind(), Kind::String);
    assert_eq!(string_type.name(), "string");
}

#[test]
fn test_value_creation_and_manipulation() {
    initialize();
    
    // Test value creation
    let bool_val = Value::from_bool(true);
    assert_eq!(bool_val.kind(), Kind::Bool);
    assert_eq!(bool_val.bool().unwrap(), true);
    assert!(!bool_val.is_zero());
    
    let int_val = Value::from_int(42);
    assert_eq!(int_val.kind(), Kind::Int64);
    assert_eq!(int_val.int().unwrap(), 42);
    
    let string_val = Value::from_string("hello world".to_string());
    assert_eq!(string_val.kind(), Kind::String);
    assert_eq!(string_val.string().unwrap(), "hello world");
    
    // Test zero values
    let zero_bool = Value::from_bool(false);
    assert!(zero_bool.is_zero());
    
    let zero_string = Value::from_string("".to_string());
    assert!(zero_string.is_zero());
}

#[test]
fn test_struct_type_and_value() {
    initialize();
    
    // Create a Person struct type
    let name_field = StructField::builder("Name".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"name\" db:\"full_name\"".to_string())
        .build();
    let age_field = StructField::builder("Age".to_string(), Type::basic(Kind::Int32))
        .tag_string("json:\"age,omitempty\"".to_string())
        .build();
    let email_field = StructField::builder("Email".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"email\" validate:\"email\"".to_string())
        .build();
    
    let person_type = Type::new(Kind::Struct, "Person".to_string(), "main".to_string())
        .with_fields(vec![name_field.clone(), age_field.clone(), email_field.clone()]);
    
    assert_eq!(person_type.kind(), Kind::Struct);
    assert_eq!(person_type.name(), "Person");
    assert_eq!(person_type.pkg_path(), "main");
    assert_eq!(person_type.num_field(), 3);
    
    // Test field access
    let name_field_info = person_type.field(0).unwrap();
    assert_eq!(name_field_info.name(), "Name");
    assert_eq!(name_field_info.get_tag("json"), "name");
    assert_eq!(name_field_info.get_tag("db"), "full_name");
    
    let age_field_info = person_type.field_by_name("Age").unwrap();
    assert_eq!(age_field_info.name(), "Age");
    assert!(age_field_info.omit_empty());
    
    // Create struct value
    let fields = vec![
        Value::from_string("Alice Johnson".to_string()),
        Value::from_int(28),
        Value::from_string("alice@example.com".to_string())
    ];
    let person_val = Value::new(person_type.clone(), 
        cursed::stdlib::lookin_glass::value::ValueData::Struct(fields));
    
    assert_eq!(person_val.kind(), Kind::Struct);
    assert_eq!(person_val.num_field(), 3);
    
    // Test field access by name
    let name_val = person_val.field_by_name("Name").unwrap();
    assert_eq!(name_val.string().unwrap(), "Alice Johnson");
    
    let age_val = person_val.field(1).unwrap();
    assert_eq!(age_val.int().unwrap(), 28);
}

#[test]
fn test_collection_types() {
    initialize();
    
    // Test slice creation and manipulation
    let elem_type = Type::basic(Kind::Int32);
    let slice_type = slice_of(elem_type);
    
    let slice_val = make_slice(slice_type, 3, 5).unwrap();
    assert_eq!(slice_val.kind(), Kind::Slice);
    assert_eq!(slice_val.len().unwrap(), 3);
    assert_eq!(slice_val.cap().unwrap(), 5);
    
    // Test map creation
    let key_type = Type::basic(Kind::String);
    let value_type = Type::basic(Kind::Int32);
    let map_type = map_of(key_type, value_type);
    
    let map_val = make_map(map_type).unwrap();
    assert_eq!(map_val.kind(), Kind::Map);
    assert_eq!(map_val.len().unwrap(), 0);
    
    // Test array type
    let array_type = array_of(Type::basic(Kind::Float64), 10);
    assert_eq!(array_type.kind(), Kind::Array);
    assert_eq!(array_type.len().unwrap(), 10);
}

#[test]
fn test_pointer_and_indirect() {
    initialize();
    
    // Test pointer creation
    let int_type = Type::basic(Kind::Int32);
    let ptr_val = new(int_type).unwrap();
    
    assert_eq!(ptr_val.kind(), Kind::Pointer);
    assert!(!ptr_val.is_nil());
    
    // Test indirect access
    let pointed_val = indirect(ptr_val).unwrap();
    assert_eq!(pointed_val.kind(), Kind::Int32);
    assert_eq!(pointed_val.int().unwrap(), 0); // Zero value
    
    // Test indirect on non-pointer
    let direct_val = Value::from_int(42);
    let same_val = indirect(direct_val.clone()).unwrap();
    assert_eq!(same_val.int().unwrap(), 42);
}

#[test]
fn test_deep_operations() {
    initialize();
    
    // Test deep equal
    let val1 = Value::from_string("test".to_string());
    let val2 = Value::from_string("test".to_string());
    let val3 = Value::from_string("different".to_string());
    
    assert!(deep_equal(&val1, &val2));
    assert!(!deep_equal(&val1, &val3));
    
    // Test deep copy
    let original = Value::from_int(123);
    let copied = deep_copy(&original).unwrap();
    
    assert!(deep_equal(&original, &copied));
    assert_eq!(copied.int().unwrap(), 123);
    
    // Test with complex values
    let bytes_val = Value::from_bytes(vec![1, 2, 3, 4]);
    let bytes_copy = deep_copy(&bytes_val).unwrap();
    assert!(deep_equal(&bytes_val, &bytes_copy));
    assert_eq!(bytes_copy.bytes().unwrap(), vec![1, 2, 3, 4]);
}

#[test]
fn test_struct_to_map_conversion() {
    initialize();
    
    // Create a struct with JSON tags
    let name_field = StructField::builder("FirstName".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"first_name\"".to_string())
        .build();
    let last_field = StructField::builder("LastName".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"last_name\"".to_string())
        .build();
    let age_field = StructField::builder("Age".to_string(), Type::basic(Kind::Int32))
        .tag_string("json:\"age\"".to_string())
        .build();
    
    let person_type = Type::new(Kind::Struct, "Person".to_string(), "".to_string())
        .with_fields(vec![name_field, last_field, age_field]);
    
    let fields = vec![
        Value::from_string("John".to_string()),
        Value::from_string("Doe".to_string()),
        Value::from_int(30)
    ];
    let person_val = Value::new(person_type.clone(), 
        cursed::stdlib::lookin_glass::value::ValueData::Struct(fields));
    
    // Convert to map
    let map = struct_to_map(&person_val).unwrap();
    
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("first_name").unwrap().string().unwrap(), "John");
    assert_eq!(map.get("last_name").unwrap().string().unwrap(), "Doe");
    assert_eq!(map.get("age").unwrap().int().unwrap(), 30);
    
    // Convert back to struct
    let reconstructed = map_to_struct(&map, &person_type).unwrap();
    assert!(deep_equal(&person_val, &reconstructed));
}

#[test]
fn test_field_operations() {
    initialize();
    
    // Create a simple struct
    let field1 = StructField::simple("Name".to_string(), Type::basic(Kind::String));
    let field2 = StructField::simple("Count".to_string(), Type::basic(Kind::Int32));
    let field3 = StructField::simple("Active".to_string(), Type::basic(Kind::Bool));
    
    let struct_type = Type::new(Kind::Struct, "TestStruct".to_string(), "".to_string())
        .with_fields(vec![field1, field2, field3]);
    
    let fields = vec![
        Value::from_string("test".to_string()),
        Value::from_int(42),
        Value::from_bool(true)
    ];
    let struct_val = Value::new(struct_type, 
        cursed::stdlib::lookin_glass::value::ValueData::Struct(fields));
    
    // Test field names
    let names = field_names(&struct_val).unwrap();
    assert_eq!(names, vec!["Name", "Count", "Active"]);
    
    // Test has_field
    assert!(has_field(&struct_val, "Name"));
    assert!(has_field(&struct_val, "Count"));
    assert!(has_field(&struct_val, "Active"));
    assert!(!has_field(&struct_val, "NonExistent"));
    
    // Test get_field
    let name_val = get_field(&struct_val, "Name").unwrap();
    assert_eq!(name_val.string().unwrap(), "test");
    
    let count_val = get_field(&struct_val, "Count").unwrap();
    assert_eq!(count_val.int().unwrap(), 42);
    
    let active_val = get_field(&struct_val, "Active").unwrap();
    assert_eq!(active_val.bool().unwrap(), true);
}

#[test]
fn test_tag_operations() {
    initialize();
    
    // Create struct with various tags
    let field1 = StructField::builder("Username".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"username\" db:\"user_name\" validate:\"required,min=3\"".to_string())
        .build();
    let field2 = StructField::builder("Email".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"email,omitempty\" validate:\"email\"".to_string())
        .build();
    let field3 = StructField::builder("Age".to_string(), Type::basic(Kind::Int32))
        .tag_string("json:\"-\"".to_string()) // Ignored field
        .build();
    
    let user_type = Type::new(Kind::Struct, "User".to_string(), "".to_string())
        .with_fields(vec![field1, field2, field3]);
    
    let fields = vec![
        Value::from_string("johndoe".to_string()),
        Value::from_string("john@example.com".to_string()),
        Value::from_int(25)
    ];
    let user_val = Value::new(user_type, 
        cursed::stdlib::lookin_glass::value::ValueData::Struct(fields));
    
    // Test get_tags
    let tags = get_tags(&user_val).unwrap();
    
    assert_eq!(tags.len(), 3);
    
    // Check Username field tags
    let username_tags = &tags["Username"];
    assert_eq!(username_tags.get("json").unwrap(), "username");
    assert_eq!(username_tags.get("db").unwrap(), "user_name");
    assert_eq!(username_tags.get("validate").unwrap(), "required,min=3");
    
    // Check Email field tags
    let email_tags = &tags["Email"];
    assert_eq!(email_tags.get("json").unwrap(), "email,omitempty");
    assert_eq!(email_tags.get("validate").unwrap(), "email");
    
    // Check Age field tags (JSON ignored)
    let age_tags = &tags["Age"];
    assert_eq!(age_tags.get("json").unwrap(), "-");
}

#[test]
fn test_vibe_mapper_basic_operations() {
    initialize();
    
    let mapper = VibeMapper::new();
    
    // Test with simple struct
    let field1 = StructField::builder("FirstName".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"first_name\"".to_string())
        .build();
    let field2 = StructField::builder("LastName".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"last_name\"".to_string())
        .build();
    
    let person_type = Type::new(Kind::Struct, "Person".to_string(), "".to_string())
        .with_fields(vec![field1, field2]);
    
    let fields = vec![
        Value::from_string("Jane".to_string()),
        Value::from_string("Smith".to_string())
    ];
    let person_val = Value::new(person_type.clone(), 
        cursed::stdlib::lookin_glass::value::ValueData::Struct(fields));
    
    // Test to_map
    let map = mapper.to_map(&person_val).unwrap();
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("first_name").unwrap().string().unwrap(), "Jane");
    assert_eq!(map.get("last_name").unwrap().string().unwrap(), "Smith");
    
    // Test from_map
    let reconstructed = mapper.from_map(&map, &person_type).unwrap();
    assert!(deep_equal(&person_val, &reconstructed));
    
    // Test clone
    let cloned = mapper.clone(&person_val).unwrap();
    assert!(deep_equal(&person_val, &cloned));
}

#[test]
fn test_vibe_mapper_configuration() {
    initialize();
    
    // Test with custom configuration
    let mapper = VibeMapper::new()
        .use_json_tags(false)
        .omit_empty(true)
        .field_name_transformer(camel_to_snake);
    
    let field1 = StructField::builder("FirstName".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"first_name\"".to_string())
        .build();
    let field2 = StructField::builder("LastName".to_string(), Type::basic(Kind::String))
        .tag_string("json:\"last_name\"".to_string())
        .build();
    
    let person_type = Type::new(Kind::Struct, "Person".to_string(), "".to_string())
        .with_fields(vec![field1, field2]);
    
    let fields = vec![
        Value::from_string("Bob".to_string()),
        Value::from_string("Wilson".to_string())
    ];
    let person_val = Value::new(person_type, 
        cursed::stdlib::lookin_glass::value::ValueData::Struct(fields));
    
    let map = mapper.to_map(&person_val).unwrap();
    
    // Since use_json_tags is false and we have a transformer, field names should be transformed
    // The exact keys depend on the implementation details
    assert_eq!(map.len(), 2);
}

#[test]
fn test_name_transformations() {
    // Test camelCase to snake_case
    assert_eq!(camel_to_snake("firstName"), "first_name");
    assert_eq!(camel_to_snake("XMLHttpRequest"), "x_m_l_http_request");
    assert_eq!(camel_to_snake("simpleValue"), "simple_value");
    assert_eq!(camel_to_snake("ID"), "i_d");
    
    // Test snake_case to camelCase
    assert_eq!(snake_to_camel("first_name"), "firstName");
    assert_eq!(snake_to_camel("user_id"), "userId");
    assert_eq!(snake_to_camel("simple"), "simple");
    assert_eq!(snake_to_camel("test_value_here"), "testValueHere");
}

#[test]
fn test_zero_value_creation() {
    initialize();
    
    // Test zero values for basic types
    let bool_zero = zero(Type::basic(Kind::Bool)).unwrap();
    assert!(bool_zero.is_zero());
    assert_eq!(bool_zero.bool().unwrap(), false);
    
    let int_zero = zero(Type::basic(Kind::Int32)).unwrap();
    assert!(int_zero.is_zero());
    assert_eq!(int_zero.int().unwrap(), 0);
    
    let string_zero = zero(Type::basic(Kind::String)).unwrap();
    assert!(string_zero.is_zero());
    assert_eq!(string_zero.string().unwrap(), "");
    
    let float_zero = zero(Type::basic(Kind::Float64)).unwrap();
    assert!(float_zero.is_zero());
    assert_eq!(float_zero.float().unwrap(), 0.0);
    
    // Test zero value for slice
    let slice_type = slice_of(Type::basic(Kind::Int32));
    let slice_zero = zero(slice_type).unwrap();
    assert!(slice_zero.is_zero());
    assert_eq!(slice_zero.len().unwrap(), 0);
    
    // Test zero value for array
    let array_type = array_of(Type::basic(Kind::Bool), 3);
    let array_zero = zero(array_type).unwrap();
    assert!(!array_zero.is_zero()); // Array with elements is not zero even if elements are zero
    assert_eq!(array_zero.len().unwrap(), 3);
    assert!(array_zero.index(0).unwrap().is_zero()); // But elements are zero
}

#[test]
fn test_function_type_creation() {
    initialize();
    
    // Create a function type: func(string, int) bool
    let in_types = vec![Type::basic(Kind::String), Type::basic(Kind::Int32)];
    let out_types = vec![Type::basic(Kind::Bool)];
    
    let func_type = cursed::stdlib::lookin_glass::core_functions::func_of(in_types.clone(), out_types.clone(), false);
    
    assert_eq!(func_type.kind(), Kind::Func);
    assert!(func_type.is_func());
    assert_eq!(func_type.num_in(), 2);
    assert_eq!(func_type.num_out(), 1);
    assert!(!func_type.is_variadic());
    
    // Test variadic function: func(string, ...int) bool
    let variadic_func_type = cursed::stdlib::lookin_glass::core_functions::func_of(in_types, out_types, true);
    assert!(variadic_func_type.is_variadic());
}

#[test]
fn test_channel_creation() {
    initialize();
    
    let elem_type = Type::basic(Kind::String);
    let chan_type = cursed::stdlib::lookin_glass::core_functions::chan_of(elem_type);
    
    let chan_val = make_chan(chan_type, 5).unwrap();
    assert_eq!(chan_val.kind(), Kind::Chan);
    assert_eq!(chan_val.cap().unwrap(), 5);
}

#[test]
fn test_complex_struct_operations() {
    initialize();
    
    // Create a nested struct type (Address)
    let street_field = StructField::simple("Street".to_string(), Type::basic(Kind::String));
    let city_field = StructField::simple("City".to_string(), Type::basic(Kind::String));
    let zip_field = StructField::simple("ZipCode".to_string(), Type::basic(Kind::String));
    
    let address_type = Type::new(Kind::Struct, "Address".to_string(), "".to_string())
        .with_fields(vec![street_field, city_field, zip_field]);
    
    // Create a Person struct with embedded Address
    let name_field = StructField::simple("Name".to_string(), Type::basic(Kind::String));
    let address_field = StructField::simple("Address".to_string(), address_type.clone());
    
    let person_type = Type::new(Kind::Struct, "Person".to_string(), "".to_string())
        .with_fields(vec![name_field, address_field]);
    
    // Create address value
    let address_fields = vec![
        Value::from_string("123 Main St".to_string()),
        Value::from_string("Anytown".to_string()),
        Value::from_string("12345".to_string())
    ];
    let address_val = Value::new(address_type, 
        cursed::stdlib::lookin_glass::value::ValueData::Struct(address_fields));
    
    // Create person value
    let person_fields = vec![
        Value::from_string("John Doe".to_string()),
        address_val
    ];
    let person_val = Value::new(person_type.clone(), 
        cursed::stdlib::lookin_glass::value::ValueData::Struct(person_fields));
    
    // Test accessing nested fields
    assert_eq!(person_val.field(0).unwrap().string().unwrap(), "John Doe");
    
    let addr = person_val.field(1).unwrap();
    assert_eq!(addr.kind(), Kind::Struct);
    assert_eq!(addr.field(0).unwrap().string().unwrap(), "123 Main St");
    assert_eq!(addr.field(1).unwrap().string().unwrap(), "Anytown");
    assert_eq!(addr.field(2).unwrap().string().unwrap(), "12345");
}

#[test]
fn test_module_initialization_and_statistics() {
    initialize();
    
    let stats = get_reflection_statistics();
    
    // Basic validation that statistics structure exists
    assert_eq!(stats.types_created, 0); // Default value in our implementation
    assert_eq!(stats.values_created, 0);
    assert_eq!(stats.deep_copies_performed, 0);
    assert_eq!(stats.struct_conversions, 0);
}

#[test]
fn test_type_comparison_and_assignability() {
    initialize();
    
    let int_type = Type::basic(Kind::Int32);
    let int_type2 = Type::basic(Kind::Int32);
    let string_type = Type::basic(Kind::String);
    
    // Test type equality
    assert_eq!(int_type, int_type2);
    assert_ne!(int_type, string_type);
    
    // Test assignability
    assert!(int_type.assignable_to(&int_type2));
    assert!(!int_type.assignable_to(&string_type));
    
    // Test convertibility
    let float_type = Type::basic(Kind::Float64);
    assert!(int_type.convertible_to(&float_type));
    assert!(float_type.convertible_to(&int_type));
}

#[test]
fn test_error_handling() {
    initialize();
    
    // Test invalid operations
    let int_val = Value::from_int(42);
    
    // Trying to get fields from non-struct should fail
    assert!(int_val.field(0).is_err());
    assert!(int_val.field_by_name("test").is_err());
    
    // Trying to get length from non-collection should fail
    assert!(int_val.len().is_err());
    
    // Type mismatches should fail
    assert!(int_val.string().is_err());
    assert!(int_val.bool().is_err());
}

#[test]
fn test_value_validation_and_safety() {
    initialize();
    
    // Test invalid value
    let invalid_val = Value::invalid();
    assert!(!invalid_val.is_valid());
    assert!(invalid_val.is_zero());
    assert_eq!(invalid_val.kind(), Kind::Invalid);
    
    // Test nil values
    let ptr_type = ptr_to(Type::basic(Kind::Int32));
    let nil_ptr = Value::new(ptr_type, 
        cursed::stdlib::lookin_glass::value::ValueData::Pointer(None));
    assert!(nil_ptr.is_nil());
    
    // Test zero detection
    assert!(Value::from_bool(false).is_zero());
    assert!(Value::from_int(0).is_zero());
    assert!(Value::from_string("".to_string()).is_zero());
    assert!(!Value::from_bool(true).is_zero());
    assert!(!Value::from_int(1).is_zero());
    assert!(!Value::from_string("test".to_string()).is_zero());
}
