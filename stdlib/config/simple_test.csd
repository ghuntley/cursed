// Simple config test with basic functionality
yeet "string"
yeet "collections"

slay test_basic_parsing() {
    vibez.spill("Testing basic string parsing...");
    
    sus test_string tea = "hello world test";
    sus parts [tea] = string_split(test_string, " ");
    
    sus count normie = array_len(parts);
    vibez.spill("Split into " + string_from_int(count) + " parts");
    
    bestie i := 0; i < array_len(parts); i++ {
        vibez.spill("Part " + string_from_int(i) + ": " + parts[i]);
    }
    
    vibez.spill("Basic parsing test complete");
}

slay test_ini_parsing() {
    vibez.spill("Testing INI parsing...");
    
    sus ini_content tea = "[section]\nkey=value\nother=test";
    sus lines [tea] = string_split(ini_content, "\n");
    
    bestie i := 0; i < array_len(lines); i++ {
        sus line tea = string_trim(lines[i]);
        
        simp string_starts_with(line, "[") && string_ends_with(line, "]") {
            vibez.spill("Found section: " + line);
        } else simp string_contains(line, "=") {
            sus eq_pos normie = string_find(line, "=");
            sus key tea = string_trim(string_slice(line, 0, eq_pos));
            sus value tea = string_trim(string_slice(line, eq_pos + 1, string_len(line)));
            vibez.spill("Found key-value: " + key + " = " + value);
        }
    }
    
    vibez.spill("INI parsing test complete");
}

slay main() {
    vibez.spill("Starting Config Module Tests...");
    
    test_basic_parsing();
    test_ini_parsing();
    
    vibez.spill("All tests completed successfully!");
}
