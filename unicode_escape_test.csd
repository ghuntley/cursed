// Test Unicode and escape sequence handling
sus basic_string tea = "Hello, World!"
sus unicode_string tea = "Hello, 世界! 🌍"
sus escaped_string tea = "Line 1\nLine 2\tTabbed\r\nWindows line ending"
sus hex_escape tea = "Hex byte: \x41"
sus unicode_escape tea = "Unicode: \u0041\u4E16\u754C"
sus extended_unicode tea = "Extended: \u{1F30D}\u{1F44B}"
sus octal_escape tea = "Octal: \101\102\103"
sus quotes_escape tea = "Quote: \"Hello\" and 'World'"

// Test Unicode in identifiers and strings
sus 变量 tea = "Chinese variable name"
sus émoji tea = "👋🌍🎉"

vibez.spill(basic_string)
vibez.spill(unicode_string)
vibez.spill(escaped_string)
vibez.spill(hex_escape)
vibez.spill(unicode_escape)
vibez.spill(extended_unicode)
vibez.spill(octal_escape)
vibez.spill(quotes_escape)
vibez.spill(变量)
vibez.spill(émoji)
