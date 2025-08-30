yeet "collections"

fr fr Test Vector operations
vibez.spill("Testing Collections Module")
vibez.spill("=========================")

fr fr Test Vec_new and Vec_len
sus vec1 extra = collections.Vec_new()
vibez.spill("New vector length: ")
vibez.spill(collections.Vec_len(vec1))

fr fr Test Vec_push
sus vec2 extra = collections.Vec_push(vec1, 42)
sus vec3 extra = collections.Vec_push(vec2, 100) 
sus vec4 extra = collections.Vec_push(vec3, 7)

vibez.spill("Vector after pushing 42, 100, 7:")
vibez.spill("Length: ")
vibez.spill(collections.Vec_len(vec4))

fr fr Test Vec_get
vibez.spill("Element at index 0: ")
vibez.spill(collections.Vec_get(vec4, 0))
vibez.spill("Element at index 1: ")
vibez.spill(collections.Vec_get(vec4, 1))
vibez.spill("Element at index 2: ")
vibez.spill(collections.Vec_get(vec4, 2))

fr fr Test Vec_pop
sus last_elem extra = collections.Vec_pop(vec4)
vibez.spill("Popped element: ")
vibez.spill(last_elem)

fr fr Test Vec_set
sus vec5 extra = collections.Vec_set(vec4, 1, 999)
vibez.spill("After setting index 1 to 999:")
vibez.spill("Element at index 1: ")
vibez.spill(collections.Vec_get(vec5, 1))

vibez.spill("")
vibez.spill("Testing HashMap operations")
vibez.spill("==========================")

fr fr Test Map operations
sus map1 extra = collections.Map_new()
sus map2 extra = collections.Map_insert(map1, "name", "John")
sus map3 extra = collections.Map_insert(map2, "age", "30")
sus map4 extra = collections.Map_insert(map3, "city", "NYC")

vibez.spill("Map length after 3 inserts: ")
vibez.spill(collections.Map_len(map4))

vibez.spill("Value for 'name': ")
vibez.spill(collections.Map_get(map4, "name"))

vibez.spill("Value for 'age': ")
vibez.spill(collections.Map_get(map4, "age"))

vibez.spill("Contains 'name'? ")
vibez.spill(collections.Map_contains_key(map4, "name"))

vibez.spill("Contains 'unknown'? ")
vibez.spill(collections.Map_contains_key(map4, "unknown"))

fr fr Test map keys
sus keys extra = collections.Map_keys(map4)
vibez.spill("Number of keys: ")
vibez.spill(collections.Vec_len(keys))

vibez.spill("")
vibez.spill("Testing Set operations")
vibez.spill("======================")

fr fr Test Set operations
sus set1 extra = collections.Set_new()
sus set2 extra = collections.Set_insert(set1, 10)
sus set3 extra = collections.Set_insert(set2, 20)
sus set4 extra = collections.Set_insert(set3, 10) fr fr Duplicate, should not change size

vibez.spill("Set length after inserting 10, 20, 10: ")
vibez.spill(collections.Set_len(set4))

vibez.spill("Set contains 10? ")
vibez.spill(collections.Set_contains(set4, 10))

vibez.spill("Set contains 30? ")
vibez.spill(collections.Set_contains(set4, 30))

sus set5 extra = collections.Set_remove(set4, 10)
vibez.spill("Set length after removing 10: ")
vibez.spill(collections.Set_len(set5))

vibez.spill("")
vibez.spill("Testing Sorting algorithms")
vibez.spill("==========================")

fr fr Create unsorted array
sus unsorted extra = collections.Vec_new()
sus unsorted2 extra = collections.Vec_push(unsorted, 64)
sus unsorted3 extra = collections.Vec_push(unsorted2, 34)
sus unsorted4 extra = collections.Vec_push(unsorted3, 25)
sus unsorted5 extra = collections.Vec_push(unsorted4, 12)
sus unsorted6 extra = collections.Vec_push(unsorted5, 22)

vibez.spill("Original array elements:")
vibez.spill("Index 0: ")
vibez.spill(collections.Vec_get(unsorted6, 0))
vibez.spill("Index 1: ")
vibez.spill(collections.Vec_get(unsorted6, 1))
vibez.spill("Index 2: ")
vibez.spill(collections.Vec_get(unsorted6, 2))

fr fr Test quick sort
sus sorted_quick extra = collections.quick_sort(unsorted6)
vibez.spill("After quicksort:")
vibez.spill("Index 0: ")
vibez.spill(collections.Vec_get(sorted_quick, 0))
vibez.spill("Index 1: ")
vibez.spill(collections.Vec_get(sorted_quick, 1))
vibez.spill("Index 2: ")
vibez.spill(collections.Vec_get(sorted_quick, 2))

fr fr Test bubble sort
sus sorted_bubble extra = collections.bubble_sort(unsorted6)
vibez.spill("After bubble sort:")
vibez.spill("Index 0: ")
vibez.spill(collections.Vec_get(sorted_bubble, 0))
vibez.spill("Index 1: ")
vibez.spill(collections.Vec_get(sorted_bubble, 1))
vibez.spill("Index 2: ")
vibez.spill(collections.Vec_get(sorted_bubble, 2))

vibez.spill("")
vibez.spill("Collections module test complete!")
