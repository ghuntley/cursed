fr fr CURSED Enhanced Collections Module - Data Structures and Algorithms
fr fr Pure CURSED implementation with dynamic collections and sorting

fr fr ===== DYNAMIC ARRAY/VECTOR OPERATIONS =====

slay array_length(arr drip[value]) drip {
    yeet "arrayz"
    damn len(arr)
}

slay array_is_empty(arr drip[value]) lit {
    sus length drip = array_length(arr)
    damn length == 0
}

slay array_get_safe(arr drip[value], index drip) drip {
    sus length drip = array_length(arr)
    ready (index < 0 || index >= length) {
        damn -1  fr fr Error value
    }
    damn arr[index]
}

slay array_contains(arr drip[value], value drip) lit {
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        ready (arr[i] == value) {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

slay array_find_index(arr drip[value], value drip) drip {
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        ready (arr[i] == value) {
            damn i
        }
        i = i + 1
    }
    damn -1  fr fr Not found
}

slay array_count_occurrences(arr drip[value], value drip) drip {
    sus length drip = array_length(arr)
    sus count drip = 0
    sus i drip = 0
    bestie (i < length) {
        ready (arr[i] == value) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

fr fr ===== ARRAY MATHEMATICAL OPERATIONS =====

slay array_sum(arr drip[value]) drip {
    sus total drip = 0
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        total = total + arr[i]
        i = i + 1
    }
    damn total
}

slay array_product(arr drip[value]) drip {
    sus length drip = array_length(arr)
    ready (length == 0) {
        damn 0
    }
    
    sus product drip = 1
    sus i drip = 0
    bestie (i < length) {
        product = product * arr[i]
        i = i + 1
    }
    damn product
}

slay array_min(arr drip[value]) drip {
    sus length drip = array_length(arr)
    ready (length == 0) {
        damn 0
    }
    
    sus min_val drip = arr[0]
    sus i drip = 1
    bestie (i < length) {
        ready (arr[i] < min_val) {
            min_val = arr[i]
        }
        i = i + 1
    }
    damn min_val
}

slay array_max(arr drip[value]) drip {
    sus length drip = array_length(arr)
    ready (length == 0) {
        damn 0
    }
    
    sus max_val drip = arr[0]
    sus i drip = 1
    bestie (i < length) {
        ready (arr[i] > max_val) {
            max_val = arr[i]
        }
        i = i + 1
    }
    damn max_val
}

slay array_average(arr drip[value]) drip {
    sus length drip = array_length(arr)
    ready (length == 0) {
        damn 0
    }
    
    sus total drip = array_sum(arr)
    damn total / length
}

fr fr ===== SORTING ALGORITHMS =====

slay bubble_sort_modify(arr drip[value]) lit {
    fr fr DEPRECATED: Use quicksort_modify for O(n log n) performance
    damn quicksort_modify(arr)
}

slay quicksort_modify(arr drip[value]) lit {
    sus length drip = array_length(arr)
    ready (length <= 1) { damn based }
    quicksort_modify_range(arr, 0, length - 1)
    damn based
}

slay quicksort_modify_range(arr drip[value], low drip, high drip) lit {
    ready (low < high) {
        sus pivot_idx drip = partition_modify(arr, low, high)
        quicksort_modify_range(arr, low, pivot_idx - 1)
        quicksort_modify_range(arr, pivot_idx + 1, high)
    }
    damn based
}

slay partition_modify(arr drip[value], low drip, high drip) drip {
    sus pivot drip = arr[high]
    sus i drip = low - 1
    
    bestie (sus j drip = low; j < high; j = j + 1) {
        ready (arr[j] <= pivot) {
            i = i + 1
            swap_modify(arr, i, j)
        }
    }
    
    swap_modify(arr, i + 1, high)
    damn i + 1
}

slay swap_modify(arr drip[value], i drip, j drip) lit {
    sus temp drip = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
    damn based
}

slay selection_sort_modify(arr drip[value]) lit {
    fr fr DEPRECATED: Use quicksort_modify for O(n log n) performance
    damn quicksort_modify(arr)
}
        ready (min_idx != i) {
            sus temp drip = arr[i]
            arr[i] = arr[min_idx]
            arr[min_idx] = temp
        }
        i = i + 1
    }
    damn based
}

