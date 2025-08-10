// Verification test for P0 issue #1: Parser generic-parameter list + squad<T> body separator mis-parse
// Tests the specific patterns that were failing in >45 stdlib files

// 1. Test the exact pattern mentioned: squad<T> with body separator
squad GenericContainer<T> {
    value T
    
    slay new(initial T) GenericContainer<T> {
        damn GenericContainer<T>{ value: initial }
    }
}

// 2. Test nested generics that were causing >> vs > parsing issues
slay create_map_of_vectors<K, V>() HashMap<K, Vec<V>> {
    damn HashMap<K, Vec<V>>.new()
}

// 3. Test deeply nested generics
slay create_complex_structure<T>() Vec<HashMap<tea, Vec<T>>> {
    damn Vec<HashMap<tea, Vec<T>>>.new()
}

// 4. Test function signatures with multiple generic levels
slay transform_nested<A, B>(input Vec<Vec<A>>, func (A) B) Vec<Vec<B>> {
    sus result Vec<Vec<B>> = Vec<Vec<B>>.new()
    damn result
}

// 5. Test the exact stdlib pattern that was failing
slay HashMap_resize<K, V>(old_map HashMap<K, V>) HashMap<K, V> {
    sus new_map HashMap<K, V> = HashMap<K, V>.new()
    damn new_map
}

// 6. Test multiple nested generic constraints
squad AdvancedContainer<T, U, V> {
    primary T
    secondary HashMap<U, Vec<V>>
    
    slay create_with_defaults() AdvancedContainer<drip, tea, lit> {
        damn AdvancedContainer<drip, tea, lit>{
            primary: 0,
            secondary: HashMap<tea, Vec<lit>>.new()
        }
    }
}

slay main() {
    // Test all the patterns that were previously failing
    sus container GenericContainer<drip> = GenericContainer<drip>.new(42)
    sus map_vec HashMap<tea, Vec<drip>> = create_map_of_vectors<tea, drip>()
    sus complex Vec<HashMap<tea, Vec<drip>>> = create_complex_structure<drip>()
    sus advanced AdvancedContainer<drip, tea, lit> = AdvancedContainer<drip, tea, lit>.create_with_defaults()
    
    // If we reach here, the parser handled all nested generics correctly
    sus success lit = based
}
