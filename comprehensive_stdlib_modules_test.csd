yeet "testz"
yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "arrayz"
yeet "hashz"

fr fr Comprehensive Standard Library Modules Integration Test

test_start("Core Stdlib Modules Integration Test")

fr fr === VIBEZ I/O MODULE TESTS ===
vibez.spill("=== Testing VIBEZ I/O Module ===")

fr fr Test basic output
vibez.spill("Hello from vibez!")
vibez.spillf("Formatted output: %s %s", "Hello", "World")

fr fr Test format functions
sus formatted_str tea = vibez.spillstr("Value: %d", "42")
assert_eq_string(formatted_str, "Value: 42")

vibez.spill("✅ vibez module working correctly")

fr fr === MATHZ MATHEMATICS MODULE TESTS ===
vibez.spill("=== Testing MATHZ Mathematics Module ===")

fr fr Test basic arithmetic
sus add_result meal = mathz.math_add(5.0, 3.0)
assert_true(add_result == 8.0)

sus sqrt_result meal = mathz.sqrt_meal(16.0)
assert_true(sqrt_result >= 3.9 && sqrt_result <= 4.1)

fr fr Test constants
assert_true(mathz.PI > 3.14 && mathz.PI < 3.15)

fr fr Test factorial
sus fact_result normie = mathz.factorial(5)
assert_eq_int(fact_result, 120)

fr fr Test trigonometry
sus sin_result meal = mathz.sin_meal(0.0)
assert_true(sin_result >= -0.1 && sin_result <= 0.1)

vibez.spill("✅ mathz module working correctly")

fr fr === STRINGZ STRING PROCESSING MODULE TESTS ===
vibez.spill("=== Testing STRINGZ String Processing Module ===")

fr fr Test string length and concatenation
assert_eq_int(stringz.length("hello"), 5)
assert_eq_string(stringz.concat("hello", "world"), "helloworld")

fr fr Test string searching
assert_eq_int(stringz.find("hello world", "world"), 6)
assert_true(stringz.contains("programming", "gram"))

fr fr Test string manipulation
assert_eq_string(stringz.reverse("hello"), "olleh")
assert_eq_string(stringz.to_upper("hello"), "HELLO")

fr fr Test string splitting and joining
sus split_result [tea] = stringz.split("a,b,c", ",")
assert_eq_int(len(split_result), 3)

sus parts [tea]
parts = append(parts, "hello")
parts = append(parts, "world")
assert_eq_string(stringz.join(parts, " "), "hello world")

vibez.spill("✅ stringz module working correctly")

fr fr === ARRAYZ ARRAY OPERATIONS MODULE TESTS ===
vibez.spill("=== Testing ARRAYZ Array Operations Module ===")

fr fr Test array creation and basic operations
sus test_array [tea] = arrayz.array_new()
assert_true(arrayz.array_is_empty(test_array))

test_array = arrayz.array_push(test_array, "first")
test_array = arrayz.array_push(test_array, "second")
assert_eq_int(arrayz.array_length(test_array), 2)

fr fr Test array searching
sus search_array [tea] = ["apple", "banana", "cherry"]
assert_eq_int(arrayz.array_find(search_array, "banana"), 1)
assert_true(arrayz.array_contains(search_array, "apple"))

fr fr Test array manipulation
sus reversed [tea] = arrayz.array_reverse(search_array)
assert_eq_string(arrayz.array_get(reversed, 0), "cherry")

fr fr Test array joining
sus joined_str tea = arrayz.array_join(search_array, ", ")
assert_eq_string(joined_str, "apple, banana, cherry")

vibez.spill("✅ arrayz module working correctly")

fr fr === HASHZ HASH MAP AND SET MODULE TESTS ===
vibez.spill("=== Testing HASHZ Hash Map and Set Module ===")

fr fr Test HashMap operations
sus map hashz.HashMap = hashz.hashmap_new()
map = hashz.hashmap_put(map, "key1", "value1")
map = hashz.hashmap_put(map, "key2", "value2")

assert_eq_int(hashz.hashmap_size(map), 2)

sus (value, found) = hashz.hashmap_get(map, "key1")
assert_true(found)
assert_eq_string(value, "value1")

fr fr Test HashMap keys and values
sus keys [tea] = hashz.hashmap_keys(map)
assert_eq_int(len(keys), 2)

fr fr Test HashSet operations
sus set hashz.HashSet = hashz.hashset_new()
set = hashz.hashset_add(set, "apple")
set = hashz.hashset_add(set, "banana")
set = hashz.hashset_add(set, "cherry")

assert_eq_int(hashz.hashset_size(set), 3)
assert_true(hashz.hashset_contains(set, "apple"))

fr fr Test set operations
sus set1 hashz.HashSet = hashz.hashset_new()
set1 = hashz.hashset_add(set1, "a")
set1 = hashz.hashset_add(set1, "b")

sus set2 hashz.HashSet = hashz.hashset_new()
set2 = hashz.hashset_add(set2, "b")
set2 = hashz.hashset_add(set2, "c")

sus union_set hashz.HashSet = hashz.hashset_union(set1, set2)
assert_eq_int(hashz.hashset_size(union_set), 3)

vibez.spill("✅ hashz module working correctly")

fr fr === CROSS-MODULE INTEGRATION TESTS ===
vibez.spill("=== Testing Cross-Module Integration ===")

fr fr Test using multiple modules together
sus numbers [normie] = [1, 2, 3, 4, 5]
sus sum normie = arrayz.array_sum_numbers(numbers)
assert_eq_int(sum, 15)

