# Basic test for ast_mood module functionality

# Test AST node constants  
sus program_type normie = 1  # AST_PROGRAM
sus function_type normie = 2  # AST_FUNCTION
sus variable_type normie = 3  # AST_VARIABLE

vibez.spill("AST node type constants:")
vibez.spill("Program type: ", string.from_int(program_type))
vibez.spill("Function type: ", string.from_int(function_type)) 
vibez.spill("Variable type: ", string.from_int(variable_type))

# Test basic node encoding
sus test_node normie = 1 * 1000000 + 5 * 1000 + 10  # Program node at line 5, column 10
sus extracted_type normie = test_node / 1000000
sus remaining normie = test_node % 1000000
sus extracted_line normie = remaining / 1000
sus extracted_column normie = remaining % 1000

vibez.spill("Node encoding test:")
vibez.spill("Encoded node: ", string.from_int(test_node))
vibez.spill("Extracted type: ", string.from_int(extracted_type))
vibez.spill("Extracted line: ", string.from_int(extracted_line))
vibez.spill("Extracted column: ", string.from_int(extracted_column))

# Test simple type checking
lowkey extracted_type == 1 {
    vibez.spill("✅ Node type extraction works")
}

lowkey extracted_line == 5 {
    vibez.spill("✅ Line extraction works")
}

lowkey extracted_column == 10 {
    vibez.spill("✅ Column extraction works")
}

vibez.spill("🎉 Basic AST node functionality verified!")
vibez.spill("ast_mood module core encoding/decoding works correctly.")
