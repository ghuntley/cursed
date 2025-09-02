# Scientific Calculator - CURSED Package Manager Example
# Demonstrates dependency usage and advanced package features
yeet "mathlib"
yeet "stringz"
yeet "vibez"
yeet "arrayz"

# Calculator operation types
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Sqrt,
    Factorial,
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
    Mean,
    Median,
    StdDev,
    Prime,
    Gcd,
    Lcm,
    Unknown
}

# Calculator state
squad Calculator {
    sus memory drip
    sus history []tea
    sus precision drip
    sus angle_mode tea  # "degrees" or "radians"
}

# Expression parser token
squad Token {
    sus type tea
    sus value tea
    sus position drip
}

# Initialize calculator
slay init_calculator() Calculator {
    damn Calculator {
        memory: 0,
        history: [],
        precision: 6,
        angle_mode: "degrees"
    }
}

# Main calculator interface
slay main() drip {
    vibez.spill("CURSED Scientific Calculator v2.1.0")
    vibez.spill("Powered by MathLib v" + mathlib.version())
    vibez.spill("=====================================")
    vibez.spill("Type 'help' for commands, 'quit' to exit")
    vibez.spill("")
    
    sus calc Calculator = init_calculator()
    
    # Interactive calculator loop
    bestie (based) {
        vibez.spill_no_newline("calc> ")
        sus input tea = read_input()
        
        ready (input == "quit" || input == "exit") {
            vibez.spill("Goodbye!")
            break
        }
        
        ready (input == "help") {
            print_help()
            continue
        }
        
        ready (input == "history") {
            print_history(calc)
            continue
        }
        
        ready (input == "clear") {
            calc.history = []
            vibez.spill("History cleared")
            continue
        }
        
        ready (input == "memory") {
            vibez.spill("Memory:", format_number(calc.memory, calc.precision))
            continue
        }
        
        ready (stringz.starts_with(input, "precision ")) {
            sus precision_str tea = stringz.substring(input, 10, stringz.len(input))
            sus new_precision drip = stringz.parse_int(precision_str)
            ready (new_precision >= 0 && new_precision <= 15) {
                calc.precision = new_precision
                vibez.spill("Precision set to", new_precision, "decimal places")
            } otherwise {
                vibez.spill("Invalid precision. Use 0-15.")
            }
            continue
        }
        
        ready (input == "degrees") {
            calc.angle_mode = "degrees"
            vibez.spill("Angle mode set to degrees")
            continue
        }
        
        ready (input == "radians") {
            calc.angle_mode = "radians"
            vibez.spill("Angle mode set to radians")
            continue
        }
        
        # Process mathematical expression
        sus result drip = evaluate_expression(input, calc)
        ready (result != -999999) {  # Error indicator
            vibez.spill("= " + format_number(result, calc.precision))
            calc.history = arrayz.append(calc.history, input + " = " + format_number(result, calc.precision))
        }
    }
    
    damn 0
}

