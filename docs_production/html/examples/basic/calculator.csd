yeet "stringz"
yeet "testz"

# Simple Calculator Example
# Demonstrates basic arithmetic operations, error handling, and user input

struct Calculator {
    last_result drip
    history []tea
}

slay new_calculator() Calculator {
    damn Calculator{
        last_result: 0.0,
        history: make([]tea, 0),
    }
}

slay (calc *Calculator) add(a drip, b drip) drip {
    sus result drip = a + b
    calc.last_result = result
    calc.add_to_history("add", a, b, result)
    damn result
}

slay (calc *Calculator) subtract(a drip, b drip) drip {
    sus result drip = a - b
    calc.last_result = result
    calc.add_to_history("subtract", a, b, result)
    damn result
}

slay (calc *Calculator) multiply(a drip, b drip) drip {
    sus result drip = a * b
    calc.last_result = result
    calc.add_to_history("multiply", a, b, result)
    damn result
}

slay (calc *Calculator) divide(a drip, b drip) drip {
    lowkey b == 0.0 {
        yikes "Division by zero"
    }
    
    sus result drip = a / b
    calc.last_result = result
    calc.add_to_history("divide", a, b, result)
    damn result
}

slay (calc *Calculator) power(base drip, exponent drip) drip {
    sus result drip = base
    sus abs_exp normie = exponent.(normie)
    
    lowkey abs_exp < 0 {
        abs_exp = -abs_exp
    }
    
    bestie i := 1; i < abs_exp; i++ {
        result *= base
    }
    
    lowkey exponent < 0 {
        result = 1.0 / result
    }
    
    calc.last_result = result
    calc.add_to_history("power", base, exponent, result)
    damn result
}

slay (calc *Calculator) sqrt(value drip) drip {
    lowkey value < 0 {
        yikes "Cannot compute square root of negative number"
    }
    
    # Simple Newton's method approximation
    sus result drip = value
    bestie i := 0; i < 10; i++ {
        result = (result + value / result) / 2.0
    }
    
    calc.last_result = result
    calc.add_to_history("sqrt", value, 0.0, result)
    damn result
}

slay (calc *Calculator) add_to_history(operation tea, a drip, b drip, result drip) {
    sus entry tea = operation + "(" + a.(tea) + ", " + b.(tea) + ") = " + result.(tea)
    calc.history = append(calc.history, entry)
}

slay (calc *Calculator) print_history() {
    vibez.spill("Calculator History:")
    vibez.spill("==================")
    
    lowkey len(calc.history) == 0 {
        vibez.spill("No operations performed yet")
        damn
    }
    
    bestie i, entry <- calc.history {
        vibez.spill((i + 1).(tea) + ". " + entry)
    }
}

slay (calc *Calculator) clear_history() {
    calc.history = make([]tea, 0)
    vibez.spill("History cleared")
}

slay (calc *Calculator) get_last_result() drip {
    damn calc.last_result
}

# Safe wrapper functions with error handling
slay safe_add(a drip, b drip) (drip, tea) {
    defer {
        lowkey err := shook(); err != cringe {
            damn 0.0, "Addition failed: " + err.(tea)
        }
    }
    
    sus calc Calculator = new_calculator()
    sus result drip = calc.add(a, b)
    damn result, ""
}

slay safe_divide(a drip, b drip) (drip, tea) {
    defer {
        lowkey err := shook(); err != cringe {
            damn 0.0, "Division failed: " + err.(tea)
        }
    }
    
    sus calc Calculator = new_calculator()
    sus result drip = calc.divide(a, b)
    damn result, ""
}

# Interactive calculator function
slay interactive_calculator() {
    vibez.spill("CURSED Calculator")
    vibez.spill("================")
    vibez.spill("Commands: add, subtract, multiply, divide, power, sqrt, history, clear, quit")
    
    sus calc Calculator = new_calculator()
    
    bestie {
        vibez.spill("\nEnter command: ")
        # In a real implementation, this would read from stdin
        # For this example, we'll demonstrate with predefined operations
        
        # Simulate user input
        demo_calculator_operations(calc)
        ghosted
    }
}

slay demo_calculator_operations(calc Calculator) {
    vibez.spill("Demo: Performing calculator operations")
    
    # Basic arithmetic
    sus result1 drip = calc.add(10.0, 5.0)
    vibez.spill("10 + 5 = " + result1.(tea))
    
    sus result2 drip = calc.subtract(20.0, 8.0)
    vibez.spill("20 - 8 = " + result2.(tea))
    
    sus result3 drip = calc.multiply(6.0, 7.0)
    vibez.spill("6 * 7 = " + result3.(tea))
    
    sus result4 drip = calc.divide(100.0, 4.0)
    vibez.spill("100 / 4 = " + result4.(tea))
    
    sus result5 drip = calc.power(2.0, 3.0)
    vibez.spill("2^3 = " + result5.(tea))
    
    sus result6 drip = calc.sqrt(16.0)
    vibez.spill("√16 = " + result6.(tea))
    
    # Error handling demo
    vibez.spill("\nError handling demo:")
    
    sus safe_result, error = safe_divide(10.0, 0.0)
    lowkey error != "" {
        vibez.spill("Error: " + error)
    } highkey {
        vibez.spill("Result: " + safe_result.(tea))
    }
    
    # Print history
    vibez.spill("")
    calc.print_history()
}

# Unit tests for calculator
slay test_calculator() {
    test_start("Calculator Tests")
    
    sus calc Calculator = new_calculator()
    
    # Test addition
    sus result drip = calc.add(5.0, 3.0)
    assert_eq_float(result, 8.0, 0.001)
    
    # Test subtraction
    result = calc.subtract(10.0, 4.0)
    assert_eq_float(result, 6.0, 0.001)
    
    # Test multiplication
    result = calc.multiply(6.0, 7.0)
    assert_eq_float(result, 42.0, 0.001)
    
    # Test division
    result = calc.divide(20.0, 4.0)
    assert_eq_float(result, 5.0, 0.001)
    
    # Test power
    result = calc.power(2.0, 3.0)
    assert_eq_float(result, 8.0, 0.001)
    
    # Test square root
    result = calc.sqrt(25.0)
    assert_eq_float(result, 5.0, 0.001)
    
    # Test history
    assert_true(len(calc.history) > 0)
    
    # Test last result
    assert_eq_float(calc.get_last_result(), 5.0, 0.001)
    
    print_test_summary()
}

# Helper function for float comparison
slay assert_eq_float(actual drip, expected drip, tolerance drip) {
    sus diff drip = actual - expected
    lowkey diff < 0 {
        diff = -diff
    }
    assert_true(diff < tolerance)
}

# Main execution
slay main_character() {
    vibez.spill("CURSED Calculator Example")
    vibez.spill("=========================")
    
    # Run tests
    test_calculator()
    
    # Run interactive demo
    interactive_calculator()
}

# Run the example
main()
