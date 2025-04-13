;; Keywords
(vibe) @keyword
(yeet) @keyword.import
(slay) @keyword.function
(sus) @keyword.storage
(facts) @keyword.storage.constant
(lowkey) @keyword.conditional
(highkey) @keyword.conditional
(bestie) @keyword.repeat
(periodt) @keyword.repeat
(vibe_check) @keyword.control
(mood) @keyword.control
(basic) @keyword.control
(ghosted) @keyword.control
(simp) @keyword.control
(be_like) @keyword.type
(squad) @keyword.type.struct
(collab) @keyword.type.interface
(tea) @keyword.type.map
(dm) @keyword.type.channel
(stan) @keyword.coroutine
(flex) @keyword.operator
(later) @keyword.control
(yolo) @keyword.return

;; Types
(type_name) @type
(pointer_type) @type.pointer
(array_type) @type.builtin
(slice_type) @type.builtin
(struct_type) @type.builtin
(interface_type) @type.builtin
(map_type) @type.builtin
(channel_type) @type.builtin
(function_type) @type.builtin
(parametrized_type) @type.builtin

;; Base types
"lit" @type.builtin
"smol" @type.builtin
"mid" @type.builtin
"normie" @type.builtin
"thicc" @type.builtin
"snack" @type.builtin
"meal" @type.builtin
"tea" @type.builtin
"sip" @type.builtin
"byte" @type.builtin
"rune" @type.builtin

;; Literals
(int_literal) @number
(float_literal) @float
(string_literal) @string
(rune_literal) @character
(byte_literal) @character
(escape_sequence) @string.escape
(bool_literal) @boolean
(nil_literal) @constant.builtin

;; Functions and methods
(function_declaration
  name: (identifier) @function)

(method_spec
  name: (identifier) @function.method)

(call_expression
  function: (identifier) @function.call)

(call_expression
  function: (selector_expression
    field: (identifier) @function.method.call))

;; Variables, parameters and fields
(parameter_declaration
  name: (identifier) @variable.parameter)

(field_declaration
  name: (identifier) @variable.member)

(selector_expression
  field: (identifier) @variable.member)

(short_var_declaration 
  left: (identifier_list
    (identifier) @variable))

(var_spec 
  name: (identifier) @variable)

(const_spec
  name: (identifier) @constant)

(identifier) @variable

;; Operators
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"!" @operator
"<" @operator
">" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"++" @operator
"--" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"%=" @operator
"&=" @operator
"|=" @operator
"^=" @operator
"<<" @operator
">>" @operator
"==" @operator
"!=" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
":=" @operator
"<-" @operator
"..." @operator

;; Punctuation
"," @punctuation.delimiter
".". @punctuation.delimiter
":" @punctuation.delimiter
";" @punctuation.delimiter

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

;; Comments
(line_comment) @comment
(block_comment) @comment