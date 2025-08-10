fr fr typez module - Type reflection, type checking, type conversions
fr fr Essential type system features for CURSED runtime and self-hosting

fr fr ===== TYPE INFORMATION STRUCTURES =====

squad TypeInfo {
    spill type_id normie
    spill name tea
    spill size normie
    spill alignment normie
    spill kind TypeKind
    spill is_primitive lit
    spill is_pointer lit
    spill is_array lit
    spill is_struct lit
    spill is_interface lit
    spill is_function lit
    spill methods []MethodInfo
}

squad FieldInfo {
    spill name tea
    spill type_info TypeInfo
    spill offset normie
    spill tag tea
}

squad MethodInfo {
    spill name tea
    spill receiver_type TypeInfo
    spill param_types []TypeInfo
    spill return_type TypeInfo
    spill is_public lit
}

squad InterfaceInfo {
    spill name tea
    spill methods []MethodInfo
    spill implementers []TypeInfo
}

squad StructInfo {
    spill name tea
    spill fields []FieldInfo
    spill methods []MethodInfo
    spill size normie
    spill alignment normie
}

fr fr Type kinds enumeration
sus TYPE_KIND_UNKNOWN normie = 0
sus TYPE_KIND_PRIMITIVE normie = 1
sus TYPE_KIND_POINTER normie = 2
sus TYPE_KIND_ARRAY normie = 3
sus TYPE_KIND_STRUCT normie = 4
sus TYPE_KIND_INTERFACE normie = 5
sus TYPE_KIND_FUNCTION normie = 6
sus TYPE_KIND_GENERIC normie = 7

squad TypeKind {
    spill id normie
    spill name tea
}

fr fr Global type registry
sus registered_types []TypeInfo = []
sus struct_infos []StructInfo = []
sus interface_infos []InterfaceInfo = []
sus type_id_counter normie = 1000

fr fr ===== PRIMITIVE TYPE DEFINITIONS =====

slay init_primitive_types() lit {
    fr fr Initialize built-in primitive types
    register_type(TypeInfo{
        type_id: 1, name: "tea", size: 8, alignment: 8,
        kind: TypeKind{id: TYPE_KIND_PRIMITIVE, name: "primitive"},
        is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap,
        methods: []
    })
    
    register_type(TypeInfo{
        type_id: 2, name: "normie", size: 4, alignment: 4,
        kind: TypeKind{id: TYPE_KIND_PRIMITIVE, name: "primitive"},
        is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap,
        methods: []
    })
    
    register_type(TypeInfo{
        type_id: 3, name: "thicc", size: 8, alignment: 8,
        kind: TypeKind{id: TYPE_KIND_PRIMITIVE, name: "primitive"},
        is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap,
        methods: []
    })
    
    register_type(TypeInfo{
        type_id: 4, name: "meal", size: 8, alignment: 8,
        kind: TypeKind{id: TYPE_KIND_PRIMITIVE, name: "primitive"},
        is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap,
        methods: []
    })
    
    register_type(TypeInfo{
        type_id: 5, name: "lit", size: 1, alignment: 1,
        kind: TypeKind{id: TYPE_KIND_PRIMITIVE, name: "primitive"},
        is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap,
        methods: []
    })
    
    register_type(TypeInfo{
        type_id: 6, name: "smol", size: 1, alignment: 1,
        kind: TypeKind{id: TYPE_KIND_PRIMITIVE, name: "primitive"},
        is_primitive: based, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap,
        methods: []
    })
    
    damn based
}

fr fr ===== TYPE REGISTRATION FUNCTIONS =====

slay register_type(type_info TypeInfo) normie {
    fr fr Register a new type in the global registry
    registered_types.push(type_info)
    damn type_info.type_id
}

