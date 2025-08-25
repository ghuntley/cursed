// CURSED FFI Bridge - Native C ABI Support for Extern Functions
// Provides easy extern "C" function declarations and calling

yeet "stringz"
yeet "vibez"
yeet "concurrenz"
yeet "testz"

// C ABI type mappings for CURSED types
enum CABIType normie {
    Void = 0,
    Int8 = 1,
    Int16 = 2,
    Int32 = 3,
    Int64 = 4,
    UInt8 = 5,
    UInt16 = 6,
    UInt32 = 7,
    UInt64 = 8,
    Float32 = 9,
    Float64 = 10,
    Pointer = 11,
    String = 12,
    // Enhanced enum support
    EnumInt8 = 13,
    EnumInt16 = 14,
    EnumInt32 = 15,
    EnumInt64 = 16,
    EnumUInt8 = 17,
    EnumUInt16 = 18,
    EnumUInt32 = 19,
    EnumUInt64 = 20
}

// Calling convention support
enum CallingConvention normie {
    C = 0,
    Stdcall = 1,
    Fastcall = 2
}

// FFI error types
enum FFIError normie {
    LibraryNotFound = 1,
    FunctionNotFound = 2,
    InvalidSignature = 3,
    CallFailed = 4,
    TypeMismatch = 5,
    SystemError = 6,
    NotSupported = 7
}

// C ABI parameter definition
squad CABIParameter {
    name tea,
    param_type CABIType
}

// C ABI function signature
squad CABISignature {
    name tea,
    return_type CABIType,
    parameters []CABIParameter,
    calling_convention CallingConvention
}

// External function information
squad ExternFunction {
    signature CABISignature,
    function_ptr drip,
    wrapper_generated lit
}

// External library interface
squad ExternLibrary {
    name tea,
    handle drip,
    functions {tea: ExternFunction},
    loaded lit
}

// Main FFI bridge structure
squad FFIBridge {
    libraries {tea: ExternLibrary},
    type_mappings {tea: CABIType},
    generated_wrappers []tea,
    enum_mappings {tea: CABIType}
}

// Initialize FFI bridge with default type mappings
slay init_ffi_bridge() FFIBridge {
    sus bridge FFIBridge = FFIBridge{
        libraries: {},
        type_mappings: {},
        generated_wrappers: [],
        enum_mappings: {}
    }
    
    // Initialize CURSED to C type mappings
    initialize_type_mappings(bridge)
    
    damn bridge
}

// Initialize default type mappings
slay initialize_type_mappings(bridge FFIBridge) vibes {
    bridge.type_mappings["lit"] = CABIType.Int8        // boolean -> char
    bridge.type_mappings["smol"] = CABIType.Int8       // small int -> char  
    bridge.type_mappings["normie"] = CABIType.Int32    // normal int -> int
    bridge.type_mappings["drip"] = CABIType.Int64      // big int -> long long
    bridge.type_mappings["thicc"] = CABIType.Int64     // huge int -> long long
    bridge.type_mappings["snack"] = CABIType.Float32   // small float -> float
    bridge.type_mappings["meal"] = CABIType.Float64    // big float -> double
    bridge.type_mappings["tea"] = CABIType.String      // string -> const char*
    bridge.type_mappings["vibes"] = CABIType.Void      // void -> void
}

// Register external library
slay register_library(bridge FFIBridge, library_name tea) ExternLibrary yikes FFIError {
    ready bridge.libraries.contains(library_name) {
        damn bridge.libraries[library_name]
    }
    
    sus library ExternLibrary = ExternLibrary{
        name: library_name,
        handle: 0,
        functions: {},
        loaded: goofy
    }
    
    bridge.libraries[library_name] = library
    damn library
}

// Load external library dynamically
slay load_library(library ExternLibrary) FFIError yikes vibes {
    ready library.loaded {
        damn // Already loaded
    }
    
    sus handle drip = dynamic_library_load(library.name) fam {
        when _ -> yikes FFIError.LibraryNotFound
    }
    
    library.handle = handle
    library.loaded = based
}

