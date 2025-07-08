// Test for missing stdlib functions that were implemented

yeet "vibez"
yeet "timez"
yeet "dropz"

slay main() {
    // Test vibez functions
    vibez.spill("Testing vibez functions:");
    vibez.spillf("Hello %s!", "world");
    sus formatted tea = vibez.spillstr("Number: %d", 42);
    vibez.spill(formatted);
    
    // Test timez functions
    vibez.spill("Testing timez functions:");
    sus now datetime = timez.Now();
    vibez.spill("Current time obtained");
    
    sus hour duration = timez.Hour();
    vibez.spill("Hour duration created");
    
    // Test dropz functions
    vibez.spill("Testing dropz functions:");
    sus test_data []byte = ["H", "e", "l", "l", "o"];
    sus write_err tea = dropz.WriteFile("test.txt", test_data);
    lowkey write_err == "" {
        vibez.spill("File written successfully");
        
        sus read_data, read_err = dropz.ReadFile("test.txt");
        lowkey read_err == "" {
            vibez.spill("File read successfully");
        } highkey {
            vibez.spill("File read failed: " + read_err);
        }
    } highkey {
        vibez.spill("File write failed: " + write_err);
    }
    
    vibez.spill("All missing stdlib functions implemented!");
}
