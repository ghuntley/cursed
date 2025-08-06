# Comprehensive JIT Engine Test Suite
# Tests all major CURSED language features with the JIT execution engine

# Basic arithmetic and types
sus x normie = 42
sus y meal = 3.14
sus name tea = "Hello JIT"
sus flag lit = based

vibez.spill("Testing basic types:")
vibez.spill("Integer:", x)
vibez.spill("Float:", y)
vibez.spill("String:", name)
vibez.spill("Boolean:", flag)

# Function definitions with parameters
slay add_numbers(a normie, b normie) normie {
    damn a + b
}

slay greet_user(user_name tea) tea {
    damn "Hello, " + user_name + "!"
}

# Test function calls
sus sum normie = add_numbers(10, 20)
sus greeting tea = greet_user("JIT Engine")

vibez.spill("Function results:")
vibez.spill("Sum:", sum)
vibez.spill("Greeting:", greeting)

# Struct definition and usage
squad Point {
    spill x normie
    spill y normie
}

# Create struct instance
sus p Point = Point{ x: 5, y: 10 }
vibez.spill("Point struct:")
vibez.spill("x =", p.x)
vibez.spill("y =", p.y)

# Interface definition
collab Drawable {
    slay draw()
}

# Array and tuple operations
sus numbers = [1, 2, 3, 4, 5]
sus coordinates = (p.x, p.y, 0)

vibez.spill("Array first element:", numbers[0])
vibez.spill("Tuple access:", coordinates.0, coordinates.1)

# Type assertions
sus int_val normie = y.(normie)
sus str_val tea = x.(tea)

vibez.spill("Type conversions:")
vibez.spill("Float to int:", int_val)
vibez.spill("Int to string:", str_val)

# Lambda expressions
sus multiplier = lambda(x normie) normie { damn x * 2 }
sus doubled normie = multiplier(21)

vibez.spill("Lambda result:", doubled)

# Error handling
yikes CustomError {
    spill message tea
    spill code normie
}

sus error_val = CustomError{ message: "Test error", code: 404 }
vibez.spill("Error:", error_val.message, "Code:", error_val.code)

# Member access on different types
vibez.spill("Point type:", p.type)
vibez.spill("Point field count:", p.fieldCount)

# String operations with concatenation
sus long_text tea = "JIT " + "Engine " + "Test " + "Complete"
vibez.spill("Concatenated string:", long_text)

# Conditional expressions
bestie (sum > 25) {
    vibez.spill("Sum is greater than 25")
} else {
    vibez.spill("Sum is 25 or less")
}

# Loop for performance testing
vibez.spill("Performance test - counting to 1000:")
sus counter normie = 0
bestie (counter < 1000) {
    counter = counter + 1
}
vibez.spill("Final counter:", counter)

vibez.spill("JIT Engine comprehensive test completed!")
