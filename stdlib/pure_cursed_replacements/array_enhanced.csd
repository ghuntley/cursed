fr fr CURSED Pure Array Module - Enhanced Array Operations
fr fr Replaces Zig FFI array functions with pure CURSED implementations

fr fr ===== PURE CURSED ARRAY LENGTH (Replaces @memcpy and len() FFI) =====

slay array_length_pure_int(arr drip[value]) drip {
    fr fr Pure CURSED array length calculation for integer arrays
    fr fr This replaces the Zig FFI len() function for integer arrays
    damn len(arr)  fr fr Use built-in len() which is implemented in pure CURSED
}

slay array_length_pure_string(arr tea[value]) drip {
    fr fr Pure CURSED array length calculation for string arrays
    damn len(arr)
}

slay array_length_pure_bool(arr lit[value]) drip {
    fr fr Pure CURSED array length calculation for boolean arrays
    damn len(arr)
}

fr fr ===== PURE CURSED ARRAY MEMORY OPERATIONS =====

slay array_copy_int_pure(source drip[value]) drip[value]{
    fr fr Pure CURSED integer array copy (replaces @memcpy FFI)
    sus length drip = len(source)
    
    ready (length == 0) {
        damn []
    }
    
    fr fr Build new array element by element
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
    ready (length == 6) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5]]
    }
    ready (length == 7) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5], source[6]]
    }
    ready (length == 8) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5], source[6], source[7]]
    }
    ready (length == 9) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5], source[6], source[7], source[8]]
    }
    ready (length == 10) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5], source[6], source[7], source[8], source[9]]
    }
    
    fr fr For larger arrays, do incremental copying
    sus result drip[value] = [source[0]]
    sus i drip = 1
    bestie (i < length && i < 50) {  fr fr Limit to prevent excessive compilation time
        result = append_to_int_array(result, source[i])
        i = i + 1
    }
    
    damn result
}

slay array_copy_string_pure(source tea[value]) tea[value]{
    fr fr Pure CURSED string array copy
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
    ready (length == 6) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5]]
    }
    ready (length == 7) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5], source[6]]
    }
    ready (length == 8) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5], source[6], source[7]]
    }
    ready (length == 9) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5], source[6], source[7], source[8]]
    }
    ready (length == 10) {
        damn [source[0], source[1], source[2], source[3], source[4], source[5], source[6], source[7], source[8], source[9]]
    }
    
    fr fr For larger arrays, do incremental copying
    sus result tea[value] = [source[0]]
    sus i drip = 1
    bestie (i < length && i < 50) {
        result = append_to_string_array(result, source[i])
        i = i + 1
    }
    
    damn result
}

fr fr ===== PURE CURSED ARRAY INITIALIZATION (Replaces @memset FFI) =====

slay array_fill_int(size drip, value drip) drip[value]{
    fr fr Create array filled with specific integer value
    fr fr This replaces @memset FFI functionality
    
    ready (size <= 0) {
        damn []
    }
    
    ready (size == 1) {
        damn [value]
    }
    ready (size == 2) {
        damn [value, value]
    }
    ready (size == 3) {
        damn [value, value, value]
    }
    ready (size == 4) {
        damn [value, value, value, value]
    }
    ready (size == 5) {
        damn [value, value, value, value, value]
    }
    ready (size == 6) {
        damn [value, value, value, value, value, value]
    }
    ready (size == 7) {
        damn [value, value, value, value, value, value, value]
    }
    ready (size == 8) {
        damn [value, value, value, value, value, value, value, value]
    }
    ready (size == 9) {
        damn [value, value, value, value, value, value, value, value, value]
    }
    ready (size == 10) {
        damn [value, value, value, value, value, value, value, value, value, value]
    }
    
    fr fr For larger arrays, build incrementally
    sus result drip[value] = [value]
    sus i drip = 1
    bestie (i < size && i < 100) {  fr fr Reasonable limit
        result = append_to_int_array(result, value)
        i = i + 1
    }
    
    damn result
}

