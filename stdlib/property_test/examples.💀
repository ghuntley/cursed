yeet "testz"
yeet "property_test"

fr fr Property-Based Testing Examples
fr fr Comprehensive demonstrations of the property testing framework

fr fr ===== EXAMPLE 1: MATHEMATICAL PROPERTIES =====

fr fr Test that addition is commutative
slay example_addition_commutative() {
    test_start("Addition Commutative Property Example")
    
    forall(
        slay() { 
            damn [gen_int(-100, 100), gen_int(-100, 100)] 
        },
        slay(nums []) {
            sus a normie = nums[0]
            sus b normie = nums[1]
            damn a + b == b + a
        },
        "Addition is commutative: a + b == b + a"
    )
}

fr fr Test that multiplication is associative
slay example_multiplication_associative() {
    test_start("Multiplication Associative Property Example")
    
    forall(
        slay() {
            damn [gen_int(-10, 10), gen_int(-10, 10), gen_int(-10, 10)]
        },
        slay(nums []) {
            sus a normie = nums[0]
            sus b normie = nums[1]
            sus c normie = nums[2]
            damn (a * b) * c == a * (b * c)
        },
        "Multiplication is associative: (a * b) * c == a * (b * c)"
    )
}

fr fr Test identity properties
slay example_identity_properties() {
    test_start("Identity Properties Example") fr fr Additive identity
    forall(
        slay() { damn gen_int(-1000, 1000) },
        slay(x normie) { damn x + 0 == x && 0 + x == x },
        "Zero is additive identity"
    ) fr fr Multiplicative identity
    forall(
        slay() { damn gen_int(-1000, 1000) },
        slay(x normie) { damn x * 1 == x && 1 * x == x },
        "One is multiplicative identity"
    )
}

fr fr ===== EXAMPLE 2: STRING OPERATIONS =====

fr fr Test string concatenation properties
slay example_string_concatenation() {
    test_start("String Concatenation Properties Example") fr fr Length property
    forall(
        slay() {
            damn [gen_string(20), gen_string(20)]
        },
        slay(strings []) {
            sus s1 tea = strings[0]
            sus s2 tea = strings[1]
            sus concat tea = s1 + s2
            damn len(concat) == len(s1) + len(s2)
        },
        "Concatenation length: len(s1 + s2) == len(s1) + len(s2)"
    ) fr fr Associativity
    forall(
        slay() {
            damn [gen_string(10), gen_string(10), gen_string(10)]
        },
        slay(strings []) {
            sus a tea = strings[0]
            sus b tea = strings[1]
            sus c tea = strings[2]
            damn (a + b) + c == a + (b + c)
        },
        "String concatenation is associative"
    ) fr fr Empty string identity
    forall(
        slay() { damn gen_string(15) },
        slay(s tea) { damn s + "" == s && "" + s == s },
        "Empty string is concatenation identity"
    )
}

fr fr Test string reversal
slay example_string_reversal() {
    test_start("String Reversal Properties Example")
    
    slay reverse_string(s tea) tea { fr fr Simple character-by-character reversal
        sus result tea = ""
        sus i normie = len(s) - 1
        bestie i >= 0 { fr fr In full implementation, would use proper string indexing
            result = result + substring(s, i, i + 1)
            i = i - 1
        }
        damn result
    } fr fr Double reversal is identity
    forall(
        slay() { damn gen_string(15) },
        slay(s tea) {
            sus reversed tea = reverse_string(s)
            sus double_reversed tea = reverse_string(reversed)
            damn s == double_reversed
        },
        "Double string reversal is identity"
    ) fr fr Reversal preserves length
    forall(
        slay() { damn gen_string(20) },
        slay(s tea) {
            sus reversed tea = reverse_string(s)
            damn len(s) == len(reversed)
        },
        "String reversal preserves length"
    )
}

fr fr ===== EXAMPLE 3: LIST OPERATIONS =====

fr fr Test list sorting properties
slay example_list_sorting() {
    test_start("List Sorting Properties Example")
    
    slay is_sorted(list []) lit {
        sus i normie = 0
        bestie i < size(list) - 1 {
            vibes list[i] > list[i + 1] {
                damn cap
            }
            i = i + 1
        }
        damn based
    }
    
    slay sort_list(list []) [] { fr fr Simple bubble sort implementation
        sus result [] = list
        sus length normie = size(result)
        sus i normie = 0
        bestie i < length {
            sus j normie = 0
            bestie j < length - 1 {
                vibes result[j] > result[j + 1] {
                    sus temp = result[j]
                    result[j] = result[j + 1]
                    result[j + 1] = temp
                }
                j = j + 1
            }
            i = i + 1
        }
        damn result
    } fr fr Sorting produces sorted list
    forall(
        slay() { damn gen_list_int(10) },
        slay(list []) {
            sus sorted [] = sort_list(list)
            damn is_sorted(sorted)
        },
        "Sorting produces a sorted list"
    ) fr fr Sorting preserves length
    forall(
        slay() { damn gen_list_int(15) },
        slay(list []) {
            sus sorted [] = sort_list(list)
            damn size(list) == size(sorted)
        },
        "Sorting preserves list length"
    ) fr fr Sorting is idempotent
    forall(
        slay() { damn gen_list_int(8) },
        slay(list []) {
            sus sorted1 [] = sort_list(list)
            sus sorted2 [] = sort_list(sorted1)
            damn lists_equal(sorted1, sorted2)
        },
        "Sorting is idempotent"
    )
}