slay register_struct_type(name tea, fields []FieldInfo, methods []MethodInfo) normie {
    fr fr Register a struct type
    sus total_size normie = calculate_struct_size(fields)
    sus alignment normie = calculate_struct_alignment(fields)
    
    sus struct_info StructInfo = StructInfo{
        name: name,
        fields: fields,
        methods: methods,
        size: total_size,
        alignment: alignment
    }
    
    struct_infos.push(struct_info)
    
    sus type_info TypeInfo = TypeInfo{
        type_id: type_id_counter,
        name: name,
        size: total_size,
        alignment: alignment,
        kind: TypeKind{id: TYPE_KIND_STRUCT, name: "struct"},
        is_primitive: cap,
        is_pointer: cap,
        is_array: cap,
        is_struct: based,
        is_interface: cap,
        is_function: cap,
        methods: methods
    }
    
    type_id_counter = type_id_counter + 1
    damn register_type(type_info)
}

slay register_interface_type(name tea, methods []MethodInfo) normie {
    fr fr Register an interface type
    sus interface_info InterfaceInfo = InterfaceInfo{
        name: name,
        methods: methods,
        implementers: []
    }
    
    interface_infos.push(interface_info)
    
    sus type_info TypeInfo = TypeInfo{
        type_id: type_id_counter,
        name: name,
        size: 16,  fr fr Interface fat pointer size
        alignment: 8,
        kind: TypeKind{id: TYPE_KIND_INTERFACE, name: "interface"},
        is_primitive: cap,
        is_pointer: cap,
        is_array: cap,
        is_struct: cap,
        is_interface: based,
        is_function: cap,
        methods: methods
    }
    
    type_id_counter = type_id_counter + 1
    damn register_type(type_info)
}

slay register_array_type(element_type TypeInfo, length normie) normie {
    fr fr Register an array type
    sus total_size normie = element_type.size * length
    sus array_name tea = "[]" + element_type.name
    
    sus type_info TypeInfo = TypeInfo{
        type_id: type_id_counter,
        name: array_name,
        size: total_size,
        alignment: element_type.alignment,
        kind: TypeKind{id: TYPE_KIND_ARRAY, name: "array"},
        is_primitive: cap,
        is_pointer: cap,
        is_array: based,
        is_struct: cap,
        is_interface: cap,
        is_function: cap,
        methods: []
    }
    
    type_id_counter = type_id_counter + 1
    damn register_type(type_info)
}

fr fr ===== TYPE LOOKUP FUNCTIONS =====

slay get_type_by_id(type_id normie) TypeInfo {
    fr fr Get type info by ID
    bestie type_info in registered_types {
        lowkey type_info.type_id == type_id {
            damn type_info
        }
    }
    
    fr fr Return unknown type if not found
    damn TypeInfo{
        type_id: 0,
        name: "unknown",
        size: 0,
        alignment: 1,
        kind: TypeKind{id: TYPE_KIND_UNKNOWN, name: "unknown"},
        is_primitive: cap, is_pointer: cap, is_array: cap, is_struct: cap, is_interface: cap, is_function: cap
    }
}

slay get_type_by_name(name tea) TypeInfo {
    fr fr Get type info by name
    bestie type_info in registered_types {
        lowkey type_info.name == name {
            damn type_info
        }
    }
    
    damn get_type_by_id(0)  fr fr Return unknown type
}

slay get_struct_info(name tea) StructInfo {
    fr fr Get detailed struct information
    bestie struct_info in struct_infos {
        lowkey struct_info.name == name {
            damn struct_info
        }
    }
    
    damn StructInfo{name: "unknown", fields: [], methods: [], size: 0, alignment: 1}
}

slay get_interface_info(name tea) InterfaceInfo {
    fr fr Get detailed interface information
    bestie interface_info in interface_infos {
        lowkey interface_info.name == name {
            damn interface_info
        }
    }
    
    damn InterfaceInfo{name: "unknown", methods: [], implementers: []}
}

slay get_all_types() []TypeInfo {
    fr fr Get all registered types
    damn registered_types
}

