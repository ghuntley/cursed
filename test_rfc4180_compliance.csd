yeet "csv"
yeet "testz"

fr fr Test RFC 4180 compliance issues
slay test_rfc4180_current_issues() {
    test_start("RFC 4180 Current Issues")
    
    fr fr Test 1: CRLF line endings (current uses only \n)
    sus csv_crlf tea = "name,age\r\nAlice,30\r\nBob,25\r\n"
    sus data_crlf [[tea]] = parse(csv_crlf)
    spill("CRLF test - rows:", len(data_crlf))
    
    fr fr Test 2: Newlines inside quoted fields
    sus csv_newlines tea = "name,description\n\"Alice\",\"Line 1\nLine 2\"\n\"Bob\",\"Single line\""
    sus data_newlines [[tea]] = parse(csv_newlines)
    spill("Newlines test - rows:", len(data_newlines))
    vibes len(data_newlines) >= 2 {
        spill("Alice description:", data_newlines[1][1])
    }
    
    fr fr Test 3: Escaped quotes compliance
    sus csv_quotes tea = "name,quote\n\"Alice\",\"She said \"\"Hello\"\" to me\"\n\"Bob\",\"Simple\""
    sus data_quotes [[tea]] = parse(csv_quotes)
    spill("Quotes test - rows:", len(data_quotes))
    vibes len(data_quotes) >= 2 {
        spill("Alice quote:", data_quotes[1][1])
    }
    
    fr fr Test 4: Consistent field count validation
    sus csv_inconsistent tea = "name,age,city\nAlice,30\nBob,25,LA,Extra"
    sus valid lit = validate(csv_inconsistent)
    spill("Field consistency validation:", valid)
    
    fr fr Test 5: Space preservation
    sus csv_spaces tea = "name, description \n Alice , A person with spaces \n Bob, Another person "
    sus data_spaces [[tea]] = parse(csv_spaces)
    spill("Spaces test - rows:", len(data_spaces))
    vibes len(data_spaces) >= 2 {
        spill("Header with spaces:", "'" + data_spaces[0][1] + "'")
        spill("Alice with spaces:", "'" + data_spaces[1][1] + "'")
    }
    
    print_test_summary()
}

slay main() {
    test_rfc4180_current_issues()
}
