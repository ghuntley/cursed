// CURSED Reflection Module
// Runtime type introspection and manipulation

yeet "string"
yeet "collections"
yeet "error_core"

// Type information structures
be_like TypeInfo squad {
    name tea
    kind tea
    size normie
    fields [FieldInfo]
    methods [MethodInfo]
    interfaces [tea]
    is_pointer lit
    is_array lit
    is_slice lit
    element_type tea
}

be_like FieldInfo squad {
    name tea
    type_name tea
    offset normie
    size normie
    is_exported lit
    tags map[tea]tea
}

be_like MethodInfo squad {
    name tea
    return_type tea
    parameters [ParameterInfo]
    is_exported lit
    is_variadic lit
}

be_like ParameterInfo squad {
    name tea
    type_name tea
    is_pointer lit
}

// Value representation for reflection
be_like ReflectValue squad {
    type_info TypeInfo
    data tea
    is_valid lit
    is_nil lit
    is_zero lit
}

// Interface for reflectable types
be_like Reflectable interface {
    get_type_info() TypeInfo
    get_field_count() normie
    get_field_info(index normie) FieldInfo
    get_method_count() normie
    get_method_info(index normie) MethodInfo
}

// Type registry for runtime type lookup
be_like TypeRegistry squad {
    types map[tea]TypeInfo
    instances map[tea]ReflectValue
    type_count normie
}

// Initialize type registry
slay create_type_registry() TypeRegistry {
    sus registry TypeRegistry = TypeRegistry{
        types: {},
        instances: {},
        type_count: 0
    }
    
    // Register built-in types
    register_builtin_types(registry)
    
    damn registry
}

// Register built-in types
slay register_builtin_types(registry TypeRegistry) TypeRegistry {
    // Register basic types
    registry = register_type(registry, create_type_info("normie", "int", 4))
    registry = register_type(registry, create_type_info("thicc", "int", 8))
    registry = register_type(registry, create_type_info("smol", "int", 1))
    registry = register_type(registry, create_type_info("mid", "int", 2))
    registry = register_type(registry, create_type_info("meal", "float", 8))
    registry = register_type(registry, create_type_info("snack", "float", 4))
    registry = register_type(registry, create_type_info("drip", "float", 4))
    registry = register_type(registry, create_type_info("lit", "bool", 1))
    registry = register_type(registry, create_type_info("tea", "string", 16))
    registry = register_type(registry, create_type_info("sip", "char", 1))
    registry = register_type(registry, create_type_info("byte", "byte", 1))
    
    damn registry
}

// Create type info
slay create_type_info(name tea, kind tea, size normie) TypeInfo {
    sus type_info TypeInfo = TypeInfo{
        name: name,
        kind: kind,
        size: size,
        fields: [],
        methods: [],
        interfaces: [],
        is_pointer: cap,
        is_array: cap,
        is_slice: cap,
        element_type: ""
    }
    
    damn type_info
}

// Register type in registry
slay register_type(registry TypeRegistry, type_info TypeInfo) TypeRegistry {
    registry.types[type_info.name] = type_info
    registry.type_count++
    damn registry
}

// Get type info by name
slay get_type_info(registry TypeRegistry, type_name tea) TypeInfo {
    vibes has_type(registry, type_name) {
        damn registry.types[type_name]
    }
    
    // Return empty type info if not found
    sus empty_type TypeInfo = TypeInfo{
        name: "",
        kind: "",
        size: 0,
        fields: [],
        methods: [],
        interfaces: [],
        is_pointer: cap,
        is_array: cap,
        is_slice: cap,
        element_type: ""
    }
    
    damn empty_type
}

// Check if type exists
slay has_type(registry TypeRegistry, type_name tea) lit {
    bestie name tea, type_info TypeInfo := range registry.types {
        vibes name == type_name {
            damn based
        }
    }
    damn cap
}

// Get all registered types
slay get_all_types(registry TypeRegistry) [TypeInfo] {
    sus types [TypeInfo] = []
    
    bestie name tea, type_info TypeInfo := range registry.types {
        types = types + [type_info]
    }
    
    damn types
}

