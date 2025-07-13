yeet "testz"

// Simple compression test
slay simple_compress(data tea) tea {
    vibes string_len(data) == 0 {
        damn ""
    }
    
    sus result tea = ""
    bestie i := 0; i < string_len(data); i++ {
        result = result + string_char_at(data, i)
    }
    
    damn result
}

vibez.spill("Testing simple compression...")
sus test tea = simple_compress("hello")
vibez.spill("Result: " + test)
