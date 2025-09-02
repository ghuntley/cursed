yeet "testz"

fr fr ================================
fr fr CURSED Runtime Core Library v1.0
fr fr Core utilities for compiler runtime
fr fr Pure CURSED implementation
fr fr ================================

fr fr Dynamic Array (ArrayList) for compiler runtime
squad RuntimeVec<T> {
    spill data T[value]
    spill len normie  
    spill cap normie
}

slay RuntimeVec_new<T>() RuntimeVec<T> {
    damn RuntimeVec<T>{
        data: T[value]{},
        len: 0,
        cap: 0
    }
}

slay RuntimeVec_with_capacity<T>(capacity normie) RuntimeVec<T> {
    damn RuntimeVec<T>{
        data: make(T[value], capacity),
        len: 0,
        cap: capacity
    }
}

slay RuntimeVec_push<T>(vec RuntimeVec<T>, item T) RuntimeVec<T> {
    vibes vec.len >= vec.cap {
        fr fr Grow capacity
        sus new_cap normie = vibes vec.cap == 0 { 4 } nah { vec.cap * 2 }
        sus new_data T[value] = make(T[value], new_cap)
        
        fr fr Copy existing elements
        bestie i := 0; i < vec.len; i = i + 1 {
            new_data[i] = vec.data[i]
        }
        
        vec.data = new_data
        vec.cap = new_cap
    }
    
    vec.data[vec.len] = item
    vec.len = vec.len + 1
    damn vec
}

slay RuntimeVec_get<T>(vec RuntimeVec<T>, index normie) T {
    vibes index >= 0 && index < vec.len {
        damn vec.data[index]
    }
    fr fr Return zero value for out of bounds
    sus zero T
    damn zero
}

slay RuntimeVec_len<T>(vec RuntimeVec<T>) normie {
    damn vec.len
}

slay RuntimeVec_is_empty<T>(vec RuntimeVec<T>) lit {
    damn vec.len == 0
}

fr fr Hash Map for symbol tables
squad RuntimeHashMap<K, V> {
    spill buckets RuntimeHashBucket[value]<K, V>
    spill size normie
    spill capacity normie
}

squad RuntimeHashBucket<K, V> {
    spill key K
    spill value V
    spill occupied lit
}

slay RuntimeHashMap_new<K, V>() RuntimeHashMap<K, V> {
    sus initial_cap normie = 16
    damn RuntimeHashMap<K, V>{
        buckets: make(RuntimeHashBucket[value]<K, V>, initial_cap),
        size: 0,
        capacity: initial_cap
    }
}

slay RuntimeHashMap_hash_string(key tea) normie {
    sus hash normie = 5381
    bestie i := 0; i < len(key); i = i + 1 {
        hash = ((hash << 5) + hash) + char_code_at(key, i)
    }
    damn hash
}

slay RuntimeHashMap_insert<K, V>(map RuntimeHashMap<K, V>, key K, value V) RuntimeHashMap<K, V> {
    sus hash normie = RuntimeHashMap_hash_string(string(key))
    sus index normie = hash % map.capacity
    
    fr fr Linear probing for collision resolution
    bestie map.buckets[index].occupied {
        vibes map.buckets[index].key == key {
            fr fr Update existing key
            map.buckets[index].value = value
            damn map
        }
        index = (index + 1) % map.capacity
    }
    
    fr fr Insert new key-value pair
    map.buckets[index] = RuntimeHashBucket<K, V>{
        key: key,
        value: value,
        occupied: based
    }
    map.size = map.size + 1
    damn map
}

slay RuntimeHashMap_get<K, V>(map RuntimeHashMap<K, V>, key K) (V, lit) {
    sus hash normie = RuntimeHashMap_hash_string(string(key))
    sus index normie = hash % map.capacity
    
    bestie map.buckets[index].occupied {
        vibes map.buckets[index].key == key {
            damn (map.buckets[index].value, based)
        }
        index = (index + 1) % map.capacity
    }
    
    sus zero V
    damn (zero, cringe)
}

slay RuntimeHashMap_contains<K, V>(map RuntimeHashMap<K, V>, key K) lit {
    (_, found) := RuntimeHashMap_get(map, key)
    damn found
}

slay RuntimeHashMap_size<K, V>(map RuntimeHashMap<K, V>) normie {
    damn map.size
}

fr fr String builder for efficient string construction
squad RuntimeStringBuilder {
    spill parts tea[value]
    spill total_len normie
}

slay RuntimeStringBuilder_new() RuntimeStringBuilder {
    damn RuntimeStringBuilder{
        parts: tea[value]{},
        total_len: 0
    }
}

slay RuntimeStringBuilder_append(sb RuntimeStringBuilder, str tea) RuntimeStringBuilder {
    sb.parts = append(sb.parts, str)
    sb.total_len = sb.total_len + len(str)
    damn sb
}

slay RuntimeStringBuilder_append_char(sb RuntimeStringBuilder, ch sip) RuntimeStringBuilder {
    sus char_str tea = string(ch)
    damn RuntimeStringBuilder_append(sb, char_str)
}

slay RuntimeStringBuilder_to_string(sb RuntimeStringBuilder) tea {
    vibes len(sb.parts) == 0 {
        damn ""
    }
    
    vibes len(sb.parts) == 1 {
        damn sb.parts[0]
    }
    
    fr fr Concatenate all parts
    sus result tea = ""
    bestie i := 0; i < len(sb.parts); i = i + 1 {
        result = result + sb.parts[i]
    }
    damn result
}

