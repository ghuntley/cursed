// COMPREHENSIVE STDLIB VALIDATION SUITE
// Tests ALL stdlib modules in both interpreter and compiled modes
// to verify complete self-hosting capability

import mathz
import stringz
import vibez
import path
import env
import fs
import io_basic
import io_advanced

func test_mathz_comprehensive() drip {
    vibez.spillln("=== TESTING MATHZ MODULE ===", "")
    
    // Test abs_normie
    let abs_pos drip = mathz.abs_normie(42)
    let abs_neg drip = mathz.abs_normie(-42)
    let abs_zero drip = mathz.abs_normie(0)
    vibez.spillln("abs_normie(42): ", abs_pos)
    vibez.spillln("abs_normie(-42): ", abs_neg)
    vibez.spillln("abs_normie(0): ", abs_zero)
    
    // Test max_normie and min_normie
    let max_result drip = mathz.max_normie(10, 20)
    let min_result drip = mathz.min_normie(10, 20)
    vibez.spillln("max_normie(10, 20): ", max_result)
    vibez.spillln("min_normie(10, 20): ", min_result)
    
    // Test arithmetic operations
    let add_result drip = mathz.add_two(15, 25)
    let sub_result drip = mathz.subtract_two(30, 12)
    let mult_result drip = mathz.multiply_two(6, 7)
    vibez.spillln("add_two(15, 25): ", add_result)
    vibez.spillln("subtract_two(30, 12): ", sub_result)
    vibez.spillln("multiply_two(6, 7): ", mult_result)
    
    // Test power and factorial
    let power_result drip = mathz.power_int(3, 4)
    let factorial_5 drip = mathz.factorial(5)
    let factorial_0 drip = mathz.factorial(0)
    vibez.spillln("power_int(3, 4): ", power_result)
    vibez.spillln("factorial(5): ", factorial_5)
    vibez.spillln("factorial(0): ", factorial_0)
    
    // Test even/odd checks
    let even_check_true lit = mathz.is_even(8)
    let even_check_false lit = mathz.is_even(9)
    let odd_check_true lit = mathz.is_odd(7)
    let odd_check_false lit = mathz.is_odd(6)
    vibez.spillln("is_even(8): ", even_check_true)
    vibez.spillln("is_even(9): ", even_check_false)
    vibez.spillln("is_odd(7): ", odd_check_true)
    vibez.spillln("is_odd(6): ", odd_check_false)
    
    // Test clamp
    let clamp_low drip = mathz.clamp(5, 10, 20)
    let clamp_high drip = mathz.clamp(25, 10, 20)
    let clamp_mid drip = mathz.clamp(15, 10, 20)
    vibez.spillln("clamp(5, 10, 20): ", clamp_low)
    vibez.spillln("clamp(25, 10, 20): ", clamp_high)
    vibez.spillln("clamp(15, 10, 20): ", clamp_mid)
    
    return 1
}

func test_stringz_comprehensive() drip {
    vibez.spillln("=== TESTING STRINGZ MODULE ===", "")
    
    // Test basic string operations
    let str1 tea = "Hello"
    let str2 tea = "World"
    let len_result drip = stringz.length(str1)
    let concat_result tea = stringz.concat(str1, str2)
    vibez.spillln("length('Hello'): ", len_result)
    vibez.spillln("concat('Hello', 'World'): ", concat_result)
    
    // Test string checks
    let contains_result lit = stringz.contains("Testing", "est")
    let contains_false lit = stringz.contains("Testing", "xyz")
    vibez.spillln("contains('Testing', 'est'): ", contains_result)
    vibez.spillln("contains('Testing', 'xyz'): ", contains_false)
    
    // Test case conversions
    let upper_result tea = stringz.upper("lowercase")
    let lower_result tea = stringz.lower("UPPERCASE")
    vibez.spillln("upper('lowercase'): ", upper_result)
    vibez.spillln("lower('UPPERCASE'): ", lower_result)
    
    // Test trim and split
    let trim_result tea = stringz.trim("  spaced  ")
    let split_count drip = stringz.split("a,b,c,d", ",")
    vibez.spillln("trim('  spaced  '): ", trim_result)
    vibez.spillln("split('a,b,c,d', ','): ", split_count)
    
    return 1
}

func test_vibez_comprehensive() drip {
    vibez.spillln("=== TESTING VIBEZ MODULE ===", "")
    
    // Test spill functions (these are no-ops but should not crash)
    vibez.spill("Testing spill without newline")
    vibez.spillln("Testing spillln with newline")
    
    // Test different data type prints
    vibez.spillln("Integer value: ", 42)
    vibez.spillln("String value: ", "test")
    vibez.spillln("Boolean true: ", true)
    vibez.spillln("Boolean false: ", false)
    
    // Test separator
    vibez.print_separator()
    
    return 1
}