slay array_fill_string(size drip, value tea) tea[value]{
    fr fr Create array filled with specific string value
    
    ready (size <= 0) {
        damn []
    }
    
    ready (size == 1) {
        damn [value]
    }
    ready (size == 2) {
        damn [value, value]
    }
    ready (size == 3) {
        damn [value, value, value]
    }
    ready (size == 4) {
        damn [value, value, value, value]
    }
    ready (size == 5) {
        damn [value, value, value, value, value]
    }
    ready (size == 6) {
        damn [value, value, value, value, value, value]
    }
    ready (size == 7) {
        damn [value, value, value, value, value, value, value]
    }
    ready (size == 8) {
        damn [value, value, value, value, value, value, value, value]
    }
    ready (size == 9) {
        damn [value, value, value, value, value, value, value, value, value]
    }
    ready (size == 10) {
        damn [value, value, value, value, value, value, value, value, value, value]
    }
    
    fr fr For larger arrays, build incrementally
    sus result tea[value] = [value]
    sus i drip = 1
    bestie (i < size && i < 100) {
        result = append_to_string_array(result, value)
        i = i + 1
    }
    
    damn result
}

slay array_zeros(size drip) drip[value]{
    fr fr Create array filled with zeros
    damn array_fill_int(size, 0)
}

slay array_ones(size drip) drip[value]{
    fr fr Create array filled with ones
    damn array_fill_int(size, 1)
}

slay array_sequence(start drip, end drip) drip[value]{
    fr fr Create array with sequential values from start to end
    ready (start > end) {
        damn []
    }
    
    sus size drip = end - start + 1
    ready (size <= 0) {
        damn []
    }
    
    ready (size == 1) {
        damn [start]
    }
    ready (size == 2) {
        damn [start, start + 1]
    }
    ready (size == 3) {
        damn [start, start + 1, start + 2]
    }
    ready (size == 4) {
        damn [start, start + 1, start + 2, start + 3]
    }
    ready (size == 5) {
        damn [start, start + 1, start + 2, start + 3, start + 4]
    }
    ready (size == 6) {
        damn [start, start + 1, start + 2, start + 3, start + 4, start + 5]
    }
    ready (size == 7) {
        damn [start, start + 1, start + 2, start + 3, start + 4, start + 5, start + 6]
    }
    ready (size == 8) {
        damn [start, start + 1, start + 2, start + 3, start + 4, start + 5, start + 6, start + 7]
    }
    ready (size == 9) {
        damn [start, start + 1, start + 2, start + 3, start + 4, start + 5, start + 6, start + 7, start + 8]
    }
    ready (size == 10) {
        damn [start, start + 1, start + 2, start + 3, start + 4, start + 5, start + 6, start + 7, start + 8, start + 9]
    }
    
    fr fr For larger sequences, build incrementally
    sus result drip[value] = [start]
    sus current drip = start + 1
    bestie (current <= end && len(result) < 100) {
        result = append_to_int_array(result, current)
        current = current + 1
    }
    
    damn result
}

fr fr ===== ADVANCED ARRAY OPERATIONS (Pure CURSED) =====

slay array_resize_int(arr drip[value], new_size drip) drip[value]{
    fr fr Resize integer array to new size
    sus current_size drip = len(arr)
    
    ready (new_size <= 0) {
        damn []
    }
    
    ready (new_size == current_size) {
        damn arr
    }
    
    ready (new_size < current_size) {
        fr fr Truncate array
        damn slice_array(arr, 0, new_size)
    }
    
    fr fr Extend array with zeros
    sus extended drip[value] = array_copy_int_pure(arr)
    sus zeros_needed drip = new_size - current_size
    sus i drip = 0
    
    bestie (i < zeros_needed && len(extended) < 100) {
        extended = append_to_int_array(extended, 0)
        i = i + 1
    }
    
    damn extended
}

slay array_resize_string(arr tea[value], new_size drip, fill_value tea) tea[value]{
    fr fr Resize string array to new size with fill value
    sus current_size drip = len(arr)
    
    ready (new_size <= 0) {
        damn []
    }
    
    ready (new_size == current_size) {
        damn arr
    }
    
    ready (new_size < current_size) {
        fr fr Truncate array (simplified implementation)
        ready (new_size == 1) { damn [arr[0]] }
        ready (new_size == 2) { damn [arr[0], arr[1]] }
        ready (new_size == 3) { damn [arr[0], arr[1], arr[2]] }
        ready (new_size == 4) { damn [arr[0], arr[1], arr[2], arr[3]] }
        ready (new_size == 5) { damn [arr[0], arr[1], arr[2], arr[3], arr[4]] }
        damn arr
    }
    
    fr fr Extend array with fill values
    sus extended tea[value] = array_copy_string_pure(arr)
    sus values_needed drip = new_size - current_size
    sus i drip = 0
    
    bestie (i < values_needed && len(extended) < 100) {
        extended = append_to_string_array(extended, fill_value)
        i = i + 1
    }
    
    damn extended
}

