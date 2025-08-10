// Minimal test for generic parsing fix - no stdlib dependencies

// Test nested generics - the core issue
squad Vec<T> {
    data []T
    
    slay new() Vec<T> {
        damn Vec<T>{ data: []T{} }
    }
}

// Test the failing pattern: Vec<Vec<T>>
slay create_nested<T>() Vec<Vec<T>> {
    damn Vec<Vec<T>>.new()
}

// Test function that should have failed before fix
slay process_map<K, V>(input HashMap<K, Vec<V>>) HashMap<K, Vec<V>> {
    damn input
}

slay main() {
    // Test instantiation of nested generics
    sus nested Vec<Vec<drip>> = Vec<Vec<drip>>.new()
    sus result HashMap<tea, Vec<drip>> = HashMap<tea, Vec<drip>>{}
    
    // Function calls with nested generics
    sus output Vec<Vec<drip>> = create_nested<drip>()
    sus processed HashMap<tea, Vec<drip>> = process_map<tea, drip>(result)
}