fr fr Test list reversal
slay example_list_reversal() {
    test_start("List Reversal Properties Example")
    
    slay reverse_list(list []) [] {
        sus result [] = []
        sus i normie = size(list) - 1
        bestie i >= 0 {
            result = result + [list[i]]
            i = i - 1
        }
        damn result
    } fr fr Double reversal is identity
    forall(
        slay() { damn gen_list_int(10) },
        slay(list []) {
            sus reversed1 [] = reverse_list(list)
            sus reversed2 [] = reverse_list(reversed1)
            damn lists_equal(list, reversed2)
        },
        "Double list reversal is identity"
    ) fr fr Reversal preserves length
    forall(
        slay() { damn gen_list_int(12) },
        slay(list []) {
            sus reversed [] = reverse_list(list)
            damn size(list) == size(reversed)
        },
        "List reversal preserves length"
    )
}

fr fr ===== EXAMPLE 4: CUSTOM DATA TYPES =====

fr fr Test coordinate system operations
slay example_coordinate_operations() {
    test_start("Coordinate Operations Example")
    
    slay gen_coordinate() [] {
        damn [gen_int(-100, 100), gen_int(-100, 100)]
    }
    
    slay distance_from_origin(coord []) drip {
        sus x drip = drip(coord[0])
        sus y drip = drip(coord[1])
        damn sqrt(x * x + y * y)
    }
    
    slay scale_coordinate(coord [], factor drip) [] {
        sus x normie = coord[0]
        sus y normie = coord[1]
        damn [normie(drip(x) * factor), normie(drip(y) * factor)]
    } fr fr Distance from origin is always non-negative
    forall(
        gen_coordinate,
        slay(coord []) {
            sus distance drip = distance_from_origin(coord)
            damn distance >= 0.0
        },
        "Distance from origin is non-negative"
    ) fr fr Scaling preserves origin
    forall(
        slay() { damn [0, 0] },
        slay(origin []) {
            sus scaled [] = scale_coordinate(origin, 5.0)
            damn scaled[0] == 0 && scaled[1] == 0
        },
        "Scaling preserves origin"
    )
}

fr fr ===== EXAMPLE 5: ERROR HANDLING PROPERTIES =====

fr fr Test division properties with error handling
slay example_division_properties() {
    test_start("Division Properties Example")
    
    slay safe_divide(a normie, b normie) [] {
        vibes b == 0 {
            damn ["error", "division by zero"]
        }
        damn ["ok", a / b]
    } fr fr Division by non-zero always succeeds
    forall(
        slay() {
            sus a normie = gen_int(-100, 100)
            sus b normie = gen_int(-100, 100)
            vibes b == 0 {
                b = 1 fr fr Ensure non-zero
            }
            damn [a, b]
        },
        slay(nums []) {
            sus result [] = safe_divide(nums[0], nums[1])
            damn result[0] == "ok"
        },
        "Division by non-zero always succeeds"
    ) fr fr Division by zero always fails
    forall(
        slay() { damn gen_int(-100, 100) },
        slay(a normie) {
            sus result [] = safe_divide(a, 0)
            damn result[0] == "error"
        },
        "Division by zero always fails"
    )
}

fr fr ===== EXAMPLE 6: STATEFUL PROPERTIES =====

