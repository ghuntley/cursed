; Inject comments as comment language
(comment) @injection.content
(#set! injection.language "comment")

; Inject string literals that look like other languages
(string_literal) @injection.content
(#match? @injection.content "^\"(SELECT|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER)")
(#set! injection.language "sql")

(string_literal) @injection.content
(#match? @injection.content "^\"(<[^>]+>|<!DOCTYPE)")
(#set! injection.language "html")

(string_literal) @injection.content
(#match? @injection.content "^\"\\{")
(#set! injection.language "json")

; Inject raw strings that look like other languages
(string_literal) @injection.content
(#match? @injection.content "^`(SELECT|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER)")
(#set! injection.language "sql")

(string_literal) @injection.content
(#match? @injection.content "^`(<[^>]+>|<!DOCTYPE)")
(#set! injection.language "html")

(string_literal) @injection.content
(#match? @injection.content "^`\\{")
(#set! injection.language "json")

; Inject CSS in style strings
(string_literal) @injection.content
(#match? @injection.content "^\".*\\{.*:")
(#set! injection.language "css")

(string_literal) @injection.content
(#match? @injection.content "^`.*\\{.*:")
(#set! injection.language "css")
