module.exports = grammar({
  name: 'cursed',

  extras: $ => [
    /\s/,
    $.line_comment,
    $.block_comment,
  ],

  word: $ => $.identifier,

  rules: {
    source_file: $ => seq(
      $.package_clause,
      optional(';'),
      repeat(seq($.import_declaration, optional(';'))),
      repeat(seq($.top_level_declaration, optional(';'))),
    ),

    package_clause: $ => seq(
      'vibe',
      $.identifier
    ),

    import_declaration: $ => seq(
      'yeet',
      choice(
        $.import_spec,
        seq('(', repeat(seq($.import_spec, optional(';'))), ')')
      )
    ),

    import_spec: $ => seq(
      optional(choice($.identifier, '.')),
      $.string_literal
    ),

    top_level_declaration: $ => choice(
      $.function_declaration,
      $.type_declaration,
      $.variable_declaration,
      $.constant_declaration,
    ),

    function_declaration: $ => seq(
      'slay',
      $.identifier,
      optional($.type_parameters),
      $.parameter_list,
      optional($.return_type),
      optional($.block)
    ),

    type_parameters: $ => seq(
      '[',
      $.identifier,
      repeat(seq(',', $.identifier)),
      ']'
    ),

    parameter_list: $ => seq(
      '(',
      optional($.parameter_declarations),
      ')'
    ),

    parameter_declarations: $ => seq(
      $.parameter_declaration,
      repeat(seq(',', $.parameter_declaration))
    ),

    parameter_declaration: $ => seq(
      oneOrMore($.identifier),
      optional($.type)
    ),

    return_type: $ => $.type,

    type_declaration: $ => seq(
      'be_like',
      choice(
        $.type_spec,
        seq('(', repeat(seq($.type_spec, optional(';'))), ')')
      )
    ),

    type_spec: $ => seq(
      $.identifier,
      $.type
    ),

    variable_declaration: $ => seq(
      'sus',
      choice(
        $.var_spec,
        seq('(', repeat(seq($.var_spec, optional(';'))), ')')
      )
    ),

    var_spec: $ => choice(
      seq($.identifier_list, $.type, optional(seq('=', $.expression_list))),
      seq($.identifier_list, '=', $.expression_list)
    ),

    constant_declaration: $ => seq(
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

    identifier_list: $ => seq(
      $.identifier,
      repeat(seq(',', $.identifier))
    ),

    expression_list: $ => seq(
      $.expression,
      repeat(seq(',', $.expression))
    ),

    type: $ => choice(
      $.type_name,
      $.pointer_type,
      $.array_type,
      $.struct_type,
      $.interface_type,
      $.slice_type,
      $.map_type,
      $.channel_type,
      $.function_type,
      $.parametrized_type,
    ),

    type_name: $ => choice(
      $.identifier,
      seq($.identifier, '.', $.identifier),
      'lit',     // bool
      'smol',    // int8
      'mid',     // int16
      'normie',  // int32
      'thicc',   // int64
      'snack',   // float32
      'meal',    // float64
      'tea',     // string
      'sip',     // char (rune)
      'byte',    // byte
      'rune'     // alias for int32
    ),

    pointer_type: $ => seq('@', $.type),

    array_type: $ => seq('[', $.expression, ']', $.type),

    slice_type: $ => seq('[]', $.type),

    struct_type: $ => seq(
      'squad',
      '{',
      repeat(seq($.field_declaration, optional(';'))),
      '}'
    ),

    field_declaration: $ => seq(
      $.identifier_list,
      $.type
    ),

    interface_type: $ => seq(
      'collab',
      '{',
      repeat(seq($.method_spec, optional(';'))),
      '}'
    ),

    method_spec: $ => seq(
      $.identifier,
      $.parameter_list,
      optional($.return_type)
    ),

    map_type: $ => seq('tea', '[', $.type, ']', $.type),

    channel_type: $ => seq('dm', '<', $.type, '>'),

    function_type: $ => seq('slay', $.parameter_list, optional($.return_type)),

    parametrized_type: $ => seq(
      $.type_name,
      '[',
      $.type,
      repeat(seq(',', $.type)),
      ']'
    ),

    block: $ => seq(
      '{',
      repeat(seq($.statement, optional(';'))),
      '}'
    ),

    statement: $ => choice(
      $.declaration,
      $.simple_statement,
      $.return_statement,
      $.break_statement,
      $.continue_statement,
      $.block,
      $.if_statement,
      $.switch_statement,
      $.for_statement,
      $.while_statement,
      $.defer_statement,
      $.go_statement,
    ),

    declaration: $ => choice(
      $.variable_declaration,
      $.constant_declaration,
      $.type_declaration,
    ),

    simple_statement: $ => choice(
      $.expression_statement,
      $.assignment_statement,
      $.short_var_declaration,
      $.inc_dec_statement,
      $.send_statement,
    ),

    expression_statement: $ => $.expression,

    assignment_statement: $ => seq(
      $.expression_list,
      choice('=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^='),
      $.expression_list
    ),

    short_var_declaration: $ => seq(
      $.identifier_list,
      ':=',
      $.expression_list
    ),

    inc_dec_statement: $ => seq(
      $.expression,
      choice('++', '--')
    ),

    send_statement: $ => seq(
      $.expression,  // channel
      '<-',
      $.expression   // value
    ),

    return_statement: $ => seq(
      'yolo',
      optional($.expression_list)
    ),

    break_statement: $ => seq(
      'ghosted',
      optional($.identifier)  // label
    ),

    continue_statement: $ => seq(
      'simp',
      optional($.identifier)  // label
    ),

    if_statement: $ => seq(
      'lowkey',
      optional(seq($.simple_statement, ';')),
      $.expression,
      $.block,
      optional(seq(
        'highkey',
        choice($.if_statement, $.block)
      ))
    ),

    switch_statement: $ => choice(
      $.expression_switch_statement,
      $.type_switch_statement
    ),

    expression_switch_statement: $ => seq(
      'vibe_check',
      optional(seq($.simple_statement, ';')),
      optional($.expression),
      '{',
      repeat($.expression_case_clause),
      '}'
    ),

    expression_case_clause: $ => seq(
      $.expression_switch_case,
      ':',
      repeat(seq($.statement, optional(';')))
    ),

    expression_switch_case: $ => choice(
      seq('mood', $.expression_list),
      'basic'
    ),

    type_switch_statement: $ => seq(
      'vibe_check',
      optional(seq($.simple_statement, ';')),
      $.type_switch_guard,
      '{',
      repeat($.type_case_clause),
      '}'
    ),

    type_switch_guard: $ => seq(
      optional(seq($.identifier, ':=')),
      $.expression,
      '.(',
      'be_like',
      ')'
    ),

    type_case_clause: $ => seq(
      $.type_switch_case,
      ':',
      repeat(seq($.statement, optional(';')))
    ),

    type_switch_case: $ => choice(
      seq('mood', choice($.type, $.type_list)),
      'basic'
    ),

    type_list: $ => seq(
      $.type,
      repeat(seq(',', $.type))
    ),

    for_statement: $ => seq(
      'bestie',
      optional(choice(
        $.for_clause,
        $.range_clause,
        $.expression,  // condition
      )),
      $.block
    ),

    for_clause: $ => seq(
      optional($.simple_statement),  // init
      ';',
      optional($.expression),        // condition
      ';',
      optional($.simple_statement)   // post
    ),

    range_clause: $ => seq(
      optional(choice(
        seq($.expression_list, '='),
        seq($.identifier_list, ':=')
      )),
      'flex',
      $.expression
    ),

    while_statement: $ => seq(
      'periodt',
      $.expression,
      $.block
    ),

    defer_statement: $ => seq(
      'later',
      $.expression
    ),

    go_statement: $ => seq(
      'stan',
      $.expression
    ),

    expression: $ => choice(
      $.unary_expression,
      $.binary_expression,
      $.primary_expression,
    ),

    unary_expression: $ => choice(
      seq('+', $.expression),
      seq('-', $.expression),
      seq('!', $.expression),
      seq('^', $.expression),
      seq('*', $.expression),
      seq('&', $.expression),
      seq('<-', $.expression)  // channel receive
    ),

    binary_expression: $ => choice(
      seq($.expression, '+', $.expression),
      seq($.expression, '-', $.expression),
      seq($.expression, '*', $.expression),
      seq($.expression, '/', $.expression),
      seq($.expression, '%', $.expression),
      seq($.expression, '&', $.expression),
      seq($.expression, '|', $.expression),
      seq($.expression, '^', $.expression),
      seq($.expression, '<<', $.expression),
      seq($.expression, '>>', $.expression),
      seq($.expression, '==', $.expression),
      seq($.expression, '!=', $.expression),
      seq($.expression, '<', $.expression),
      seq($.expression, '<=', $.expression),
      seq($.expression, '>', $.expression),
      seq($.expression, '>=', $.expression),
      seq($.expression, '&&', $.expression),
      seq($.expression, '||', $.expression),
    ),

    primary_expression: $ => choice(
      $.operand,
      $.conversion,
      $.selector_expression,
      $.index_expression,
      $.slice_expression,
      $.call_expression,
      $.type_assertion,
    ),

    operand: $ => choice(
      $.literal,
      $.identifier,
      seq($.identifier, '.', $.identifier), // qualified identifier
      seq('(', $.expression, ')')
    ),

    literal: $ => choice(
      $.int_literal,
      $.float_literal,
      $.string_literal,
      $.bool_literal,
      $.nil_literal,
      $.rune_literal,
      $.byte_literal,
      $.composite_literal,
    ),

    int_literal: $ => token(choice(
      /[0-9]+/,            // decimal
      /0[oO][0-7]+/,       // octal
      /0[xX][0-9a-fA-F]+/, // hex
      /0[bB][01]+/         // binary
    )),

    float_literal: $ => token(choice(
      /[0-9]+(\.[0-9]+)?([eE][\+\-]?[0-9]+)?/,
      /\.[0-9]+([eE][\+\-]?[0-9]+)?/
    )),

    string_literal: $ => choice(
      seq('"', repeat(choice(/[^"\\\n]/, $.escape_sequence)), '"'),
      seq('`', /[^`]*/, '`')
    ),

    rune_literal: $ => seq(
      '\'',
      choice(/[^'\\\n]/, $.escape_sequence),
      '\''
    ),

    byte_literal: $ => seq(
      'b\'',
      choice(/[^'\\\n]/, $.escape_sequence),
      '\''
    ),

    escape_sequence: $ => token.immediate(seq(
      '\\',
      choice(
        /[abfnrtv\\'"]/,
        /\d{3}/,
        /x[0-9a-fA-F]{2}/,
        /u[0-9a-fA-F]{4}/,
        /U[0-9a-fA-F]{8}/
      )
    )),

    bool_literal: $ => choice('based', 'sus'),

    nil_literal: $ => 'cap',

    composite_literal: $ => seq(
      optional($.type),
      $.literal_value
    ),

    literal_value: $ => seq(
      '{',
      optional($.element_list),
      '}'
    ),

    element_list: $ => seq(
      $.element,
      repeat(seq(',', $.element)),
      optional(',')
    ),

    element: $ => choice(
      $.expression,
      seq($.key, ':', $.expression)
    ),

    key: $ => choice(
      $.identifier,
      $.expression,
      $.literal_value
    ),

    selector_expression: $ => seq(
      $.expression,
      '.',
      $.identifier
    ),

    index_expression: $ => seq(
      $.expression,
      '[', $.expression, ']'
    ),

    slice_expression: $ => seq(
      $.expression,
      '[',
      optional($.expression),
      ':',
      optional($.expression),
      optional(seq(':', $.expression)),
      ']'
    ),

    call_expression: $ => seq(
      $.expression,
      $.arguments
    ),

    arguments: $ => seq(
      '(',
      optional(seq(
        choice(
          $.expression_list,
          seq($.type, optional(seq(',', $.expression_list)))
        ),
        optional(',')
      )),
      ')'
    ),

    type_assertion: $ => seq(
      $.expression,
      '.(',
      $.type,
      ')'
    ),

    conversion: $ => seq(
      $.type,
      '(',
      $.expression,
      optional(','),
      ')'
    ),

    identifier: $ => /[_a-zA-Z][_a-zA-Z0-9]*/,

    line_comment: $ => token(seq('fr fr', /.*/)),

    block_comment: $ => token(seq(
      'no cap',
      /([^o]|o[^n]|on[^\s]|on\s[^g]|on\sg[^o]|on\sgo[^d]|on\sgod[^\s])*/,
      'on god'
    )),

    oneOrMore: (rule) => seq(rule, repeat(rule))
  }
});

function oneOrMore(rule) {
  return seq(rule, repeat(rule));
}