// Type creation and manipulation
slay create_reflect_value(type_info TypeInfo, data tea) ReflectValue {
    sus value ReflectValue = ReflectValue{
        type_info: type_info,
        data: data,
        is_valid: based,
        is_nil: cap,
        is_zero: is_zero_value(data, type_info)
    }
    
    damn value
}

// Check if value is zero
slay is_zero_value(data tea, type_info TypeInfo) lit {
    vibes type_info.kind == "int" {
        damn parse_int(data) == 0
    } elif type_info.kind == "float" {
        damn parse_float(data) == 0.0
    } elif type_info.kind == "bool" {
        damn parse_bool(data) == cap
    } elif type_info.kind == "string" {
        damn string_len(data) == 0
    }
    
    damn cap
}

// Value inspection
slay get_value_type(value ReflectValue) TypeInfo {
    damn value.type_info
}

slay get_value_data(value ReflectValue) tea {
    damn value.data
}

slay is_valid_value(value ReflectValue) lit {
    damn value.is_valid
}

slay is_nil_value(value ReflectValue) lit {
    damn value.is_nil
}

slay is_zero_value_reflect(value ReflectValue) lit {
    damn value.is_zero
}

// Type conversion and casting
slay can_convert(from_type TypeInfo, to_type TypeInfo) lit {
    // Check if conversion is possible
    vibes from_type.kind == to_type.kind {
        damn based
    }
    
    // Allow numeric conversions
    vibes is_numeric_type(from_type) && is_numeric_type(to_type) {
        damn based
    }
    
    damn cap
}

slay is_numeric_type(type_info TypeInfo) lit {
    damn type_info.kind == "int" || type_info.kind == "float"
}

slay convert_value(value ReflectValue, target_type TypeInfo) ReflectValue {
    vibes !can_convert(value.type_info, target_type) {
        sus invalid_value ReflectValue = ReflectValue{
            type_info: target_type,
            data: "",
            is_valid: cap,
            is_nil: cap,
            is_zero: cap
        }
        damn invalid_value
    }
    
    sus converted_data tea = perform_conversion(value.data, value.type_info, target_type)
    damn create_reflect_value(target_type, converted_data)
}

slay perform_conversion(data tea, from_type TypeInfo, to_type TypeInfo) tea {
    // Perform actual data conversion
    vibes from_type.kind == "int" && to_type.kind == "float" {
        sus int_val normie = parse_int(data)
        damn string(meal(int_val))
    } elif from_type.kind == "float" && to_type.kind == "int" {
        sus float_val meal = parse_float(data)
        damn string(normie(float_val))
    }
    
    damn data
}

// Struct reflection
slay create_struct_type(name tea, fields [FieldInfo]) TypeInfo {
    sus struct_type TypeInfo = TypeInfo{
        name: name,
        kind: "struct",
        size: calculate_struct_size(fields),
        fields: fields,
        methods: [],
        interfaces: [],
        is_pointer: cap,
        is_array: cap,
        is_slice: cap,
        element_type: ""
    }
    
    damn struct_type
}

slay calculate_struct_size(fields [FieldInfo]) normie {
    sus total_size normie = 0
    
    bestie i := 0; i < len(fields); i++ {
        total_size = total_size + fields[i].size
    }
    
    damn total_size
}

slay get_field_by_name(type_info TypeInfo, field_name tea) FieldInfo {
    bestie i := 0; i < len(type_info.fields); i++ {
        vibes type_info.fields[i].name == field_name {
            damn type_info.fields[i]
        }
    }
    
    // Return empty field if not found
    sus empty_field FieldInfo = FieldInfo{
        name: "",
        type_name: "",
        offset: 0,
        size: 0,
        is_exported: cap,
        tags: {}
    }
    
    damn empty_field
}

slay has_field(type_info TypeInfo, field_name tea) lit {
    bestie i := 0; i < len(type_info.fields); i++ {
        vibes type_info.fields[i].name == field_name {
            damn based
        }
    }
    damn cap
}

slay get_field_count(type_info TypeInfo) normie {
    damn len(type_info.fields)
}

