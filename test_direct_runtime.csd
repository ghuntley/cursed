fr Test direct runtime function calls
sus test_str tea = "hello"
sus char_result sip = runtime_string_char_at(test_str, 0)
vibez.spill("First char:", char_result)

sus char_str tea = runtime_char_to_string('h')
vibez.spill("Char to string:", char_str)