// Declare extern function in library
slay declare_extern_function(library ExternLibrary, signature CABISignature) FFIError yikes vibes {
    sus function ExternFunction = ExternFunction{
        signature: signature,
        function_ptr: 0,
        wrapper_generated: goofy
    }
    
    library.functions[signature.name] = function
}

// Resolve function pointer from loaded library
slay resolve_function(library ExternLibrary, func_name tea) FFIError yikes drip {
    ready !library.loaded {
        load_library(library) yikes shook
    }
    
    sus func_ptr drip = dynamic_library_symbol(library.handle, func_name) fam {
        when _ -> yikes FFIError.FunctionNotFound
    }
    
    // Update function entry
    ready library.functions.contains(func_name) {
        library.functions[func_name].function_ptr = func_ptr
    }
    
    damn func_ptr
}

// Parse extern declaration from text
slay parse_extern_declaration(bridge FFIBridge, decl_text tea) FFIError yikes CABISignature {
    // Simple parser for extern declarations like:
    // extern "C" int add(int a, int b);
    
    sus tokens []tea = tokenize_declaration(decl_text)
    ready tokens.length < 4 {
        yikes FFIError.InvalidSignature
    }
    
    sus signature CABISignature = CABISignature{
        name: "",
        return_type: CABIType.Void,
        parameters: [],
        calling_convention: CallingConvention.C
    }
    
    sus token_index normie = 0
    
    // Skip "extern"
    ready tokens[token_index] == "extern" {
        token_index = token_index + 1
    }
    
    // Parse ABI
    ready token_index < tokens.length {
        sus abi tea = tokens[token_index]
        ready abi == "\"C\"" {
            signature.calling_convention = CallingConvention.C
        } otherwise ready abi == "\"stdcall\"" {
            signature.calling_convention = CallingConvention.Stdcall
        }
        token_index = token_index + 1
    }
    
    // Parse return type
    ready token_index < tokens.length {
        sus return_type_str tea = tokens[token_index]
        signature.return_type = parse_c_type(bridge, return_type_str)
        token_index = token_index + 1
    }
    
    // Parse function name
    ready token_index < tokens.length {
        signature.name = tokens[token_index]
        token_index = token_index + 1
    }
    
    // Parse parameters (simplified - assumes alternating type/name pairs)
    bestie token_index < tokens.length - 1 {
        sus param_type_str tea = tokens[token_index]
        sus param_name tea = tokens[token_index + 1]
        
        sus param CABIParameter = CABIParameter{
            name: param_name,
            param_type: parse_c_type(bridge, param_type_str)
        }
        
        signature.parameters.append(param)
        token_index = token_index + 2
    }
    
    damn signature
}

// Parse C type string to CABIType
slay parse_c_type(bridge FFIBridge, type_str tea) CABIType {
    // Check enum mappings first
    ready bridge.enum_mappings.contains(type_str) {
        damn bridge.enum_mappings[type_str]
    }
    
    // Standard C type parsing
    sick type_str {
        when "void" -> damn CABIType.Void
        when "char" -> damn CABIType.Int8
        when "signed char" -> damn CABIType.Int8
        when "unsigned char" -> damn CABIType.UInt8
        when "short" -> damn CABIType.Int16
        when "unsigned short" -> damn CABIType.UInt16
        when "int" -> damn CABIType.Int32
        when "unsigned int" -> damn CABIType.UInt32
        when "unsigned" -> damn CABIType.UInt32
        when "long" -> damn CABIType.Int64
        when "unsigned long" -> damn CABIType.UInt64
        when "long long" -> damn CABIType.Int64
        when "unsigned long long" -> damn CABIType.UInt64
        when "float" -> damn CABIType.Float32
        when "double" -> damn CABIType.Float64
        otherwise -> {
            ready type_str.contains("*") {
                ready type_str.contains("char*") {
                    damn CABIType.String
                }
                damn CABIType.Pointer
            }
            ready type_str.starts_with("enum ") {
                damn CABIType.EnumInt32 // Default enum size
            }
            damn CABIType.Int32 // Default fallback
        }
    }
}

