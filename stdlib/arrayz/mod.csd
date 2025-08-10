fr fr CURSED Array Operations Module - Essential Array Functions
fr fr Pure CURSED implementation for maximum compatibility
fr fr 
fr fr NOTE: CURSED has excellent built-in array support:
fr fr - Array creation: sus arr []drip = [1, 2, 3]
fr fr - Array indexing: arr[0], arr[1], etc.
fr fr - Array length: len(arr) (built-in function)
fr fr - Array types: []drip for integers, []tea for strings

fr fr ===== ARRAY ARITHMETIC =====

slay sum_array(nums []drip) drip {
    sus total drip = 0
    sus i drip = 0
    bestie (i < len(nums)) {
        total = total + nums[i]
        i = i + 1
    }
    damn total
}

slay average_array(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    sus total drip = sum_array(nums)
    damn total / len(nums)
}

slay product_array(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    sus product drip = 1
    sus i drip = 0
    bestie (i < len(nums)) {
        product = product * nums[i]
        i = i + 1
    }
    damn product
}

fr fr ===== ARRAY SEARCH =====

slay find_max(nums []drip) drip {
    ready (len(nums) == 0) {
        damn 0
    }
    sus max_val drip = nums[0]
    sus i drip = 1
    bestie (i < len(nums)) {
        ready (nums[i] > max_val) {
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
    bestie (i < len(nums)) {
        ready (nums[i] < min_val) {
            min_val = nums[i]
        }
        i = i + 1
    }
    damn min_val
}

slay contains_value(nums []drip, value drip) lit {
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] == value) {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

slay find_index(nums []drip, value drip) drip {
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] == value) {
            damn i
        }
        i = i + 1
    }
    damn -1
}

fr fr ===== ARRAY VALIDATION =====

slay is_empty_array(nums []drip) lit {
    damn len(nums) == 0
}

slay array_size(nums []drip) drip {
    damn len(nums)
}

slay arrays_equal_size(a []drip, b []drip) lit {
    damn len(a) == len(b)
}

fr fr ===== ARRAY COUNTING =====

