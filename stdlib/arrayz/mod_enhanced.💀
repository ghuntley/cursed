fr fr Enhanced ARRAYZ Module - Comprehensive Array Operations for CURSED
fr fr Production-ready implementation with advanced array manipulation

fr fr ===== CORE ARRAY CREATION =====

slay new_array() tea[value]{
    sus result tea[value] = []
    damn result
}

slay array_new() tea[value]{
    damn new_array()
}

slay array_with_capacity(capacity normie) tea[value]{
    fr fr Create array with estimated capacity
    sus result tea[value] = []
    damn result
}

slay array_from_values(values ...tea) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < len_variadic(values) {
        result = append_to_array(result, values[i])
        i = i + 1
    }
    damn result
}

slay array_fill(size normie, value tea) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < size {
        result = append_to_array(result, value)
        i = i + 1
    }
    damn result
}

slay array_range(start normie, end normie) normie[value]{
    sus result normie[value] = []
    sus i normie = start
    bestie i < end {
        result = append_to_int_array(result, i)
        i = i + 1
    }
    damn result
}

slay array_range_step(start normie, end normie, step normie) normie[value]{
    check step <= 0 {
        damn []
    }
    
    sus result normie[value] = []
    sus i normie = start
    bestie i < end {
        result = append_to_int_array(result, i)
        i = i + step
    }
    damn result
}

fr fr ===== BASIC ARRAY OPERATIONS =====

slay length(arr tea[value]) normie {
    damn len_array(arr)
}

slay len(arr tea[value]) normie {
    damn length(arr)
}

slay size(arr tea[value]) normie {
    damn length(arr)
}

slay is_empty(arr tea[value]) lit {
    damn length(arr) == 0
}

slay get(arr tea[value], index normie) tea {
    check index < 0 || index >= length(arr) {
        damn ""
    }
    damn arr[index]
}

slay set(arr tea[value], index normie, value tea) tea[value]{
    check index < 0 || index >= length(arr) {
        damn arr
    }
    
    sus result tea[value] = []
    sus i normie = 0
    bestie i < length(arr) {
        check i == index {
            result = append_to_array(result, value)
        } highkey {
            result = append_to_array(result, arr[i])
        }
        i = i + 1
    }
    damn result
}

slay first(arr tea[value]) tea {
    check is_empty(arr) {
        damn ""
    }
    damn arr[0]
}

slay last(arr tea[value]) tea {
    check is_empty(arr) {
        damn ""
    }
    damn arr[length(arr) - 1]
}

slay push(arr tea[value], value tea) tea[value]{
    damn append_to_array(arr, value)
}

slay append(arr tea[value], value tea) tea[value]{
    damn push(arr, value)
}

slay pop(arr tea[value]) (tea[value], tea) {
    check is_empty(arr) {
        damn (arr, "")
    }
    
    sus last_index normie = length(arr) - 1
    sus last_value tea = arr[last_index]
    sus result tea[value] = slice(arr, 0, last_index)
    damn (result, last_value)
}

slay shift(arr tea[value]) (tea[value], tea) {
    check is_empty(arr) {
        damn (arr, "")
    }
    
    sus first_value tea = arr[0]
    sus result tea[value] = slice(arr, 1, length(arr))
    damn (result, first_value)
}

slay unshift(arr tea[value], value tea) tea[value]{
    sus result tea[value] = []
    result = append_to_array(result, value)
    sus i normie = 0
    bestie i < length(arr) {
        result = append_to_array(result, arr[i])
        i = i + 1
    }
    damn result
}

fr fr ===== ARRAY INSERTION AND REMOVAL =====

slay insert(arr tea[value], index normie, value tea) tea[value]{
    check index <= 0 {
        damn unshift(arr, value)
    }
    check index >= length(arr) {
        damn push(arr, value)
    }
    
    sus result tea[value] = []
    sus i normie = 0
    
    bestie i < index {
        result = append_to_array(result, arr[i])
        i = i + 1
    }
    
    result = append_to_array(result, value)
    
    bestie i < length(arr) {
        result = append_to_array(result, arr[i])
        i = i + 1
    }
    
    damn result
}

slay insert_multiple(arr tea[value], index normie, values tea[value]) tea[value]{
    check index <= 0 {
        damn concat(values, arr)
    }
    check index >= length(arr) {
        damn concat(arr, values)
    }
    
    sus before tea[value] = slice(arr, 0, index)
    sus after tea[value] = slice(arr, index, length(arr))
    sus result tea[value] = concat(before, values)
    damn concat(result, after)
}

