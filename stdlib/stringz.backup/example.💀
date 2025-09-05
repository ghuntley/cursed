fr fr StringZ Core Module Usage Example
fr fr Demonstrates the four core categories of string functionality

yeet "stringz"

vibez.spill("=== StringZ Core Module Demo ===")

fr fr ===== STRING MANIPULATION EXAMPLES =====

vibez.spill("\n📝 STRING MANIPULATION:")

fr fr Split and rejoin CSV data
sus csv_data tea = "Alice,25,Engineer"
sus fields tea[value] = split(csv_data, ",")
vibez.spill("Original CSV:", csv_data)
vibez.spill("Split into", len(fields), "fields:", fields[0], fields[1], fields[2])

sus rejoined tea = join(fields, " | ")
vibez.spill("Rejoined with pipes:", rejoined)

fr fr String replacement 
sus message tea = "Hello world, welcome to world"
sus updated tea = replace_all(message, "world", "CURSED")
vibez.spill("Original:", message)
vibez.spill("Replaced:", updated)

fr fr String reversal
sus original tea = "programming"
sus backwards tea = reverse(original)
vibez.spill("Forward:", original)
vibez.spill("Backward:", backwards)

fr fr ===== STRING FORMATTING EXAMPLES =====

vibez.spill("\n🎨 STRING FORMATTING:")

fr fr Template formatting
sus user_template tea = "User: {}, Age: {}, Role: {}"
sus user_info tea = format_template(user_template, ["Alice", "25", "Engineer"])
vibez.spill("Formatted user info:", user_info)

fr fr String padding
sus number tea = "42"
sus padded_number tea = pad_left(number, 6, "0")
sus title tea = "REPORT"
sus centered_title tea = center(title, 20, "=")

vibez.spill("Padded number:", padded_number)
vibez.spill("Centered title:", centered_title)

fr fr Character repetition
sus separator tea = repeat_char("-", 40)
vibez.spill("Separator:", separator)

fr fr ===== STRING PARSING EXAMPLES =====

vibez.spill("\n🔢 STRING PARSING:")

fr fr Parse different data types
sus age_str tea = "25"
sus is_active_str tea = "true"
sus score_str tea = "invalid"

sus age drip = parse_int(age_str)
sus is_active lit = parse_bool(is_active_str)
sus score drip = parse_int(score_str)  fr fr Will default to 0

vibez.spill("Age string:", age_str, "→ number:", age)
vibez.spill("Active string:", is_active_str, "→ boolean:", is_active)
vibez.spill("Score string:", score_str, "→ number (defaulted):", score)

fr fr Convert back to strings
sus age_back tea = to_int(age)
sus active_back tea = to_string(is_active)

vibez.spill("Age back to string:", age_back)
vibez.spill("Active back to string:", active_back)

fr fr ===== STRING VALIDATION EXAMPLES =====

vibez.spill("\n✅ STRING VALIDATION:")

fr fr Test different string types
sus test_strings tea[value] = ["123", "abc", "abc123", "", "hello world"]

sus i drip = 0
bestie (i < len(test_strings)) {
    sus current tea = test_strings[i]
    sus empty_status tea = ""
    ready (is_empty(current)) {
        empty_status = "empty"
    } otherwise {
        empty_status = "not empty"
    }
    
    sus type_info tea = ""
    ready (is_numeric(current)) {
        type_info = "numeric"
    } otherwise ready (is_alpha(current)) {
        type_info = "alphabetic"
    } otherwise ready (is_alphanumeric(current)) {
        type_info = "alphanumeric"
    } otherwise {
        type_info = "mixed/special"
    }
    
    vibez.spill("String:", current, "→", empty_status, ",", type_info, ", length:", len_string(current))
    i = i + 1
}

fr fr ===== COMPLEX STRING PROCESSING PIPELINE =====

vibez.spill("\n🔄 COMPLEX PROCESSING PIPELINE:")

sus raw_input tea = "  Hello, World! Welcome to CURSED Programming.  "
vibez.spill("1. Raw input:", raw_input)

fr fr Step 1: Clean whitespace
sus cleaned tea = trim(raw_input)
vibez.spill("2. Trimmed:", cleaned)

fr fr Step 2: Convert to lowercase
sus lowercase tea = to_lowercase(cleaned)
vibez.spill("3. Lowercase:", lowercase)

fr fr Step 3: Split into words
sus words tea[value] = split(lowercase, " ")
vibez.spill("4. Split into", len(words), "words")

fr fr Step 4: Filter out punctuation (simplified)
sus clean_words tea[value] = []
sus j drip = 0
bestie (j < len(words)) {
    sus word tea = words[j]
    ready (!contains(word, ",") && !contains(word, ".") && !contains(word, "!")) {
        ready (len(clean_words) == 0) {
            clean_words = [word]
        } otherwise ready (len(clean_words) == 1) {
            clean_words = [clean_words[0], word]
        } otherwise ready (len(clean_words) == 2) {
            clean_words = [clean_words[0], clean_words[1], word]
        } otherwise {
            fr fr Keep first few words for demo
            clean_words = [clean_words[0], clean_words[1], clean_words[2]]
        }
    }
    j = j + 1
}

vibez.spill("5. Clean words:", len(clean_words), "words")

fr fr Step 5: Create final result
sus final_result tea = join(clean_words, "_")
sus final_upper tea = to_uppercase(final_result)

vibez.spill("6. Final result:", final_upper)

vibez.spill("\n🎉 StringZ demo completed successfully!")
vibez.spill("The StringZ module provides comprehensive string processing capabilities in pure CURSED!")
