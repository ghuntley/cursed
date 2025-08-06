yeet "testz"

test_start("Loop Control Flow Tests")

# Basic while loop (bestie)
sus count drip = 0
sus sum drip = 0

bestie (count < 5) {
    sum = sum + count
    count = count + 1
}

vibez.spill("Sum 0-4: " + str(sum))
assert_eq_int(sum, 10)

# For loop with range
sus total drip = 0
sus i drip

range i, 0, 10 {
    total = total + i
}

vibez.spill("Sum 0-9: " + str(total))
assert_eq_int(total, 45)

# For loop with array iteration
sus numbers := [1, 2, 3, 4, 5]
sus array_sum drip = 0

range num in numbers {
    array_sum = array_sum + num
}

vibez.spill("Array sum: " + str(array_sum))
assert_eq_int(array_sum, 15)

# Nested loops
sus matrix_sum drip = 0
sus row drip
sus col drip

range row, 0, 3 {
    range col, 0, 3 {
        matrix_sum = matrix_sum + (row * col)
    }
}

vibez.spill("Matrix sum: " + str(matrix_sum))

# Loop with break
sus break_sum drip = 0
sus j drip = 0

bestie (based) {
    ready (j >= 10) {
        break
    }
    break_sum = break_sum + j
    j = j + 1
}

vibez.spill("Break sum: " + str(break_sum))
assert_eq_int(break_sum, 45)

# Loop with continue
sus odd_sum drip = 0
sus k drip

range k, 0, 10 {
    ready (k % 2 == 0) {
        continue
    }
    odd_sum = odd_sum + k
}

vibez.spill("Odd sum: " + str(odd_sum))
assert_eq_int(odd_sum, 25)

print_test_summary()
