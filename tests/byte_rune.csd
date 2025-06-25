fr fr Test file for byte and rune literals

fr fr Test byte literals
sus b = b'a';
puts(b); fr fr Should print 97 (ASCII for 'a')

fr fr Test rune literals
sus r = 'X';
puts(r); fr fr Should print 88 (ASCII/Unicode for 'X')

fr fr Using other byte values
sus newline = b'\n';
sus tab = b'\t';
sus backslash = b'\\';
sus single_quote = b'\'';

puts(newline);
puts(tab);
puts(backslash);
puts(single_quote);

fr fr Using other rune values
sus emoji = 'X';
sus newline_r = '\n';
sus backslash_r = '\\';

puts(emoji);
puts(newline_r);
puts(backslash_r);