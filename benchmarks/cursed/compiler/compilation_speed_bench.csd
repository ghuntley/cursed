fr fr Compilation Speed Performance Benchmark Suite

yeet "benchz"
yeet "testz"

slay benchmark_basic_compilation() lit {
    benchmark_suite_start("Basic Compilation Speed")
    
    fr fr Simple programs
    benchmark_compilation("hello_world.csd")
    benchmark_compilation("simple_arithmetic.csd")
    benchmark_compilation("basic_variables.csd")
    benchmark_compilation("simple_function.csd")
    benchmark_compilation("basic_control_flow.csd")
    
    generate_benchmark_report()
    damn based
}

slay benchmark_program_size_scaling() lit {
    benchmark_suite_start("Program Size Scaling")
    
    fr fr Create test programs of different sizes
    create_test_programs()
    
    benchmark_compilation("small_program.csd")
    benchmark_compilation("medium_program.csd")
    benchmark_compilation("large_program.csd")
    benchmark_compilation("very_large_program.csd")
    
    generate_benchmark_report()
    damn based
}

slay benchmark_feature_compilation() lit {
    benchmark_suite_start("Feature-Specific Compilation")
    
    fr fr Different language features
    benchmark_compilation("structs_test.csd")
    benchmark_compilation("interfaces_test.csd")
    benchmark_compilation("generics_test.csd")
    benchmark_compilation("pattern_matching_test.csd")
    benchmark_compilation("concurrency_test.csd")
    benchmark_compilation("error_handling_test.csd")
    
    generate_benchmark_report()
    damn based
}

slay benchmark_optimization_levels() lit {
    benchmark_suite_start("Optimization Level Performance")
    
    fr fr Test different optimization levels
    benchmark_compilation_with_opts("test_program.csd", "O0")
    benchmark_compilation_with_opts("test_program.csd", "O1")
    benchmark_compilation_with_opts("test_program.csd", "O2")
    benchmark_compilation_with_opts("test_program.csd", "O3")
    benchmark_compilation_with_opts("test_program.csd", "Os")  fr fr Size optimization
    
    generate_benchmark_report()
    damn based
}

slay benchmark_stdlib_compilation() lit {
    benchmark_suite_start("Standard Library Compilation")
    
    fr fr Programs using different stdlib modules
    benchmark_compilation("stringz_usage.csd")
    benchmark_compilation("arrayz_usage.csd")
    benchmark_compilation("mathz_usage.csd")
    benchmark_compilation("cryptz_usage.csd")
    benchmark_compilation("concurrenz_usage.csd")
    benchmark_compilation("comprehensive_stdlib_test.csd")
    
    generate_benchmark_report()
    damn based
}

slay benchmark_interpreter_vs_compilation() lit {
    benchmark_suite_start("Interpreter vs Compilation Performance")
    
    fr fr Compare interpretation and compilation times
    sus test_files []tea = [
        "arithmetic_heavy.csd",
        "function_heavy.csd",
        "loop_heavy.csd",
        "string_heavy.csd",
        "array_heavy.csd"
    ]
    
    bestie file in test_files {
        benchmark_interpretation_vs_compilation(file)
    }
    
    generate_benchmark_report()
    damn based
}

slay benchmark_incremental_compilation() lit {
    benchmark_suite_start("Incremental Compilation")
    
    fr fr Test incremental compilation benefits
    benchmark_precise("Clean Build", slay() {
        clean_build_cache()
        compile_project("test_project")
    })
    
    benchmark_precise("Incremental Build No Changes", slay() {
        compile_project("test_project")  fr fr Should be cached
    })
    
    benchmark_precise("Incremental Build Small Change", slay() {
        modify_single_file("test_project/main.csd")
        compile_project("test_project")
    })
    
    benchmark_precise("Incremental Build Module Change", slay() {
        modify_single_file("test_project/utils.csd")
        compile_project("test_project")
    })
    
    generate_benchmark_report()
    damn based
}

fr fr Helper functions for creating test programs
slay create_test_programs() lit {
    create_small_program()
    create_medium_program()
    create_large_program()
    create_very_large_program()
    damn based
}

