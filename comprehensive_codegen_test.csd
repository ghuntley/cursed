vibe comprehensive_codegen_test

# Test comprehensive CURSED codegen features
# This program tests all the implemented language constructs

# Struct definition
squad Point {
    spill x normie
    spill y normie
}

# Interface definition
collab Drawable {
    slay draw() normie
}

# Implementation block
Point vibes Drawable {
    slay draw() normie {
        damn self.x + self.y
    }
}

# Constants
facts PI = 3.14159
facts MAX_SIZE = 100

# Function with pattern matching
slay process_value(val normie) normie {
    vibe_check val {
        mood 0:
            damn 42
        mood 1, 2, 3:
            damn val * 2
        basic:
            damn val + 1
    }
}

# Lambda expression test
slay test_lambda() normie {
    sus add_func = (x, y) => x + y
    damn add_func(5, 3)
}

# Array and tuple operations
slay test_collections() {
    # Array literal
    sus numbers = [1, 2, 3, 4, 5]
    
    # Tuple literal
    sus point_tuple = (10, 20)
    
    # Map literal
    sus colors = {"red": 1, "green": 2, "blue": 3}
    
    # Array access
    sus first_num = numbers[0]
    
    # Member access
    sus p = Point{x: 10, y: 20}
    sus px = p.x
    
    vibez.spill("Array and tuple operations complete")
}

# Goroutine test
slay background_work() {
    vibez.spill("Background work running")
}

slay test_concurrency() {
    # Start goroutine
    stan background_work()
    
    vibez.spill("Main thread continues")
}

# Defer statement test
slay test_defer() {
    vibez.spill("Function start")
    
    defer {
        vibez.spill("Deferred cleanup")
    }
    
    vibez.spill("Function end")
}

# Main function
slay main() normie {
    vibez.spill("=== Comprehensive CURSED Codegen Test ===")
    
    # Test basic operations
    sus result = process_value(2)
    vibez.spill("Pattern match result: ")
    
    # Test lambda
    sus lambda_result = test_lambda()
    vibez.spill("Lambda result: ")
    
    # Test collections
    test_collections()
    
    # Test concurrency
    test_concurrency()
    
    # Test defer
    test_defer()
    
    vibez.spill("=== Test Complete ===")
    damn 0
}
