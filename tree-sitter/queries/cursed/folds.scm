;; Blocks and code sections
(block) @fold

;; Function declarations
(function_declaration
  body: (block)? @fold)

;; Control flow blocks
(if_statement
  consequence: (block) @fold
  alternative: (block)? @fold)

(for_statement
  body: (block) @fold)

(while_statement
  body: (block) @fold)

;; Type declarations
(struct_type
  body: (_) @fold)

(interface_type
  body: (_) @fold)

;; Import groups
(import_declaration
  "(" @fold.begin
  ")" @fold.end) @fold

;; Variable declarations in blocks
(variable_declaration
  "(" @fold.begin
  ")" @fold.end) @fold

;; Constant declarations in blocks
(constant_declaration
  "(" @fold.begin
  ")" @fold.end) @fold

;; Switch cases
(expression_switch_statement
  body: (_) @fold
  (expression_case_clause) @fold)

(type_switch_statement
  body: (_) @fold
  (type_case_clause) @fold)

;; Composite literals
(composite_literal
  value: (literal_value) @fold)