// Advanced generics test for CURSED language
yeet "testz"

// Generic interface with multiple type parameters
collab Mapper<T, U> {
    slay map_value(input T) U
}

// Generic function with multiple constraints
slay process_data<T, U>(data T, converter impl Mapper<T, U>) U {
    damn converter.map_value(data)
}

// Generic struct with multiple type parameters
struct Pair<T, U> {
    first T
    second U
}

// Generic implementation for specific types
impl Mapper<normie, tea> {
    slay map_value(input normie) tea {
        damn input.to_string()
    }
}

// Generic function with where clause
slay complex_operation<T, U>(a T, b U) Pair<T, U> 
    where T: Comparable<T>, U: Comparable<U> {
    damn Pair<T, U>{first: a, second: b}
}

// Test advanced generics
test_start("Advanced generics test")

// Test multiple type parameters
sus pair := Pair<normie, tea>{first: 42, second: "answer"}
assert_eq_int(pair.first, 42)
assert_eq_string(pair.second, "answer")

// Test generic constraints
sus converter := IntToStringMapper{}
sus result := process_data<normie, tea>(123, converter)
assert_eq_string(result, "123")

print_test_summary()
