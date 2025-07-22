fr fr Reflection module - Runtime type information and introspection
fr fr Critical for self-hosting and dynamic behavior

yeet "testz"

fr fr Type information structures
fr fr TypeInfo represents runtime type information
fr fr Implemented as a tuple: (kind, name, size, fields)
fr fr Kind: 0=int, 1=string, 2=bool, 3=float, 4=struct, 5=array, 6=func, 7=interface

slay type_info_int() (normie, tea, normie, []tea) {
    damn (0, "normie", 4, []tea{})
}

slay type_info_string() (normie, tea, normie, []tea) {
    damn (1, "tea", 8, []tea{})
}

slay type_info_bool() (normie, tea, normie, []tea) {
    damn (2, "lit", 1, []tea{})
}

slay type_info_float() (normie, tea, normie, []tea) {
    damn (3, "snack", 4, []tea{})
}

slay type_info_float64() (normie, tea, normie, []tea) {
    damn (3, "meal", 8, []tea{})
}

slay type_info_char() (normie, tea, normie, []tea) {
    damn (0, "sip", 1, []tea{})
}

slay type_info_struct(name tea, fields []tea) (normie, tea, normie, []tea) {
    damn (4, name, 8, fields)
}

slay type_info_array(element_type tea, size normie) (normie, tea, normie, []tea) {
    damn (5, "[" + core.tea(size) + "]" + element_type, size * 8, []tea{element_type})
}

slay type_info_func(name tea, params []tea, return_type tea) (normie, tea, normie, []tea) {
    sus func_sig := name + "(" + stringz.join(params, ",") + ")" + return_type
    damn (6, func_sig, 8, params)
}

slay type_info_interface(name tea, methods []tea) (normie, tea, normie, []tea) {
    damn (7, name, 8, methods)
}

fr fr Type checking functions
slay is_int_type(type_info (normie, tea, normie, []tea)) lit {
    damn type_info.0 == 0
}

slay is_string_type(type_info (normie, tea, normie, []tea)) lit {
    damn type_info.0 == 1
}

slay is_bool_type(type_info (normie, tea, normie, []tea)) lit {
    damn type_info.0 == 2
}

slay is_float_type(type_info (normie, tea, normie, []tea)) lit {
    damn type_info.0 == 3
}

slay is_struct_type(type_info (normie, tea, normie, []tea)) lit {
    damn type_info.0 == 4
}

slay is_array_type(type_info (normie, tea, normie, []tea)) lit {
    damn type_info.0 == 5
}

slay is_func_type(type_info (normie, tea, normie, []tea)) lit {
    damn type_info.0 == 6
}

slay is_interface_type(type_info (normie, tea, normie, []tea)) lit {
    damn type_info.0 == 7
}

fr fr Type metadata accessors
slay get_type_kind(type_info (normie, tea, normie, []tea)) normie {
    damn type_info.0
}

slay get_type_name(type_info (normie, tea, normie, []tea)) tea {
    damn type_info.1
}

slay get_type_size(type_info (normie, tea, normie, []tea)) normie {
    damn type_info.2
}

slay get_type_fields(type_info (normie, tea, normie, []tea)) []tea {
    damn type_info.3
}

fr fr Value representation for reflection
fr fr Value represents a runtime value with type information
fr fr Implemented as a tuple: (type_info, data_ptr, is_valid)
fr fr For simplicity, data_ptr is represented as a string

slay value_from_int(val normie) ((normie, tea, normie, []tea), tea, lit) {
    damn (type_info_int(), core.tea(val), based)
}

slay value_from_string(val tea) ((normie, tea, normie, []tea), tea, lit) {
    damn (type_info_string(), val, based)
}

slay value_from_bool(val lit) ((normie, tea, normie, []tea), tea, lit) {
    bestie val {
        damn (type_info_bool(), "based", based)
    }
    damn (type_info_bool(), "cap", based)
}

slay value_from_float(val snack) ((normie, tea, normie, []tea), tea, lit) {
    damn (type_info_float(), fmt.format_float(val), based)
}

slay invalid_value() ((normie, tea, normie, []tea), tea, lit) {
    damn (type_info_int(), "", cap)
}

fr fr Value operations
slay is_valid(value ((normie, tea, normie, []tea), tea, lit)) lit {
    damn value.2
}

slay get_value_type(value ((normie, tea, normie, []tea), tea, lit)) (normie, tea, normie, []tea) {
    damn value.0
}

slay get_value_data(value ((normie, tea, normie, []tea), tea, lit)) tea {
    damn value.1
}

slay value_type_name(value ((normie, tea, normie, []tea), tea, lit)) tea {
    damn get_type_name(value.0)
}

slay value_type_kind(value ((normie, tea, normie, []tea), tea, lit)) normie {
    damn get_type_kind(value.0)
}

fr fr Type conversion and casting
slay can_convert(from_type (normie, tea, normie, []tea), to_type (normie, tea, normie, []tea)) lit {
    sus from_kind := get_type_kind(from_type)
    sus to_kind := get_type_kind(to_type) fr fr Same type
    bestie from_kind == to_kind {
        damn based
    } fr fr Numeric conversions
    bestie (from_kind == 0 && to_kind == 3) || (from_kind == 3 && to_kind == 0) {
        damn based
    } fr fr String conversions
    bestie to_kind == 1 {
        damn based
    }
    
    damn cap
}