// Generate CURSED wrapper for extern function  
slay generate_wrapper(bridge FFIBridge, signature CABISignature, library_name tea) tea {
    sus wrapper tea = ""
    
    wrapper = wrapper + "// Auto-generated wrapper for extern function " + signature.name + "\n"
    wrapper = wrapper + "slay " + signature.name + "("
    
    // Generate parameters
    bestie i normie in 0..signature.parameters.length {
        ready i > 0 {
            wrapper = wrapper + ", "
        }
        sus param CABIParameter = signature.parameters[i]
        sus cursed_type tea = c_type_to_cursed_type(param.param_type)
        wrapper = wrapper + param.name + " " + cursed_type
    }
    
    sus return_type tea = c_type_to_cursed_type(signature.return_type)
    ready return_type == "vibes" {
        wrapper = wrapper + ") vibes {\n"
    } otherwise {
        wrapper = wrapper + ") " + return_type + " {\n"
    }
    
    // Generate FFI call
    wrapper = wrapper + "    // FFI call to " + library_name + "." + signature.name + "\n"
    
    ready signature.return_type != CABIType.Void {
        wrapper = wrapper + "    sus result " + return_type + " = "
    } otherwise {
        wrapper = wrapper + "    "
    }
    
    wrapper = wrapper + "cursed_ffi_call(\"" + library_name + "\", \"" + signature.name + "\""
    
    bestie param CABIParameter in signature.parameters {
        wrapper = wrapper + ", " + param.name
    }
    
    wrapper = wrapper + ")\n"
    
    ready signature.return_type != CABIType.Void {
        wrapper = wrapper + "    damn result\n"
    }
    
    wrapper = wrapper + "}\n\n"
    
    bridge.generated_wrappers.append(wrapper)
    damn wrapper
}

// Convert C type to CURSED type
slay c_type_to_cursed_type(c_type CABIType) tea {
    sick c_type {
        when CABIType.Void -> damn "vibes"
        when CABIType.Int8 -> damn "smol"
        when CABIType.Int16 -> damn "smol"
        when CABIType.Int32 -> damn "normie"
        when CABIType.Int64 -> damn "drip"
        when CABIType.UInt8 -> damn "smol"
        when CABIType.UInt16 -> damn "smol"
        when CABIType.UInt32 -> damn "normie"
        when CABIType.UInt64 -> damn "drip"
        when CABIType.Float32 -> damn "snack"
        when CABIType.Float64 -> damn "meal"
        when CABIType.Pointer -> damn "*vibes"
        when CABIType.String -> damn "tea"
        // Enhanced enum support - enums use same CURSED types as underlying types
        when CABIType.EnumInt8 -> damn "smol"
        when CABIType.EnumInt16 -> damn "smol"
        when CABIType.EnumInt32 -> damn "normie"
        when CABIType.EnumInt64 -> damn "drip"
        when CABIType.EnumUInt8 -> damn "smol"
        when CABIType.EnumUInt16 -> damn "smol"
        when CABIType.EnumUInt32 -> damn "normie"
        when CABIType.EnumUInt64 -> damn "drip"
        otherwise -> damn "vibes"
    }
}

// Convert CURSED type to C type
slay cursed_type_to_c_type(cursed_type tea) tea {
    sick cursed_type {
        when "lit" -> damn "int"
        when "smol" -> damn "char"
        when "normie" -> damn "int"
        when "drip" -> damn "long long"
        when "thicc" -> damn "long long"
        when "snack" -> damn "float"
        when "meal" -> damn "double"
        when "tea" -> damn "const char*"
        when "vibes" -> damn "void"
        otherwise -> {
            ready cursed_type.starts_with("*") {
                damn "void*"
            }
            ready cursed_type.starts_with("[]") {
                damn "void*" // Arrays passed as pointers
            }
            damn "void*"
        }
    }
}

