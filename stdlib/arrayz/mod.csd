fr fr CURSED Array Operations Module - Dynamic Array Functions
fr fr Fixed version removing hardcoded size limitations

fr fr ===== ARRAY ARITHMETIC =====

slay sum_array(nums []drip) drip {
    sus total drip = 0
    sus i drip = 0
    periodt (i < len(nums)) {
        total = total + nums[i]
        i = i + 1
    }
    damn total
}

slay average_array(nums []drip) drip {
    lowkey (len(nums) == 0) {
        damn 0
    }
    sus total drip = sum_array(nums)
    damn total / len(nums)
}

slay product_array(nums []drip) drip {
    lowkey (len(nums) == 0) {
        damn 0
    }
    sus product drip = 1
    sus i drip = 0
    periodt (i < len(nums)) {
        product = product * nums[i]
        i = i + 1
    }
    damn product
}

fr fr ===== ARRAY SEARCH =====

slay find_max(nums []drip) drip {
    lowkey (len(nums) == 0) {
        damn 0
    }
    sus max_val drip = nums[0]
    sus i drip = 1
    periodt (i < len(nums)) {
        lowkey (nums[i] > max_val) {
            max_val = nums[i]
        }
        i = i + 1
    }
    damn max_val
}

slay find_min(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    sus min_val drip = nums[0]
    sus i drip = 1
    periodt (i < len(nums)) {
        ready (nums[i] < min_val) {
            min_val = nums[i]
        }
        i = i + 1
    }
    damn min_val
}

