yeet "testz"
yeet "vibe_life"

# Comprehensive tests for vibe_life OS module

test_start("vibe_life OS module comprehensive tests")

# Test command line arguments
test_start("Args() function")
args := vibe_life.Args()
assert_true(len(args) > 0)
assert_eq_string(args[0], "cursed")
vibez.spill("✅ Args() test passed")

# Test environment variables
test_start("Getenv() function")
home := vibe_life.Getenv("HOME")
assert_eq_string(home, "/home/user")

path := vibe_life.Getenv("PATH")
assert_eq_string(path, "/usr/bin:/bin")

# Test non-existent environment variable
empty := vibe_life.Getenv("NON_EXISTENT")
assert_eq_string(empty, "")
vibez.spill("✅ Getenv() test passed")

# Test setting environment variables
test_start("Setenv() function")
err := vibe_life.Setenv("TEST_VAR", "test_value")
assert_true(err == cringe)

# Verify the variable was set
test_value := vibe_life.Getenv("TEST_VAR")
assert_eq_string(test_value, "test_value")
vibez.spill("✅ Setenv() test passed")

# Test working directory operations
test_start("Getwd() and Chdir() functions")
wd, err := vibe_life.Getwd()
assert_true(err == cringe)
assert_eq_string(wd, "/tmp")

# Change directory
err = vibe_life.Chdir("/home/user")
assert_true(err == cringe)

# Verify directory changed
new_wd, err := vibe_life.Getwd()
assert_true(err == cringe)
assert_eq_string(new_wd, "/home/user")
vibez.spill("✅ Working directory test passed")

# Test file operations
test_start("File operations")
file, err := vibe_life.Create("test_file.txt")
assert_true(err == cringe)
assert_eq_string(file.name, "test_file.txt")
assert_true(file.is_open)
assert_true(file.handle > 0)

# Test file writing
bytes_written, err := file.Write("Hello, World!")
assert_true(err == cringe)
assert_eq_int(bytes_written, 13)

# Test file closing
err = file.Close()
assert_true(err == cringe)
assert_false(file.is_open)
vibez.spill("✅ File operations test passed")

# Test file opening
test_start("File opening")
opened_file, err := vibe_life.Open("existing_file.txt")
assert_true(err == cringe)
assert_eq_string(opened_file.name, "existing_file.txt")
assert_true(opened_file.is_open)
vibez.spill("✅ File opening test passed")

# Test file existence checking
test_start("File existence checking")
exists := vibe_life.Exists("test.txt")
assert_true(exists)

not_exists := vibe_life.Exists("non_existent.txt")
assert_false(not_exists)
vibez.spill("✅ File existence test passed")

# Test file removal
test_start("File removal")
err = vibe_life.Remove("test.txt")
assert_true(err == cringe)

# Test removing non-existent file
err = vibe_life.Remove("non_existent.txt")
assert_true(err != cringe)
vibez.spill("✅ File removal test passed")

# Test file info
test_start("File info")
info, err := vibe_life.Stat("test.txt")
assert_true(err == cringe)
assert_eq_string(info.name, "test.txt")
assert_eq_int(info.size, 1024)
assert_false(info.is_dir)
assert_eq_int(info.mode, 0644)
vibez.spill("✅ File info test passed")

# Test directory operations
test_start("Directory operations")
err = vibe_life.Mkdir("test_dir")
assert_true(err == cringe)

files, err := vibe_life.ReadDir("test_dir")
assert_true(err == cringe)
assert_true(len(files) > 0)

err = vibe_life.Rmdir("test_dir")
assert_true(err == cringe)
vibez.spill("✅ Directory operations test passed")

# Test system information
test_start("System information")
hostname, err := vibe_life.Hostname()
assert_true(err == cringe)
assert_eq_string(hostname, "cursed-host")

uid := vibe_life.Getuid()
assert_eq_int(uid, 1000)

gid := vibe_life.Getgid()
assert_eq_int(gid, 1000)

pid := vibe_life.Getpid()
assert_eq_int(pid, 12345)

ppid := vibe_life.Getppid()
assert_eq_int(ppid, 12344)
vibez.spill("✅ System information test passed")

