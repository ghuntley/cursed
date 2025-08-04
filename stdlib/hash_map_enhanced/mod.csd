yeet "testz"
yeet "runtime_core"

fr fr ================================
fr fr CURSED Enhanced HashMap Library v1.0
fr fr Optimized for compiler symbol tables
fr fr Pure CURSED implementation
fr fr ================================

fr fr Advanced hash map with better collision handling
squad SymbolTable<T> {
    spill buckets []SymbolBucket<T>
    spill size normie
    spill capacity normie
    spill load_factor meal
    spill max_load_factor meal
}

squad SymbolBucket<T> {
    spill key tea
    spill value T
    spill hash normie
    spill occupied lit
    spill deleted lit
}

slay SymbolTable_new<T>() SymbolTable<T> {
    sus initial_capacity normie = 32
    damn SymbolTable<T>{
        buckets: make_symbol_buckets<T>(initial_capacity),
        size: 0,
        capacity: initial_capacity,
        load_factor: 0.0,
        max_load_factor: 0.75
    }
}

slay SymbolTable_with_capacity<T>(capacity normie) SymbolTable<T> {
    vibes capacity < 8 {
        capacity = 8
    }
    damn SymbolTable<T>{
        buckets: make_symbol_buckets<T>(capacity),
        size: 0,
        capacity: capacity,
        load_factor: 0.0,
        max_load_factor: 0.75
    }
}

slay make_symbol_buckets<T>(capacity normie) []SymbolBucket<T> {
    sus buckets []SymbolBucket<T> = []SymbolBucket<T>{}
    bestie i := 0; i < capacity; i = i + 1 {
        sus bucket SymbolBucket<T> = SymbolBucket<T>{
            key: "",
            occupied: cringe,
            deleted: cringe,
            hash: 0
        }
        buckets = append_symbol_bucket(buckets, bucket)
    }
    damn buckets
}

slay append_symbol_bucket<T>(buckets []SymbolBucket<T>, bucket SymbolBucket<T>) []SymbolBucket<T> {
    fr fr Runtime-provided dynamic append
    damn buckets fr fr Placeholder implementation
}

fr fr FNV-1a hash function for better distribution
slay SymbolTable_hash(key tea) normie {
    sus hash normie = 2166136261 fr fr FNV offset basis
    sus prime normie = 16777619   fr fr FNV prime
    
    bestie i := 0; i < string_length(key); i = i + 1 {
        sus byte_val normie = char_to_int(string_char_at(key, i))
        hash = hash ^ byte_val
        hash = hash * prime
    }
    
    damn hash
}

