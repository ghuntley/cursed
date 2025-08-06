fr fr CURSED Array Operations Module - Pure CURSED Implementation
fr fr Comprehensive array manipulation operations without FFI dependencies

fr fr === CORE ARRAY CREATION ===

slay array_new() [tea] {
    sus result [tea] = []
    damn result
}

slay array_with_capacity(capacity normie) [tea] {
    sus result [tea] = []
    damn result
}

slay array_from_slice(slice [tea]) [tea] {
    sus result [tea] = []
    bestie i := 0; i < len(slice); i++ {
        result = append(result, slice[i])
    }
    damn result
}

slay array_fill(size normie, value tea) [tea] {
    sus result [tea] = []
    bestie i := 0; i < size; i++ {
        result = append(result, value)
    }
    damn result
}

slay array_range(start normie, end normie) [normie] {
    sus result [normie] = []
    bestie i := start; i < end; i++ {
        result = append(result, i)
    }
    damn result
}

fr fr === BASIC ARRAY OPERATIONS ===

slay array_length(arr [tea]) normie {
    damn len(arr)
}

slay array_is_empty(arr [tea]) lit {
    damn len(arr) == 0
}

slay array_get(arr [tea], index normie) tea {
    lowkey index < 0 || index >= len(arr) {
        damn ""
    }
    damn arr[index]
}

slay array_set(arr [tea], index normie, value tea) [tea] {
    lowkey index < 0 || index >= len(arr) {
        damn arr
    }
    sus result [tea] = []
    bestie i := 0; i < len(arr); i++ {
        lowkey i == index {
            result = append(result, value)
        } nah {
            result = append(result, arr[i])
        }
    }
    damn result
}

slay array_push(arr [tea], value tea) [tea] {
    damn append(arr, value)
}

slay array_pop(arr [tea]) ([tea], tea) {
    lowkey len(arr) == 0 {
        damn (arr, "")
    }
    sus last_index normie = len(arr) - 1
    sus last_value tea = arr[last_index]
    sus result [tea] = []
    bestie i := 0; i < last_index; i++ {
        result = append(result, arr[i])
    }
    damn (result, last_value)
}

slay array_insert(arr [tea], index normie, value tea) [tea] {
    lowkey index < 0 {
        damn array_push(arr, value)
    }
    lowkey index >= len(arr) {
        damn array_push(arr, value)
    }
    
    sus result [tea] = []
    bestie i := 0; i < index; i++ {
        result = append(result, arr[i])
    }
    result = append(result, value)
    bestie i := index; i < len(arr); i++ {
        result = append(result, arr[i])
    }
    damn result
}

slay array_remove(arr [tea], index normie) ([tea], tea) {
    lowkey index < 0 || index >= len(arr) {
        damn (arr, "")
    }
    
    sus removed_value tea = arr[index]
    sus result [tea] = []
    bestie i := 0; i < len(arr); i++ {
        lowkey i != index {
            result = append(result, arr[i])
        }
    }
    damn (result, removed_value)
}

fr fr === SEARCHING OPERATIONS ===

slay array_find(arr [tea], value tea) normie {
    bestie i := 0; i < len(arr); i++ {
        lowkey arr[i] == value {
            damn i
        }
    }
    damn -1
}

slay array_contains(arr [tea], value tea) lit {
    damn array_find(arr, value) != -1
}

slay array_find_last(arr [tea], value tea) normie {
    sus last_index normie = -1
    bestie i := 0; i < len(arr); i++ {
        lowkey arr[i] == value {
            last_index = i
        }
    }
    damn last_index
}

slay array_count(arr [tea], value tea) normie {
    sus count normie = 0
    bestie i := 0; i < len(arr); i++ {
        lowkey arr[i] == value {
            count = count + 1
        }
    }
    damn count
}

fr fr === ARRAY MANIPULATION ===

slay array_reverse(arr [tea]) [tea] {
    sus result [tea] = []
    sus i normie = len(arr) - 1
    bestie i >= 0 {
        result = append(result, arr[i])
        i = i - 1
    }
    damn result
}

slay array_slice(arr [tea], start normie, end normie) [tea] {
    lowkey start < 0 { start = 0 }
    lowkey end > len(arr) { end = len(arr) }
    lowkey start >= end { damn [] }
    
    sus result [tea] = []
    bestie i := start; i < end; i++ {
        result = append(result, arr[i])
    }
    damn result
}

slay array_concat(arr1 [tea], arr2 [tea]) [tea] {
    sus result [tea] = []
    bestie i := 0; i < len(arr1); i++ {
        result = append(result, arr1[i])
    }
    bestie i := 0; i < len(arr2); i++ {
        result = append(result, arr2[i])
    }
    damn result
}

