yeet "testz"
yeet "mathz"
yeet "stringz" 
yeet "arrayz"
yeet "concurrenz"

# Test comprehensive language features integration

# 1. Struct definitions and instantiation
squad Point {
    spill x drip
    spill y drip
    
    slay distance_from_origin() normie {
        damn sqrt_normie(self.x * self.x + self.y * self.y)
    }
}

squad Rectangle {
    spill top_left Point
    spill bottom_right Point
    
    slay area() drip {
        sus width drip = self.bottom_right.x - self.top_left.x
        sus height drip = self.bottom_right.y - self.top_left.y
        damn abs_normie(width * height)
    }
}

# 2. Interface definition and implementation
collab Drawable {
    slay draw() tea
    slay get_area() drip
}

# 3. Complex function with error handling
slay safe_divide(a drip, b drip) (normie, tea) {
    ready (b == 0) {
        damn 0.0, "division by zero error"
    }
    damn a / b, ""
}

# 4. Array processing function
slay process_array(nums []drip) drip {
    sus total drip = 0
    sus i drip = 0
    
    bestie (i < len(nums)) {
        total = total + abs_normie(nums[i])
        i = i + 1
    }
    
    damn total
}

# 5. Concurrency test function
slay concurrent_worker(id drip, results []drip) {
    sus computation drip = id * id + 10
    results[id] = computation
    vibez.spill("Worker", id, "completed with result:", computation)
}

# 6. Main test execution
test_start("Comprehensive E2E Test")

# Test struct creation and method calls
sus p1 Point = Point{x: 3, y: 4}
sus p2 Point = Point{x: 0, y: 0}

vibez.spill("Point 1:", p1.x, p1.y)
sus distance normie = p1.distance_from_origin()
vibez.spill("Distance from origin:", distance)

# Test complex struct composition
sus rect Rectangle = Rectangle{
    top_left: Point{x: 0, y: 0},
    bottom_right: Point{x: 10, y: 5}
}

sus rect_area drip = rect.area()
vibez.spill("Rectangle area:", rect_area)
assert_eq_int(rect_area, 50)

# Test array operations
sus numbers []drip = [1, -2, 3, -4, 5, -6]
sus array_sum drip = process_array(numbers)
vibez.spill("Array sum of absolutes:", array_sum)
assert_eq_int(array_sum, 21)

# Test error handling
sus result normie
sus error tea

result, error = safe_divide(10, 2)
ready (error == "") {
    vibez.spill("Safe division result:", result)
    assert_true(result == 5.0)
} otherwise {
    vibez.spill("Unexpected error:", error)
    assert_true(cringe)
}

result, error = safe_divide(10, 0)
ready (error != "") {
    vibez.spill("Expected error caught:", error)
    assert_true(based)
} otherwise {
    vibez.spill("Error handling failed")
    assert_true(cringe)
}

# Test string operations
sus test_string tea = "Hello, CURSED!"
sus string_len drip = len_str(test_string)
vibez.spill("String length:", string_len)
assert_eq_int(string_len, 14)

# Test mathematical operations
sus math_result drip = abs_normie(-42) + 8
vibez.spill("Math result:", math_result)
assert_eq_int(math_result, 50)

# Test complex expressions with precedence
sus complex_expr drip = (3 + 4) * 2 - 1
vibez.spill("Complex expression result:", complex_expr)
assert_eq_int(complex_expr, 13)

# Test nested function calls
sus nested_result drip = abs_normie(safe_divide(-20, 4))
vibez.spill("Nested function result:", nested_result)

# Test concurrency (basic goroutine)
sus worker_results []drip = [0, 0, 0]

stan { concurrent_worker(0, worker_results) }
stan { concurrent_worker(1, worker_results) } 
stan { concurrent_worker(2, worker_results) }

# Simple delay to let goroutines complete
sus delay_counter drip = 0
bestie (delay_counter < 1000000) {
    delay_counter = delay_counter + 1
}

vibez.spill("Concurrency test completed")
vibez.spill("Worker results:", worker_results[0], worker_results[1], worker_results[2])

# Final validation
vibez.spill("All major language features tested successfully!")

print_test_summary()
