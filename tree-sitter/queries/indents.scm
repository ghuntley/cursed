; Increase indent
[
  (block)
  (literal_value)
  (expression_case)
  (select_case)
  (interface_type)
  (struct_type)
  (function_declaration)
  (method_declaration)
  (if_statement)
  (for_statement)
  (while_statement)
  (switch_statement)
  (select_statement)
] @indent.increase

; Decrease indent
[
  "}"
  "]"
  ")"
] @indent.decrease

; Align with opening bracket
[
  (argument_list)
  (parameter_list)
  (expression_list)
  (identifier_list)
] @indent.align

; Don't indent these
[
  (comment)
  (string_literal)
] @indent.ignore
