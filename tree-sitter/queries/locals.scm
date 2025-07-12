; Scopes
(source_file) @local.scope
(block) @local.scope
(function_declaration) @local.scope
(method_declaration) @local.scope
(function_literal) @local.scope
(if_statement) @local.scope
(for_statement) @local.scope
(while_statement) @local.scope
(switch_statement) @local.scope
(select_statement) @local.scope

; Definitions
(var_declaration (identifier) @local.definition.var)
(const_declaration (identifier) @local.definition.constant)
(short_var_declaration left: (identifier) @local.definition.var)
(parameter_declaration (identifier) @local.definition.parameter)
(function_declaration name: (identifier) @local.definition.function)
(method_declaration name: (identifier) @local.definition.method)
(type_declaration (type_spec name: (identifier) @local.definition.type))
(field_declaration (identifier) @local.definition.field)

; Package-level definitions
(package_clause name: (identifier) @local.definition.namespace)
(import_spec (identifier) @local.definition.import)

; References
(identifier) @local.reference

; Special scoping for range clauses
(range_clause (identifier) @local.definition.var)
(for_clause init: (short_var_declaration left: (identifier) @local.definition.var))

; Method receivers create a new scope
(method_receiver (identifier) @local.definition.parameter)

; Type parameters
(type_parameter name: (identifier) @local.definition.type)

; Labels
(break_statement label: (identifier) @local.reference)
(continue_statement label: (identifier) @local.reference)

; Qualified identifiers - package references
(qualified_identifier package: (identifier) @local.reference)

; Select statement variable bindings
(select_case (receive_statement (identifier) @local.definition.var))

; Embedded fields in structs
(embedded_field (identifier) @local.reference)

; Composite literal keys
(keyed_element key: (identifier) @local.reference)

; Channel operations variable bindings
(receive_statement (identifier) @local.definition.var)

; Error handling patterns
(assignment_statement 
  left: (expression_list (identifier) @local.definition.var)
  right: (expression_list (call_expression)))

; Type switch variable bindings
(type_switch_statement 
  (short_var_declaration left: (identifier) @local.definition.var))

; Anonymous function scoping
(function_literal) @local.scope

; Block-level scoping for control structures
(if_statement consequence: (block) @local.scope)
(if_statement alternative: (block) @local.scope)
(for_statement body: (block) @local.scope)
(while_statement body: (block) @local.scope)
(switch_statement (block) @local.scope)
(select_statement (block) @local.scope)

; Defer statement scoping
(defer_statement) @local.scope

; Go statement scoping
(go_statement) @local.scope

; Case-specific scoping
(expression_case) @local.scope
(select_case) @local.scope
