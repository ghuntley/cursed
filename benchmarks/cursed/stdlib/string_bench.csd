fr fr String Operations Performance Benchmark Suite

yeet "benchz"
yeet "testz"
yeet "stringz"

slay benchmark_basic_string_operations() lit {
    benchmark_suite_start("Basic String Operations")
    
    fr fr String creation and assignment
    benchmark_precise("String Literal Assignment", slay() {
        sus text tea = "Hello, world!"
    })
    
    benchmark_precise("Empty String Assignment", slay() {
        sus text tea = ""
    })
    
    benchmark_precise("Long String Assignment", slay() {
        sus text tea = "This is a much longer string that contains more characters and should test the performance of string allocation and assignment with larger data."
    })
    
    fr fr String length operations
    benchmark_precise("String Length Short", slay() {
        sus text tea = "Hello"
        sus length normie = len_str(text)
    })
    
    benchmark_precise("String Length Medium", slay() {
        sus text tea = "This is a medium length string for testing"
        sus length normie = len_str(text)
    })
    
    benchmark_precise("String Length Long", slay() {
        sus text tea = "This is a very long string that contains many characters and should test the performance of string length calculation with larger strings that might require more processing time."
        sus length normie = len_str(text)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_string_concatenation() lit {
    benchmark_suite_start("String Concatenation")
    
    fr fr Simple concatenation
    benchmark_precise("Two String Concat", slay() {
        sus a tea = "Hello"
        sus b tea = "World"
        sus result tea = concat_str(a, b)
    })
    
    benchmark_precise("Three String Concat", slay() {
        sus a tea = "Hello"
        sus b tea = " "
        sus c tea = "World"
        sus result tea = concat_str(concat_str(a, b), c)
    })
    
    benchmark_precise("String with Space Concat", slay() {
        sus greeting tea = "Hello"
        sus space tea = " "
        sus name tea = "CURSED"
        sus result tea = concat_str(concat_str(greeting, space), name)
    })
    
    fr fr Multiple concatenations
    benchmark_precise("Multiple Small Concat", slay() {
        sus result tea = ""
        sus i normie = 0
        bestie (i < 10) {
            result = concat_str(result, "x")
            i = i + 1
        }
    })
    
    benchmark_precise("Multiple Medium Concat", slay() {
        sus result tea = ""
        sus i normie = 0
        bestie (i < 5) {
            result = concat_str(result, "test")
            i = i + 1
        }
    })
    
    fr fr Large string concatenation
    benchmark_precise("Large String Concat", slay() {
        sus large1 tea = "This is the first large string that contains quite a bit of text"
        sus large2 tea = "This is the second large string that also contains significant content"
        sus result tea = concat_str(large1, large2)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_string_comparison() lit {
    benchmark_suite_start("String Comparison")
    
    fr fr Equality comparisons
    benchmark_precise("Equal Strings Short", slay() {
        sus a tea = "hello"
        sus b tea = "hello"
        sus result lit = equals_str(a, b)
    })
    
    benchmark_precise("Equal Strings Long", slay() {
        sus a tea = "This is a longer string for equality testing"
        sus b tea = "This is a longer string for equality testing"
        sus result lit = equals_str(a, b)
    })
    
    benchmark_precise("Unequal Strings", slay() {
        sus a tea = "hello"
        sus b tea = "world"
        sus result lit = equals_str(a, b)
    })
    
    benchmark_precise("Unequal Length Strings", slay() {
        sus a tea = "short"
        sus b tea = "much longer string"
        sus result lit = equals_str(a, b)
    })
    
    fr fr String ordering
    benchmark_precise("String Compare Less", slay() {
        sus a tea = "apple"
        sus b tea = "banana"
        sus result normie = compare_str(a, b)
    })
    
    benchmark_precise("String Compare Greater", slay() {
        sus a tea = "zebra"
        sus b tea = "apple"
        sus result normie = compare_str(a, b)
    })
    
    benchmark_precise("String Compare Equal", slay() {
        sus a tea = "same"
        sus b tea = "same"
        sus result normie = compare_str(a, b)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_string_searching() lit {
    benchmark_suite_start("String Searching")
    
    fr fr Contains operations
    benchmark_precise("Contains Found Short", slay() {
        sus haystack tea = "hello world"
        sus needle tea = "world"
        sus result lit = contains_str(haystack, needle)
    })
    
    benchmark_precise("Contains Found Long", slay() {
        sus haystack tea = "This is a much longer string that contains the word target somewhere in the middle of all this text"
        sus needle tea = "target"
        sus result lit = contains_str(haystack, needle)
    })
    
    benchmark_precise("Contains Not Found", slay() {
        sus haystack tea = "hello world"
        sus needle tea = "xyz"
        sus result lit = contains_str(haystack, needle)
    })
    
    benchmark_precise("Contains Single Char", slay() {
        sus haystack tea = "hello world"
        sus needle tea = "o"
        sus result lit = contains_str(haystack, needle)
    })
    
    fr fr Index operations
    benchmark_precise("Index Of Found", slay() {
        sus haystack tea = "hello world"
        sus needle tea = "world"
        sus result normie = index_of_str(haystack, needle)
    })
    
    benchmark_precise("Index Of Not Found", slay() {
        sus haystack tea = "hello world"
        sus needle tea = "xyz"
        sus result normie = index_of_str(haystack, needle)
    })
    
    benchmark_precise("Index Of First Occurrence", slay() {
        sus haystack tea = "hello hello world"
        sus needle tea = "hello"
        sus result normie = index_of_str(haystack, needle)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_string_manipulation() lit {
    benchmark_suite_start("String Manipulation")
    
    fr fr Case conversion
    benchmark_precise("To Uppercase Short", slay() {
        sus text tea = "hello"
        sus result tea = to_upper_str(text)
    })
    
    benchmark_precise("To Uppercase Long", slay() {
        sus text tea = "this is a longer string with mixed case letters that need conversion"
        sus result tea = to_upper_str(text)
    })
    
    benchmark_precise("To Lowercase Short", slay() {
        sus text tea = "HELLO"
        sus result tea = to_lower_str(text)
    })
    
    benchmark_precise("To Lowercase Long", slay() {
        sus text tea = "THIS IS A LONGER STRING WITH MIXED CASE LETTERS THAT NEED CONVERSION"
        sus result tea = to_lower_str(text)
    })
    
    fr fr Substring operations
    benchmark_precise("Substring Beginning", slay() {
        sus text tea = "hello world"
        sus result tea = substring_str(text, 0, 5)
    })
    
    benchmark_precise("Substring Middle", slay() {
        sus text tea = "hello world"
        sus result tea = substring_str(text, 3, 8)
    })
    
    benchmark_precise("Substring End", slay() {
        sus text tea = "hello world"
        sus result tea = substring_str(text, 6, 11)
    })
    
    fr fr Trimming operations
    benchmark_precise("Trim Whitespace", slay() {
        sus text tea = "  hello world  "
        sus result tea = trim_str(text)
    })
    
    benchmark_precise("Trim No Whitespace", slay() {
        sus text tea = "hello world"
        sus result tea = trim_str(text)
    })
    
    benchmark_precise("Trim Left", slay() {
        sus text tea = "  hello world"
        sus result tea = trim_left_str(text)
    })
    
    benchmark_precise("Trim Right", slay() {
        sus text tea = "hello world  "
        sus result tea = trim_right_str(text)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_string_splitting() lit {
    benchmark_suite_start("String Splitting and Joining")
    
    fr fr String splitting
    benchmark_precise("Split on Space", slay() {
        sus text tea = "hello world test string"
        sus result []tea = split_str(text, " ")
    })
    
    benchmark_precise("Split on Comma", slay() {
        sus text tea = "apple,banana,cherry,date,elderberry"
        sus result []tea = split_str(text, ",")
    })
    
    benchmark_precise("Split Single Character", slay() {
        sus text tea = "a-b-c-d-e-f-g"
        sus result []tea = split_str(text, "-")
    })
    
    benchmark_precise("Split No Delimiter", slay() {
        sus text tea = "no delimiter here"
        sus result []tea = split_str(text, ",")
    })
    
    fr fr String joining
    benchmark_precise("Join with Space", slay() {
        sus parts []tea = ["hello", "world", "test"]
        sus result tea = join_str(parts, " ")
    })
    
    benchmark_precise("Join with Comma", slay() {
        sus parts []tea = ["apple", "banana", "cherry"]
        sus result tea = join_str(parts, ",")
    })
    
    benchmark_precise("Join Empty Delimiter", slay() {
        sus parts []tea = ["a", "b", "c", "d"]
        sus result tea = join_str(parts, "")
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_string_formatting() lit {
    benchmark_suite_start("String Formatting")
    
    fr fr Basic formatting
    benchmark_precise("Format with Integer", slay() {
        sus template tea = "The value is {}"
        sus value normie = 42
        sus result tea = format_str(template, value)
    })
    
    benchmark_precise("Format with String", slay() {
        sus template tea = "Hello, {}!"
        sus name tea = "CURSED"
        sus result tea = format_str(template, name)
    })
    
    benchmark_precise("Format Multiple Args", slay() {
        sus template tea = "{} + {} = {}"
        sus a normie = 10
        sus b normie = 20
        sus sum normie = 30
        sus result tea = format_str(template, a, b, sum)
    })
    
    fr fr Complex formatting
    benchmark_precise("Format Complex Template", slay() {
        sus template tea = "User {} has {} points and {} achievements"
        sus user tea = "player1"
        sus points normie = 1500
        sus achievements normie = 23
        sus result tea = format_str(template, user, points, achievements)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_string_regex() lit {
    benchmark_suite_start("String Regular Expressions")
    
    fr fr Basic regex matching
    benchmark_precise("Regex Match Simple", slay() {
        sus text tea = "hello123"
        sus pattern tea = "[a-z]+"
        sus result lit = regex_match_str(text, pattern)
    })
    
    benchmark_precise("Regex Match Complex", slay() {
        sus text tea = "user@example.com"
        sus pattern tea = "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"
        sus result lit = regex_match_str(text, pattern)
    })
    
    benchmark_precise("Regex No Match", slay() {
        sus text tea = "hello world"
        sus pattern tea = "\d+"
        sus result lit = regex_match_str(text, pattern)
    })
    
    fr fr Regex replacement
    benchmark_precise("Regex Replace Simple", slay() {
        sus text tea = "hello world"
        sus pattern tea = "world"
        sus replacement tea = "CURSED"
        sus result tea = regex_replace_str(text, pattern, replacement)
    })
    
    benchmark_precise("Regex Replace Multiple", slay() {
        sus text tea = "The quick brown fox jumps"
        sus pattern tea = "[aeiou]"
        sus replacement tea = "*"
        sus result tea = regex_replace_str(text, pattern, replacement)
    })
    
    generate_benchmark_report()
    damn based
}

slay run_all_string_benchmarks() lit {
    vibez.spill("🚀 Running All String Benchmarks")
    vibez.spill("═══════════════════════════════════")
    
    benchmark_basic_string_operations()
    benchmark_string_concatenation()
    benchmark_string_comparison()
    benchmark_string_searching()
    benchmark_string_manipulation()
    benchmark_string_splitting()
    benchmark_string_formatting()
    benchmark_string_regex()
    
    vibez.spill("\n✅ All string benchmarks completed!")
    
    fr fr Performance analysis
    compare_benchmarks("String Length Short", "String Length Long")
    compare_benchmarks("Two String Concat", "Large String Concat")
    compare_benchmarks("Equal Strings Short", "Equal Strings Long")
    compare_benchmarks("Contains Found Short", "Contains Found Long")
    
    damn based
}

fr fr Run benchmarks if this file is executed directly
run_all_string_benchmarks()
