fr fr Function Call Performance Benchmark Suite

yeet "benchz"
yeet "testz"

fr fr Test function definitions
slay empty_function() lit {
    damn based
}

slay single_param_function(x normie) normie {
    damn x
}

slay two_param_function(x normie, y normie) normie {
    damn x + y
}

slay five_param_function(a normie, b normie, c normie, d normie, e normie) normie {
    damn a + b + c + d + e
}

slay computation_function(x normie) normie {
    sus result normie = x * x + x - 1
    damn result
}

slay string_function(text tea) tea {
    damn text + " processed"
}

slay recursive_factorial(n normie) normie {
    ready (n <= 1) {
        damn 1
    }
    damn n * recursive_factorial(n - 1)
}

slay recursive_fibonacci(n normie) normie {
    ready (n <= 1) {
        damn n
    }
    damn recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
}

slay tail_recursive_helper(n normie, acc normie) normie {
    ready (n <= 0) {
        damn acc
    }
    damn tail_recursive_helper(n - 1, acc + n)
}

slay tail_recursive_sum(n normie) normie {
    damn tail_recursive_helper(n, 0)
}

slay higher_order_function(func slay(normie) normie, value normie) normie {
    damn func(value)
}

slay benchmark_basic_function_calls() lit {
    benchmark_suite_start("Basic Function Calls")
    
    benchmark_precise("Empty Function Call", slay() {
        empty_function()
    })
    
    benchmark_precise("Single Parameter Function", slay() {
        sus result normie = single_param_function(42)
    })
    
    benchmark_precise("Two Parameter Function", slay() {
        sus result normie = two_param_function(10, 20)
    })
    
    benchmark_precise("Five Parameter Function", slay() {
        sus result normie = five_param_function(1, 2, 3, 4, 5)
    })
    
    benchmark_precise("Computation Function", slay() {
        sus result normie = computation_function(42)
    })
    
    benchmark_precise("String Function", slay() {
        sus result tea = string_function("test")
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_recursive_functions() lit {
    benchmark_suite_start("Recursive Function Calls")
    
    benchmark_precise("Factorial Recursive (5)", slay() {
        sus result normie = recursive_factorial(5)
    })
    
    benchmark_precise("Factorial Recursive (10)", slay() {
        sus result normie = recursive_factorial(10)
    })
    
    benchmark_precise("Factorial Recursive (15)", slay() {
        sus result normie = recursive_factorial(15)
    })
    
    benchmark_precise("Fibonacci Recursive (5)", slay() {
        sus result normie = recursive_fibonacci(5)
    })
    
    benchmark_precise("Fibonacci Recursive (10)", slay() {
        sus result normie = recursive_fibonacci(10)
    })
    
    benchmark_precise("Fibonacci Recursive (15)", slay() {
        sus result normie = recursive_fibonacci(15)
    })
    
    benchmark_precise("Tail Recursive Sum (100)", slay() {
        sus result normie = tail_recursive_sum(100)
    })
    
    benchmark_precise("Tail Recursive Sum (1000)", slay() {
        sus result normie = tail_recursive_sum(1000)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_function_call_patterns() lit {
    benchmark_suite_start("Function Call Patterns")
    
    fr fr Function calls in loops
    benchmark_precise("Function Calls in Loop (100)", slay() {
        sus i normie = 0
        sus total normie = 0
        bestie (i < 100) {
            total = total + computation_function(i)
            i = i + 1
        }
    })
    
    benchmark_precise("Function Calls in Loop (1000)", slay() {
        sus i normie = 0
        sus total normie = 0
        bestie (i < 1000) {
            total = total + single_param_function(i)
            i = i + 1
        }
    })
    
    fr fr Nested function calls
    benchmark_precise("Nested Function Calls", slay() {
        sus result normie = computation_function(
            two_param_function(
                single_param_function(5),
                single_param_function(10)
            )
        )
    })
    
    fr fr Function calls with complex expressions
    benchmark_precise("Function Call with Expression Args", slay() {
        sus result normie = five_param_function(
            10 + 5,
            20 * 2,
            30 - 5,
            40 / 2,
            50 % 7
        )
    })
    
    fr fr Higher-order function calls
    benchmark_precise("Higher-Order Function Call", slay() {
        sus result normie = higher_order_function(computation_function, 42)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_parameter_passing() lit {
    benchmark_suite_start("Parameter Passing Performance")
    
    fr fr Different parameter types and sizes
    slay large_struct_function(data []normie) normie {
        sus sum normie = 0
        sus i normie = 0
        bestie (i < data.len()) {
            sum = sum + data[i]
            i = i + 1
        }
        damn sum
    }
    
    slay multiple_string_function(a tea, b tea, c tea, d tea) tea {
        damn a + b + c + d
    }
    
    benchmark_precise("Pass by Value Integer", slay() {
        sus value normie = 42
        sus result normie = single_param_function(value)
    })
    
    benchmark_precise("Pass by Value String", slay() {
        sus text tea = "Hello, world!"
        sus result tea = string_function(text)
    })
    
    benchmark_precise("Pass Large Array", slay() {
        sus data []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        sus result normie = large_struct_function(data)
    })
    
    benchmark_precise("Pass Multiple Strings", slay() {
        sus result tea = multiple_string_function("a", "b", "c", "d")
    })
    
    fr fr Variable argument patterns
    benchmark_precise("Many Small Parameters", slay() {
        sus result normie = five_param_function(1, 2, 3, 4, 5)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_function_overhead() lit {
    benchmark_suite_start("Function Call Overhead")
    
    fr fr Compare direct computation vs function call
    benchmark_precise("Direct Computation", slay() {
        sus x normie = 42
        sus result normie = x * x + x - 1
    })
    
    benchmark_precise("Function Call Equivalent", slay() {
        sus result normie = computation_function(42)
    })
    
    fr fr Compare inline vs function call
    benchmark_precise("Inline Addition", slay() {
        sus result normie = 10 + 20
    })
    
    benchmark_precise("Function Addition", slay() {
        sus result normie = two_param_function(10, 20)
    })
    
    fr fr Function call depth analysis
    slay depth_1_function(x normie) normie {
        damn x + 1
    }
    
    slay depth_2_function(x normie) normie {
        damn depth_1_function(x) + 1
    }
    
    slay depth_3_function(x normie) normie {
        damn depth_2_function(x) + 1
    }
    
    slay depth_5_function(x normie) normie {
        damn depth_3_function(depth_2_function(x))
    }
    
    benchmark_precise("Call Depth 1", slay() {
        sus result normie = depth_1_function(42)
    })
    
    benchmark_precise("Call Depth 2", slay() {
        sus result normie = depth_2_function(42)
    })
    
    benchmark_precise("Call Depth 3", slay() {
        sus result normie = depth_3_function(42)
    })
    
    benchmark_precise("Call Depth 5", slay() {
        sus result normie = depth_5_function(42)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_return_value_handling() lit {
    benchmark_suite_start("Return Value Handling")
    
    slay return_integer() normie {
        damn 42
    }
    
    slay return_string() tea {
        damn "Hello, world!"
    }
    
    slay return_boolean() lit {
        damn based
    }
    
    slay return_array() []normie {
        damn [1, 2, 3, 4, 5]
    }
    
    slay return_nothing() lit {
        damn based
    }
    
    benchmark_precise("Return Integer", slay() {
        sus result normie = return_integer()
    })
    
    benchmark_precise("Return String", slay() {
        sus result tea = return_string()
    })
    
    benchmark_precise("Return Boolean", slay() {
        sus result lit = return_boolean()
    })
    
    benchmark_precise("Return Array", slay() {
        sus result []normie = return_array()
    })
    
    benchmark_precise("Return Nothing", slay() {
        return_nothing()
    })
    
    fr fr Unused return values
    benchmark_precise("Ignore Return Value", slay() {
        return_integer()  fr fr Result not stored
    })
    
    generate_benchmark_report()
    damn based
}

slay run_all_function_call_benchmarks() lit {
    vibez.spill("🚀 Running All Function Call Benchmarks")
    vibez.spill("═══════════════════════════════════════════")
    
    benchmark_basic_function_calls()
    benchmark_recursive_functions()
    benchmark_function_call_patterns()
    benchmark_parameter_passing()
    benchmark_function_overhead()
    benchmark_return_value_handling()
    
    vibez.spill("\n✅ All function call benchmarks completed!")
    
    fr fr Comparative analysis
    compare_benchmarks("Direct Computation", "Function Call Equivalent")
    compare_benchmarks("Inline Addition", "Function Addition")
    compare_benchmarks("Call Depth 1", "Call Depth 5")
    
    damn based
}

fr fr Run benchmarks if this file is executed directly
run_all_function_call_benchmarks()
