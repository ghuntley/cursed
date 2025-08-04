fr fr Error recovery validation test
fr fr Tests parser's ability to recover from errors and continue compilation

slay valid_function() normie {
    sus x normie = 42
    damn x
}

fr fr ERROR: Missing semicolon - should recover and continue
sus broken_syntax normie = invalid

fr fr This should still parse after error recovery
slay another_valid_function() tea {
    sus message tea = "recovered!"
    damn message
}

fr fr ERROR: Unbalanced braces - should recover
slay broken_braces() {
    sus x normie = 1
    fr fr Missing closing brace

fr fr ERROR: Invalid token sequence - should recover
invalid_keyword unexpected_tokens @#$%

fr fr This should parse correctly after multiple errors
squad ValidStruct {
    spill name tea
    spill value normie
}

fr fr ERROR: Invalid field syntax - should recover  
squad BrokenStruct {
    invalid field syntax
    missing_type_annotation
}

fr fr Final valid code to test complete recovery
slay final_function() lit {
    sus result lit = based
    damn result
}