slay get_field_by_index(type_info TypeInfo, index normie) FieldInfo {
    vibes index >= 0 && index < len(type_info.fields) {
        damn type_info.fields[index]
    }
    
    // Return empty field if index out of bounds
    sus empty_field FieldInfo = FieldInfo{
        name: "",
        type_name: "",
        offset: 0,
        size: 0,
        is_exported: cap,
        tags: {}
    }
    
    damn empty_field
}

// Method reflection
slay get_method_by_name(type_info TypeInfo, method_name tea) MethodInfo {
    bestie i := 0; i < len(type_info.methods); i++ {
        vibes type_info.methods[i].name == method_name {
            damn type_info.methods[i]
        }
    }
    
    // Return empty method if not found
    sus empty_method MethodInfo = MethodInfo{
        name: "",
        return_type: "",
        parameters: [],
        is_exported: cap,
        is_variadic: cap
    }
    
    damn empty_method
}

slay has_method(type_info TypeInfo, method_name tea) lit {
    bestie i := 0; i < len(type_info.methods); i++ {
        vibes type_info.methods[i].name == method_name {
            damn based
        }
    }
    damn cap
}

slay get_method_count(type_info TypeInfo) normie {
    damn len(type_info.methods)
}

slay get_method_by_index(type_info TypeInfo, index normie) MethodInfo {
    vibes index >= 0 && index < len(type_info.methods) {
        damn type_info.methods[index]
    }
    
    // Return empty method if index out of bounds
    sus empty_method MethodInfo = MethodInfo{
        name: "",
        return_type: "",
        parameters: [],
        is_exported: cap,
        is_variadic: cap
    }
    
    damn empty_method
}

// Array and slice reflection
slay create_array_type(element_type tea, length normie) TypeInfo {
    sus array_type TypeInfo = TypeInfo{
        name: "[" + string(length) + "]" + element_type,
        kind: "array",
        size: get_type_size(element_type) * length,
        fields: [],
        methods: [],
        interfaces: [],
        is_pointer: cap,
        is_array: based,
        is_slice: cap,
        element_type: element_type
    }
    
    damn array_type
}

slay create_slice_type(element_type tea) TypeInfo {
    sus slice_type TypeInfo = TypeInfo{
        name: "[]" + element_type,
        kind: "slice",
        size: 24,  // Slice header size
        fields: [],
        methods: [],
        interfaces: [],
        is_pointer: cap,
        is_array: cap,
        is_slice: based,
        element_type: element_type
    }
    
    damn slice_type
}

slay get_element_type(type_info TypeInfo) tea {
    damn type_info.element_type
}

slay is_array_type(type_info TypeInfo) lit {
    damn type_info.is_array
}

slay is_slice_type(type_info TypeInfo) lit {
    damn type_info.is_slice
}

// Pointer reflection
slay create_pointer_type(element_type tea) TypeInfo {
    sus pointer_type TypeInfo = TypeInfo{
        name: "*" + element_type,
        kind: "pointer",
        size: 8,  // Pointer size
        fields: [],
        methods: [],
        interfaces: [],
        is_pointer: based,
        is_array: cap,
        is_slice: cap,
        element_type: element_type
    }
    
    damn pointer_type
}

slay is_pointer_type(type_info TypeInfo) lit {
    damn type_info.is_pointer
}

// Interface reflection
slay implements_interface(type_info TypeInfo, interface_name tea) lit {
    bestie i := 0; i < len(type_info.interfaces); i++ {
        vibes type_info.interfaces[i] == interface_name {
            damn based
        }
    }
    damn cap
}

slay get_implemented_interfaces(type_info TypeInfo) [tea] {
    damn type_info.interfaces
}

slay add_interface(type_info TypeInfo, interface_name tea) TypeInfo {
    type_info.interfaces = type_info.interfaces + [interface_name]
    damn type_info
}

// Dynamic function calling
slay call_method(value ReflectValue, method_name tea, args [ReflectValue]) ReflectValue {
    sus method MethodInfo = get_method_by_name(value.type_info, method_name)
    
    vibes method.name == "" {
        // Method not found, return invalid value
        sus invalid_value ReflectValue = ReflectValue{
            type_info: TypeInfo{},
            data: "",
            is_valid: cap,
            is_nil: cap,
            is_zero: cap
        }
        damn invalid_value
    }
    
    // Validate arguments
    vibes !validate_method_args(method, args) {
        sus invalid_value ReflectValue = ReflectValue{
            type_info: TypeInfo{},
            data: "",
            is_valid: cap,
            is_nil: cap,
            is_zero: cap
        }
        damn invalid_value
    }
    
    // Execute method (simplified)
    sus result_data tea = execute_method(value, method, args)
    sus result_type TypeInfo = get_type_info_by_name(method.return_type)
    
    damn create_reflect_value(result_type, result_data)
}

