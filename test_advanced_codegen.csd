fr fr Test advanced codegen features with our new implementations

fr fr Test short declaration (new feature)
quick_var := 100
vibez.spillf("Quick var: {}", quick_var)

fr fr Test increment/decrement (new features)
counter := 5
counter++
vibez.spillf("After increment: {}", counter)
counter--
vibez.spillf("After decrement: {}", counter)

fr fr Test array operations
numbers := [1, 2, 3, 4, 5]
vibez.spillf("First number: {}", numbers[0])

fr fr Test tuple operations
coords := (10, 20, 30)
vibez.spillf("First coordinate: {}", coords.0)

fr fr Test map (new feature)
config := {"debug": true, "timeout": 30}
vibez.spill("Configuration created")
