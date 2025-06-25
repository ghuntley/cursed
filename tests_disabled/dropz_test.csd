vibe main

yeet "vibez"  fr fr For printing results
yeet "dropz"  fr fr I/O functions package
yeet "vibe_life" fr fr For file operations

slay main() {
    vibez.spill("Testing dropz package")
    
    fr fr Create a test file for I/O operations
    tea filename := "cursed_dropz_test.txt"
    tea testdata := "Testing the dropz package's I/O functionality"
    
    fr fr Clean up any previous test file
    lowkey vibe_life.Exists(filename) {
        vibe_life.Remove(filename)
    }
    
    fr fr Test write_file
    err := dropz.WriteFile(filename, testdata.bytes())
    lowkey err != cap {
        vibez.spill("Error writing file:", err)
    } highkey {
        vibez.spill("Successfully wrote test file")
    }
    
    fr fr Test read_file
    tea data, err := dropz.ReadFile(filename)
    lowkey err != cap {
        vibez.spill("Error reading file:", err)
    } highkey {
        vibez.spill("Read", data.len(), "bytes from file")
    }
    
    fr fr Test read_file_string
    tea content, err := dropz.ReadFileString(filename)
    lowkey err != cap {
        vibez.spill("Error reading file as string:", err)
    } highkey {
        vibez.spill("File content:", content)
    }
    
    fr fr Test file operations with a file object
    tea file, err := vibe_life.Open(filename)
    lowkey err != cap {
        vibez.spill("Error opening file:", err)
    } highkey {
        fr fr Create a buffer for reading
        tea buffer := make([]byte, 10)
        
        fr fr Read first 10 bytes
        tea bytesRead, err := dropz.Read(file, buffer)
        lowkey err != cap {
            vibez.spill("Error reading from file:", err)
        } highkey {
            vibez.spill("Read", bytesRead, "bytes:", string(buffer))
        }
        
        fr fr Seek back to beginning
        tea newPos, err := dropz.Seek(file, 0, dropz.SEEK_START)
        lowkey err != cap {
            vibez.spill("Error seeking in file:", err)
        } highkey {
            vibez.spill("Seeked to position:", newPos)
        }
    }
    
    fr fr Clean up - remove the test file
    vibe_life.Remove(filename)
    vibez.spill("Test file removed")
}