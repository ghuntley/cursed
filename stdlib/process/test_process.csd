yeet "testz"
yeet "process"
yeet "stringz"

# Comprehensive Process Module Tests

# Test environment variable operations
test_start("Environment variable get/set")
process.set_env("TEST_VAR", "test_value")
result := process.get_env("TEST_VAR")
assert_eq_string(result, "test_value")

test_start("Environment variable unset")
process.set_env("TEMP_VAR", "temp_value")
process.unset_env("TEMP_VAR")
result := process.get_env("TEMP_VAR")
assert_eq_string(result, "")

test_start("Default environment variables")
home := process.get_env("HOME")
assert_eq_string(home, "/home/user")
user := process.get_env("USER")
assert_eq_string(user, "user")

test_start("Get all environment variables")
all_env := process.get_all_env()
assert_true(len(all_env) > 0)

# Test command line argument operations
test_start("Set and get command line arguments")
test_args := []tea{"cursed", "test.csd", "--optimize", "--verbose"}
process.set_args(test_args)
args := process.get_args()
assert_eq_int(len(args), 4)
assert_eq_string(args[0], "cursed")
assert_eq_string(args[1], "test.csd")

test_start("Get specific argument by index")
arg := process.get_arg(1)
assert_eq_string(arg, "test.csd")

test_start("Parse command line arguments")
test_args := []tea{"cursed", "program.csd", "--optimize", "--output=binary", "-v"}
parsed := process.parse_args(test_args)
assert_eq_string(parsed["optimize"], "based")
assert_eq_string(parsed["output"], "binary")
assert_eq_string(parsed["v"], "based")
assert_eq_string(parsed["1"], "program.csd")

# Test process execution
test_start("Run command - llc version")
exit_code := process.run_command("llc --version")
assert_eq_int(exit_code, 0)

test_start("Run command - gcc version")
exit_code := process.run_command("gcc --version")
assert_eq_int(exit_code, 0)

test_start("Spawn process with arguments")
args := []tea{"--version"}
exit_code := process.spawn_process("llc", args)
assert_eq_int(exit_code, 0)

test_start("Command exists check")
assert_true(process.command_exists("llc"))
assert_true(process.command_exists("gcc"))
assert_true(process.command_exists("cursed"))

# Test working directory operations
test_start("Get and set working directory")
original_cwd := process.get_cwd()
process.set_cwd("/tmp/test")
new_cwd := process.get_cwd()
assert_eq_string(new_cwd, "/tmp/test")

test_start("Change directory")
process.change_dir("/home/user/projects")
cwd := process.get_cwd()
assert_eq_string(cwd, "/home/user/projects")

# Test process information
test_start("Get process ID")
pid := process.get_pid()
assert_true(pid > 0)

test_start("Get user information")
user := process.get_user()
assert_eq_string(user, "user")

test_start("Get hostname")
hostname := process.get_hostname()
assert_eq_string(hostname, "cursed-host")

test_start("Get platform information")
platform := process.get_platform()
assert_eq_string(platform, "linux")

test_start("Get architecture")
arch := process.get_arch()
assert_eq_string(arch, "x86_64")

# Test self-hosting helper functions
test_start("Setup compiler environment")
success := process.setup_compiler_environment()
assert_true(success)

stage := process.get_env("CURSED_STAGE")
assert_eq_string(stage, "2")

self_hosting := process.get_env("CURSED_SELF_HOSTING")
assert_eq_string(self_hosting, "based")

test_start("Get compiler arguments")
test_args := []tea{"cursed", "main.csd", "--optimize", "--output=binary"}
process.set_args(test_args)
compiler_args := process.get_compiler_args()
assert_eq_int(len(compiler_args), 3)
assert_eq_string(compiler_args[0], "main.csd")
assert_eq_string(compiler_args[1], "--optimize")
assert_eq_string(compiler_args[2], "--output=binary")

test_start("Execute llc command")
exit_code := process.execute_llc("test.ll", "test.o")
assert_eq_int(exit_code, 0)

test_start("Execute gcc command")
exit_code := process.execute_gcc("test.o", "test")
assert_eq_int(exit_code, 0)

test_start("Check build tools")
tools_available := process.check_build_tools()
assert_true(tools_available)

# Test argument parsing edge cases
test_start("Parse empty arguments")
empty_args := []tea{}
parsed := process.parse_args(empty_args)
assert_eq_int(len(parsed), 0)

test_start("Parse single argument")
single_arg := []tea{"program.csd"}
parsed := process.parse_args(single_arg)
assert_eq_string(parsed["0"], "program.csd")

test_start("Parse mixed argument formats")
mixed_args := []tea{"cursed", "--input=file.csd", "-v", "--debug", "output.exe"}
parsed := process.parse_args(mixed_args)
assert_eq_string(parsed["input"], "file.csd")
assert_eq_string(parsed["v"], "based")
assert_eq_string(parsed["debug"], "based")
assert_eq_string(parsed["4"], "output.exe")

# Test environment variable edge cases
test_start("Get non-existent environment variable")
result := process.get_env("NON_EXISTENT_VAR")
assert_eq_string(result, "")

test_start("Set empty environment variable")
process.set_env("EMPTY_VAR", "")
result := process.get_env("EMPTY_VAR")
assert_eq_string(result, "")

test_start("Set environment variable with spaces")
process.set_env("SPACE_VAR", "value with spaces")
result := process.get_env("SPACE_VAR")
assert_eq_string(result, "value with spaces")

# Test argument access edge cases
test_start("Get argument with invalid index")
result := process.get_arg(-1)
assert_eq_string(result, "")

result := process.get_arg(999)
assert_eq_string(result, "")

# Test compiler integration scenarios
test_start("Compiler environment setup")
process.setup_compiler_environment()
compiler_path := process.get_env("CURSED_COMPILER_PATH")
assert_true(stringz.contains(compiler_path, "cursed"))

llvm_config := process.get_env("LLVM_CONFIG")
assert_eq_string(llvm_config, "/usr/bin/llvm-config")

test_start("Self-hosting flag detection")
self_hosting_flag := process.get_env("CURSED_SELF_HOSTING")
assert_eq_string(self_hosting_flag, "based")

# Test directory operations
test_start("Working directory persistence")
process.set_cwd("/usr/local/bin")
cwd1 := process.get_cwd()
cwd2 := process.get_cwd()
assert_eq_string(cwd1, cwd2)

# Test process information consistency
test_start("Process information consistency")
pid1 := process.get_pid()
pid2 := process.get_pid()
assert_eq_int(pid1, pid2)

user1 := process.get_user()
user2 := process.get_user()
assert_eq_string(user1, user2)

print_test_summary()
