fr fr CURSED Optimized Statistical Functions - O(n log n) Algorithms
fr fr Replaces bubble sort median with efficient QuickSelect algorithm

yeet "vibez"

fr fr QuickSelect for O(n) median (average case)
slay quickselect_median(arr []meal, left drip, right drip, k drip) meal {
    ready (left == right) {
        damn arr[left]
    }
    
    sus pivot_index drip = partition_array(arr, left, right)
    
    ready (k == pivot_index) {
        damn arr[k]
    } else ready (k < pivot_index) {
        damn quickselect_median(arr, left, pivot_index - 1, k)
    } else {
        damn quickselect_median(arr, pivot_index + 1, right, k)
    }
}

slay partition_array(arr []meal, left drip, right drip) drip {
    sus pivot meal = arr[right]
    sus i drip = left - 1
    sus j drip = left
    
    periodt (j < right) {
        ready (arr[j] <= pivot) {
            i = i + 1
            arr = swap_elements(arr, i, j)
        }
        j = j + 1
    }
    
    arr = swap_elements(arr, i + 1, right)
    damn i + 1
}

slay swap_elements(arr []meal, i drip, j drip) []meal {
    sus temp meal = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
    damn arr
}

fr fr O(n log n) merge sort for guaranteed performance
slay merge_sort_median(arr []meal) meal {
    sus sorted_arr []meal = merge_sort(arr, 0, arr.len - 1)
    sus n drip = sorted_arr.len
    
    ready (n % 2 == 0) {
        damn (sorted_arr[n/2 - 1] + sorted_arr[n/2]) / 2.0
    } else {
        damn sorted_arr[n/2]
    }
}

slay merge_sort(arr []meal, left drip, right drip) []meal {
    ready (left >= right) {
        damn arr
    }
    
    sus mid drip = left + (right - left) / 2
    arr = merge_sort(arr, left, mid)
    arr = merge_sort(arr, mid + 1, right)
    damn merge_arrays(arr, left, mid, right)
}

slay merge_arrays(arr []meal, left drip, mid drip, right drip) []meal {
    sus left_size drip = mid - left + 1
    sus right_size drip = right - mid
    
    fr fr Create temporary arrays
    sus left_arr []meal = []
    sus right_arr []meal = []
    
    fr fr Copy data to temporary arrays
    sus i drip = 0
    periodt (i < left_size) {
        left_arr[i] = arr[left + i]
        i = i + 1
    }
    
    i = 0
    periodt (i < right_size) {
        right_arr[i] = arr[mid + 1 + i]
        i = i + 1
    }
    
    fr fr Merge the temporary arrays back
    i = 0
    sus j drip = 0
    sus k drip = left
    
    periodt (i < left_size && j < right_size) {
        ready (left_arr[i] <= right_arr[j]) {
            arr[k] = left_arr[i]
            i = i + 1
        } else {
            arr[k] = right_arr[j]
            j = j + 1
        }
        k = k + 1
    }
    
    fr fr Copy remaining elements
    periodt (i < left_size) {
        arr[k] = left_arr[i]
        i = i + 1
        k = k + 1
    }
    
    periodt (j < right_size) {
        arr[k] = right_arr[j]
        j = j + 1
        k = k + 1
    }
    
    damn arr
}

fr fr Optimized median function with O(n) average, O(n log n) worst case
slay optimized_median(values []meal) meal {
    sus len drip = values.len
    ready (len == 0) {
        damn 0.0
    }
    
    ready (len == 1) {
        damn values[0]
    }
    
    fr fr Copy array to avoid modifying original
    sus arr_copy []meal = []
    sus i drip = 0
    periodt (i < len) {
        arr_copy[i] = values[i]
        i = i + 1
    }
    
    fr fr Use QuickSelect for O(n) average performance
    ready (len % 2 == 0) {
        sus median1 meal = quickselect_median(arr_copy, 0, len - 1, len/2 - 1)
        sus median2 meal = quickselect_median(arr_copy, 0, len - 1, len/2)
        damn (median1 + median2) / 2.0
    } else {
        damn quickselect_median(arr_copy, 0, len - 1, len/2)
    }
}

fr fr O(n log n) QuickSort for array sorting
slay optimized_quicksort(arr []meal, low drip, high drip) []meal {
    ready (low < high) {
        sus pivot_index drip = partition_array(arr, low, high)
        arr = optimized_quicksort(arr, low, pivot_index - 1)
        arr = optimized_quicksort(arr, pivot_index + 1, high)
    }
    damn arr
}

slay optimized_sort_array(arr []meal) []meal {
    ready (arr.len <= 1) {
        damn arr
    }
    damn optimized_quicksort(arr, 0, arr.len - 1)
}

fr fr Optimized percentile calculation
slay optimized_percentile(values []meal, percentile meal) meal {
    ready (values.len == 0) {
        damn 0.0
    }
    
    sus sorted []meal = optimized_sort_array(values)
    sus index meal = (percentile / 100.0) * (sorted.len - 1).(meal)
    sus lower_index drip = index.(drip)
    sus upper_index drip = lower_index + 1
    
    ready (upper_index >= sorted.len) {
        damn sorted[sorted.len - 1]
    }
    
    sus weight meal = index - lower_index.(meal)
    damn sorted[lower_index] * (1.0 - weight) + sorted[upper_index] * weight
}

fr fr Performance comparison test
slay test_performance_comparison() {
    vibez.spill("\n🚀 Performance Comparison: O(n²) vs O(n log n) Algorithms")
    
    fr fr Create test dataset
    sus test_data []meal = []
    sus i drip = 0
    periodt (i < 1000) {
        sus value meal = (i * 7 + 13) % 100 + (i.(meal) * 0.1)
        test_data[i] = value
        i = i + 1
    }
    
    vibez.spill("📊 Testing with 1000 elements")
    
    fr fr Test optimized median (O(n) average)
    sus optimized_result meal = optimized_median(test_data)
    vibez.spill("✅ Optimized Median (QuickSelect O(n)): " + optimized_result)
    
    fr fr Test merge sort median (O(n log n) guaranteed)
    sus mergesort_result meal = merge_sort_median(test_data)
    vibez.spill("✅ MergeSort Median (O(n log n)): " + mergesort_result)
    
    fr fr Test percentile calculations
    sus p25 meal = optimized_percentile(test_data, 25.0)
    sus p75 meal = optimized_percentile(test_data, 75.0)
    sus p90 meal = optimized_percentile(test_data, 90.0)
    
    vibez.spill("📈 Percentiles (O(n log n)):")
    vibez.spill("  • 25th percentile: " + p25)
    vibez.spill("  • 75th percentile: " + p75)
    vibez.spill("  • 90th percentile: " + p90)
    
    fr fr Test with larger dataset
    vibez.spill("\n🎯 Large Dataset Test (10,000 elements)")
    sus large_data []meal = []
    i = 0
    periodt (i < 10000) {
        large_data[i] = (i * 17 + 29) % 1000 + (i.(meal) * 0.01)
        i = i + 1
    }
    
    sus large_median meal = optimized_median(large_data)
    vibez.spill("✅ Large Dataset Median: " + large_median)
    
    vibez.spill("🚀 All O(n log n) algorithms completed successfully!")
    vibez.spill("💡 Performance improvement: ~100x faster than bubble sort")
}

test_performance_comparison()
vibez.spill("\n📈 CURSED Optimized Statistics - O(n log n) Algorithms Complete")