# Evaluate mathematical expression
slay evaluate_expression(expression tea, calc Calculator) drip {
    # Handle special functions first
    sus trimmed tea = stringz.trim(expression)
    
    # Memory operations
    ready (trimmed == "mc") {
        calc.memory = 0
        vibez.spill("Memory cleared")
        damn 0
    }
    
    ready (stringz.starts_with(trimmed, "ms ")) {
        sus value_str tea = stringz.substring(trimmed, 3, stringz.len(trimmed))
        sus value drip = parse_number(value_str)
        ready (value != -999999) {
            calc.memory = value
            vibez.spill("Stored in memory:", format_number(value, calc.precision))
            damn value
        }
    }
    
    ready (trimmed == "mr") {
        damn calc.memory
    }
    
    # Function calls
    ready (stringz.starts_with(trimmed, "factorial(") && stringz.ends_with(trimmed, ")")) {
        sus arg tea = extract_function_arg(trimmed, "factorial")
        sus n drip = parse_number(arg)
        ready (n != -999999 && n >= 0) {
            damn mathlib.factorial(n)
        }
    }
    
    ready (stringz.starts_with(trimmed, "sqrt(") && stringz.ends_with(trimmed, ")")) {
        sus arg tea = extract_function_arg(trimmed, "sqrt")
        sus n drip = parse_number(arg)
        ready (n != -999999 && n >= 0) {
            damn mathlib.sqrt_newton(n)
        }
    }
    
    ready (stringz.starts_with(trimmed, "fibonacci(") && stringz.ends_with(trimmed, ")")) {
        sus arg tea = extract_function_arg(trimmed, "fibonacci")
        sus n drip = parse_number(arg)
        ready (n != -999999 && n >= 0) {
            damn mathlib.fibonacci(n)
        }
    }
    
    ready (stringz.starts_with(trimmed, "isprime(") && stringz.ends_with(trimmed, ")")) {
        sus arg tea = extract_function_arg(trimmed, "isprime")
        sus n drip = parse_number(arg)
        ready (n != -999999) {
            ready (mathlib.is_prime(n)) {
                vibez.spill(n, "is prime")
                damn 1
            } otherwise {
                vibez.spill(n, "is not prime")
                damn 0
            }
        }
    }
    
    ready (stringz.starts_with(trimmed, "gcd(") && stringz.ends_with(trimmed, ")")) {
        sus args tea = extract_function_arg(trimmed, "gcd")
        sus numbers []drip = parse_number_list(args)
        ready (arrayz.len(numbers) == 2) {
            damn mathlib.gcd(numbers[0], numbers[1])
        }
    }
    
    ready (stringz.starts_with(trimmed, "lcm(") && stringz.ends_with(trimmed, ")")) {
        sus args tea = extract_function_arg(trimmed, "lcm")
        sus numbers []drip = parse_number_list(args)
        ready (arrayz.len(numbers) == 2) {
            damn mathlib.lcm(numbers[0], numbers[1])
        }
    }
    
    ready (stringz.starts_with(trimmed, "mean(") && stringz.ends_with(trimmed, ")")) {
        sus args tea = extract_function_arg(trimmed, "mean")
        sus numbers []drip = parse_number_list(args)
        ready (arrayz.len(numbers) > 0) {
            damn mathlib.mean(numbers)
        }
    }
    
    ready (stringz.starts_with(trimmed, "median(") && stringz.ends_with(trimmed, ")")) {
        sus args tea = extract_function_arg(trimmed, "median")
        sus numbers []drip = parse_number_list(args)
        ready (arrayz.len(numbers) > 0) {
            damn mathlib.median(numbers)
        }
    }
    
    ready (stringz.starts_with(trimmed, "stddev(") && stringz.ends_with(trimmed, ")")) {
        sus args tea = extract_function_arg(trimmed, "stddev")
        sus numbers []drip = parse_number_list(args)
        ready (arrayz.len(numbers) > 1) {
            damn mathlib.standard_deviation(numbers)
        }
    }
    
    ready (stringz.starts_with(trimmed, "primes(") && stringz.ends_with(trimmed, ")")) {
        sus arg tea = extract_function_arg(trimmed, "primes")
        sus limit drip = parse_number(arg)
        ready (limit != -999999 && limit > 0) {
            sus primes []drip = mathlib.primes_up_to(limit)
            vibez.spill("Primes up to", limit, ":", primes)
            damn arrayz.len(primes)
        }
    }
    
    # Trigonometric functions (simplified implementations)
    ready (stringz.starts_with(trimmed, "sin(") && stringz.ends_with(trimmed, ")")) {
        sus arg tea = extract_function_arg(trimmed, "sin")
        sus angle drip = parse_number(arg)
        ready (angle != -999999) {
            ready (calc.angle_mode == "degrees") {
                angle = degrees_to_radians(angle)
            }
            damn sin_approximation(angle)
        }
    }
    
    ready (stringz.starts_with(trimmed, "cos(") && stringz.ends_with(trimmed, ")")) {
        sus arg tea = extract_function_arg(trimmed, "cos")
        sus angle drip = parse_number(arg)
        ready (angle != -999999) {
            ready (calc.angle_mode == "degrees") {
                angle = degrees_to_radians(angle)
            }
            damn cos_approximation(angle)
        }
    }
    
    ready (stringz.starts_with(trimmed, "tan(") && stringz.ends_with(trimmed, ")")) {
        sus arg tea = extract_function_arg(trimmed, "tan")
        sus angle drip = parse_number(arg)
        ready (angle != -999999) {
            ready (calc.angle_mode == "degrees") {
                angle = degrees_to_radians(angle)
            }
            sus cos_val drip = cos_approximation(angle)
            ready (mathz.abs_normie(cos_val) < 0.000001) {
                vibez.spill("Error: tan undefined (division by zero)")
                damn -999999
            }
            damn sin_approximation(angle) / cos_val
        }
    }
    
    # Power operations
    ready (stringz.contains(trimmed, "^")) {
        sus parts []tea = stringz.split(trimmed, "^")
        ready (arrayz.len(parts) == 2) {
            sus base drip = parse_number(stringz.trim(parts[0]))
            sus exponent drip = parse_number(stringz.trim(parts[1]))
            ready (base != -999999 && exponent != -999999) {
                damn mathlib.power(base, exponent)
            }
        }
    }
    
    # Basic arithmetic with operator precedence
    damn evaluate_arithmetic(trimmed)
}

