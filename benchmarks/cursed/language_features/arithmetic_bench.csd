fr fr Arithmetic Operations Benchmark Suite

yeet "benchz"
yeet "testz"
yeet "mathz"

slay benchmark_basic_arithmetic() lit {
    benchmark_suite_start("Arithmetic Operations")
    
    fr fr Integer arithmetic benchmarks
    benchmark_precise("Integer Addition", slay() {
        sus result normie = 42 + 37
    })
    
    benchmark_precise("Integer Subtraction", slay() {
        sus result normie = 1000 - 237
    })
    
    benchmark_precise("Integer Multiplication", slay() {
        sus result normie = 123 * 456
    })
    
    benchmark_precise("Integer Division", slay() {
        sus result normie = 10000 / 37
    })
    
    benchmark_precise("Integer Modulo", slay() {
        sus result normie = 12345 % 67
    })
    
    fr fr Float arithmetic benchmarks
    benchmark_precise("Float Addition", slay() {
        sus result meal = 3.14159 + 2.71828
    })
    
    benchmark_precise("Float Multiplication", slay() {
        sus result meal = 3.14159 * 2.71828
    })
    
    benchmark_precise("Float Division", slay() {
        sus result meal = 3.14159 / 2.71828
    })
    
    fr fr Complex arithmetic expressions
    benchmark_precise("Complex Expression", slay() {
        sus result normie = ((42 + 37) * (1000 - 237)) / (123 + 456)
    })
    
    benchmark_precise("Nested Arithmetic", slay() {
        sus a normie = 10
        sus b normie = 20
        sus c normie = 30
        sus result normie = a * (b + c) - (a / 2) + (b * c)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_math_functions() lit {
    benchmark_suite_start("Math Library Functions")
    
    benchmark_precise("Absolute Value", slay() {
        sus result normie = abs_normie(-42)
    })
    
    benchmark_precise("Power Function", slay() {
        sus result meal = pow(2.0, 10.0)
    })
    
    benchmark_precise("Square Root", slay() {
        sus result meal = sqrt(144.0)
    })
    
    benchmark_precise("Trigonometry Sin", slay() {
        sus result meal = sin(3.14159 / 4.0)
    })
    
    benchmark_precise("Trigonometry Cos", slay() {
        sus result meal = cos(3.14159 / 4.0)
    })
    
    benchmark_precise("Natural Log", slay() {
        sus result meal = ln(2.71828)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_bitwise_operations() lit {
    benchmark_suite_start("Bitwise Operations")
    
    benchmark_precise("Bitwise AND", slay() {
        sus result normie = 0xFF & 0xAA
    })
    
    benchmark_precise("Bitwise OR", slay() {
        sus result normie = 0xFF | 0xAA
    })
    
    benchmark_precise("Bitwise XOR", slay() {
        sus result normie = 0xFF ^ 0xAA
    })
    
    benchmark_precise("Left Shift", slay() {
        sus result normie = 42 << 3
    })
    
    benchmark_precise("Right Shift", slay() {
        sus result normie = 1024 >> 2
    })
    
    benchmark_precise("Bitwise NOT", slay() {
        sus result normie = ~0xAA
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_comparison_operations() lit {
    benchmark_suite_start("Comparison Operations")
    
    benchmark_precise("Integer Equality", slay() {
        sus result lit = 42 == 42
    })
    
    benchmark_precise("Integer Inequality", slay() {
        sus result lit = 42 != 37
    })
    
    benchmark_precise("Less Than", slay() {
        sus result lit = 37 < 42
    })
    
    benchmark_precise("Greater Than", slay() {
        sus result lit = 42 > 37
    })
    
    benchmark_precise("Less Than Equal", slay() {
        sus result lit = 37 <= 42
    })
    
    benchmark_precise("Greater Than Equal", slay() {
        sus result lit = 42 >= 42
    })
    
    benchmark_precise("Float Comparison", slay() {
        sus result lit = 3.14159 > 2.71828
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_logical_operations() lit {
    benchmark_suite_start("Logical Operations")
    
    benchmark_precise("Logical AND", slay() {
        sus result lit = based && based
    })
    
    benchmark_precise("Logical OR", slay() {
        sus result lit = based || cringe
    })
    
    benchmark_precise("Logical NOT", slay() {
        sus result lit = !cringe
    })
    
    benchmark_precise("Complex Logical", slay() {
        sus a lit = based
        sus b lit = cringe
        sus c lit = based
        sus result lit = (a && b) || (a && c) || (!b && c)
    })
    
    generate_benchmark_report()
    damn based
}

slay run_all_arithmetic_benchmarks() lit {
    vibez.spill("🚀 Running All Arithmetic Benchmarks")
    vibez.spill("═══════════════════════════════════════")
    
    benchmark_basic_arithmetic()
    benchmark_math_functions()
    benchmark_bitwise_operations()
    benchmark_comparison_operations()
    benchmark_logical_operations()
    
    vibez.spill("\n✅ All arithmetic benchmarks completed!")
    damn based
}

fr fr Run benchmarks if this file is executed directly
run_all_arithmetic_benchmarks()
