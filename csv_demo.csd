yeet "csv"
yeet "string"

slay main() {
    vibez.spill("CSV Module Demo")
    
    # Test basic CSV parsing
    sus csv_data tea = "name,age,city\nJohn,25,NYC\nJane,30,LA"
    vibez.spill("Original CSV:")
    vibez.spill(csv_data)
    
    sus parsed array = csv.parse(csv_data)
    vibez.spill("Parsed data:")
    vibez.spill("Number of rows: " + string.from_int(string.len(parsed)))
    
    # Test delimiter detection
    sus semicolon_csv tea = "name;age;city\nJohn;25;NYC"
    sus detected tea = csv.detect_delimiter(semicolon_csv)
    vibez.spill("Detected delimiter: " + detected)
    
    # Test round-trip
    sus stringified tea = csv.stringify(parsed)
    vibez.spill("Round-trip result:")
    vibez.spill(stringified)
    
    # Test validation
    sus valid lit = csv.validate(csv_data)
    vibez.spill("CSV is valid: " + string.from_bool(valid))
    
    vibez.spill("CSV module working correctly!")
}

main()