// Generate C header for CURSED functions
slay generate_c_header(bridge FFIBridge, cursed_functions []tea) tea {
    sus header tea = ""
    
    header = header + "#ifndef CURSED_C_BINDINGS_H\n"
    header = header + "#define CURSED_C_BINDINGS_H\n\n"
    header = header + "#ifdef __cplusplus\n"
    header = header + "extern \"C\" {\n"
    header = header + "#endif\n\n"
    
    bestie func tea in cursed_functions {
        // Parse function to extract signature info
        sus signature CABISignature = parse_cursed_function(func) fam {
            when _ -> CABISignature{name: "unknown", return_type: CABIType.Void, parameters: [], calling_convention: CallingConvention.C}
        }
        
        sus c_return_type tea = c_type_to_c_string(signature.return_type)
        header = header + c_return_type + " cursed_" + signature.name + "("
        
        bestie i normie in 0..signature.parameters.length {
            ready i > 0 {
                header = header + ", "
            }
            sus param CABIParameter = signature.parameters[i]
            sus c_param_type tea = c_type_to_c_string(param.param_type)
            header = header + c_param_type + " " + param.name
        }
        
        header = header + ");\n"
    }
    
    header = header + "\n#ifdef __cplusplus\n"
    header = header + "}\n"
    header = header + "#endif\n\n"
    header = header + "#endif // CURSED_C_BINDINGS_H\n"
    
    damn header
}

// Convert CABIType to C type string
slay c_type_to_c_string(c_type CABIType) tea {
    sick c_type {
        when CABIType.Void -> damn "void"
        when CABIType.Int8 -> damn "char"
        when CABIType.Int16 -> damn "short"
        when CABIType.Int32 -> damn "int"
        when CABIType.Int64 -> damn "long long"
        when CABIType.UInt8 -> damn "unsigned char"
        when CABIType.UInt16 -> damn "unsigned short"
        when CABIType.UInt32 -> damn "unsigned int"
        when CABIType.UInt64 -> damn "unsigned long long"
        when CABIType.Float32 -> damn "float"
        when CABIType.Float64 -> damn "double"
        when CABIType.Pointer -> damn "void*"
        when CABIType.String -> damn "const char*"
        // Enhanced enum support
        when CABIType.EnumInt8 -> damn "signed char"
        when CABIType.EnumInt16 -> damn "short"
        when CABIType.EnumInt32 -> damn "int"
        when CABIType.EnumInt64 -> damn "long long"
        when CABIType.EnumUInt8 -> damn "unsigned char"
        when CABIType.EnumUInt16 -> damn "unsigned short"
        when CABIType.EnumUInt32 -> damn "unsigned int"
        when CABIType.EnumUInt64 -> damn "unsigned long long"
        otherwise -> damn "void*"
    }
}

// Generate FFI runtime implementation
slay generate_ffi_runtime(bridge FFIBridge) tea {
    sus runtime tea = ""
    
    runtime = runtime + "// Auto-generated FFI runtime for C interop\n\n"
    runtime = runtime + "yeet \"cursed_ffi_internal\"\n\n"
    
    runtime = runtime + "// FFI call dispatcher\n"
    runtime = runtime + "slay cursed_ffi_call(library_name tea, function_name tea, ...args) tea {\n"
    runtime = runtime + "    // Load library if not already loaded\n"
    runtime = runtime + "    sus lib_handle drip = load_dynamic_library(library_name)\n"
    runtime = runtime + "    ready lib_handle == 0 {\n"
    runtime = runtime + "        damn \"ERROR: Could not load library \" + library_name\n"
    runtime = runtime + "    }\n\n"
    
    runtime = runtime + "    // Get function pointer\n"
    runtime = runtime + "    sus func_ptr drip = get_function_symbol(lib_handle, function_name)\n"
    runtime = runtime + "    ready func_ptr == 0 {\n"
    runtime = runtime + "        damn \"ERROR: Could not find function \" + function_name\n"
    runtime = runtime + "    }\n\n"
    
    runtime = runtime + "    // Call function with arguments\n"
    runtime = runtime + "    damn call_c_function(func_ptr, args)\n"
    runtime = runtime + "}\n\n"
    
    damn runtime
}

