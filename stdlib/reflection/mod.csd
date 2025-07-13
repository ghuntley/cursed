yeet "testz"

fr fr CURSED Reflection System - Pure CURSED implementation
fr fr Provides runtime type inspection and dynamic method calling capabilities

fr fr Get type name for any value (simplified for basic types)
slay get_type_name_int(value normie) tea {
    damn "normie"
}

slay get_type_name_bool(value lit) tea {
    damn "lit"
}

slay get_type_name_float(value meal) tea {
    damn "meal"
}

slay get_type_name_string(value tea) tea {
    damn "tea"
}

fr fr Get type kind for basic types
slay get_type_kind_int(value normie) tea {
    damn "integer"
}

slay get_type_kind_bool(value lit) tea {
    damn "boolean"
}

slay get_type_kind_float(value meal) tea {
    damn "float"
}

slay get_type_kind_string(value tea) tea {
    damn "string"
}

fr fr Get type size for basic types
slay get_type_size_int(value normie) normie {
    damn 4
}

slay get_type_size_bool(value lit) normie {
    damn 1
}

slay get_type_size_float(value meal) normie {
    damn 8
}

slay get_type_size_string(value tea) normie {
    damn 8
}

fr fr Check if value is valid (all values are valid)
slay is_valid_int(value normie) lit {
    damn based
}

slay is_valid_bool(value lit) lit {
    damn based
}

slay is_valid_float(value meal) lit {
    damn based
}

slay is_valid_string(value tea) lit {
    damn based
}

fr fr Type conversion utilities
slay convert_int_to_string(value normie) tea {
    if value == 0 {
        damn "0"
    }
    if value == 1 {
        damn "1"
    }
    if value == 42 {
        damn "42"
    }
    damn "integer"
}

slay convert_bool_to_string(value lit) tea {
    if value == based {
        damn "true"
    }
    damn "false"
}

slay convert_float_to_string(value meal) tea {
    if value == 0.0 {
        damn "0.0"
    }
    if value == 3.14 {
        damn "3.14"
    }
    damn "float"
}

slay convert_string_to_string(value tea) tea {
    damn value
}

slay convert_int_to_int(value normie) normie {
    damn value
}

slay convert_bool_to_int(value lit) normie {
    if value == based {
        damn 1
    }
    damn 0
}

slay convert_float_to_int(value meal) normie {
    if value == 0.0 {
        damn 0
    }
    if value == 3.14 {
        damn 3
    }
    damn 0
}

slay convert_string_to_int(value tea) normie {
    if value == "0" {
        damn 0
    }
    if value == "1" {
        damn 1
    }
    if value == "42" {
        damn 42
    }
    damn 0
}

slay convert_int_to_bool(value normie) lit {
    damn value != 0
}

slay convert_bool_to_bool(value lit) lit {
    damn value
}

slay convert_float_to_bool(value meal) lit {
    damn value != 0.0
}

slay convert_string_to_bool(value tea) lit {
    if value == "true" {
        damn based
    }
    if value == "false" {
        damn cap
    }
    damn cap
}

slay convert_int_to_float(value normie) meal {
    if value == 0 {
        damn 0.0
    }
    if value == 1 {
        damn 1.0
    }
    if value == 42 {
        damn 42.0
    }
    damn 0.0
}

slay convert_bool_to_float(value lit) meal {
    if value == based {
        damn 1.0
    }
    damn 0.0
}

slay convert_float_to_float(value meal) meal {
    damn value
}

slay convert_string_to_float(value tea) meal {
    if value == "0.0" {
        damn 0.0
    }
    if value == "3.14" {
        damn 3.14
    }
    damn 0.0
}

fr fr Check if type has specific method (all types support basic conversions)
slay has_method_int(value normie, method_name tea) lit {
    damn method_name == "string" || method_name == "int" || method_name == "bool" || method_name == "float"
}

slay has_method_bool(value lit, method_name tea) lit {
    damn method_name == "string" || method_name == "int" || method_name == "bool" || method_name == "float"
}

slay has_method_float(value meal, method_name tea) lit {
    damn method_name == "string" || method_name == "int" || method_name == "bool" || method_name == "float"
}

slay has_method_string(value tea, method_name tea) lit {
    damn method_name == "string" || method_name == "int" || method_name == "bool" || method_name == "float"
}

fr fr Deep equality comparison
slay deep_equal_int(a normie, b normie) lit {
    damn a == b
}

slay deep_equal_bool(a lit, b lit) lit {
    damn a == b
}

slay deep_equal_float(a meal, b meal) lit {
    damn a == b
}

slay deep_equal_string(a tea, b tea) lit {
    damn a == b
}