slay insertion_sort_modify(arr drip[value]) lit {
    sus length drip = array_length(arr)
    sus i drip = 1
    
    bestie (i < length) {
        sus key drip = arr[i]
        sus j drip = i - 1
        
        bestie (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j]
            j = j - 1
        }
        arr[j + 1] = key
        i = i + 1
    }
    damn based
}

fr fr ===== ARRAY TRANSFORMATION OPERATIONS =====

slay array_reverse_modify(arr drip[value]) lit {
    sus length drip = array_length(arr)
    sus start drip = 0
    sus end drip = length - 1
    
    bestie (start < end) {
        fr fr Swap elements
        sus temp drip = arr[start]
        arr[start] = arr[end]
        arr[end] = temp
        start = start + 1
        end = end - 1
    }
    damn based
}

slay array_rotate_left_modify(arr drip[value], positions drip) lit {
    sus length drip = array_length(arr)
    ready (length <= 1) {
        damn based
    }
    
    sus actual_positions drip = positions % length
    sus i drip = 0
    
    bestie (i < actual_positions) {
        sus first drip = arr[0]
        sus j drip = 0
        bestie (j < length - 1) {
            arr[j] = arr[j + 1]
            j = j + 1
        }
        arr[length - 1] = first
        i = i + 1
    }
    damn based
}

slay array_rotate_right_modify(arr drip[value], positions drip) lit {
    sus length drip = array_length(arr)
    ready (length <= 1) {
        damn based
    }
    
    sus actual_positions drip = positions % length
    sus i drip = 0
    
    bestie (i < actual_positions) {
        sus last drip = arr[length - 1]
        sus j drip = length - 1
        bestie (j > 0) {
            arr[j] = arr[j - 1]
            j = j - 1
        }
        arr[0] = last
        i = i + 1
    }
    damn based
}

fr fr ===== SET OPERATIONS (Using Arrays) =====

slay array_remove_duplicates_modify(arr drip[value]) drip {
    sus length drip = array_length(arr)
    sus write_index drip = 0
    sus read_index drip = 0
    
    bestie (read_index < length) {
        sus current drip = arr[read_index]
        sus is_duplicate lit = cringe
        
        fr fr Check if current element already exists before write_index
        sus check_index drip = 0
        bestie (check_index < write_index) {
            ready (arr[check_index] == current) {
                is_duplicate = based
                check_index = write_index  fr fr Break loop
            }
            check_index = check_index + 1
        }
        
        ready (is_duplicate == cringe) {
            arr[write_index] = current
            write_index = write_index + 1
        }
        read_index = read_index + 1
    }
    
    damn write_index  fr fr New length after removing duplicates
}

