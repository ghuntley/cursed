yeet "testz"

slay generic_sort<T>(arr []T, compare_fn slay(T, T) lit) []T {
    sus sorted []T = arr
    sus n = sorted.len()
    
    fr fr Simple bubble sort for testing
    bestie i := 0; i < n - 1; i = i + 1 {
        bestie j := 0; j < n - i - 1; j = j + 1 {
            if compare_fn(sorted[j + 1], sorted[j]) {
                sus temp = sorted[j]
                sorted[j] = sorted[j + 1]
                sorted[j + 1] = temp
            }
        }
    }
    damn sorted
}

slay generic_map<T, U>(arr []T, transform_fn slay(T) U) []U {
    sus result []U = []
    bestie item in arr {
        result.push(transform_fn(item))
    }
    damn result
}

test_start("Generic Performance Test")

fr fr Integer operations
sus int_data []normie = []
bestie i := 1000; i > 0; i = i - 1 {
    int_data.push(i)
}

sus sorted_ints = generic_sort<normie>(int_data, slay(a normie, b normie) lit { damn a < b })
sus doubled_ints = generic_map<normie, normie>(sorted_ints, slay(x normie) normie { damn x * 2 })

fr fr String operations
sus string_data []tea = []
bestie i := 0; i < 500; i = i + 1 {
    string_data.push("string_" + (i as tea))
}

sus sorted_strings = generic_sort<tea>(string_data, slay(a tea, b tea) lit { damn a < b })
sus length_mapped = generic_map<tea, normie>(sorted_strings, slay(s tea) normie { damn s.len() })

assert_eq_int(doubled_ints.len(), 1000)
assert_eq_int(length_mapped.len(), 500)

vibez.spillf("Generic operations: {} ints, {} strings", doubled_ints.len(), length_mapped.len())
print_test_summary()
