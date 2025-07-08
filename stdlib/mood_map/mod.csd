yeet "testz"

fr fr MoodMap (maps package) - Functions for working with maps in various "moods"

fr fr Core map operation functions

fr fr Clone creates a copy of the map
slay Clone(m map[tea]normie) map[tea]normie {
    sus result := make(map[tea]normie)
    for key, value := range m {
        result[key] = value
    }
    damn result
}

fr fr Copy copies all key/value pairs from src to dst  
slay Copy(dst map[tea]normie, src map[tea]normie) {
    for key, value := range src {
        dst[key] = value
    }
}

fr fr Clear removes all entries from the map
slay Clear(m map[tea]normie) {
    for key := range m {
        delete(m, key)
    }
}

fr fr DeleteFunc deletes entries where f returns true
slay DeleteFunc(m map[tea]normie, f slay(tea, normie) lit) {
    sus toDelete := make([]tea, 0)
    for key, value := range m {
        if f(key, value) {
            toDelete = append(toDelete, key)
        }
    }
    for i := 0; i < len(toDelete); i++ {
        delete(m, toDelete[i])
    }
}

fr fr Enhanced Map Operations

fr fr Vibe creates a new map from entries that match the given condition
slay Vibe(m map[tea]normie, f slay(tea, normie) lit) map[tea]normie {
    sus result := make(map[tea]normie)
    for key, value := range m {
        if f(key, value) {
            result[key] = value
        }
    }
    damn result
}

fr fr Moody transforms a map by applying a function to each value
slay Moody(m map[tea]normie, f slay(tea, normie) normie) map[tea]normie {
    sus result := make(map[tea]normie)
    for key, value := range m {
        result[key] = f(key, value)
    }
    damn result
}

fr fr MoodSwitch returns two maps - one for keys passing condition, one for others
slay MoodSwitch(m map[tea]normie, f slay(tea, normie) lit) (map[tea]normie, map[tea]normie) {
    sus passing := make(map[tea]normie)
    sus failing := make(map[tea]normie)
    
    for key, value := range m {
        if f(key, value) {
            passing[key] = value
        } else {
            failing[key] = value
        }
    }
    
    damn passing, failing
}

fr fr MoodyMerge merges maps with a function to resolve conflicts
slay MoodyMerge(maps []map[tea]normie, resolver slay(tea, normie, normie) normie) map[tea]normie {
    sus result := make(map[tea]normie)
    
    for i := 0; i < len(maps); i++ {
        sus currentMap := maps[i]
        for key, value := range currentMap {
            sus existing, exists := result[key]
            if exists {
                result[key] = resolver(key, existing, value)
            } else {
                result[key] = value
            }
        }
    }
    
    damn result
}

fr fr MoodCheck tests if all entries satisfy a condition
slay MoodCheck(m map[tea]normie, f slay(tea, normie) lit) lit {
    for key, value := range m {
        if !f(key, value) {
            damn cap
        }
    }
    damn based
}

fr fr FeelingSome tests if at least one entry satisfies a condition
slay FeelingSome(m map[tea]normie, f slay(tea, normie) lit) lit {
    for key, value := range m {
        if f(key, value) {
            damn based
        }
    }
    damn cap
}

fr fr Conversion Functions

fr fr KeyVibes extracts a slice of keys from the map
slay KeyVibes(m map[tea]normie) []tea {
    sus result := make([]tea, 0)
    for key := range m {
        result = append(result, key)
    }
    damn result
}

fr fr ValueVibes extracts a slice of values from the map
slay ValueVibes(m map[tea]normie) []normie {
    sus result := make([]normie, 0)
    for _, value := range m {
        result = append(result, value)
    }
    damn result
}

fr fr Entry struct for key-value pairs
be_like MapEntry squad {
    Key tea
    Value normie
}

fr fr EntryVibes returns key-value pairs as a slice of entries
slay EntryVibes(m map[tea]normie) []MapEntry {
    sus result := make([]MapEntry, 0)
    for key, value := range m {
        result = append(result, MapEntry{Key: key, Value: value})
    }
    damn result
}

