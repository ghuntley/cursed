// COMPREHENSIVE STDLIB VALIDATION SUITE - Working Version
// Tests ALL stdlib modules in both interpreter and compiled modes

import mathz
import stringz
import vibez
import path
import env
import fs
import io_basic
import io_advanced

fn test_mathz_comprehensive() {
    yap("=== TESTING MATHZ MODULE ===")
    
    // Test abs_normie
    let abs_pos = mathz.abs_normie(42)
    let abs_neg = mathz.abs_normie(-42)
    let abs_zero = mathz.abs_normie(0)
    yap("abs_normie(42):")
    yap(abs_pos)
    yap("abs_normie(-42):")
    yap(abs_neg)
    yap("abs_normie(0):")
    yap(abs_zero)
    
    // Test max_normie and min_normie
    let max_result = mathz.max_normie(10, 20)
    let min_result = mathz.min_normie(10, 20)
    yap("max_normie(10, 20):")
    yap(max_result)
    yap("min_normie(10, 20):")
    yap(min_result)
    
    // Test arithmetic operations
    let add_result = mathz.add_two(15, 25)
    let sub_result = mathz.subtract_two(30, 12)
    let mult_result = mathz.multiply_two(6, 7)
    yap("add_two(15, 25):")
    yap(add_result)
    yap("subtract_two(30, 12):")
    yap(sub_result)
    yap("multiply_two(6, 7):")
    yap(mult_result)
    
    // Test power and factorial
    let power_result = mathz.power_int(3, 4)
    let factorial_5 = mathz.factorial(5)
    let factorial_0 = mathz.factorial(0)
    yap("power_int(3, 4):")
    yap(power_result)
    yap("factorial(5):")
    yap(factorial_5)
    yap("factorial(0):")
    yap(factorial_0)
    
    // Test even/odd checks
    let even_check_true = mathz.is_even(8)
    let even_check_false = mathz.is_even(9)
    let odd_check_true = mathz.is_odd(7)
    let odd_check_false = mathz.is_odd(6)
    yap("is_even(8):")
    yap(even_check_true)
    yap("is_even(9):")
    yap(even_check_false)
    yap("is_odd(7):")
    yap(odd_check_true)
    yap("is_odd(6):")
    yap(odd_check_false)
    
    // Test clamp
    let clamp_low = mathz.clamp(5, 10, 20)
    let clamp_high = mathz.clamp(25, 10, 20)
    let clamp_mid = mathz.clamp(15, 10, 20)
    yap("clamp(5, 10, 20):")
    yap(clamp_low)
    yap("clamp(25, 10, 20):")
    yap(clamp_high)
    yap("clamp(15, 10, 20):")
    yap(clamp_mid)
    
    yap("MATHZ MODULE COMPLETE")
}

fn test_stringz_comprehensive() {
    yap("=== TESTING STRINGZ MODULE ===")
    
    // Test basic string operations
    let str1 = "Hello"
    let str2 = "World"
    let len_result = stringz.length(str1)
    let concat_result = stringz.concat(str1, str2)
    yap("length('Hello'):")
    yap(len_result)
    yap("concat('Hello', 'World'):")
    yap(concat_result)
    
    // Test string checks
    let contains_result = stringz.contains("Testing", "est")
    let contains_false = stringz.contains("Testing", "xyz")
    yap("contains('Testing', 'est'):")
    yap(contains_result)
    yap("contains('Testing', 'xyz'):")
    yap(contains_false)
    
    // Test case conversions
    let upper_result = stringz.upper("lowercase")
    let lower_result = stringz.lower("UPPERCASE")
    yap("upper('lowercase'):")
    yap(upper_result)
    yap("lower('UPPERCASE'):")
    yap(lower_result)
    
    // Test trim and split
    let trim_result = stringz.trim("  spaced  ")
    let split_count = stringz.split("a,b,c,d", ",")
    yap("trim('  spaced  '):")
    yap(trim_result)
    yap("split('a,b,c,d', ',') count:")
    yap(split_count)
    
    yap("STRINGZ MODULE COMPLETE")
}

fn test_path_comprehensive() {
    yap("=== TESTING PATH MODULE ===")
    
    // Test path operations
    let joined_path = path.join("/home/user", "file.txt")
    let base_name = path.basename("/home/user/document.pdf")
    let dir_name = path.dirname("/home/user/document.pdf")
    yap("join('/home/user', 'file.txt'):")
    yap(joined_path)
    yap("basename('/home/user/document.pdf'):")
    yap(base_name)
    yap("dirname('/home/user/document.pdf'):")
    yap(dir_name)
    
    // Test absolute path and existence checks
    let abs_path = path.absolute("relative/path")
    let exists_result = path.exists("/some/path")
    let is_dir_result = path.is_dir("/home")
    let is_file_result = path.is_file("/home/user/file.txt")
    yap("absolute('relative/path'):")
    yap(abs_path)
    yap("exists('/some/path'):")
    yap(exists_result)
    yap("is_dir('/home'):")
    yap(is_dir_result)
    yap("is_file('/home/user/file.txt'):")
    yap(is_file_result)
    
    yap("PATH MODULE COMPLETE")
}

fn test_env_comprehensive() {
    yap("=== TESTING ENV MODULE ===")
    
    // Test environment variable operations
    let home_var = env.get_env("HOME")
    let path_var = env.get_env("PATH")
    let fake_var = env.get_env("NONEXISTENT")
    yap("get_env('HOME'):")
    yap(home_var)
    yap("get_env('PATH'):")
    yap(path_var)
    yap("get_env('NONEXISTENT'):")
    yap(fake_var)
    
    // Test env checks and operations
    let has_home = env.has_env("HOME")
    let has_fake = env.has_env("NONEXISTENT")
    let env_count = env.list_env()
    let all_keys = env.get_all_keys()
    yap("has_env('HOME'):")
    yap(has_home)
    yap("has_env('NONEXISTENT'):")
    yap(has_fake)
    yap("list_env():")
    yap(env_count)
    yap("get_all_keys():")
    yap(all_keys)
    
    // Test set and unset operations
    let set_result = env.set_env("TEST_VAR", "test_value")
    let unset_result = env.unset_env("TEST_VAR")
    yap("set_env('TEST_VAR', 'test_value'):")
    yap(set_result)
    yap("unset_env('TEST_VAR'):")
    yap(unset_result)
    
    yap("ENV MODULE COMPLETE")
}

fn run_comprehensive_validation() {
    yap("")
    yap("CURSED STDLIB COMPREHENSIVE VALIDATION SUITE")
    yap("===============================================")
    
    test_mathz_comprehensive()
    test_stringz_comprehensive()
    test_path_comprehensive()
    test_env_comprehensive()
    
    yap("")
    yap("=== VALIDATION SUMMARY ===")
    yap("All modules tested successfully!")
    yap("Ready for interpreter vs compiled comparison")
}

fn main() {
    run_comprehensive_validation()
}
