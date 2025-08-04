fr fr Comprehensive error reporting test for CURSED Zig compiler
fr fr Tests various error categories with rich diagnostics

fr fr Lexical Errors
"unterminated string
'unterminated char
/* unterminated comment
sus invalid_escape tea = "test\invalid"

fr fr Syntax Errors  
slay missing_params( { damn "no params" }
sus missing_type = 42
slay invalid_return_type() invalid_type { damn 0 }
sus unbalanced { braces
collab missing_body {

fr fr Semantic Errors
sus x normie = "string"  fr fr Type mismatch
vibez.spill(undefined_var)  fr fr Undefined variable
nonexistent_function()  fr fr Undefined function
sus x normie = 42; sus x normie = 43  fr fr Duplicate definition

fr fr Advanced syntax errors
match incomplete_match
bestie invalid_loop
stan { // goroutine without proper syntax
if (missing_condition {

fr fr Interface errors without implementation
collab TestInterface {
    slay test_method()
}

squad TestStruct {}
flex TestStruct => NonExistentInterface {
    slay wrong_method() { damn }
}
