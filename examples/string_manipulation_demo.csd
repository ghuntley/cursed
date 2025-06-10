// String Manipulation Demo for CURSED
// This example demonstrates comprehensive string operations

import "stdlib";

fn main() {
    println("=== CURSED String Manipulation Demo ===\n")?;
    
    // Core operations
    demo_core_operations();
    
    // Search and replace
    demo_search_operations();
    
    // Transformations
    demo_transformations();
    
    // Splitting and joining
    demo_splitting_joining();
    
    // Validation
    demo_validation();
    
    // Formatting
    demo_formatting();
}

fn demo_core_operations() {
    println("🔧 Core String Operations:")?;
    
    sus text = "Hello, World! 🦀";
    
    printf("Original: '{}'\n", &[text])?;
    printf("Length: {} characters\n", &[length(text).to_string()])?;
    printf("Reversed: '{}'\n", &[reverse(text)])?;
    printf("Repeated 3x: '{}'\n", &[repeat("CURSED", 3)])?;
    printf("Character at index 7: '{}'\n", &[char_at(text, 7).unwrap_or(' ').to_string()])?;
    printf("Is ASCII: {}\n", &[is_ascii(text).to_string()])?;
    println("")?;
}

fn demo_search_operations() {
    println("🔍 Search and Replace Operations:")?;
    
    sus text = "The quick brown fox jumps over the lazy dog";
    
    printf("Text: '{}'\n", &[text])?;
    printf("Contains 'fox': {}\n", &[contains(text, "fox").to_string()])?;
    printf("Starts with 'The': {}\n", &[starts_with(text, "The").to_string()])?;
    printf("Ends with 'dog': {}\n", &[ends_with(text, "dog").to_string()])?;
    printf("Position of 'fox': {}\n", &[find(text, "fox").unwrap_or(999).to_string()])?;
    printf("Replace 'fox' with 'cat': '{}'\n", &[replace(text, "fox", "cat")])?;
    printf("Count of 'the': {}\n", &[count_occurrences(text, "the").to_string()])?;
    println("")?;
}

fn demo_transformations() {
    println("🎨 String Transformations:")?;
    
    sus text = "  Hello World Programming  ";
    
    printf("Original: '{}'\n", &[text])?;
    printf("Trimmed: '{}'\n", &[trim(text)])?;
    printf("Uppercase: '{}'\n", &[to_uppercase(trim(text))])?;
    printf("Lowercase: '{}'\n", &[to_lowercase(trim(text))])?;
    printf("Title Case: '{}'\n", &[to_title_case(trim(text))])?;
    printf("Camel Case: '{}'\n", &[to_camel_case(trim(text))])?;
    printf("Snake Case: '{}'\n", &[to_snake_case(trim(text))])?;
    printf("Kebab Case: '{}'\n", &[to_kebab_case(trim(text))])?;
    printf("Substring (0,5): '{}'\n", &[substring(trim(text), 0, 5).unwrap_or("ERROR".to_string())])?;
    println("")?;
}

fn demo_splitting_joining() {
    println("✂️ Splitting and Joining:")?;
    
    sus csv_data = "name,age,city,country";
    sus sentence = "The quick brown fox jumps";
    
    printf("CSV: '{}'\n", &[csv_data])?;
    facts csv_parts = split(csv_data, ",");
    printf("Split by comma: {:?}\n", &[format!("{:?}", csv_parts)])?;
    
    printf("Sentence: '{}'\n", &[sentence])?;
    facts words = split_whitespace(sentence);
    printf("Words: {:?}\n", &[format!("{:?}", words)])?;
    printf("Joined with ' | ': '{}'\n", &[join(&words.iter().map(|s| s.as_str()).collect::<Vec<_>>(), " | ")])?;
    
    facts (before, sep, after) = partition(sentence, "brown");
    printf("Partition by 'brown': before='{}', sep='{}', after='{}'\n", &[before, sep, after])?;
    
    facts chunks = chunk(sentence, 5).unwrap_or(vec![]);
    printf("Chunks of 5: {:?}\n", &[format!("{:?}", chunks)])?;
    println("")?;
}

fn demo_validation() {
    println("✅ String Validation:")?;
    
    sus numbers = vec!["123", "123.45", "abc", "-42"];
    sus emails = vec!["user@example.com", "invalid.email", "test@domain.co.uk"];
    sus urls = vec!["https://example.com", "not-a-url", "ftp://files.example.com"];
    
    println("Number validation:")?;
    for num in numbers {
        printf("  '{}': numeric={}, integer={}\n", 
               &[num.to_string(), is_numeric(num).to_string(), is_integer(num).to_string()])?;
    }
    
    println("Email validation:")?;
    for email in emails {
        printf("  '{}': valid={}\n", &[email.to_string(), is_email(email).to_string()])?;
    }
    
    println("URL validation:")?;
    for url in urls {
        printf("  '{}': valid={}\n", &[url.to_string(), is_url(url).to_string()])?;
    }
    
    // Special validations
    printf("Palindrome 'racecar': {}\n", &[is_palindrome("racecar").to_string()])?;
    printf("Balanced '((()))': {}\n", &[has_balanced_parentheses("((()))").to_string()])?;
    printf("Balanced brackets '{{[()]}}': {}\n", &[has_balanced_brackets("{[()]}").to_string()])?;
    println("")?;
}

fn demo_formatting() {
    println("📐 String Formatting:")?;
    
    sus name = "CURSED";
    sus description = "A powerful programming language with Gen Z slang";
    
    printf("Pad left (10): '{}'\n", &[pad_left(name, 10, ' ')])?;
    printf("Pad right (10): '{}'\n", &[pad_right(name, 10, ' ')])?;
    printf("Center (12): '{}'\n", &[center(name, 12, '*')])?;
    
    printf("Truncate (20): '{}'\n", &[truncate(description, 20, true)])?;
    
    // Table formatting
    facts headers = vec!["Language", "Type", "Year"];
    facts row1 = vec!["CURSED", "Systems", "2024"];
    facts row2 = vec!["Rust", "Systems", "2010"];
    facts row3 = vec!["Python", "Interpreted", "1991"];
    
    facts rows = vec![headers, row1, row2, row3];
    facts table = format_table(&rows.iter().map(|row| row.iter().map(|s| s.as_str()).collect()).collect::<Vec<_>>(), " | ").unwrap_or(vec![]);
    
    println("Table formatting:")?;
    for line in table {
        printf("  {}\n", &[line])?;
    }
    
    // Line numbering
    sus code = "fn main() {\n    println(\"Hello\");\n}";
    println("Code with line numbers:")?;
    printf("{}\n", &[add_line_numbers(code, 1, ": ")])?;
    
    // Escaping
    sus html = "<script>alert('xss')</script>";
    printf("HTML escaped: '{}'\n", &[escape_html(html)])?;
    
    sus json_str = "Line 1\nLine 2\t\"quoted\"";
    printf("JSON escaped: '{}'\n", &[escape_json(json_str)])?;
    
    println("")?;
}
