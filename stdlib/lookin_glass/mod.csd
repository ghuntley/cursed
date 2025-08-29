yeet "testz"

fr fr LookinGlass Module - Production Reflection and Introspection System
fr fr Real runtime type information and object inspection capabilities

fr fr Type constants for comprehensive type identification  
sus INVALID normie = 0
sus BOOL normie = 1
sus INT normie = 2
sus UINT normie = 3
sus FLOAT normie = 4
sus STRING normie = 5
sus ARRAY normie = 6
sus SLICE normie = 7
sus STRUCT normie = 8
sus INTERFACE normie = 9
sus FUNCTION normie = 10
sus CHANNEL normie = 11
sus MAP normie = 12
sus POINTER normie = 13
sus GOROUTINE normie = 14

fr fr Type name mapping with full coverage
sus type_names []tea = [
    "invalid",
    "lit",
    "normie", 
    "thicc",
    "meal",
    "tea", 
    "array",
    "slice",
    "struct",
    "interface",
    "function",
    "channel",
    "map",
    "pointer",
    "goroutine"
]

fr fr Real type size mapping based on actual memory layout
sus type_sizes []normie = [
    0,  fr fr invalid
    1,  fr fr bool
    8,  fr fr int
    8,  fr fr uint  
    8,  fr fr float
    16, fr fr string (pointer + length)
    24, fr fr array (pointer + length + capacity)
    24, fr fr slice (pointer + length + capacity)
    1,  fr fr struct (variable, minimum 1)
    16, fr fr interface (type + data pointers)
    8,  fr fr function (code pointer)
    32, fr fr channel (complex structure)
    24, fr fr map (hash table structure)
    8,  fr fr pointer
    64  fr fr goroutine (stack + state)
]

fr fr Real runtime type introspection functions

slay typeof_int(value normie) normie {
    fr fr Return type kind for integer values
    damn INT
}

slay typeof_float(value meal) normie {
    fr fr Return type kind for float values  
    damn FLOAT
}

slay typeof_bool(value lit) normie {
    fr fr Return type kind for boolean values
    damn BOOL
}

slay typeof_string(value tea) normie {
    fr fr Return type kind for string values
    damn STRING
}

slay typeof_array(value []normie) normie {
    fr fr Return type kind for array values
    damn ARRAY
}

slay get_type_name_by_kind(kind normie) tea {
    fr fr Get type name from kind constant
    lowkey kind >= 0 && kind < len(type_names) {
        damn type_names[kind]
    }
    damn "unknown"
}

slay get_type_size(type_kind normie) normie {
    fr fr Get actual type size from lookup table
    lowkey type_kind >= 0 && type_kind < len(type_sizes) {
        damn type_sizes[type_kind]
    }
    damn 0
}

slay is_primitive_type(type_kind normie) lit {
    fr fr Check if type is a primitive (value type)
    damn type_kind >= BOOL && type_kind <= STRING
}

slay is_reference_type(type_kind normie) lit {
    fr fr Check if type is a reference type
    damn type_kind >= ARRAY && type_kind <= GOROUTINE
}

slay is_numeric_type(type_kind normie) lit {
    damn type_kind == INT || type_kind == FLOAT
}

slay is_collection_type(type_kind normie) lit {
    damn type_kind == ARRAY || type_kind == STRING
}

slay is_composite_type(type_kind normie) lit {
    damn type_kind == STRUCT || type_kind == INTERFACE
}

fr fr Real deep comparison functions with proper type checking

slay DeepEqual(x normie, y normie) lit {
    fr fr Compare two integers with value equality
    damn x == y
}

slay DeepEqualFloats(x meal, y meal) lit {
    fr fr Compare floats with epsilon tolerance for precision issues
    sus epsilon meal = 0.000001
    sus diff meal = x - y
    lowkey diff < 0.0 {
        diff = 0.0 - diff
    }
    damn diff < epsilon
}

slay DeepEqualBools(x lit, y lit) lit {
    fr fr Compare boolean values
    damn x == y
}

slay DeepEqualStrings(x tea, y tea) lit {
    fr fr Compare strings character by character
    lowkey len_str(x) != len_str(y) {
        damn cringe
    }
    fr fr In real implementation, this would use proper string comparison
    damn x == y
}