// Parse extern block containing multiple declarations
slay parse_extern_block(bridge FFIBridge, block_text tea) FFIError yikes vibes {
    sus lines []tea = block_text.split("\n")
    sus current_library ?tea = null
    
    bestie line tea in lines {
        sus trimmed tea = line.trim()
        ready trimmed.length == 0 || trimmed.starts_with("//") {
            continue // Skip empty lines and comments
        }
        
        ready trimmed.starts_with("library") {
            // Parse library declaration: library "libname"
            current_library = parse_library_declaration(trimmed) fam {
                when _ -> yikes FFIError.InvalidSignature
            }
        } otherwise ready trimmed.starts_with("extern") {
            // Parse extern function: extern "C" int func(int x);
            ready current_library != null {
                sus signature CABISignature = parse_extern_declaration(bridge, trimmed) yikes shook
                sus library ExternLibrary = register_library(bridge, current_library?) yikes shook
                declare_extern_function(library, signature) yikes shook
            }
        }
    }
}

// Parse library declaration
slay parse_library_declaration(line tea) tea yikes FFIError {
    // Parse: library "libname"
    sus start_quote normie = line.find("\"") fam {
        when _ -> yikes FFIError.InvalidSignature
    }
    sus end_quote normie = line.rfind("\"") fam {
        when _ -> yikes FFIError.InvalidSignature
    }
    
    ready start_quote >= end_quote {
        yikes FFIError.InvalidSignature
    }
    
    damn line.substr(start_quote + 1, end_quote - start_quote - 1)
}

// Enhanced enum support methods
slay register_c_enum(bridge FFIBridge, enum_name tea, enum_type CABIType) vibes {
    bridge.enum_mappings[enum_name] = enum_type
}

slay generate_enum_binding(bridge FFIBridge, enum_name tea, values {tea: normie}) tea {
    sus binding tea = ""
    
    binding = binding + "// Auto-generated CURSED enum binding for " + enum_name + "\n"
    binding = binding + "enum " + enum_name + " normie {\n"
    
    bestie name tea, value normie in values {
        binding = binding + "    " + name + " = " + value.to_string() + ",\n"
    }
    
    binding = binding + "}\n\n"
    
    // Generate conversion functions
    binding = binding + "slay " + enum_name + "_to_raw(value " + enum_name + ") normie {\n"
    binding = binding + "    damn value as normie\n"
    binding = binding + "}\n\n"
    
    binding = binding + "slay raw_to_" + enum_name + "(value normie) " + enum_name + " {\n"
    binding = binding + "    damn value as " + enum_name + "\n"
    binding = binding + "}\n\n"
    
    damn binding
}

// Marshall enum values between CURSED and C
slay marshall_enum_to_c(bridge FFIBridge, enum_name tea, cursed_value normie) normie {
    // In most cases, values are the same
    damn cursed_value
}

slay marshall_enum_from_c(bridge FFIBridge, enum_name tea, c_value normie) normie {
    // In most cases, values are the same
    damn c_value
}

// Utility functions (would be implemented by runtime)
slay dynamic_library_load(name tea) drip {
    // Platform-specific library loading
    // Linux: dlopen(), Windows: LoadLibrary(), etc.
    damn system_ffi_call("dlopen", name, 2) fam { // RTLD_NOW
        when _ -> damn 0
    }
}

slay dynamic_library_symbol(handle drip, name tea) drip {
    // Platform-specific symbol lookup
    // Linux: dlsym(), Windows: GetProcAddress(), etc.
    damn system_ffi_call("dlsym", handle, name) fam {
        when _ -> damn 0
    }
}

slay call_c_function(func_ptr drip, args []tea) tea {
    // Call C function with arguments
    // This would be implemented in the CURSED runtime
    damn system_ffi_call("call_func", func_ptr, args) fam {
        when _ -> damn ""
    }
}

slay load_dynamic_library(name tea) drip {
    damn dynamic_library_load(name)
}

slay get_function_symbol(handle drip, name tea) drip {
    damn dynamic_library_symbol(handle, name)
}

