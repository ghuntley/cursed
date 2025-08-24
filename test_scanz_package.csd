// Test scanner/tabwriter (scanz package)
yeet "scanz"
yeet "vibez"

vibez.spill("=== Testing Scanz Scanner/Tabwriter Package ===")

// Test text scanner
sus input_text tea = "word1 word2 word3\nline2 data1 data2\nline3 value1 value2"
sus scanner Scanner = create_scanner(input_text)

sus tokens []tea = []
bestie (has_next_token(scanner)) {
    sus token tea = next_token(scanner)
    append(tokens, token)
}

vibez.spill("Scanned tokens:")
bestie (sus i drip = 0; i < len(tokens); i++) {
    vibez.spill("-", tokens[i])
}

ready (len(tokens) >= 6) {
    vibez.spill("✅ Text scanning: PASSED")
} otherwise {
    vibez.spill("❌ Text scanning: FAILED")
}

// Test line scanner
sus line_scanner LineScanner = create_line_scanner(input_text)
sus lines []tea = []

bestie (has_next_line(line_scanner)) {
    sus line tea = next_line(line_scanner)
    append(lines, line)
}

ready (len(lines) == 3) {
    vibez.spill("✅ Line scanning: PASSED")
} otherwise {
    vibez.spill("❌ Line scanning: FAILED")
}

// Test tabwriter
sus tab_writer TabWriter = create_tab_writer(4, 2, 1, ' ', 0)

write_tab(tab_writer, "Name\tAge\tCity\n")
write_tab(tab_writer, "Alice\t30\tNew York\n")
write_tab(tab_writer, "Bob\t25\tLos Angeles\n")
write_tab(tab_writer, "Charlie\t35\tChicago\n")

sus formatted_output tea = flush_tab_writer(tab_writer)
vibez.spill("Formatted table output:")
vibez.spill(formatted_output)

ready (contains(formatted_output, "Name") && contains(formatted_output, "Alice")) {
    vibez.spill("✅ Tab writer formatting: PASSED")
} otherwise {
    vibez.spill("❌ Tab writer formatting: FAILED")
}

// Test CSV scanner
sus csv_data tea = "name,age,city\nAlice,30,\"New York\"\nBob,25,\"Los Angeles\"\nCharlie,35,Chicago"
sus csv_scanner CSVScanner = create_csv_scanner(csv_data)

sus csv_records [][]tea = []
bestie (has_next_record(csv_scanner)) {
    sus record []tea = next_record(csv_scanner)
    append(csv_records, record)
}

vibez.spill("CSV records parsed:")
bestie (sus i drip = 0; i < len(csv_records); i++) {
    vibez.spill("Record", i, ":", csv_records[i])
}

ready (len(csv_records) == 4) {  // Header + 3 data rows
    vibez.spill("✅ CSV scanning: PASSED")
} otherwise {
    vibez.spill("❌ CSV scanning: FAILED")
}

vibez.spill("=== Scanz Testing Complete ===")
