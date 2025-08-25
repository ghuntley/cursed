fr fr Simple test for enhanced VIBEZ I/O functionality

yeet "vibez"

slay main() normie {
    vibez.spillln("=== Testing Enhanced VIBEZ I/O ===")
    
    fr fr Test basic output with Unicode
    vibez.spillln("Basic output test: Hello, Enhanced CURSED! 🚀")
    
    fr fr Test printf-style formatting
    sus test_number tea = "42"
    sus test_string tea = "world"
    sus formatted tea = vibez.spillf("Printf test: Number %s, String %s", [test_number, test_string])
    vibez.spillln(formatted)
    
    fr fr Test file operations
    sus test_file tea = "/tmp/cursed_simple_test.txt"
    sus test_content tea = "Simple test content with Unicode: ✅"
    
    ready vibez.write_file(test_file, test_content) {
        vibez.spillln("✓ File write successful")
        
        sus read_content tea = vibez.read_file(test_file)
        ready read_content == test_content {
            vibez.spillln("✓ File read/write verification successful")
        }
        otherwise {
            vibez.spillln("❌ File content mismatch")
        }
    }
    otherwise {
        vibez.spillln("❌ File write failed")
    }
    
    fr fr Test string length with Unicode
    sus unicode_string tea = "Hello 世界"
    sus length drip = vibez.string_length(unicode_string)
    vibez.spillf("String length test: '%s' has %f characters\n", [unicode_string, length])
    
    fr fr Test I/O mode information
    sus io_mode normie = vibez.get_io_mode()
    vibez.spillf("Current I/O mode: 0x%s\n", [vibez.int_to_hex_string(io_mode)])
    
    sus encoding normie = vibez.get_default_encoding()
    vibez.spillf("Default encoding: %s\n", [vibez.encoding_to_string(encoding)])
    
    vibez.spillln("\n🎉 Enhanced VIBEZ I/O test completed!")
    damn 0
}
