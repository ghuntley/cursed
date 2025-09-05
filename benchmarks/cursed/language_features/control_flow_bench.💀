fr fr Control Flow Performance Benchmark Suite

yeet "benchz"
yeet "testz"

slay benchmark_conditional_statements() lit {
    benchmark_suite_start("Conditional Statements")
    
    fr fr Simple if statements
    benchmark_precise("Simple If True", slay() {
        sus x normie = 42
        ready (x > 0) {
            sus result normie = x + 1
        }
    })
    
    benchmark_precise("Simple If False", slay() {
        sus x normie = -42
        ready (x > 0) {
            sus result normie = x + 1
        }
    })
    
    benchmark_precise("If-Else True Branch", slay() {
        sus x normie = 42
        ready (x > 0) {
            sus result normie = x + 1
        } otherwise {
            sus result normie = x - 1
        }
    })
    
    benchmark_precise("If-Else False Branch", slay() {
        sus x normie = -42
        ready (x > 0) {
            sus result normie = x + 1
        } otherwise {
            sus result normie = x - 1
        }
    })
    
    fr fr Nested conditionals
    benchmark_precise("Nested If Statements", slay() {
        sus x normie = 42
        sus y normie = 37
        ready (x > 0) {
            ready (y > 0) {
                sus result normie = x + y
            } otherwise {
                sus result normie = x - y
            }
        } otherwise {
            sus result normie = 0
        }
    })
    
    fr fr Complex conditions
    benchmark_precise("Complex Boolean Expression", slay() {
        sus a normie = 10
        sus b normie = 20
        sus c normie = 30
        ready ((a > 5 && b < 25) || (c > 25 && a != b)) {
            sus result normie = a + b + c
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_loops() lit {
    benchmark_suite_start("Loop Constructs")
    
    fr fr Simple while loops
    benchmark_precise("While Loop 100 iterations", slay() {
        sus i normie = 0
        sus sum normie = 0
        bestie (i < 100) {
            sum = sum + i
            i = i + 1
        }
    })
    
    benchmark_precise("While Loop 1000 iterations", slay() {
        sus i normie = 0
        sus sum normie = 0
        bestie (i < 1000) {
            sum = sum + i
            i = i + 1
        }
    })
    
    fr fr Nested loops
    benchmark_precise("Nested Loops 10x10", slay() {
        sus outer normie = 0
        sus total normie = 0
        bestie (outer < 10) {
            sus inner normie = 0
            bestie (inner < 10) {
                total = total + (outer * inner)
                inner = inner + 1
            }
            outer = outer + 1
        }
    })
    
    benchmark_precise("Nested Loops 50x50", slay() {
        sus outer normie = 0
        sus total normie = 0
        bestie (outer < 50) {
            sus inner normie = 0
            bestie (inner < 50) {
                total = total + 1
                inner = inner + 1
            }
            outer = outer + 1
        }
    })
    
    fr fr Loop with complex body
    benchmark_precise("Loop with Complex Operations", slay() {
        sus i normie = 0
        sus sum normie = 0
        sus product normie = 1
        bestie (i < 100) {
            sum = sum + (i * i)
            ready (i % 2 == 0) {
                product = product * 2
            } otherwise {
                product = product + 1
            }
            i = i + 1
        }
    })
    
    fr fr Loop with early termination
    benchmark_precise("Loop with Break", slay() {
        sus i normie = 0
        sus found normie = 0
        bestie (i < 1000) {
            ready (i == 500) {
                found = i
                fr fr break statement would go here
                i = 1000  fr fr Simulate break
            }
            i = i + 1
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_pattern_matching() lit {
    benchmark_suite_start("Pattern Matching")
    
    fr fr Simple pattern matching
    benchmark_precise("Pattern Match Integer", slay() {
        sus x normie = 42
        sus result normie = 0
        ready (x) {
            1 => { result = 100 }
            42 => { result = 200 }
            _ => { result = 0 }
        }
    })
    
    benchmark_precise("Pattern Match String", slay() {
        sus x tea = "hello"
        sus result normie = 0
        ready (x) {
            "world" => { result = 100 }
            "hello" => { result = 200 }
            _ => { result = 0 }
        }
    })
    
    fr fr Range patterns
    benchmark_precise("Pattern Match Range", slay() {
        sus x normie = 75
        sus result tea = ""
        ready (x) {
            0..10 => { result = "small" }
            11..50 => { result = "medium" }
            51..100 => { result = "large" }
            _ => { result = "unknown" }
        }
    })
    
    fr fr Guard patterns
    benchmark_precise("Pattern Match with Guards", slay() {
        sus x normie = 42
        sus y normie = 37
        sus result normie = 0
        ready (x) {
            n when n > 50 => { result = n * 2 }
            n when n > 20 => { result = n + y }
            _ => { result = 0 }
        }
    })
    
    fr fr Complex pattern matching
    benchmark_precise("Complex Pattern Match", slay() {
        sus data normie = 123
        sus category tea = "number"
        sus result normie = 0
        
        ready (data) {
            x when x < 0 => { result = -1 }
            x when x > 0 && x < 100 => { result = x * 2 }
            x when x >= 100 && category == "number" => { result = x / 2 }
            _ => { result = 0 }
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_function_calls() lit {
    benchmark_suite_start("Function Call Performance")
    
    fr fr Define test functions
    slay simple_function(x normie) normie {
        damn x + 1
    }
    
    slay recursive_function(n normie) normie {
        ready (n <= 1) {
            damn 1
        }
        damn n * recursive_function(n - 1)
    }
    
    slay function_with_multiple_params(a normie, b normie, c normie) normie {
        damn a + b + c
    }
    
    fr fr Simple function calls
    benchmark_precise("Simple Function Call", slay() {
        sus result normie = simple_function(42)
    })
    
    benchmark_precise("Function with Multiple Parameters", slay() {
        sus result normie = function_with_multiple_params(10, 20, 30)
    })
    
    fr fr Recursive function calls
    benchmark_precise("Recursive Function (depth 5)", slay() {
        sus result normie = recursive_function(5)
    })
    
    benchmark_precise("Recursive Function (depth 10)", slay() {
        sus result normie = recursive_function(10)
    })
    
    fr fr Function calls in loops
    benchmark_precise("Function Calls in Loop", slay() {
        sus i normie = 0
        sus total normie = 0
        bestie (i < 100) {
            total = total + simple_function(i)
            i = i + 1
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_variable_operations() lit {
    benchmark_suite_start("Variable Operations")
    
    fr fr Variable assignment
    benchmark_precise("Integer Assignment", slay() {
        sus x normie = 42
    })
    
    benchmark_precise("String Assignment", slay() {
        sus text tea = "Hello, world!"
    })
    
    benchmark_precise("Boolean Assignment", slay() {
        sus flag lit = based
    })
    
    fr fr Variable access
    benchmark_precise("Variable Read", slay() {
        sus x normie = 42
        sus y normie = x
    })
    
    benchmark_precise("Multiple Variable Access", slay() {
        sus a normie = 10
        sus b normie = 20
        sus c normie = 30
        sus result normie = a + b + c
    })
    
    fr fr Variable modification
    benchmark_precise("Variable Increment", slay() {
        sus x normie = 0
        x = x + 1
    })
    
    benchmark_precise("Variable Compound Assignment", slay() {
        sus x normie = 42
        x = x * 2 + 10
    })
    
    fr fr Scope operations
    benchmark_precise("Local Variable Scope", slay() {
        sus outer normie = 42
        ready (based) {
            sus inner normie = outer + 10
            sus result normie = inner * 2
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay run_all_control_flow_benchmarks() lit {
    vibez.spill("🚀 Running All Control Flow Benchmarks")
    vibez.spill("════════════════════════════════════════")
    
    benchmark_conditional_statements()
    benchmark_loops()
    benchmark_pattern_matching()
    benchmark_function_calls()
    benchmark_variable_operations()
    
    vibez.spill("\n✅ All control flow benchmarks completed!")
    damn based
}

fr fr Run benchmarks if this file is executed directly
run_all_control_flow_benchmarks()
