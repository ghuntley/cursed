yeet "collections"

vibez.spill("Testing Vector operations...")

fr fr Test Vec_new and Vec_len
sus vec1 extra = collections.Vec_new()
vibez.spill("New vector length: ")
vibez.spill(collections.Vec_len(vec1))

fr fr Test Vec_push
sus vec2 extra = collections.Vec_push(vec1, 42)
sus vec3 extra = collections.Vec_push(vec2, 100) 

vibez.spill("Vector after pushing 42, 100:")
vibez.spill("Length: ")
vibez.spill(collections.Vec_len(vec3))

fr fr Test Vec_get
vibez.spill("Element at index 0: ")
vibez.spill(collections.Vec_get(vec3, 0))
vibez.spill("Element at index 1: ")
vibez.spill(collections.Vec_get(vec3, 1))

fr fr Test HashMap operations
vibez.spill("Testing HashMap operations...")
sus map1 extra = collections.Map_new()
sus map2 extra = collections.Map_insert(map1, "name", "John")
sus map3 extra = collections.Map_insert(map2, "age", "30")

vibez.spill("Map length after inserts: ")
vibez.spill(collections.Map_len(map3))

vibez.spill("Value for 'name': ")
vibez.spill(collections.Map_get(map3, "name"))

vibez.spill("Contains 'name'? ")
vibez.spill(collections.Map_contains_key(map3, "name"))

vibez.spill("Collections test complete!")
