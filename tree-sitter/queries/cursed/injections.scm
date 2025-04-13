;; Inject regex language into regex string literals
((call_expression
  function: (selector_expression
    operand: (identifier) @regex
    field: (identifier) @method)
  arguments: (arguments
    (expression_list
      (expression
        (primary_expression
          (operand
            (literal
              (string_literal) @injection.content)))))
  (#eq? @regex "regex")
  (#match? @method "^(Compile|Match|MatchString)$")
  (#set! injection.language "regex"))

;; Inject SQL language into SQL query strings
((call_expression
  function: (selector_expression
    operand: (identifier) @db
    field: (identifier) @method)
  arguments: (arguments
    (expression_list
      (expression
        (primary_expression
          (operand
            (literal
              (string_literal) @injection.content)))))
  (#match? @db "^(sql|db|database|conn)$")
  (#match? @method "^(Exec|Query|QueryRow|Prepare)$")
  (#set! injection.language "sql"))

;; Inject JSON into string literals that are likely JSON
((call_expression
  function: (selector_expression
    operand: (identifier) @json
    field: (identifier) @method)
  arguments: (arguments
    (expression_list
      (expression
        (primary_expression
          (operand
            (literal
              (string_literal) @injection.content)))))
  (#match? @json "^(json)$")
  (#match? @method "^(Marshal|Unmarshal|Parse)$")
  (#set! injection.language "json"))