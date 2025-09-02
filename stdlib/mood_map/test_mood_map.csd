yeet "testz"
yeet "mood_map"

test_start("mood_map basic operations")

fr fr Test Clone
sus original := make(map[tea]normie)
original["one"] = 1
original["two"] = 2
original["three"] = 3

sus cloned := mood_map.Clone(original)
assert_eq_int(cloned["one"], 1)
assert_eq_int(cloned["two"], 2)
assert_eq_int(cloned["three"], 3)

fr fr Test Copy
sus destination := make(map[tea]normie)
mood_map.Copy(destination, original)
assert_eq_int(destination["one"], 1)
assert_eq_int(destination["two"], 2)

fr fr Test Clear
mood_map.Clear(destination)
assert_eq_int(len(destination), 0)

test_start("mood_map DeleteFunc")

fr fr Test DeleteFunc - remove even values
sus testMap := make(map[tea]normie)
testMap["one"] = 1
testMap["two"] = 2
testMap["three"] = 3
testMap["four"] = 4

mood_map.DeleteFunc(testMap, slay(key tea, value normie) lit {
    damn value % 2 == 0
})

assert_eq_int(len(testMap), 2)
assert_eq_int(testMap["one"], 1)
assert_eq_int(testMap["three"], 3)

test_start("mood_map Vibe filtering")

fr fr Test Vibe - filter for even values
sus sourceMap := make(map[tea]normie)
sourceMap["one"] = 1
sourceMap["two"] = 2
sourceMap["three"] = 3
sourceMap["four"] = 4

sus evenMap := mood_map.Vibe(sourceMap, slay(key tea, value normie) lit {
    damn value % 2 == 0
})

assert_eq_int(len(evenMap), 2)
assert_eq_int(evenMap["two"], 2)
assert_eq_int(evenMap["four"], 4)

test_start("mood_map Moody transformation")

fr fr Test Moody - square all values
sus transformed := mood_map.Moody(sourceMap, slay(key tea, value normie) normie {
    damn value * value
})

assert_eq_int(transformed["one"], 1)
assert_eq_int(transformed["two"], 4)
assert_eq_int(transformed["three"], 9)
assert_eq_int(transformed["four"], 16)

test_start("mood_map MoodSwitch")

fr fr Test MoodSwitch - split by even/odd
sus even, odd := mood_map.MoodSwitch(sourceMap, slay(key tea, value normie) lit {
    damn value % 2 == 0
})

assert_eq_int(len(even), 2)
assert_eq_int(len(odd), 2)
assert_eq_int(even["two"], 2)
assert_eq_int(even["four"], 4)
assert_eq_int(odd["one"], 1)
assert_eq_int(odd["three"], 3)

test_start("mood_map MoodyMerge")

fr fr Test MoodyMerge - merge with sum resolver
sus map1 := make(map[tea]normie)
map1["a"] = 1
map1["b"] = 2

sus map2 := make(map[tea]normie)
map2["b"] = 3
map2["c"] = 4

sus maps := map[value][tea]normie{map1, map2}
sus merged := mood_map.MoodyMerge(maps, slay(key tea, existing normie, new normie) normie {
    damn existing + new
})

assert_eq_int(merged["a"], 1)
assert_eq_int(merged["b"], 5)  fr fr 2 + 3
assert_eq_int(merged["c"], 4)

test_start("mood_map MoodCheck and FeelingSome")

fr fr Test MoodCheck - all positive
sus positiveMap := make(map[tea]normie)
positiveMap["a"] = 1
positiveMap["b"] = 2
positiveMap["c"] = 3

sus allPositive := mood_map.MoodCheck(positiveMap, slay(key tea, value normie) lit {
    damn value > 0
})
assert_true(allPositive)

fr fr Test MoodCheck - not all even
sus notAllEven := mood_map.MoodCheck(positiveMap, slay(key tea, value normie) lit {
    damn value % 2 == 0
})
assert_false(notAllEven)

fr fr Test FeelingSome - has even values
sus hasEven := mood_map.FeelingSome(positiveMap, slay(key tea, value normie) lit {
    damn value % 2 == 0
})
assert_true(hasEven)

fr fr Test FeelingSome - has negative values
sus hasNegative := mood_map.FeelingSome(positiveMap, slay(key tea, value normie) lit {
    damn value < 0
})
assert_false(hasNegative)

test_start("mood_map conversion functions")

fr fr Test KeyVibes
sus keys := mood_map.KeyVibes(positiveMap)
assert_eq_int(len(keys), 3)
assert_true(contains(keys, "a"))
assert_true(contains(keys, "b"))
assert_true(contains(keys, "c"))

fr fr Test ValueVibes
sus values := mood_map.ValueVibes(positiveMap)
assert_eq_int(len(values), 3)
assert_true(containsInt(values, 1))
assert_true(containsInt(values, 2))
assert_true(containsInt(values, 3))

fr fr Test EntryVibes
sus entries := mood_map.EntryVibes(positiveMap)
assert_eq_int(len(entries), 3)
assert_eq_string(entries[0].Key, "a")
assert_eq_int(entries[0].Value, 1)

test_start("mood_map MapFromPairs")

fr fr Test MapFromPairs
sus pairs := mood_map[value].MapEntry{
    {Key: "x", Value: 10},
    {Key: "y", Value: 20},
    {Key: "z", Value: 30}
}
sus fromPairs := mood_map.MapFromPairs(pairs)
assert_eq_int(fromPairs["x"], 10)
assert_eq_int(fromPairs["y"], 20)
assert_eq_int(fromPairs["z"], 30)

