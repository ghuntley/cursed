yeet "vibe_life"

vibez.spill("Testing vibe_life module...")

# Test Args function
args := vibe_life.Args()
vibez.spill("Command args: ", len(args))

# Test environment variables
home := vibe_life.Getenv("HOME")
vibez.spill("Home directory: ", home)

# Test working directory
wd, err := vibe_life.Getwd()
vibez.spill("Working directory: ", wd)

# Test file creation
file, err := vibe_life.Create("test.txt")
vibez.spill("Created file: ", file.name)

# Test system info
pid := vibe_life.Getpid()
vibez.spill("Process ID: ", pid)

vibez.spill("✅ vibe_life module test successful!")
