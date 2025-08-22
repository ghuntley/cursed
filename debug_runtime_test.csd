// Test direct runtime function calls

vibez.spill("Testing runtime_string_length...")
test_length := runtime_string_length("hello")
vibez.spill("Length of 'hello':", test_length)

vibez.spill("Testing runtime_get_env...")
env_result := runtime_get_env("HOME")
vibez.spill("Environment result received")

vibez.spill("Done")
