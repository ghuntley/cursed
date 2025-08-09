fr fr Comprehensive test for new language features
fr fr P26: Exhaustive pattern checking
fr fr P29: Generic type inference  
fr fr P30: Compile-time reflection
fr fr P31: Macro hygiene
fr fr P33: Extern function ABI

yeet "testz"
yeet "reflection"

fr fr =================================
fr fr P26: EXHAUSTIVE PATTERN CHECKING
fr fr =================================

fr fr Define an enum for testing exhaustiveness
enum Color {
    Red,
    Green,
    Blue,
    Custom(normie)
}

fr fr Test non-exhaustive pattern matching (should warn)
slay test_exhaustive_patterns() {
    sus color Color = Color::Red
    
    fr fr This match is NOT exhaustive - missing Custom variant
    sick (color) {
        when Red -> vibez.spill("Red color")
        when Green -> vibez.spill("Green color") 
        when Blue -> vibez.spill("Blue color")
        fr fr Missing: Custom case - compiler should warn
    }
    
    fr fr This match IS exhaustive with wildcard
    sick (color) {
        when Red -> vibez.spill("Red")
        when Green -> vibez.spill("Green")
        when _ -> vibez.spill("Other color")
    }
}

fr fr =================================
fr fr P29: GENERIC TYPE INFERENCE
fr fr =================================

fr fr Generic function that should infer types from arguments
slay swap<T>(a T, b T) (T, T) {
    damn (b, a)
}

fr fr Generic container
struct Container<T> {
    spill value T
    spill count normie
}

slay test_type_inference() {
    fr fr These calls should infer T automatically
    sus result1 = swap(42, 84)        fr fr Should infer T = normie
    sus result2 = swap("hello", "world")  fr fr Should infer T = tea
    sus result3 = swap(3.14, 2.71)   fr fr Should infer T = meal
    
    vibez.spill("Inferred swap results:")
    vibez.spill("Numbers: " + result1.0 + ", " + result1.1)
    vibez.spill("Strings: " + result2.0 + ", " + result2.1)
    
    fr fr Generic container with inference
    sus int_container = Container{value: 42, count: 1}     fr fr Should infer T = normie
    sus str_container = Container{value: "test", count: 1} fr fr Should infer T = tea
    
    vibez.spill("Container values: " + int_container.value + ", " + str_container.value)
}

fr fr =================================
fr fr P30: COMPILE-TIME REFLECTION
fr fr =================================

struct Person {
    spill name tea
    spill age normie  
    spill active lit
    spill score meal
}

slay test_compile_time_reflection() {
    vibez.spill("=== Compile-time Reflection Tests ===")
    
    fr fr type.fields should work at compile time
    sus person_fields = Person.fields  fr fr Compile-time field information
    vibez.spill("Person has " + len(person_fields) + " fields")
    
    fr fr type.size should work at compile time  
    sus person_size = Person.size      fr fr Compile-time size calculation
    vibez.spill("Person size: " + person_size + " bytes")
    
    fr fr type.methods for interfaces
    collab Printable {
        slay to_string() tea
    }
    
    sus printable_methods = Printable.methods
    vibez.spill("Printable has " + len(printable_methods) + " methods")
    
    fr fr Generate code based on reflection
    fr fr This should generate getters/setters at compile time
    @generate_accessors(Person)
    
    fr fr Test generated accessors
    sus person Person = Person{name: "Alice", age: 30, active: based, score: 95.5}
    vibez.spill("Name via accessor: " + get_Person_name(&person))
    
    set_Person_age(&person, 31)
    vibez.spill("Updated age: " + get_Person_age(&person))
}

fr fr =================================
fr fr P31: MACRO HYGIENE
fr fr =================================

fr fr Define a macro that could have hygiene issues
@macro
slay debug_print(expr) {
    sus temp = expr  fr fr This 'temp' should not capture outer 'temp'
    vibez.spill("DEBUG: " + temp)
    damn temp
}