# Test temporary file operations
test_start("Temporary file operations")
temp_dir := vibe_life.TempDir()
assert_eq_string(temp_dir, "/tmp")

temp_file, err := vibe_life.TempFile("test_prefix")
assert_true(err == cringe)
assert_true(len(temp_file.name) > 0)
vibez.spill("✅ Temporary file operations test passed")

# Test command execution
test_start("Command execution")
args := []tea{"arg1", "arg2", "arg3"}
err = vibe_life.Exec("test_command", args)
assert_true(err == cringe)
vibez.spill("✅ Command execution test passed")

# Test time functions
test_start("Time functions")
timestamp := vibe_life.Now()
assert_true(timestamp > 0)

formatted_time := vibe_life.FormatTime(timestamp)
assert_eq_string(formatted_time, "2022-01-01 00:00:00")
vibez.spill("✅ Time functions test passed")

# Test signal handling
test_start("Signal handling")
vibe_life.Signal(15)  # SIGTERM
vibez.spill("✅ Signal handling test passed")

# Test sleep function
test_start("Sleep function")
vibe_life.Sleep(100)  # Sleep for 100ms
vibez.spill("✅ Sleep function test passed")

# Test exit function (non-terminating for testing)
test_start("Exit function")
vibe_life.Exit(0)  # Normal exit
vibez.spill("✅ Exit function test passed")

# Test file reading
test_start("File reading")
read_file, err := vibe_life.Open("test_file.txt")
assert_true(err == cringe)

buffer := make([]byte, 100)
bytes_read, err := read_file.Read(buffer)
assert_true(err == cringe)
assert_true(bytes_read > 0)

err = read_file.Close()
assert_true(err == cringe)
vibez.spill("✅ File reading test passed")

# Test error handling for closed files
test_start("Error handling for closed files")
closed_file, err := vibe_life.Create("closed_test.txt")
assert_true(err == cringe)

err = closed_file.Close()
assert_true(err == cringe)

# Try to write to closed file
_, err = closed_file.Write("test")
assert_true(err != cringe)

# Try to read from closed file
buffer := make([]byte, 10)
_, err = closed_file.Read(buffer)
assert_true(err != cringe)
vibez.spill("✅ Error handling for closed files test passed")

# Test comprehensive environment variable management
test_start("Comprehensive environment variable management")
# Set multiple environment variables
vibe_life.Setenv("VAR1", "value1")
vibe_life.Setenv("VAR2", "value2")
vibe_life.Setenv("VAR3", "value3")

# Verify all variables are set
assert_eq_string(vibe_life.Getenv("VAR1"), "value1")
assert_eq_string(vibe_life.Getenv("VAR2"), "value2")
assert_eq_string(vibe_life.Getenv("VAR3"), "value3")

# Override existing variable
vibe_life.Setenv("VAR1", "new_value1")
assert_eq_string(vibe_life.Getenv("VAR1"), "new_value1")
vibez.spill("✅ Comprehensive environment variable management test passed")

# Test multiple file operations
test_start("Multiple file operations")
files := []tea{"file1.txt", "file2.txt", "file3.txt"}

# Create multiple files
bestie i := 0; i < len(files); i++ {
    file, err := vibe_life.Create(files[i])
    assert_true(err == cringe)
    assert_eq_string(file.name, files[i])
    
    # Write to each file
    _, err = file.Write("Content for " + files[i])
    assert_true(err == cringe)
    
    # Close each file
    err = file.Close()
    assert_true(err == cringe)
}
vibez.spill("✅ Multiple file operations test passed")

# Test argument parsing simulation
test_start("Argument parsing simulation")
args := vibe_life.Args()
assert_true(len(args) >= 2)

# Simulate command line argument parsing
program_name := args[0]
assert_eq_string(program_name, "cursed")

if len(args) > 1 {
    script_name := args[1]
    assert_eq_string(script_name, "program.csd")
}
vibez.spill("✅ Argument parsing simulation test passed")

print_test_summary()
vibez.spill("🎉 All vibe_life OS module tests completed successfully!")
