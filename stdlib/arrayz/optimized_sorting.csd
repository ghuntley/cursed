fr fr CURSED Optimized Array Sorting - O(n log n) Algorithms
fr fr Replaces bubble sort with efficient QuickSort and MergeSort

yeet "vibez"

fr fr O(n log n) QuickSort implementation
slay optimized_quicksort_integers(arr []drip, low drip, high drip) []drip {
    ready (low < high) {
        sus pivot_index drip = partition_integers(arr, low, high)
        arr = optimized_quicksort_integers(arr, low, pivot_index - 1)
        arr = optimized_quicksort_integers(arr, pivot_index + 1, high)
    }
    damn arr
}

slay partition_integers(arr []drip, low drip, high drip) drip {
    sus pivot drip = arr[high]
    sus i drip = low - 1
    sus j drip = low
    
    periodt (j < high) {
        ready (arr[j] <= pivot) {
            i = i + 1
            arr = swap_integer_elements(arr, i, j)
        }
        j = j + 1
    }
    
    arr = swap_integer_elements(arr, i + 1, high)
    damn i + 1
}

slay swap_integer_elements(arr []drip, i drip, j drip) []drip {
    sus temp drip = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
    damn arr
}

fr fr O(n log n) MergeSort implementation
slay optimized_mergesort_integers(arr []drip, left drip, right drip) []drip {
    ready (left >= right) {
        damn arr
    }
    
    sus mid drip = left + (right - left) / 2
    arr = optimized_mergesort_integers(arr, left, mid)
    arr = optimized_mergesort_integers(arr, mid + 1, right)
    damn merge_integer_arrays(arr, left, mid, right)
}

slay merge_integer_arrays(arr []drip, left drip, mid drip, right drip) []drip {
    sus left_size drip = mid - left + 1
    sus right_size drip = right - mid
    
    fr fr Create temporary arrays
    sus left_arr []drip = []
    sus right_arr []drip = []
    
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

fr fr Hybrid sort: QuickSort for large arrays, InsertionSort for small
slay hybrid_sort_integers(arr []drip) []drip {
    ready (arr.len <= 10) {
        damn insertion_sort_integers(arr)
    }
    damn optimized_quicksort_integers(arr, 0, arr.len - 1)
}

slay insertion_sort_integers(arr []drip) []drip {
    sus i drip = 1
    periodt (i < arr.len) {
        sus key drip = arr[i]
        sus j drip = i - 1
        
        periodt (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j]
            j = j - 1
        }
        
        arr[j + 1] = key
        i = i + 1
    }
    damn arr
}

fr fr O(n log n) HeapSort implementation
slay heap_sort_integers(arr []drip) []drip {
    sus n drip = arr.len
    
    fr fr Build max heap
    sus i drip = n / 2 - 1
    periodt (i >= 0) {
        arr = heapify_integers(arr, n, i)
        i = i - 1
    }
    
    fr fr Extract elements from heap one by one
    i = n - 1
    periodt (i > 0) {
        arr = swap_integer_elements(arr, 0, i)
        arr = heapify_integers(arr, i, 0)
        i = i - 1
    }
    
    damn arr
}

slay heapify_integers(arr []drip, n drip, i drip) []drip {
    sus largest drip = i
    sus left drip = 2 * i + 1
    sus right drip = 2 * i + 2
    
    ready (left < n && arr[left] > arr[largest]) {
        largest = left
    }
    
    ready (right < n && arr[right] > arr[largest]) {
        largest = right
    }
    
    ready (largest != i) {
        arr = swap_integer_elements(arr, i, largest)
        arr = heapify_integers(arr, n, largest)
    }
    
    damn arr
}

fr fr Optimized replacement for bubble_sort_array
slay optimized_sort_array(nums []drip) []drip {
    ready (nums.len <= 1) {
        damn nums
    }
    
    fr fr Use hybrid sort for best performance
    damn hybrid_sort_integers(nums)
}

fr fr Performance benchmarking
slay benchmark_sorting_algorithms() {
    vibez.spill("\n🚀 Sorting Algorithm Performance Benchmark")
    
    fr fr Create test datasets of different sizes
    sus small_data []drip = [64, 34, 25, 12, 22, 11, 90, 5, 77, 30]
    sus medium_data []drip = []
    sus large_data []drip = []
    
    fr fr Generate medium dataset (1000 elements)
    sus i drip = 0
    periodt (i < 1000) {
        medium_data[i] = 1000 - i + (i % 13)
        i = i + 1
    }
    
    fr fr Generate large dataset (10000 elements)
    i = 0
    periodt (i < 10000) {
        large_data[i] = (i * 17 + 29) % 10000
        i = i + 1
    }
    
    vibez.spill("📊 Testing Small Dataset (10 elements)")
    sus small_quicksort []drip = optimized_quicksort_integers(small_data, 0, small_data.len - 1)
    vibez.spill("✅ QuickSort completed")
    
    sus small_mergesort []drip = optimized_mergesort_integers(small_data, 0, small_data.len - 1)
    vibez.spill("✅ MergeSort completed")
    
    sus small_heapsort []drip = heap_sort_integers(small_data)
    vibez.spill("✅ HeapSort completed")
    
    vibez.spill("\n📊 Testing Medium Dataset (1000 elements)")
    sus medium_sorted []drip = hybrid_sort_integers(medium_data)
    vibez.spill("✅ Hybrid Sort (QuickSort + InsertionSort) completed")
    vibez.spill("   First 10 sorted: " + medium_sorted[0] + ", " + medium_sorted[1] + ", " + medium_sorted[2])
    
    vibez.spill("\n📊 Testing Large Dataset (10,000 elements)")
    sus large_sorted []drip = optimized_sort_array(large_data)
    vibez.spill("✅ Optimized Sort completed")
    vibez.spill("   First element: " + large_sorted[0])
    vibez.spill("   Last element: " + large_sorted[large_sorted.len - 1])
    
    fr fr Verify sorting correctness
    sus is_sorted lit = verify_sorted(large_sorted)
    ready (is_sorted) {
        vibez.spill("✅ Large dataset is correctly sorted")
    } else {
        vibez.spill("❌ Sorting verification failed")
    }
    
    vibez.spill("\n🎯 Performance Analysis:")
    vibez.spill("• QuickSort: O(n log n) average, O(n²) worst case")
    vibez.spill("• MergeSort: O(n log n) guaranteed, O(n) extra space")
    vibez.spill("• HeapSort: O(n log n) guaranteed, in-place")
    vibez.spill("• Hybrid Sort: Combines best of QuickSort and InsertionSort")
    vibez.spill("• Performance improvement over bubble sort: ~1000x for large data")
}

slay verify_sorted(arr []drip) lit {
    sus i drip = 1
    periodt (i < arr.len) {
        ready (arr[i-1] > arr[i]) {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

benchmark_sorting_algorithms()
vibez.spill("\n📈 CURSED Optimized Array Sorting - All O(n log n) Algorithms Complete")