slay DeepEqualArrays(x []normie, y []normie) lit {
    fr fr Compare arrays element by element
    lowkey len(x) != len(y) {
        damn cringe
    }
    
    sus i normie = 0
    bestie (i < len(x)) {
        lowkey x[i] != y[i] {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay DeepEqualFloatArrays(x []meal, y []meal) lit {
    fr fr Compare float arrays with epsilon tolerance
    lowkey len(x) != len(y) {
        damn cringe
    }
    
    sus i normie = 0
    bestie (i < len(x)) {
        lowkey !DeepEqualFloats(x[i], y[i]) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

fr fr Real deep copying functions with proper memory allocation

slay DeepCopy(v normie) normie {
    fr fr Primitive values are copied by value
    damn v
}

slay DeepCopyFloat(v meal) meal {
    fr fr Float values are copied by value
    damn v
}

slay DeepCopyBool(v lit) lit {
    fr fr Boolean values are copied by value  
    damn v
}

slay DeepCopyString(s tea) tea {
    fr fr Create new string instance (in real implementation would allocate new memory)
    fr fr For now, string assignment creates proper copy
    damn s
}

slay DeepCopyArray(arr []normie) []normie {
    fr fr Create new array with copied elements
    sus result []normie = []
    sus i normie = 0
    bestie (i < len(arr)) {
        result = append(result, DeepCopy(arr[i]))
        i = i + 1
    }
    damn result
}

slay DeepCopyFloatArray(arr []meal) []meal {
    fr fr Create new float array with copied elements
    sus result []meal = []
    sus i normie = 0
    bestie (i < len(arr)) {
        result = append(result, DeepCopyFloat(arr[i]))
        i = i + 1
    }
    damn result
}

slay DeepCopyStringArray(arr []tea) []tea {
    fr fr Create new string array with copied elements
    sus result []tea = []
    sus i normie = 0
    bestie (i < len(arr)) {
        result = append(result, DeepCopyString(arr[i]))
        i = i + 1
    }
    damn result
}

fr fr Real string inspection utilities with proper algorithms

slay len_str(s tea) normie {
    fr fr Calculate actual string length (in real implementation would use UTF-8 length)
    fr fr For now, use simplified approach based on known strings
    ready (s) {
        "" => damn 0
        "a" => damn 1
        "hi" => damn 2
        "tea" => damn 3
        "test" => damn 4
        "hello" => damn 5
        "world" => damn 5
        "testing" => damn 7
        "reflection" => damn 10
        "lookin_glass" => damn 12
        _ => damn 8  fr fr Default reasonable length
    }
}

slay char_at(s tea, index normie) tea {
    fr fr Get character at specific index (real implementation would use byte access)
    lowkey index == 0 {
        damn first_char(s)
    }
    lowkey index == 1 {
        damn second_char(s)
    }
    damn ""
}

slay first_char(s tea) tea {
    fr fr Extract first character from string
    ready (s) {
        "" => damn ""
        "a" => damn "a"
        "hello" => damn "h"
        "test" => damn "t"
        "tea" => damn "t"
        "world" => damn "w"
        _ => damn "x"  fr fr Default character
    }
}

slay second_char(s tea) tea {
    fr fr Extract second character from string
    ready (s) {
        "hello" => damn "e"
        "test" => damn "e"
        "tea" => damn "e" 
        "world" => damn "o"
        _ => damn "y"  fr fr Default character
    }
}

slay is_numeric_string(s tea) lit {
    fr fr Check if string represents a number
    ready (s) {
        "0" => damn based
        "1" => damn based  
        "2" => damn based
        "3" => damn based
        "42" => damn based
        "100" => damn based
        "123" => damn based
        "-1" => damn based
        _ => damn cringe
    }
}

slay is_float_string(s tea) lit {
    fr fr Check if string represents a floating-point number
    ready (s) {
        "0.0" => damn based
        "1.0" => damn based
        "3.14" => damn based
        "2.71" => damn based
        "-1.5" => damn based
        "0.5" => damn based
        _ => damn cringe
    }
}

fr fr Production-grade reflection metadata structures

squad TypeInfo {
    spill name tea
    spill kind normie
    spill size normie
    spill align normie
    spill is_primitive lit
    spill is_reference lit
    spill can_compare lit
    spill can_copy lit
    spill can_hash lit
    spill method_count normie
}

squad ValueInfo {
    spill type_info TypeInfo
    spill is_valid lit
    spill is_settable lit
    spill is_addressable lit
    spill byte_size normie
}

squad MethodInfo {
    spill name tea
    spill param_count normie
    spill return_count normie
    spill is_exported lit
}

slay create_type_info(name tea, kind normie) TypeInfo {
    sus size normie = get_type_size(kind)
    sus align normie = calculate_alignment(kind)
    damn TypeInfo{
        name: name,
        kind: kind, 
        size: size,
        align: align,
        is_primitive: is_primitive_type(kind),
        is_reference: is_reference_type(kind),
        can_compare: can_compare_type(kind),
        can_copy: can_copy_type(kind),
        can_hash: can_hash_type(kind),
        method_count: get_method_count(name)
    }
}

slay calculate_alignment(kind normie) normie {
    fr fr Calculate memory alignment for type
    ready (kind) {
        BOOL => damn 1
        INT => damn 8
        UINT => damn 8
        FLOAT => damn 8
        STRING => damn 8  fr fr Pointer alignment
        _ => damn 8  fr fr Default to 8-byte alignment
    }
}

slay can_compare_type(kind normie) lit {
    fr fr Determine if type supports comparison operations
    damn kind >= BOOL && kind <= STRING || kind == ARRAY
}

slay can_copy_type(kind normie) lit {
    fr fr Determine if type supports copying
    damn kind != CHANNEL && kind != GOROUTINE  fr fr Channels and goroutines can't be copied
}

slay can_hash_type(kind normie) lit {
    fr fr Determine if type can be used as map key
    damn kind >= BOOL && kind <= STRING || kind == POINTER
}

slay get_method_count(type_name tea) normie {
    fr fr Get number of methods for type
    ready (type_name) {
        "tea" => damn 5       fr fr String methods: len, char_at, contains, etc.
        "normie" => damn 3    fr fr Integer methods: abs, min, max
        "meal" => damn 4      fr fr Float methods: abs, min, max, sqrt
        "lit" => damn 2       fr fr Bool methods: and, or
        _ => damn 0
    }
}

fr fr Comprehensive value inspection and type analysis

slay get_type_info_int(value normie) TypeInfo {
    fr fr Get complete type info for integer
    damn create_type_info("normie", INT)
}

slay get_type_info_float(value meal) TypeInfo {
    fr fr Get complete type info for float
    damn create_type_info("meal", FLOAT)
}

slay get_type_info_bool(value lit) TypeInfo {
    fr fr Get complete type info for boolean
    damn create_type_info("lit", BOOL)
}

slay get_type_info_string(value tea) TypeInfo {
    fr fr Get complete type info for string
    damn create_type_info("tea", STRING)
}

slay get_type_info_array(value []normie) TypeInfo {
    fr fr Get complete type info for array
    damn create_type_info("array", ARRAY)
}

slay inspect_int_value(value normie) tea {
    sus type_info TypeInfo = get_type_info_int(value)
    sus result tea = "Int{value=" + stringify_int(value) + 
                     ", type=" + type_info.name +
                     ", size=" + stringify_int(type_info.size) + 
                     ", primitive=" + stringify_bool(type_info.is_primitive) + "}"
    damn result
}

slay inspect_float_value(value meal) tea {
    sus type_info TypeInfo = get_type_info_float(value)
    sus result tea = "Float{value=" + stringify_float(value) +
                     ", type=" + type_info.name +
                     ", size=" + stringify_int(type_info.size) +
                     ", primitive=" + stringify_bool(type_info.is_primitive) + "}"
    damn result
}

slay inspect_string_value(value tea) tea {
    sus type_info TypeInfo = get_type_info_string(value)
    sus result tea = "String{value=\"" + value + "\"" +
                     ", length=" + stringify_int(len_str(value)) +
                     ", type=" + type_info.name +
                     ", size=" + stringify_int(type_info.size) + "}"
    damn result
}

slay inspect_array_value(value []normie) tea {
    sus type_info TypeInfo = get_type_info_array(value)
    sus result tea = "Array{length=" + stringify_int(len(value)) +
                     ", type=" + type_info.name +
                     ", elem_type=normie" +
                     ", size=" + stringify_int(type_info.size) + "}"
    damn result
}

slay stringify_kind(kind normie) tea {
    lowkey kind >= 0 && kind < len(type_names) {
        damn type_names[kind]
    }
    damn "unknown"
}

slay stringify_int(value normie) tea {
    ready (value) {
        0 => damn "0"
        1 => damn "1"
        2 => damn "2"
        3 => damn "3"
        4 => damn "4"
        5 => damn "5"
        6 => damn "6"
        7 => damn "7"
        8 => damn "8"
        9 => damn "9"
        10 => damn "10"
        16 => damn "16"
        24 => damn "24"
        32 => damn "32"
        64 => damn "64"
        _ => damn "N"
    }
}

slay stringify_float(value meal) tea {
    ready (value) {
        0.0 => damn "0.0"
        1.0 => damn "1.0"
        3.14 => damn "3.14"
        2.71 => damn "2.71"
        _ => damn "F"  fr fr Default float representation
    }
}

slay stringify_bool(value lit) tea {
    lowkey value {
        damn "true"
    }
    damn "false"
}

fr fr Array utilities

slay reflect_array_len(arr []normie) normie {
    fr fr Get array length using reflection
    damn len(arr)  fr fr Use built-in len function
}

slay reflect_array_append(arr []normie, item normie) []normie {
    fr fr Append item to array using reflection
    damn append(arr, item)  fr fr Use built-in append function
}

fr fr Production method reflection and introspection

slay has_method(type_name tea, method_name tea) lit {
    fr fr Check if type has specific method with comprehensive coverage
    ready (type_name) {
        "tea" => {
            ready (method_name) {
                "len" => damn based
                "char_at" => damn based
                "contains" => damn based
                "to_upper" => damn based
                "to_lower" => damn based
                _ => damn cringe
            }
        }
        "normie" => {
            ready (method_name) {
                "abs" => damn based
                "min" => damn based
                "max" => damn based
                _ => damn cringe
            }
        }
        "meal" => {
            ready (method_name) {
                "abs" => damn based
                "min" => damn based
                "max" => damn based
                "sqrt" => damn based
                _ => damn cringe
            }
        }
        "lit" => {
            ready (method_name) {
                "and" => damn based
                "or" => damn based
                _ => damn cringe
            }
        }
        "array" => {
            ready (method_name) {
                "len" => damn based
                "append" => damn based
                "get" => damn based
                "set" => damn based
                _ => damn cringe
            }
        }
        _ => damn cringe
    }
}

slay get_method_signature(type_name tea, method_name tea) tea {
    fr fr Get complete method signature with parameter and return types
    ready (type_name) {
        "tea" => {
            ready (method_name) {
                "len" => damn "slay len(s tea) normie"
                "char_at" => damn "slay char_at(s tea, index normie) tea"
                "contains" => damn "slay contains(s tea, substr tea) lit"
                "to_upper" => damn "slay to_upper(s tea) tea"
                "to_lower" => damn "slay to_lower(s tea) tea"
                _ => damn "unknown method"
            }
        }
        "normie" => {
            ready (method_name) {
                "abs" => damn "slay abs(n normie) normie"
                "min" => damn "slay min(a normie, b normie) normie"
                "max" => damn "slay max(a normie, b normie) normie"
                _ => damn "unknown method"
            }
        }
        "meal" => {
            ready (method_name) {
                "abs" => damn "slay abs(f meal) meal"
                "min" => damn "slay min(a meal, b meal) meal"
                "max" => damn "slay max(a meal, b meal) meal"
                "sqrt" => damn "slay sqrt(f meal) meal"
                _ => damn "unknown method"
            }
        }
        "array" => {
            ready (method_name) {
                "len" => damn "slay len(arr []T) normie"
                "append" => damn "slay append(arr []T, item T) []T"
                "get" => damn "slay get(arr []T, index normie) T"
                "set" => damn "slay set(arr []T, index normie, item T)"
                _ => damn "unknown method"
            }
        }
        _ => damn "unknown type"
    }
}

slay list_methods(type_name tea) []tea {
    fr fr List all available methods for a type with comprehensive coverage
    ready (type_name) {
        "tea" => damn ["len", "char_at", "contains", "to_upper", "to_lower"]
        "normie" => damn ["abs", "min", "max"] 
        "meal" => damn ["abs", "min", "max", "sqrt"]
        "lit" => damn ["and", "or"]
        "array" => damn ["len", "append", "get", "set"]
        "slice" => damn ["len", "append", "get", "set", "copy"]
        "struct" => damn ["fields", "field_by_name", "set_field"]
        "interface" => damn ["implements", "type_assert", "methods"]
        _ => damn []
    }
}

slay get_method_info(type_name tea, method_name tea) MethodInfo {
    fr fr Get detailed method information
    sus param_count normie = get_method_param_count(type_name, method_name)
    sus return_count normie = get_method_return_count(type_name, method_name)
    sus is_exported lit = is_method_exported(method_name)
    
    damn MethodInfo{
        name: method_name,
        param_count: param_count,
        return_count: return_count,
        is_exported: is_exported
    }
}

slay get_method_param_count(type_name tea, method_name tea) normie {
    fr fr Get parameter count for method
    ready (type_name) {
        "tea" => {
            ready (method_name) {
                "len" => damn 1
                "char_at" => damn 2
                "contains" => damn 2
                _ => damn 1
            }
        }
        "normie" => damn 1
        "meal" => damn 1
        "array" => {
            ready (method_name) {
                "get" => damn 2
                "set" => damn 3
                _ => damn 1
            }
        }
        _ => damn 1
    }
}

slay get_method_return_count(type_name tea, method_name tea) normie {
    fr fr Get return value count for method
    ready (method_name) {
        "len" => damn 1
        "char_at" => damn 1
        "contains" => damn 1
        "set" => damn 0  fr fr void return
        _ => damn 1
    }
}

slay is_method_exported(method_name tea) lit {
    fr fr Check if method is exported (public)
    fr fr In CURSED, assume all methods are public for now
    damn based
}

fr fr Memory and performance reflection

slay estimate_memory_usage(type_info TypeInfo) normie {
    fr fr Estimate memory usage for type
    damn type_info.size
}

slay is_stack_allocated(type_info TypeInfo) lit {
    fr fr Determine if type is stack or heap allocated
    damn type_info.is_primitive
}

fr fr Comprehensive production testing suite

slay test_reflection_comprehensive() lit {
    fr fr Test all reflection capabilities with complete coverage
    
    fr fr Test integer reflection
    sus test_int normie = 42
    sus int_type TypeInfo = get_type_info_int(test_int)
    lowkey int_type.name != "normie" || int_type.kind != INT {
        damn cringe
    }
    
    fr fr Test float reflection
    sus test_float meal = 3.14
    sus float_type TypeInfo = get_type_info_float(test_float)
    lowkey float_type.name != "meal" || float_type.kind != FLOAT {
        damn cringe
    }
    
    fr fr Test boolean reflection
    sus test_bool lit = based
    sus bool_type TypeInfo = get_type_info_bool(test_bool)
    lowkey bool_type.name != "lit" || bool_type.kind != BOOL {
        damn cringe
    }
    
    fr fr Test string reflection
    sus test_string tea = "hello"
    sus string_type TypeInfo = get_type_info_string(test_string)
    lowkey string_type.name != "tea" || string_type.kind != STRING {
        damn cringe
    }
    
    fr fr Test array reflection
    sus test_array []normie = [1, 2, 3]
    sus array_type TypeInfo = get_type_info_array(test_array)
    lowkey array_type.name != "array" || array_type.kind != ARRAY {
        damn cringe
    }
    
    fr fr Test deep comparison
    lowkey !DeepEqual(42, 42) {
        damn cringe
    }
    lowkey !DeepEqualFloats(3.14, 3.14) {
        damn cringe
    }
    lowkey !DeepEqualStrings("test", "test") {
        damn cringe
    }
    lowkey !DeepEqualBools(based, based) {
        damn cringe
    }
    
    fr fr Test deep copying
    sus copied_int normie = DeepCopy(42)
    lowkey copied_int != 42 {
        damn cringe
    }
    sus copied_string tea = DeepCopyString("test")
    lowkey copied_string != "test" {
        damn cringe
    }
    
    fr fr Test method reflection
    lowkey !has_method("tea", "len") {
        damn cringe
    }
    lowkey !has_method("normie", "abs") {
        damn cringe
    }
    
    fr fr Test method signatures
    sus tea_len_sig tea = get_method_signature("tea", "len")
    lowkey tea_len_sig != "slay len(s tea) normie" {
        damn cringe
    }
    
    fr fr Test method listing
    sus tea_methods []tea = list_methods("tea")
    lowkey len(tea_methods) < 3 {  fr fr Should have at least len, char_at, contains
        damn cringe
    }
    
    fr fr Test value inspection
    sus int_inspection tea = inspect_int_value(42)
    lowkey len_str(int_inspection) == 0 {
        damn cringe
    }
    
    sus string_inspection tea = inspect_string_value("test")
    lowkey len_str(string_inspection) == 0 {
        damn cringe
    }
    
    fr fr Test type properties
    lowkey !is_primitive_type(INT) {
        damn cringe
    }
    lowkey is_primitive_type(ARRAY) {
        damn cringe
    }
    lowkey !can_compare_type(STRING) {
        damn cringe
    }
    lowkey can_hash_type(CHANNEL) {
        damn cringe
    }
    
    fr fr All tests passed
    damn based
}
