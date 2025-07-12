; Keywords
[
  "vibe"
  "yeet"
  "facts"
  "sus"
  "be_like"
  "slay"
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
  "later"
  "stan"
  "close"
  "squad"
  "vibes"
  "map"
] @keyword

; Gen Z slang keywords with special highlighting
[
  "slay"
  "lowkey"
  "highkey"
  "vibe_check"
  "bestie"
  "periodt"
  "ghosted"
  "simp"
  "stan"
] @keyword.control

; Control flow
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
  "later"
] @keyword.control

; Declaration keywords
[
  "vibe"
  "yeet"
  "facts"
  "sus"
  "be_like"
  "slay"
] @keyword.storage

; Builtin types
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
] @type.builtin

; Custom types
(type_name) @type
(type_spec name: (identifier) @type)
(struct_type) @type
(interface_type) @type
(channel_type) @type
(array_type) @type
(slice_type) @type
(map_type) @type
(pointer_type) @type
(function_type) @type

; Function definitions
(function_declaration name: (identifier) @function)
(method_declaration name: (identifier) @function.method)
(function_literal) @function

; Function calls
(call_expression function: (identifier) @function.call)
(call_expression function: (selector_expression field: (identifier) @function.call))

; Variables
(var_declaration (identifier) @variable)
(const_declaration (identifier) @variable)
(short_var_declaration left: (identifier) @variable)
(parameter_declaration (identifier) @variable.parameter)
(field_declaration (identifier) @variable.field)

; Operators
[
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
  "+"
  "-"
  "*"
  "/"
  "%"
  "&"
  "|"
  "^"
  "<<"
  ">>"
  "&^"
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "&&"
  "||"
  "!"
  "++"
  "--"
  "<-"
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
  "."
  ":"
  "?"
] @punctuation.delimiter

; Literals
(int_literal) @number
(float_literal) @number.float
(string_literal) @string
(char_literal) @string.special
(bool_literal) @boolean
(nil_literal) @constant.builtin

; Boolean literals with Gen Z slang
"based" @boolean
"cap" @boolean
"cringe" @constant.builtin

; Comments
(comment) @comment

; Identifiers
(identifier) @variable
(qualified_identifier name: (identifier) @variable)
(qualified_identifier package: (identifier) @namespace)

; Package names
(package_clause name: (identifier) @namespace)
(import_spec path: (string_literal) @string)

; Labels
(break_statement label: (identifier) @label)
(continue_statement label: (identifier) @label)

; Tags
(tag) @string.special

; Error handling
(error_type) @type.builtin

; Channel operations
(send_statement) @keyword.operator
(receive_expression) @keyword.operator
(receive_statement) @keyword.operator

; Select statement cases
(select_case) @keyword.control

; Method receivers
(method_receiver) @variable.parameter

; Type assertions
(type_assertion_expression) @keyword.operator

; Embedded fields
(embedded_field) @type

; Composite literals
(composite_literal type: (identifier) @type)
(keyed_element key: (identifier) @property)

; Gen Z slang highlighting for better readability
"dm" @type.builtin  ; channel type
"squad" @keyword.storage  ; struct
"vibes" @keyword.storage  ; interface

; Special highlighting for CURSED-specific constructs
(go_statement) @keyword.control
(defer_statement) @keyword.control
(channel_type) @type.builtin
(range_clause) @keyword.control