slay get_types_by_kind(kind_id normie) []TypeInfo {
    fr fr Get all types of a specific kind
    sus result []TypeInfo = []
    
    bestie type_info in registered_types {
        lowkey type_info.kind.id == kind_id {
            result.push(type_info)
        }
    }
    
    damn result
}

fr fr ===== TYPE CHECKING FUNCTIONS =====

slay is_assignable(from_type TypeInfo, to_type TypeInfo) lit {
    fr fr Check if from_type can be assigned to to_type
    lowkey from_type.type_id == to_type.type_id {
        damn based  fr fr Same type
    }
    
    fr fr Check interface implementation
    lowkey to_type.is_interface {
        damn implements_interface(from_type, to_type)
    }
    
    fr fr Check numeric conversions
    lowkey from_type.is_primitive && to_type.is_primitive {
        damn is_numeric_convertible(from_type, to_type)
    }
    
    fr fr Check pointer compatibility
    lowkey from_type.is_pointer && to_type.is_pointer {
        damn based  fr fr Allow pointer assignments for now
    }
    
    damn cap
}

slay is_numeric_convertible(from_type TypeInfo, to_type TypeInfo) lit {
    fr fr Check if numeric types can be converted
    sus numeric_types []tea = ["normie", "thicc", "meal", "smol"]
    
    sus from_numeric lit = cap
    sus to_numeric lit = cap
    
    bestie numeric_type in numeric_types {
        lowkey from_type.name == numeric_type {
            from_numeric = based
        }
        lowkey to_type.name == numeric_type {
            to_numeric = based
        }
    }
    
    damn from_numeric && to_numeric
}

slay implements_interface(impl_type TypeInfo, interface_type TypeInfo) lit {
    fr fr Check if type implements interface
    lowkey !interface_type.is_interface {
        damn cap
    }
    
    sus interface_info InterfaceInfo = get_interface_info(interface_type.name)
    lowkey interface_info.name == "unknown" {
        damn cap
    }
    
    fr fr Check if type has all required methods
    bestie required_method in interface_info.methods {
        lowkey !type_has_method(impl_type, required_method) {
            damn cap
        }
    }
    
    damn based
}

slay type_has_method(type_info TypeInfo, method MethodInfo) lit {
    fr fr Check if type has a specific method
    lowkey type_info.is_struct {
        sus struct_info StructInfo = get_struct_info(type_info.name)
        bestie struct_method in struct_info.methods {
            lowkey struct_method.name == method.name {
                damn method_signatures_match(struct_method, method)
            }
        }
    }
    
    damn cap
}

slay method_signatures_match(method1 MethodInfo, method2 MethodInfo) lit {
    fr fr Check if two method signatures match
    lowkey method1.name != method2.name {
        damn cap
    }
    
    lowkey method1.param_types.len() != method2.param_types.len() {
        damn cap
    }
    
    lowkey method1.return_type.type_id != method2.return_type.type_id {
        damn cap
    }
    
    bestie i := 0; i < method1.param_types.len(); i++ {
        lowkey method1.param_types[i].type_id != method2.param_types[i].type_id {
            damn cap
        }
    }
    
    damn based
}

fr fr ===== TYPE CONVERSION FUNCTIONS =====

slay convert_value(value normie, from_type TypeInfo, to_type TypeInfo) normie {
    fr fr Convert value between types
    lowkey from_type.type_id == to_type.type_id {
        damn value  fr fr No conversion needed
    }
    
    fr fr Handle numeric conversions
    lowkey from_type.is_primitive && to_type.is_primitive {
        damn convert_numeric(value, from_type, to_type)
    }
    
    fr fr Handle pointer conversions
    lowkey from_type.is_pointer && to_type.is_pointer {
        damn value  fr fr Direct pointer assignment
    }
    
    damn 0  fr fr Conversion not supported
}