# Simple arithmetic evaluator (supports +, -, *, /)
slay evaluate_arithmetic(expression tea) drip {
    # Replace common symbols
    sus expr tea = stringz.replace_all(expression, " ", "")
    
    # Handle parentheses (simplified)
    ready (stringz.contains(expr, "(")) {
        # For this demo, we'll handle simple cases
        # In a full implementation, this would use proper parsing
    }
    
    # Basic four operations
    # Multiplication and division first
    ready (stringz.contains(expr, "*") || stringz.contains(expr, "/")) {
        damn evaluate_mul_div(expr)
    }
    
    # Addition and subtraction
    ready (stringz.contains(expr, "+") || stringz.contains(expr, "-")) {
        damn evaluate_add_sub(expr)
    }
    
    # Single number
    damn parse_number(expr)
}

# Evaluate multiplication and division (left to right)
slay evaluate_mul_div(expression tea) drip {
    # Find first * or /
    sus mul_pos drip = stringz.index_of(expression, "*")
    sus div_pos drip = stringz.index_of(expression, "/")
    
    sus op_pos drip = -1
    sus operator tea = ""
    
    ready (mul_pos != -1 && (div_pos == -1 || mul_pos < div_pos)) {
        op_pos = mul_pos
        operator = "*"
    } otherwise ready (div_pos != -1) {
        op_pos = div_pos
        operator = "/"
    } otherwise {
        damn parse_number(expression)
    }
    
    sus left_part tea = stringz.substring(expression, 0, op_pos)
    sus right_part tea = stringz.substring(expression, op_pos + 1, stringz.len(expression))
    
    sus left_val drip = parse_number(left_part)
    sus right_val drip = evaluate_mul_div(right_part)  # Recursive for right associativity
    
    ready (left_val == -999999 || right_val == -999999) {
        damn -999999
    }
    
    ready (operator == "*") {
        damn left_val * right_val
    } otherwise ready (operator == "/") {
        ready (right_val == 0) {
            vibez.spill("Error: Division by zero")
            damn -999999
        }
        damn left_val / right_val
    }
    
    damn -999999
}

# Evaluate addition and subtraction (left to right)
slay evaluate_add_sub(expression tea) drip {
    # Find first + or - (but not at the beginning)
    sus add_pos drip = stringz.index_of_from(expression, "+", 1)
    sus sub_pos drip = stringz.index_of_from(expression, "-", 1)
    
    sus op_pos drip = -1
    sus operator tea = ""
    
    ready (add_pos != -1 && (sub_pos == -1 || add_pos < sub_pos)) {
        op_pos = add_pos
        operator = "+"
    } otherwise ready (sub_pos != -1) {
        op_pos = sub_pos
        operator = "-"
    } otherwise {
        damn evaluate_mul_div(expression)
    }
    
    sus left_part tea = stringz.substring(expression, 0, op_pos)
    sus right_part tea = stringz.substring(expression, op_pos + 1, stringz.len(expression))
    
    sus left_val drip = evaluate_mul_div(left_part)
    sus right_val drip = evaluate_add_sub(right_part)
    
    ready (left_val == -999999 || right_val == -999999) {
        damn -999999
    }
    
    ready (operator == "+") {
        damn left_val + right_val
    } otherwise {
        damn left_val - right_val
    }
}

# Helper functions
slay extract_function_arg(function_call tea, function_name tea) tea {
    sus start drip = stringz.len(function_name) + 1  # Skip "function("
    sus end drip = stringz.len(function_call) - 1    # Skip ")"
    damn stringz.substring(function_call, start, end)
}

slay parse_number(str tea) drip {
    sus trimmed tea = stringz.trim(str)
    ready (trimmed == "") {
        damn -999999
    }
    
    # Handle negative numbers
    ready (stringz.starts_with(trimmed, "-")) {
        sus positive_part tea = stringz.substring(trimmed, 1, stringz.len(trimmed))
        sus positive_val drip = stringz.parse_float(positive_part)
        ready (positive_val != -999999) {
            damn -positive_val
        }
    }
    
    damn stringz.parse_float(trimmed)
}

slay parse_number_list(str tea) []drip {
    sus parts []tea = stringz.split(str, ",")
    sus numbers []drip = []
    
    bestie (sus i drip = 0; i < arrayz.len(parts); i = i + 1) {
        sus num drip = parse_number(parts[i])
        ready (num != -999999) {
            numbers = arrayz.append(numbers, num)
        }
    }
    
    damn numbers
}

