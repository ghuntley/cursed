use std::collections::HashMap;

// Mock the configuration structures
#[derive(Debug, Clone)]
struct FormatterConfig {
    use_tabs: bool,
    indent_size: usize,
    max_line_length: usize,
    spaces_around_operators: bool,
    max_array_elements_single_line: usize,
    max_map_elements_single_line: usize,
    max_struct_fields_single_line: usize,
    blank_lines_between_statements: usize,
    blank_lines_between_functions: usize,
    group_imports: bool,
    sort_imports: bool,
    format_comments: bool,
    preserve_empty_lines: bool,
    max_empty_lines: usize,
    trailing_comma: bool,
    break_function_parameters: bool,
    break_function_arguments: bool,
    space_before_brace: bool,
    space_inside_parentheses: bool,
    space_inside_brackets: bool,
    space_inside_braces: bool,
    align_assignments: bool,
    align_struct_fields: bool,
    insert_final_newline: bool,
    trim_trailing_whitespace: bool,
}

impl FormatterConfig {
    fn compact() -> Self {
        Self {
            use_tabs: false,
            indent_size: 2,
            max_line_length: 120,
            spaces_around_operators: false,
            max_array_elements_single_line: 10,
            max_map_elements_single_line: 8,
            max_struct_fields_single_line: 8,
            blank_lines_between_statements: 0,
            blank_lines_between_functions: 0,
            group_imports: true,
            sort_imports: true,
            format_comments: false,
            preserve_empty_lines: false,
            max_empty_lines: 0,
            trailing_comma: false,
            break_function_parameters: false,
            break_function_arguments: false,
            space_before_brace: false,
            space_inside_parentheses: false,
            space_inside_brackets: false,
            space_inside_braces: false,
            align_assignments: false,
            align_struct_fields: false,
            insert_final_newline: false,
            trim_trailing_whitespace: true,
        }
    }
}

fn main() {
    let config = FormatterConfig::compact();
    let source = "nah x > 0 {\nvibez.spill(\"positive\")\n}";
    
    println!("Original:");
    println!("{}", source);
    println!();
    
    println!("Config spaces_around_operators: {}", config.spaces_around_operators);
    println!("Config space_before_brace: {}", config.space_before_brace);
    println!();
    
    // The issue is that if we're replacing ">=" with " >= " (with spaces),
    // then "x > 0" would NOT match this pattern, so it shouldn't be changed.
    // But if we're handling single ">" characters, that would be the problem.
    
    let test_line = "nah x > 0 {";
    println!("Test line: '{}'", test_line);
    
    // Let's check what happens when we replace operators
    let mut result = test_line.to_string();
    
    // This is similar to what the formatter does
    let operators = [
        (">=", " >= "),
        (">", " > "),
    ];
    
    for (op, replacement) in &operators {
        if !result.contains(replacement) {
            result = result.replace(op, replacement);
        }
    }
    
    println!("After operator replacement: '{}'", result);
    
    // Now let's check what happens when we remove spaces for compact mode
    if !config.spaces_around_operators {
        // We need to remove spaces around operators
        result = result.replace(" >= ", ">=");
        result = result.replace(" > ", ">");
        result = result.replace(" < ", "<");
        result = result.replace(" == ", "==");
        result = result.replace(" != ", "!=");
        result = result.replace(" <= ", "<=");
    }
    
    println!("After removing spaces for compact: '{}'", result);
    
    // Now handle braces
    if !config.space_before_brace {
        result = result.replace(" {", "{");
    }
    
    println!("After removing brace spaces: '{}'", result);
}
