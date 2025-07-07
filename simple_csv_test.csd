yeet "csv"

slay main() {
    vibez.spill("Testing CSV module")
    
    sus csv_data tea = "name,age\nJohn,25\nJane,30"
    vibez.spill("CSV data:")
    vibez.spill(csv_data)
    
    sus result array = csv.parse(csv_data)
    vibez.spill("Parse successful")
    
    sus count normie = csv.count_rows(csv_data)
    vibez.spill("Row count:")
    vibez.spill(count)
    
    sus delimiter tea = csv.detect_delimiter(csv_data)
    vibez.spill("Delimiter detected:")
    vibez.spill(delimiter)
}

main()
