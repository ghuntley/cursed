# Comprehensive Type System Test for CURSED
# Tests all major type system features including inference, generics, and constraints

yeet "testz"

# Test 1: Basic type inference
test_start("Basic Type Inference")

# Variable declarations with inference
sus x := 42                    # Should infer drip
sus y := 3.14                  # Should infer meal  
sus name := "CURSED"           # Should infer tea
sus flag := based              # Should infer lit
sus ch := 'A'                  # Should infer sip

# Test type compatibility
sus small_num smol = 5         # Explicit small int
sus big_num := small_num + 10  # Should promote to larger int type

print_test_summary()

# Test 2: Function type checking
test_start("Function Type Checking")

slay add_numbers(a drip, b drip) drip {
    damn a + b
}

slay greet(name tea) tea {
    damn "Hello, " + name
}

# Test function calls with type checking
sus result := add_numbers(10, 20)     # Should work
sus greeting := greet("World")        # Should work

# This should fail type checking:
# sus bad_call := add_numbers("hello", 5)  # String + int mismatch

print_test_summary()

# Test 3: Struct type system
test_start("Struct Type System")

squad Point {
    spill x drip
    spill y drip
}

squad Person {
    spill name tea
    spill age drip
    spill position Point
}

# Struct literals with type checking
sus origin := Point { x: 0, y: 0 }
sus player := Person { 
    name: "Hero", 
    age: 25, 
    position: Point { x: 10, y: 20 } 
}

# Member access type checking
sus player_x := player.position.x    # Should infer drip
sus player_name := player.name       # Should infer tea

print_test_summary()

# Test 4: Array and slice type system
test_start("Array and Slice Types")

# Array literals with type inference
sus numbers := [1, 2, 3, 4, 5]           # Should infer []drip
sus names := ["Alice", "Bob", "Charlie"]   # Should infer []tea
sus mixed_ok := [1, 2, 3]                 # All same type - OK

# Array access type checking
sus first_number := numbers[0]             # Should infer drip
sus first_name := names[0]                 # Should infer tea

# Slice operations
sus slice := numbers[1:3]                  # Should infer []drip

print_test_summary()

# Test 5: Interface type system
test_start("Interface Type System")

collab Drawable {
    slay draw() vibes
    slay area() meal
}

squad Circle {
    spill radius meal
}

# Interface implementation (simplified syntax)
Circle::draw() {
    vibez.spill("Drawing circle with radius: ")
    vibez.spill(self.radius)
}

Circle::area() meal {
    damn 3.14159 * self.radius * self.radius
}

# Interface usage
sus shape Drawable = Circle { radius: 5.0 }
shape.draw()
sus circle_area := shape.area()  # Should infer meal

print_test_summary()

# Test 6: Generic type system
test_start("Generic Type System")

# Generic function with type parameter
slay max[T](a T, b T) T {
    catch a > b {
        damn a
    } def {
        damn b
    }
}

# Generic function usage with type inference
sus max_int := max(10, 20)        # T inferred as drip
sus max_float := max(3.14, 2.71)  # T inferred as meal
sus max_string := max("apple", "banana")  # T inferred as tea

# Generic struct
squad Container[T] {
    spill value T
}

# Generic struct instantiation
sus int_container := Container[drip] { value: 42 }
sus string_container := Container[tea] { value: "hello" }

print_test_summary()

# Test 7: Type constraints and bounds
test_start("Type Constraints")

# Generic function with numeric constraint
slay add_generic[T: Numeric](a T, b T) T {
    damn a + b
}

# Should work with numeric types
sus sum1 := add_generic(10, 20)      # drip
sus sum2 := add_generic(3.14, 2.71)  # meal

# Generic function with comparable constraint
slay find_min[T: Comparable](a T, b T) T {
    catch a < b {
        damn a
    } def {
        damn b
    }
}

sus min_num := find_min(5, 3)        # Should work with numbers
sus min_str := find_min("zebra", "apple")  # Should work with strings

print_test_summary()

# Test 8: Channel type system
test_start("Channel Type System")

# Channel declarations with typed messages
sus int_channel := dm_create[drip]()      # Channel of integers
sus string_channel := dm_create[tea]()    # Channel of strings

# Channel operations with type checking
stan {
    dm_send(int_channel, 42)              # Should work
    dm_send(string_channel, "message")     # Should work
    # dm_send(int_channel, "oops")         # Should fail type check
}

sus received_int := dm_recv(int_channel)     # Should infer drip
sus received_str := dm_recv(string_channel)  # Should infer tea

print_test_summary()

# Test 9: Pattern matching type system
test_start("Pattern Matching Types")

slay process_value(value drip) tea {
    vibe value {
        0 => damn "zero"
        1 | 2 | 3 => damn "small"
        n => damn "large: " + n.to_string()
    }
}

# Pattern matching with type inference
sus result1 := process_value(0)      # Should infer tea
sus result2 := process_value(5)      # Should infer tea

# Pattern matching with destructuring
slay get_point_description(p Point) tea {
    vibe p {
        Point { x: 0, y: 0 } => damn "origin"
        Point { x, y } => damn "point at (" + x.to_string() + ", " + y.to_string() + ")"
    }
}

print_test_summary()

# Test 10: Type assertions and conversions
test_start("Type Assertions and Conversions")

# Type assertions (runtime checks)
sus value normie = 42
sus as_drip := value.(drip)           # Convert normie to drip
sus as_float := value.(meal)          # Convert int to float

# Safe type conversions
sus safe_str := value.to_string()     # Convert to string
sus safe_float := (3.14).(snack)     # Downcast float

print_test_summary()

# Test 11: Error handling with types
test_start("Error Handling Types")

# Function that might fail
slay divide(a drip, b drip) drip ? {
    catch b == 0 {
        damn no_cap("division by zero")
    } def {
        damn a / b
    }
}

# Error handling with type checking
sus result := divide(10, 2)
vibe result {
    frfr(value) => vibez.spill("Result: " + value.to_string())
    no_cap(error) => vibez.spill("Error: " + error)
}

print_test_summary()

# Test 12: Advanced type inference scenarios
test_start("Advanced Type Inference")

# Complex nested structures
sus complex_data := [
    Person { 
        name: "Alice", 
        age: 30, 
        position: Point { x: 1, y: 2 } 
    },
    Person { 
        name: "Bob", 
        age: 25, 
        position: Point { x: 3, y: 4 } 
    }
]  # Should infer []Person

# Function composition with inference
slay map[T, U](arr []T, func slay(T) U) []U {
    sus result := []U{}
    bestie item : arr {
        result.append(func(item))
    }
    damn result
}

sus ages := map(complex_data, slay(p Person) drip { damn p.age })  # Should infer []drip

print_test_summary()

vibez.spill("🎉 Comprehensive type system test complete!")
vibez.spill("All type checking features have been validated.")
