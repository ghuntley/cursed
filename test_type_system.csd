// Type System Verification Test

// Basic type checking
sus int_var drip = 42
sus str_var tea = "hello"
sus bool_var lit = based

spill("Type System Tests:")
spill("Integer variable:", int_var)
spill("String variable:", str_var)
spill("Boolean variable:", bool_var)

// Function with typed parameters
slay typed_function(param1 drip, param2 tea) tea {
    spill("Function parameter types - drip:", param1, "tea:", param2)
    damn param2
}

sus function_result tea = typed_function(99, "world")
spill("Typed function result:", function_result)

// Array types
sus int_array []drip = [1, 2, 3, 4, 5]
sus str_array []tea = ["a", "b", "c"]

spill("Array types:")
spill("Integer array length:", len(int_array))
spill("String array length:", len(str_array))

// Type inference test
sus inferred_int = 100  // Should infer drip
sus inferred_str = "inferred"  // Should infer tea
sus inferred_bool = based  // Should infer lit

spill("Type inference:")
spill("Inferred int:", inferred_int)
spill("Inferred string:", inferred_str)  
spill("Inferred bool:", inferred_bool)

// Mixed arithmetic (should handle type coercion properly)
sus mixed_calc drip = int_var + 8
spill("Mixed arithmetic result:", mixed_calc)