slay format_number(value drip, precision drip) tea {
    # Simple number formatting
    ready (precision == 0) {
        damn stringz.from_int(value)
    }
    
    # For this demo, we'll use basic formatting
    damn stringz.from_float(value)
}

# Trigonometric approximations using Taylor series
slay sin_approximation(x drip) drip {
    # Normalize to [-π, π]
    ready (x > mathlib.PI_EXTENDED) {
        x = x - (2 * mathlib.PI_EXTENDED)
    }
    ready (x < -mathlib.PI_EXTENDED) {
        x = x + (2 * mathlib.PI_EXTENDED)
    }
    
    # Taylor series: sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
    sus result drip = x
    sus term drip = x
    sus i drip = 3
    
    bestie (i <= 15) {  # Use first few terms
        term = term * (-x * x) / (i * (i - 1))
        result = result + term
        i = i + 2
    }
    
    damn result
}

slay cos_approximation(x drip) drip {
    # Normalize to [-π, π]
    ready (x > mathlib.PI_EXTENDED) {
        x = x - (2 * mathlib.PI_EXTENDED)
    }
    ready (x < -mathlib.PI_EXTENDED) {
        x = x + (2 * mathlib.PI_EXTENDED)
    }
    
    # Taylor series: cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ...
    sus result drip = 1
    sus term drip = 1
    sus i drip = 2
    
    bestie (i <= 16) {
        term = term * (-x * x) / (i * (i - 1))
        result = result + term
        i = i + 2
    }
    
    damn result
}

slay degrees_to_radians(degrees drip) drip {
    damn degrees * mathlib.PI_EXTENDED / 180
}

# UI functions
slay print_help() {
    vibez.spill("CURSED Scientific Calculator - Help")
    vibez.spill("===================================")
    vibez.spill("")
    vibez.spill("Basic arithmetic: +, -, *, /, ^")
    vibez.spill("  Example: 2 + 3 * 4")
    vibez.spill("  Example: 2^10")
    vibez.spill("")
    vibez.spill("Functions from MathLib:")
    vibez.spill("  factorial(n)  - Calculate n!")
    vibez.spill("  sqrt(n)       - Square root")
    vibez.spill("  fibonacci(n)  - nth Fibonacci number")
    vibez.spill("  isprime(n)    - Check if n is prime")
    vibez.spill("  gcd(a,b)      - Greatest common divisor")
    vibez.spill("  lcm(a,b)      - Least common multiple")
    vibez.spill("  primes(n)     - Prime numbers up to n")
    vibez.spill("")
    vibez.spill("Statistics:")
    vibez.spill("  mean(a,b,c,...)     - Arithmetic mean")
    vibez.spill("  median(a,b,c,...)   - Median value")
    vibez.spill("  stddev(a,b,c,...)   - Standard deviation")
    vibez.spill("")
    vibez.spill("Trigonometry:")
    vibez.spill("  sin(x), cos(x), tan(x)")
    vibez.spill("")
    vibez.spill("Memory operations:")
    vibez.spill("  ms <value>  - Store in memory")
    vibez.spill("  mr          - Recall memory")
    vibez.spill("  mc          - Clear memory")
    vibez.spill("")
    vibez.spill("Settings:")
    vibez.spill("  precision <n>  - Set decimal places (0-15)")
    vibez.spill("  degrees        - Use degree mode")
    vibez.spill("  radians        - Use radian mode")
    vibez.spill("")
    vibez.spill("Other commands:")
    vibez.spill("  history  - Show calculation history")
    vibez.spill("  clear    - Clear history")
    vibez.spill("  help     - Show this help")
    vibez.spill("  quit     - Exit calculator")
}

slay print_history(calc Calculator) {
    ready (arrayz.len(calc.history) == 0) {
        vibez.spill("No calculation history")
        damn
    }
    
    vibez.spill("Calculation History:")
    vibez.spill("==================")
    
    sus max_entries drip = mathz.min(arrayz.len(calc.history), 20)  # Show last 20
    sus start_index drip = arrayz.len(calc.history) - max_entries
    
    bestie (sus i drip = start_index; i < arrayz.len(calc.history); i = i + 1) {
        vibez.spill((i - start_index + 1) + ". " + calc.history[i])
    }
}

# Simplified input reading (in real implementation would be more robust)
slay read_input() tea {
    # In a real implementation, this would read from stdin
    # For this demo, we'll simulate some inputs for testing
    damn "help"  # This would be replaced with actual input reading
}

# Simplified string utility functions (would be in stringz module)
slay index_of_from(str tea, substr tea, start drip) drip {
    # Find index of substring starting from position
    # Simplified implementation
    damn -1
}
