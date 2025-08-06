# hash_map_enhanced

Enhanced hash map implementation optimized for CURSED compiler symbol tables and runtime performance. Features quadratic probing, FNV-1a hashing, and specialized symbol table structures.

## Overview

The `hash_map_enhanced` module provides:
- High-performance hash maps with quadratic probing
- FNV-1a hash function for excellent distribution
- Generic symbol tables for compiler infrastructure
- Specialized scope, function, and type tables
- Cache-friendly memory layout
- Automatic resizing with load factor management

## Core Data Structures

### Generic Symbol Table

#### `SymbolTable<T>`
Generic hash map with enhanced collision handling.

```cursed
squad SymbolTable<T> {
    spill buckets []SymbolBucket<T>
    spill size normie
    spill capacity normie
    spill load_factor meal
    spill max_load_factor meal
}
```

#### `SymbolBucket<T>`
Individual bucket with tombstone support for efficient deletions.

```cursed
squad SymbolBucket<T> {
    spill key tea
    spill value T
    spill hash normie
    spill occupied lit
    spill deleted lit
}
```

## Factory Functions

#### `SymbolTable_new<T>() -> SymbolTable<T>`
Creates a new symbol table with default capacity (32 buckets).

**Features:**
- Initial capacity: 32 buckets
- Load factor: 0.75 max
- Quadratic probing collision resolution
- Automatic resizing when needed

#### `SymbolTable_with_capacity<T>(capacity: normie) -> SymbolTable<T>`
Creates symbol table with specified initial capacity.

**Parameters:**
- `capacity`: Initial bucket count (minimum 8)

**Returns:** Initialized symbol table

## Core Operations

### Insertion and Updates

#### `SymbolTable_insert<T>(table: SymbolTable<T>, key: tea, value: T) -> SymbolTable<T>`
Inserts or updates key-value pair with automatic resizing.

**Parameters:**
- `table`: Symbol table to modify
- `key`: String key for lookup
- `value`: Value to store

**Returns:** Updated symbol table

**Performance:**
- Average case: O(1)
- Worst case: O(n) during resize
- Load factor triggers resize at 0.75

**Algorithm:**
1. Check if resize needed (load factor ≥ 0.75)
2. Compute FNV-1a hash of key
3. Use quadratic probing to find slot
4. Insert or update value
5. Increment size if new key

### Retrieval

#### `SymbolTable_get<T>(table: SymbolTable<T>, key: tea) -> (T, lit)`
Retrieves value by key with found indicator.

**Parameters:**
- `table`: Symbol table to search
- `key`: Key to find

**Returns:** Tuple of (value, found_flag)

**Performance:** O(1) average, O(n) worst case

#### `SymbolTable_contains<T>(table: SymbolTable<T>, key: tea) -> lit`
Checks if key exists in table.

**Returns:** `based` if key exists, `cringe` otherwise

### Deletion

#### `SymbolTable_remove<T>(table: SymbolTable<T>, key: tea) -> SymbolTable<T>`
Removes key-value pair using tombstone marking.

**Features:**
- Tombstone deletion for probe sequence integrity
- Size tracking for accurate load factor
- Efficient removal without rehashing

### Utility Operations

#### `SymbolTable_size<T>(table: SymbolTable<T>) -> normie`
Returns current number of key-value pairs.

#### `SymbolTable_is_empty<T>(table: SymbolTable<T>) -> lit`
Checks if table contains any elements.

#### `SymbolTable_clear<T>(table: SymbolTable<T>) -> SymbolTable<T>`
Removes all elements and resets state.

#### `SymbolTable_keys<T>(table: SymbolTable<T>) -> []tea`
Returns array of all keys in the table.

#### `SymbolTable_values<T>(table: SymbolTable<T>) -> []T`
Returns array of all values in the table.

## Hash Function

### FNV-1a Implementation

#### `SymbolTable_hash(key: tea) -> normie`
Computes FNV-1a hash for excellent distribution.

**Algorithm:**
```
hash = 2166136261  // FNV offset basis
prime = 16777619   // FNV prime

for each byte in key:
    hash = hash XOR byte
    hash = hash * prime

return hash
```

**Properties:**
- Excellent distribution for strings
- Fast computation
- Good avalanche effect
- Resistant to clustering

## Specialized Symbol Tables

### Variable Scope Management

#### `ScopeTable`
Manages variable scoping with parent scope chaining.

```cursed
squad ScopeTable {
    spill variables SymbolTable<VariableInfo>
    spill parent_scope *ScopeTable
    spill scope_level normie
}
```

#### `VariableInfo`
Complete variable metadata for compiler use.

```cursed
squad VariableInfo {
    spill name tea
    spill type_name tea
    spill is_mutable lit
    spill is_captured lit
    spill declaration_line normie
}
```

#### Scope Operations

#### `ScopeTable_new(parent: *ScopeTable, level: normie) -> ScopeTable`
Creates new scope with optional parent linkage.