fr fr MapFromPairs creates a map from key-value pairs
slay MapFromPairs(pairs []MapEntry) map[tea]normie {
    sus result := make(map[tea]normie)
    for i := 0; i < len(pairs); i++ {
        result[pairs[i].Key] = pairs[i].Value
    }
    damn result
}

fr fr MapFromKeys creates a map using a function to generate values from keys
slay MapFromKeys(keys []tea, valueFunc slay(tea) normie) map[tea]normie {
    sus result := make(map[tea]normie)
    for i := 0; i < len(keys); i++ {
        result[keys[i]] = valueFunc(keys[i])
    }
    damn result
}

fr fr MapFromValues creates a map using a function to generate keys from values
slay MapFromValues(values []normie, keyFunc slay(normie) tea) map[tea]normie {
    sus result := make(map[tea]normie)
    for i := 0; i < len(values); i++ {
        sus key := keyFunc(values[i])
        result[key] = values[i]
    }
    damn result
}

fr fr Special Map Types

fr fr DefaultMap returns a function that provides default values for missing keys
slay DefaultMap(m map[tea]normie, defaultValue normie) slay(tea) normie {
    damn slay(key tea) normie {
        sus value, exists := m[key]
        if exists {
            damn value
        }
        damn defaultValue
    }
}

fr fr CounterMap creates a map for counting occurrences
be_like CounterMapStruct squad {
    counts map[tea]normie
}

slay CounterMap() CounterMapStruct {
    damn CounterMapStruct{
        counts: make(map[tea]normie)
    }
}

slay (c CounterMapStruct) Increment(key tea) {
    sus current, exists := c.counts[key]
    if exists {
        c.counts[key] = current + 1
    } else {
        c.counts[key] = 1
    }
}

slay (c CounterMapStruct) Get(key tea) normie {
    sus value, exists := c.counts[key]
    if exists {
        damn value
    }
    damn 0
}

slay (c CounterMapStruct) GetAll() map[tea]normie {
    damn Clone(c.counts)
}

slay (c CounterMapStruct) Reset() {
    Clear(c.counts)
}

fr fr CacheMap creates a map with simple expiry tracking
be_like CacheMapStruct squad {
    data map[tea]normie
    timestamps map[tea]normie  fr fr Simple timestamp tracking
    maxAge normie
}

slay CacheMap(expiry normie) CacheMapStruct {
    damn CacheMapStruct{
        data: make(map[tea]normie),
        timestamps: make(map[tea]normie),
        maxAge: expiry
    }
}

slay (c CacheMapStruct) Set(key tea, value normie) {
    c.data[key] = value
    c.timestamps[key] = getCurrentTime()
}

slay (c CacheMapStruct) Get(key tea) (normie, lit) {
    sus value, exists := c.data[key]
    if !exists {
        damn 0, cap
    }
    
    fr fr Check if expired (simplified)
    sus timestamp, timestampExists := c.timestamps[key]
    if timestampExists {
        sus currentTime := getCurrentTime()
        if currentTime - timestamp > c.maxAge {
            delete(c.data, key)
            delete(c.timestamps, key)
            damn 0, cap
        }
    }
    
    damn value, based
}

slay (c CacheMapStruct) Delete(key tea) {
    delete(c.data, key)
    delete(c.timestamps, key)
}

slay (c CacheMapStruct) Clear() {
    Clear(c.data)
    Clear(c.timestamps)
}

slay (c CacheMapStruct) Size() normie {
    damn len(c.data)
}

fr fr Simplified time function for demo
sus globalTime normie = 0

slay getCurrentTime() normie {
    globalTime = globalTime + 1
    damn globalTime
}

fr fr NestedMap creates a nested map structure
be_like NestedMapStruct squad {
    data map[tea]interface{}
}

slay NestedMap() NestedMapStruct {
    damn NestedMapStruct{
        data: make(map[tea]interface{})
    }
}