slay create_small_program() lit {
    sus content tea = "
yeet \"mathz\"

slay main() lit {
    sus x normie = 42
    sus y normie = abs_normie(-10)
    vibez.spill(\"Result:\", x + y)
    damn based
}

main()
"
    write_file("small_program.csd", content)
    damn based
}

slay create_medium_program() lit {
    sus content tea = "
yeet \"mathz\"
yeet \"stringz\"
yeet \"arrayz\"

squad Point {
    spill x normie
    spill y normie
}

slay calculate_distance(p1 Point, p2 Point) meal {
    sus dx meal = p1.x - p2.x
    sus dy meal = p1.y - p2.y
    damn sqrt(dx * dx + dy * dy)
}

slay process_points(points []Point) []meal {
    sus distances []meal = []
    sus i normie = 0
    bestie (i < len(points) - 1) {
        sus dist meal = calculate_distance(points[i], points[i + 1])
        distances.push(dist)
        i = i + 1
    }
    damn distances
}

slay main() lit {
    sus points []Point = [
        Point{x: 0, y: 0},
        Point{x: 3, y: 4},
        Point{x: 6, y: 8},
        Point{x: 9, y: 12}
    ]
    
    sus distances []meal = process_points(points)
    bestie dist in distances {
        vibez.spill(\"Distance:\", dist)
    }
    damn based
}

main()
"
    write_file("medium_program.csd", content)
    damn based
}

slay create_large_program() lit {
    sus content tea = "
yeet \"mathz\"
yeet \"stringz\"
yeet \"arrayz\"
yeet \"hashz\"

squad Vector3D {
    spill x meal
    spill y meal
    spill z meal
}

squad Matrix3x3 {
    spill data [][]meal
}

collab Drawable {
    slay draw()
    slay get_area() meal
}

squad Circle {
    spill radius meal
    spill center Vector3D
    
    slay draw() {
        vibez.spill(\"Drawing circle with radius\", self.radius)
    }
    
    slay get_area() meal {
        damn 3.14159 * self.radius * self.radius
    }
}

squad Rectangle {
    spill width meal
    spill height meal
    spill position Vector3D
    
    slay draw() {
        vibez.spill(\"Drawing rectangle\", self.width, \"x\", self.height)
    }
    
    slay get_area() meal {
        damn self.width * self.height
    }
}

slay vector_add(a Vector3D, b Vector3D) Vector3D {
    damn Vector3D{
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z
    }
}

slay vector_magnitude(v Vector3D) meal {
    damn sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
}

slay matrix_multiply(a Matrix3x3, b Matrix3x3) Matrix3x3 {
    sus result Matrix3x3 = Matrix3x3{data: []}
    sus i normie = 0
    bestie (i < 3) {
        sus row []meal = []
        sus j normie = 0
        bestie (j < 3) {
            sus sum meal = 0.0
            sus k normie = 0
            bestie (k < 3) {
                sum = sum + a.data[i][k] * b.data[k][j]
                k = k + 1
            }
            row.push(sum)
            j = j + 1
        }
        result.data.push(row)
        i = i + 1
    }
    damn result
}

slay process_scene(objects []Drawable) meal {
    sus total_area meal = 0.0
    bestie obj in objects {
        obj.draw()
        total_area = total_area + obj.get_area()
    }
    damn total_area
}

slay fibonacci(n normie) normie {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay main() lit {
    sus objects []Drawable = [
        Circle{radius: 5.0, center: Vector3D{x: 0.0, y: 0.0, z: 0.0}},
        Rectangle{width: 10.0, height: 8.0, position: Vector3D{x: 5.0, y: 5.0, z: 0.0}},
        Circle{radius: 3.0, center: Vector3D{x: 10.0, y: 10.0, z: 0.0}}
    ]
    
    sus total_area meal = process_scene(objects)
    vibez.spill(\"Total area:\", total_area)
    
    sus fib_result normie = fibonacci(10)
    vibez.spill(\"Fibonacci(10):\", fib_result)
    
    sus v1 Vector3D = Vector3D{x: 1.0, y: 2.0, z: 3.0}
    sus v2 Vector3D = Vector3D{x: 4.0, y: 5.0, z: 6.0}
    sus v3 Vector3D = vector_add(v1, v2)
    sus magnitude meal = vector_magnitude(v3)
    
    vibez.spill(\"Vector magnitude:\", magnitude)
    damn based
}

main()
"
    write_file("large_program.csd", content)
    damn based
}

slay create_very_large_program() lit {
    sus content tea = ""
    
    fr fr Generate a very large program with many functions and classes
    sus i normie = 0
    bestie (i < 50) {
        content = content + "
squad Class" + i + " {
    spill value" + i + " normie
    
    slay method" + i + "(param normie) normie {
        damn self.value" + i + " + param
    }
    
    slay compute" + i + "() normie {
        sus result normie = self.value" + i + "
        sus j normie = 0
        bestie (j < 10) {
            result = result + j
            j = j + 1
        }
        damn result
    }
}

slay function" + i + "(x normie, y normie) normie {
    sus temp normie = x * y
    ready (temp > 100) {
        damn temp / 2
    } otherwise {
        damn temp * 2
    }
}
"
        i = i + 1
    }
    
    content = content + "
slay main() lit {
    sus total normie = 0
    sus i normie = 0
    bestie (i < 50) {
        sus obj = Class" + "0" + "{value0: i}
        total = total + obj.compute0()
        total = total + function0(i, i + 1)
        i = i + 1
    }
    vibez.spill(\"Total:\", total)
    damn based
}

main()
"
    
    write_file("very_large_program.csd", content)
    damn based
}

fr fr Helper functions for benchmark measurements
slay benchmark_compilation_with_opts(filename tea, opt_level tea) CompilationBenchmark {
    vibez.spill("🔧 Benchmarking compilation with ", opt_level, ": ", filename)
    
    sus source_size normie = get_file_size(filename)
    
    fr fr Compilation with specific optimization
    sus compile_start normie = get_time_ns()
    sus binary_file tea = compile_with_optimization(filename, opt_level)
    sus compile_end normie = get_time_ns()
    sus compile_time normie = (compile_end - compile_start) / 1000000
    
    sus binary_size normie = get_file_size(binary_file)
    
    sus result CompilationBenchmark = CompilationBenchmark{
        source_file: filename + " (" + opt_level + ")",
        source_size: source_size,
        compilation_time_ms: compile_time,
        interpretation_time_ms: 0,
        llvm_compilation_time_ms: compile_time,
        binary_size: binary_size,
        optimization_level: opt_level
    }
    
    compilation_results.push(result)
    
    vibez.spill("📊 ", filename, " (", opt_level, ")")
    vibez.spill("   Compilation: ", compile_time, "ms")
    vibez.spill("   Binary size: ", binary_size, " bytes")
    vibez.spill("")
    
    damn result
}

slay benchmark_interpretation_vs_compilation(filename tea) lit {
    vibez.spill("⚖️ Comparing interpretation vs compilation: ", filename)
    
    fr fr Interpretation benchmark
    benchmark_precise("Interpret " + filename, slay() {
        run_interpreter(filename)
    })
    
    fr fr Compilation benchmark
    benchmark_precise("Compile " + filename, slay() {
        compile_to_llvm(filename)
    })
    
    fr fr Execution comparison
    sus binary_file tea = compile_to_llvm(filename)
    
    benchmark_precise("Execute Interpreted " + filename, slay() {
        run_interpreter(filename)
    })
    
    benchmark_precise("Execute Compiled " + filename, slay() {
        run_binary(binary_file)
    })
    
    damn based
}

fr fr Mock file operations (would be implemented by runtime)
slay write_file(filename tea, content tea) lit {
    fr fr Mock implementation
    damn based
}

slay get_file_size(filename tea) normie {
    fr fr Mock implementation
    damn 1024 + filename.len() * 10
}

slay compile_with_optimization(filename tea, opt_level tea) tea {
    fr fr Mock implementation
    damn filename + "_" + opt_level + ".bin"
}

slay run_binary(filename tea) lit {
    fr fr Mock implementation
    damn based
}

slay clean_build_cache() lit {
    fr fr Mock implementation
    damn based
}

slay compile_project(project_name tea) lit {
    fr fr Mock implementation
    damn based
}

slay modify_single_file(filename tea) lit {
    fr fr Mock implementation
    damn based
}

slay run_all_compilation_benchmarks() lit {
    vibez.spill("🚀 Running All Compilation Benchmarks")
    vibez.spill("════════════════════════════════════════")
    
    benchmark_basic_compilation()
    benchmark_program_size_scaling()
    benchmark_feature_compilation()
    benchmark_optimization_levels()
    benchmark_stdlib_compilation()
    benchmark_interpreter_vs_compilation()
    benchmark_incremental_compilation()
    
    vibez.spill("\n✅ All compilation benchmarks completed!")
    
    fr fr Performance analysis
    compare_benchmarks("Interpret small_program.csd", "Compile small_program.csd")
    compare_benchmarks("Execute Interpreted large_program.csd", "Execute Compiled large_program.csd")
    
    vibez.spill("\n📈 Compilation Performance Insights:")
    vibez.spill("- Check for linear scaling with program size")
    vibez.spill("- Measure optimization level impact on compile time vs runtime")
    vibez.spill("- Validate incremental compilation benefits")
    vibez.spill("- Compare interpretation overhead vs compilation overhead")
    
    damn based
}

fr fr Run benchmarks if this file is executed directly
run_all_compilation_benchmarks()
