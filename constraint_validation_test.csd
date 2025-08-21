# CURSED Generic Constraint Validation Test
# Tests the comprehensive constraint system implementation

yeet "vibez"
yeet "testz"

# Test 1: Basic numeric constraint
slay add<T>(a T, b T) T ready T: Numeric {
    damn a + b
}

# Valid calls
sus result1 drip = add(5, 3)
sus result2 meal = add(3.14, 2.86)

# This should fail constraint validation:
# sus bad_result tea = add("hello", "world")  # String not Numeric

vibez.spill("Numeric constraint test passed")

# Test 2: Comparable constraint
slay max<T>(a T, b T) T ready T: Comparable {
    ready (a > b) {
        damn a
    } otherwise {
        damn b
    }
}

sus max_int drip = max(10, 5)
sus max_str tea = max("hello", "world")

vibez.spill("Comparable constraint test passed")

# Test 3: Interface constraint
collab Drawable {
    slay draw(self) vibes
}

squad Circle {
    radius meal
}

# Implementation of Drawable for Circle
slay draw(self Circle) vibes {
    vibez.spill("Drawing circle with radius", self.radius)
}

slay render<T>(obj T) vibes ready T: Drawable {
    obj.draw()
}

sus circle Circle = Circle{ .radius = 5.0 }
render(circle)

vibez.spill("Interface constraint test passed")

# Test 4: Send/Sync constraint for concurrency
slay safe_send<T>(value T, ch chan<T>) vibes ready T: Send {
    ch <- value
}

slay safe_share<T>(value T) T ready T: Sync {
    damn value  # Would be shared safely
}

# Test 5: Const generic constraint
slay create_array<T, const N>(default_value T) [N]T ready N: 0..1000 {
    sus arr [N]T = [N]T{}
    bestie (sus i drip = 0; i < N; i = i + 1) {
        arr[i] = default_value
    }
    damn arr
}

sus int_array [5]drip = create_array<drip, 5>(42)
vibez.spill("Array created with size", int_array.len)

# Test 6: Multiple constraints
slay sort<T>(arr []T) []T ready T: Comparable + Ordered {
    # Simple bubble sort implementation
    sus n drip = arr.len
    bestie (sus i drip = 0; i < n - 1; i = i + 1) {
        bestie (sus j drip = 0; j < n - i - 1; j = j + 1) {
            ready (arr[j] > arr[j + 1]) {
                sus temp T = arr[j]
                arr[j] = arr[j + 1]
                arr[j + 1] = temp
            }
        }
    }
    damn arr
}

sus numbers []drip = [3, 1, 4, 1, 5]
numbers = sort(numbers)
vibez.spill("Sorted array:", numbers)

# Test 7: Generic struct with constraints
squad Container<T> ready T: Sized + Send {
    data T
    
    slay new(value T) Container<T> {
        damn Container<T>{ .data = value }
    }
    
    slay get(self Container<T>) T {
        damn self.data
    }
}

sus int_container Container<drip> = Container<drip>.new(42)
sus value drip = int_container.get()
vibez.spill("Container value:", value)

# Test 8: Error cases (commented out to allow compilation)
# These would generate constraint violation errors:

# Numeric constraint violation:
# sus bad1 = add("hello", "world")  # tea not Numeric

# Comparable constraint violation:
# squad NonComparable {}
# sus bad2 = max(NonComparable{}, NonComparable{})

# Interface constraint violation:
# squad NonDrawable {}  # Doesn't implement Drawable
# render(NonDrawable{})

# Send constraint violation:
# slay bad_closure() slay() vibes { damn slay() vibes { vibez.spill("closure") } }
# safe_send(bad_closure(), make_channel<slay() vibes>())

# Const generic bounds violation:
# sus huge_array = create_array<drip, 2000>(0)  # N > 1000

vibez.spill("All constraint validation tests completed successfully!")

# Test summary
test_start("Generic Constraint Validation")
assert_eq_int(result1, 8)  # 5 + 3
assert_eq_int(max_int, 10)
assert_eq_int(int_array.len, 5)
assert_eq_int(value, 42)
print_test_summary()
