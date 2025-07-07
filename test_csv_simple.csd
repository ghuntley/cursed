yeet "csv"

slay main() {
    vibez.spill("Testing CSV module")
    
    sus csv_data tea = "name,age\nJohn,25\nJane,30"
    vibez.spill("CSV data:")
    vibez.spill(csv_data)
    
    sus delimiter tea = csv.detect_delimiter(csv_data)
    vibez.spill("Detected delimiter:")
    vibez.spill(delimiter)
    
    sus count normie = csv.count_rows(csv_data)
    vibez.spill("Row count:")
    vibez.spill(count)
    
    vibez.spill("CSV module test complete")
}

main()
