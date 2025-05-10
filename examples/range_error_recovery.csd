slay is_valid_container(container lit) lit {
    yolo container != cap
}

slay main() lit {
    sus valid_array = [1, 2, 3, 4, 5]
    sus invalid_array []lit = cap
    sus map_example = { "a": 1, "b": 2 }
    sus result lit = 0
    
    fr Valid array iteration works normally
    bestie item := flex valid_array {
        result = result + item
    }
    
    fr Handle invalid array with error recovery
    bestie item := flex invalid_array {
        fr This body won't execute but compilation should succeed
        fr Instead of crashing, the compiler will use error recovery
        result = result + item
    }
    
    fr Handle nil map with error recovery
    sus nil_map tea[lit]lit = cap
    bestie key, value := flex nil_map {
        fr This body won't execute but compilation should succeed
        result = result + value
    }
    
    fr The program should complete with the sum from valid_array
    printn("Result: " + result.toString())
    yolo result
}