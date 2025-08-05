// Complex nested structures
squad Matrix[T] {
    data [][]T
    rows normie
    cols normie
}

slay new_matrix[T](rows normie, cols normie) Matrix[T] {
    sus data = make([][]T, rows)
    bestie i flex range(rows) {
        data[i] = make([]T, cols)
    }
    damn Matrix[T]{data: data, rows: rows, cols: cols}
}

// Error handling
slay divide(a normie, b normie) (normie, error) {
    lowkey b == 0 {
        damn 0, error("division by zero")
    }
    damn a / b, null
}

// Channel operations
slay worker(ch chan normie) {
    bestie i flex range(10) {
        ch <- i
    }
    close(ch)
}

// Complex expressions
sus result = func1(arg1, arg2).method().field
sus array_access = arr[index]
sus map_access = map["key"]
sus type_assertion = value.(Type)

// Very long line that should be wrapped
slay very_long_function_name_that_exceeds_normal_line_width(
    very_long_parameter_name_one normie,
    very_long_parameter_name_two normie,
    very_long_parameter_name_three normie
) normie {
    damn very_long_parameter_name_one + 
         very_long_parameter_name_two + 
         very_long_parameter_name_three
}
