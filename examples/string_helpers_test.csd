vibe string_helpers_test

yeet "vibez"  fr fr For printing results
yeet "stringz" fr fr String manipulation package

slay main() {
    vibez.spill("Testing enhanced stringz package functions")
    
    fr fr Test string searching
    test_string_searching()
    
    fr fr Test string trimming
    test_string_trimming()
    
    fr fr Test string replacement
    test_string_replacement()
    
    fr fr Test string repetition
    test_string_repetition()
    
    vibez.spill("All string helper tests passed!")
}

slay test_string_searching() {
    vibez.spill("Testing string searching functions")
    
    tea s := "Hello, world!"
    
    fr fr Test index
    lowkey stringz.index(s, "world") == 7 {
        vibez.spill("index: Found 'world' at position 7")
    } highkey {
        panic("index: Failed to find 'world' at correct position. Got: " + stringz.index(s, "world"))
    }
    
    lowkey stringz.index(s, "universe") == -1 {
        vibez.spill("index: 'universe' not found, returned -1 as expected")
    } highkey {
        panic("index: Found 'universe' unexpectedly at position: " + stringz.index(s, "universe"))
    }
    
    fr fr Test last_index
    tea repeated := "hello hello hello"
    lowkey stringz.last_index(repeated, "hello") == 12 {
        vibez.spill("last_index: Found last 'hello' at position 12")
    } highkey {
        panic("last_index: Failed to find last 'hello' at correct position. Got: " + 
              stringz.last_index(repeated, "hello"))
    }
    
    vibez.spill("String searching functions passed")
}

slay test_string_trimming() {
    vibez.spill("Testing string trimming functions")
    
    fr fr Test trim_space
    tea whitespace := "  hello world  "
    tea trimmed := stringz.trim_space(whitespace)
    lowkey trimmed == "hello world" {
        vibez.spill("trim_space: Correctly trimmed whitespace")
    } highkey {
        panic("trim_space: Failed to trim whitespace. Got: '" + trimmed + "'")
    }
    
    fr fr Test trim_prefix
    tea prefixed := "prefixHello"
    tea without_prefix := stringz.trim_prefix(prefixed, "prefix")
    lowkey without_prefix == "Hello" {
        vibez.spill("trim_prefix: Correctly removed prefix")
    } highkey {
        panic("trim_prefix: Failed to remove prefix. Got: '" + without_prefix + "'")
    }
    
    fr fr Test when prefix doesn't exist
    tea non_prefixed := "Hello"
    tea still_same := stringz.trim_prefix(non_prefixed, "prefix")
    lowkey still_same == "Hello" {
        vibez.spill("trim_prefix: Correctly handled non-existent prefix")
    } highkey {
        panic("trim_prefix: Incorrectly modified string without prefix. Got: '" + still_same + "'")
    }
    
    fr fr Test trim_suffix
    tea suffixed := "HelloSuffix"
    tea without_suffix := stringz.trim_suffix(suffixed, "Suffix")
    lowkey without_suffix == "Hello" {
        vibez.spill("trim_suffix: Correctly removed suffix")
    } highkey {
        panic("trim_suffix: Failed to remove suffix. Got: '" + without_suffix + "'")
    }
    
    fr fr Test when suffix doesn't exist
    tea non_suffixed := "Hello"
    tea still_same2 := stringz.trim_suffix(non_suffixed, "Suffix")
    lowkey still_same2 == "Hello" {
        vibez.spill("trim_suffix: Correctly handled non-existent suffix")
    } highkey {
        panic("trim_suffix: Incorrectly modified string without suffix. Got: '" + still_same2 + "'")
    }
    
    vibez.spill("String trimming functions passed")
}

slay test_string_replacement() {
    vibez.spill("Testing string replacement functions")
    
    fr fr Test replace (with default count = 1)
    tea original := "hello hello hello"
    tea replaced := stringz.replace(original, "hello", "bye")
    lowkey replaced == "bye hello hello" {
        vibez.spill("replace: Correctly replaced first occurrence")
    } highkey {
        panic("replace: Failed to replace first occurrence correctly. Got: '" + replaced + "'")
    }
    
    fr fr Test replace with count
    tea replaced2 := stringz.replace(original, "hello", "bye", 2)
    lowkey replaced2 == "bye bye hello" {
        vibez.spill("replace: Correctly replaced first 2 occurrences")
    } highkey {
        panic("replace: Failed to replace first 2 occurrences correctly. Got: '" + replaced2 + "'")
    }
    
    fr fr Test replace_all
    tea replaced_all := stringz.replace_all(original, "hello", "bye")
    lowkey replaced_all == "bye bye bye" {
        vibez.spill("replace_all: Correctly replaced all occurrences")
    } highkey {
        panic("replace_all: Failed to replace all occurrences correctly. Got: '" + replaced_all + "'")
    }
    
    vibez.spill("String replacement functions passed")
}

slay test_string_repetition() {
    vibez.spill("Testing string repetition function")
    
    fr fr Test repeat
    tea base := "abc"
    tea repeated := stringz.repeat(base, 3)
    lowkey repeated == "abcabcabc" {
        vibez.spill("repeat: Correctly repeated string")
    } highkey {
        panic("repeat: Failed to repeat string correctly. Got: '" + repeated + "'")
    }
    
    fr fr Test repeat with count = 0
    tea empty := stringz.repeat(base, 0)
    lowkey empty == "" {
        vibez.spill("repeat: Correctly handled zero repetitions")
    } highkey {
        panic("repeat: Failed to handle zero repetitions correctly. Got: '" + empty + "'")
    }
    
    vibez.spill("String repetition function passed")
}