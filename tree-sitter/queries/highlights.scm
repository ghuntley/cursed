; Keywords
[
  "vibe"
  "yeet"
  "facts"
  "sus"
  "be_like"
  "slay"
  "squad"
  "vibes"
] @keyword

; Control flow keywords
[
  "lowkey"
  "highkey"
  "vibe_check"
  "mood"
  "basic"
  "bestie"
  "flex"
  "periodt"
  "ready"
  "yolo"
  "ghosted"
  "simp"
  "defer"
  "stan"
  "later"
] @keyword.control

; Error handling keywords
[
  "yikes"
  "shook"
  "fam"
] @keyword.exception

; Types
[
  "normie"
  "smol"
  "mid"
  "thicc"
  "drip"
  "snack"
  "meal"
  "byte"
  "rune"
  "extra"
  "tea"
  "lit"
  "sip"
  "dm"
  "chan"
  "map"
] @type.builtin

; Literals
[
  "based"
  "cap"
  "cringe"
] @constant.builtin

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
(call_expression function: (identifier) @function)
(call_expression function: (selector_expression field: (identifier) @function))

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
  "<-"
  "->"
  "++"
  "--"
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