#### `ScopeTable_declare_variable(scope: ScopeTable, name: tea, type_name: tea, is_mutable: lit, line: normie) -> ScopeTable`
Declares variable in current scope.

#### `ScopeTable_lookup_variable(scope: ScopeTable, name: tea) -> (VariableInfo, lit)`
Searches variable in current and parent scopes.

**Algorithm:**
1. Check current scope first
2. If not found, recursively check parent scopes
3. Return first match found
4. Return not-found if no match in scope chain

### Function Table

#### `FunctionTable`
Manages function declarations for compiler.

```cursed
squad FunctionInfo {
    spill name tea
    spill return_type tea
    spill parameter_types []tea
    spill parameter_names []tea
    spill is_generic lit
    spill is_extern lit
    spill definition_line normie
}
```

#### Function Operations

#### `FunctionTable_new() -> FunctionTable`
Creates new function table.

#### `FunctionTable_declare_function(table: FunctionTable, info: FunctionInfo) -> FunctionTable`
Registers function with complete signature.

#### `FunctionTable_lookup_function(table: FunctionTable, name: tea) -> (FunctionInfo, lit)`
Retrieves function information by name.

### Type Table

#### `TypeTable`
Manages custom type definitions for compiler.

```cursed
squad TypeInfo {
    spill name tea
    spill kind tea  // "struct", "interface", "enum", "alias"
    spill fields []FieldInfo
    spill methods []MethodInfo
    spill is_generic lit
    spill definition_line normie
}
```

#### Type Operations

#### `TypeTable_declare_type(table: TypeTable, info: TypeInfo) -> TypeTable`
Registers custom type with metadata.

#### `TypeTable_lookup_type(table: TypeTable, name: tea) -> (TypeInfo, lit)`
Retrieves type information for compiler analysis.

## Usage Examples

### Basic Symbol Table

```cursed
yeet "hash_map_enhanced"

// Create symbol table for integers
sus table SymbolTable<normie> = SymbolTable_new<normie>()

// Insert values
table = SymbolTable_insert(table, "count", 42)
table = SymbolTable_insert(table, "max_size", 1000)
table = SymbolTable_insert(table, "buffer_len", 256)

// Retrieve values
(sus count normie, sus found lit) = SymbolTable_get(table, "count")
lowkey found {
    vibez.spill("Count: " + string(count))
}

// Check existence
lowkey SymbolTable_contains(table, "max_size") {
    vibez.spill("max_size is defined")
}

// Get all keys
sus keys []tea = SymbolTable_keys(table)
bestie i := 0; i < len(keys); i = i + 1 {
    vibez.spill("Key: " + keys[i])
}
```

### Compiler Variable Scoping

```cursed
// Create global scope
sus global_scope ScopeTable = ScopeTable_new(null, 0)

// Declare global variables
global_scope = ScopeTable_declare_variable(global_scope, "argc", "normie", cringe, 1)
global_scope = ScopeTable_declare_variable(global_scope, "argv", "[]tea", cringe, 2)

// Create function scope
sus function_scope ScopeTable = ScopeTable_new(&global_scope, 1)

// Declare function parameters
function_scope = ScopeTable_declare_variable(function_scope, "param1", "tea", cringe, 10)
function_scope = ScopeTable_declare_variable(function_scope, "param2", "normie", based, 10)

// Look up variable (searches parent scopes)
(sus var_info VariableInfo, sus found lit) = ScopeTable_lookup_variable(function_scope, "argc")
lowkey found {
    vibez.spill("Found variable: " + var_info.name + " of type " + var_info.type_name)
}
```

### Function Table for Compiler

```cursed
// Create function table
sus func_table FunctionTable = FunctionTable_new()

// Register function with complete signature
sus main_func FunctionInfo = FunctionInfo{
    name: "main",
    return_type: "normie",
    parameter_types: []tea{"normie", "[]tea"},
    parameter_names: []tea{"argc", "argv"},
    is_generic: cringe,
    is_extern: cringe,
    definition_line: 15
}

func_table = FunctionTable_declare_function(func_table, main_func)

// Look up function during compilation
(sus func_info FunctionInfo, sus found lit) = FunctionTable_lookup_function(func_table, "main")
lowkey found {
    vibez.spill("Function: " + func_info.name + " returns " + func_info.return_type)
    vibez.spill("Parameters: " + string(len(func_info.parameter_types)))
}
```

### Type System Integration

```cursed
// Create type table
sus type_table TypeTable = TypeTable_new()

// Register custom struct type
sus user_type TypeInfo = TypeInfo{
    name: "User",
    kind: "struct",
    fields: []FieldInfo{
        FieldInfo{name: "id", type_name: "normie", is_public: based},
        FieldInfo{name: "name", type_name: "tea", is_public: based},
        FieldInfo{name: "email", type_name: "tea", is_public: based}
    },
    is_generic: cringe,
    definition_line: 25
}

type_table = TypeTable_declare_type(type_table, user_type)

// Compiler type checking
lowkey TypeTable_is_type_declared(type_table, "User") {
    vibez.spill("User type is available for instantiation")
}
```