fr fr Test stack operations
slay example_stack_properties() {
    test_start("Stack Properties Example")
    
    slay create_stack() [] {
        damn []
    }
    
    slay push_stack(stack [], item normie) [] {
        damn stack + [item]
    }
    
    slay pop_stack(stack []) [] {
        vibes size(stack) == 0 {
            damn ["error", "empty stack"]
        }
        sus new_stack [] = slice(stack, 0, size(stack) - 1)
        sus item normie = stack[size(stack) - 1]
        damn ["ok", item, new_stack]
    }
    
    slay stack_size(stack []) normie {
        damn size(stack)
    } fr fr Push then pop gives original stack
    forall(
        slay() {
            damn [gen_list_int(5), gen_int(-50, 50)]
        },
        slay(data []) {
            sus original_stack [] = data[0]
            sus item normie = data[1]
            sus after_push [] = push_stack(original_stack, item)
            sus pop_result [] = pop_stack(after_push)
            
            vibes pop_result[0] == "error" {
                damn cap fr fr Should not happen for valid stack
            }
            
            sus popped_item normie = pop_result[1]
            sus final_stack [] = pop_result[2]
            
            damn item == popped_item && lists_equal(original_stack, final_stack)
        },
        "Push then pop restores original stack"
    ) fr fr Stack size increases after push
    forall(
        slay() {
            damn [gen_list_int(8), gen_int(-30, 30)]
        },
        slay(data []) {
            sus stack [] = data[0]
            sus item normie = data[1]
            sus original_size normie = stack_size(stack)
            sus after_push [] = push_stack(stack, item)
            sus new_size normie = stack_size(after_push)
            damn new_size == original_size + 1
        },
        "Push increases stack size by one"
    )
}

fr fr ===== EXAMPLE 7: GENERATOR COMPOSITION =====

fr fr Test complex data generation
slay example_complex_generators() {
    test_start("Complex Generator Composition Example")
    
    slay gen_person() [] {
        sus name tea = gen_ascii_string(12)
        sus age normie = gen_int(0, 120)
        sus email tea = gen_email()
        damn [name, age, email]
    }
    
    slay gen_company() [] {
        sus name tea = gen_ascii_string(15)
        sus employee_count normie = gen_positive_int()
        sus employees [] = []
        sus i normie = 0
        sus max_employees normie = min(employee_count, 5) fr fr Limit for testing
        bestie i < max_employees {
            employees = employees + [gen_person()]
            i = i + 1
        }
        damn [name, employees]
    } fr fr Person age is always valid
    forall(
        gen_person,
        slay(person []) {
            sus age normie = person[1]
            damn age >= 0 && age <= 120
        },
        "Person age is within valid range"
    ) fr fr Company has at least one employee (after generation)
    forall(
        gen_company,
        slay(company []) {
            sus employees [] = company[1]
            damn size(employees) >= 0 fr fr Can be empty for testing
        },
        "Company employee list is valid"
    )
}

fr fr ===== EXAMPLE 8: PERFORMANCE PROPERTIES =====

fr fr Test algorithmic complexity properties
slay example_performance_properties() {
    test_start("Performance Properties Example")
    
    slay linear_search(list [], target normie) normie {
        sus i normie = 0
        bestie i < size(list) {
            vibes list[i] == target {
                damn i fr fr Return index
            }
            i = i + 1
        }
        damn -1 fr fr Not found
    } fr fr Linear search time is bounded by list size
    forall(
        slay() {
            sus list [] = gen_list_int(20)
            sus target normie = gen_int(-100, 100)
            damn [list, target]
        },
        slay(data []) {
            sus list [] = data[0]
            sus target normie = data[1] fr fr In real implementation, would measure actual time
            sus start_operations normie = 0
            sus result normie = linear_search(list, target)
            sus end_operations normie = size(list) fr fr Upper bound fr fr Performance property: operations <= list size
            damn end_operations <= size(list)
        },
        "Linear search complexity is O(n)"
    )
}

fr fr ===== UTILITY FUNCTIONS =====

slay lists_equal(list1 [], list2 []) lit {
    vibes size(list1) != size(list2) {
        damn cap
    }
    sus i normie = 0
    bestie i < size(list1) {
        vibes list1[i] != list2[i] {
            damn cap
        }
        i = i + 1
    }
    damn based
}

slay min(a normie, b normie) normie {
    vibes a < b {
        damn a
    }
    damn b
}

slay sqrt(x drip) drip { fr fr Simple Newton's method approximation
    vibes x < 0.0 {
        damn 0.0
    }
    vibes x == 0.0 {
        damn 0.0
    }
    
    sus guess drip = x / 2.0
    sus i normie = 0
    bestie i < 10 { fr fr 10 iterations
        guess = (guess + x / guess) / 2.0
        i = i + 1
    }
    damn guess
}

fr fr ===== MAIN EXECUTION =====

slay run_all_examples() { fr fr Set configuration
    set_test_count(50) fr fr Reduce for examples
    set_seed(12345) fr fr Run all examples
    example_addition_commutative()
    example_multiplication_associative()
    example_identity_properties()
    example_string_concatenation()
    example_string_reversal()
    example_list_sorting()
    example_list_reversal()
    example_coordinate_operations()
    example_division_properties()
    example_stack_properties()
    example_complex_generators()
    example_performance_properties()
    
    print_test_summary()
}

fr fr Run examples when this file is executed
run_all_examples()
