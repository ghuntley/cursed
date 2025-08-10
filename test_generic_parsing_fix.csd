// Test file for generic parameter parsing fix - P0 issue #1
// This tests the >> vs > ambiguity resolution

yeet "arrayz"
yeet "vibez"

// Test basic generic types
sus simple Vec<drip> = Vec<drip>.new()
sus simple2 HashMap<tea, drip> = HashMap<tea, drip>.new()

// Test nested generics - this should work now (was failing before)
sus nested Vec<Vec<drip>> = Vec<Vec<drip>>.new()
sus nested_deep Option<Vec<HashMap<tea, drip>>> = Option<Vec<HashMap<tea, drip>>>.new()

// Test squad<T> with generic parameters
squad Container<T> {
    items Vec<T>
    
    slay new() Container<T> {
        damn Container<T>{ items: Vec<T>.new() }
    }
    
    slay add(self *Container<T>, item T) {
        self.items.push(item)
    }
    
    slay get_nested() Vec<Vec<T>> {
        damn Vec<Vec<T>>.new()
    }
}

// Test generic functions with nested type parameters
slay create_nested_map<K, V>() HashMap<K, Vec<V>> {
    damn HashMap<K, Vec<V>>.new()
}

slay deeply_nested<T>() Vec<Vec<Vec<T>>> {
    damn Vec<Vec<Vec<T>>>.new()
}

// Test the specific pattern that was failing in stdlib
slay HashMap_resize<K, V>(old_map HashMap<K, V>) HashMap<K, V> {
    sus new_map HashMap<K, V> = HashMap<K, V>.new()
    damn new_map
}

// Function to test everything
slay main() {
    vibez.spill("Testing generic parameter parsing fix...")
    
    // Test basic generics
    sus container Container<drip> = Container<drip>.new()
    container.add(42)
    
    // Test nested generics
    sus nested_vec Vec<Vec<drip>> = Vec<Vec<drip>>.new()
    sus deep_option Option<Vec<HashMap<tea, drip>>> = Option<Vec<HashMap<tea, drip>>>.new()
    
    // Test generic function calls
    sus map_result HashMap<tea, Vec<drip>> = create_nested_map<tea, drip>()
    sus triple_nested Vec<Vec<Vec<tea>>> = deeply_nested<tea>()
    
    vibez.spill("All generic parsing tests passed!")
}
