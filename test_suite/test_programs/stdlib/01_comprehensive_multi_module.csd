vibe main
yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "collections"

// Test comprehensive stdlib integration across multiple modules
slay process_numeric_data() {
    vibez.spill("=== Numeric Processing with Mathz ===")
    
    sus numbers []normie = []normie{-42, 17, 0, -3, 99, -156}
    sus processed_numbers []normie = []normie{}
    
    finna i normie = 0; i < len(numbers); i++ {
        sus num = numbers[i]
        sus abs_val = mathz.abs(num)
        sus squared = mathz.pow(abs_val, 2)
        
        processed_numbers = append(processed_numbers, squared)
        vibez.spill("Processed", num, "-> abs:", abs_val, "-> squared:", squared)
    }
    
    // Find min and max of processed numbers
    sus min_val = processed_numbers[0]
    sus max_val = processed_numbers[0]
    
    finna i normie = 1; i < len(processed_numbers); i++ {
        min_val = mathz.min(min_val, processed_numbers[i])
        max_val = mathz.max(max_val, processed_numbers[i])
    }
    
    vibez.spill("Min processed value:", min_val)
    vibez.spill("Max processed value:", max_val)
}

slay process_string_data() {
    vibez.spill("=== String Processing with Stringz ===")
    
    sus texts []tea = []tea{"Hello", "World", "CURSED", "Programming", "Language"}
    sus combined_text tea = ""
    
    finna i normie = 0; i < len(texts); i++ {
        sus text = texts[i]
        sus lower_text = stringz.to_lower(text)
        sus upper_text = stringz.to_upper(text) 
        sus length = stringz.length(text)
        
        vibez.spill("Text:", text, "->", lower_text, "->", upper_text, "(len:", length, ")")
        
        combined_text = stringz.concat(combined_text, lower_text)
        ready (i < len(texts) - 1) {
            combined_text = stringz.concat(combined_text, " ")
        }
    }
    
    vibez.spill("Combined text:", combined_text)
    vibez.spill("Final combined length:", stringz.length(combined_text))
}

slay process_collections_data() {
    vibez.spill("=== Collections Processing ===")
    
    sus data []normie = []normie{5, 2, 8, 1, 9, 3, 7, 4, 6}
    vibez.spill("Original data:", data)
    
    // Use collections operations (if available)
    vibez.spill("Data length:", len(data))
    
    // Manual sorting demonstration
    sus sorted_data = data
    finna i normie = 0; i < len(sorted_data); i++ {
        finna j normie = i + 1; j < len(sorted_data); j++ {
            ready (sorted_data[i] > sorted_data[j]) {
                sus temp = sorted_data[i]
                sorted_data[i] = sorted_data[j] 
                sorted_data[j] = temp
            }
        }
    }
    
    vibez.spill("Manually sorted:", sorted_data)
}

slay integrated_processing_pipeline() {
    vibez.spill("=== Integrated Processing Pipeline ===")
    
    // Create test data
    sus numbers []normie = []normie{-15, 23, -8, 42, -99, 17}
    sus names []tea = []tea{"alice", "BOB", "Charlie", "DIANA"}
    
    vibez.spill("Processing", len(numbers), "numbers and", len(names), "names")
    
    // Process numbers with mathz
    sus sum normie = 0
    finna i normie = 0; i < len(numbers); i++ {
        sus abs_num = mathz.abs(numbers[i])
        sum = sum + abs_num
    }
    
    // Process strings with stringz  
    sus processed_names []tea = []tea{}
    finna i normie = 0; i < len(names); i++ {
        sus name = names[i]
        sus normalized = stringz.to_lower(name)
        sus capitalized = stringz.to_upper(stringz.substring(normalized, 0, 1))
        sus rest = stringz.substring(normalized, 1, stringz.length(normalized))
        sus final_name = stringz.concat(capitalized, rest)
        
        processed_names = append(processed_names, final_name)
    }
    
    vibez.spill("Sum of absolute values:", sum)
    vibez.spill("Processed names:", processed_names)
    
    // Combine results
    finna i normie = 0; i < len(processed_names); i++ {
        sus name = processed_names[i]
        sus number_index = i % len(numbers)
        sus associated_number = mathz.abs(numbers[number_index])
        
        vibez.spill("Name:", name, "-> Number:", associated_number, "-> Square:", mathz.pow(associated_number, 2))
    }
}

slay main() {
    vibez.spill("=== Comprehensive Stdlib Integration Tests ===")
    
    process_numeric_data()
    process_string_data() 
    process_collections_data()
    integrated_processing_pipeline()
    
    vibez.spill("All stdlib integration tests completed successfully")
}
