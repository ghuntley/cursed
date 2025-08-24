# CURSED scanz Module Demo
# Demonstrates scanner, CSV parsing, and table formatting functionality

yeet "stdlib/scanz/scanz"

slay main() {
    vibez.spill("=== CURSED Scanner and TabWriter Demo ===")
    vibez.spill("")
    
    # Demonstrate basic text scanning
    vibez.spill("1. Basic Text Scanning")
    vibez.spill("-----------------------")
    
    sus sample_text tea = "apple,banana|grape;orange:lemon"
    sus delims []tea = [",", "|", ";", ":"]
    sus tokens []tea = scan_tokens(sample_text, delims)
    
    vibez.spill("Sample text:", sample_text)
    vibez.spill("Delimiters: [',', '|', ';', ':']")
    vibez.spill("Tokens found:")
    bestie (sus i drip = 0; i < tokens.length; i += 1) {
        vibez.spill("  ", i + 1, ":", tokens[i])
    }
    vibez.spill("")
    
    # Demonstrate line scanning
    vibez.spill("2. Line-by-Line Scanning")
    vibez.spill("-------------------------")
    
    sus multi_line_text tea = "word1 word2 word3\nitem1 item2 item3 item4\ndata1 data2"
    sus line_data [][]tea = scan_lines(multi_line_text)
    
    vibez.spill("Multi-line text:")
    vibez.spill(multi_line_text)
    vibez.spill("Parsed lines:")
    bestie (sus i drip = 0; i < line_data.length; i += 1) {
        vibez.spill("  Line", i + 1, ":")
        bestie (sus j drip = 0; j < line_data[i].length; j += 1) {
            vibez.spill("    [", j, "]", line_data[i][j])
        }
    }
    vibez.spill("")
    
    # Demonstrate CSV parsing
    vibez.spill("3. CSV Parsing")
    vibez.spill("--------------")
    
    sus csv_data tea = "Name,Age,City,Country\nJohn Doe,25,New York,USA\nJane Smith,30,Los Angeles,USA\nBob Johnson,35,Chicago,USA"
    sus csv_records [][]tea = parse_csv(csv_data)
    
    vibez.spill("CSV Data:")
    vibez.spill(csv_data)
    vibez.spill("Parsed records:")
    bestie (sus i drip = 0; i < csv_records.length; i += 1) {
        vibez.spill("  Record", i, ":")
        bestie (sus j drip = 0; j < csv_records[i].length; j += 1) {
            vibez.spill("    [", j, "]", csv_records[i][j])
        }
    }
    vibez.spill("")
    
    # Demonstrate table formatting
    vibez.spill("4. Table Formatting")
    vibez.spill("-------------------")
    
    sus headers []tea = ["Product", "Price", "Stock", "Category"]
    sus table_data [][]tea = [
        ["Laptop", "$999", "15", "Electronics"],
        ["Mouse", "$25", "100", "Accessories"],
        ["Keyboard", "$75", "50", "Accessories"],
        ["Monitor", "$299", "25", "Electronics"]
    ]
    
    sus simple_table tea = format_table(table_data, headers)
    vibez.spill("Simple Table:")
    vibez.spill(simple_table)
    
    sus bordered_table tea = format_table_with_border(table_data, headers)
    vibez.spill("Bordered Table:")
    vibez.spill(bordered_table)
    
    # Demonstrate word and token statistics
    vibez.spill("5. Text Analysis")
    vibez.spill("----------------")
    
    sus analysis_text tea = "The quick brown fox jumps over the lazy dog"
    sus words []tea = split_words(analysis_text)
    sus stats []drip = get_token_stats(analysis_text, [" "])
    
    vibez.spill("Text:", analysis_text)
    vibez.spill("Word count:", words.length)
    vibez.spill("Token statistics:")
    vibez.spill("  Total tokens:", stats[0])
    vibez.spill("  Average length:", stats[1])
    vibez.spill("  Max length:", stats[2])
    vibez.spill("  Min length:", stats[3])
    
    sus longest tea = find_longest_token(analysis_text, [" "])
    vibez.spill("  Longest token:", longest)
    vibez.spill("")
    
    vibez.spill("6. Advanced CSV with Quotes")
    vibez.spill("---------------------------")
    
    sus quoted_csv tea = "Name,Description,Notes\n\"Smith, John\",\"Software Engineer, Senior\",\"Likes coffee, works late\"\n\"Johnson, Bob\",\"Designer\",\"Creative type\""
    sus quoted_records [][]tea = parse_csv(quoted_csv)
    
    vibez.spill("Quoted CSV:")
    vibez.spill(quoted_csv)
    vibez.spill("Parsed records:")
    bestie (sus i drip = 0; i < quoted_records.length; i += 1) {
        vibez.spill("  Record", i, ":")
        bestie (sus j drip = 0; j < quoted_records[i].length; j += 1) {
            vibez.spill("    Field", j, ":", quoted_records[i][j])
        }
    }
    vibez.spill("")
    
    vibez.spill("=== Demo Complete ===")
}

main()
