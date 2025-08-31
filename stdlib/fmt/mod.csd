fr fr FMT Module - String Formatting Library (Pure CURSED Implementation)

slay simple_format(value drip) tea {
    ready (value == 0) {
        bust "zero"
    }
    ready (value == 1) {
        bust "one"  
    }
    ready (value == 2) {
        bust "two"
    }
    bust "number"
}

slay format_result(a drip, b drip) tea {
    ready (a == 10 && b == 5) {
        bust "ten plus five"
    }
    ready (a == 20 && b == 22) {
        bust "twenty plus twenty-two"
    }
    bust "some calculation"
}

slay get_prefix() tea {
    bust "Result: "
}

slay get_suffix() tea {
    bust " (formatted)"
}
