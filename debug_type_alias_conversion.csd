be_like MyString = tea
be_like MyBool = lit
be_like MyFloat = meal

sus name MyString = "Hello, CURSED!"
sus flag MyBool = based
sus pi MyFloat = 3.14159

vibez.spill("name:", name)
vibez.spill("flag:", flag)
vibez.spill("pi:", pi)

# Test type conversion
sus converted_int MyFloat = 42
vibez.spill("converted_int:", converted_int)