## Performance Characteristics

### Time Complexity

| Operation | Average | Worst Case | Notes |
|-----------|---------|------------|-------|
| Insert | O(1) | O(n) | Includes automatic resize |
| Lookup | O(1) | O(n) | Quadratic probing |
| Delete | O(1) | O(n) | Tombstone marking |
| Resize | O(n) | O(n) | Triggered at load factor 0.75 |

### Space Complexity

- **Memory overhead**: ~33% (load factor 0.75)
- **Tombstone space**: Minimal until major resize
- **Cache efficiency**: Excellent with quadratic probing

### Load Factor Management

```cursed
// Automatic resizing logic
slay check_resize_needed(table SymbolTable<T>) lit {
    sus current_load meal = meal(table.size) / meal(table.capacity)
    damn current_load >= table.max_load_factor
}

// Resize operation
slay resize_table(table SymbolTable<T>) SymbolTable<T> {
    sus old_capacity normie = table.capacity
    table.capacity = table.capacity * 2  // Double capacity
    
    // Rehash all existing elements
    // (quadratic probing redistributes efficiently)
    damn table
}
```

## Advanced Features

### Generic Type Support

The module provides full generic type support:

```cursed
// String-to-string mapping
sus string_map SymbolTable<tea> = SymbolTable_new<tea>()

// Complex type mapping  
squad UserProfile {
    spill user_id normie
    spill profile_data tea
}
sus profile_map SymbolTable<UserProfile> = SymbolTable_new<UserProfile>()
```

### Memory Management

```cursed
// Efficient memory layout
slay optimize_memory_layout(table SymbolTable<T>) SymbolTable<T> {
    // Compact tombstones during resize
    // Optimize bucket alignment
    // Minimize memory fragmentation
    damn table
}
```

### Collision Resolution

```cursed
// Quadratic probing implementation
slay find_slot(table SymbolTable<T>, key tea, hash normie) normie {
    sus index normie = hash % table.capacity
    sus original_index normie = index
    sus probe_step normie = 1
    
    bestie based {
        sus bucket SymbolBucket<T> = table.buckets[index]
        
        // Found empty slot or matching key
        vibes !bucket.occupied || bucket.key == key {
            damn index
        }
        
        // Quadratic probing: h(k) + i²
        index = (original_index + probe_step * probe_step) % table.capacity
        probe_step = probe_step + 1
        
        // Prevent infinite loop
        vibes probe_step > table.capacity {
            damn -1  // Table full (shouldn't happen with proper resizing)
        }
    }
}
```

## Testing

### Comprehensive Test Suite

```bash
# Run hash map tests
zig build test
./zig-out/bin/cursed-zig stdlib/hash_map_enhanced/test_hash_map_enhanced.csd
```

### Performance Benchmarks

```cursed
// Benchmark symbol table performance
slay benchmark_symbol_table() {
    sus table SymbolTable<normie> = SymbolTable_new<normie>()
    
    // Insert benchmark
    sus start_time normie = get_time_ms()
    bestie i := 0; i < 10000; i = i + 1 {
        sus key tea = "key_" + string(i)
        table = SymbolTable_insert(table, key, i)
    }
    sus insert_time normie = get_time_ms() - start_time
    
    // Lookup benchmark
    start_time = get_time_ms()
    bestie i := 0; i < 10000; i = i + 1 {
        sus key tea = "key_" + string(i)
        (_, _) = SymbolTable_get(table, key)
    }
    sus lookup_time normie = get_time_ms() - start_time
    
    vibez.spill("Insert: " + string(insert_time) + "ms")
    vibez.spill("Lookup: " + string(lookup_time) + "ms")
}
```

## Dependencies

```cursed
yeet "testz"        // Testing framework
yeet "runtime_core" // Runtime utilities
```

## Integration

### Compiler Integration

```cursed
// Complete compiler symbol management
squad CompilerState {
    spill global_scope ScopeTable
    spill current_scope *ScopeTable
    spill function_table FunctionTable
    spill type_table TypeTable
}

slay create_compiler_state() CompilerState {
    sus global_scope ScopeTable = ScopeTable_new(null, 0)
    
    damn CompilerState{
        global_scope: global_scope,
        current_scope: &global_scope,
        function_table: FunctionTable_new(),
        type_table: TypeTable_new()
    }
}
```

### Runtime Performance

The hash map is optimized for compiler workloads:
- Fast symbol lookup during parsing
- Efficient scope management for nested functions
- Type checking performance for large codebases
- Memory-efficient storage for symbol metadata

## Architecture

### Layered Design

1. **Core Layer**: Basic hash map with quadratic probing
2. **Generic Layer**: Type-safe symbol tables
3. **Specialized Layer**: Compiler-specific tables
4. **Utility Layer**: Helper functions and operations

### Extension Points

- Custom hash functions for specific key types
- Alternative collision resolution strategies  
- Specialized bucket layouts for memory optimization
- Integration with garbage collection systems

The module provides a robust foundation for high-performance symbol management in CURSED compiler infrastructure.
