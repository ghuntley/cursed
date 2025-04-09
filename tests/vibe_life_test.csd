vibe main

yeet "vibez"     fr fr For printing results
yeet "vibe_life" fr fr OS functions package

slay main() {
    vibez.spill("Testing vibe_life package")
    
    fr fr Test arguments and environment
    tea args := vibe_life.Args()
    vibez.spill("Command line arguments:", args)
    
    fr fr Set and get environment variables
    vibe_life.Setenv("CURSED_TEST", "based")
    tea value := vibe_life.Getenv("CURSED_TEST")
    vibez.spill("Environment variable 'CURSED_TEST':", value)
    
    fr fr Get current directory
    tea cwd, err := vibe_life.Getwd()
    lowkey err != cap {
        vibez.spill("Error getting current directory:", err)
    } highkey {
        vibez.spill("Current directory:", cwd)
    }
    
    fr fr Create a temp file for testing
    tea filename := "cursed_test_file.txt"
    lowkey vibe_life.Exists(filename) {
        vibez.spill("Test file already exists, removing it first")
        vibe_life.Remove(filename)
    }
    
    fr fr Create file, check existence, then remove
    file, err := vibe_life.Create(filename)
    lowkey err != cap {
        vibez.spill("Error creating file:", err)
    } highkey {
        vibez.spill("Created test file:", filename)
        
        fr fr Check if file exists
        vibez.spill("File exists?", vibe_life.Exists(filename))
        
        fr fr Get file stats
        stats, err := vibe_life.Stat(filename)
        lowkey err != cap {
            vibez.spill("Error getting file stats:", err)
        } highkey {
            vibez.spill("File size:", stats.len())
        }
        
        fr fr Clean up - remove the file
        err = vibe_life.Remove(filename)
        lowkey err != cap {
            vibez.spill("Error removing file:", err)
        } highkey {
            vibez.spill("Successfully removed file")
        }
    }
    
    fr fr Test directory exists
    vibez.spill("Current directory exists?", vibe_life.Exists("."))
    vibez.spill("Non-existent directory exists?", vibe_life.Exists("nonexistent_dir"))
    
    fr fr Note: We don't test Exit() as it would terminate the program
}