slay array_join(arr [tea], separator tea) tea {
    lowkey len(arr) == 0 {
        damn ""
    }
    
    sus result tea = arr[0]
    bestie i := 1; i < len(arr); i++ {
        result = result + separator + arr[i]
    }
    damn result
}

fr fr === FILTERING AND MAPPING ===

slay array_filter(arr [tea], predicate slay(tea) lit) [tea] {
    sus result [tea] = []
    bestie i := 0; i < len(arr); i++ {
        lowkey predicate(arr[i]) {
            result = append(result, arr[i])
        }
    }
    damn result
}

slay array_map(arr [tea], mapper slay(tea) tea) [tea] {
    sus result [tea] = []
    bestie i := 0; i < len(arr); i++ {
        sus mapped tea = mapper(arr[i])
        result = append(result, mapped)
    }
    damn result
}

slay array_reduce(arr [tea], initial tea, reducer slay(tea, tea) tea) tea {
    sus accumulator tea = initial
    bestie i := 0; i < len(arr); i++ {
        accumulator = reducer(accumulator, arr[i])
    }
    damn accumulator
}

fr fr === SORTING OPERATIONS ===

slay array_sort_strings(arr [tea]) [tea] {
    lowkey len(arr) <= 1 {
        damn arr
    }
    
    fr fr Simple bubble sort for strings
    sus result [tea] = []
    bestie i := 0; i < len(arr); i++ {
        result = append(result, arr[i])
    }
    
    bestie i := 0; i < len(result) - 1; i++ {
        bestie j := 0; j < len(result) - 1 - i; j++ {
            lowkey string_compare(result[j], result[j + 1]) > 0 {
                sus temp tea = result[j]
                result = array_set(result, j, result[j + 1])
                result = array_set(result, j + 1, temp)
            }
        }
    }
    
    damn result
}

slay array_sort_numbers(arr [normie]) [normie] {
    lowkey len(arr) <= 1 {
        damn arr
    }
    
    fr fr Simple bubble sort for numbers
    sus result [normie] = []
    bestie i := 0; i < len(arr); i++ {
        result = append(result, arr[i])
    }
    
    bestie i := 0; i < len(result) - 1; i++ {
        bestie j := 0; j < len(result) - 1 - i; j++ {
            lowkey result[j] > result[j + 1] {
                sus temp normie = result[j]
                result[j] = result[j + 1]
                result[j + 1] = temp
            }
        }
    }
    
    damn result
}

fr fr === ARRAY COMPARISON ===

slay array_equals(arr1 [tea], arr2 [tea]) lit {
    lowkey len(arr1) != len(arr2) {
        damn cringe
    }
    
    bestie i := 0; i < len(arr1); i++ {
        lowkey arr1[i] != arr2[i] {
            damn cringe
        }
    }
    damn based
}

slay array_starts_with(arr [tea], prefix [tea]) lit {
    lowkey len(prefix) > len(arr) {
        damn cringe
    }
    
    bestie i := 0; i < len(prefix); i++ {
        lowkey arr[i] != prefix[i] {
            damn cringe
        }
    }
    damn based
}

slay array_ends_with(arr [tea], suffix [tea]) lit {
    lowkey len(suffix) > len(arr) {
        damn cringe
    }
    
    sus start_index normie = len(arr) - len(suffix)
    bestie i := 0; i < len(suffix); i++ {
        lowkey arr[start_index + i] != suffix[i] {
            damn cringe
        }
    }
    damn based
}

fr fr === ARRAY CHUNKING ===

slay array_chunk(arr [tea], chunk_size normie) [[tea]] {
    lowkey chunk_size <= 0 {
        damn []
    }
    
    sus result [[tea]] = []
    sus i normie = 0
    
    bestie i < len(arr) {
        sus chunk [tea] = []
        sus end normie = i + chunk_size
        lowkey end > len(arr) { end = len(arr) }
        
        bestie j := i; j < end; j++ {
            chunk = append(chunk, arr[j])
        }
        
        result = append(result, chunk)
        i = end
    }
    
    damn result
}

slay array_flatten(nested_arr [[tea]]) [tea] {
    sus result [tea] = []
    bestie i := 0; i < len(nested_arr); i++ {
        bestie j := 0; j < len(nested_arr[i]); j++ {
            result = append(result, nested_arr[i][j])
        }
    }
    damn result
}

fr fr === ARRAY UTILITIES ===

slay array_unique(arr [tea]) [tea] {
    sus result [tea] = []
    bestie i := 0; i < len(arr); i++ {
        lowkey !array_contains(result, arr[i]) {
            result = append(result, arr[i])
        }
    }
    damn result
}

