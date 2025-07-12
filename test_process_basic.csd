yeet "process"
yeet "vibez"
yeet "stringz"

# Test basic process module functionality
process.set_env("TEST_VAR", "hello_world")
result := process.get_env("TEST_VAR")
vibez.spill("Environment variable test: " + result)

# Test command line arguments
args := process.get_args()
vibez.spill("Program name: " + args[0])

# Test process information
pid := process.get_pid()
vibez.spill("Process ID: " + stringz.from_int(pid))

user := process.get_user()
vibez.spill("User: " + user)

cwd := process.get_cwd()
vibez.spill("Working directory: " + cwd)

# Test self-hosting setup
process.setup_compiler_environment()
stage := process.get_env("CURSED_STAGE")
vibez.spill("Compiler stage: " + stage)

vibez.spill("Process module test complete!")
