;; Variable definitions

;; Function parameters
(parameter_declaration
  (identifier) @local.definition.parameter)

;; Local variables from short declarations
(short_var_declaration
  (identifier_list
    (identifier) @local.definition.var))

;; Local variables from regular declarations
(var_spec
  (identifier_list
    (identifier) @local.definition.var))

;; Range loop variables
(range_clause
  left: (identifier_list
    (identifier) @local.definition.var))

;; Function definitions
(function_declaration
  name: (identifier) @local.definition.function) @local.scope

;; Type definitions
(type_spec
  name: (identifier) @local.definition.type)

;; Constants
(const_spec
  (identifier_list
    (identifier) @local.definition.constant))

;; References
(identifier) @local.reference

;; Scopes
(block) @local.scope
(function_declaration) @local.scope
(if_statement) @local.scope
(for_statement) @local.scope
(while_statement) @local.scope

;; CURSED-specific scope handling
(expression_switch_statement) @local.scope
(type_switch_statement) @local.scope
(expression_case_clause) @local.scope
(type_case_clause) @local.scope