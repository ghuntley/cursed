;; Indent after opening brackets
[
  "{"
  "("
  "["
] @indent.begin

;; Dedent for closing brackets
[
  "}"
  ")"
  "]"
] @indent.end

;; Indent for blocks and structures
[
  (block)
  (struct_type)
  (interface_type)
  (literal_value)
  (import_declaration)
] @indent.begin

;; Align continuations with the first parameter/argument
(parameter_list
  "(" @indent.begin
  ")" @indent.end) @indent.align

(arguments
  "(" @indent.begin
  ")" @indent.end) @indent.align

;; Align case statements in a switch
(expression_case_clause
  ":" @indent.begin) @indent.align

(type_case_clause
  ":" @indent.begin) @indent.align

;; Align fields in struct literals
(literal_value
  "{" @indent.begin
  "}" @indent.end) @indent.align