slay RuntimeStringBuilder_len(sb RuntimeStringBuilder) normie {
    damn sb.total_len
}

slay RuntimeStringBuilder_clear(sb RuntimeStringBuilder) RuntimeStringBuilder {
    damn RuntimeStringBuilder{
        parts: tea[value]{},
        total_len: 0
    }
}

fr fr Memory pool for efficient allocation
squad RuntimeMemoryPool {
    spill blocks byte[value][value]
    spill block_size normie
    spill current_block normie
    spill current_offset normie
}

slay RuntimeMemoryPool_new(block_size normie) RuntimeMemoryPool {
    damn RuntimeMemoryPool{
        blocks: byte[value][value]{},
        block_size: block_size,
        current_block: -1,
        current_offset: 0
    }
}

slay RuntimeMemoryPool_allocate(pool RuntimeMemoryPool, size normie) byte[value]{
    vibes pool.current_block == -1 || pool.current_offset + size > pool.block_size {
        fr fr Need new block
        sus new_block byte[value] = make(byte[value], pool.block_size)
        pool.blocks = append(pool.blocks, new_block)
        pool.current_block = len(pool.blocks) - 1
        pool.current_offset = 0
    }
    
    sus start normie = pool.current_offset
    pool.current_offset = pool.current_offset + size
    damn pool.blocks[pool.current_block][start:start+size]
}

slay RuntimeMemoryPool_reset(pool RuntimeMemoryPool) RuntimeMemoryPool {
    pool.current_block = -1
    pool.current_offset = 0
    damn pool
}

fr fr Error handling for runtime
squad RuntimeError {
    spill code normie
    spill message tea
    spill source_file tea
    spill source_line normie
}

slay RuntimeError_new(code normie, message tea) RuntimeError {
    damn RuntimeError{
        code: code,
        message: message,
        source_file: "",
        source_line: 0
    }
}

slay RuntimeError_with_source(code normie, message tea, file tea, line normie) RuntimeError {
    damn RuntimeError{
        code: code,
        message: message,
        source_file: file,
        source_line: line
    }
}

slay RuntimeError_to_string(err RuntimeError) tea {
    vibes err.source_file != "" {
        damn string_format_three("Error {}: {} at {}:{}", 
                                string(err.code), 
                                err.message, 
                                err.source_file, 
                                string(err.source_line))
    }
    damn string_format("Error {}: {}", string(err.code), err.message)
}

fr fr Stack data structure for runtime
squad RuntimeStack<T> {
    spill items T[value]
    spill top normie
}

slay RuntimeStack_new<T>() RuntimeStack<T> {
    damn RuntimeStack<T>{
        items: T[value]{},
        top: -1
    }
}

slay RuntimeStack_push<T>(stack RuntimeStack<T>, item T) RuntimeStack<T> {
    stack.items = append(stack.items, item)
    stack.top = stack.top + 1
    damn stack
}

slay RuntimeStack_pop<T>(stack RuntimeStack<T>) (T, lit) {
    vibes stack.top < 0 {
        sus zero T
        damn (zero, cringe)
    }
    
    sus item T = stack.items[stack.top]
    stack.items = stack.items[:stack.top]
    stack.top = stack.top - 1
    damn (item, based)
}

slay RuntimeStack_peek<T>(stack RuntimeStack<T>) (T, lit) {
    vibes stack.top < 0 {
        sus zero T
        damn (zero, cringe)
    }
    damn (stack.items[stack.top], based)
}

slay RuntimeStack_is_empty<T>(stack RuntimeStack<T>) lit {
    damn stack.top < 0
}

slay RuntimeStack_size<T>(stack RuntimeStack<T>) normie {
    damn stack.top + 1
}

fr fr Runtime utility functions
slay make<T>(size normie) T[value]{
    fr fr Runtime-provided dynamic allocation
    damn runtime_make_slice<T>(size)
}

slay append<T>(slice T[value], item T) T[value]{
    fr fr Runtime-provided dynamic append
    damn runtime_slice_append<T>(slice, item)
}

slay len(str tea) normie {
    fr fr Runtime-provided string length
    damn runtime_string_length(str)
}

slay string(value normie) tea {
    fr fr Runtime-provided number to string conversion
    damn runtime_int_to_string(value)
}

slay char_code_at(str tea, index normie) normie {
    fr fr Runtime-provided character code extraction
    damn runtime_char_to_ascii(runtime_string_char_at(str, index))
}

slay string_format(template tea, arg tea) tea {
    fr fr Runtime-provided string formatting  
    damn runtime_string_format(template, arg)
}

slay string_format_three(template tea, arg1 tea, arg2 tea, arg3 tea, arg4 tea) tea {
    fr fr Runtime-provided string formatting with multiple args
    damn runtime_string_format_multiple(template, arg1, arg2, arg3, arg4)
}

vibez.spill("🚀 CURSED Runtime Core Library v1.0 Loaded")
vibez.spill("✅ Dynamic arrays, hash maps, string builders")
vibez.spill("🔧 Memory pools and error handling")
vibez.spill("⚡ Optimized for compiler runtime usage")