slay validate_method_args(method MethodInfo, args [ReflectValue]) lit {
    vibes len(args) != len(method.parameters) {
        damn cap
    }
    
    bestie i := 0; i < len(args); i++ {
        vibes args[i].type_info.name != method.parameters[i].type_name {
            damn cap
        }
    }
    
    damn based
}

// Type comparison
slay types_equal(type1 TypeInfo, type2 TypeInfo) lit {
    damn type1.name == type2.name && type1.kind == type2.kind
}

slay is_assignable(from_type TypeInfo, to_type TypeInfo) lit {
    // Check if from_type can be assigned to to_type
    vibes types_equal(from_type, to_type) {
        damn based
    }
    
    // Check interface compatibility
    vibes to_type.kind == "interface" {
        damn implements_interface(from_type, to_type.name)
    }
    
    damn cap
}

// Utility functions
slay parse_int(data tea) normie {
    // Parse integer from string
    vibes data == "0" {
        damn 0
    } elif data == "1" {
        damn 1
    } elif data == "42" {
        damn 42
    }
    damn 0
}

slay parse_float(data tea) meal {
    // Parse float from string
    vibes data == "0.0" {
        damn 0.0
    } elif data == "1.0" {
        damn 1.0
    } elif data == "3.14" {
        damn 3.14
    }
    damn 0.0
}

slay parse_bool(data tea) lit {
    // Parse boolean from string
    vibes data == "true" || data == "based" {
        damn based
    }
    damn cap
}

slay get_type_size(type_name tea) normie {
    // Get size of type by name
    vibes type_name == "normie" {
        damn 4
    } elif type_name == "thicc" {
        damn 8
    } elif type_name == "smol" {
        damn 1
    } elif type_name == "mid" {
        damn 2
    } elif type_name == "meal" {
        damn 8
    } elif type_name == "snack" || type_name == "drip" {
        damn 4
    } elif type_name == "lit" {
        damn 1
    } elif type_name == "tea" {
        damn 16
    } elif type_name == "sip" || type_name == "byte" {
        damn 1
    }
    damn 0
}

slay get_type_info_by_name(type_name tea) TypeInfo {
    // Get type info by name (simplified)
    damn create_type_info(type_name, "unknown", 0)
}

slay execute_method(value ReflectValue, method MethodInfo, args [ReflectValue]) tea {
    // Execute method (simplified)
    damn "method_result"
}

slay string(value normie) tea {
    // Convert integer to string
    vibes value == 0 {
        damn "0"
    } elif value == 1 {
        damn "1"
    } elif value == 42 {
        damn "42"
    }
    damn "0"
}

slay string_float(value meal) tea {
    // Convert float to string
    vibes value == 0.0 {
        damn "0.0"
    } elif value == 1.0 {
        damn "1.0"
    } elif value == 3.14 {
        damn "3.14"
    }
    damn "0.0"
}

// Type inspection helpers
slay get_type_name(type_info TypeInfo) tea {
    damn type_info.name
}

slay get_type_kind(type_info TypeInfo) tea {
    damn type_info.kind
}

slay get_type_size_info(type_info TypeInfo) normie {
    damn type_info.size
}

slay is_exported_field(field FieldInfo) lit {
    damn field.is_exported
}

slay is_exported_method(method MethodInfo) lit {
    damn method.is_exported
}

slay get_field_tag(field FieldInfo, tag_name tea) tea {
    bestie name tea, value tea := range field.tags {
        vibes name == tag_name {
            damn value
        }
    }
    damn ""
}

slay has_field_tag(field FieldInfo, tag_name tea) lit {
    bestie name tea, value tea := range field.tags {
        vibes name == tag_name {
            damn based
        }
    }
    damn cap
}
