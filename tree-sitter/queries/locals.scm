; Scopes
(source_file) @local.scope
(block) @local.scope
(function_declaration) @local.scope
(method_declaration) @local.scope
(if_statement) @local.scope
(for_statement) @local.scope
(while_statement) @local.scope
(switch_statement) @local.scope
(select_statement) @local.scope

; Definitions
(function_declaration name: (identifier) @local.definition.function)
(method_declaration name: (identifier) @local.definition.method)
(var_declaration (var_spec (identifier_list (identifier) @local.definition.var)))
(const_declaration (const_spec (identifier_list (identifier) @local.definition.constant)))
(type_declaration (type_spec name: (identifier) @local.definition.type))
(parameter_declaration (identifier_list (identifier) @local.definition.parameter))
(short_var_declaration left: (identifier_list (identifier) @local.definition.var))
(short_var_declaration left: (tuple_destructure (identifier) @local.definition.var))
(range_clause (identifier_list (identifier) @local.definition.var))
(error_statement variable: (identifier) @local.definition.var)
(recovery_statement error_variable: (identifier) @local.definition.var)

; References
(identifier) @local.reference

; Imports
(import_declaration) @local.import
