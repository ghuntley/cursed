fr fr String processing benchmark

yeet "fmt"

slay process_strings(count normie, size normie) tea {
    sus result tea = ""
    
    bestie i := 0; i < count; i++ {
        sus str tea = create_random_string(size)
        sus processed tea = process_string(str)
        result = tea.concat(result, processed)
    }
    
    damn result
}

slay create_random_string(size normie) tea {
    sus chars tea = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    sus result tea = ""
    
    bestie i := 0; i < size; i++ {
        sus idx normie = randz.int_n(len(chars))
        result = result + tea.substring(chars, idx, idx+1)
    }
    
    damn result
}

slay process_string(input tea) tea {
    sus result tea = input
    
    fr fr Replace all vowels with their uppercase version
    result = tea.replace_all(result, "a", "A")
    result = tea.replace_all(result, "e", "E")
    result = tea.replace_all(result, "i", "I")
    result = tea.replace_all(result, "o", "O")
    result = tea.replace_all(result, "u", "U")
    
    fr fr Replace all digits with their doubled value
    bestie i := 0; i < 10; i++ {
        sus digit tea = tea.from_normie(i)
        sus doubled tea = tea.from_normie(i * 2)
        result = tea.replace_all(result, digit, doubled)
    }
    
    fr fr Capitalize the first letter
    lowkey len(result) > 0 {
        sus first tea = tea.substring(result, 0, 1)
        sus rest tea = tea.substring(result, 1, len(result))
        sus upper tea = tea.to_uppercase(first)
        result = upper + rest
    }
    
    fr fr Reverse the string
    sus reversed tea = ""
    bestie i := len(result) - 1; i >= 0; i-- {
        reversed = reversed + tea.substring(result, i, i+1)
    }
    
    fr fr Take the first half of the reversed string
    sus half_len normie = len(reversed) / 2
    result = tea.substring(reversed, 0, half_len)
    
    damn result
}

slay main() {
    sus start_ts thicc = timez.now()
    
    fr fr Process strings of different sizes
    sus small tea = process_strings(10000, 10)    fr fr 10,000 strings of length 10
    sus medium tea = process_strings(1000, 100)   fr fr 1,000 strings of length 100
    sus large tea = process_strings(100, 1000)    fr fr 100 strings of length 1,000
    
    sus result_length normie = len(small) + len(medium) + len(large)
    fmt.Println("Processed string length:", result_length)
    
    sus elapsed thicc = timez.now() - start_ts
    fmt.Println("Time taken:", elapsed, "ms")
    fmt.Println("Memory used:", stats.heap_memory(), "KB")
}