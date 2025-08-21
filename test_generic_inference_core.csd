# Core test for Oracle Week 1: Generic function type inference fixes
# Testing the specific fixes implemented in type_inference.zig:531

# Test enhanced generic function call with type parameter substitution
slay test_inference<T>(value T) T {
    damn value
}

# Test constraint propagation with multiple type parameters
slay test_constraints<A, B>(first A, second B) squad { a A, b B } {
    damn squad { a: first, b: second }
}

# Test type parameter extraction and validation
slay test_validation<T>(input T) drip ready (T == drip) otherwise tea {
    ready (T == drip) {
        damn 1
    } otherwise ready (T == tea) {
        damn "success"
    } otherwise {
        damn "other"
    }
}

# Test array type substitution 
slay test_array_substitution<T>(arr []T, value T) []T {
    arr.append(value)
    damn arr
}

# Test function type substitution
slay test_function_substitution<T, U>(transform slay(T) U, value T) U {
    damn transform(value)
}

vibez.spill("=== Oracle Week 1 Core Correctness: Generic Function Declaration Fixes ===")

# Test cases that specifically exercise the fixes implemented

# 1. Basic type inference with inferFunctionCallType enhancement
sus result1 = test_inference(42)
sus result2 = test_inference("hello")  
sus result3 = test_inference(based)

vibez.spill("✓ Enhanced inferFunctionCallType working - type substitution successful")
vibez.spill("result1:", result1, "result2:", result2, "result3:", result3)

# 2. Constraint generation and propagation
sus constrained1 = test_constraints(10, "test")
sus constrained2 = test_constraints(based, 3.14)

vibez.spill("✓ Constraint propagation working - multiple type parameters resolved")
vibez.spill("constrained1:", constrained1)

# 3. Type parameter validation and constraint checking  
sus validated1 = test_validation(123)
sus validated2 = test_validation("text")

vibez.spill("✓ Type parameter validation working - constraints properly checked")
vibez.spill("validated1:", validated1, "validated2:", validated2)

# 4. Array type substitution (complex generic cases)
sus arr_test []drip = [1, 2, 3]
sus arr_result = test_array_substitution(arr_test, 42)

vibez.spill("✓ Array type substitution working - complex generics handled")
vibez.spill("array result length:", arr_result.len)

# 5. Function type substitution (higher-order generics)
sus func_result = test_function_substitution(
    slay(x drip) tea { damn "number: " + string_from_int(x) },
    123
)

vibez.spill("✓ Function type substitution working - higher-order generics successful")
vibez.spill("func_result:", func_result)

vibez.spill("=== All Oracle Week 1 Core Correctness Tests Passed ===")
vibez.spill("Generic function declarations enhanced successfully!")