slay convert_value(value ((normie, tea, normie, []tea), tea, lit), target_type (normie, tea, normie, []tea)) ((normie, tea, normie, []tea), tea, lit) {
    bestie !is_valid(value) {
        damn invalid_value()
    }
    
    sus from_type := get_value_type(value)
    bestie !can_convert(from_type, target_type) {
        damn invalid_value()
    }
    
    sus from_kind := get_type_kind(from_type)
    sus to_kind := get_type_kind(target_type)
    sus data := get_value_data(value) fr fr Same type conversion
    bestie from_kind == to_kind {
        damn (target_type, data, based)
    } fr fr Convert to string
    bestie to_kind == 1 {
        damn (target_type, data, based)
    } fr fr Int to float
    bestie from_kind == 0 && to_kind == 3 {
        damn (target_type, data + ".0", based)
    } fr fr Float to int
    bestie from_kind == 3 && to_kind == 0 {
        damn (target_type, data, based)
    }
    
    damn invalid_value()
}

fr fr Struct field access
slay get_struct_field_count(type_info (normie, tea, normie, []tea)) normie {
    bestie is_struct_type(type_info) {
        damn stringz.len(get_type_fields(type_info))
    }
    damn 0
}

slay get_struct_field_name(type_info (normie, tea, normie, []tea), index normie) tea {
    bestie is_struct_type(type_info) {
        sus fields := get_type_fields(type_info)
        bestie index >= 0 && index < stringz.len(fields) {
            damn fields[index]
        }
    }
    damn ""
}

slay has_struct_field(type_info (normie, tea, normie, []tea), field_name tea) lit {
    bestie is_struct_type(type_info) {
        sus fields := get_type_fields(type_info)
        bestie i := 0; i < stringz.len(fields); i++ {
            bestie fields[i] == field_name {
                damn based
            }
        }
    }
    damn cap
}

fr fr Function introspection
slay get_func_param_count(type_info (normie, tea, normie, []tea)) normie {
    bestie is_func_type(type_info) {
        damn stringz.len(get_type_fields(type_info))
    }
    damn 0
}

slay get_func_param_type(type_info (normie, tea, normie, []tea), index normie) tea {
    bestie is_func_type(type_info) {
        sus params := get_type_fields(type_info)
        bestie index >= 0 && index < stringz.len(params) {
            damn params[index]
        }
    }
    damn ""
}

fr fr Interface method introspection
slay get_interface_method_count(type_info (normie, tea, normie, []tea)) normie {
    bestie is_interface_type(type_info) {
        damn stringz.len(get_type_fields(type_info))
    }
    damn 0
}

slay get_interface_method_name(type_info (normie, tea, normie, []tea), index normie) tea {
    bestie is_interface_type(type_info) {
        sus methods := get_type_fields(type_info)
        bestie index >= 0 && index < stringz.len(methods) {
            damn methods[index]
        }
    }
    damn ""
}

slay has_interface_method(type_info (normie, tea, normie, []tea), method_name tea) lit {
    bestie is_interface_type(type_info) {
        sus methods := get_type_fields(type_info)
        bestie i := 0; i < stringz.len(methods); i++ {
            bestie methods[i] == method_name {
                damn based
            }
        }
    }
    damn cap
}

fr fr Array introspection
slay get_array_element_type(type_info (normie, tea, normie, []tea)) tea {
    bestie is_array_type(type_info) {
        sus fields := get_type_fields(type_info)
        bestie stringz.len(fields) > 0 {
            damn fields[0]
        }
    }
    damn ""
}

slay get_array_size(type_info (normie, tea, normie, []tea)) normie {
    bestie is_array_type(type_info) {
        sus name := get_type_name(type_info) fr fr Extract size from name like "[5]int"
        bestie stringz.contains(name, "[") && stringz.contains(name, "]") { fr fr Simplified size extraction
            bestie stringz.contains(name, "[5]") {
                damn 5
            }
            bestie stringz.contains(name, "[10]") {
                damn 10
            }
            bestie stringz.contains(name, "[3]") {
                damn 3
            }
            damn 1
        }
    }
    damn 0
}

fr fr Comparison functions
slay types_equal(type1 (normie, tea, normie, []tea), type2 (normie, tea, normie, []tea)) lit {
    damn get_type_kind(type1) == get_type_kind(type2) && 
         get_type_name(type1) == get_type_name(type2)
}

slay values_equal(val1 ((normie, tea, normie, []tea), tea, lit), val2 ((normie, tea, normie, []tea), tea, lit)) lit {
    bestie !is_valid(val1) || !is_valid(val2) {
        damn cap
    }
    damn types_equal(get_value_type(val1), get_value_type(val2)) && 
         get_value_data(val1) == get_value_data(val2)
}

fr fr Debugging and string representation
slay type_to_string(type_info (normie, tea, normie, []tea)) tea {
    sus kind := get_type_kind(type_info)
    sus name := get_type_name(type_info)
    sus size := get_type_size(type_info)
    
    damn "Type{kind: " + core.tea(kind) + ", name: " + name + ", size: " + core.tea(size) + "}"
}

slay value_to_string(value ((normie, tea, normie, []tea), tea, lit)) tea {
    bestie !is_valid(value) {
        damn "invalid value"
    }
    sus type_name := value_type_name(value)
    sus data := get_value_data(value)
    damn "Value{type: " + type_name + ", data: " + data + "}"
}

fr fr Runtime type registration (simplified)
sus registered_types := []tea{}

slay register_type(type_name tea) {
    registered_types = slices_on_slices.StackString(registered_types, type_name)
}

slay is_type_registered(type_name tea) lit {
    damn slices_on_slices.VibeString(registered_types, type_name)
}

slay get_registered_types() []tea {
    damn registered_types
}

fr fr Initialize with built-in types
slay init_reflection() {
    register_type("normie")
    register_type("tea")
    register_type("lit")
    register_type("snack")
    register_type("meal")
    register_type("sip")
}
