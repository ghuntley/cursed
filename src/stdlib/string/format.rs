/// String formatting and padding utilities
use super::{StringError, StringResult};

/// Pad string on the left with specified character to reach target width
pub fn pad_left(s: &str, width: usize, pad_char: char) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        return s.to_string();
    }
    
    let padding = pad_char.to_string().repeat(width - char_count);
    format!("{}{}", padding, s)
}

/// Pad string on the right with specified character to reach target width
pub fn pad_right(s: &str, width: usize, pad_char: char) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        return s.to_string();
    }
    
    let padding = pad_char.to_string().repeat(width - char_count);
    format!("{}{}", s, padding)
}

/// Center string with specified padding character to reach target width
pub fn center(s: &str, width: usize, pad_char: char) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        return s.to_string();
    }
    
    let total_padding = width - char_count;
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;
    
    let left_pad = pad_char.to_string().repeat(left_padding);
    let right_pad = pad_char.to_string().repeat(right_padding);
    
    format!("{}{}{}", left_pad, s, right_pad)
}

/// Truncate string to specified length, optionally adding ellipsis
pub fn truncate(s: &str, max_length: usize, add_ellipsis: bool) -> String {
    let chars: Vec<char> = s.chars().collect();
    let char_count = chars.len();
    
    if char_count <= max_length {
        return s.to_string();
    }
    
    if add_ellipsis && max_length >= 3 {
        let truncated: String = chars[..max_length - 3].iter().collect();
        format!("{}...", truncated)
    } else {
        chars[..max_length].iter().collect()
    }
}