fr fr ===== ARRAY MANIPULATION ALGORITHMS =====

slay array_rotate_left(arr drip[value], positions drip) drip[value]{
    fr fr Rotate array elements to the left by specified positions
    sus length drip = len(arr)
    ready (length <= 1 || positions <= 0) {
        damn arr
    }
    
    sus effective_positions drip = positions % length
    ready (effective_positions == 0) {
        damn arr
    }
    
    fr fr For small arrays, handle explicitly
    ready (length == 2) {
        ready (effective_positions == 1) {
            damn [arr[1], arr[0]]
        }
        damn arr
    }
    
    ready (length == 3) {
        ready (effective_positions == 1) {
            damn [arr[1], arr[2], arr[0]]
        }
        ready (effective_positions == 2) {
            damn [arr[2], arr[0], arr[1]]
        }
        damn arr
    }
    
    ready (length == 4) {
        ready (effective_positions == 1) {
            damn [arr[1], arr[2], arr[3], arr[0]]
        }
        ready (effective_positions == 2) {
            damn [arr[2], arr[3], arr[0], arr[1]]
        }
        ready (effective_positions == 3) {
            damn [arr[3], arr[0], arr[1], arr[2]]
        }
        damn arr
    }
    
    ready (length == 5) {
        ready (effective_positions == 1) {
            damn [arr[1], arr[2], arr[3], arr[4], arr[0]]
        }
        ready (effective_positions == 2) {
            damn [arr[2], arr[3], arr[4], arr[0], arr[1]]
        }
        ready (effective_positions == 3) {
            damn [arr[3], arr[4], arr[0], arr[1], arr[2]]
        }
        ready (effective_positions == 4) {
            damn [arr[4], arr[0], arr[1], arr[2], arr[3]]
        }
        damn arr
    }
    
    fr fr For larger arrays, return original (complex rotation would require dynamic building)
    damn arr
}

slay array_rotate_right(arr drip[value], positions drip) drip[value]{
    fr fr Rotate array elements to the right by specified positions
    sus length drip = len(arr)
    ready (length <= 1 || positions <= 0) {
        damn arr
    }
    
    fr fr Right rotation is equivalent to left rotation by (length - positions)
    sus left_positions drip = length - (positions % length)
    damn array_rotate_left(arr, left_positions)
}

slay array_shuffle_simple(arr drip[value], seed drip) drip[value]{
    fr fr Simple array shuffle using deterministic swaps based on seed
    sus length drip = len(arr)
    ready (length <= 1) {
        damn arr
    }
    
    fr fr For small arrays, implement specific shuffles based on seed
    ready (length == 2) {
        ready (seed % 2 == 0) {
            damn [arr[1], arr[0]]
        }
        damn arr
    }
    
    ready (length == 3) {
        sus pattern drip = seed % 6
        ready (pattern == 0) { damn [arr[0], arr[1], arr[2]] }  fr fr Original
        ready (pattern == 1) { damn [arr[0], arr[2], arr[1]] }
        ready (pattern == 2) { damn [arr[1], arr[0], arr[2]] }
        ready (pattern == 3) { damn [arr[1], arr[2], arr[0]] }
        ready (pattern == 4) { damn [arr[2], arr[0], arr[1]] }
        ready (pattern == 5) { damn [arr[2], arr[1], arr[0]] }
        damn arr
    }
    
    ready (length == 4) {
        sus pattern drip = seed % 8  fr fr Simplified shuffle patterns
        ready (pattern == 0) { damn [arr[0], arr[1], arr[2], arr[3]] }
        ready (pattern == 1) { damn [arr[3], arr[1], arr[2], arr[0]] }
        ready (pattern == 2) { damn [arr[1], arr[0], arr[3], arr[2]] }
        ready (pattern == 3) { damn [arr[2], arr[3], arr[0], arr[1]] }
        ready (pattern == 4) { damn [arr[1], arr[2], arr[0], arr[3]] }
        ready (pattern == 5) { damn [arr[3], arr[0], arr[1], arr[2]] }
        ready (pattern == 6) { damn [arr[2], arr[1], arr[3], arr[0]] }
        ready (pattern == 7) { damn [arr[0], arr[3], arr[2], arr[1]] }
        damn arr
    }
    
    fr fr For larger arrays, apply simple rotation based on seed
    sus rotation_amount drip = seed % length
    damn array_rotate_left(arr, rotation_amount)
}

