;; Definitions (where variables, types, or functions are defined)

;; Function definitions
(function_declaration
  name: (identifier) @local.definition.function) @local.scope

;; Parameter definitions
(parameter_declaration
  (identifier) @local.definition.parameter)

;; Variable definitions
(var_spec 
  (identifier_list
    (identifier) @local.definition.var))

(short_var_declaration
  (identifier_list
    (identifier) @local.definition.var))

;; Constant definitions
(const_spec
  (identifier_list
    (identifier) @local.definition.constant))

;; Type definitions
(type_spec
  name: (identifier) @local.definition.type)

;; Loop iteration variables
(range_clause
  (identifier_list
    (identifier) @local.definition.var))

;; Scopes - blocks that create variable scopes
[
  (block)
  (function_declaration)
  (if_statement)
  (for_statement)
  (while_statement)
] @local.scope

;; References (where variables or functions are used)
(identifier) @local.reference