;; Fold blocks and braces
[  
  (block)
  (struct_type)
  (interface_type)
  (literal_value)
] @fold

;; Function declarations
(function_declaration) @fold

;; Type declarations
(type_declaration) @fold

;; Variable and constant declarations
(variable_declaration
  (_) @fold)

(constant_declaration
  (_) @fold)

;; Control flow constructs
(if_statement) @fold
(for_statement) @fold
(while_statement) @fold
(switch_statement) @fold

;; Import blocks
(import_declaration
  "(" @fold.begin
  ")" @fold.end) @fold

;; Parameter lists
(parameter_list) @fold