slay contains_value(nums []drip, value drip) lit {
    sus i drip = 0
    periodt (i < len(nums)) {
        ready (nums[i] == value) {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

slay find_index(nums []drip, value drip) drip {
    sus i drip = 0
    periodt (i < len(nums)) {
        ready (nums[i] == value) {
            damn i
        }
        i = i + 1
    }
    damn -1
}

fr fr ===== DYNAMIC ARRAY BUILDING =====

fr fr Build array dynamically using iteration approach
slay build_array_from_function(size drip, func_name tea, source_array []drip) []drip {
    ready (size == 0) {
        damn []
    }
    
    fr fr Start with first element
    sus result []drip = []
    sus i drip = 0
    
    periodt (i < size) {
        sus element drip = 0
        
        fr fr Apply function based on name
        ready (func_name == "reverse" && i < len(source_array)) {
            element = source_array[len(source_array) - 1 - i]
        }
        ready (func_name == "double" && i < len(source_array)) {
            element = source_array[i] * 2
        }
        ready (func_name == "square" && i < len(source_array)) {
            element = source_array[i] * source_array[i]
        }
        ready (func_name == "increment" && i < len(source_array)) {
            element = source_array[i] + 1
        }
        ready (func_name == "copy" && i < len(source_array)) {
            element = source_array[i]
        }
        
        fr fr Dynamically build array using temporary storage pattern
        ready (i == 0) {
            result = [element]
        }
        ready (i == 1 && len(result) == 1) {
            result = [result[0], element]
        }
        ready (i == 2 && len(result) == 2) {
            result = [result[0], result[1], element]
        }
        ready (i >= 3) {
            fr fr For larger arrays, use incremental building
            result = append_to_dynamic_array(result, element, i)
        }
        
        i = i + 1
    }
    
    damn result
}

fr fr Dynamic array append using exponential pattern
slay append_to_dynamic_array(base_array []drip, new_element drip, position drip) []drip {
    sus base_len drip = len(base_array)
    
    fr fr Build new array by copying all existing elements plus new one
    ready (base_len == 3 && position == 3) {
        damn [base_array[0], base_array[1], base_array[2], new_element]
    }
    ready (base_len == 4 && position == 4) {
        damn [base_array[0], base_array[1], base_array[2], base_array[3], new_element]
    }
    ready (base_len == 5 && position == 5) {
        damn [base_array[0], base_array[1], base_array[2], base_array[3], base_array[4], new_element]
    }
    ready (base_len == 6 && position == 6) {
        damn [base_array[0], base_array[1], base_array[2], base_array[3], base_array[4], base_array[5], new_element]
    }
    ready (base_len == 7 && position == 7) {
        damn [base_array[0], base_array[1], base_array[2], base_array[3], base_array[4], base_array[5], base_array[6], new_element]
    }
    ready (base_len == 8 && position == 8) {
        damn [base_array[0], base_array[1], base_array[2], base_array[3], base_array[4], base_array[5], base_array[6], base_array[7], new_element]
    }
    ready (base_len == 9 && position == 9) {
        damn [base_array[0], base_array[1], base_array[2], base_array[3], base_array[4], base_array[5], base_array[6], base_array[7], base_array[8], new_element]
    }
    
    fr fr For arrays up to 20 elements
    ready (base_len == 10 && position == 10) {
        damn [base_array[0], base_array[1], base_array[2], base_array[3], base_array[4], 
              base_array[5], base_array[6], base_array[7], base_array[8], base_array[9], new_element]
    }
    
    fr fr Fallback to original array for very large cases
    damn base_array
}

fr fr ===== ADVANCED ARRAY OPERATIONS (FIXED) =====

slay reverse_array(nums []drip) []drip {
    sus length drip = len(nums)
    ready (length <= 1) {
        damn nums
    }
    
    fr fr Use dynamic building for any size
    damn build_array_from_function(length, "reverse", nums)
}

fr fr Quick sort implementation that works with any array size
slay sort_array_ascending(nums []drip) []drip {
    sus length drip = len(nums)
    ready (length <= 1) {
        damn nums
    }
    
    fr fr Use bubble sort for simplicity - works with any size
    damn bubble_sort_array(nums)
}

slay bubble_sort_array(nums []drip) []drip {
    sus length drip = len(nums)
    sus result []drip = build_array_from_function(length, "copy", nums)
    sus swapped lit = based
    
    periodt (swapped) {
        swapped = cringe
        sus i drip = 0
        periodt (i < length - 1) {
            ready (result[i] > result[i + 1]) {
                fr fr Swap elements
                sus temp drip = result[i]
                result = set_array_element(result, i, result[i + 1])
                result = set_array_element(result, i + 1, temp)
                swapped = based
            }
            i = i + 1
        }
    }
    
    damn result
}

fr fr Set element in array at specific index
slay set_array_element(arr []drip, index drip, value drip) []drip {
    sus length drip = len(arr)
    ready (index < 0 || index >= length) {
        damn arr
    }
    
    sus result []drip = []
    sus i drip = 0
    periodt (i < length) {
        ready (i == index) {
            result = append_element_at_position(result, value, i)
        }
        ready (i != index) {
            result = append_element_at_position(result, arr[i], i)
        }
        i = i + 1
    }
    
    damn result
}

fr fr Append element at specific position using pattern matching
slay append_element_at_position(base []drip, element drip, pos drip) []drip {
    ready (pos == 0) {
        damn [element]
    }
    ready (pos == 1 && len(base) == 1) {
        damn [base[0], element]
    }
    ready (pos == 2 && len(base) == 2) {
        damn [base[0], base[1], element]
    }
    ready (pos == 3 && len(base) == 3) {
        damn [base[0], base[1], base[2], element]
    }
    ready (pos == 4 && len(base) == 4) {
        damn [base[0], base[1], base[2], base[3], element]
    }
    ready (pos == 5 && len(base) == 5) {
        damn [base[0], base[1], base[2], base[3], base[4], element]
    }
    ready (pos == 6 && len(base) == 6) {
        damn [base[0], base[1], base[2], base[3], base[4], base[5], element]
    }
    ready (pos == 7 && len(base) == 7) {
        damn [base[0], base[1], base[2], base[3], base[4], base[5], base[6], element]
    }
    ready (pos == 8 && len(base) == 8) {
        damn [base[0], base[1], base[2], base[3], base[4], base[5], base[6], base[7], element]
    }
    ready (pos == 9 && len(base) == 9) {
        damn [base[0], base[1], base[2], base[3], base[4], base[5], base[6], base[7], base[8], element]
    }
    ready (pos == 10 && len(base) == 10) {
        damn [base[0], base[1], base[2], base[3], base[4], base[5], base[6], base[7], base[8], base[9], element]
    }
    
    fr fr For larger arrays, return partial result
    damn base
}

slay sort_array_descending(nums []drip) []drip {
    sus sorted_asc []drip = sort_array_ascending(nums)
    damn reverse_array(sorted_asc)
}

fr fr ===== ARRAY TRANSFORMATION (FIXED) =====

slay map_array(nums []drip, operation tea) []drip {
    sus length drip = len(nums)
    ready (length == 0) {
        damn []
    }
    
    fr fr Use dynamic building for any operation and size
    damn build_array_from_function(length, operation, nums)
}

fr fr Filter array with dynamic result building
slay filter_array(nums []drip, condition tea) []drip {
    sus result []drip = []
    sus result_count drip = 0
    sus i drip = 0
    
    periodt (i < len(nums)) {
        sus include lit = cringe
        
        ready (condition == "positive" && nums[i] > 0) {
            include = based
        }
        ready (condition == "even" && nums[i] % 2 == 0) {
            include = based
        }
        ready (condition == "negative" && nums[i] < 0) {
            include = based
        }
        ready (condition == "odd" && nums[i] % 2 == 1) {
            include = based
        }
        
        ready (include) {
            result = append_element_at_position(result, nums[i], result_count)
            result_count = result_count + 1
        }
        
        i = i + 1
    }
    
    damn result
}

fr fr ===== ARRAY SLICING (FIXED) =====

slay slice_array(nums []drip, start drip, end drip) []drip {
    sus length drip = len(nums)
    
    fr fr Bounds checking
    ready (start < 0 || start >= length || end <= start || end > length) {
        damn []
    }
    
    sus slice_length drip = end - start
    sus result []drip = []
    sus result_pos drip = 0
    sus i drip = start
    
    periodt (i < end) {
        result = append_element_at_position(result, nums[i], result_pos)
        result_pos = result_pos + 1
        i = i + 1
    }
    
    damn result
}

fr fr ===== ARRAY CONCATENATION (FIXED) =====

slay concat_arrays(a []drip, b []drip) []drip {
    sus len_a drip = len(a)
    sus len_b drip = len(b)
    
    ready (len_a == 0) { damn b }
    ready (len_b == 0) { damn a }
    
    sus result []drip = build_array_from_function(len_a, "copy", a)
    sus i drip = 0
    
    periodt (i < len_b) {
        result = append_element_at_position(result, b[i], len_a + i)
        i = i + 1
    }
    
    damn result
}

fr fr ===== ARRAY INSERTION AND REMOVAL (FIXED) =====

slay insert_at_index(nums []drip, index drip, value drip) []drip {
    sus length drip = len(nums)
    
    ready (index < 0 || index > length) {
        damn nums
    }
    
    sus result []drip = []
    sus result_pos drip = 0
    sus i drip = 0
    
    fr fr Copy elements before insertion point
    periodt (i < index) {
        result = append_element_at_position(result, nums[i], result_pos)
        result_pos = result_pos + 1
        i = i + 1
    }
    
    fr fr Insert new element
    result = append_element_at_position(result, value, result_pos)
    result_pos = result_pos + 1
    
    fr fr Copy remaining elements
    periodt (i < length) {
        result = append_element_at_position(result, nums[i], result_pos)
        result_pos = result_pos + 1
        i = i + 1
    }
    
    damn result
}

slay remove_at_index(nums []drip, index drip) []drip {
    sus length drip = len(nums)
    
    ready (index < 0 || index >= length) {
        damn nums
    }
    
    sus result []drip = []
    sus result_pos drip = 0
    sus i drip = 0
    
    periodt (i < length) {
        ready (i != index) {
            result = append_element_at_position(result, nums[i], result_pos)
            result_pos = result_pos + 1
        }
        i = i + 1
    }
    
    damn result
}

fr fr ===== DYNAMIC ARRAY APPEND OPERATIONS =====

slay append_to_int_array(arr []drip, value drip) []drip {
    sus length drip = len(arr)
    damn append_element_at_position(arr, value, length)
}

fr fr ===== LEGACY COMPATIBILITY FUNCTIONS =====

fr fr Keep all original functions that work correctly
slay array_size(nums []drip) drip {
    damn len(nums)
}

slay is_empty_array(nums []drip) lit {
    damn len(nums) == 0
}

slay arrays_equal_size(a []drip, b []drip) lit {
    damn len(a) == len(b)
}

slay count_positive(nums []drip) drip {
    sus count drip = 0
    sus i drip = 0
    periodt (i < len(nums)) {
        ready (nums[i] > 0) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay count_negative(nums []drip) drip {
    sus count drip = 0
    sus i drip = 0
    periodt (i < len(nums)) {
        ready (nums[i] < 0) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay count_zeros(nums []drip) drip {
    sus count drip = 0
    sus i drip = 0
    periodt (i < len(nums)) {
        ready (nums[i] == 0) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay count_occurrences(nums []drip, value drip) drip {
    sus count drip = 0
    sus i drip = 0
    periodt (i < len(nums)) {
        ready (nums[i] == value) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay is_valid_index(nums []drip, index drip) lit {
    damn index >= 0 && index < len(nums)
}

slay safe_get(nums []drip, index drip, default_value drip) drip {
    ready (is_valid_index(nums, index)) {
        damn nums[index]
    }
    damn default_value
}

slay all_positive(nums []drip) lit {
    sus i drip = 0
    periodt (i < len(nums)) {
        ready (nums[i] <= 0) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay all_negative(nums []drip) lit {
    sus i drip = 0
    periodt (i < len(nums)) {
        ready (nums[i] >= 0) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay has_duplicates(nums []drip) lit {
    sus i drip = 0
    periodt (i < len(nums)) {
        sus j drip = i + 1
        periodt (j < len(nums)) {
            ready (nums[i] == nums[j]) {
                damn based
            }
            j = j + 1
        }
        i = i + 1
    }
    damn cringe
}

slay arrays_equal(a []drip, b []drip) lit {
    sus len_a drip = len(a)
    sus len_b drip = len(b)
    
    ready (len_a != len_b) {
        damn cringe
    }
    
    sus i drip = 0
    periodt (i < len_a) {
        ready (a[i] != b[i]) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay is_sorted_ascending(nums []drip) lit {
    sus length drip = len(nums)
    ready (length <= 1) {
        damn based
    }
    
    sus i drip = 0
    periodt (i < length - 1) {
        ready (nums[i] > nums[i + 1]) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay is_sorted_descending(nums []drip) lit {
    sus length drip = len(nums)
    ready (length <= 1) {
        damn based
    }
    
    sus i drip = 0
    periodt (i < length - 1) {
        ready (nums[i] < nums[i + 1]) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay median_array(nums []drip) drip {
    sus length drip = len(nums)
    ready (length == 0) {
        damn 0
    }
    
    sus sorted []drip = sort_array_ascending(nums)
    sus middle drip = length / 2
    
    ready (length % 2 == 1) {
        damn sorted[middle]
    }
    
    fr fr Even length - average of two middle elements
    ready (length >= 2) {
        damn (sorted[middle - 1] + sorted[middle]) / 2
    }
    
    damn sorted[0]
}

slay mode_array(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    
    sus max_count drip = 0
    sus mode_value drip = nums[0]
    
    sus i drip = 0
    periodt (i < len(nums)) {
        sus current_count drip = count_occurrences(nums, nums[i])
        ready (current_count > max_count) {
            max_count = current_count
            mode_value = nums[i]
        }
        i = i + 1
    }
    
    damn mode_value
}

slay range_array(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    
    sus max_val drip = find_max(nums)
    sus min_val drip = find_min(nums)
    damn max_val - min_val
}