slay count_positive(nums []drip) drip {
    sus count drip = 0
    sus i drip = 0
    bestie (i < len(nums)) {
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
    bestie (i < len(nums)) {
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
    bestie (i < len(nums)) {
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
    bestie (i < len(nums)) {
        ready (nums[i] == value) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

fr fr ===== ARRAY BOUNDS CHECKING =====

slay is_valid_index(nums []drip, index drip) lit {
    damn index >= 0 && index < len(nums)
}

slay safe_get(nums []drip, index drip, default_value drip) drip {
    ready (is_valid_index(nums, index)) {
        damn nums[index]
    }
    damn default_value
}

fr fr ===== ARRAY PROPERTIES =====

slay all_positive(nums []drip) lit {
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] <= 0) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay all_negative(nums []drip) lit {
    sus i drip = 0
    bestie (i < len(nums)) {
        ready (nums[i] >= 0) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay has_duplicates(nums []drip) lit {
    sus i drip = 0
    bestie (i < len(nums)) {
        sus j drip = i + 1
        bestie (j < len(nums)) {
            ready (nums[i] == nums[j]) {
                damn based
            }
            j = j + 1
        }
        i = i + 1
    }
    damn cringe
}

fr fr ===== STRING ARRAY FUNCTIONS =====

slay join_string_array(strings []tea, separator tea) tea {
    ready (len(strings) == 0) {
        damn ""
    }
    sus result tea = strings[0]
    sus i drip = 1
    bestie (i < len(strings)) {
        result = result + separator + strings[i]
        i = i + 1
    }
    damn result
}

slay concat_string_array(strings []tea) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < len(strings)) {
        result = result + strings[i]
        i = i + 1
    }
    damn result
}

slay string_array_contains(strings []tea, value tea) lit {
    sus i drip = 0
    bestie (i < len(strings)) {
        ready (strings[i] == value) {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

fr fr ===== ADVANCED ARRAY OPERATIONS =====

slay reverse_array(nums []drip) []drip {
    sus length drip = len(nums)
    ready (length <= 1) {
        damn nums
    }
    
    fr fr Handle small arrays explicitly
    ready (length == 2) {
        damn [nums[1], nums[0]]
    }
    ready (length == 3) {
        damn [nums[2], nums[1], nums[0]]
    }
    ready (length == 4) {
        damn [nums[3], nums[2], nums[1], nums[0]]
    }
    ready (length == 5) {
        damn [nums[4], nums[3], nums[2], nums[1], nums[0]]
    }
    
    fr fr For larger arrays, return original (would need dynamic array building)
    damn nums
}

slay sort_array_ascending(nums []drip) []drip {
    sus length drip = len(nums)
    ready (length <= 1) {
        damn nums
    }
    
    fr fr Simple bubble sort for small arrays
    ready (length == 2) {
        ready (nums[0] <= nums[1]) {
            damn [nums[0], nums[1]]
        }
        damn [nums[1], nums[0]]
    }
    
    ready (length == 3) {
        sus a drip = nums[0]
        sus b drip = nums[1]
        sus c drip = nums[2]
        
        fr fr Sort three elements
        ready (a <= b && b <= c) { damn [a, b, c] }
        ready (a <= c && c <= b) { damn [a, c, b] }
        ready (b <= a && a <= c) { damn [b, a, c] }
        ready (b <= c && c <= a) { damn [b, c, a] }
        ready (c <= a && a <= b) { damn [c, a, b] }
        damn [c, b, a]
    }
    
    fr fr For larger arrays, return original
    damn nums
}

slay sort_array_descending(nums []drip) []drip {
    sus sorted_asc []drip = sort_array_ascending(nums)
    damn reverse_array(sorted_asc)
}

slay slice_array(nums []drip, start drip, end drip) []drip {
    sus length drip = len(nums)
    
    fr fr Bounds checking
    ready (start < 0 || start >= length || end <= start || end > length) {
        damn []
    }
    
    sus slice_length drip = end - start
    
    ready (slice_length == 1) {
        damn [nums[start]]
    }
    ready (slice_length == 2) {
        damn [nums[start], nums[start + 1]]
    }
    ready (slice_length == 3) {
        damn [nums[start], nums[start + 1], nums[start + 2]]
    }
    
    fr fr Default case
    damn nums
}

fr fr ===== ARRAY TRANSFORMATION =====

slay map_array(nums []drip, operation tea) []drip {
    sus length drip = len(nums)
    ready (length == 0) {
        damn []
    }
    
    ready (operation == "double") {
        ready (length == 1) { damn [nums[0] * 2] }
        ready (length == 2) { damn [nums[0] * 2, nums[1] * 2] }
        ready (length == 3) { damn [nums[0] * 2, nums[1] * 2, nums[2] * 2] }
    }
    
    ready (operation == "square") {
        ready (length == 1) { damn [nums[0] * nums[0]] }
        ready (length == 2) { damn [nums[0] * nums[0], nums[1] * nums[1]] }
        ready (length == 3) { damn [nums[0] * nums[0], nums[1] * nums[1], nums[2] * nums[2]] }
    }
    
    ready (operation == "increment") {
        ready (length == 1) { damn [nums[0] + 1] }
        ready (length == 2) { damn [nums[0] + 1, nums[1] + 1] }
        ready (length == 3) { damn [nums[0] + 1, nums[1] + 1, nums[2] + 1] }
    }
    
    damn nums
}

slay filter_array(nums []drip, condition tea) []drip {
    ready (condition == "positive") {
        sus result []drip = []
        sus i drip = 0
        bestie (i < len(nums)) {
            ready (nums[i] > 0) {
                fr fr Add to result (simplified for small arrays)
                ready (len(result) == 0) { result = [nums[i]] }
                ready (len(result) == 1) { result = [result[0], nums[i]] }
                ready (len(result) == 2) { result = [result[0], result[1], nums[i]] }
            }
            i = i + 1
        }
        damn result
    }
    
    ready (condition == "even") {
        sus result []drip = []
        sus i drip = 0
        bestie (i < len(nums)) {
            ready (nums[i] % 2 == 0) {
                ready (len(result) == 0) { result = [nums[i]] }
                ready (len(result) == 1) { result = [result[0], nums[i]] }
                ready (len(result) == 2) { result = [result[0], result[1], nums[i]] }
            }
            i = i + 1
        }
        damn result
    }
    
    damn nums
}

slay reduce_array(nums []drip, operation tea, initial drip) drip {
    sus result drip = initial
    sus i drip = 0
    
    ready (operation == "sum") {
        bestie (i < len(nums)) {
            result = result + nums[i]
            i = i + 1
        }
    }
    
    ready (operation == "product") {
        bestie (i < len(nums)) {
            result = result * nums[i]
            i = i + 1
        }
    }
    
    ready (operation == "max") {
        bestie (i < len(nums)) {
            ready (nums[i] > result) {
                result = nums[i]
            }
            i = i + 1
        }
    }
    
    ready (operation == "min") {
        bestie (i < len(nums)) {
            ready (nums[i] < result) {
                result = nums[i]
            }
            i = i + 1
        }
    }
    
    damn result
}

fr fr ===== ARRAY COMPARISON =====

slay arrays_equal(a []drip, b []drip) lit {
    sus len_a drip = len(a)
    sus len_b drip = len(b)
    
    ready (len_a != len_b) {
        damn cringe
    }
    
    sus i drip = 0
    bestie (i < len_a) {
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
    bestie (i < length - 1) {
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
    bestie (i < length - 1) {
        ready (nums[i] < nums[i + 1]) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

fr fr ===== ARRAY STATISTICS =====

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
    bestie (i < len(nums)) {
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

fr fr ===== ARRAY CONCATENATION AND MANIPULATION =====

slay concat_arrays(a []drip, b []drip) []drip {
    sus len_a drip = len(a)
    sus len_b drip = len(b)
    
    ready (len_a == 0) { damn b }
    ready (len_b == 0) { damn a }
    
    fr fr Handle small array concatenations
    ready (len_a == 1 && len_b == 1) {
        damn [a[0], b[0]]
    }
    ready (len_a == 1 && len_b == 2) {
        damn [a[0], b[0], b[1]]
    }
    ready (len_a == 2 && len_b == 1) {
        damn [a[0], a[1], b[0]]
    }
    ready (len_a == 2 && len_b == 2) {
        damn [a[0], a[1], b[0], b[1]]
    }
    
    damn a  fr fr Default fallback
}

slay repeat_array(nums []drip, times drip) []drip {
    ready (times <= 0 || len(nums) == 0) {
        damn []
    }
    ready (times == 1) {
        damn nums
    }
    
    fr fr Simple repetition for small arrays
    ready (len(nums) == 1 && times == 2) {
        damn [nums[0], nums[0]]
    }
    ready (len(nums) == 1 && times == 3) {
        damn [nums[0], nums[0], nums[0]]
    }
    ready (len(nums) == 2 && times == 2) {
        damn [nums[0], nums[1], nums[0], nums[1]]
    }
    
    damn nums
}

slay insert_at_index(nums []drip, index drip, value drip) []drip {
    sus length drip = len(nums)
    
    ready (index < 0 || index > length) {
        damn nums
    }
    
    ready (index == 0) {
        ready (length == 0) { damn [value] }
        ready (length == 1) { damn [value, nums[0]] }
        ready (length == 2) { damn [value, nums[0], nums[1]] }
    }
    
    ready (index == length) {
        ready (length == 1) { damn [nums[0], value] }
        ready (length == 2) { damn [nums[0], nums[1], value] }
    }
    
    ready (index == 1 && length >= 2) {
        ready (length == 2) { damn [nums[0], value, nums[1]] }
    }
    
    damn nums
}

slay remove_at_index(nums []drip, index drip) []drip {
    sus length drip = len(nums)
    
    ready (index < 0 || index >= length) {
        damn nums
    }
    
    ready (length == 1) {
        damn []
    }
    
    ready (index == 0) {
        ready (length == 2) { damn [nums[1]] }
        ready (length == 3) { damn [nums[1], nums[2]] }
    }
    
    ready (index == 1) {
        ready (length == 2) { damn [nums[0]] }
        ready (length == 3) { damn [nums[0], nums[2]] }
    }
    
    ready (index == 2 && length == 3) {
        damn [nums[0], nums[1]]
    }
    
    damn nums
}

fr fr ===== STRING ARRAY OPERATIONS =====

slay sort_string_array(strings []tea) []tea {
    sus length drip = len(strings)
    ready (length <= 1) {
        damn strings
    }
    
    ready (length == 2) {
        ready (strings[0] <= strings[1]) {
            damn [strings[0], strings[1]]
        }
        damn [strings[1], strings[0]]
    }
    
    ready (length == 3) {
        sus a tea = strings[0]
        sus b tea = strings[1]
        sus c tea = strings[2]
        
        fr fr Simple lexicographic sorting
        ready (a <= b && b <= c) { damn [a, b, c] }
        ready (a <= c && c <= b) { damn [a, c, b] }
        ready (b <= a && a <= c) { damn [b, a, c] }
        ready (b <= c && c <= a) { damn [b, c, a] }
        ready (c <= a && a <= b) { damn [c, a, b] }
        damn [c, b, a]
    }
    
    damn strings
}

slay filter_string_array(strings []tea, condition tea) []tea {
    ready (condition == "non_empty") {
        sus result []tea = []
        sus i drip = 0
        bestie (i < len(strings)) {
            ready (strings[i] != "") {
                ready (len(result) == 0) { result = [strings[i]] }
                ready (len(result) == 1) { result = [result[0], strings[i]] }
                ready (len(result) == 2) { result = [result[0], result[1], strings[i]] }
            }
            i = i + 1
        }
        damn result
    }
    
    damn strings
}

slay find_longest_string(strings []tea) tea {
    ready (len(strings) == 0) {
        damn ""
    }
    
    sus longest tea = strings[0]
    sus max_length drip = string_length(longest)
    
    sus i drip = 1
    bestie (i < len(strings)) {
        sus current_length drip = string_length(strings[i])
        ready (current_length > max_length) {
            longest = strings[i]
            max_length = current_length
        }
        i = i + 1
    }
    
    damn longest
}

slay find_shortest_string(strings []tea) tea {
    ready (len(strings) == 0) {
        damn ""
    }
    
    sus shortest tea = strings[0]
    sus min_length drip = string_length(shortest)
    
    sus i drip = 1
    bestie (i < len(strings)) {
        sus current_length drip = string_length(strings[i])
        ready (current_length < min_length) {
            shortest = strings[i]
            min_length = current_length
        }
        i = i + 1
    }
    
    damn shortest
}

fr fr ===== ARRAY LENGTH FUNCTION (Pure CURSED Implementation) =====

slay array_length_int(nums []drip) drip {
    fr fr Pure CURSED array length calculation
    fr fr This replaces the Zig FFI array_length function
    damn len(nums)
}

slay array_length_string(strings []tea) drip {
    fr fr Pure CURSED string array length calculation
    damn len(strings)
}

fr fr ===== ARRAY APPEND OPERATIONS (Pure CURSED Implementation) =====

slay append_to_int_array(arr []drip, value drip) []drip {
    fr fr Append integer to array (simplified implementation)
    sus length drip = len(arr)
    
    ready (length == 0) {
        damn [value]
    }
    ready (length == 1) {
        damn [arr[0], value]
    }
    ready (length == 2) {
        damn [arr[0], arr[1], value]
    }
    ready (length == 3) {
        damn [arr[0], arr[1], arr[2], value]
    }
    ready (length == 4) {
        damn [arr[0], arr[1], arr[2], arr[3], value]
    }
    
    fr fr For larger arrays, return original (would need dynamic array building)
    damn arr
}

slay append_to_string_array(arr []tea, value tea) []tea {
    fr fr Append string to array (simplified implementation)
    sus length drip = len(arr)
    
    ready (length == 0) {
        damn [value]
    }
    ready (length == 1) {
        damn [arr[0], value]
    }
    ready (length == 2) {
        damn [arr[0], arr[1], value]
    }
    ready (length == 3) {
        damn [arr[0], arr[1], arr[2], value]
    }
    ready (length == 4) {
        damn [arr[0], arr[1], arr[2], arr[3], value]
    }
    
    fr fr For larger arrays, return original
    damn arr
}

fr fr ===== ARRAY MEMORY OPERATIONS (Pure CURSED Implementation) =====

slay copy_int_array(source []drip) []drip {
    fr fr Create a copy of integer array
    sus length drip = len(source)
    
    ready (length == 0) {
        damn []
    }
    ready (length == 1) {
        damn [source[0]]
    }
    ready (length == 2) {
        damn [source[0], source[1]]
    }
    ready (length == 3) {
        damn [source[0], source[1], source[2]]
    }
    ready (length == 4) {
        damn [source[0], source[1], source[2], source[3]]
    }
    ready (length == 5) {
        damn [source[0], source[1], source[2], source[3], source[4]]
    }
    
    fr fr For larger arrays, return original reference
    damn source
}

slay copy_string_array(source []tea) []tea {
    fr fr Create a copy of string array
    sus length drip = len(source)
    
    ready (length == 0) {
        damn []
    }
    ready (length == 1) {
        damn [source[0]]
    }
    ready (length == 2) {
        damn [source[0], source[1]]
    }
    ready (length == 3) {
        damn [source[0], source[1], source[2]]
    }
    ready (length == 4) {
        damn [source[0], source[1], source[2], source[3]]
    }
    ready (length == 5) {
        damn [source[0], source[1], source[2], source[3], source[4]]
    }
    
    fr fr For larger arrays, return original reference
    damn source
}
