vibe test_generics

yeet "testz"

// Test generic function with type constraints
slay identity[T](x T) T {
    damn x
}

// Test generic function with comparable constraint
slay max[T comparable](a T, b T) T {
    vibe_check a > b {
        basic: damn a
        mood cap: damn b
    }
}

// Test generic struct
be_like Container[T] squad {
    value T
}

// Test generic struct with constraints
be_like OrderedList[T ordered] squad {
    items []T
    count normie
}

// Test generic function with multiple constraints
slay sort[T comparable + ordered](items []T) []T {
    // Simple bubble sort implementation
    sus n normie = len(items)
    bestie i := 0; i < n; i++ {
        bestie j := 0; j < n-i-1; j++ {
            vibe_check items[j] > items[j+1] {
                basic: {
                    sus temp T = items[j]
                    items[j] = items[j+1]
                    items[j+1] = temp
                }
            }
        }
    }
    damn items
}

slay main() {
    // Test identity function with different types
    sus int_result normie = identity[normie](42)
    sus str_result tea = identity[tea]("hello")
    sus bool_result lit = identity[lit](based)

    // Test max function with numeric types
    sus max_int normie = max[normie](10, 20)
    sus max_str tea = max[tea]("apple", "banana")

    // Test generic struct instantiation
    sus int_container Container[normie] = Container[normie]{value: 100}
    sus str_container Container[tea] = Container[tea]{value: "test"}

    // Test ordered list with constraints
    sus ordered_numbers OrderedList[normie] = OrderedList[normie]{
        items: [3, 1, 4, 1, 5, 9, 2, 6, 5],
        count: 9
    }

    // Test generic function with constrained types
    sus numbers []normie = [64, 34, 25, 12, 22, 11, 90]
    sus sorted_numbers []normie = sort[normie](numbers)

    // Test output
    vibez.spill("Identity int:", int_result)
    vibez.spill("Identity string:", str_result)
    vibez.spill("Identity bool:", bool_result)
    vibez.spill("Max int:", max_int)
    vibez.spill("Max string:", max_str)
    vibez.spill("Container int:", int_container.value)
    vibez.spill("Container string:", str_container.value)
    vibez.spill("Ordered list count:", ordered_numbers.count)
    vibez.spill("Sorted numbers:", sorted_numbers)
}