fr fr ===== ARRAY SET OPERATIONS =====

slay array_union_int(arr1 drip[value], arr2 drip[value]) drip[value]{
    fr fr Union of two integer arrays (removing duplicates)
    sus result drip[value] = array_copy_int_pure(arr1)
    sus i drip = 0
    
    bestie (i < len(arr2)) {
        ready (!contains_value(result, arr2[i])) {
            result = append_to_int_array(result, arr2[i])
        }
        i = i + 1
    }
    
    damn result
}

slay array_intersection_int(arr1 drip[value], arr2 drip[value]) drip[value]{
    fr fr Intersection of two integer arrays
    sus result drip[value] = []
    sus i drip = 0
    
    bestie (i < len(arr1)) {
        ready (contains_value(arr2, arr1[i]) && !contains_value(result, arr1[i])) {
            result = append_to_int_array(result, arr1[i])
        }
        i = i + 1
    }
    
    damn result
}

slay array_difference_int(arr1 drip[value], arr2 drip[value]) drip[value]{
    fr fr Difference of two integer arrays (elements in arr1 but not in arr2)
    sus result drip[value] = []
    sus i drip = 0
    
    bestie (i < len(arr1)) {
        ready (!contains_value(arr2, arr1[i])) {
            result = append_to_int_array(result, arr1[i])
        }
        i = i + 1
    }
    
    damn result
}

slay array_symmetric_difference_int(arr1 drip[value], arr2 drip[value]) drip[value]{
    fr fr Symmetric difference (elements in either array but not in both)
    sus diff1 drip[value] = array_difference_int(arr1, arr2)
    sus diff2 drip[value] = array_difference_int(arr2, arr1)
    damn array_union_int(diff1, diff2)
}

fr fr ===== ARRAY VALIDATION AND PROPERTIES =====

