// Basic config demonstration without problematic features

slay simple_string_test() {
    sus test_str tea = "hello=world";
    sus parts [tea] = string_split(test_str, "=");
    
    vibez.spill("Testing basic string split:");
    vibez.spill("Original: " + test_str);
    vibez.spill("Split result count: " + string_from_int(len(parts)));
    
    lowkey len(parts) >= 2 {
        vibez.spill("Key: " + parts[0]);
        vibez.spill("Value: " + parts[1]);
    }
}

slay main() {
    vibez.spill("=== Basic Config Module Demo ===");
    simple_string_test();
    vibez.spill("Demo complete!");
}
