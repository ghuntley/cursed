# Advanced JIT Features Test
# Tests complex language constructs and edge cases

yeet "testz"

# Advanced struct with nested types
squad Vector3D {
    spill x meal
    spill y meal  
    spill z meal
    spill magnitude meal
}

# Interface with multiple methods
collab Transformable {
    slay translate(dx meal, dy meal, dz meal)
    slay scale(factor meal)
    slay rotate(angle meal)
}

# Complex function with multiple parameters and return types
slay create_vector(x meal, y meal, z meal) Vector3D {
    sus mag meal = sqrt(x*x + y*y + z*z)
    damn Vector3D{ x: x, y: y, z: z, magnitude: mag }
}

# Function with struct parameter
slay vector_info(v Vector3D) tea {
    damn "Vector(" + v.x.(tea) + ", " + v.y.(tea) + ", " + v.z.(tea) + ") mag=" + v.magnitude.(tea)
}

# Test advanced features
sus vec Vector3D = create_vector(1.0, 2.0, 3.0)
vibez.spill("Created vector:", vector_info(vec))

# Array of structs
sus vectors = [
    Vector3D{ x: 1.0, y: 0.0, z: 0.0, magnitude: 1.0 },
    Vector3D{ x: 0.0, y: 1.0, z: 0.0, magnitude: 1.0 },
    Vector3D{ x: 0.0, y: 0.0, z: 1.0, magnitude: 1.0 }
]

# Iterate through array
sus i normie = 0
bestie (i < 3) {
    vibez.spill("Vector", i, ":", vector_info(vectors[i]))
    i = i + 1
}

# Complex tuple operations
sus complex_tuple = (vec, "named_vector", 42, based)
vibez.spill("Complex tuple:")
vibez.spill("- Vector:", vector_info(complex_tuple.0))
vibez.spill("- Name:", complex_tuple.1)
vibez.spill("- ID:", complex_tuple.2)
vibez.spill("- Active:", complex_tuple.3)

# Nested member access
sus nested_access = complex_tuple.0.x + complex_tuple.0.y
vibez.spill("Nested access result:", nested_access)

# Type conversion chain
sus original meal = 3.14159
sus as_int normie = original.(normie)
sus back_to_float meal = as_int.(meal)
sus as_string tea = back_to_float.(tea)

vibez.spill("Conversion chain:")
vibez.spill("Original:", original)
vibez.spill("As int:", as_int)
vibez.spill("Back to float:", back_to_float)
vibez.spill("As string:", as_string)

# Error handling with context
yikes MathError {
    spill operation tea
    spill operand1 meal
    spill operand2 meal
    spill message tea
}

slay safe_divide(a meal, b meal) meal {
    bestie (b == 0.0) {
        sus error = MathError{
            operation: "division",
            operand1: a,
            operand2: b,
            message: "Division by zero"
        }
        vibez.spill("Error:", error.message)
        damn 0.0
    }
    damn a / b
}

sus division_result meal = safe_divide(10.0, 3.0)
sus error_result meal = safe_divide(10.0, 0.0)

vibez.spill("Division results:")
vibez.spill("10.0 / 3.0 =", division_result)
vibez.spill("10.0 / 0.0 =", error_result)

# Lambda with closures
sus base_value normie = 100
sus add_to_base = lambda(x normie) normie {
    damn base_value + x
}

sus result1 normie = add_to_base(50)
sus result2 normie = add_to_base(25)

vibez.spill("Lambda with closure:")
vibez.spill("Base value:", base_value)
vibez.spill("add_to_base(50):", result1)
vibez.spill("add_to_base(25):", result2)

# Interface implementation simulation
slay implement_drawable(obj Vector3D) {
    vibez.spill("Drawing vector at (", obj.x, ",", obj.y, ",", obj.z, ")")
}

# Simulate interface usage
implement_drawable(vec)

# Performance test with complex operations
vibez.spill("Performance test - complex calculations:")
sus iterations normie = 100
sus performance_counter normie = 0

bestie (performance_counter < iterations) {
    sus temp_vec Vector3D = create_vector(
        performance_counter.(meal),
        (performance_counter * 2).(meal),
        (performance_counter * 3).(meal)
    )
    performance_counter = performance_counter + 1
}

vibez.spill("Completed", iterations, "complex operations")

test_start("JIT Advanced Features Test")
assert_true(vec.x == 1.0)
assert_true(division_result > 3.0)
assert_true(result1 == 150)
assert_eq_int(performance_counter, iterations)
print_test_summary()

vibez.spill("Advanced JIT features test completed!")