slay remove(arr tea[value], index normie) (tea[value], tea) {
    check index < 0 || index >= length(arr) {
        damn (arr, "")
    }
    
    sus removed_value tea = arr[index]
    sus result tea[value] = []
    sus i normie = 0
    
    bestie i < length(arr) {
        check i != index {
            result = append_to_array(result, arr[i])
        }
        i = i + 1
    }
    
    damn (result, removed_value)
}

slay remove_range(arr tea[value], start normie, count normie) tea[value]{
    check start < 0 || start >= length(arr) || count <= 0 {
        damn arr
    }
    
    sus end normie = start + count
    check end > length(arr) {
        end = length(arr)
    }
    
    sus before tea[value] = slice(arr, 0, start)
    sus after tea[value] = slice(arr, end, length(arr))
    damn concat(before, after)
}

slay clear(arr tea[value]) tea[value]{
    damn new_array()
}

fr fr ===== ARRAY SLICING AND COPYING =====

slay slice(arr tea[value], start normie, end normie) tea[value]{
    check start < 0 {
        start = 0
    }
    check end > length(arr) {
        end = length(arr)
    }
    check start >= end {
        damn new_array()
    }
    
    sus result tea[value] = []
    sus i normie = start
    bestie i < end {
        result = append_to_array(result, arr[i])
        i = i + 1
    }
    damn result
}

slay subarray(arr tea[value], start normie, length normie) tea[value]{
    check length <= 0 {
        damn new_array()
    }
    damn slice(arr, start, start + length)
}

slay copy(arr tea[value]) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < length(arr) {
        result = append_to_array(result, arr[i])
        i = i + 1
    }
    damn result
}

slay clone(arr tea[value]) tea[value]{
    damn copy(arr)
}

fr fr ===== ARRAY CONCATENATION =====

slay concat(arr1 tea[value], arr2 tea[value]) tea[value]{
    sus result tea[value] = copy(arr1)
    sus i normie = 0
    bestie i < length(arr2) {
        result = append_to_array(result, arr2[i])
        i = i + 1
    }
    damn result
}

slay concat_multiple(arrays tea[value][value]) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < len_2d_array(arrays) {
        sus j normie = 0
        bestie j < length(arrays[i]) {
            result = append_to_array(result, arrays[i][j])
            j = j + 1
        }
        i = i + 1
    }
    damn result
}

slay join(arr tea[value], separator tea) tea {
    check is_empty(arr) {
        damn ""
    }
    check length(arr) == 1 {
        damn arr[0]
    }
    
    sus result tea = arr[0]
    sus i normie = 1
    bestie i < length(arr) {
        result = result + separator + arr[i]
        i = i + 1
    }
    damn result
}

fr fr ===== SEARCHING OPERATIONS =====

