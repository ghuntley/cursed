fr fr Array Operations Performance Benchmark Suite

yeet "benchz"
yeet "testz"
yeet "arrayz"

slay benchmark_basic_array_operations() lit {
    benchmark_suite_start("Basic Array Operations")
    
    fr fr Array creation and initialization
    benchmark_precise("Empty Array Creation", slay() {
        sus arr []normie = []
    })
    
    benchmark_precise("Small Array Creation", slay() {
        sus arr []normie = [1, 2, 3, 4, 5]
    })
    
    benchmark_precise("Medium Array Creation", slay() {
        sus arr []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
    })
    
    benchmark_precise("Large Array Creation", slay() {
        sus arr []normie = []
        sus i normie = 0
        bestie (i < 100) {
            arr.push(i)
            i = i + 1
        }
    })
    
    fr fr Array length operations
    benchmark_precise("Array Length Small", slay() {
        sus arr []normie = [1, 2, 3, 4, 5]
        sus length normie = len(arr)
    })
    
    benchmark_precise("Array Length Large", slay() {
        sus arr []normie = []
        sus i normie = 0
        bestie (i < 1000) {
            arr.push(i)
            i = i + 1
        }
        sus length normie = len(arr)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_array_access() lit {
    benchmark_suite_start("Array Access Operations")
    
    fr fr Setup test arrays
    sus small_array []normie = [1, 2, 3, 4, 5]
    sus medium_array []normie = []
    sus i normie = 0
    bestie (i < 100) {
        medium_array.push(i)
        i = i + 1
    }
    
    sus large_array []normie = []
    i = 0
    bestie (i < 1000) {
        large_array.push(i)
        i = i + 1
    }
    
    fr fr Index access benchmarks
    benchmark_precise("Access First Element Small", slay() {
        sus value normie = small_array[0]
    })
    
    benchmark_precise("Access Last Element Small", slay() {
        sus value normie = small_array[4]
    })
    
    benchmark_precise("Access Middle Element Medium", slay() {
        sus value normie = medium_array[50]
    })
    
    benchmark_precise("Access First Element Large", slay() {
        sus value normie = large_array[0]
    })
    
    benchmark_precise("Access Last Element Large", slay() {
        sus value normie = large_array[999]
    })
    
    benchmark_precise("Access Random Elements", slay() {
        sus value1 normie = large_array[123]
        sus value2 normie = large_array[456]
        sus value3 normie = large_array[789]
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_array_modification() lit {
    benchmark_suite_start("Array Modification")
    
    fr fr Array element assignment
    benchmark_precise("Set First Element", slay() {
        sus arr []normie = [1, 2, 3, 4, 5]
        arr[0] = 99
    })
    
    benchmark_precise("Set Last Element", slay() {
        sus arr []normie = [1, 2, 3, 4, 5]
        arr[4] = 99
    })
    
    benchmark_precise("Set Multiple Elements", slay() {
        sus arr []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        arr[0] = 99
        arr[5] = 88
        arr[9] = 77
    })
    
    fr fr Array append operations
    benchmark_precise("Push Single Element", slay() {
        sus arr []normie = [1, 2, 3]
        arr.push(4)
    })
    
    benchmark_precise("Push Multiple Elements", slay() {
        sus arr []normie = [1, 2, 3]
        arr.push(4)
        arr.push(5)
        arr.push(6)
    })
    
    benchmark_precise("Push to Large Array", slay() {
        sus arr []normie = []
        sus i normie = 0
        bestie (i < 100) {
            arr.push(i)
            i = i + 1
        }
        arr.push(100)
    })
    
    fr fr Array remove operations
    benchmark_precise("Pop Last Element", slay() {
        sus arr []normie = [1, 2, 3, 4, 5]
        sus value normie = arr.pop()
    })
    
    benchmark_precise("Remove First Element", slay() {
        sus arr []normie = [1, 2, 3, 4, 5]
        remove_at(arr, 0)
    })
    
    benchmark_precise("Remove Middle Element", slay() {
        sus arr []normie = [1, 2, 3, 4, 5]
        remove_at(arr, 2)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_array_iteration() lit {
    benchmark_suite_start("Array Iteration")
    
    fr fr Setup test arrays
    sus small_array []normie = [1, 2, 3, 4, 5]
    sus medium_array []normie = []
    sus i normie = 0
    bestie (i < 100) {
        medium_array.push(i)
        i = i + 1
    }
    
    sus large_array []normie = []
    i = 0
    bestie (i < 1000) {
        large_array.push(i)
        i = i + 1
    }
    
    fr fr Index-based iteration
    benchmark_precise("Iterate Small Array Index", slay() {
        sus sum normie = 0
        sus i normie = 0
        bestie (i < len(small_array)) {
            sum = sum + small_array[i]
            i = i + 1
        }
    })
    
    benchmark_precise("Iterate Medium Array Index", slay() {
        sus sum normie = 0
        sus i normie = 0
        bestie (i < len(medium_array)) {
            sum = sum + medium_array[i]
            i = i + 1
        }
    })
    
    benchmark_precise("Iterate Large Array Index", slay() {
        sus sum normie = 0
        sus i normie = 0
        bestie (i < len(large_array)) {
            sum = sum + large_array[i]
            i = i + 1
        }
    })
    
    fr fr For-each style iteration
    benchmark_precise("For-Each Small Array", slay() {
        sus sum normie = 0
        bestie element in small_array {
            sum = sum + element
        }
    })
    
    benchmark_precise("For-Each Medium Array", slay() {
        sus sum normie = 0
        bestie element in medium_array {
            sum = sum + element
        }
    })
    
    benchmark_precise("For-Each Large Array", slay() {
        sus sum normie = 0
        bestie element in large_array {
            sum = sum + element
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_array_searching() lit {
    benchmark_suite_start("Array Searching")
    
    fr fr Setup test data
    sus small_array []normie = [1, 2, 3, 4, 5]
    sus medium_array []normie = []
    sus i normie = 0
    bestie (i < 100) {
        medium_array.push(i)
        i = i + 1
    }
    
    sus large_array []normie = []
    i = 0
    bestie (i < 1000) {
        large_array.push(i)
        i = i + 1
    }
    
    fr fr Linear search operations
    benchmark_precise("Find First Element Small", slay() {
        sus index normie = find_element(small_array, 1)
    })
    
    benchmark_precise("Find Last Element Small", slay() {
        sus index normie = find_element(small_array, 5)
    })
    
    benchmark_precise("Find Middle Element Medium", slay() {
        sus index normie = find_element(medium_array, 50)
    })
    
    benchmark_precise("Find First Element Large", slay() {
        sus index normie = find_element(large_array, 0)
    })
    
    benchmark_precise("Find Last Element Large", slay() {
        sus index normie = find_element(large_array, 999)
    })
    
    benchmark_precise("Find Non-existent Element", slay() {
        sus index normie = find_element(large_array, -1)
    })
    
    fr fr Contains operations
    benchmark_precise("Contains Existing Element", slay() {
        sus found lit = contains_element(large_array, 500)
    })
    
    benchmark_precise("Contains Non-existing Element", slay() {
        sus found lit = contains_element(large_array, -1)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_array_sorting() lit {
    benchmark_suite_start("Array Sorting")
    
    fr fr Create unsorted test arrays
    slay create_random_array(size normie) []normie {
        sus arr []normie = []
        sus i normie = 0
        bestie (i < size) {
            fr fr Simple pseudo-random number generation
            sus random_val normie = (i * 73 + 29) % 1000
            arr.push(random_val)
            i = i + 1
        }
        damn arr
    }
    
    slay create_reverse_sorted_array(size normie) []normie {
        sus arr []normie = []
        sus i normie = size - 1
        bestie (i >= 0) {
            arr.push(i)
            i = i - 1
        }
        damn arr
    }
    
    fr fr Sort benchmarks
    benchmark_precise("Sort Small Random Array", slay() {
        sus arr []normie = create_random_array(10)
        sort_array(arr)
    })
    
    benchmark_precise("Sort Medium Random Array", slay() {
        sus arr []normie = create_random_array(100)
        sort_array(arr)
    })
    
    benchmark_precise("Sort Large Random Array", slay() {
        sus arr []normie = create_random_array(1000)
        sort_array(arr)
    })
    
    benchmark_precise("Sort Already Sorted Array", slay() {
        sus arr []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        sort_array(arr)
    })
    
    benchmark_precise("Sort Reverse Sorted Array", slay() {
        sus arr []normie = create_reverse_sorted_array(100)
        sort_array(arr)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_array_transformations() lit {
    benchmark_suite_start("Array Transformations")
    
    fr fr Setup test data
    sus test_array []normie = []
    sus i normie = 0
    bestie (i < 100) {
        test_array.push(i)
        i = i + 1
    }
    
    fr fr Map operations
    benchmark_precise("Map Double Values", slay() {
        sus result []normie = map_array(test_array, slay(x normie) normie { damn x * 2 })
    })
    
    benchmark_precise("Map Square Values", slay() {
        sus result []normie = map_array(test_array, slay(x normie) normie { damn x * x })
    })
    
    fr fr Filter operations
    benchmark_precise("Filter Even Numbers", slay() {
        sus result []normie = filter_array(test_array, slay(x normie) lit { damn x % 2 == 0 })
    })
    
    benchmark_precise("Filter Large Numbers", slay() {
        sus result []normie = filter_array(test_array, slay(x normie) lit { damn x > 50 })
    })
    
    fr fr Reduce operations
    benchmark_precise("Reduce Sum", slay() {
        sus result normie = reduce_array(test_array, 0, slay(acc normie, x normie) normie { damn acc + x })
    })
    
    benchmark_precise("Reduce Product", slay() {
        sus small_array []normie = [1, 2, 3, 4, 5]
        sus result normie = reduce_array(small_array, 1, slay(acc normie, x normie) normie { damn acc * x })
    })
    
    fr fr Array concatenation
    benchmark_precise("Concatenate Small Arrays", slay() {
        sus arr1 []normie = [1, 2, 3]
        sus arr2 []normie = [4, 5, 6]
        sus result []normie = concat_arrays(arr1, arr2)
    })
    
    benchmark_precise("Concatenate Large Arrays", slay() {
        sus arr1 []normie = []
        sus arr2 []normie = []
        sus i normie = 0
        bestie (i < 500) {
            arr1.push(i)
            arr2.push(i + 500)
            i = i + 1
        }
        sus result []normie = concat_arrays(arr1, arr2)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_multidimensional_arrays() lit {
    benchmark_suite_start("Multidimensional Arrays")
    
    fr fr 2D array operations
    benchmark_precise("2D Array Creation", slay() {
        sus matrix [][]normie = []
        sus i normie = 0
        bestie (i < 10) {
            sus row []normie = []
            sus j normie = 0
            bestie (j < 10) {
                row.push(i * 10 + j)
                j = j + 1
            }
            matrix.push(row)
            i = i + 1
        }
    })
    
    benchmark_precise("2D Array Access", slay() {
        sus matrix [][]normie = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        sus value normie = matrix[1][1]
    })
    
    benchmark_precise("2D Array Iteration", slay() {
        sus matrix [][]normie = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        sus sum normie = 0
        bestie row in matrix {
            bestie element in row {
                sum = sum + element
            }
        }
    })
    
    fr fr Matrix operations
    benchmark_precise("Matrix Addition", slay() {
        sus m1 [][]normie = [[1, 2], [3, 4]]
        sus m2 [][]normie = [[5, 6], [7, 8]]
        sus result [][]normie = add_matrices(m1, m2)
    })
    
    generate_benchmark_report()
    damn based
}

slay run_all_array_benchmarks() lit {
    vibez.spill("🚀 Running All Array Benchmarks")
    vibez.spill("═══════════════════════════════════")
    
    benchmark_basic_array_operations()
    benchmark_array_access()
    benchmark_array_modification()
    benchmark_array_iteration()
    benchmark_array_searching()
    benchmark_array_sorting()
    benchmark_array_transformations()
    benchmark_multidimensional_arrays()
    
    vibez.spill("\n✅ All array benchmarks completed!")
    
    fr fr Performance analysis
    compare_benchmarks("Access First Element Small", "Access Last Element Large")
    compare_benchmarks("Iterate Small Array Index", "For-Each Small Array")
    compare_benchmarks("Sort Small Random Array", "Sort Large Random Array")
    compare_benchmarks("Find First Element Large", "Find Last Element Large")
    
    damn based
}

fr fr Run benchmarks if this file is executed directly
run_all_array_benchmarks()
