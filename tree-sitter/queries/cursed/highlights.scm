;; Keywords
((identifier) @keyword.package
 (#eq? @keyword.package "vibe"))

((identifier) @keyword.import
 (#eq? @keyword.import "yeet"))

((identifier) @keyword.function
 (#eq? @keyword.function "slay"))

((identifier) @keyword.var
 (#eq? @keyword.var "sus"))

((identifier) @keyword.const
 (#eq? @keyword.const "facts"))

((identifier) @keyword.if
 (#eq? @keyword.if "lowkey"))

((identifier) @keyword.else
 (#eq? @keyword.else "highkey"))

((identifier) @keyword.for
 (#eq? @keyword.for "bestie"))

((identifier) @keyword.while
 (#eq? @keyword.while "periodt"))

((identifier) @keyword.switch
 (#eq? @keyword.switch "vibe_check"))

((identifier) @keyword.case
 (#eq? @keyword.case "mood"))

((identifier) @keyword.default
 (#eq? @keyword.default "basic"))

((identifier) @keyword.break
 (#eq? @keyword.break "ghosted"))

((identifier) @keyword.continue
 (#eq? @keyword.continue "simp"))

((identifier) @keyword.type
 (#eq? @keyword.type "be_like"))

((identifier) @keyword.struct
 (#eq? @keyword.struct "squad"))

((identifier) @keyword.interface
 (#eq? @keyword.interface "collab"))

((identifier) @keyword.map
 (#eq? @keyword.map "tea"))

((identifier) @keyword.channel
 (#eq? @keyword.channel "dm"))

((identifier) @keyword.go
 (#eq? @keyword.go "stan"))

((identifier) @keyword.range
 (#eq? @keyword.range "flex"))

((identifier) @keyword.defer
 (#eq? @keyword.defer "later"))

((identifier) @keyword.return
 (#eq? @keyword.return "yolo"))

;; Boolean literals
((identifier) @boolean
 (#match? @boolean "^(based|sus)$"))

;; Nil literal
((identifier) @constant.builtin
 (#eq? @constant.builtin "cap"))

;; Type literals
((identifier) @type.builtin
 (#match? @type.builtin "^(lit|smol|mid|normie|thicc|snack|meal|tea|sip|byte|rune)$"))

;; Comments
(line_comment) @comment
(block_comment) @comment

;; Special comment markers
((line_comment) @comment.documentation
 (#match? @comment.documentation "^fr fr TODO:"))

((line_comment) @comment.documentation
 (#match? @comment.documentation "^fr fr FIXME:"))

((line_comment) @comment.documentation
 (#match? @comment.documentation "^fr fr NOTE:"))