slay test_macro_hygiene() {
    vibez.spill("=== Macro Hygiene Tests ===")
    
    sus temp normie = 100  fr fr This should not be captured by macro
    
    fr fr The macro should generate hygienic names to avoid capture
    sus result1 = debug_print(42)
    sus result2 = debug_print("hello")
    
    vibez.spill("Original temp still: " + temp)  fr fr Should still be 100
    vibez.spill("Macro results: " + result1 + ", " + result2)
    
    fr fr Test nested macro expansions
    @macro
    slay double_debug(expr) {
        debug_print(debug_print(expr))
    }
    
    sus nested_result = double_debug(123)
    vibez.spill("Nested macro result: " + nested_result)
}

fr fr =================================
fr fr P33: EXTERN FUNCTION ABI
fr fr =================================

fr fr Simple extern block for C functions
extern "C" {
    library "libc"
    
    fr fr Standard C library functions
    slay strlen(str tea) normie
    slay strcmp(str1 tea, str2 tea) normie
    slay printf(format tea, ...args) normie
    slay malloc(size normie) *vibes
    slay free(ptr *vibes)
}

fr fr Custom C library
extern "C" {
    library "libmath"
    
    slay sin(x meal) meal
    slay cos(x meal) meal
    slay sqrt(x meal) meal
}

slay test_extern_functions() {
    vibez.spill("=== Extern Function Tests ===")
    
    fr fr Test C string functions
    sus str tea = "Hello, World!"
    sus len normie = strlen(str)
    vibez.spill("String length: " + len)
    
    sus cmp_result normie = strcmp("hello", "world")
    vibez.spill("String comparison: " + cmp_result)
    
    fr fr Test math functions
    sus angle meal = 3.14159 / 4.0
    sus sin_val meal = sin(angle)
    sus cos_val meal = cos(angle)
    
    vibez.spill("sin(π/4) = " + sin_val)
    vibez.spill("cos(π/4) = " + cos_val)
    
    fr fr Test memory allocation
    sus ptr *vibes = malloc(1024)
    lowkey ptr != null {
        vibez.spill("Successfully allocated 1024 bytes")
        free(ptr)
        vibez.spill("Memory freed")
    } kapish {
        vibez.spill("Memory allocation failed")
    }
}

fr fr =================================
fr fr INTEGRATION TESTS
fr fr =================================

slay test_combined_features() {
    vibez.spill("=== Combined Feature Tests ===")
    
    fr fr Combine generics with reflection
    struct GenericContainer<T> {
        spill data T
        spill metadata tea
    }
    
    fr fr Should infer T from constructor
    sus container = GenericContainer{data: 42, metadata: "number"}
    
    fr fr Use reflection on generic type
    sus fields = GenericContainer.fields  fr fr This should work with generic
    vibez.spill("Generic container has " + len(fields) + " fields")
    
    fr fr Combine macros with extern functions
    @macro
    slay safe_strlen(str) {
        lowkey str == null {
            damn 0
        } kapish {
            damn strlen(str)
        }
    }
    
    sus safe_len = safe_strlen("test string")
    vibez.spill("Safe string length: " + safe_len)
    
    fr fr Pattern match with generic enums
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }
    
    sus result Result<normie, tea> = Result::Ok(42)
    
    sick (result) {
        when Ok(value) -> vibez.spill("Success: " + value)
        when Err(error) -> vibez.spill("Error: " + error)
        fr fr This should be exhaustive (both variants covered)
    }
}

fr fr =================================
fr fr MAIN TEST RUNNER
fr fr =================================

slay language_features_test_suite() lit {
    vibez.spill("CURSED Language Features Test Suite")
    vibez.spill("=====================================")
    
    test_start("Exhaustive Pattern Checking")
    test_exhaustive_patterns()
    
    test_start("Generic Type Inference")
    test_type_inference()
    
    test_start("Compile-time Reflection")
    test_compile_time_reflection()
    
    test_start("Macro Hygiene")
    test_macro_hygiene()
    
    test_start("Extern Function ABI")
    test_extern_functions()
    
    test_start("Combined Features")
    test_combined_features()
    
    print_test_summary()
    damn based
}

fr fr Run the test suite
sus test_result lit = language_features_test_suite()