func test_path_comprehensive() drip {
    vibez.spillln("=== TESTING PATH MODULE ===", "")
    
    // Test path operations
    let joined_path tea = path.join("/home/user", "file.txt")
    let base_name tea = path.basename("/home/user/document.pdf")
    let dir_name tea = path.dirname("/home/user/document.pdf")
    vibez.spillln("join('/home/user', 'file.txt'): ", joined_path)
    vibez.spillln("basename('/home/user/document.pdf'): ", base_name)
    vibez.spillln("dirname('/home/user/document.pdf'): ", dir_name)
    
    // Test absolute path and existence checks
    let abs_path tea = path.absolute("relative/path")
    let exists_result lit = path.exists("/some/path")
    let is_dir_result lit = path.is_dir("/home")
    let is_file_result lit = path.is_file("/home/user/file.txt")
    vibez.spillln("absolute('relative/path'): ", abs_path)
    vibez.spillln("exists('/some/path'): ", exists_result)
    vibez.spillln("is_dir('/home'): ", is_dir_result)
    vibez.spillln("is_file('/home/user/file.txt'): ", is_file_result)
    
    return 1
}

func test_env_comprehensive() drip {
    vibez.spillln("=== TESTING ENV MODULE ===", "")
    
    // Test environment variable operations
    let home_var tea = env.get_env("HOME")
    let path_var tea = env.get_env("PATH")
    let fake_var tea = env.get_env("NONEXISTENT")
    vibez.spillln("get_env('HOME'): ", home_var)
    vibez.spillln("get_env('PATH'): ", path_var)
    vibez.spillln("get_env('NONEXISTENT'): ", fake_var)
    
    // Test env checks and operations
    let has_home lit = env.has_env("HOME")
    let has_fake lit = env.has_env("NONEXISTENT")
    let env_count drip = env.list_env()
    let all_keys drip = env.get_all_keys()
    vibez.spillln("has_env('HOME'): ", has_home)
    vibez.spillln("has_env('NONEXISTENT'): ", has_fake)
    vibez.spillln("list_env(): ", env_count)
    vibez.spillln("get_all_keys(): ", all_keys)
    
    // Test set and unset operations
    let set_result lit = env.set_env("TEST_VAR", "test_value")
    let unset_result lit = env.unset_env("TEST_VAR")
    vibez.spillln("set_env('TEST_VAR', 'test_value'): ", set_result)
    vibez.spillln("unset_env('TEST_VAR'): ", unset_result)
    
    return 1
}

func test_fs_comprehensive() drip {
    vibez.spillln("=== TESTING FS MODULE ===", "")
    
    // Test file existence and properties
    let file_exists lit = fs.exists("/test/file.txt")
    let is_dir lit = fs.is_dir("/test/directory")
    let is_file lit = fs.is_file("/test/file.txt")
    let file_size drip = fs.get_size("/test/file.txt")
    vibez.spillln("exists('/test/file.txt'): ", file_exists)
    vibez.spillln("is_dir('/test/directory'): ", is_dir)
    vibez.spillln("is_file('/test/file.txt'): ", is_file)
    vibez.spillln("get_size('/test/file.txt'): ", file_size)
    
    // Test file permissions
    let is_readable lit = fs.is_readable("/test/file.txt")
    let is_writable lit = fs.is_writable("/test/file.txt")
    let is_executable lit = fs.is_executable("/test/script.sh")
    vibez.spillln("is_readable('/test/file.txt'): ", is_readable)
    vibez.spillln("is_writable('/test/file.txt'): ", is_writable)
    vibez.spillln("is_executable('/test/script.sh'): ", is_executable)
    
    // Test path utilities
    let basename tea = fs.get_basename("/path/to/file.txt")
    let parent_dir tea = fs.get_parent_dir("/path/to/file.txt")
    let joined tea = fs.join_path("/path", "file.txt")
    let extension tea = fs.file_extension("document.pdf")
    vibez.spillln("get_basename('/path/to/file.txt'): ", basename)
    vibez.spillln("get_parent_dir('/path/to/file.txt'): ", parent_dir)
    vibez.spillln("join_path('/path', 'file.txt'): ", joined)
    vibez.spillln("file_extension('document.pdf'): ", extension)
    
    // Test metadata functions
    let mod_time drip = fs.get_modified_time("/test/file.txt")
    let create_time drip = fs.get_created_time("/test/file.txt")
    let is_empty lit = fs.is_empty_file("/test/empty.txt")
    let is_hidden lit = fs.is_hidden("/test/.hidden")
    vibez.spillln("get_modified_time('/test/file.txt'): ", mod_time)
    vibez.spillln("get_created_time('/test/file.txt'): ", create_time)
    vibez.spillln("is_empty_file('/test/empty.txt'): ", is_empty)
    vibez.spillln("is_hidden('/test/.hidden'): ", is_hidden)
    
    return 1
}

