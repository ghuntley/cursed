# Simple Collections (collections_simple)

The `collections_simple` module provides basic data structures and collection operations for CURSED programs.

## Purpose

This module implements essential collection types including arrays, lists, sets, and maps with simple and efficient operations.

## Main Functions

### Array Operations
- `collections_simple.array_new(size)` - Create new array with size
- `collections_simple.array_push(arr, item)` - Add item to end
- `collections_simple.array_pop(arr)` - Remove and return last item
- `collections_simple.array_get(arr, index)` - Get item at index
- `collections_simple.array_set(arr, index, item)` - Set item at index
- `collections_simple.array_len(arr)` - Get array length
- `collections_simple.array_slice(arr, start, end)` - Get subarray

### List Operations  
- `collections_simple.list_new()` - Create new list
- `collections_simple.list_append(list, item)` - Add to end
- `collections_simple.list_prepend(list, item)` - Add to beginning
- `collections_simple.list_insert(list, index, item)` - Insert at index
- `collections_simple.list_remove(list, index)` - Remove at index
- `collections_simple.list_find(list, item)` - Find item index

### Set Operations
- `collections_simple.set_new()` - Create new set
- `collections_simple.set_add(set, item)` - Add item to set
- `collections_simple.set_remove(set, item)` - Remove item from set
- `collections_simple.set_contains(set, item)` - Check if item exists
- `collections_simple.set_union(set1, set2)` - Union of two sets
- `collections_simple.set_intersection(set1, set2)` - Intersection

### Map Operations
- `collections_simple.map_new()` - Create new map
- `collections_simple.map_put(map, key, value)` - Set key-value pair
- `collections_simple.map_get(map, key)` - Get value by key
- `collections_simple.map_remove(map, key)` - Remove key
- `collections_simple.map_keys(map)` - Get all keys
- `collections_simple.map_values(map)` - Get all values

## Usage Examples

### Array Operations

```cursed
yeet "collections_simple"

fr fr Create and manipulate arrays
sus numbers = collections_simple.array_new(5)
collections_simple.array_push(numbers, 10)
collections_simple.array_push(numbers, 20)
collections_simple.array_push(numbers, 30)

vibez.spillf("Array length: {}", collections_simple.array_len(numbers))
vibez.spillf("First item: {}", collections_simple.array_get(numbers, 0))

fr fr Modify array
collections_simple.array_set(numbers, 1, 25)
vibez.spillf("Modified item: {}", collections_simple.array_get(numbers, 1))

fr fr Get slice
sus subset = collections_simple.array_slice(numbers, 0, 2)
vibez.spillf("Subset length: {}", collections_simple.array_len(subset))
```

### List Operations

```cursed
yeet "collections_simple"

fr fr Create dynamic list
sus tasks = collections_simple.list_new()
collections_simple.list_append(tasks, "Buy groceries")
collections_simple.list_append(tasks, "Write code")
collections_simple.list_prepend(tasks, "Wake up")

vibez.spill("Task list:")
bestie i := 0; i < collections_simple.list_len(tasks); i = i + 1 {
    sus task = collections_simple.list_get(tasks, i)
    vibez.spillf("  {}. {}", i + 1, task)
}

fr fr Insert and remove
collections_simple.list_insert(tasks, 1, "Drink coffee")
collections_simple.list_remove(tasks, 3)  # Remove "Write code"
```

### Set Operations

```cursed
yeet "collections_simple"

fr fr Create sets
sus fruits = collections_simple.set_new()
collections_simple.set_add(fruits, "apple")
collections_simple.set_add(fruits, "banana")
collections_simple.set_add(fruits, "orange")
collections_simple.set_add(fruits, "apple")  # Duplicate, won't be added

sus citrus = collections_simple.set_new()
collections_simple.set_add(citrus, "orange")
collections_simple.set_add(citrus, "lemon")
collections_simple.set_add(citrus, "lime")

fr fr Set operations
vibez.spillf("Fruits count: {}", collections_simple.set_size(fruits))
vibez.spillf("Has banana: {}", collections_simple.set_contains(fruits, "banana"))

sus all_fruits = collections_simple.set_union(fruits, citrus)
sus common = collections_simple.set_intersection(fruits, citrus)

vibez.spillf("All fruits: {}", collections_simple.set_size(all_fruits))
vibez.spillf("Common: {}", collections_simple.set_size(common))
```

### Map Operations

```cursed
yeet "collections_simple"

fr fr Create and populate map
sus scores = collections_simple.map_new()
collections_simple.map_put(scores, "Alice", 95)
collections_simple.map_put(scores, "Bob", 87)
collections_simple.map_put(scores, "Charlie", 92)

fr fr Access values
sus alice_score = collections_simple.map_get(scores, "Alice")
if alice_score.is_some() {
    vibez.spillf("Alice's score: {}", alice_score.unwrap())
}

fr fr Iterate over keys and values
sus all_keys = collections_simple.map_keys(scores)
bestie key in all_keys {
    sus value = collections_simple.map_get(scores, key).unwrap()
    vibez.spillf("{}: {}", key, value)
}

fr fr Update and remove
collections_simple.map_put(scores, "Bob", 90)  # Update Bob's score
collections_simple.map_remove(scores, "Charlie")
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "collections_simple"
sus list = collections_simple.list_new()
collections_simple.list_append(list, "Hello")
vibez.spillf("List size: {}", collections_simple.list_len(list))' > collections_test.csd

./cursed-unified collections_test.csd
```

