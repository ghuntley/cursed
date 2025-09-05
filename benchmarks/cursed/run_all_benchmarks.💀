fr fr CURSED Comprehensive Benchmark Suite Runner

yeet "benchz"
yeet "testz"

fr fr Import all benchmark modules
yeet "language_features/arithmetic_bench"
yeet "language_features/control_flow_bench"
yeet "language_features/function_call_bench"
yeet "stdlib/string_bench"
yeet "stdlib/array_bench"
yeet "stdlib/crypto_bench"
yeet "compiler/compilation_speed_bench"
yeet "compiler/memory_usage_bench"

sus benchmark_start_time normie = 0
sus total_benchmarks_run normie = 0
sus failed_benchmarks normie = 0

squad BenchmarkSuiteResult {
    spill suite_name tea
    spill duration_ms normie
    spill benchmarks_run normie
    spill success lit
    spill error_message tea
}

sus suite_results []BenchmarkSuiteResult = []

slay benchmark_suite_wrapper(suite_name tea, suite_function slay()) BenchmarkSuiteResult {
    vibez.spill("\n🚀 Starting benchmark suite: ", suite_name)
    vibez.spill("═══════════════════════════════════════════════")
    
    sus start_time normie = get_time_ns()
    sus success lit = based
    sus error_msg tea = ""
    sus benchmarks_before normie = benchmark_results.len()
    
    ready {
        suite_function()
    } yikes {
        success = cringe
        error_msg = "Suite execution failed"
        failed_benchmarks = failed_benchmarks + 1
    }
    
    sus end_time normie = get_time_ns()
    sus duration normie = (end_time - start_time) / 1000000
    sus benchmarks_run normie = benchmark_results.len() - benchmarks_before
    total_benchmarks_run = total_benchmarks_run + benchmarks_run
    
    sus result BenchmarkSuiteResult = BenchmarkSuiteResult{
        suite_name: suite_name,
        duration_ms: duration,
        benchmarks_run: benchmarks_run,
        success: success,
        error_message: error_msg
    }
    
    suite_results.push(result)
    
    lowkey success {
        vibez.spill("✅ ", suite_name, " completed in ", duration, "ms (", benchmarks_run, " benchmarks)")
    } highkey {
        vibez.spill("❌ ", suite_name, " failed: ", error_msg)
    }
    
    damn result
}

slay run_language_feature_benchmarks() lit {
    vibez.spill("\n📊 LANGUAGE FEATURES BENCHMARKS")
    vibez.spill("════════════════════════════════════════")
    
    benchmark_suite_wrapper("Arithmetic Operations", run_all_arithmetic_benchmarks)
    benchmark_suite_wrapper("Control Flow", run_all_control_flow_benchmarks)
    benchmark_suite_wrapper("Function Calls", run_all_function_call_benchmarks)
    
    damn based
}

slay run_stdlib_benchmarks() lit {
    vibez.spill("\n📚 STANDARD LIBRARY BENCHMARKS")
    vibez.spill("════════════════════════════════════════")
    
    benchmark_suite_wrapper("String Operations", run_all_string_benchmarks)
    benchmark_suite_wrapper("Array Operations", run_all_array_benchmarks)
    benchmark_suite_wrapper("Cryptography", run_all_crypto_benchmarks)
    
    damn based
}

slay run_compiler_benchmarks() lit {
    vibez.spill("\n🔧 COMPILER BENCHMARKS")
    vibez.spill("════════════════════════════════════════")
    
    benchmark_suite_wrapper("Compilation Speed", run_all_compilation_benchmarks)
    benchmark_suite_wrapper("Memory Usage", run_all_memory_benchmarks)
    
    damn based
}

slay run_concurrency_benchmarks() lit {
    vibez.spill("\n⚡ CONCURRENCY BENCHMARKS")
    vibez.spill("════════════════════════════════════════")
    
    benchmark_suite_start("Concurrency Performance")
    
    fr fr Goroutine benchmarks
    benchmark_precise("Single Goroutine Creation", slay() {
        stan {
            sus x normie = 42
        }
    })
    
    benchmark_precise("Multiple Goroutine Creation", slay() {
        sus i normie = 0
        bestie (i < 10) {
            stan {
                sus local_val normie = i
            }
            i = i + 1
        }
    })
    
    benchmark_precise("Goroutine with Channel Communication", slay() {
        sus ch dm<normie> = make_channel()
        stan {
            dm_send(ch, 42)
        }
        sus value normie = dm_recv(ch)
    })
    
    fr fr Channel benchmarks
    benchmark_precise("Channel Send/Receive", slay() {
        sus ch dm<normie> = make_channel()
        dm_send(ch, 123)
        sus value normie = dm_recv(ch)
    })
    
    benchmark_precise("Buffered Channel Operations", slay() {
        sus ch dm<normie> = make_buffered_channel(10)
        sus i normie = 0
        bestie (i < 5) {
            dm_send(ch, i)
            i = i + 1
        }
        i = 0
        bestie (i < 5) {
            sus value normie = dm_recv(ch)
            i = i + 1
        }
    })
    
    fr fr Synchronization benchmarks
    benchmark_precise("Mutex Lock/Unlock", slay() {
        sus mutex Mutex = create_mutex()
        lock(mutex)
        sus x normie = 42
        unlock(mutex)
    })
    
    generate_benchmark_report()
    damn based
}