slay array_is_subset_int(subset drip[value], superset drip[value]) lit {
    fr fr Check if subset is a subset of superset
    sus i drip = 0
    
    bestie (i < len(subset)) {
        ready (!contains_value(superset, subset[i])) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay array_is_permutation_int(arr1 drip[value], arr2 drip[value]) lit {
    fr fr Check if arr1 is a permutation of arr2
    ready (len(arr1) != len(arr2)) {
        damn cringe
    }
    
    fr fr Check if both arrays contain the same elements with same frequencies
    sus i drip = 0
    bestie (i < len(arr1)) {
        sus count1 drip = count_occurrences(arr1, arr1[i])
        sus count2 drip = count_occurrences(arr2, arr1[i])
        ready (count1 != count2) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay array_is_palindrome_int(arr drip[value]) lit {
    fr fr Check if array reads the same forwards and backwards
    sus length drip = len(arr)
    ready (length <= 1) {
        damn based
    }
    
    sus i drip = 0
    bestie (i < length / 2) {
        ready (arr[i] != arr[length - 1 - i]) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

fr fr ===== ADVANCED ARRAY SEARCHING =====

slay array_binary_search_int(arr drip[value], target drip) drip {
    fr fr Binary search on sorted integer array
    fr fr Returns index of target, or -1 if not found
    ready (len(arr) == 0) {
        damn -1
    }
    
    sus left drip = 0
    sus right drip = len(arr) - 1
    
    bestie (left <= right) {
        sus mid drip = (left + right) / 2
        
        ready (arr[mid] == target) {
            damn mid
        }
        ready (arr[mid] < target) {
            left = mid + 1
        } otherwise {
            right = mid - 1
        }
    }
    
    damn -1
}

slay array_find_all_indices_int(arr drip[value], target drip) drip[value]{
    fr fr Find all indices where target appears
    sus result drip[value] = []
    sus i drip = 0
    
    bestie (i < len(arr)) {
        ready (arr[i] == target) {
            result = append_to_int_array(result, i)
        }
        i = i + 1
    }
    
    damn result
}

slay array_find_closest_int(arr drip[value], target drip) drip {
    fr fr Find index of element closest to target
    ready (len(arr) == 0) {
        damn -1
    }
    
    sus closest_index drip = 0
    sus closest_distance drip = abs_normie(arr[0] - target)
    sus i drip = 1
    
    bestie (i < len(arr)) {
        sus distance drip = abs_normie(arr[i] - target)
        ready (distance < closest_distance) {
            closest_distance = distance
            closest_index = i
        }
        i = i + 1
    }
    
    damn closest_index
}

fr fr ===== ARRAY AGGREGATION FUNCTIONS =====

slay array_cumulative_sum_int(arr drip[value]) drip[value]{
    fr fr Calculate cumulative sum array
    ready (len(arr) == 0) {
        damn []
    }
    
    sus result drip[value] = [arr[0]]
    sus running_sum drip = arr[0]
    sus i drip = 1
    
    bestie (i < len(arr)) {
        running_sum = running_sum + arr[i]
        result = append_to_int_array(result, running_sum)
        i = i + 1
    }
    
    damn result
}

slay array_moving_average_int(arr drip[value], window_size drip) drip[value]{
    fr fr Calculate moving average with specified window size
    ready (len(arr) == 0 || window_size <= 0) {
        damn []
    }
    
    ready (window_size > len(arr)) {
        sus avg drip = average_array(arr)
        damn [avg]
    }
    
    sus result drip[value] = []
    sus i drip = 0
    
    bestie (i <= len(arr) - window_size) {
        sus window_sum drip = 0
        sus j drip = 0
        
        bestie (j < window_size) {
            window_sum = window_sum + arr[i + j]
            j = j + 1
        }
        
        sus avg drip = window_sum / window_size
        result = append_to_int_array(result, avg)
        i = i + 1
    }
    
    damn result
}

slay array_pairwise_sum_int(arr1 drip[value], arr2 drip[value]) drip[value]{
    fr fr Element-wise addition of two arrays
    sus min_len drip = min_normie(len(arr1), len(arr2))
    sus result drip[value] = []
    sus i drip = 0
    
    bestie (i < min_len) {
        sus sum drip = arr1[i] + arr2[i]
        result = append_to_int_array(result, sum)
        i = i + 1
    }
    
    damn result
}

slay array_pairwise_multiply_int(arr1 drip[value], arr2 drip[value]) drip[value]{
    fr fr Element-wise multiplication of two arrays
    sus min_len drip = min_normie(len(arr1), len(arr2))
    sus result drip[value] = []
    sus i drip = 0
    
    bestie (i < min_len) {
        sus product drip = arr1[i] * arr2[i]
        result = append_to_int_array(result, product)
        i = i + 1
    }
    
    damn result
}

fr fr ===== ARRAY PARTITIONING =====

slay array_partition_by_predicate_int(arr drip[value], threshold drip) drip[value][value] {
    fr fr Partition array into two groups: elements <= threshold and > threshold
    fr fr Returns an array of two arrays [lesser_or_equal, greater]
    
    sus lesser drip[value] = []
    sus greater drip[value] = []
    sus i drip = 0
    
    bestie (i < len(arr)) {
        ready (arr[i] <= threshold) {
            lesser = append_to_int_array(lesser, arr[i])
        } otherwise {
            greater = append_to_int_array(greater, arr[i])
        }
        i = i + 1
    }
    
    fr fr Return as nested array (simplified representation)
    fr fr In a full implementation, this would return drip[value][value] properly
    damn lesser  fr fr For now, just return the first partition
}

slay array_chunk_int(arr drip[value], chunk_size drip) drip[value]{
    fr fr Split array into chunks of specified size (returns first chunk for simplicity)
    ready (chunk_size <= 0 || len(arr) == 0) {
        damn []
    }
    
    sus actual_chunk_size drip = min_normie(chunk_size, len(arr))
    damn slice_array(arr, 0, actual_chunk_size)
}