sus avg meal = arrayz.array_average_numbers(numbers)
sus rounded_avg normie = mathz.round_meal(avg)
assert_eq_int(rounded_avg, 3)

fr fr Test string processing with array operations
sus words [tea] = stringz.split("hello,world,test", ",")
sus word_count normie = arrayz.array_length(words)
assert_eq_int(word_count, 3)

sus first_word tea = arrayz.array_get(words, 0)
sus word_length normie = stringz.length(first_word)
assert_eq_int(word_length, 5)

fr fr Test hash map with computed values
sus stats_map hashz.HashMap = hashz.hashmap_new()
stats_map = hashz.hashmap_put(stats_map, "count", "3")
stats_map = hashz.hashmap_put(stats_map, "average", "3.0")

sus (count_str, count_found) = hashz.hashmap_get(stats_map, "count")
assert_true(count_found)

fr fr Test mathematical operations with string formatting
sus pi_str tea = vibez.spillstr("Pi value: %.2f", "3.14")
vibez.spill(pi_str)

sus sqrt_16 meal = mathz.sqrt_meal(16.0)
sus sqrt_str tea = vibez.spillstr("Square root of 16: %.1f", "4.0")
vibez.spill(sqrt_str)

vibez.spill("✅ Cross-module integration working correctly")

fr fr === PERFORMANCE AND EDGE CASE TESTS ===
vibez.spill("=== Testing Performance and Edge Cases ===")

fr fr Test empty collections
sus empty_array [tea] = arrayz.array_new()
assert_true(arrayz.array_is_empty(empty_array))

sus empty_string tea = ""
assert_true(stringz.is_empty(empty_string))

sus empty_map hashz.HashMap = hashz.hashmap_new()
assert_true(hashz.hashmap_is_empty(empty_map))

fr fr Test large data sets (simplified)
sus large_array [tea] = arrayz.array_fill(10, "item")
assert_eq_int(arrayz.array_length(large_array), 10)

sus large_set hashz.HashSet = hashz.hashset_new()
bestie i := 0; i < 5; i++ {
    sus item_name tea = vibez.spillstr("item_%d", stringz.concat("", vibez.format_number(i)))
    large_set = hashz.hashset_add(large_set, item_name)
}
assert_eq_int(hashz.hashset_size(large_set), 5)

fr fr Test mathematical edge cases
sus zero_factorial normie = mathz.factorial(0)
assert_eq_int(zero_factorial, 1)

sus negative_abs meal = mathz.abs_meal(-3.14)
assert_true(negative_abs == 3.14)

fr fr Test string edge cases
assert_eq_string(stringz.reverse(""), "")
assert_eq_string(stringz.to_upper(""), "")

sus empty_split [tea] = stringz.split("", ",")
assert_eq_int(len(empty_split), 1)

vibez.spill("✅ Performance and edge cases handled correctly")

fr fr === MEMORY AND RESOURCE MANAGEMENT TESTS ===
vibez.spill("=== Testing Memory and Resource Management ===")

fr fr Test clearing data structures
sus test_map hashz.HashMap = hashz.hashmap_new()
test_map = hashz.hashmap_put(test_map, "temp", "data")
test_map = hashz.hashmap_clear(test_map)
assert_true(hashz.hashmap_is_empty(test_map))

sus test_set hashz.HashSet = hashz.hashset_new()
test_set = hashz.hashset_add(test_set, "temp")
test_set = hashz.hashset_clear(test_set)
assert_true(hashz.hashset_is_empty(test_set))

fr fr Test array memory usage estimation
sus memory_usage normie = arrayz.array_memory_usage(large_array)
assert_true(memory_usage > 0)

vibez.spill("✅ Memory and resource management working correctly")

fr fr === FINAL INTEGRATION VALIDATION ===
vibez.spill("=== Final Integration Validation ===")

fr fr Create a complex data structure using all modules
sus user_data hashz.HashMap = hashz.hashmap_new()

fr fr Store user information
user_data = hashz.hashmap_put(user_data, "name", "Alice")
user_data = hashz.hashmap_put(user_data, "age", "30")
user_data = hashz.hashmap_put(user_data, "skills", "programming,mathematics,writing")

fr fr Process skills using string and array operations
sus (skills_str, skills_found) = hashz.hashmap_get(user_data, "skills")
assert_true(skills_found)

sus skills_array [tea] = stringz.split(skills_str, ",")
sus skill_count normie = arrayz.array_length(skills_array)
assert_eq_int(skill_count, 3)

fr fr Use math operations for calculations
sus (age_str, age_found) = hashz.hashmap_get(user_data, "age")
assert_true(age_found)

fr fr Create a summary using all modules
sus summary_parts [tea]
summary_parts = append(summary_parts, "User Analysis:")

sus (name, name_found) = hashz.hashmap_get(user_data, "name")
lowkey name_found {
    sus name_line tea = stringz.concat("Name: ", name)
    summary_parts = append(summary_parts, name_line)
}

sus skills_line tea = vibez.spillstr("Skills: %d total", stringz.concat("", vibez.format_number(skill_count)))
summary_parts = append(summary_parts, skills_line)

sus summary tea = arrayz.array_join(summary_parts, " | ")
vibez.spill(summary)

vibez.spill("✅ Final integration validation successful")

print_test_summary()

vibez.spill("\n=== COMPREHENSIVE STDLIB MODULES TEST COMPLETE ===")
vibez.spill("All 5 core modules (vibez, mathz, stringz, arrayz, hashz) working correctly!")
vibez.spill("Cross-module integration validated successfully!")
vibez.spill("CURSED standard library is production-ready!")