slay convert_numeric(value normie, from_type TypeInfo, to_type TypeInfo) normie {
    fr fr Convert between numeric types
    fr fr This is a simplified implementation
    lowkey from_type.name == "normie" && to_type.name == "thicc" {
        damn value  fr fr normie to thicc
    }
    
    lowkey from_type.name == "thicc" && to_type.name == "normie" {
        damn value  fr fr thicc to normie (potential data loss)
    }
    
    lowkey from_type.name == "smol" && to_type.name == "normie" {
        damn value  fr fr smol to normie
    }
    
    damn value  fr fr Default: no conversion
}

slay safe_convert(value normie, from_type TypeInfo, to_type TypeInfo) (normie, lit) {
    fr fr Safe conversion with success flag
    lowkey !is_assignable(from_type, to_type) {
        damn (0, cap)  fr fr Conversion not allowed
    }
    
    sus converted normie = convert_value(value, from_type, to_type)
    damn (converted, based)
}

fr fr ===== REFLECTION FUNCTIONS =====

slay get_field_value(object_ptr normie, object_type TypeInfo, field_name tea) normie {
    fr fr Get field value from object
    lowkey !object_type.is_struct {
        damn 0
    }
    
    sus struct_info StructInfo = get_struct_info(object_type.name)
    bestie field in struct_info.fields {
        lowkey field.name == field_name {
            damn read_memory_at_offset(object_ptr, field.offset, field.type_info.size)
        }
    }
    
    damn 0  fr fr Field not found
}

slay set_field_value(object_ptr normie, object_type TypeInfo, field_name tea, value normie) lit {
    fr fr Set field value in object
    lowkey !object_type.is_struct {
        damn cap
    }
    
    sus struct_info StructInfo = get_struct_info(object_type.name)
    bestie field in struct_info.fields {
        lowkey field.name == field_name {
            write_memory_at_offset(object_ptr, field.offset, value, field.type_info.size)
            damn based
        }
    }
    
    damn cap  fr fr Field not found
}

slay get_field_info(type_info TypeInfo, field_name tea) FieldInfo {
    fr fr Get information about a field
    lowkey type_info.is_struct {
        sus struct_info StructInfo = get_struct_info(type_info.name)
        bestie field in struct_info.fields {
            lowkey field.name == field_name {
                damn field
            }
        }
    }
    
    damn FieldInfo{name: "unknown", type_info: get_type_by_id(0), offset: 0, tag: ""}
}

slay call_method(object_ptr normie, object_type TypeInfo, method_name tea, args []normie) normie {
    fr fr Call method on object (simplified)
    lowkey !object_type.is_struct {
        damn 0
    }
    
    sus struct_info StructInfo = get_struct_info(object_type.name)
    bestie method in struct_info.methods {
        lowkey method.name == method_name {
            fr fr In a real implementation, this would use dynamic dispatch
            damn invoke_method(object_ptr, method, args)
        }
    }
    
    damn 0  fr fr Method not found
}

fr fr ===== UTILITY FUNCTIONS =====

slay calculate_struct_size(fields []FieldInfo) normie {
    fr fr Calculate total size of struct with padding
    sus total_size normie = 0
    sus max_alignment normie = 1
    
    bestie field in fields {
        fr fr Align field offset
        sus field_offset normie = align_to_boundary(total_size, field.type_info.alignment)
        field.offset = field_offset
        total_size = field_offset + field.type_info.size
        
        lowkey field.type_info.alignment > max_alignment {
            max_alignment = field.type_info.alignment
        }
    }
    
    fr fr Align total size to largest field alignment
    damn align_to_boundary(total_size, max_alignment)
}

slay calculate_struct_alignment(fields []FieldInfo) normie {
    fr fr Calculate required alignment for struct
    sus max_alignment normie = 1
    
    bestie field in fields {
        lowkey field.type_info.alignment > max_alignment {
            max_alignment = field.type_info.alignment
        }
    }
    
    damn max_alignment
}

