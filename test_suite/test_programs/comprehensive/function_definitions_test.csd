vibe main
yeet "vibez"

fr fr Function Definitions Test
fr fr Tests: Various function declaration patterns and return types
fr fr Expected: Functions compile and execute correctly in both modes

slay add_numbers(a, b) {
    damn a + b
}

slay multiply_three(x, y, z) {
    damn x * y * z
}

slay print_message(msg) {
    vibez.spill(msg)
}

slay calculate_average(a, b, c) {
    sus sum = a + b + c
    damn sum / 3
}

slay main_character() {
    vibez.spill("=== Function Definitions Test ===")
    
    fr fr Test basic function calls
    sus result1 = add_numbers(10, 5)
    vibez.spill("10 + 5 =")
    vibez.spill(result1)
    
    fr fr Test multiple parameter function
    sus result2 = multiply_three(2, 3, 4)
    vibez.spill("2 * 3 * 4 =")
    vibez.spill(result2)
    
    fr fr Test void function
    vibez.spill("Message from function:")
    print_message("Hello from function!")
    
    fr fr Test function with local variables
    sus avg = calculate_average(10, 20, 30)
    vibez.spill("Average of 10, 20, 30:")
    vibez.spill(avg)
    
    vibez.spill("=== Test Complete ===")
}
