# Baseline Benchmark 4: String Processing
# Tests: String creation, concatenation, searching, manipulation

yeet "stringz"
yeet "timez"

slay create_strings(count drip) []tea {
    sus strings []tea = []
    bestie (sus i drip = 0; i < count; i++) {
        sus base tea = "string_number_"
        sus num_str tea = stringz.from_int(i)
        sus full_string tea = stringz.concat(base, num_str)
        strings = stringz.push_string_array(strings, full_string)
    }
    damn strings
}

slay process_strings(strings []tea) drip {
    sus total_length drip = 0
    sus pattern_count drip = 0
    sus len drip = stringz.string_array_len(strings)
    
    bestie (sus i drip = 0; i < len; i++) {
        sus str tea = strings[i]
        total_length = total_length + stringz.len(str)
        
        # Count occurrences of "number"
        ready (stringz.contains(str, "number")) {
            pattern_count = pattern_count + 1
        }
    }
    
    damn total_length + pattern_count
}

slay manipulate_strings(strings []tea) []tea {
    sus modified []tea = []
    sus len drip = stringz.string_array_len(strings)
    
    bestie (sus i drip = 0; i < len; i++) {
        sus str tea = strings[i]
        sus upper tea = stringz.to_upper(str)
        sus reversed tea = stringz.reverse(upper)
        modified = stringz.push_string_array(modified, reversed)
    }
    
    damn modified
}

slay benchmark_strings() drip {
    sus iterations drip = 500
    sus string_count drip = 200
    sus total drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        sus strings []tea = create_strings(string_count)
        total = total + process_strings(strings)
        sus modified []tea = manipulate_strings(strings)
        total = total + stringz.string_array_len(modified)
    }
    
    damn total
}

slay main() drip {
    sus start drip = timez.now_microseconds()
    sus result drip = benchmark_strings()
    sus end drip = timez.now_microseconds()
    
    vibez.spill("Strings benchmark result:", result)
    vibez.spill("Execution time (μs):", end - start)
    
    damn 0
}
