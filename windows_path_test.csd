// Test Windows-specific path handling
vibez.spill("Testing Windows path handling...")

// Test path separators
sus windows_path tea = "C:\\Users\\user\\documents\\file.txt"
sus unix_path tea = "/home/user/documents/file.txt"

vibez.spill("Windows path: " + windows_path)
vibez.spill("Unix path: " + unix_path)

// Test basic operations that might be platform-specific
sus current_time drip = 42
vibez.spill("Current timestamp: " + tea(current_time))

vibez.spill("Windows path test complete!")
