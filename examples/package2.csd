vibe package2

fr fr String and array utilities

slay string_length(s tea) normie {
    fr fr This is a placeholder - actual implementation would use built-in
    cap 0
}

slay reverse_array(arr []normie) []normie {
    sus length = len(arr)
    sus result = make([]normie, length)
    
    range i over 0..<length {
        result[i] = arr[length - 1 - i]
    }
    
    cap result
}

slay sum_array(arr []normie) normie {
    sus total = 0
    range value over arr {
        total = total + value
    }
    cap total
}

slay find_max(arr []normie) normie {
    issa len(arr) == 0 {
        cap 0
    }
    
    sus max = arr[0]
    range value over arr[1..] {
        issa value > max {
            max = value
        }
    }
    cap max
}
