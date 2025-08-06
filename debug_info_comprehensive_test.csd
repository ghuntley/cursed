sus global_counter drip = 0

squad Point {
    spill x normie
    spill y normie
}

slay add_numbers(first normie, second normie) normie {
    sus local_sum normie = first + second
    sus debug_point Point = Point{ x: local_sum, y: 42 }
    
    sus loop_counter drip = 0
    bestie (loop_counter < 3) {
        sus inner_val normie = loop_counter.(normie)
        local_sum = local_sum + inner_val
        loop_counter = loop_counter + 1
    }
    
    damn local_sum
}

slay string_operations(input tea) tea {
    sus local_string tea = input + " processed"
    sus char_count normie = local_string.len()
    
    match char_count {
        case 0:
            damn "empty"
        case 1..10:
            damn "short"
        default:
            damn local_string
    }
}

slay main() {
    sus x normie = 10
    sus y normie = 20
    sus result normie = add_numbers(x, y)
    
    vibez.spill("Result: ")
    vibez.spill(result)
    
    sus text tea = "hello"
    sus processed tea = string_operations(text)
    vibez.spill(processed)
    
    sus point Point = Point{ x: result, y: 100 }
    vibez.spill("Point x: ")
    vibez.spill(point.x)
    
    global_counter = global_counter + 1
}
