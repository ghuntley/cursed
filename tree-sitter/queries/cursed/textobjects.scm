;; Function textobjects
(function_declaration) @function.outer
(function_declaration
  body: (block) @function.inner)

;; Class-like textobjects (structs/interfaces)
(struct_type) @class.outer
(struct_type
  "{" . (_) . "}" @class.inner)
  
(interface_type) @class.outer
(interface_type
  "{" . (_) . "}" @class.inner)

;; Parameter textobjects
(parameter_list) @parameter.outer
(parameter_list
  "(" . (_) . ")" @parameter.inner)

;; Call textobjects
(call_expression
  arguments: (arguments)) @call.outer
(call_expression
  arguments: (arguments
    "(" . (_) . ")" @call.inner))

;; Comment textobjects
(line_comment) @comment.outer
(block_comment) @comment.outer

;; Statement textobjects
[
  (expression_statement)
  (assignment_statement)
  (short_var_declaration)
  (return_statement)
  (block)
  (if_statement)
  (for_statement)
  (while_statement)
  (break_statement)
  (continue_statement)
] @statement.outer

;; Block textobjects
(block) @block.outer
(block "{" . (_) . "}" @block.inner)

;; Conditional textobjects
(if_statement) @conditional.outer
(if_statement
  consequence: (block) @conditional.inner)

;; Loop textobjects
(for_statement) @loop.outer
(for_statement
  body: (block) @loop.inner)

(while_statement) @loop.outer
(while_statement
  body: (block) @loop.inner)