// String utilities
slay tokenize_declaration(decl tea) []tea {
    // Simple tokenizer - splits on whitespace and punctuation
    sus tokens []tea = []
    sus current_token tea = ""
    
    bestie ch smol in decl {
        ready ch == ' ' || ch == '\t' || ch == '(' || ch == ')' || ch == ',' || ch == ';' {
            ready current_token.length > 0 {
                tokens.append(current_token)
                current_token = ""
            }
        } otherwise {
            current_token = current_token + ch.to_string()
        }
    }
    
    ready current_token.length > 0 {
        tokens.append(current_token)
    }
    
    damn tokens
}

slay parse_cursed_function(func_text tea) CABISignature yikes FFIError {
    // Parse CURSED function to extract signature (simplified)
    sus signature CABISignature = CABISignature{
        name: "unknown",
        return_type: CABIType.Void,
        parameters: [],
        calling_convention: CallingConvention.C
    }
    
    // Extract function name (after "slay ")
    sus slay_pos normie = func_text.find("slay ") fam {
        when _ -> yikes FFIError.InvalidSignature
    }
    
    sus name_start normie = slay_pos + 5
    sus paren_pos normie = func_text.find("(", name_start) fam {
        when _ -> yikes FFIError.InvalidSignature
    }
    
    signature.name = func_text.substr(name_start, paren_pos - name_start).trim()
    
    damn signature
}

// Generic system FFI call interface
slay system_ffi_call(call_name tea, ...args) drip {
    // This would be implemented by the CURSED runtime
    // For now, return placeholder
    damn 42
}

// Test functions
slay test_ffi_bridge() vibes {
    sus bridge FFIBridge = init_ffi_bridge()
    
    // Test library registration
    sus libc ExternLibrary = register_library(bridge, "libc") fam {
        when _ -> {
            vibez.spill("Failed to register library")
            damn
        }
    }
    
    // Test extern declaration parsing
    sus decl tea = "extern \"C\" int strlen(const char* str)"
    sus signature CABISignature = parse_extern_declaration(bridge, decl) fam {
        when _ -> {
            vibez.spill("Failed to parse extern declaration")
            damn
        }
    }
    
    vibez.spill("Parsed function:", signature.name)
    vibez.spill("Return type:", signature.return_type)
    vibez.spill("Parameters:", signature.parameters.length)
    
    // Test wrapper generation
    sus wrapper tea = generate_wrapper(bridge, signature, "libc")
    vibez.spill("Generated wrapper:")
    vibez.spill(wrapper)
    
    // Test C header generation
    sus header tea = generate_c_header(bridge, ["slay test_func(x normie) normie"])
    vibez.spill("Generated C header:")
    vibez.spill(header)
    
    vibez.spill("FFI bridge test completed")
}

// Example usage
slay example_ffi_usage() vibes {
    sus bridge FFIBridge = init_ffi_bridge()
    
    // Register and load library
    sus math_lib ExternLibrary = register_library(bridge, "libm") fam {
        when _ -> {
            vibez.spill("Failed to register math library")
            damn
        }
    }
    
    load_library(math_lib) fam {
        when FFIError.LibraryNotFound -> {
            vibez.spill("Math library not found")
            damn
        }
        otherwise -> {
            vibez.spill("Failed to load math library")
            damn
        }
    }
    
    // Declare extern functions
    sus sin_signature CABISignature = CABISignature{
        name: "sin",
        return_type: CABIType.Float64,
        parameters: [CABIParameter{name: "x", param_type: CABIType.Float64}],
        calling_convention: CallingConvention.C
    }
    
    declare_extern_function(math_lib, sin_signature) fam {
        when _ -> {
            vibez.spill("Failed to declare sin function")
            damn
        }
    }
    
    // Generate wrapper
    sus wrapper tea = generate_wrapper(bridge, sin_signature, "libm")
    vibez.spill("Generated sin wrapper:")
    vibez.spill(wrapper)
    
    vibez.spill("Example FFI usage completed")
}