### Compilation Mode
```bash
./cursed-unified --compile collections_test.csd
./collections_test
```

## Advanced Examples

### Shopping Cart Implementation

```cursed
yeet "collections_simple"

squad ShoppingCart {
    spill items map[tea]normie
    spill total meal
}

slay cart_new() ShoppingCart {
    damn ShoppingCart{
        items: collections_simple.map_new(),
        total: 0.0
    }
}

slay cart_add_item(cart ShoppingCart, item tea, price meal) {
    collections_simple.map_put(cart.items, item, price)
    cart.total = cart.total + price
}

slay cart_remove_item(cart ShoppingCart, item tea) {
    sus price = collections_simple.map_get(cart.items, item)
    if price.is_some() {
        cart.total = cart.total - price.unwrap()
        collections_simple.map_remove(cart.items, item)
    }
}

slay cart_print(cart ShoppingCart) {
    vibez.spill("Shopping Cart:")
    sus items = collections_simple.map_keys(cart.items)
    bestie item in items {
        sus price = collections_simple.map_get(cart.items, item).unwrap()
        vibez.spillf("  {} - ${}", item, price)
    }
    vibez.spillf("Total: ${}", cart.total)
}

sus cart = cart_new()
cart_add_item(cart, "Apples", 3.50)
cart_add_item(cart, "Bread", 2.25)
cart_add_item(cart, "Milk", 4.00)
cart_print(cart)
```

### Word Frequency Counter

```cursed
yeet "collections_simple"
yeet "string_simple"

slay count_words(text tea) map[tea]normie {
    sus word_counts = collections_simple.map_new()
    sus words = string_simple.split(text, " ")
    
    bestie word in words {
        sus clean_word = string_simple.trim(string_simple.to_lower(word))
        if !string_simple.empty(clean_word) {
            sus current = collections_simple.map_get(word_counts, clean_word)
            if current.is_some() {
                collections_simple.map_put(word_counts, clean_word, current.unwrap() + 1)
            } else {
                collections_simple.map_put(word_counts, clean_word, 1)
            }
        }
    }
    
    damn word_counts
}

sus text = "the quick brown fox jumps over the lazy dog the fox is quick"
sus counts = count_words(text)

vibez.spill("Word frequencies:")
sus words = collections_simple.map_keys(counts)
bestie word in words {
    sus count = collections_simple.map_get(counts, word).unwrap()
    vibez.spillf("  '{}': {}", word, count)
}
```

### Simple Graph Implementation

```cursed
yeet "collections_simple"

squad Graph {
    spill adjacency_list map[tea][]tea
}

slay graph_new() Graph {
    damn Graph{adjacency_list: collections_simple.map_new()}
}

slay graph_add_edge(graph Graph, from tea, to tea) {
    sus neighbors = collections_simple.map_get(graph.adjacency_list, from)
    if neighbors.is_none() {
        sus new_list = collections_simple.list_new()
        collections_simple.list_append(new_list, to)
        collections_simple.map_put(graph.adjacency_list, from, new_list)
    } else {
        collections_simple.list_append(neighbors.unwrap(), to)
    }
}

slay graph_get_neighbors(graph Graph, node tea) []tea {
    sus neighbors = collections_simple.map_get(graph.adjacency_list, node)
    if neighbors.is_some() {
        damn neighbors.unwrap()
    } else {
        damn collections_simple.list_new()
    }
}

sus graph = graph_new()
graph_add_edge(graph, "A", "B")
graph_add_edge(graph, "A", "C")
graph_add_edge(graph, "B", "D")
graph_add_edge(graph, "C", "D")

sus a_neighbors = graph_get_neighbors(graph, "A")
vibez.spillf("A connects to {} nodes", collections_simple.list_len(a_neighbors))
```

## Implementation Notes

- Memory-efficient implementations
- Thread-safe for concurrent access
- Generic type support where applicable
- Optimized for common operations
- Pure CURSED implementation

## Dependencies

- `memory` - For collection memory management
- Core runtime types
- No external dependencies

## Performance Considerations

- O(1) array access and modification
- O(1) average map operations
- O(n) list operations (linked list implementation)
- O(1) set membership testing (hash-based)
- Efficient memory usage for large collections

## Best Practices

1. **Choose appropriate collection** for your use case
2. **Pre-size collections** when possible for performance
3. **Use sets for membership testing** instead of arrays
4. **Use maps for key-based lookups** instead of lists
5. **Consider memory usage** for large datasets
6. **Handle empty collections** gracefully
7. **Use iterators** for collection traversal when available
