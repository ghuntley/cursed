// CURSED SortaFresh Module - Modern Sorting and Caching Library
// Pure CURSED implementation with advanced sorting and caching mechanisms

// Core sorting functions

// Sort integers in ascending order
slay SortInts(arr []normie) []normie {
    sus len normie = array_length(arr)
    if len <= 1 {
        damn copy_int_array(arr)
    }
    
    sus result []normie = copy_int_array(arr)
    quicksort_ints(result, 0, len - 1)
    damn result
}

// Sort strings in ascending order  
slay SortStrings(arr []tea) []tea {
    sus len normie = array_length_str(arr)
    if len <= 1 {
        damn copy_string_array(arr)
    }
    
    sus result []tea = copy_string_array(arr)
    quicksort_strings(result, 0, len - 1)
    damn result
}

// Sort floats in ascending order
slay SortFloat64s(arr []meal) []meal {
    sus len normie = array_length_float(arr)
    if len <= 1 {
        damn copy_float_array(arr)
    }
    
    sus result []meal = copy_float_array(arr)
    quicksort_floats(result, 0, len - 1)
    damn result
}

// Check if integer array is sorted
slay IntsAreSorted(arr []normie) lit {
    sus len normie = array_length(arr)
    if len <= 1 {
        damn based
    }
    
    bestie i := 0; i < len - 1; i++ {
        if arr[i] > arr[i + 1] {
            damn cap
        }
    }
    damn based
}

// Check if string array is sorted
slay StringsAreSorted(arr []tea) lit {
    sus len normie = array_length_str(arr)
    if len <= 1 {
        damn based
    }
    
    bestie i := 0; i < len - 1; i++ {
        if string_compare(arr[i], arr[i + 1]) > 0 {
            damn cap
        }
    }
    damn based
}

// Check if float array is sorted
slay Float64sAreSorted(arr []meal) lit {
    sus len normie = array_length_float(arr)
    if len <= 1 {
        damn based
    }
    
    bestie i := 0; i < len - 1; i++ {
        if arr[i] > arr[i + 1] {
            damn cap
        }
    }
    damn based
}