slay find(arr tea[value], value tea) normie {
    sus i normie = 0
    bestie i < length(arr) {
        check arr[i] == value {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay index_of(arr tea[value], value tea) normie {
    damn find(arr, value)
}

slay last_index_of(arr tea[value], value tea) normie {
    sus last_found normie = -1
    sus i normie = 0
    bestie i < length(arr) {
        check arr[i] == value {
            last_found = i
        }
        i = i + 1
    }
    damn last_found
}

slay find_index(arr tea[value], predicate slay(tea) lit) normie {
    sus i normie = 0
    bestie i < length(arr) {
        check predicate(arr[i]) {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay find_last_index(arr tea[value], predicate slay(tea) lit) normie {
    sus last_found normie = -1
    sus i normie = 0
    bestie i < length(arr) {
        check predicate(arr[i]) {
            last_found = i
        }
        i = i + 1
    }
    damn last_found
}

slay contains(arr tea[value], value tea) lit {
    damn find(arr, value) != -1
}

slay includes(arr tea[value], value tea) lit {
    damn contains(arr, value)
}

slay count(arr tea[value], value tea) normie {
    sus count normie = 0
    sus i normie = 0
    bestie i < length(arr) {
        check arr[i] == value {
            count = count + 1
        }
        i = i + 1
    }
    damn count
}

fr fr ===== ARRAY MANIPULATION =====

slay reverse(arr tea[value]) tea[value]{
    sus result tea[value] = []
    sus i normie = length(arr) - 1
    bestie i >= 0 {
        result = append_to_array(result, arr[i])
        i = i - 1
    }
    damn result
}

slay shuffle(arr tea[value]) tea[value]{
    sus result tea[value] = copy(arr)
    sus len normie = length(result)
    
    fr fr Fisher-Yates shuffle
    sus i normie = len - 1
    bestie i > 0 {
        sus j normie = random_int_range(0, i + 1)
        sus temp tea = result[i]
        result = set(result, i, result[j])
        result = set(result, j, temp)
        i = i - 1
    }
    
    damn result
}

slay rotate_left(arr tea[value], positions normie) tea[value]{
    sus len normie = length(arr)
    check len <= 1 || positions <= 0 {
        damn copy(arr)
    }
    
    positions = positions % len
    sus left tea[value] = slice(arr, positions, len)
    sus right tea[value] = slice(arr, 0, positions)
    damn concat(left, right)
}

slay rotate_right(arr tea[value], positions normie) tea[value]{
    sus len normie = length(arr)
    check len <= 1 || positions <= 0 {
        damn copy(arr)
    }
    
    positions = positions % len
    damn rotate_left(arr, len - positions)
}

fr fr ===== FUNCTIONAL OPERATIONS =====

slay map(arr tea[value], mapper slay(tea) tea) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < length(arr) {
        sus mapped tea = mapper(arr[i])
        result = append_to_array(result, mapped)
        i = i + 1
    }
    damn result
}

slay filter(arr tea[value], predicate slay(tea) lit) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < length(arr) {
        check predicate(arr[i]) {
            result = append_to_array(result, arr[i])
        }
        i = i + 1
    }
    damn result
}

slay reduce(arr tea[value], initial tea, reducer slay(tea, tea) tea) tea {
    sus accumulator tea = initial
    sus i normie = 0
    bestie i < length(arr) {
        accumulator = reducer(accumulator, arr[i])
        i = i + 1
    }
    damn accumulator
}

slay reduce_right(arr tea[value], initial tea, reducer slay(tea, tea) tea) tea {
    sus accumulator tea = initial
    sus i normie = length(arr) - 1
    bestie i >= 0 {
        accumulator = reducer(accumulator, arr[i])
        i = i - 1
    }
    damn accumulator
}

slay for_each(arr tea[value], action slay(tea) cringe) cringe {
    sus i normie = 0
    bestie i < length(arr) {
        action(arr[i])
        i = i + 1
    }
    damn cringe
}

slay for_each_indexed(arr tea[value], action slay(normie, tea) cringe) cringe {
    sus i normie = 0
    bestie i < length(arr) {
        action(i, arr[i])
        i = i + 1
    }
    damn cringe
}

fr fr ===== ARRAY TESTING =====

slay every(arr tea[value], predicate slay(tea) lit) lit {
    sus i normie = 0
    bestie i < length(arr) {
        check !predicate(arr[i]) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay some(arr tea[value], predicate slay(tea) lit) lit {
    sus i normie = 0
    bestie i < length(arr) {
        check predicate(arr[i]) {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

slay none(arr tea[value], predicate slay(tea) lit) lit {
    damn !some(arr, predicate)
}

slay all(arr tea[value], predicate slay(tea) lit) lit {
    damn every(arr, predicate)
}

slay any(arr tea[value], predicate slay(tea) lit) lit {
    damn some(arr, predicate)
}

fr fr ===== ARRAY SORTING =====

slay sort(arr tea[value]) tea[value]{
    damn sort_strings(arr)
}

slay sort_strings(arr tea[value]) tea[value]{
    sus len normie = length(arr)
    check len <= 1 {
        damn copy(arr)
    }
    
    fr fr Quick sort implementation
    sus pivot tea = arr[len / 2]
    sus less tea[value] = []
    sus equal tea[value] = []
    sus greater tea[value] = []
    
    sus i normie = 0
    bestie i < len {
        sus item tea = arr[i]
        check string_compare_values(item, pivot) < 0 {
            less = append_to_array(less, item)
        } highkey string_compare_values(item, pivot) > 0 {
            greater = append_to_array(greater, item)
        } highkey {
            equal = append_to_array(equal, item)
        }
        i = i + 1
    }
    
    sus sorted_less tea[value] = sort_strings(less)
    sus sorted_greater tea[value] = sort_strings(greater)
    
    sus result tea[value] = concat(sorted_less, equal)
    damn concat(result, sorted_greater)
}

slay sort_numbers(arr normie[value]) normie[value]{
    sus len normie = len_int_array(arr)
    check len <= 1 {
        damn copy_int_array(arr)
    }
    
    fr fr Quick sort for numbers
    sus pivot normie = arr[len / 2]
    sus less normie[value] = []
    sus equal normie[value] = []
    sus greater normie[value] = []
    
    sus i normie = 0
    bestie i < len {
        sus item normie = arr[i]
        check item < pivot {
            less = append_to_int_array(less, item)
        } highkey item > pivot {
            greater = append_to_int_array(greater, item)
        } highkey {
            equal = append_to_int_array(equal, item)
        }
        i = i + 1
    }
    
    sus sorted_less normie[value] = sort_numbers(less)
    sus sorted_greater normie[value] = sort_numbers(greater)
    
    sus result normie[value] = concat_int_arrays(sorted_less, equal)
    damn concat_int_arrays(result, sorted_greater)
}

slay sort_by(arr tea[value], key_func slay(tea) tea) tea[value]{
    fr fr Create pairs of (key, original_value)
    sus pairs [](tea, tea) = []
    sus i normie = 0
    bestie i < length(arr) {
        sus key tea = key_func(arr[i])
        sus pair (tea, tea) = (key, arr[i])
        pairs = append_to_pair_array(pairs, pair)
        i = i + 1
    }
    
    fr fr Sort pairs by key
    sus sorted_pairs [](tea, tea) = sort_pairs_by_first(pairs)
    
    fr fr Extract values
    sus result tea[value] = []
    i = 0
    bestie i < len_pair_array(sorted_pairs) {
        sus (key, value) = sorted_pairs[i]
        result = append_to_array(result, value)
        i = i + 1
    }
    
    damn result
}

fr fr ===== ARRAY COMPARISON =====

slay equals(arr1 tea[value], arr2 tea[value]) lit {
    check length(arr1) != length(arr2) {
        damn cringe
    }
    
    sus i normie = 0
    bestie i < length(arr1) {
        check arr1[i] != arr2[i] {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay starts_with(arr tea[value], prefix tea[value]) lit {
    check length(prefix) > length(arr) {
        damn cringe
    }
    
    sus i normie = 0
    bestie i < length(prefix) {
        check arr[i] != prefix[i] {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay ends_with(arr tea[value], suffix tea[value]) lit {
    sus arr_len normie = length(arr)
    sus suffix_len normie = length(suffix)
    
    check suffix_len > arr_len {
        damn cringe
    }
    
    sus start_index normie = arr_len - suffix_len
    sus i normie = 0
    bestie i < suffix_len {
        check arr[start_index + i] != suffix[i] {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

fr fr ===== ARRAY UTILITIES =====

slay unique(arr tea[value]) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < length(arr) {
        check !contains(result, arr[i]) {
            result = append_to_array(result, arr[i])
        }
        i = i + 1
    }
    damn result
}

slay distinct(arr tea[value]) tea[value]{
    damn unique(arr)
}

slay intersection(arr1 tea[value], arr2 tea[value]) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < length(arr1) {
        check contains(arr2, arr1[i]) && !contains(result, arr1[i]) {
            result = append_to_array(result, arr1[i])
        }
        i = i + 1
    }
    damn result
}

slay union(arr1 tea[value], arr2 tea[value]) tea[value]{
    sus result tea[value] = unique(arr1)
    sus i normie = 0
    bestie i < length(arr2) {
        check !contains(result, arr2[i]) {
            result = append_to_array(result, arr2[i])
        }
        i = i + 1
    }
    damn result
}

slay difference(arr1 tea[value], arr2 tea[value]) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < length(arr1) {
        check !contains(arr2, arr1[i]) {
            result = append_to_array(result, arr1[i])
        }
        i = i + 1
    }
    damn result
}

slay symmetric_difference(arr1 tea[value], arr2 tea[value]) tea[value]{
    sus diff1 tea[value] = difference(arr1, arr2)
    sus diff2 tea[value] = difference(arr2, arr1)
    damn concat(diff1, diff2)
}

fr fr ===== ARRAY CHUNKING AND GROUPING =====

slay chunk(arr tea[value], size normie) tea[value][value] {
    check size <= 0 {
        damn []
    }
    
    sus result tea[value][value] = []
    sus i normie = 0
    
    bestie i < length(arr) {
        sus chunk_array tea[value] = []
        sus end normie = i + size
        check end > length(arr) {
            end = length(arr)
        }
        
        sus j normie = i
        bestie j < end {
            chunk_array = append_to_array(chunk_array, arr[j])
            j = j + 1
        }
        
        result = append_to_2d_array(result, chunk_array)
        i = end
    }
    
    damn result
}

slay flatten(nested_arr tea[value][value]) tea[value]{
    sus result tea[value] = []
    sus i normie = 0
    bestie i < len_2d_array(nested_arr) {
        sus j normie = 0
        bestie j < length(nested_arr[i]) {
            result = append_to_array(result, nested_arr[i][j])
            j = j + 1
        }
        i = i + 1
    }
    damn result
}

slay group_by(arr tea[value], key_func slay(tea) tea) tea[value][value] {
    sus groups tea[value][value] = []
    sus processed tea[value] = []
    
    sus i normie = 0
    bestie i < length(arr) {
        sus item tea = arr[i]
        check !contains(processed, item) {
            sus key tea = key_func(item)
            sus group tea[value] = []
            
            sus j normie = 0
            bestie j < length(arr) {
                check key_func(arr[j]) == key {
                    group = append_to_array(group, arr[j])
                    processed = append_to_array(processed, arr[j])
                }
                j = j + 1
            }
            
            groups = append_to_2d_array(groups, group)
        }
        i = i + 1
    }
    
    damn groups
}

fr fr ===== ARRAY STATISTICS =====

slay sum_numbers(arr normie[value]) normie {
    sus total normie = 0
    sus i normie = 0
    bestie i < len_int_array(arr) {
        total = total + arr[i]
        i = i + 1
    }
    damn total
}

slay average_numbers(arr normie[value]) meal {
    sus len normie = len_int_array(arr)
    check len == 0 {
        damn 0.0
    }
    sus sum normie = sum_numbers(arr)
    damn sum / len
}

slay min_numbers(arr normie[value]) normie {
    sus len normie = len_int_array(arr)
    check len == 0 {
        damn 0
    }
    
    sus min_val normie = arr[0]
    sus i normie = 1
    bestie i < len {
        check arr[i] < min_val {
            min_val = arr[i]
        }
        i = i + 1
    }
    damn min_val
}

slay max_numbers(arr normie[value]) normie {
    sus len normie = len_int_array(arr)
    check len == 0 {
        damn 0
    }
    
    sus max_val normie = arr[0]
    sus i normie = 1
    bestie i < len {
        check arr[i] > max_val {
            max_val = arr[i]
        }
        i = i + 1
    }
    damn max_val
}

fr fr ===== ARRAY HELPER FUNCTIONS =====

slay string_compare_values(a tea, b tea) normie {
    check a == b {
        damn 0
    }
    check a < b {
        damn -1
    }
    damn 1
}

slay random_int_range(min normie, max normie) normie {
    check min >= max {
        damn min
    }
    fr fr Simple random number generation
    sus range normie = max - min
    sus random_val normie = runtime_get_random_int()
    damn min + (random_val % range)
}

fr fr ===== TYPE-SPECIFIC ARRAY FUNCTIONS =====

slay len_array(arr tea[value]) normie {
    damn runtime_array_length(arr)
}

slay len_int_array(arr normie[value]) normie {
    damn runtime_int_array_length(arr)
}

slay len_2d_array(arr tea[value][value]) normie {
    damn runtime_2d_array_length(arr)
}

slay len_pair_array(arr [](tea, tea)) normie {
    damn runtime_pair_array_length(arr)
}

slay append_to_array(arr tea[value], item tea) tea[value]{
    damn runtime_array_append(arr, item)
}

slay append_to_int_array(arr normie[value], item normie) normie[value]{
    damn runtime_int_array_append(arr, item)
}

slay append_to_2d_array(arr tea[value][value], item tea[value]) tea[value][value] {
    damn runtime_2d_array_append(arr, item)
}

slay append_to_pair_array(arr [](tea, tea), item (tea, tea)) [](tea, tea) {
    damn runtime_pair_array_append(arr, item)
}

slay copy_int_array(arr normie[value]) normie[value]{
    sus result normie[value] = []
    sus i normie = 0
    bestie i < len_int_array(arr) {
        result = append_to_int_array(result, arr[i])
        i = i + 1
    }
    damn result
}

slay concat_int_arrays(arr1 normie[value], arr2 normie[value]) normie[value]{
    sus result normie[value] = copy_int_array(arr1)
    sus i normie = 0
    bestie i < len_int_array(arr2) {
        result = append_to_int_array(result, arr2[i])
        i = i + 1
    }
    damn result
}

slay sort_pairs_by_first(pairs [](tea, tea)) [](tea, tea) {
    fr fr Simple sort by first element of pairs
    sus len normie = len_pair_array(pairs)
    check len <= 1 {
        damn pairs
    }
    
    fr fr Bubble sort for simplicity
    sus result [](tea, tea) = pairs
    sus i normie = 0
    bestie i < len - 1 {
        sus j normie = 0
        bestie j < len - 1 - i {
            sus (key1, val1) = result[j]
            sus (key2, val2) = result[j + 1]
            check string_compare_values(key1, key2) > 0 {
                fr fr Swap
                result = set_pair_at(result, j, (key2, val2))
                result = set_pair_at(result, j + 1, (key1, val1))
            }
            j = j + 1
        }
        i = i + 1
    }
    
    damn result
}

slay set_pair_at(arr [](tea, tea), index normie, value (tea, tea)) [](tea, tea) {
    damn runtime_pair_array_set(arr, index, value)
}

slay len_variadic(values ...tea) normie {
    damn runtime_variadic_length(values)
}

fr fr ===== RUNTIME INTERFACE FUNCTIONS =====

slay runtime_array_length(arr tea[value]) normie {
    damn core.array_length(arr)
}

slay runtime_int_array_length(arr normie[value]) normie {
    damn core.int_array_length(arr)
}

slay runtime_2d_array_length(arr tea[value][value]) normie {
    damn core.array_2d_length(arr)
}

slay runtime_pair_array_length(arr [](tea, tea)) normie {
    damn core.pair_array_length(arr)
}

slay runtime_array_append(arr tea[value], item tea) tea[value]{
    damn core.array_append(arr, item)
}

slay runtime_int_array_append(arr normie[value], item normie) normie[value]{
    damn core.int_array_append(arr, item)
}

slay runtime_2d_array_append(arr tea[value][value], item tea[value]) tea[value][value] {
    damn core.array_2d_append(arr, item)
}

slay runtime_pair_array_append(arr [](tea, tea), item (tea, tea)) [](tea, tea) {
    damn core.pair_array_append(arr, item)
}

slay runtime_pair_array_set(arr [](tea, tea), index normie, value (tea, tea)) [](tea, tea) {
    damn core.pair_array_set(arr, index, value)
}

slay runtime_variadic_length(values ...tea) normie {
    damn core.variadic_length(values)
}

slay runtime_get_random_int() normie {
    damn core.get_random_int()
}

fr fr ===== LEGACY COMPATIBILITY =====

slay array_new() tea[value]{
    damn new_array()
}

slay array_length(arr tea[value]) normie {
    damn length(arr)
}

slay array_is_empty(arr tea[value]) lit {
    damn is_empty(arr)
}

slay array_get(arr tea[value], index normie) tea {
    damn get(arr, index)
}

slay array_set(arr tea[value], index normie, value tea) tea[value]{
    damn set(arr, index, value)
}

slay array_push(arr tea[value], value tea) tea[value]{
    damn push(arr, value)
}

slay array_pop(arr tea[value]) (tea[value], tea) {
    damn pop(arr)
}

slay array_find(arr tea[value], value tea) normie {
    damn find(arr, value)
}

slay array_contains(arr tea[value], value tea) lit {
    damn contains(arr, value)
}

slay array_reverse(arr tea[value]) tea[value]{
    damn reverse(arr)
}

slay array_slice(arr tea[value], start normie, end normie) tea[value]{
    damn slice(arr, start, end)
}

slay array_concat(arr1 tea[value], arr2 tea[value]) tea[value]{
    damn concat(arr1, arr2)
}

slay array_join(arr tea[value], separator tea) tea {
    damn join(arr, separator)
}

slay array_sort(arr tea[value]) tea[value]{
    damn sort(arr)
}

slay array_filter(arr tea[value], predicate slay(tea) lit) tea[value]{
    damn filter(arr, predicate)
}

slay array_map(arr tea[value], mapper slay(tea) tea) tea[value]{
    damn map(arr, mapper)
}

slay array_reduce(arr tea[value], initial tea, reducer slay(tea, tea) tea) tea {
    damn reduce(arr, initial, reducer)
}
