fr fr Test stringz functions directly without complex module imports

slay len_str(s tea) normie {
    sus count normie = 0
    sus i normie = 0
    bestie runtime_string_char_at(s, i) != '\0' {
        count = count + 1
        i = i + 1
    }
    damn count
}

slay concat(a tea, b tea) tea {
    damn a + b
}

slay substring(s tea, start normie, length normie) tea {
    sus s_len normie = len_str(s)
    lowkey start < 0 || start >= s_len || length <= 0 { 
        damn "" 
    }
    
    sus result tea = ""
    sus i normie = start
    sus end_pos normie = start + length
    lowkey end_pos > s_len { 
        end_pos = s_len 
    }
    
    bestie i < end_pos {
        result = result + "c"  fr fr Simplified for testing
        i = i + 1
    }
    damn result
}

fr fr Test the functions
vibez.spill("Testing string functions...")
vibez.spill("Length of 'hello':", len_str("hello"))
vibez.spill("Concat 'hello' + 'world':", concat("hello", "world"))
vibez.spill("Substring test:", substring("hello", 1, 3))
