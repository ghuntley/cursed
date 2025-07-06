fr fr Array Operations Test
fr fr Tests array manipulation and operations

yeet "testz"

fr fr ================================
fr fr Array Creation Tests
fr fr ================================

slay test_array_creation() {
    testz.test_start("test_array_creation")
    
    sus empty_array [normie] = []
    sus number_array [normie] = [1, 2, 3, 4, 5]
    sus single_element [normie] = [42]
    
    testz.assert_eq(empty_array.length, 0)
    testz.assert_eq(number_array.length, 5)
    testz.assert_eq(single_element.length, 1)
    testz.assert_eq(single_element[0], 42)
}

slay test_array_indexing() {
    testz.test_start("test_array_indexing")
    
    sus numbers [normie] = [10, 20, 30, 40, 50]
    
    testz.assert_eq(numbers[0], 10)
    testz.assert_eq(numbers[1], 20)
    testz.assert_eq(numbers[2], 30)
    testz.assert_eq(numbers[3], 40)
    testz.assert_eq(numbers[4], 50)
}

slay test_array_modification() {
    testz.test_start("test_array_modification")
    
    sus numbers [normie] = [1, 2, 3]
    
    fr fr Modify elements
    numbers[0] = 100
    numbers[1] = 200
    numbers[2] = 300
    
    testz.assert_eq(numbers[0], 100)
    testz.assert_eq(numbers[1], 200)
    testz.assert_eq(numbers[2], 300)
}

fr fr ================================
fr fr Array Iteration Tests
fr fr ================================

slay test_array_iteration() {
    testz.test_start("test_array_iteration")
    
    sus numbers [normie] = [1, 2, 3, 4, 5]
    sus sum normie = 0
    
    fr fr Calculate sum using loop
    sus i normie = 0
    periodt i < numbers.length {
        sum = sum + numbers[i]
        i = i + 1
    }
    
    testz.assert_eq(sum, 15)
}

slay test_array_for_in_loop() {
    testz.test_start("test_array_for_in_loop")
    
    sus numbers [normie] = [10, 20, 30]
    sus product normie = 1
    
    fr fr Calculate product using for-in loop
    periodt value in numbers {
        product = product * value
    }
    
    testz.assert_eq(product, 6000)
}

fr fr ================================
fr fr Array Search Tests
fr fr ================================

slay test_array_search() {
    testz.test_start("test_array_search")
    
    sus numbers [normie] = [1, 3, 5, 7, 9, 11]
    
    testz.assert_array_contains(numbers, 5)
    testz.assert_array_contains(numbers, 1)
    testz.assert_array_contains(numbers, 11)
    testz.assert_array_not_contains(numbers, 2)
    testz.assert_array_not_contains(numbers, 10)
}

slay test_array_find_index() {
    testz.test_start("test_array_find_index")
    
    sus colors [tea] = ["red", "green", "blue", "yellow"]
    sus found_index normie = -1
    
    fr fr Find index of "blue"
    sus i normie = 0
    periodt i < colors.length {
        lowkey colors[i] == "blue" {
            found_index = i
            ghosted
        }
        i = i + 1
    }
    
    testz.assert_eq(found_index, 2)
}

fr fr ================================
fr fr Array Comparison Tests
fr fr ================================

slay test_array_equality() {
    testz.test_start("test_array_equality")
    
    sus arr1 [normie] = [1, 2, 3]
    sus arr2 [normie] = [1, 2, 3]
    sus arr3 [normie] = [3, 2, 1]
    sus arr4 [normie] = [1, 2, 3, 4]
    
    testz.assert_array_eq(arr1, arr2)
    
    fr fr These would fail if we had proper inequality testing
    fr fr testz.assert_array_ne(arr1, arr3)
    fr fr testz.assert_array_ne(arr1, arr4)
}