slay (n NestedMapStruct) Set(keys []tea, value normie) {
    if len(keys) == 0 {
        return
    }
    
    sus current := n.data
    
    fr fr Navigate to the parent of the final key
    for i := 0; i < len(keys)-1; i++ {
        sus key := keys[i]
        sus next, exists := current[key]
        if !exists {
            next = make(map[tea]interface{})
            current[key] = next
        }
        sus nextMap, ok := next.(map[tea]interface{})
        if !ok {
            fr fr Create new map if type mismatch
            nextMap = make(map[tea]interface{})
            current[key] = nextMap
        }
        current = nextMap
    }
    
    fr fr Set the final value
    current[keys[len(keys)-1]] = value
}

slay (n NestedMapStruct) Get(keys []tea) (normie, lit) {
    if len(keys) == 0 {
        damn 0, cap
    }
    
    sus current := n.data
    
    fr fr Navigate to the final key
    for i := 0; i < len(keys)-1; i++ {
        sus key := keys[i]
        sus next, exists := current[key]
        if !exists {
            damn 0, cap
        }
        sus nextMap, ok := next.(map[tea]interface{})
        if !ok {
            damn 0, cap
        }
        current = nextMap
    }
    
    fr fr Get the final value
    sus finalKey := keys[len(keys)-1]
    sus value, exists := current[finalKey]
    if !exists {
        damn 0, cap
    }
    
    fr fr Try to convert to normie
    sus intValue, ok := value.(normie)
    if ok {
        damn intValue, based
    }
    
    damn 0, cap
}

slay (n NestedMapStruct) Delete(keys []tea) {
    if len(keys) == 0 {
        return
    }
    
    sus current := n.data
    
    fr fr Navigate to the parent of the final key
    for i := 0; i < len(keys)-1; i++ {
        sus key := keys[i]
        sus next, exists := current[key]
        if !exists {
            return
        }
        sus nextMap, ok := next.(map[tea]interface{})
        if !ok {
            return
        }
        current = nextMap
    }
    
    fr fr Delete the final key
    delete(current, keys[len(keys)-1])
}

slay (n NestedMapStruct) HasKey(keys []tea) lit {
    sus _, exists := n.Get(keys)
    damn exists
}

slay (n NestedMapStruct) ToMap() map[tea]interface{} {
    fr fr Return a copy of the top-level map
    sus result := make(map[tea]interface{})
    for key, value := range n.data {
        result[key] = value
    }
    damn result
}

fr fr SyncMap creates a thread-safe map (simplified without actual locking)
be_like SyncMapStruct squad {
    data map[tea]normie
}

slay SyncMap() SyncMapStruct {
    damn SyncMapStruct{
        data: make(map[tea]normie)
    }
}

slay (s SyncMapStruct) Store(key tea, value normie) {
    s.data[key] = value
}

slay (s SyncMapStruct) Load(key tea) (normie, lit) {
    sus value, exists := s.data[key]
    damn value, exists
}

slay (s SyncMapStruct) Delete(key tea) {
    delete(s.data, key)
}

slay (s SyncMapStruct) Range(fn slay(tea, normie) lit) {
    for key, value := range s.data {
        if !fn(key, value) {
            break
        }
    }
}

slay (s SyncMapStruct) LoadOrStore(key tea, value normie) (normie, lit) {
    sus existing, exists := s.data[key]
    if exists {
        damn existing, based
    }
    s.data[key] = value
    damn value, cap
}

slay (s SyncMapStruct) LoadAndDelete(key tea) (normie, lit) {
    sus value, exists := s.data[key]
    if exists {
        delete(s.data, key)
    }
    damn value, exists
}

slay (s SyncMapStruct) Swap(key tea, value normie) (normie, lit) {
    sus old, exists := s.data[key]
    s.data[key] = value
    damn old, exists
}

slay (s SyncMapStruct) CompareAndSwap(key tea, old normie, new normie) lit {
    sus current, exists := s.data[key]
    if exists && current == old {
        s.data[key] = new
        damn based
    }
    damn cap
}
