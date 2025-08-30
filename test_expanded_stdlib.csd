fr fr Test expanded CURSED stdlib modules
yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "fmt"
yeet "time"
yeet "fs"
yeet "io"

slay test_fs_module() {
    vibez.spillln("=== Testing fs module ===")
    
    fr fr Test file existence
    sus exists lit = fs.file_exists("test.txt")
    vibez.spillln(fmt.format_bool(exists))
    
    fr fr Test reading a file
    sus content tea = fs.read_file("test.txt")
    vibez.spillln("File content: " + content)
    
    fr fr Test file size
    sus size thicc = fs.get_file_size("test.txt")
    vibez.spillln("File size: " + fmt.format_int(size))
    
    fr fr Test directory operations
    sus is_directory lit = fs.is_dir("test_dir")
    vibez.spillln("Is directory: " + fmt.format_bool(is_directory))
    
    fr fr Test writing a file
    sus write_success lit = fs.write_file("output.txt", "Hello from CURSED!")
    vibez.spillln("Write success: " + fmt.format_bool(write_success))
}

slay test_io_module() {
    vibez.spillln("=== Testing io module ===")
    
    fr fr Test print functions
    io.print("Testing io.print: ")
    io.println("Success!")
    
    fr fr Test read line
    sus input tea = io.read_line()
    vibez.spillln("Read line result: " + input)
}

slay test_all_modules() {
    vibez.spillln("=== Testing All Expanded Stdlib Modules ===")
    
    fr fr Test vibez
    vibez.spillln("vibez working: based")
    vibez.print_separator()
    
    fr fr Test mathz
    sus math_result meal = mathz.sqrt(16.0)
    vibez.spillln("mathz.sqrt(16) = " + fmt.format_float(math_result))
    
    fr fr Test stringz
    sus str_len thicc = stringz.length("Hello CURSED!")
    vibez.spillln("String length: " + fmt.format_int(str_len))
    
    fr fr Test fmt
    sus formatted tea = fmt.format_int(42)
    vibez.spillln("Formatted int: " + formatted)
    
    fr fr Test time
    sus current_time thicc = time.current_time_millis()
    vibez.spillln("Current time: " + fmt.format_int(current_time))
    
    fr fr Test fs
    test_fs_module()
    
    fr fr Test io
    test_io_module()
    
    vibez.spillln("=== All modules tested successfully! ===")
}

fr fr Main function
slay cursed_main() {
    test_all_modules()
}
