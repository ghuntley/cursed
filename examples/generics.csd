vibe main

fr fr Define a generic Box type
be_like Box[T] squad {
    value T
}

fr fr Define a generic Pair type
be_like Pair[A, B] squad {
    first A
    second B
}

fr fr Define a generic interface
be_like Container[T] collab {
    add(item T)
    get(index normie) T
    size() normie
}

fr fr Define a function with generic parameters
slay map[T, U](items []T, transformer slay(T) U) []U {
    sus result = make([]U, len(items))
    sus i = 0
    
    bestie i < len(items) {
        result[i] = transformer(items[i])
        i = i + 1
    }
    
    damn result
}

fr fr Example implementation of Container
be_like List[T] squad {
    items []T
    length normie
}

fr fr Implement Container for List
slay add[T](l @List[T], item T) {
    l.items = append(l.items, item)
    l.length = l.length + 1
}

slay get[T](l @List[T], index normie) T {
    lowkey index < 0 || index >= l.length {
        panic("Index out of bounds")
    }
    damn l.items[index]
}

slay size[T](l @List[T]) normie {
    damn l.length
}

fr fr Main function to demonstrate the usage
slay main_character() {
    fr fr Create a Box of normie
    sus box_int = Box[normie]{
        value: 42
    }
    puts(box_int.value)
    
    fr fr Create a Pair of tea and normie
    sus pair = Pair[tea, normie]{
        first: "Hello",
        second: 123
    }
    puts(pair.first)
    puts(pair.second)
    
    fr fr Use the generic map function
    sus numbers = []normie{1, 2, 3, 4, 5}
    sus toStrings = slay(n normie) tea {
        damn sprintf("%d", n)
    }
    
    sus strings = map[normie, tea](numbers, toStrings)
    
    fr fr Create a list of integers
    sus myList = List[normie]{
        items: []normie{},
        length: 0
    }
    
    fr fr Add items
    add[normie](@myList, 10)
    add[normie](@myList, 20)
    add[normie](@myList, 30)
    
    fr fr Get an item
    sus val = get[normie](@myList, 1)
    puts(val) fr fr Should output 20
    
    fr fr Get the size
    sus listSize = size[normie](@myList)
    puts(listSize) fr fr Should output 3
}