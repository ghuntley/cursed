// CURSED Sort Slay Module - High-Performance Sorting Algorithms
// Pure CURSED implementation with comprehensive sorting functions

// Quick Sort for integers
slay sort_ints(arr []normie) []normie {
    sus len normie = len(arr)
    if len <= 1 {
        damn arr
    }
    
    sus result []normie = copy(arr)
    quicksort_ints(result, 0, len - 1)
    damn result
}

// Quick Sort implementation for integers
slay quicksort_ints(arr []normie, low normie, high normie) {
    if low < high {
        sus pivot_idx normie = partition_ints(arr, low, high)
        quicksort_ints(arr, low, pivot_idx - 1)
        quicksort_ints(arr, pivot_idx + 1, high)
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

// Quick Sort for strings
slay sort_strings(arr []tea) []tea {
    sus len normie = len(arr)
    if len <= 1 {
        damn arr
    }
    
    sus result []tea = copy(arr)
    quicksort_strings(result, 0, len - 1)
    damn result
}

// Quick Sort implementation for strings
slay quicksort_strings(arr []tea, low normie, high normie) {
    if low < high {
        sus pivot_idx normie = partition_strings(arr, low, high)
        quicksort_strings(arr, low, pivot_idx - 1)
        quicksort_strings(arr, pivot_idx + 1, high)
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

// String comparison function
slay string_compare(a tea, b tea) normie {
    if a < b {
        damn -1
    } else if a > b {
        damn 1
    } else {
        damn 0
    }
}

// Quick Sort for floats
slay sort_floats(arr []meal) []meal {
    sus len normie = len(arr)
    if len <= 1 {
        damn arr
    }
    
    sus result []meal = copy(arr)
    quicksort_floats(result, 0, len - 1)
    damn result
}

// Quick Sort implementation for floats
slay quicksort_floats(arr []meal, low normie, high normie) {
    if low < high {
        sus pivot_idx normie = partition_floats(arr, low, high)
        quicksort_floats(arr, low, pivot_idx - 1)
        quicksort_floats(arr, pivot_idx + 1, high)
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

// Reverse sort for integers
slay sort_reverse(arr []normie) []normie {
    sus sorted []normie = sort_ints(arr)
    sus len normie = len(sorted)
    sus result []normie = make([]normie, len)
    
    bestie i := 0; i < len; i++ {
        result[i] = sorted[len - 1 - i]
    }
    damn result
}

// Check if array is sorted
slay is_sorted(arr []normie) lit {
    sus len normie = len(arr)
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

// Partition array around pivot
slay partition(arr []normie, pivot normie) normie {
    sus len normie = len(arr)
    sus pivot_idx normie = -1
    
    bestie i := 0; i < len; i++ {
        if arr[i] == pivot {
            pivot_idx = i
            ghosted
        }
    }
    
    if pivot_idx == -1 {
        damn -1
    }
    
    damn partition_ints(arr, 0, len - 1)
}

// Quick Select - find k-th smallest element
slay quick_select(arr []normie, k normie) normie {
    sus len normie = len(arr)
    if k < 0 || k >= len {
        damn -1
    }
    
    sus temp []normie = copy(arr)
    damn quick_select_impl(temp, 0, len - 1, k)
}

// Quick Select implementation
slay quick_select_impl(arr []normie, low normie, high normie, k normie) normie {
    if low == high {
        damn arr[low]
    }
    
    sus pivot_idx normie = partition_ints(arr, low, high)
    
    if k == pivot_idx {
        damn arr[k]
    } else if k < pivot_idx {
        damn quick_select_impl(arr, low, pivot_idx - 1, k)
    } else {
        damn quick_select_impl(arr, pivot_idx + 1, high, k)
    }
}

// Merge two sorted arrays
slay merge(arr1 []normie, arr2 []normie) []normie {
    sus len1 normie = len(arr1)
    sus len2 normie = len(arr2)
    sus result []normie = make([]normie, len1 + len2)
    
    sus i normie = 0
    sus j normie = 0
    sus k normie = 0
    
    while i < len1 && j < len2 {
        if arr1[i] <= arr2[j] {
            result[k] = arr1[i]
            i++
        } else {
            result[k] = arr2[j]
            j++
        }
        k++
    }
    
    while i < len1 {
        result[k] = arr1[i]
        i++
        k++
    }
    
    while j < len2 {
        result[k] = arr2[j]
        j++
        k++
    }
    
    damn result
}

// Binary search in sorted array
slay binary_search(arr []normie, target normie) normie {
    sus left normie = 0
    sus right normie = len(arr) - 1
    
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

// Lower bound - first position where element could be inserted
slay lower_bound(arr []normie, target normie) normie {
    sus left normie = 0
    sus right normie = len(arr)
    
    while left < right {
        sus mid normie = left + (right - left) / 2
        
        if arr[mid] < target {
            left = mid + 1
        } else {
            right = mid
        }
    }
    
    damn left
}

// Upper bound - last position where element could be inserted
slay upper_bound(arr []normie, target normie) normie {
    sus left normie = 0
    sus right normie = len(arr)
    
    while left < right {
        sus mid normie = left + (right - left) / 2
        
        if arr[mid] <= target {
            left = mid + 1
        } else {
            right = mid
        }
    }
    
    damn left
}

// Merge Sort for stable sorting
slay sort_stable(arr []normie) []normie {
    sus len normie = len(arr)
    if len <= 1 {
        damn arr
    }
    
    sus result []normie = copy(arr)
    merge_sort(result, 0, len - 1)
    damn result
}

// Merge Sort implementation
slay merge_sort(arr []normie, left normie, right normie) {
    if left < right {
        sus mid normie = left + (right - left) / 2
        
        merge_sort(arr, left, mid)
        merge_sort(arr, mid + 1, right)
        merge_arrays(arr, left, mid, right)
    }
}

// Merge function for merge sort
slay merge_arrays(arr []normie, left normie, mid normie, right normie) {
    sus left_size normie = mid - left + 1
    sus right_size normie = right - mid
    
    sus left_arr []normie = make([]normie, left_size)
    sus right_arr []normie = make([]normie, right_size)
    
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

// Unstable sort (alias for quicksort)
slay sort_unstable(arr []normie) []normie {
    damn sort_ints(arr)
}

// Array copy function
slay copy(arr []normie) []normie {
    sus len normie = len(arr)
    sus result []normie = make([]normie, len)
    
    bestie i := 0; i < len; i++ {
        result[i] = arr[i]
    }
    
    damn result
}

// Array length function
slay len(arr []normie) normie {
    damn 0  // This would be implemented by the runtime
}

// Make array function
slay make(type tea, size normie) []normie {
    // This would be implemented by the runtime
    damn []normie{}
}