slay run_pattern_matching_benchmarks() lit {
    vibez.spill("\n🎯 PATTERN MATCHING BENCHMARKS")
    vibez.spill("════════════════════════════════════════")
    
    benchmark_suite_start("Pattern Matching Performance")
    
    fr fr Basic pattern matching
    benchmark_precise("Simple Integer Pattern", slay() {
        sus x normie = 42
        sus result normie = 0
        ready (x) {
            42 => { result = 1 }
            _ => { result = 0 }
        }
    })
    
    benchmark_precise("Range Pattern Matching", slay() {
        sus x normie = 75
        sus result tea = ""
        ready (x) {
            0..10 => { result = "small" }
            11..50 => { result = "medium" }
            51..100 => { result = "large" }
            _ => { result = "unknown" }
        }
    })
    
    benchmark_precise("String Pattern Matching", slay() {
        sus text tea = "hello"
        sus result normie = 0
        ready (text) {
            "hello" => { result = 1 }
            "world" => { result = 2 }
            _ => { result = 0 }
        }
    })
    
    benchmark_precise("Guard Pattern Matching", slay() {
        sus x normie = 25
        sus y normie = 30
        sus result normie = 0
        ready (x) {
            n when n > 50 => { result = 1 }
            n when n > 20 && y > 25 => { result = 2 }
            _ => { result = 0 }
        }
    })
    
    fr fr Complex pattern matching
    benchmark_precise("Nested Pattern Matching", slay() {
        sus data normie = 15
        sus category tea = "number"
        sus result normie = 0
        
        ready (data) {
            x when x < 0 => { result = -1 }
            x when x > 0 && x < 20 => {
                ready (category) {
                    "number" => { result = x * 2 }
                    _ => { result = x }
                }
            }
            _ => { result = 0 }
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay run_interface_benchmarks() lit {
    vibez.spill("\n🔗 INTERFACE BENCHMARKS")
    vibez.spill("════════════════════════════════════════")
    
    benchmark_suite_start("Interface Performance")
    
    fr fr Define test interfaces and implementations
    collab Drawable {
        slay draw()
        slay get_name() tea
    }
    
    squad Circle {
        spill radius meal
        
        slay draw() {
            vibez.spill("Drawing circle")
        }
        
        slay get_name() tea {
            damn "Circle"
        }
    }
    
    squad Rectangle {
        spill width meal
        spill height meal
        
        slay draw() {
            vibez.spill("Drawing rectangle")
        }
        
        slay get_name() tea {
            damn "Rectangle"
        }
    }
    
    fr fr Interface method dispatch benchmarks
    benchmark_precise("Interface Method Call", slay() {
        sus drawable Drawable = Circle{radius: 5.0}
        drawable.draw()
    })
    
    benchmark_precise("Interface Return Value", slay() {
        sus drawable Drawable = Rectangle{width: 10.0, height: 8.0}
        sus name tea = drawable.get_name()
    })
    
    benchmark_precise("Multiple Interface Objects", slay() {
        sus objects []Drawable = [
            Circle{radius: 3.0},
            Rectangle{width: 5.0, height: 4.0},
            Circle{radius: 7.0}
        ]
        
        bestie obj in objects {
            obj.draw()
        }
    })
    
    benchmark_precise("Interface Type Switching", slay() {
        sus drawable Drawable = Circle{radius: 5.0}
        ready (drawable) {
            Circle => { vibez.spill("It's a circle") }
            Rectangle => { vibez.spill("It's a rectangle") }
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay generate_comprehensive_report() lit {
    vibez.spill("\n")
    vibez.spill("═══════════════════════════════════════════════════════")
    vibez.spill("🏆 COMPREHENSIVE BENCHMARK REPORT")
    vibez.spill("═══════════════════════════════════════════════════════")
    
    sus total_duration normie = get_time_ns() - benchmark_start_time
    sus total_duration_ms normie = total_duration / 1000000
    
    vibez.spill("⏱️ Total execution time: ", total_duration_ms, "ms")
    vibez.spill("📊 Total benchmarks run: ", total_benchmarks_run)
    vibez.spill("✅ Successful suites: ", suite_results.len() - failed_benchmarks)
    vibez.spill("❌ Failed suites: ", failed_benchmarks)
    vibez.spill("")
    
    fr fr Suite summary
    vibez.spill("📋 SUITE SUMMARY")
    vibez.spill("───────────────────────────────────────────────────────")
    bestie suite in suite_results {
        sus status tea = "✅"
        lowkey !suite.success {
            status = "❌"
        }
        vibez.spill(status, " ", suite.suite_name, " (", suite.duration_ms, "ms, ", suite.benchmarks_run, " benchmarks)")
        lowkey !suite.success {
            vibez.spill("   Error: ", suite.error_message)
        }
    }
    vibez.spill("")
    
    fr fr Performance insights
    vibez.spill("📈 PERFORMANCE INSIGHTS")
    vibez.spill("───────────────────────────────────────────────────────")
    
    fr fr Find fastest and slowest benchmarks
    sus fastest_name tea = ""
    sus fastest_ops meal = 0.0
    sus slowest_name tea = ""
    sus slowest_ops meal = 999999999.0
    
    bestie result in benchmark_results {
        lowkey result.ops_per_sec > fastest_ops {
            fastest_ops = result.ops_per_sec
            fastest_name = result.name
        }
        lowkey result.ops_per_sec < slowest_ops {
            slowest_ops = result.ops_per_sec
            slowest_name = result.name
        }
    }
    
    lowkey fastest_name != "" {
        vibez.spill("🚀 Fastest operation: ", fastest_name, " (", fastest_ops, " ops/sec)")
        vibez.spill("🐌 Slowest operation: ", slowest_name, " (", slowest_ops, " ops/sec)")
        vibez.spill("📊 Performance range: ", fastest_ops / slowest_ops, "x difference")
    }
    
    fr fr Memory insights
    sus total_memory normie = 0
    sus benchmark_count normie = 0
    bestie result in memory_results {
        total_memory = total_memory + result.peak_memory
        benchmark_count = benchmark_count + 1
    }
    
    lowkey benchmark_count > 0 {
        sus avg_memory normie = total_memory / benchmark_count
        vibez.spill("🧠 Average peak memory: ", avg_memory, " bytes")
    }
    
    vibez.spill("")
    
    fr fr Export results
    vibez.spill("💾 EXPORT OPTIONS")
    vibez.spill("───────────────────────────────────────────────────────")
    vibez.spill("JSON export: ", export_benchmark_json())
    vibez.spill("CSV export: ", export_benchmark_csv())
    
    fr fr Recommendations
    vibez.spill("")
    vibez.spill("💡 RECOMMENDATIONS")
    vibez.spill("───────────────────────────────────────────────────────")
    vibez.spill("• Run benchmarks multiple times for statistical significance")
    vibez.spill("• Monitor memory usage patterns for potential leaks")
    vibez.spill("• Compare results across different compiler optimization levels")
    vibez.spill("• Profile hotspots identified in slowest operations")
    vibez.spill("• Consider parallel execution for CPU-bound operations")
    
    lowkey failed_benchmarks > 0 {
        vibez.spill("⚠️ Address failed benchmark suites before production use")
    }
    
    vibez.spill("")
    vibez.spill("═══════════════════════════════════════════════════════")
    vibez.spill("🎯 Benchmark suite completed successfully!")
    vibez.spill("═══════════════════════════════════════════════════════")
    
    damn based
}

fr fr Mock runtime functions for concurrency
slay make_channel() dm<normie> {
    fr fr Mock implementation
    damn null
}

slay make_buffered_channel(size normie) dm<normie> {
    fr fr Mock implementation
    damn null
}

squad Mutex {
    spill locked lit
}

slay create_mutex() Mutex {
    damn Mutex{locked: cringe}
}

slay lock(mutex Mutex) lit {
    mutex.locked = based
    damn based
}

slay unlock(mutex Mutex) lit {
    mutex.locked = cringe
    damn based
}

slay main_character() lit {
    vibez.spill("🚀 CURSED Comprehensive Performance Benchmark Suite")
    vibez.spill("═══════════════════════════════════════════════════════")
    vibez.spill("This benchmark suite will test all aspects of the CURSED compiler:")
    vibez.spill("• Language features (arithmetic, control flow, functions)")
    vibez.spill("• Standard library performance (strings, arrays, crypto)")
    vibez.spill("• Compiler performance (compilation speed, memory usage)")
    vibez.spill("• Advanced features (concurrency, pattern matching, interfaces)")
    vibez.spill("")
    
    benchmark_start_time = get_time_ns()
    
    fr fr Run all benchmark suites
    run_language_feature_benchmarks()
    run_stdlib_benchmarks()
    run_compiler_benchmarks()
    run_concurrency_benchmarks()
    run_pattern_matching_benchmarks()
    run_interface_benchmarks()
    
    fr fr Generate final report
    generate_comprehensive_report()
    
    damn based
}

fr fr Run the main benchmark suite
main()