test_start("mood_map MapFromKeys and MapFromValues")

fr fr Test MapFromKeys
sus keyList := tea[value]{"a", "b", "c"}
sus fromKeys := mood_map.MapFromKeys(keyList, slay(key tea) normie {
    damn len(key)
})
assert_eq_int(fromKeys["a"], 1)
assert_eq_int(fromKeys["b"], 1)
assert_eq_int(fromKeys["c"], 1)

fr fr Test MapFromValues
sus valueList := normie[value]{10, 20, 30}
sus fromValues := mood_map.MapFromValues(valueList, slay(value normie) tea {
    damn "key" + tea(value)
})
assert_eq_int(fromValues["key10"], 10)
assert_eq_int(fromValues["key20"], 20)
assert_eq_int(fromValues["key30"], 30)

test_start("mood_map DefaultMap")

fr fr Test DefaultMap
sus baseMap := make(map[tea]normie)
baseMap["exists"] = 42

sus defaultMapFunc := mood_map.DefaultMap(baseMap, 999)
assert_eq_int(defaultMapFunc("exists"), 42)
assert_eq_int(defaultMapFunc("missing"), 999)

test_start("mood_map CounterMap")

fr fr Test CounterMap
sus counter := mood_map.CounterMap()
counter.Increment("a")
counter.Increment("a")
counter.Increment("b")

assert_eq_int(counter.Get("a"), 2)
assert_eq_int(counter.Get("b"), 1)
assert_eq_int(counter.Get("c"), 0)

sus counts := counter.GetAll()
assert_eq_int(counts["a"], 2)
assert_eq_int(counts["b"], 1)

counter.Reset()
assert_eq_int(counter.Get("a"), 0)

test_start("mood_map CacheMap")

fr fr Test CacheMap
sus cache := mood_map.CacheMap(5)  fr fr Expiry of 5 time units
cache.Set("key1", 100)
cache.Set("key2", 200)

sus value1, found1 := cache.Get("key1")
assert_true(found1)
assert_eq_int(value1, 100)

sus value2, found2 := cache.Get("key2")
assert_true(found2)
assert_eq_int(value2, 200)

sus missing, foundMissing := cache.Get("missing")
assert_false(foundMissing)

assert_eq_int(cache.Size(), 2)

cache.Delete("key1")
assert_eq_int(cache.Size(), 1)

cache.Clear()
assert_eq_int(cache.Size(), 0)

test_start("mood_map NestedMap")

fr fr Test NestedMap
sus nested := mood_map.NestedMap()
sus keys1 := tea[value]{"user", "profile", "age"}
nested.Set(keys1, 25)

sus age, foundAge := nested.Get(keys1)
assert_true(foundAge)
assert_eq_int(age, 25)

sus keys2 := tea[value]{"user", "profile", "name"}
nested.Set(keys2, 42)  fr fr Using normie for demo

sus name, foundName := nested.Get(keys2)
assert_true(foundName)
assert_eq_int(name, 42)

assert_true(nested.HasKey(keys1))
assert_true(nested.HasKey(keys2))

sus nonExistentKeys := tea[value]{"user", "settings", "theme"}
assert_false(nested.HasKey(nonExistentKeys))

nested.Delete(keys1)
assert_false(nested.HasKey(keys1))
assert_true(nested.HasKey(keys2))

test_start("mood_map SyncMap")

fr fr Test SyncMap
sus syncMap := mood_map.SyncMap()
syncMap.Store("key1", 100)
syncMap.Store("key2", 200)

sus val1, found1 := syncMap.Load("key1")
assert_true(found1)
assert_eq_int(val1, 100)

sus val2, found2 := syncMap.Load("key2")
assert_true(found2)
assert_eq_int(val2, 200)

fr fr Test LoadOrStore
sus existing, loaded := syncMap.LoadOrStore("key1", 999)
assert_true(loaded)  fr fr Key already existed
assert_eq_int(existing, 100)

sus newVal, newLoaded := syncMap.LoadOrStore("key3", 300)
assert_false(newLoaded)  fr fr New key was stored
assert_eq_int(newVal, 300)

fr fr Test LoadAndDelete
sus deletedVal, wasDeleted := syncMap.LoadAndDelete("key2")
assert_true(wasDeleted)
assert_eq_int(deletedVal, 200)

sus _, notFound := syncMap.Load("key2")
assert_false(notFound)

fr fr Test Swap
sus oldVal, hadOld := syncMap.Swap("key1", 150)
assert_true(hadOld)
assert_eq_int(oldVal, 100)

sus currentVal, found := syncMap.Load("key1")
assert_true(found)
assert_eq_int(currentVal, 150)

fr fr Test CompareAndSwap
sus swapped := syncMap.CompareAndSwap("key1", 150, 175)
assert_true(swapped)

sus notSwapped := syncMap.CompareAndSwap("key1", 999, 200)
assert_false(notSwapped)

fr fr Test Range
sus count := 0
syncMap.Range(slay(key tea, value normie) lit {
    count++
    damn based  fr fr Continue iteration
})
assert_true(count >= 2)  fr fr Should have at least key1 and key3

print_test_summary()

fr fr Helper functions for tests
slay contains(slice tea[value], item tea) lit {
    for i := 0; i < len(slice); i++ {
        if slice[i] == item {
            damn based
        }
    }
    damn cap
}

slay containsInt(slice normie[value], item normie) lit {
    for i := 0; i < len(slice); i++ {
        if slice[i] == item {
            damn based
        }
    }
    damn cap
}