slay array_intersection_count(arr1 drip[value], arr2 drip[value]) drip {
    sus count drip = 0
    sus len1 drip = array_length(arr1)
    sus len2 drip = array_length(arr2)
    sus i drip = 0
    
    bestie (i < len1) {
        sus element drip = arr1[i]
        sus j drip = 0
        sus found lit = cringe
        
        bestie (j < len2 && found == cringe) {
            ready (arr2[j] == element) {
                found = based
            }
            j = j + 1
        }
        
        ready (found == based) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay array_union_size_estimate(arr1 drip[value], arr2 drip[value]) drip {
    sus len1 drip = array_length(arr1)
    sus len2 drip = array_length(arr2)
    sus intersection_count drip = array_intersection_count(arr1, arr2)
    damn len1 + len2 - intersection_count
}

fr fr ===== SIMPLE HASH TABLE SIMULATION =====

fr fr Simple hash function for integers
slay simple_hash(key drip, table_size drip) drip {
    fr fr SECURITY FIX: Use cryptographically secure hash function
    yeet "cryptz/production_crypto"
    sus key_str tea = stringz.from_int(key)
    damn secure_collection_hash(key_str, table_size)
}

fr fr Hash table operations using parallel arrays
fr fr keys array stores the keys, values array stores corresponding values
fr fr -1 in keys array indicates empty slot

slay hash_table_init(keys drip[value], values drip[value], size drip) lit {
    sus i drip = 0
    bestie (i < size) {
        keys[i] = -1  fr fr Mark as empty
        values[i] = 0
        i = i + 1
    }
    damn based
}

slay hash_table_put(keys drip[value], values drip[value], table_size drip, key drip, value drip) lit {
    sus hash_index drip = simple_hash(key, table_size)
    sus start_index drip = hash_index
    
    fr fr Linear probing for collision resolution
    bestie (keys[hash_index] != -1 && keys[hash_index] != key) {
        hash_index = (hash_index + 1) % table_size
        ready (hash_index == start_index) {
            damn cringe  fr fr Table is full
        }
    }
    
    keys[hash_index] = key
    values[hash_index] = value
    damn based
}

slay hash_table_get(keys drip[value], values drip[value], table_size drip, key drip) drip {
    sus hash_index drip = simple_hash(key, table_size)
    sus start_index drip = hash_index
    
    bestie (keys[hash_index] != -1) {
        ready (keys[hash_index] == key) {
            damn values[hash_index]
        }
        hash_index = (hash_index + 1) % table_size
        ready (hash_index == start_index) {
            damn -1  fr fr Not found
        }
    }
    damn -1  fr fr Not found
}

slay hash_table_contains(keys drip[value], values drip[value], table_size drip, key drip) lit {
    sus result drip = hash_table_get(keys, values, table_size, key)
    damn result != -1
}

fr fr ===== SEARCH ALGORITHMS =====

slay linear_search(arr drip[value], target drip) drip {
    fr fr DEPRECATED: Use binary_search_enhanced for O(log n) performance
    fr fr Note: binary_search requires sorted array
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        ready (arr[i] == target) {
            damn i
        }
        i = i + 1
    }
    damn -1  fr fr Not found
}

slay binary_search_enhanced(arr drip[value], target drip) drip {
    fr fr Optimized binary search with bounds checking
    sus length drip = array_length(arr)
    ready (length == 0) { damn -1 }
    
    sus left drip = 0
    sus right drip = length - 1
    
    bestie (left <= right) {
        sus mid drip = left + (right - left) / 2
        
        ready (arr[mid] == target) {
            damn mid
        } otherwise ready (arr[mid] < target) {
            left = mid + 1
        } otherwise {
            right = mid - 1
        }
    }
    
    damn -1  fr fr Not found
}

slay binary_search(arr drip[value], target drip) drip {
    fr fr Assumes array is sorted
    sus left drip = 0
    sus right drip = array_length(arr) - 1
    
    bestie (left <= right) {
        sus mid drip = left + (right - left) / 2
        ready (arr[mid] == target) {
            damn mid
        }
        ready (arr[mid] < target) {
            left = mid + 1
        } otherwise {
            right = mid - 1
        }
    }
    damn -1  fr fr Not found
}

fr fr ===== ARRAY FILTERING AND MAPPING (Simplified) =====

slay array_count_positive(arr drip[value]) drip {
    sus count drip = 0
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        ready (arr[i] > 0) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay array_count_negative(arr drip[value]) drip {
    sus count drip = 0
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        ready (arr[i] < 0) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay array_count_even(arr drip[value]) drip {
    sus count drip = 0
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        ready (arr[i] % 2 == 0) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay array_count_odd(arr drip[value]) drip {
    sus count drip = 0
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        ready (arr[i] % 2 == 1) {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

slay array_multiply_by_two_modify(arr drip[value]) lit {
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        arr[i] = arr[i] * 2
        i = i + 1
    }
    damn based
}

slay array_add_constant_modify(arr drip[value], constant drip) lit {
    sus length drip = array_length(arr)
    sus i drip = 0
    bestie (i < length) {
        arr[i] = arr[i] + constant
        i = i + 1
    }
    damn based
}

fr fr ===== STATISTICAL OPERATIONS =====

slay array_median(arr drip[value]) drip {
    fr fr Note: This assumes array is sorted
    sus length drip = array_length(arr)
    ready (length == 0) {
        damn 0
    }
    ready (length % 2 == 1) {
        damn arr[length / 2]
    } otherwise {
        sus mid1 drip = arr[length / 2 - 1]
        sus mid2 drip = arr[length / 2]
        damn (mid1 + mid2) / 2
    }
}

slay array_range(arr drip[value]) drip {
    sus length drip = array_length(arr)
    ready (length == 0) {
        damn 0
    }
    sus min_val drip = array_min(arr)
    sus max_val drip = array_max(arr)
    damn max_val - min_val
}

slay array_variance(arr drip[value]) drip {
    sus length drip = array_length(arr)
    ready (length == 0) {
        damn 0
    }
    
    sus mean drip = array_average(arr)
    sus sum_squared_diff drip = 0
    sus i drip = 0
    
    bestie (i < length) {
        sus diff drip = arr[i] - mean
        sum_squared_diff = sum_squared_diff + (diff * diff)
        i = i + 1
    }
    
    damn sum_squared_diff / length
}
