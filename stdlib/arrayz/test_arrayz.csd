yeet "testz"
yeet "arrayz"

test_start("arrayz basic functions")
sus test_nums drip[value] = [1, 2, 3, 4, 5]
sus sum_result drip = sum_array(test_nums)
sus max_result drip = find_max(test_nums)
sus min_result drip = find_min(test_nums)
vibez.spill("Array:", test_nums)
vibez.spill("sum_array result:", sum_result)
vibez.spill("find_max result:", max_result)
vibez.spill("find_min result:", min_result)

test_start("arrayz search functions")
sus contains_result lit = contains_value(test_nums, 3)
sus index_result drip = find_index(test_nums, 3)
sus not_found drip = find_index(test_nums, 99)
vibez.spill("contains_value(3):", contains_result)
vibez.spill("find_index(3):", index_result)
vibez.spill("find_index(99):", not_found)

test_start("arrayz validation functions")
sus empty_nums drip[value] = []
sus is_empty lit = is_empty_array(empty_nums)
sus size_result drip = array_size(test_nums)
sus valid_index lit = is_valid_index(test_nums, 2)
sus invalid_index lit = is_valid_index(test_nums, 10)
vibez.spill("is_empty_array([]):", is_empty)
vibez.spill("array_size:", size_result)
vibez.spill("is_valid_index(2):", valid_index)
vibez.spill("is_valid_index(10):", invalid_index)

test_start("arrayz counting functions")
sus mixed_nums drip[value] = [-2, 0, 1, -1, 3, 0]
sus positive_count drip = count_positive(mixed_nums)
sus negative_count drip = count_negative(mixed_nums)
sus zero_count drip = count_zeros(mixed_nums)
vibez.spill("Mixed array:", mixed_nums)
vibez.spill("count_positive:", positive_count)
vibez.spill("count_negative:", negative_count)
vibez.spill("count_zeros:", zero_count)

test_start("arrayz string functions")
sus test_strings tea[value] = ["hello", "world", "test"]
sus joined_result tea = join_string_array(test_strings, " ")
sus concat_result tea = concat_string_array(test_strings)
sus contains_string lit = string_array_contains(test_strings, "world")
vibez.spill("String array:", test_strings)
vibez.spill("join_string_array:", joined_result)
vibez.spill("concat_string_array:", concat_result)
vibez.spill("contains 'world':", contains_string)

test_start("arrayz safe operations")
sus safe_value drip = safe_get(test_nums, 2, -1)
sus safe_invalid drip = safe_get(test_nums, 10, -1)
vibez.spill("safe_get(2, -1):", safe_value)
vibez.spill("safe_get(10, -1):", safe_invalid)

print_test_summary()
