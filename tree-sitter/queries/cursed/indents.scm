;; Automatically indent after these tokens
[
  "{"
  "("
  "["
  ":"
] @indent.begin

;; Dedent before these tokens
[
  "}"
  ")"
  "]"
] @indent.end

;; Indent these nodes
[
  (block)
  (struct_type)
  (interface_type)
  (expression_case_clause)
  (type_case_clause)
  (literal_value)
] @indent.begin

;; Aligned indentation for multi-line declarations
(parameter_list) @indent.align
(argument_list) @indent.align
(import_declaration) @indent.align

;; Specific indentation rules for CURSED
(function_declaration
  body: (block) @indent.begin)

(if_statement
  consequence: (block) @indent.begin
  alternative: (block)? @indent.begin)

(for_statement
  body: (block) @indent.begin)

(while_statement
  body: (block) @indent.begin)

(expression_switch_statement
  body: (_) @indent.begin)

(type_switch_statement
  body: (_) @indent.begin)

;; Ensure closing braces align with the start
(block "}") @indent.branch
(struct_type "}") @indent.branch
(interface_type "}") @indent.branch
(literal_value "}") @indent.branch