fr fr Test codegen for byte and rune literals

fr fr Byte literals
sus a = b'a';
puts(a); fr fr Should print 97 (ASCII for 'a')

fr fr Rune literals
sus b = 'X';
puts(b); fr fr Should print 88 (ASCII/Unicode for 'X')