slay array_intersection(arr1 [tea], arr2 [tea]) [tea] {
    sus result [tea] = []
    bestie i := 0; i < len(arr1); i++ {
        lowkey array_contains(arr2, arr1[i]) && !array_contains(result, arr1[i]) {
            result = append(result, arr1[i])
        }
    }
    damn result
}

slay array_difference(arr1 [tea], arr2 [tea]) [tea] {
    sus result [tea] = []
    bestie i := 0; i < len(arr1); i++ {
        lowkey !array_contains(arr2, arr1[i]) {
            result = append(result, arr1[i])
        }
    }
    damn result
}

slay array_union(arr1 [tea], arr2 [tea]) [tea] {
    sus result [tea] = []
    bestie i := 0; i < len(arr1); i++ {
        result = append(result, arr1[i])
    }
    bestie i := 0; i < len(arr2); i++ {
        lowkey !array_contains(result, arr2[i]) {
            result = append(result, arr2[i])
        }
    }
    damn result
}

fr fr === HELPER FUNCTIONS ===

slay string_compare(a tea, b tea) normie {
    lowkey a == b { damn 0 }
    lowkey a < b { damn -1 }
    damn 1
}

slay array_sum_numbers(arr [normie]) normie {
    sus sum normie = 0
    bestie i := 0; i < len(arr); i++ {
        sum = sum + arr[i]
    }
    damn sum
}

slay array_average_numbers(arr [normie]) meal {
    lowkey len(arr) == 0 {
        damn 0.0
    }
    sus sum normie = array_sum_numbers(arr)
    damn sum / len(arr)
}

slay array_min_numbers(arr [normie]) normie {
    lowkey len(arr) == 0 {
        damn 0
    }
    
    sus min_val normie = arr[0]
    bestie i := 1; i < len(arr); i++ {
        lowkey arr[i] < min_val {
            min_val = arr[i]
        }
    }
    damn min_val
}

slay array_max_numbers(arr [normie]) normie {
    lowkey len(arr) == 0 {
        damn 0
    }
    
    sus max_val normie = arr[0]
    bestie i := 1; i < len(arr); i++ {
        lowkey arr[i] > max_val {
            max_val = arr[i]
        }
    }
    damn max_val
}

fr fr === ARRAY VALIDATION ===

slay array_all(arr [tea], predicate slay(tea) lit) lit {
    bestie i := 0; i < len(arr); i++ {
        lowkey !predicate(arr[i]) {
            damn cringe
        }
    }
    damn based
}

slay array_any(arr [tea], predicate slay(tea) lit) lit {
    bestie i := 0; i < len(arr); i++ {
        lowkey predicate(arr[i]) {
            damn based
        }
    }
    damn cringe
}

slay array_none(arr [tea], predicate slay(tea) lit) lit {
    damn !array_any(arr, predicate)
}

fr fr === ARRAY ITERATORS ===

slay array_for_each(arr [tea], action slay(tea) cringe) cringe {
    bestie i := 0; i < len(arr); i++ {
        action(arr[i])
    }
    damn cringe
}

slay array_for_each_indexed(arr [tea], action slay(normie, tea) cringe) cringe {
    bestie i := 0; i < len(arr); i++ {
        action(i, arr[i])
    }
    damn cringe
}

fr fr === SPECIALIZED ARRAY OPERATIONS ===

slay array_zip(arr1 [tea], arr2 [tea]) [(tea, tea)] {
    sus result [(tea, tea)] = []
    sus min_len normie = len(arr1)
    lowkey len(arr2) < min_len { min_len = len(arr2) }
    
    bestie i := 0; i < min_len; i++ {
        sus pair (tea, tea) = (arr1[i], arr2[i])
        result = append(result, pair)
    }
    damn result
}

slay array_transpose(matrix [[tea]]) [[tea]] {
    lowkey len(matrix) == 0 {
        damn []
    }
    
    sus rows normie = len(matrix)
    sus cols normie = len(matrix[0])
    sus result [[tea]] = []
    
    bestie col := 0; col < cols; col++ {
        sus new_row [tea] = []
        bestie row := 0; row < rows; row++ {
            lowkey col < len(matrix[row]) {
                new_row = append(new_row, matrix[row][col])
            } nah {
                new_row = append(new_row, "")
            }
        }
        result = append(result, new_row)
    }
    
    damn result
}

fr fr === ARRAY PERFORMANCE MONITORING ===

slay array_memory_usage(arr [tea]) normie {
    fr fr Estimate memory usage (simplified)
    damn len(arr) * 32 fr fr Assume 32 bytes per string element
}

slay array_capacity_info(arr [tea]) (normie, normie) {
    sus length normie = len(arr)
    sus capacity normie = length * 2 fr fr Estimated capacity
    damn (length, capacity)
}
