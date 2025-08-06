# Advanced Type System Features Test
# Demonstrates comprehensive type inference, generics, and constraints

yeet "testz"

test_start("Advanced Type Features")

# Test 1: Complex type inference with nested structures
squad Point3D {
    spill x meal
    spill y meal
    spill z meal
}

squad Line {
    spill start Point3D
    spill end Point3D
}

# Complex nested initialization with type inference
sus complex_line := Line {
    start: Point3D { x: 0.0, y: 0.0, z: 0.0 },
    end: Point3D { x: 1.0, y: 1.0, z: 1.0 }
}

# Member access through nested structures (type inference)
sus distance_x := complex_line.end.x - complex_line.start.x
assert_true(distance_x > 0.5)

print_test_summary()

# Test 2: Generic containers with type constraints
test_start("Generic Type Constraints")

# Generic container (simplified syntax)
squad Container[T] {
    spill value T
    spill count drip
}

# Generic function with numeric constraint
slay calculate_average[T: Numeric](values []T) T {
    sus sum := values[0]  # Start with first element
    bestie i : 1..values.length {
        sum = sum + values[i]
    }
    damn sum / values.length
}

# Test with different numeric types
sus int_values := [10, 20, 30, 40, 50]
sus avg_int := calculate_average(int_values)  # T inferred as drip
assert_true(avg_int >= 25)

sus float_values := [1.5, 2.5, 3.5, 4.5]
sus avg_float := calculate_average(float_values)  # T inferred as meal
assert_true(avg_float > 2.0)

print_test_summary()

# Test 3: Interface implementation and method dispatch
test_start("Interface Implementation")

collab Drawable {
    slay draw() vibes
    slay get_area() meal
}

squad Rectangle {
    spill width meal
    spill height meal
}

# Implement Drawable for Rectangle
Rectangle::draw() {
    vibez.spill("Drawing rectangle")
}

Rectangle::get_area() meal {
    damn self.width * self.height
}

squad Circle {
    spill radius meal
}

# Implement Drawable for Circle
Circle::draw() {
    vibez.spill("Drawing circle")
}

Circle::get_area() meal {
    damn 3.14159 * self.radius * self.radius
}

# Use interface polymorphism
sus shapes := [
    Rectangle { width: 10.0, height: 5.0 },
    Circle { radius: 3.0 }
]

sus total_area := 0.0
bestie shape : shapes {
    shape.draw()
    total_area = total_area + shape.get_area()
}

assert_true(total_area > 70.0)  # Rectangle(50) + Circle(~28) > 70

print_test_summary()

# Test 4: Type assertions and safe conversions
test_start("Type Assertions")

# Type assertion examples
sus some_value drip = 42
sus as_float := some_value.(meal)      # Safe numeric conversion
sus as_string := some_value.to_string() # Method call conversion

assert_true(as_float > 40.0)
assert_eq_string(as_string, "42")

# Optional/nullable type handling (simplified)
slay safe_divide(a drip, b drip) meal? {
    catch b == 0 {
        damn no_cap("Division by zero")
    } def {
        damn frfr(a.(meal) / b.(meal))
    }
}

sus safe_result := safe_divide(10, 2)
vibe safe_result {
    frfr(value) => assert_true(value == 5.0)
    no_cap(error) => assert_true(cringe)  # Should not reach here
}

print_test_summary()

# Test 5: Pattern matching with type inference
test_start("Pattern Matching Types")

slay classify_number(n drip) tea {
    vibe n {
        0 => damn "zero"
        1..10 => damn "small"
        11..100 => damn "medium"
        _ => damn "large"
    }
}

sus classification := classify_number(25)
assert_eq_string(classification, "medium")

# Pattern matching with destructuring
slay point_quadrant(p Point3D) tea {
    vibe p {
        Point3D { x, y, z } when x > 0.0 and y > 0.0 and z > 0.0 => damn "positive"
        Point3D { x, y, z } when x < 0.0 or y < 0.0 or z < 0.0 => damn "negative"
        _ => damn "origin"
    }
}

sus quadrant := point_quadrant(Point3D { x: 1.0, y: 2.0, z: 3.0 })
assert_eq_string(quadrant, "positive")

print_test_summary()

# Test 6: Higher-order functions with type inference
test_start("Higher-Order Functions")

# Map function with type inference
slay map[T, U](values []T, transform slay(T) U) []U {
    sus result := []U{}
    bestie value : values {
        result.push(transform(value))
    }
    damn result
}

# Filter function with type inference
slay filter[T](values []T, predicate slay(T) lit) []T {
    sus result := []T{}
    bestie value : values {
        catch predicate(value) {
            result.push(value)
        }
    }
    damn result
}

# Test higher-order functions
sus numbers := [1, 2, 3, 4, 5]
sus doubled := map(numbers, slay(x drip) drip { damn x * 2 })
sus evens := filter(numbers, slay(x drip) lit { damn x % 2 == 0 })

assert_eq_int(doubled[0], 2)
assert_eq_int(evens[0], 2)

print_test_summary()

vibez.spill("🚀 Advanced type system features validated!")
vibez.spill("✨ Type inference, generics, constraints, and polymorphism all working!")
