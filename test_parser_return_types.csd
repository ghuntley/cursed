fr fr Test complex function signatures and match statements to validate parser improvements

fr fr Test basic function with return type
slay simple_function(x normie) -> normie {
    damn x + 1
}

fr fr Test function with tuple return type
slay get_coords() -> (normie, normie) {
    damn (42, 24)
}

fr fr Test function with complex parameters
slay complex_function(a normie, b tea, c lit) -> tea {
    match a {
        1 -> "one",
        2 -> "two",
        default -> "unknown"
    }
}

fr fr Test variadic function signature
slay variadic_function(format tea, ...args normie) -> tea {
    damn format
}

fr fr Test function pointer parameter
slay callback_function(func fn(normie) -> lit) -> lit {
    damn func(42)
}

fr fr Test generic function with bounds
slay generic_function<T: Clone>(item T) -> T {
    damn item
}

fr fr Test match statement with complex patterns
slay pattern_matching_test(value normie) -> tea {
    match value {
        1 | 2 | 3 -> "small",
        10..20 -> "medium", 
        default -> "other"
    }
}

fr fr Test nested tuple access
slay tuple_access_test() -> normie {
    sus coords (normie, normie) = (10, 20)
    damn coords.0 + coords.1
}

vibez.spill("Parser return type test complete!")