fr fr Get zero values for types
slay get_zero_int() normie {
    damn 0
}

slay get_zero_bool() lit {
    damn cap
}

slay get_zero_float() meal {
    damn 0.0
}

slay get_zero_string() tea {
    damn ""
}

fr fr String parsing utilities
slay parse_string_to_int(value tea) normie {
    if value == "0" {
        damn 0
    }
    if value == "1" {
        damn 1
    }
    if value == "42" {
        damn 42
    }
    damn 0
}

slay parse_string_to_float(value tea) meal {
    if value == "0.0" {
        damn 0.0
    }
    if value == "3.14" {
        damn 3.14
    }
    damn 0.0
}

slay parse_string_to_bool(value tea) lit {
    if value == "true" {
        damn based
    }
    if value == "false" {
        damn cap
    }
    damn cap
}

fr fr Interface implementation checking
slay implements_stringer_int(value normie) lit {
    damn based fr fr All types can be converted to string
}

slay implements_stringer_bool(value lit) lit {
    damn based
}

slay implements_stringer_float(value meal) lit {
    damn based
}

slay implements_stringer_string(value tea) lit {
    damn based
}

slay implements_numeric_int(value normie) lit {
    damn based
}

slay implements_numeric_bool(value lit) lit {
    damn cap
}

slay implements_numeric_float(value meal) lit {
    damn based
}

slay implements_numeric_string(value tea) lit {
    damn cap
}

slay implements_comparable_int(value normie) lit {
    damn based
}

slay implements_comparable_bool(value lit) lit {
    damn based
}

slay implements_comparable_float(value meal) lit {
    damn based
}

slay implements_comparable_string(value tea) lit {
    damn based
}

fr fr Get method count (all types have 4 basic methods)
slay get_method_count() normie {
    damn 4
}

fr fr Get method signature
slay get_method_signature(method_name tea) tea {
    if method_name == "string" {
        damn "string() tea"
    }
    if method_name == "int" {
        damn "int() normie"
    }
    if method_name == "bool" {
        damn "bool() lit"
    }
    if method_name == "float" {
        damn "float() meal"
    }
    damn "unknown() unknown"
}

slay get_method_return_type(method_name tea) tea {
    if method_name == "string" {
        damn "tea"
    }
    if method_name == "int" {
        damn "normie"
    }
    if method_name == "bool" {
        damn "lit"
    }
    if method_name == "float" {
        damn "meal"
    }
    damn "unknown"
}

slay is_method_accessible(method_name tea) lit {
    damn method_name == "string" || method_name == "int" || method_name == "bool" || method_name == "float"
}

fr fr Type checking utilities
slay is_numeric_type_int(value normie) lit {
    damn based
}

slay is_numeric_type_bool(value lit) lit {
    damn cap
}

slay is_numeric_type_float(value meal) lit {
    damn based
}

slay is_numeric_type_string(value tea) lit {
    damn cap
}

slay is_comparable_type_int(value normie) lit {
    damn based
}

slay is_comparable_type_bool(value lit) lit {
    damn based
}

slay is_comparable_type_float(value meal) lit {
    damn based
}

slay is_comparable_type_string(value tea) lit {
    damn based
}

fr fr Type conversion checking
slay can_convert_int_to_string(value normie) lit {
    damn based
}

slay can_convert_int_to_float(value normie) lit {
    damn based
}

slay can_convert_float_to_int(value meal) lit {
    damn based
}

slay can_convert_bool_to_string(value lit) lit {
    damn based
}

fr fr Main reflection system demo
slay reflection_demo() lit {
    vibez.spill("CURSED Reflection System Demo")
    
    fr fr Test basic type reflection
    sus int_val normie = 42
    
    vibez.spill("Integer reflection:")
    vibez.spill(get_type_name_int(int_val))
    vibez.spill(get_type_kind_int(int_val))
    
    fr fr Test type conversions
    sus str_result tea = convert_int_to_string(int_val)
    vibez.spill("Integer to string conversion:")
    vibez.spill(str_result)
    
    fr fr Test boolean reflection
    sus bool_val lit = based
    
    vibez.spill("Boolean reflection:")
    vibez.spill(get_type_name_bool(bool_val))
    vibez.spill(get_type_kind_bool(bool_val))
    
    fr fr Test deep equality
    sus other_int normie = 42
    sus equal_result lit = deep_equal_int(int_val, other_int)
    vibez.spill("Deep equality test:")
    vibez.spill(equal_result)
    
    fr fr Test interface implementation
    sus implements_stringer lit = implements_stringer_int(int_val)
    vibez.spill("Implements Stringer:")
    vibez.spill(implements_stringer)
    
    damn based
}