// Binary search in sorted integer array
slay SearchInts(arr []normie, target normie) normie {
    sus left normie = 0
    sus right normie = array_length(arr) - 1
    
    while left <= right {
        sus mid normie = left + (right - left) / 2
        
        if arr[mid] == target {
            damn mid
        } else if arr[mid] < target {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }
    
    damn -1
}

// Binary search in sorted string array
slay SearchStrings(arr []tea, target tea) normie {
    sus left normie = 0
    sus right normie = array_length_str(arr) - 1
    
    while left <= right {
        sus mid normie = left + (right - left) / 2
        sus cmp normie = string_compare(arr[mid], target)
        
        if cmp == 0 {
            damn mid
        } else if cmp < 0 {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }
    
    damn -1
}

// Binary search in sorted float array
slay SearchFloat64s(arr []meal, target meal) normie {
    sus left normie = 0
    sus right normie = array_length_float(arr) - 1
    
    while left <= right {
        sus mid normie = left + (right - left) / 2
        
        if arr[mid] == target {
            damn mid
        } else if arr[mid] < target {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }
    
    damn -1
}

// Stable sort for integers
slay StableSort(arr []normie) []normie {
    sus len normie = array_length(arr)
    if len <= 1 {
        damn copy_int_array(arr)
    }
    
    sus result []normie = copy_int_array(arr)
    merge_sort_ints(result, 0, len - 1)
    damn result
}

// Reverse sort for integers
slay ReverseSort(arr []normie) []normie {
    sus sorted []normie = SortInts(arr)
    sus len normie = array_length(sorted)
    sus result []normie = make_int_array(len)
    
    bestie i := 0; i < len; i++ {
        result[i] = sorted[len - 1 - i]
    }
    damn result
}

// Shuffle array randomly
slay Shuffle(arr []normie) []normie {
    sus len normie = array_length(arr)
    sus result []normie = copy_int_array(arr)
    
    bestie i := len - 1; i > 0; i-- {
        sus j normie = random_int(i + 1)
        swap_ints(result, i, j)
    }
    damn result
}

// Top-K largest elements
slay TopK(arr []normie, k normie) []normie {
    sus len normie = array_length(arr)
    if k <= 0 || k > len {
        damn make_int_array(0)
    }
    
    sus sorted []normie = SortInts(arr)
    sus result []normie = make_int_array(k)
    
    bestie i := 0; i < k; i++ {
        result[i] = sorted[len - 1 - i]
    }
    damn result
}

// Bottom-K smallest elements
slay BottomK(arr []normie, k normie) []normie {
    sus len normie = array_length(arr)
    if k <= 0 || k > len {
        damn make_int_array(0)
    }
    
    sus sorted []normie = SortInts(arr)
    sus result []normie = make_int_array(k)
    
    bestie i := 0; i < k; i++ {
        result[i] = sorted[i]
    }
    damn result
}

// Find median element
slay Median(arr []normie) normie {
    sus len normie = array_length(arr)
    if len == 0 {
        damn 0
    }
    
    sus sorted []normie = SortInts(arr)
    damn sorted[len / 2]
}

// Quick select k-th smallest element
slay QuickSelect(arr []normie, k normie) normie {
    sus len normie = array_length(arr)
    if k < 0 || k >= len {
        damn -1
    }
    
    sus temp []normie = copy_int_array(arr)
    damn quickselect_impl(temp, 0, len - 1, k)
}

// Caching mechanism for sorted results (simplified)
be_like SortCache squad {
    cache_size normie
    current_size normie
}

// Create new sort cache
slay NewSortCache(size normie) SortCache {
    sus cache SortCache
    cache.cached_arrays = make_array_of_arrays(size)
    cache.cached_results = make_array_of_arrays(size)
    cache.cache_size = size
    cache.current_size = 0
    damn cache
}

// Cached sort - checks cache first, then sorts if needed
slay CachedSort(cache SortCache, arr []normie) []normie {
    // Check if array is in cache
    bestie i := 0; i < cache.current_size; i++ {
        if arrays_equal(cache.cached_arrays[i], arr) {
            damn copy_int_array(cache.cached_results[i])
        }
    }
    
    // Not in cache, sort and cache result
    sus result []normie = SortInts(arr)
    add_to_cache(cache, arr, result)
    damn result
}

// Add array and result to cache
slay add_to_cache(cache SortCache, arr []normie, result []normie) {
    if cache.current_size < cache.cache_size {
        cache.cached_arrays[cache.current_size] = copy_int_array(arr)
        cache.cached_results[cache.current_size] = copy_int_array(result)
        cache.current_size++
    } else {
        // Replace oldest entry (simple FIFO)
        bestie i := 0; i < cache.cache_size - 1; i++ {
            cache.cached_arrays[i] = cache.cached_arrays[i + 1]
            cache.cached_results[i] = cache.cached_results[i + 1]
        }
        cache.cached_arrays[cache.cache_size - 1] = copy_int_array(arr)
        cache.cached_results[cache.cache_size - 1] = copy_int_array(result)
    }
}

// Clear cache
slay ClearCache(cache SortCache) {
    cache.current_size = 0
}

// Get cache statistics
slay GetCacheStats(cache SortCache) (normie, normie) {
    damn cache.current_size, cache.cache_size
}

// Gen Z sorting features

// Vibe Sort - sorts by custom vibe score
slay VibeSort(arr []normie) []normie {
    // Simple implementation: sort by value with "vibe" bias
    sus len normie = array_length(arr)
    sus result []normie = copy_int_array(arr)
    
    bestie i := 0; i < len - 1; i++ {
        bestie j := i + 1; j < len; j++ {
            sus vibe_i normie = get_vibe_score(result[i])
            sus vibe_j normie = get_vibe_score(result[j])
            
            if vibe_i < vibe_j {
                swap_ints(result, i, j)
            }
        }
    }
    damn result
}

// No Cap Sort - absolute factual ordering
slay NoCapSort(arr []normie) []normie {
    // Just regular sort with attitude
    damn SortInts(arr)
}

// Bussin Sort - highlights excellent items first
slay BussinSort(arr []normie) []normie {
    // Sort by "bussin" score (higher values first)
    sus len normie = array_length(arr)
    sus result []normie = copy_int_array(arr)
    
    bestie i := 0; i < len - 1; i++ {
        bestie j := i + 1; j < len; j++ {
            sus bussin_i normie = get_bussin_score(result[i])
            sus bussin_j normie = get_bussin_score(result[j])
            
            if bussin_i < bussin_j {
                swap_ints(result, i, j)
            }
        }
    }
    damn result
}

// Slay Sort - high-performance sort
slay SlaySort(arr []normie) []normie {
    // Just optimized quicksort
    damn SortInts(arr)
}

// Yeet Sort - filter and sort
slay YeetSort(arr []normie, min_value normie) []normie {
    sus len normie = array_length(arr)
    sus filtered []normie = make_int_array(len)
    sus filtered_count normie = 0
    
    // Filter elements >= min_value
    bestie i := 0; i < len; i++ {
        if arr[i] >= min_value {
            filtered[filtered_count] = arr[i]
            filtered_count++
        }
    }
    
    // Create result array with only filtered elements
    sus result []normie = make_int_array(filtered_count)
    bestie i := 0; i < filtered_count; i++ {
        result[i] = filtered[i]
    }
    
    damn SortInts(result)
}

// Helper functions

// Quicksort implementation for integers
slay quicksort_ints(arr []normie, low normie, high normie) {
    if low < high {
        sus pivot normie = partition_ints(arr, low, high)
        quicksort_ints(arr, low, pivot - 1)
        quicksort_ints(arr, pivot + 1, high)
    }
}

// Partition function for integers
slay partition_ints(arr []normie, low normie, high normie) normie {
    sus pivot normie = arr[high]
    sus i normie = low - 1
    
    bestie j := low; j < high; j++ {
        if arr[j] <= pivot {
            i++
            swap_ints(arr, i, j)
        }
    }
    swap_ints(arr, i + 1, high)
    damn i + 1
}

// Swap function for integers
slay swap_ints(arr []normie, i normie, j normie) {
    sus temp normie = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
}

// Quicksort implementation for strings
slay quicksort_strings(arr []tea, low normie, high normie) {
    if low < high {
        sus pivot normie = partition_strings(arr, low, high)
        quicksort_strings(arr, low, pivot - 1)
        quicksort_strings(arr, pivot + 1, high)
    }
}

// Partition function for strings
slay partition_strings(arr []tea, low normie, high normie) normie {
    sus pivot tea = arr[high]
    sus i normie = low - 1
    
    bestie j := low; j < high; j++ {
        if string_compare(arr[j], pivot) <= 0 {
            i++
            swap_strings(arr, i, j)
        }
    }
    swap_strings(arr, i + 1, high)
    damn i + 1
}

// Swap function for strings
slay swap_strings(arr []tea, i normie, j normie) {
    sus temp tea = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
}

// Quicksort implementation for floats
slay quicksort_floats(arr []meal, low normie, high normie) {
    if low < high {
        sus pivot normie = partition_floats(arr, low, high)
        quicksort_floats(arr, low, pivot - 1)
        quicksort_floats(arr, pivot + 1, high)
    }
}

// Partition function for floats
slay partition_floats(arr []meal, low normie, high normie) normie {
    sus pivot meal = arr[high]
    sus i normie = low - 1
    
    bestie j := low; j < high; j++ {
        if arr[j] <= pivot {
            i++
            swap_floats(arr, i, j)
        }
    }
    swap_floats(arr, i + 1, high)
    damn i + 1
}

// Swap function for floats
slay swap_floats(arr []meal, i normie, j normie) {
    sus temp meal = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
}

// Merge sort for stable sorting
slay merge_sort_ints(arr []normie, left normie, right normie) {
    if left < right {
        sus mid normie = left + (right - left) / 2
        merge_sort_ints(arr, left, mid)
        merge_sort_ints(arr, mid + 1, right)
        merge_arrays(arr, left, mid, right)
    }
}

// Merge function for merge sort
slay merge_arrays(arr []normie, left normie, mid normie, right normie) {
    sus left_size normie = mid - left + 1
    sus right_size normie = right - mid
    
    sus left_arr []normie = make_int_array(left_size)
    sus right_arr []normie = make_int_array(right_size)
    
    bestie i := 0; i < left_size; i++ {
        left_arr[i] = arr[left + i]
    }
    
    bestie j := 0; j < right_size; j++ {
        right_arr[j] = arr[mid + 1 + j]
    }
    
    sus i normie = 0
    sus j normie = 0
    sus k normie = left
    
    while i < left_size && j < right_size {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i]
            i++
        } else {
            arr[k] = right_arr[j]
            j++
        }
        k++
    }
    
    while i < left_size {
        arr[k] = left_arr[i]
        i++
        k++
    }
    
    while j < right_size {
        arr[k] = right_arr[j]
        j++
        k++
    }
}

// Quick select implementation
slay quickselect_impl(arr []normie, low normie, high normie, k normie) normie {
    if low == high {
        damn arr[low]
    }
    
    sus pivot normie = partition_ints(arr, low, high)
    
    if k == pivot {
        damn arr[k]
    } else if k < pivot {
        damn quickselect_impl(arr, low, pivot - 1, k)
    } else {
        damn quickselect_impl(arr, pivot + 1, high, k)
    }
}

// Utility functions

// String comparison
slay string_compare(a tea, b tea) normie {
    if a < b {
        damn -1
    } else if a > b {
        damn 1
    } else {
        damn 0
    }
}

// Array copy functions
slay copy_int_array(arr []normie) []normie {
    sus len normie = array_length(arr)
    sus result []normie = make_int_array(len)
    
    bestie i := 0; i < len; i++ {
        result[i] = arr[i]
    }
    damn result
}

slay copy_string_array(arr []tea) []tea {
    sus len normie = array_length_str(arr)
    sus result []tea = make_string_array(len)
    
    bestie i := 0; i < len; i++ {
        result[i] = arr[i]
    }
    damn result
}

slay copy_float_array(arr []meal) []meal {
    sus len normie = array_length_float(arr)
    sus result []meal = make_float_array(len)
    
    bestie i := 0; i < len; i++ {
        result[i] = arr[i]
    }
    damn result
}

// Array comparison
slay arrays_equal(arr1 []normie, arr2 []normie) lit {
    sus len1 normie = array_length(arr1)
    sus len2 normie = array_length(arr2)
    
    if len1 != len2 {
        damn cap
    }
    
    bestie i := 0; i < len1; i++ {
        if arr1[i] != arr2[i] {
            damn cap
        }
    }
    damn based
}

// Vibe score function
slay get_vibe_score(value normie) normie {
    // Simple vibe scoring: lower values have better vibes
    damn 100 - value
}

// Bussin score function
slay get_bussin_score(value normie) normie {
    // Bussin score: higher values are more bussin
    damn value * 2
}

// Random number generator (simple)
slay random_int(max normie) normie {
    // Simple linear congruential generator
    // This would be replaced with better random in real implementation
    damn (max * 17 + 3) % max
}

// Runtime-provided functions (these would be implemented by the runtime)
slay array_length(arr []normie) normie {
    damn 0  // Implementation provided by runtime
}

slay array_length_str(arr []tea) normie {
    damn 0  // Implementation provided by runtime
}

slay array_length_float(arr []meal) normie {
    damn 0  // Implementation provided by runtime
}

slay make_int_array(size normie) []normie {
    damn []normie{}  // Implementation provided by runtime
}

slay make_string_array(size normie) []tea {
    damn []tea{}  // Implementation provided by runtime
}

slay make_float_array(size normie) []meal {
    damn []meal{}  // Implementation provided by runtime
}

slay make_array_of_arrays(size normie) [][]normie {
    damn [][]normie{}  // Implementation provided by runtime
}
