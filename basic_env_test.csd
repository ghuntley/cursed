yeet "envz"
yeet "vibez"

vibez.spill("Testing environment variables...")

sus user tea = get_env("USER")
vibez.spill("USER: " + user)

sus home tea = get_env("HOME") 
vibez.spill("HOME: " + home)

set_env("TEST_VAR", "test_value")
sus test_result tea = get_env("TEST_VAR")
vibez.spill("TEST_VAR: " + test_result)

sus exists_result lit = env_exists("TEST_VAR")
vibez.spill("TEST_VAR exists: " + exists_result.to_string())

vibez.spill("Environment variable test complete!")