slay align_to_boundary(value normie, alignment normie) normie {
    fr fr Align value to alignment boundary
    lowkey alignment <= 1 {
        damn value
    }
    damn ((value + alignment - 1) / alignment) * alignment
}

slay get_type_size(type_info TypeInfo) normie {
    damn type_info.size
}

slay get_type_alignment(type_info TypeInfo) normie {
    damn type_info.alignment
}

slay is_type_primitive(type_info TypeInfo) lit {
    damn type_info.is_primitive
}

slay is_type_pointer(type_info TypeInfo) lit {
    damn type_info.is_pointer
}

slay is_type_struct(type_info TypeInfo) lit {
    damn type_info.is_struct
}

slay is_type_interface(type_info TypeInfo) lit {
    damn type_info.is_interface
}

fr fr ===== TYPE SYSTEM INTROSPECTION =====

slay print_type_info(type_info TypeInfo) lit {
    vibez.spill("🔍 Type Information")
    vibez.spill("═══════════════════")
    vibez.spill("Name: ", type_info.name)
    vibez.spill("ID: ", type_info.type_id)
    vibez.spill("Size: ", type_info.size, " bytes")
    vibez.spill("Alignment: ", type_info.alignment, " bytes")
    vibez.spill("Kind: ", type_info.kind.name)
    vibez.spill("Primitive: ", format_bool(type_info.is_primitive))
    vibez.spill("Pointer: ", format_bool(type_info.is_pointer))
    vibez.spill("Array: ", format_bool(type_info.is_array))
    vibez.spill("Struct: ", format_bool(type_info.is_struct))
    vibez.spill("Interface: ", format_bool(type_info.is_interface))
    vibez.spill("Function: ", format_bool(type_info.is_function))
    damn based
}

slay print_struct_info(struct_info StructInfo) lit {
    vibez.spill("🏗️ Struct Information")
    vibez.spill("═══════════════════")
    vibez.spill("Name: ", struct_info.name)
    vibez.spill("Size: ", struct_info.size, " bytes")
    vibez.spill("Alignment: ", struct_info.alignment, " bytes")
    vibez.spill("Fields: ", struct_info.fields.len())
    
    bestie field in struct_info.fields {
        vibez.spill("  - ", field.name, ": ", field.type_info.name, " (offset: ", field.offset, ")")
    }
    
    vibez.spill("Methods: ", struct_info.methods.len())
    bestie method in struct_info.methods {
        vibez.spill("  - ", method.name, "()")
    }
    
    damn based
}

slay print_all_types() lit {
    vibez.spill("📋 Registered Types")
    vibez.spill("══════════════════")
    
    bestie type_info in registered_types {
        vibez.spill(type_info.type_id, ": ", type_info.name, " (", type_info.size, " bytes)")
    }
    
    damn based
}

slay format_bool(value lit) tea {
    lowkey value {
        damn "true"
    }
    damn "false"
}

fr fr ===== MEMORY ACCESS HELPERS =====

slay read_memory_at_offset(base_ptr normie, offset normie, size normie) normie {
    fr fr Read memory at offset (simplified)
    sus ptr normie = base_ptr + offset
    damn core.read_memory(ptr, size)
}

slay write_memory_at_offset(base_ptr normie, offset normie, value normie, size normie) lit {
    fr fr Write memory at offset (simplified)
    sus ptr normie = base_ptr + offset
    core.write_memory(ptr, value, size)
    damn based
}

slay invoke_method(object_ptr normie, method MethodInfo, args []normie) normie {
    fr fr Invoke method using dynamic dispatch (simplified)
    fr fr Real implementation would use vtables or function pointers
    damn core.dynamic_invoke(object_ptr, method.name, args)
}

fr fr Initialize type system
slay init_type_system() lit {
    init_primitive_types()
    damn based
}
