# CURSED Cross-Backend Compatibility Test
# Tests that same code works across script, AST, and LLVM backends

yeet "vibez"
yeet "mathz"
yeet "arrayz"
yeet "stringz"

# ===== BASIC LANGUAGE FEATURES =====

# Variables and arithmetic (should work on all backends)
sus global_counter drip = 0

slay increment_counter() {
    global_counter = global_counter + 1
}

slay test_basic_arithmetic() {
    vibez.spill("🔢 Testing Basic Arithmetic")
    
    sus a drip = 10
    sus b drip = 20
    sus c drip = a + b
    sus d drip = c * 2
    sus e drip = d / 4
    
    vibez.spill("10 + 20 =", c)
    vibez.spill("30 * 2 =", d)  
    vibez.spill("60 / 4 =", e)
    
    # Test counter
    increment_counter()
    increment_counter()
    vibez.spill("Global counter:", global_counter)
}

# ===== CONTROL STRUCTURES =====

slay test_control_structures() {
    vibez.spill("🔀 Testing Control Structures")
    
    # If-else testing
    sus score drip = 85
    sus grade tea = ""
    
    ready (score >= 90) {
        grade = "A"
    } otherwise ready (score >= 80) {
        grade = "B"
    } otherwise ready (score >= 70) {
        grade = "C"
    } otherwise {
        grade = "F"
    }
    
    vibez.spill("Score", score, "gets grade:", grade)
    
    # Loop testing
    sus loop_sum drip = 0
    bestie (sus i drip = 1; i <= 5; i = i + 1) {
        loop_sum = loop_sum + i
    }
    vibez.spill("Sum 1-5:", loop_sum)
}

# ===== FUNCTION CALLS AND RECURSION =====

slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay test_function_calls() {
    vibez.spill("🔧 Testing Function Calls")
    
    sus fact5 drip = factorial(5)
    sus fib7 drip = fibonacci(7)
    
    vibez.spill("5! =", fact5)
    vibez.spill("fib(7) =", fib7)
}

# ===== ARRAYS AND DATA STRUCTURES =====

squad Person {
    name tea,
    age drip
}

slay Person.greet(self Person) tea {
    damn "Hello, I'm " + self.name
}

slay test_data_structures() {
    vibez.spill("📊 Testing Data Structures")
    
    # Array operations
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus sum drip = 0
    
    bestie (sus num drip in numbers) {
        sum = sum + num
    }
    
    vibez.spill("Array sum:", sum)
    
    # Struct operations
    sus person Person = Person{
        name: "Alice",
        age: 30
    }
    
    vibez.spill(person.greet())
    vibez.spill("Age:", person.age)
}

# ===== PATTERN MATCHING =====

slay classify_number(n drip) tea {
    sick (n) {
        when 0 -> damn "zero"
        when 1..10 -> damn "small"
        when 11..100 -> damn "medium" 
        when _ -> damn "large"
    }
}

slay test_pattern_matching() {
    vibez.spill("🎯 Testing Pattern Matching")
    
    sus numbers []drip = [0, 5, 50, 500]
    
    bestie (sus num drip in numbers) {
        sus classification tea = classify_number(num)
        vibez.spill("Number", num, "is", classification)
    }
}

# ===== STANDARD LIBRARY USAGE =====

slay test_stdlib_compatibility() {
    vibez.spill("📚 Testing Standard Library")
    
    # Math functions
    sus sqrt_val drip = mathz.sqrt(16)
    sus pow_val drip = mathz.pow(2, 8)
    
    vibez.spill("sqrt(16) =", sqrt_val)
    vibez.spill("2^8 =", pow_val)
    
    # String operations
    sus text tea = "Hello World"
    sus upper_text tea = stringz.to_upper(text)
    sus length drip = stringz.len(text)
    
    vibez.spill("Original:", text)
    vibez.spill("Uppercase:", upper_text)
    vibez.spill("Length:", length)
    
    # Array operations  
    sus test_array []drip = [3, 1, 4, 1, 5, 9]
    arrayz.sort(test_array)
    
    vibez.spill("Sorted array:", test_array)
}

# ===== ERROR HANDLING COMPATIBILITY =====

slay risky_operation(should_fail lit) yikes<drip> {
    ready (should_fail) {
        yikes "Operation failed as requested"
    }
    damn 42
}

slay test_error_handling() {
    vibez.spill("⚠️ Testing Error Handling")
    
    # Successful operation
    sus result1 drip = risky_operation(nah) fam {
        when _ -> damn -1
    }
    vibez.spill("Successful operation:", result1)
    
    # Failed operation
    sus result2 drip = risky_operation(based) fam {
        when msg tea -> {
            vibez.spill("Caught error:", msg)
            damn 0
        }
    }
    vibez.spill("Handled failure:", result2)
}

# ===== MEMORY INTENSIVE OPERATIONS =====

slay test_memory_operations() {
    vibez.spill("💾 Testing Memory Operations")
    
    # Create and manipulate large arrays
    sus large_array []drip = []
    
    bestie (sus i drip = 0; i < 1000; i = i + 1) {
        arrayz.push(large_array, i * i)
    }
    
    sus total drip = 0
    bestie (sus val drip in large_array) {
        total = total + val
    }
    
    vibez.spill("Processed", arrayz.len(large_array), "elements")
    vibez.spill("Total sum:", total)
}

# ===== BACKEND SPECIFIC FEATURES =====

slay test_backend_specific() {
    vibez.spill("🔧 Testing Backend-Specific Features")
    
    # This should work on all backends but may have different performance
    sus iterations drip = 10000
    sus computation_result drip = 0
    
    bestie (sus i drip = 0; i < iterations; i = i + 1) {
        computation_result = computation_result + (i % 7) * (i % 11)
    }
    
    vibez.spill("Complex computation result:", computation_result)
}

# ===== MAIN COMPATIBILITY TEST =====

slay main() {
    vibez.spill("🌉 CURSED Cross-Backend Compatibility Test")
    vibez.spill("==========================================")
    vibez.spill("This test validates identical behavior across:")
    vibez.spill("- Script Backend (Direct interpretation)")
    vibez.spill("- AST Backend (Abstract Syntax Tree execution)")
    vibez.spill("- LLVM Backend (Native compilation)")
    vibez.spill("")
    
    test_basic_arithmetic()
    vibez.spill("")
    
    test_control_structures()
    vibez.spill("")
    
    test_function_calls()
    vibez.spill("")
    
    test_data_structures()
    vibez.spill("")
    
    test_pattern_matching()
    vibez.spill("")
    
    test_stdlib_compatibility()
    vibez.spill("")
    
    test_error_handling()
    vibez.spill("")
    
    test_memory_operations()
    vibez.spill("")
    
    test_backend_specific()
    
    vibez.spill("==========================================")
    vibez.spill("✅ Cross-backend compatibility validated!")
    vibez.spill("🎯 All backends should produce identical output")
    vibez.spill("⚡ Performance may vary between backends")
}
