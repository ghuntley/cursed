yeet "testz"

test_start("Array Data Structure Tests")

# Basic array declaration
sus numbers := [1, 2, 3, 4, 5]
sus names := ["Alice", "Bob", "Charlie"]

vibez.spill("First number: " + str(numbers[0]))
vibez.spill("Second name: " + names[1])

assert_eq_int(numbers[0], 1)
assert_eq_string(names[1], "Bob")

# Array length
sus num_count drip = len(numbers)
sus name_count drip = len(names)

vibez.spill("Number count: " + str(num_count))
vibez.spill("Name count: " + str(name_count))

assert_eq_int(num_count, 5)
assert_eq_int(name_count, 3)

# Array modification
numbers[2] = 99
names[0] = "Updated Alice"

vibez.spill("Modified number: " + str(numbers[2]))
vibez.spill("Modified name: " + names[0])

assert_eq_int(numbers[2], 99)
assert_eq_string(names[0], "Updated Alice")

# Array iteration
sus sum drip = 0
range num in numbers {
    sum = sum + num
}

vibez.spill("Sum: " + str(sum))
assert_eq_int(sum, 109) # 1 + 2 + 99 + 4 + 5

# Multi-dimensional arrays
sus matrix := [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

vibez.spill("Matrix[1][2]: " + str(matrix[1][2]))
assert_eq_int(matrix[1][2], 6)

# Array slicing
sus slice := numbers[1:4]
sus slice_sum drip = 0

range val in slice {
    slice_sum = slice_sum + val
}

vibez.spill("Slice sum: " + str(slice_sum))

# Array of structs
squad Point {
    spill x drip
    spill y drip
}

sus points := [
    Point{x: 0, y: 0},
    Point{x: 1, y: 1},
    Point{x: 2, y: 2}
]

vibez.spill("Point 1: (" + str(points[1].x) + ", " + str(points[1].y) + ")")
assert_eq_int(points[1].x, 1)
assert_eq_int(points[1].y, 1)

# Dynamic array operations
sus dynamic := []
dynamic = append(dynamic, 10)
dynamic = append(dynamic, 20)
dynamic = append(dynamic, 30)

vibez.spill("Dynamic length: " + str(len(dynamic)))
vibez.spill("Dynamic[1]: " + str(dynamic[1]))

assert_eq_int(len(dynamic), 3)
assert_eq_int(dynamic[1], 20)

print_test_summary()
