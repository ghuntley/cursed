; Fold blocks
[
  (block)
  (literal_value)
  (field_declaration_list)
  (interface_type)
  (struct_type)
  (function_declaration)
  (method_declaration)
  (if_statement)
  (for_statement)
  (while_statement)
  (switch_statement)
  (select_statement)
] @fold

; Fold comments
(comment) @fold

; Fold imports
(import_declaration) @fold

; Fold const/var declarations
(const_declaration) @fold
(var_declaration) @fold
(type_declaration) @fold

; Fold parameter lists
(parameter_list) @fold
(argument_list) @fold

; Fold array/slice literals
(composite_literal) @fold
