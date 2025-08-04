module.exports = grammar({
  name: 'cursed',
  
  extras: $ => [
    /\s/,
    $.comment,
  ],
  
  conflicts: $ => [
    [$.expression, $.type],
    [$.expression, $.simple_statement],
    [$.primary_expression, $.type],
    [$.type, $.type_literal],
    [$.signature],
    [$.operand, $.qualified_identifier],
    [$.type, $.type_literal, $.composite_literal],
    [$.method_receiver, $.pointer_type],
    [$.type_name, $.identifier_list],
    [$.type, $.tuple_type],
    [$.type_name, $.operand],
    [$.function_type, $.function_literal],
    [$.pointer_type, $.composite_literal],
    [$.unary_expression, $.conditional_expression],
    [$.return_statement],
    [$.break_statement],
    [$.continue_statement],
    [$.operand, $.identifier_list],
    [$.statement, $.simple_statement],
    [$.inc_dec_statement, $.expression_statement],
    [$.unary_expression, $.call_expression],
    [$.unary_expression, $.index_expression, $.slice_expression],
    [$.parameter_declaration, $.type, $.tuple_type],
    [$.parameter_declaration, $.tuple_type],
    [$.element_list],
    [$.binary_expression],
    [$.expression_list],
    [$.type_name, $.tuple_destructure, $.operand],
    [$.simple_statement, $.for_clause],
    [$.method_receiver, $.identifier_list],
    [$.method_receiver, $.parameter_declaration],
    [$.function_declaration],
    [$.method_declaration],
    [$.if_statement, $.operand],
    [$.channel_type],
  ],
  
  rules: {
    // Source file structure
    source_file: $ => seq(
      optional($.package_clause),
      repeat($.import_declaration),
      repeat($.top_level_declaration)
    ),
    
    package_clause: $ => seq(
      'vibe',
      field('name', $.identifier),
      optional(';')
    ),
    
    // Import declarations
    import_declaration: $ => seq(
      'yeet',
      choice(
        $.import_spec,
        seq('(', repeat(seq($.import_spec, optional(';'))), ')')
      ),
      optional(';')
    ),
    
    import_spec: $ => seq(
      optional(choice($.identifier, '.')),
      field('path', $.string_literal)
    ),
    
    // Top-level declarations
    top_level_declaration: $ => choice(
      $.const_declaration,
      $.type_declaration,
      $.var_declaration,
      $.function_declaration,
      $.method_declaration
    ),
    
    // Constants
    const_declaration: $ => seq(
      'facts',
      choice(
        $.const_spec,
        seq('(', repeat(seq($.const_spec, optional(';'))), ')')
      )
    ),
    
    const_spec: $ => seq(
      $.identifier_list,
      optional($.type),
      '=',
      $.expression_list
    ),
    
    // Variables
    var_declaration: $ => seq(
      'sus',
      choice(
        $.var_spec,
        seq('(', repeat(seq($.var_spec, optional(';'))), ')')
      )
    ),
    
    var_spec: $ => seq(
      $.identifier_list,
      choice(
        seq($.type, optional(seq('=', $.expression_list))),
        seq('=', $.expression_list)
      )
    ),
    
    // Types
    type_declaration: $ => seq(
      'be_like',
      choice(
        $.type_spec,
        seq('(', repeat(seq($.type_spec, optional(';'))), ')')
      )
    ),
    
    type_spec: $ => seq(
      field('name', $.identifier),
      field('type', $.type)
    ),
    
    // Functions
    function_declaration: $ => seq(
      'slay',
      field('name', $.identifier),
      optional($.type_parameters),
      $.signature,
      optional($.block)
    ),
    
    method_declaration: $ => seq(
      'slay',
      $.method_receiver,
      field('name', $.identifier),
      optional($.type_parameters),
      $.signature,
      optional($.block)
    ),
    
    method_receiver: $ => seq(
      '(',
      optional($.identifier),
      choice($.type, seq('*', $.type)),
      ')'
    ),
    
    signature: $ => seq(
      $.parameters,
      optional(choice($.type, $.parameters))
    ),
    
    parameters: $ => seq(
      '(',
      optional($.parameter_list),
      ')'
    ),
    
    parameter_list: $ => seq(
      $.parameter_declaration,
      repeat(seq(',', $.parameter_declaration)),
      optional(',')
    ),
    
    parameter_declaration: $ => seq(
      optional($.identifier_list),
      choice($.type, '...', seq('...', $.type))
    ),
    
    // Types
    type: $ => choice(
      $.type_name,
      $.type_literal,
      seq('(', $.type, ')'),
      $.pointer_type,
      $.array_type,
      $.slice_type,
      $.channel_type,
      $.function_type,
      $.interface_type,
      $.map_type,
      $.struct_type,
      $.tuple_type
    ),
    
    type_name: $ => choice(
      $.identifier,
      $.qualified_identifier,
      $.builtin_type
    ),
    
    builtin_type: $ => choice(
      'normie',    // i32
      'smol',      // i8
      'mid',       // i16
      'thicc',     // i64
      'drip',      // f32
      'snack',     // f32
      'meal',      // f64
      'byte',      // u8
      'rune',      // i32
      'extra',     // complex
      'tea',       // string
      'lit',       // bool
      'sip'        // char
    ),
    
    type_literal: $ => choice(
      $.array_type,
      $.struct_type,
      $.pointer_type,
      $.function_type,
      $.interface_type,
      $.slice_type,
      $.map_type,
      $.channel_type,
      $.tuple_type
    ),
    
    // Specific type definitions
    array_type: $ => seq(
      '[',
      field('length', $.expression),
      ']',
      field('element', $.type)
    ),
    
    slice_type: $ => seq(
      '[',
      ']',
      field('element', $.type)
    ),
    
    pointer_type: $ => seq(
      '*',
      field('element', $.type)
    ),
    
    channel_type: $ => choice(
      seq(
        'dm',
        '<',
        field('element', $.type),
        '>',
        optional(seq('[', field('capacity', $.expression), ']'))
      ),
      seq(
        'chan',
        field('element', $.type)
      )
    ),
    
    struct_type: $ => seq(
      'squad',
      '{',
      repeat($.field_declaration),
      '}'
    ),
    
    field_declaration: $ => seq(
      choice(
        seq($.identifier_list, $.type),
        $.embedded_field
      ),
      optional($.tag),
      optional(';')
    ),
    
    embedded_field: $ => seq(
      optional('*'),
      $.type_name
    ),
    
    tag: $ => $.string_literal,
    
    function_type: $ => seq(
      'slay',
      $.signature
    ),
    
    interface_type: $ => seq(
      'vibes',
      '{',
      repeat($.method_spec),
      '}'
    ),
    
    method_spec: $ => seq(
      field('name', $.identifier),
      $.signature,
      optional(';')
    ),
    
    map_type: $ => seq(
      'map',
      '[',
      field('key', $.type),
      ']',
      field('value', $.type)
    ),
    
    tuple_type: $ => seq(
      '(',
      $.type,
      repeat(seq(',', $.type)),
      optional(','),
      ')'
    ),
    
    // Statements
    declaration: $ => choice(
      $.const_declaration,
      $.type_declaration,
      $.var_declaration,
      $.function_declaration,
      $.method_declaration
    ),
    
    statement: $ => choice(
      $.declaration,
      $.simple_statement,
      $.if_statement,
      $.switch_statement,
      $.for_statement,
      $.while_statement,
      $.select_statement,
      $.return_statement,
      $.break_statement,
      $.continue_statement,
      $.defer_statement,
      $.go_statement,
      $.error_statement,
      $.recovery_statement,
      $.block,
      $.expression_statement
    ),
    
    simple_statement: $ => choice(
      $.expression_statement,
      $.assignment_statement,
      $.short_var_declaration,
      $.inc_dec_statement,
      ';'
    ),
    
    // Control flow statements
    if_statement: $ => seq(
      'lowkey',
      optional(seq($.simple_statement, ';')),
      optional('('),
      field('condition', $.expression),
      optional(')'),
      field('consequence', $.block),
      optional(seq(
        'highkey',
        field('alternative', choice($.if_statement, $.block))
      ))
    ),
    
    switch_statement: $ => seq(
      'vibe_check',
      optional(seq($.simple_statement, ';')),
      optional(field('value', $.expression)),
      '{',
      repeat($.expression_case),
      '}'
    ),
    
    expression_case: $ => seq(
      choice(
        seq('mood', $.expression_list),
        'basic'
      ),
      ':',
      repeat($.statement)
    ),
    
    for_statement: $ => seq(
      'bestie',
      optional(choice(
        field('condition', $.expression),
        $.for_clause,
        $.range_clause
      )),
      field('body', $.block)
    ),
    
    for_clause: $ => seq(
      optional(field('init', $.simple_statement)),
      ';',
      optional(field('condition', $.expression)),
      ';',
      optional(field('update', $.simple_statement))
    ),
    
    range_clause: $ => seq(
      optional(choice(
        seq($.expression_list, '='),
        seq($.identifier_list, ':=')
      )),
      'flex',
      field('right', $.expression)
    ),
    
    while_statement: $ => seq(
      'periodt',
      field('condition', $.expression),
      field('body', $.block)
    ),
    
    select_statement: $ => seq(
      'ready',
      '{',
      repeat($.select_case),
      '}'
    ),
    
    select_case: $ => seq(
      choice(
        seq('mood', choice($.send_statement, $.receive_statement)),
        'basic'
      ),
      ':',
      repeat($.statement)
    ),
    
    send_statement: $ => seq(
      field('channel', $.expression),
      '<-',
      field('value', $.expression)
    ),
    
    receive_statement: $ => seq(
      optional(choice(
        seq($.expression_list, '='),
        seq($.identifier_list, ':=')
      )),
      field('channel', $.receive_expression)
    ),
    
    receive_expression: $ => seq(
      '<-',
      field('channel', $.expression)
    ),
    
    // Other statements
    return_statement: $ => seq(
      'damn',
      optional($.expression_list)
    ),
    
    break_statement: $ => seq(
      'ghosted',
      optional(field('label', $.identifier))
    ),
    
    continue_statement: $ => seq(
      'simp',
      optional(field('label', $.identifier))
    ),
    
    defer_statement: $ => seq(
      'defer',
      field('call', $.expression)
    ),
    
    go_statement: $ => seq(
      'stan',
      field('call', $.expression)
    ),
    
    // Error handling statements
    error_statement: $ => seq(
      'yikes',
      field('variable', $.identifier),
      ':=',
      field('value', $.expression)
    ),
    
    recovery_statement: $ => seq(
      'fam',
      field('error_variable', $.identifier),
      field('body', $.block),
      optional(seq(
        'highkey',
        field('recovery_body', $.block)
      ))
    ),
    
    assignment_statement: $ => seq(
      field('left', $.expression_list),
      field('operator', choice('=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '<<=', '>>=')),
      field('right', $.expression_list)
    ),
    
    short_var_declaration: $ => seq(
      field('left', choice(
        $.identifier_list,
        $.tuple_destructure
      )),
      ':=',
      field('right', $.expression_list)
    ),
    
    tuple_destructure: $ => seq(
      '(',
      $.identifier,
      repeat(seq(',', $.identifier)),
      optional(','),
      ')'
    ),
    
    inc_dec_statement: $ => choice(
      seq(field('operand', $.expression), '++'),
      seq(field('operand', $.expression), '--'),
      seq('++', field('operand', $.expression)),
      seq('--', field('operand', $.expression))
    ),
    
    expression_statement: $ => $.expression,
    
    // Expressions
    expression: $ => choice(
      $.unary_expression,
      $.binary_expression,
      $.conditional_expression
    ),
    
    unary_expression: $ => choice(
      $.primary_expression,
      seq(
        field('operator', choice('+', '-', '!', '^', '*', '&', '<-', 'shook')),
        field('operand', $.expression)
      )
    ),
    
    binary_expression: $ => choice(
      ...[
        ['||', 1],
        ['&&', 2],
        ['==', 3], ['!=', 3], ['<', 3], ['<=', 3], ['>', 3], ['>=', 3],
        ['+', 4], ['-', 4], ['|', 4], ['^', 4],
        ['*', 5], ['/', 5], ['%', 5], ['<<', 5], ['>>', 5], ['&', 5], ['&^', 5],
      ].map(([op, precedence]) => 
        prec(precedence, seq(
          field('left', $.expression),
          field('operator', op),
          field('right', $.expression)
        ))
      )
    ),
    
    conditional_expression: $ => prec.right(seq(
      field('condition', $.expression),
      '?',
      field('consequence', $.expression),
      ':',
      field('alternative', $.expression)
    )),
    
    primary_expression: $ => choice(
      $.operand,
      $.selector_expression,
      $.index_expression,
      $.slice_expression,
      $.type_assertion_expression,
      $.call_expression
    ),
    
    operand: $ => choice(
      $.literal,
      $.identifier,
      $.qualified_identifier,
      seq('(', $.expression, ')')
    ),
    
    selector_expression: $ => seq(
      field('operand', $.primary_expression),
      '.',
      field('field', $.identifier)
    ),
    
    index_expression: $ => seq(
      field('operand', $.primary_expression),
      '[',
      field('index', $.expression),
      ']'
    ),
    
    slice_expression: $ => seq(
      field('operand', $.primary_expression),
      '[',
      optional(field('low', $.expression)),
      ':',
      optional(field('high', $.expression)),
      optional(seq(':', field('max', $.expression))),
      ']'
    ),
    
    type_assertion_expression: $ => seq(
      field('operand', $.primary_expression),
      '.',
      '(',
      field('type', $.type),
      ')'
    ),
    
    call_expression: $ => seq(
      field('function', $.primary_expression),
      field('arguments', $.argument_list)
    ),
    
    argument_list: $ => seq(
      '(',
      optional(seq(
        $.expression_list,
        optional(',')
      )),
      ')'
    ),
    
    // Literals
    literal: $ => choice(
      $.basic_literal,
      $.composite_literal,
      $.function_literal
    ),
    
    basic_literal: $ => choice(
      $.int_literal,
      $.float_literal,
      $.string_literal,
      $.char_literal,
      $.bool_literal,
      $.nil_literal,
      $.tuple_literal
    ),
    
    bool_literal: $ => choice('based', 'cringe'),
    nil_literal: $ => 'nah',
    
    tuple_literal: $ => seq(
      '(',
      $.expression,
      repeat1(seq(',', $.expression)),
      optional(','),
      ')'
    ),
    
    composite_literal: $ => seq(
      field('type', choice($.type, $.struct_type, $.array_type, $.slice_type, $.map_type)),
      field('body', $.literal_value)
    ),
    
    literal_value: $ => seq(
      '{',
      optional(seq(
        $.element_list,
        optional(',')
      )),
      '}'
    ),
    
    element_list: $ => seq(
      $.keyed_element,
      repeat(seq(',', $.keyed_element))
    ),
    
    keyed_element: $ => choice(
      $.expression,
      seq(
        field('key', choice($.expression, $.literal_value)),
        ':',
        field('value', $.expression)
      )
    ),
    
    function_literal: $ => seq(
      'slay',
      $.signature,
      $.block
    ),
    
    // Blocks and lists
    block: $ => seq(
      '{',
      repeat($.statement),
      '}'
    ),
    
    identifier_list: $ => seq(
      $.identifier,
      repeat(seq(',', $.identifier))
    ),
    
    expression_list: $ => seq(
      $.expression,
      repeat(seq(',', $.expression))
    ),
    
    type_parameters: $ => seq(
      '[',
      $.type_parameter_list,
      ']'
    ),
    
    type_parameter_list: $ => seq(
      $.type_parameter,
      repeat(seq(',', $.type_parameter))
    ),
    
    type_parameter: $ => seq(
      field('name', $.identifier),
      optional(seq('~', field('constraint', $.type)))
    ),
    
    qualified_identifier: $ => seq(
      field('package', $.identifier),
      '.',
      field('name', $.identifier)
    ),
    
    // Terminals
    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
    
    int_literal: $ => choice(
      /[0-9]+/,
      /0[xX][0-9a-fA-F]+/,
      /0[oO][0-7]+/,
      /0[bB][01]+/
    ),
    
    float_literal: $ => choice(
      /[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?/,
      /[0-9]+[eE][+-]?[0-9]+/,
      /\.[0-9]+([eE][+-]?[0-9]+)?/
    ),
    
    string_literal: $ => choice(
      seq('"', repeat(choice(/[^"\\]/, /\\./)), '"'),
      seq('`', repeat(/[^`]/), '`')
    ),
    
    char_literal: $ => seq(
      "'",
      choice(
        /[^'\\]/,
        /\\./,
        /\\x[0-9a-fA-F]{2}/,
        /\\u[0-9a-fA-F]{4}/,
        /\\U[0-9a-fA-F]{8}/
      ),
      "'"
    ),
    
    comment: $ => choice(
      seq('//', /.*/),
      seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')
    ),
  }
});