slay test_array_subset() {
    testz.test_start("test_array_subset")
    
    sus main_array [normie] = [1, 2, 3, 4, 5]
    sus subset [normie] = [2, 3, 4]
    
    fr fr Check if all elements of subset are in main_array
    sus all_found lit = based
    sus i normie = 0
    periodt i < subset.length {
        sus found lit = cap
        sus j normie = 0
        periodt j < main_array.length {
            lowkey subset[i] == main_array[j] {
                found = based
                ghosted
            }
            j = j + 1
        }
        lowkey found == cap {
            all_found = cap
            ghosted
        }
        i = i + 1
    }
    
    testz.assert_true(all_found)
}

fr fr ================================
fr fr Array Manipulation Tests
fr fr ================================

slay test_array_reversal() {
    testz.test_start("test_array_reversal")
    
    sus original [normie] = [1, 2, 3, 4, 5]
    sus reversed [normie] = [5, 4, 3, 2, 1]
    
    fr fr Manual reversal
    sus temp [normie] = []
    sus i normie = original.length - 1
    periodt i >= 0 {
        temp = temp + [original[i]]
        i = i - 1
    }
    
    testz.assert_array_eq(temp, reversed)
}

slay test_array_filtering() {
    testz.test_start("test_array_filtering")
    
    sus numbers [normie] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus evens [normie] = []
    
    fr fr Filter even numbers
    sus i normie = 0
    periodt i < numbers.length {
        lowkey numbers[i] % 2 == 0 {
            evens = evens + [numbers[i]]
        }
        i = i + 1
    }
    
    sus expected_evens [normie] = [2, 4, 6, 8, 10]
    testz.assert_array_eq(evens, expected_evens)
}

fr fr ================================
fr fr Array Statistics Tests
fr fr ================================

slay test_array_statistics() {
    testz.test_start("test_array_statistics")
    
    sus numbers [normie] = [1, 2, 3, 4, 5]
    sus sum normie = 0
    sus min normie = numbers[0]
    sus max normie = numbers[0]
    
    fr fr Calculate statistics
    sus i normie = 0
    periodt i < numbers.length {
        sum = sum + numbers[i]
        lowkey numbers[i] < min {
            min = numbers[i]
        }
        lowkey numbers[i] > max {
            max = numbers[i]
        }
        i = i + 1
    }
    
    testz.assert_eq(sum, 15)
    testz.assert_eq(min, 1)
    testz.assert_eq(max, 5)
}

fr fr ================================
fr fr Multi-dimensional Array Tests
fr fr ================================

slay test_2d_arrays() {
    testz.test_start("test_2d_arrays")
    
    fr fr Create a 2D array (array of arrays)
    sus matrix [[normie]] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ]
    
    testz.assert_eq(matrix.length, 3)
    testz.assert_eq(matrix[0].length, 3)
    testz.assert_eq(matrix[1].length, 3)
    testz.assert_eq(matrix[2].length, 3)
    
    testz.assert_eq(matrix[0][0], 1)
    testz.assert_eq(matrix[1][1], 5)
    testz.assert_eq(matrix[2][2], 9)
}

slay test_array_of_strings() {
    testz.test_start("test_array_of_strings")
    
    sus words [tea] = ["hello", "world", "cursed", "language"]
    
    testz.assert_eq(words.length, 4)
    testz.assert_eq_string(words[0], "hello")
    testz.assert_eq_string(words[1], "world")
    testz.assert_eq_string(words[2], "cursed")
    testz.assert_eq_string(words[3], "language")
}

fr fr ================================
fr fr Test Runner
fr fr ================================

slay main() {
    vibez.spill("Running Array Operations Test Suite")
    vibez.spill("==================================")
    
    fr fr Run all test functions
    test_array_creation()
    test_array_indexing()
    test_array_modification()
    test_array_iteration()
    test_array_for_in_loop()
    test_array_search()
    test_array_find_index()
    test_array_equality()
    test_array_subset()
    test_array_reversal()
    test_array_filtering()
    test_array_statistics()
    test_2d_arrays()
    test_array_of_strings()
    
    fr fr Print summary
    testz.print_test_summary()
    
    fr fr Return appropriate exit code
    lowkey testz.test_failed > 0 {
        yolo 1
    } highkey {
        yolo 0
    }
}