/// Wrap text to specified line width
pub fn wrap_text(s: &str, width: usize) -> StringResult<Vec<String>> {
    if width == 0 {
        return Err(StringError::InvalidParameter {
            param: "width".to_string(),
            value: "cannot be zero".to_string(),
        });
    }
    
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_length = 0;
    
    for word in s.split_whitespace() {
        let word_length = word.chars().count();
        
        // If adding this word would exceed the width, start a new line
        if current_length > 0 && current_length + 1 + word_length > width {
            lines.push(current_line);
            current_line = word.to_string();
            current_length = word_length;
        } else {
            if current_length > 0 {
                current_line.push(' ');
                current_length += 1;
            }
            current_line.push_str(word);
            current_length += word_length;
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    Ok(lines)
}

/// Format string with fixed width columns
pub fn format_columns(rows: &[Vec<&str>], column_widths: &[usize], separator: &str) -> StringResult<Vec<String>> {
    let mut result = Vec::new();
    
    for row in rows {
        if row.len() != column_widths.len() {
            return Err(StringError::InvalidParameter {
                param: "row".to_string(),
                value: format!("Expected {} columns, got {}", column_widths.len(), row.len()),
            });
        }
        
        let formatted_cells: Vec<String> = row.iter()
            .enumerate()
            .map(|(i, &cell)| pad_right(cell, column_widths[i], ' '))
            .collect();
        
        result.push(formatted_cells.join(separator));
    }
    
    Ok(result)
}

/// Auto-detect column widths for table formatting
pub fn auto_detect_column_widths(rows: &[Vec<&str>]) -> Vec<usize> {
    if rows.is_empty() {
        return Vec::new();
    }
    
    let num_cols = rows[0].len();
    let mut widths = vec![0; num_cols];
    
    for row in rows {
        for (i, &cell) in row.iter().enumerate() {
            if i < widths.len() {
                let cell_width = cell.chars().count();
                widths[i] = widths[i].max(cell_width);
            }
        }
    }
    
    widths
}

/// Format table with auto-detected column widths
pub fn format_table(rows: &[Vec<&str>], separator: &str) -> StringResult<Vec<String>> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    
    let widths = auto_detect_column_widths(rows);
    format_columns(rows, &widths, separator)
}

/// Add line numbers to text
pub fn add_line_numbers(s: &str, start_number: usize, separator: &str) -> String {
    let lines: Vec<&str> = s.split("\n").collect();
    let max_digits = (start_number + lines.len()).to_string().len();
    
    lines.iter()
        .enumerate()
        .map(|(i, &line)| {
            let line_num = start_number + i;
            format!("{:width$}{}{}", line_num, separator, line, width = max_digits)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Indent all lines by specified amount
pub fn indent_lines(s: &str, indent: usize, indent_char: char) -> String {
    let indent_str = indent_char.to_string().repeat(indent);
    s.split("\n")
        .map(|line| format!("{}{}", indent_str, line))
        .collect::<Vec<String>>()
        .join("\n")
}

/// Remove common indentation from all lines
pub fn dedent(s: &str) -> String {
    let lines: Vec<&str> = s.split("\n").collect();
    if lines.is_empty() {
        return String::new();
    }
    
    // Find minimum indentation (ignoring empty lines)
    let min_indent = lines.iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|&c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);
    
    lines.iter()
        .map(|line| {
            if line.trim().is_empty() {
                line.to_string()
            } else {
                line.chars().skip(min_indent).collect()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Escape special characters for various formats
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Escape characters for JSON strings
pub fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escape characters for CSV fields
pub fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_left() {
        assert_eq!(pad_left("hello", 8, ' '), "   hello");
        assert_eq!(pad_left("hello", 5, ' '), "hello");
        assert_eq!(pad_left("hello", 3, ' '), "hello");
        assert_eq!(pad_left("hi", 5, '*'), "***hi");
    }

    #[test]
    fn test_pad_right() {
        assert_eq!(pad_right("hello", 8, ' '), "hello   ");
        assert_eq!(pad_right("hello", 5, ' '), "hello");
        assert_eq!(pad_right("hello", 3, ' '), "hello");
        assert_eq!(pad_right("hi", 5, '*'), "hi***");
    }

    #[test]
    fn test_center() {
        assert_eq!(center("hello", 9, ' '), "  hello  ");
        assert_eq!(center("hello", 8, ' '), " hello  ");
        assert_eq!(center("hello", 5, ' '), "hello");
        assert_eq!(center("hi", 6, '*'), "**hi**");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello world", 5, false), "hello");
        assert_eq!(truncate("hello world", 8, true), "hello...");
        assert_eq!(truncate("hello", 10, true), "hello");
        assert_eq!(truncate("hello world", 2, true), "he");
    }

    #[test]
    fn test_wrap_text() {
        let result = wrap_text("hello world foo bar", 10).unwrap();
        assert_eq!(result, vec!["hello", "world foo", "bar"]);
        
        let result = wrap_text("short", 10).unwrap();
        assert_eq!(result, vec!["short"]);
        
        assert!(wrap_text("hello", 0).is_err());
    }

    #[test]
    fn test_format_columns() {
        let rows = vec![
            vec!["Name", "Age", "City"],
            vec!["Alice", "30", "New York"],
            vec!["Bob", "25", "Los Angeles"],
        ];
        let widths = vec![8, 5, 12];
        let result = format_columns(&rows, &widths, " | ").unwrap();
        
        assert_eq!(result[0], "Name     | Age   | City        ");
        assert_eq!(result[1], "Alice    | 30    | New York    ");
        assert_eq!(result[2], "Bob      | 25    | Los Angeles ");
    }

    #[test]
    fn test_auto_detect_column_widths() {
        let rows = vec![
            vec!["Name", "Age", "City"],
            vec!["Alice", "30", "New York"],
            vec!["Bob", "25", "Los Angeles"],
        ];
        let widths = auto_detect_column_widths(&rows);
        assert_eq!(widths, vec![5, 3, 11]);
    }

    #[test]
    fn test_add_line_numbers() {
        let text = "hello\nworld\nfoo";
        let result = add_line_numbers(text, 1, ": ");
        assert_eq!(result, "1: hello\n2: world\n3: foo");
        
        let result = add_line_numbers(text, 10, " | ");
        assert_eq!(result, "10 | hello\n11 | world\n12 | foo");
    }

    #[test]
    fn test_indent_lines() {
        let text = "hello\nworld";
        assert_eq!(indent_lines(text, 2, ' '), "  hello\n  world");
        assert_eq!(indent_lines(text, 1, '\t'), "\thello\n\tworld");
    }

    #[test]
    fn test_dedent() {
        let text = "    hello\n    world\n        nested";
        assert_eq!(dedent(text), "hello\nworld\n    nested");
        
        let text = "no indent\n  some indent";
        assert_eq!(dedent(text), "no indent\n  some indent");
    }

    #[test]
    fn test_escape_functions() {
        assert_eq!(escape_html("<script>alert('xss')</script>"), 
                  "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
        
        assert_eq!(escape_json("hello\n\"world\""), "hello\\n\\\"world\\\"");
        
        assert_eq!(escape_csv("hello,world"), "\"hello,world\"");
        assert_eq!(escape_csv("hello \"world\""), "\"hello \"\"world\"\"");
        assert_eq!(escape_csv("simple"), "simple");
    }

    #[test]
    fn test_unicode_formatting() {
        assert_eq!(pad_left("café", 6, ' '), "  café");
        assert_eq!(center("🦀", 5, '*'), "**🦀**");
        assert_eq!(truncate("café world", 4, false), "café");
    }
}
