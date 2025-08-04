fr fr Test complex type parsing

slay process_map(data map[tea]normie) normie {
    vibez.spill("Processing map with complex types")
    damn 42
}

slay channel_handler(ch chan<normie>) {
    vibez.spill("Handling channel with complex types")
}

slay tuple_function(coords (meal, meal)) meal {
    vibez.spill("Processing tuple types")
    damn 3.14
}

slay main() {
    sus data map[tea]normie = {}
    sus result normie = process_map(data)
    
    sus coordinates (meal, meal) = (1.0, 2.0)
    sus area meal = tuple_function(coordinates)
    
    vibez.spill("Complex types test completed")
}

main()
