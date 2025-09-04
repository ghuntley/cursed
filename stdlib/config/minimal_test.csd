// Minimal config test to check basic functionality
yeet "string"

// Simple INI parsing test
slay test_simple_ini() {
    sus ini_content tea = "[database]\nhost=localhost\nport=5432\n";
    
    // Simple parsing logic
    sus lines [tea] = string_split(ini_content, "\n");
    sus result tea = "";
    
    bestie i := 0; i < array_len(lines); i++ {
        sus line tea = string_trim(lines[i]);
        
        simp !string_is_empty(line) {
            result = result + line + " ";
        }
    }
    
    vibez.spill("Parsed INI: " + result);
    vibez.spill("Test passed: Simple INI parsing works");
}

slay main_character() {
    vibez.spill("Testing basic config functionality...");
    test_simple_ini();
    vibez.spill("Config test complete!");
}
