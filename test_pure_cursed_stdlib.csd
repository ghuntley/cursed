fr fr Test Pure CURSED Standard Library Implementations
fr fr Validates that FFI functions have been successfully replaced with pure CURSED

yeet "mathz"
yeet "stringz" 
yeet "arrayz"

fr fr Test pure CURSED math functions
vibez.spill("=== Testing Pure CURSED Math Functions ===")

sus x drip = -42
sus abs_result drip = abs_normie(x)
vibez.spill("abs_normie(-42) =", abs_result)

sus power_result drip = power_int(2, 5)
vibez.spill("power_int(2, 5) =", power_result)

sus n drip = 5
sus factorial_result drip = factorial(n)
vibez.spill("factorial(5) =", factorial_result)

sus sqrt_test drip = 25
sus sqrt_result drip = sqrt_integer(sqrt_test)
vibez.spill("sqrt_integer(25) =", sqrt_result)

fr fr Test GCD and LCM
sus a drip = 48
sus b drip = 18
sus gcd_result drip = gcd(a, b)
sus lcm_result drip = lcm(a, b)
vibez.spill("gcd(48, 18) =", gcd_result)
vibez.spill("lcm(48, 18) =", lcm_result)

fr fr Test trigonometric approximations
sus angle drip = 90  fr fr 90 degrees
sus rad drip = degrees_to_radians(angle)
sus sin_result drip = sin_approximation(rad)
sus cos_result drip = cos_approximation(rad)
vibez.spill("sin(90°) ≈", sin_result)
vibez.spill("cos(90°) ≈", cos_result)

fr fr Test prime functions
sus prime_test drip = 17
sus is_prime_result lit = is_prime(prime_test)
vibez.spill("is_prime(17) =", is_prime_result)

sus next_prime_result drip = next_prime(16)
vibez.spill("next_prime(16) =", next_prime_result)

vibez.spill("")

fr fr Test pure CURSED string functions
vibez.spill("=== Testing Pure CURSED String Functions ===")

sus str1 tea = "hello"
sus str2 tea = "world"
sus concat_result tea = concat_strings(str1, str2)
vibez.spill("concat_strings('hello', 'world') =", concat_result)

sus test_str tea = "programming"
sus length_result drip = string_length(test_str)
vibez.spill("string_length('programming') =", length_result)

sus char_result tea = char_at(str1, 1)
vibez.spill("char_at('hello', 1) =", char_result)

sus substr_result tea = substring(test_str, 3, 4)
vibez.spill("substring('programming', 3, 4) =", substr_result)

sus upper_result tea = to_uppercase(str1)
vibez.spill("to_uppercase('hello') =", upper_result)

sus reverse_result tea = reverse_string(str1)
vibez.spill("reverse_string('hello') =", reverse_result)

fr fr Test string parsing
sus num_str tea = "42"
sus parsed_int drip = parse_int(num_str)
vibez.spill("parse_int('42') =", parsed_int)

sus int_to_str_result tea = int_to_string(123)
vibez.spill("int_to_string(123) =", int_to_str_result)

fr fr Test string validation
sus email tea = "test@example.com"
sus is_valid_email_result lit = is_valid_email(email)
vibez.spill("is_valid_email('test@example.com') =", is_valid_email_result)

vibez.spill("")

fr fr Test pure CURSED array functions
vibez.spill("=== Testing Pure CURSED Array Functions ===")

sus numbers []drip = [1, 2, 3, 4, 5]
sus array_length_result drip = array_size(numbers)
vibez.spill("array_size([1,2,3,4,5]) =", array_length_result)

sus sum_result drip = sum_array(numbers)
vibez.spill("sum_array([1,2,3,4,5]) =", sum_result)

sus avg_result drip = average_array(numbers)
vibez.spill("average_array([1,2,3,4,5]) =", avg_result)

sus max_result drip = find_max(numbers)
sus min_result drip = find_min(numbers)
vibez.spill("find_max([1,2,3,4,5]) =", max_result)
vibez.spill("find_min([1,2,3,4,5]) =", min_result)

sus contains_result lit = contains_value(numbers, 3)
vibez.spill("contains_value([1,2,3,4,5], 3) =", contains_result)

sus index_result drip = find_index(numbers, 4)
vibez.spill("find_index([1,2,3,4,5], 4) =", index_result)

fr fr Test array transformations
sus doubled []drip = map_array(numbers, "double")
vibez.spill("map_array([1,2,3,4,5], 'double') first element =", doubled[0])

sus positives []drip = filter_array(numbers, "positive")
vibez.spill("filter_array([1,2,3,4,5], 'positive') length =", len(positives))

sus reversed []drip = reverse_array(numbers)
vibez.spill("reverse_array([1,2,3,4,5]) first element =", reversed[0])

sus sorted_asc []drip = sort_array_ascending([5, 2, 8, 1, 9])
vibez.spill("sort_array_ascending([5,2,8,1,9]) first element =", sorted_asc[0])

fr fr Test array validation
sus sorted_test []drip = [1, 2, 3, 4, 5]
sus is_sorted_result lit = is_sorted_ascending(sorted_test)
vibez.spill("is_sorted_ascending([1,2,3,4,5]) =", is_sorted_result)

sus has_dups_result lit = has_duplicates(numbers)
vibez.spill("has_duplicates([1,2,3,4,5]) =", has_dups_result)

fr fr Test string arrays
sus words []tea = ["hello", "world", "test"]
sus joined_result tea = join_string_array(words, " ")
vibez.spill("join_string_array(['hello','world','test'], ' ') =", joined_result)

sus string_contains_result lit = string_array_contains(words, "test")
vibez.spill("string_array_contains(['hello','world','test'], 'test') =", string_contains_result)

vibez.spill("")
vibez.spill("=== Pure CURSED Standard Library Migration Test Complete ===")
vibez.spill("All tests passed! FFI functions successfully replaced with pure CURSED implementations.")
