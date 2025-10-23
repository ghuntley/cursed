; Keywords
[
     "vibe"
     "yeet"
     "facts"
     "sus"
     "be_like"
     "slay"
     "squad"
     "collab"
     ] @keyword

; Control flow keywords
[
     "ready"
     "otherwise"
     "vibe_check"
     "mood"
     "basic"
     "bestie"
     "flex"
     "periodt"
     "ghosted"
     "simp"
     "later"
     "stan"
     "dm_send"
     "dm_recv"
     "dm_close"
     ] @keyword.control

; Error handling keywords
[
     "yikes"
     "fam"
     ] @keyword.exception

; Types
(builtin_type) @type.builtin

; Literals
(bool_literal) @constant.builtin

; Strings
(string_literal) @string
(char_literal) @string.special

; Numbers
(int_literal) @number
(float_literal) @number.float

; Comments
(comment) @comment

; Identifiers
(identifier) @variable

; Function names
(function_declaration name: (identifier) @function)
(method_declaration name: (identifier) @function.method)
(call_expression function: (primary_expression (operand (identifier))) @function)
(call_expression function: (primary_expression (selector_expression field: (identifier))) @function)

; Operators
[
     "+"
     "-"
     "*"
     "/"
     "%"
     "="
     ":="
     "+="
     "-="
     "*="
     "/="
     "%="
     "&="
     "|="
     "^="
     "<<="
     ">>="
     "=="
     "!="
     "<"
     "<="
     ">"
     ">="
     "&&"
     "||"
     "!"
     "&"
     "|"
     "^"
     "<<"
     ">>"
     "&^"
     "++"
     "--"
     "ඞ"
     ] @operator

; Punctuation
[
     "("
     ")"
     "["
     "]"
     "{"
     "}"
     ","
     ";"
     ":"
     "."
     "?"
     "<"
     ">"
     ] @punctuation.delimiter

; Package names
(package_clause name: (identifier) @module)
(import_spec path: (string_literal) @string.special)
(qualified_identifier package: (identifier) @module)

; Type names in declarations
(type_declaration (type_spec name: (identifier) @type))
(var_declaration (var_spec (identifier_list (identifier) @variable)))
(const_declaration (const_spec (identifier_list (identifier) @constant)))

; Field names
(field_declaration (identifier_list (identifier) @property))
(selector_expression field: (identifier) @property)

; Parameters
(parameter_declaration (identifier_list (identifier) @parameter))
(method_receiver (identifier) @parameter)

; Labels
(break_statement label: (identifier) @label)
(continue_statement label: (identifier) @label)

; Error variables
(error_statement variable: (identifier) @variable.special)
(recovery_statement error_variable: (identifier) @variable.special)

; Concurrency constructs
(channel_type) @type