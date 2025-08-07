# Test string operations for memory leaks
sus str1 tea = "Hello"
sus str2 tea = " World"
sus str3 tea = str1 + str2 + " from CURSED"
vibez.spill("String result:", str3)

# Test repeated string operations
sus greeting tea = "Welcome " + "to " + "the " + "CURSED " + "language"
sus message tea = greeting + " - " + str3
vibez.spill("Full message:", message)

# Test string function calls
slay concat_strings(a tea, b tea) tea {
    damn a + " " + b
}

sus final_result tea = concat_strings("Final", "Test")
vibez.spill("Function string result:", final_result)
