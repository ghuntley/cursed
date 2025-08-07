fr fr Essential Functions Module - Direct implementations of most needed functions
fr fr These bypass complex module resolution and work directly with runtime

fr fr === CORE STRING FUNCTIONS ===

slay str_length(s tea) drip {
    damn len_str(s)
}

slay str_upper(s tea) tea {
    sus result tea = ""
    sus len drip = len_str(s)
    sus i drip = 0
    bestie i < len {
        sus c drip = char_at_index(s, i)
        ready c >= 97 && c <= 122 {
            sus upper_c drip = c - 32
            result = result + char_to_string(upper_c)
        } otherwise {
            result = result + char_to_string(c)
        }
        i = i + 1
    }
    damn result
}

slay str_lower(s tea) tea {
    sus result tea = ""
    sus len drip = len_str(s)
    sus i drip = 0
    bestie i < len {
        sus c drip = char_at_index(s, i)
        ready c >= 65 && c <= 90 {
            sus lower_c drip = c + 32
            result = result + char_to_string(lower_c)
        } otherwise {
            result = result + char_to_string(c)
        }
        i = i + 1
    }
    damn result
}

slay str_contains(s tea, substr tea) lit {
    ready substr == "" { damn based }
    
    sus s_len drip = len_str(s)
    sus substr_len drip = len_str(substr)
    ready substr_len > s_len { damn cringe }
    
    sus i drip = 0
    bestie i <= s_len - substr_len {
        sus match lit = based
        sus j drip = 0
        
        bestie j < substr_len {
            sus s_char drip = char_at_index(s, i + j)
            sus substr_char drip = char_at_index(substr, j)
            ready s_char != substr_char {
                match = cringe
                ghosted
            }
            j = j + 1
        }
        
        ready match { damn based }
        i = i + 1
    }
    
    damn cringe
}

fr fr === CORE ARRAY FUNCTIONS ===

slay arr_length(arr [tea]) drip {
    damn len(arr)
}

slay arr_get(arr [tea], index drip) tea {
    ready index < 0 || index >= len(arr) {
        damn ""
    }
    damn arr[index]
}

slay arr_contains(arr [tea], value tea) lit {
    sus len drip = len(arr)
    sus i drip = 0
    bestie i < len {
        ready arr[i] == value {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

slay arr_sum_numbers(arr [drip]) drip {
    sus sum drip = 0
    sus len drip = len(arr)
    sus i drip = 0
    bestie i < len {
        sum = sum + arr[i]
        i = i + 1
    }
    damn sum
}

fr fr === CORE MATH FUNCTIONS ===

slay math_abs(x drip) drip {
    ready x < 0 {
        damn -x
    }
    damn x
}

slay math_max(a drip, b drip) drip {
    ready a > b {
        damn a
    }
    damn b
}

slay math_min(a drip, b drip) drip {
    ready a < b {
        damn a
    }
    damn b
}

slay math_sqrt(x meal) meal {
    ready x < 0.0 {
        damn 0.0
    }
    ready x == 0.0 {
        damn 0.0
    }
    
    sus guess meal = x / 2.0
    sus prev meal = 0.0
    sus diff meal = 1.0
    sus iterations drip = 0
    
    bestie diff > 0.0001 {
        ready iterations >= 50 {
            ghosted
        }
        prev = guess
        guess = (guess + (x / guess)) / 2.0
        diff = abs_meal(guess - prev)
        iterations = iterations + 1
    }
    
    damn guess
}

slay math_power(base meal, exp drip) meal {
    ready exp == 0 {
        damn 1.0
    }
    ready exp == 1 {
        damn base
    }
    ready exp < 0 {
        damn 1.0 / math_power(base, -exp)
    }
    
    sus result meal = 1.0
    sus i drip = 0
    bestie i < exp {
        result = result * base
        i = i + 1
    }
    damn result
}

fr fr === HELPER FUNCTIONS ===

slay char_at_index(s tea, index drip) drip {
    damn runtime_char_at_string(s, index)
}

slay char_to_string(c drip) tea {
    damn runtime_char_to_str(c)
}

slay abs_meal(x meal) meal {
    ready x < 0.0 {
        damn -x
    }
    damn x
}