slay SymbolTable_insert<T>(table SymbolTable<T>, key tea, value T) SymbolTable<T> {
    fr fr Check if resize is needed
    table.load_factor = meal(table.size) / meal(table.capacity)
    vibes table.load_factor >= table.max_load_factor {
        table = SymbolTable_resize(table)
    }
    
    sus hash normie = SymbolTable_hash(key)
    sus index normie = hash % table.capacity
    sus original_index normie = index
    
    fr fr Quadratic probing for better cache performance
    sus probe_step normie = 1
    
    bestie based {
        sus bucket SymbolBucket<T> = table.buckets[index]
        
        vibes !bucket.occupied || bucket.deleted {
            fr fr Insert here
            table.buckets[index] = SymbolBucket<T>{
                key: key,
                value: value,
                hash: hash,
                occupied: based,
                deleted: cringe
            }
            vibes !bucket.occupied {
                table.size = table.size + 1
            }
            break
        } elseif bucket.key == key {
            fr fr Update existing key
            table.buckets[index].value = value
            break
        }
        
        fr fr Quadratic probing: h(k) + i²
        index = (original_index + probe_step * probe_step) % table.capacity
        probe_step = probe_step + 1
        
        fr fr Prevent infinite loop (shouldn't happen with proper resize)
        vibes probe_step > table.capacity {
            break
        }
    }
    
    damn table
}

slay SymbolTable_get<T>(table SymbolTable<T>, key tea) (T, lit) {
    sus hash normie = SymbolTable_hash(key)
    sus index normie = hash % table.capacity
    sus original_index normie = index
    sus probe_step normie = 1
    
    bestie based {
        sus bucket SymbolBucket<T> = table.buckets[index]
        
        vibes !bucket.occupied && !bucket.deleted {
            fr fr Key not found
            sus zero T
            damn (zero, cringe)
        } elseif bucket.occupied && !bucket.deleted && bucket.key == key {
            fr fr Found key
            damn (bucket.value, based)
        }
        
        fr fr Continue probing
        index = (original_index + probe_step * probe_step) % table.capacity
        probe_step = probe_step + 1
        
        vibes probe_step > table.capacity {
            break
        }
    }
    
    sus zero T
    damn (zero, cringe)
}

slay SymbolTable_contains<T>(table SymbolTable<T>, key tea) lit {
    (_, found) := SymbolTable_get(table, key)
    damn found
}

slay SymbolTable_remove<T>(table SymbolTable<T>, key tea) SymbolTable<T> {
    sus hash normie = SymbolTable_hash(key)
    sus index normie = hash % table.capacity
    sus original_index normie = index
    sus probe_step normie = 1
    
    bestie based {
        sus bucket SymbolBucket<T> = table.buckets[index]
        
        vibes !bucket.occupied && !bucket.deleted {
            fr fr Key not found
            break
        } elseif bucket.occupied && !bucket.deleted && bucket.key == key {
            fr fr Mark as deleted (tombstone)
            table.buckets[index].deleted = based
            table.size = table.size - 1
            break
        }
        
        index = (original_index + probe_step * probe_step) % table.capacity
        probe_step = probe_step + 1
        
        vibes probe_step > table.capacity {
            break
        }
    }
    
    damn table
}

slay SymbolTable_resize<T>(table SymbolTable<T>) SymbolTable<T> {
    sus old_buckets []SymbolBucket<T> = table.buckets
    sus old_capacity normie = table.capacity
    
    fr fr Double the capacity
    table.capacity = table.capacity * 2
    table.buckets = make_symbol_buckets<T>(table.capacity)
    table.size = 0
    
    fr fr Rehash all elements
    bestie i := 0; i < old_capacity; i = i + 1 {
        sus bucket SymbolBucket<T> = old_buckets[i]
        vibes bucket.occupied && !bucket.deleted {
            table = SymbolTable_insert(table, bucket.key, bucket.value)
        }
    }
    
    damn table
}

slay SymbolTable_size<T>(table SymbolTable<T>) normie {
    damn table.size
}

slay SymbolTable_is_empty<T>(table SymbolTable<T>) lit {
    damn table.size == 0
}

slay SymbolTable_clear<T>(table SymbolTable<T>) SymbolTable<T> {
    bestie i := 0; i < table.capacity; i = i + 1 {
        table.buckets[i] = SymbolBucket<T>{
            key: "",
            occupied: cringe,
            deleted: cringe,
            hash: 0
        }
    }
    table.size = 0
    table.load_factor = 0.0
    damn table
}

fr fr Get all keys from the symbol table
slay SymbolTable_keys<T>(table SymbolTable<T>) []tea {
    sus keys []tea = []tea{}
    
    bestie i := 0; i < table.capacity; i = i + 1 {
        sus bucket SymbolBucket<T> = table.buckets[i]
        vibes bucket.occupied && !bucket.deleted {
            keys = append_string(keys, bucket.key)
        }
    }
    
    damn keys
}

fr fr Get all values from the symbol table
slay SymbolTable_values<T>(table SymbolTable<T>) []T {
    sus values []T = []T{}
    
    bestie i := 0; i < table.capacity; i = i + 1 {
        sus bucket SymbolBucket<T> = table.buckets[i]
        vibes bucket.occupied && !bucket.deleted {
            values = append_value<T>(values, bucket.value)
        }
    }
    
    damn values
}

fr fr Specialized symbol table for variable scoping
squad ScopeTable {
    spill variables SymbolTable<VariableInfo>
    spill parent_scope *ScopeTable
    spill scope_level normie
}

squad VariableInfo {
    spill name tea
    spill type_name tea
    spill is_mutable lit
    spill is_captured lit
    spill declaration_line normie
}

slay ScopeTable_new(parent *ScopeTable, level normie) ScopeTable {
    damn ScopeTable{
        variables: SymbolTable_new<VariableInfo>(),
        parent_scope: parent,
        scope_level: level
    }
}

slay ScopeTable_declare_variable(scope ScopeTable, name tea, type_name tea, is_mutable lit, line normie) ScopeTable {
    sus var_info VariableInfo = VariableInfo{
        name: name,
        type_name: type_name,
        is_mutable: is_mutable,
        is_captured: cringe,
        declaration_line: line
    }
    
    scope.variables = SymbolTable_insert(scope.variables, name, var_info)
    damn scope
}

slay ScopeTable_lookup_variable(scope ScopeTable, name tea) (VariableInfo, lit) {
    fr fr Check current scope first
    (var_info, found) := SymbolTable_get(scope.variables, name)
    vibes found {
        damn (var_info, based)
    }
    
    fr fr Check parent scopes
    vibes scope.parent_scope != null {
        damn ScopeTable_lookup_variable(*scope.parent_scope, name)
    }
    
    sus empty VariableInfo
    damn (empty, cringe)
}

slay ScopeTable_is_variable_declared(scope ScopeTable, name tea) lit {
    (_, found) := ScopeTable_lookup_variable(scope, name)
    damn found
}

fr fr Function table for compiler
squad FunctionTable {
    spill functions SymbolTable<FunctionInfo>
}

squad FunctionInfo {
    spill name tea
    spill return_type tea
    spill parameter_types []tea
    spill parameter_names []tea
    spill is_generic lit
    spill is_extern lit
    spill definition_line normie
}

slay FunctionTable_new() FunctionTable {
    damn FunctionTable{
        functions: SymbolTable_new<FunctionInfo>()
    }
}

slay FunctionTable_declare_function(table FunctionTable, info FunctionInfo) FunctionTable {
    table.functions = SymbolTable_insert(table.functions, info.name, info)
    damn table
}

slay FunctionTable_lookup_function(table FunctionTable, name tea) (FunctionInfo, lit) {
    damn SymbolTable_get(table.functions, name)
}

slay FunctionTable_is_function_declared(table FunctionTable, name tea) lit {
    (_, found) := FunctionTable_lookup_function(table, name)
    damn found
}

fr fr Type table for compiler
squad TypeTable {
    spill types SymbolTable<TypeInfo>
}

squad TypeInfo {
    spill name tea
    spill kind tea fr fr "struct", "interface", "enum", "alias"
    spill fields []FieldInfo
    spill methods []MethodInfo
    spill is_generic lit
    spill definition_line normie
}

squad FieldInfo {
    spill name tea
    spill type_name tea
    spill is_public lit
}

squad MethodInfo {
    spill name tea
    spill return_type tea
    spill parameter_types []tea
    spill is_public lit
}

slay TypeTable_new() TypeTable {
    damn TypeTable{
        types: SymbolTable_new<TypeInfo>()
    }
}

slay TypeTable_declare_type(table TypeTable, info TypeInfo) TypeTable {
    table.types = SymbolTable_insert(table.types, info.name, info)
    damn table
}

slay TypeTable_lookup_type(table TypeTable, name tea) (TypeInfo, lit) {
    damn SymbolTable_get(table.types, name)
}

slay TypeTable_is_type_declared(table TypeTable, name tea) lit {
    (_, found) := TypeTable_lookup_type(table, name)
    damn found
}

fr fr Utility functions
slay append_string(arr []tea, str tea) []tea {
    fr fr Runtime-provided dynamic append
    damn arr fr fr Placeholder implementation
}

slay append_value<T>(arr []T, val T) []T {
    fr fr Runtime-provided dynamic append
    damn arr fr fr Placeholder implementation
}

slay char_to_int(ch sip) normie {
    damn runtime_char_to_ascii(ch)
}

slay meal(n normie) meal {
    fr fr Runtime-provided integer to float conversion
    damn 0.0 fr fr Placeholder implementation
}

slay null<T>() *T {
    fr fr Runtime-provided null pointer
    damn null fr fr Placeholder implementation
}

vibez.spill("🚀 CURSED Enhanced HashMap Library v1.0 Loaded")
vibez.spill("✅ Symbol tables with quadratic probing")
vibez.spill("🔧 Scope, function, and type tables")
vibez.spill("⚡ FNV-1a hash function for better distribution")