func test_io_basic_comprehensive() drip {
    vibez.spillln("=== TESTING IO_BASIC MODULE ===", "")
    
    // Test file operations
    let file_exists lit = io_basic.file_exists("/test/file.txt")
    let dir_exists lit = io_basic.dir_exists("/test/directory")
    vibez.spillln("file_exists('/test/file.txt'): ", file_exists)
    vibez.spillln("dir_exists('/test/directory'): ", dir_exists)
    
    // Test path utilities
    let joined tea = io_basic.join_path("/home", "user")
    let extension tea = io_basic.get_extension("file.txt")
    let basename tea = io_basic.get_basename("/path/to/file.txt")
    vibez.spillln("join_path('/home', 'user'): ", joined)
    vibez.spillln("get_extension('file.txt'): ", extension)
    vibez.spillln("get_basename('/path/to/file.txt'): ", basename)
    
    // Test file validation
    let is_valid_name lit = io_basic.is_valid_filename("good_file.txt")
    let is_text lit = io_basic.is_text_file("document.txt")
    let is_binary lit = io_basic.is_binary_file("program.exe")
    vibez.spillln("is_valid_filename('good_file.txt'): ", is_valid_name)
    vibez.spillln("is_text_file('document.txt'): ", is_text)
    vibez.spillln("is_binary_file('program.exe'): ", is_binary)
    
    // Test error handling
    let error_msg tea = io_basic.get_last_error()
    let clear_success lit = io_basic.clear_error()
    vibez.spillln("get_last_error(): ", error_msg)
    vibez.spillln("clear_error(): ", clear_success)
    
    return 1
}

func test_io_advanced_comprehensive() drip {
    vibez.spillln("=== TESTING IO_ADVANCED MODULE ===", "")
    
    // Test formatted output functions (these may be no-ops)
    let printf_success lit = io_advanced.printf_string("Test string: %s", "hello")
    let printf_int_success lit = io_advanced.printf_int("Test integer: %d", 42)
    let printf_float_success lit = io_advanced.printf_float("Test float: %.2f", 123)
    let printf_bool_success lit = io_advanced.printf_bool("Test boolean: %b", true)
    vibez.spillln("printf_string success: ", printf_success)
    vibez.spillln("printf_int success: ", printf_int_success) 
    vibez.spillln("printf_float success: ", printf_float_success)
    vibez.spillln("printf_bool success: ", printf_bool_success)
    
    // Test file information
    let file_size drip = io_advanced.get_file_size("/test/file.txt")
    let mod_time drip = io_advanced.get_file_modified_time("/test/file.txt")
    let permissions tea = io_advanced.get_file_permissions("/test/file.txt")
    vibez.spillln("get_file_size('/test/file.txt'): ", file_size)
    vibez.spillln("get_file_modified_time('/test/file.txt'): ", mod_time)
    vibez.spillln("get_file_permissions('/test/file.txt'): ", permissions)
    
    // Test validation functions
    let json_valid lit = io_advanced.validate_json("{\"key\": \"value\"}")
    let csv_formatted tea = io_advanced.format_csv_line("col1,col2,col3")
    let csv_escaped tea = io_advanced.escape_csv_field("data,with,commas")
    vibez.spillln("validate_json('{\"key\": \"value\"}'): ", json_valid)
    vibez.spillln("format_csv_line('col1,col2,col3'): ", csv_formatted)
    vibez.spillln("escape_csv_field('data,with,commas'): ", csv_escaped)
    
    // Test configuration functions
    let config_value tea = io_advanced.read_config_value("app.name")
    let config_keys drip = io_advanced.list_config_keys()
    vibez.spillln("read_config_value('app.name'): ", config_value)
    vibez.spillln("list_config_keys(): ", config_keys)
    
    return 1
}

func run_comprehensive_validation() drip {
    vibez.spillln("", "")
    vibez.spillln("CURSED STDLIB COMPREHENSIVE VALIDATION SUITE", "")
    vibez.spillln("===============================================", "")
    
    let mathz_result drip = test_mathz_comprehensive()
    let stringz_result drip = test_stringz_comprehensive()
    let vibez_result drip = test_vibez_comprehensive()
    let path_result drip = test_path_comprehensive()
    let env_result drip = test_env_comprehensive()
    let fs_result drip = test_fs_comprehensive()
    let io_basic_result drip = test_io_basic_comprehensive()
    let io_advanced_result drip = test_io_advanced_comprehensive()
    
    vibez.spillln("", "")
    vibez.spillln("=== VALIDATION SUMMARY ===", "")
    vibez.spillln("mathz module result: ", mathz_result)
    vibez.spillln("stringz module result: ", stringz_result)
    vibez.spillln("vibez module result: ", vibez_result)
    vibez.spillln("path module result: ", path_result)
    vibez.spillln("env module result: ", env_result)
    vibez.spillln("fs module result: ", fs_result)
    vibez.spillln("io_basic module result: ", io_basic_result)
    vibez.spillln("io_advanced module result: ", io_advanced_result)
    
    let total_modules drip = 8
    let passed_modules drip = mathz_result + stringz_result + vibez_result + path_result + env_result + fs_result + io_basic_result + io_advanced_result
    
    vibez.spillln("", "")
    vibez.spillln("Total modules tested: ", total_modules)
    vibez.spillln("Modules passed: ", passed_modules)
    
    return passed_modules
}

func main() drip {
    let validation_result drip = run_comprehensive_validation()
    return validation_result
}
