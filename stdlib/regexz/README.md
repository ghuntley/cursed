# CURSED Regex Module (regexz)

A pure CURSED implementation of regular expression and pattern matching functionality for server-side text processing.

## Features

- ✅ **Pattern Matching**: Basic regex pattern matching without external dependencies
- ✅ **Text Validation**: Email, URL, phone number, date format validation  
- ✅ **Text Extraction**: Extract emails, URLs, numbers, and words from text
- ✅ **String Processing**: Find, replace, split, and count pattern matches
- ✅ **Format Validation**: IP addresses, MAC addresses, credit cards
- ✅ **Character Classification**: Alpha, numeric, alphanumeric detection
- ✅ **Utility Functions**: Escape regex characters, text manipulation

## Quick Start

```cursed
yeet "regexz"

# Basic pattern matching
ready (regex_match("hello@world.com", "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")) {
    vibez.spill("Valid email format!")
}

# Find and replace
sus cleaned tea = regex_replace_all("hello world hello", "hello", "hi")
vibez.spill(cleaned)  # "hi world hi"

# Extract patterns
sus emails []tea = regex_extract_emails("Contact info@company.com or support@help.org")
bestie i := 0; i < len(emails); i++ {
    vibez.spill("Found email:", emails[i])
}
```

## Core Functions

### Pattern Matching
- `regex_match(text, pattern)` - Test if text matches pattern
- `regex_find(text, pattern)` - Find first occurrence position
- `regex_find_all(text, pattern)` - Find all occurrence positions

### Text Processing
- `regex_replace(text, pattern, replacement)` - Replace first match
- `regex_replace_all(text, pattern, replacement)` - Replace all matches
- `regex_split(text, pattern)` - Split text by pattern
- `regex_count_matches(text, pattern)` - Count pattern occurrences

### Validation Functions
- `is_email_format(text)` - Validate email format
- `is_phone_format(text)` - Validate phone number (XXX-XXX-XXXX)
- `is_date_format(text)` - Validate date format (YYYY-MM-DD)
- `validate_ip_address(ip)` - Validate IPv4 address
- `validate_mac_address(mac)` - Validate MAC address
- `validate_credit_card(card)` - Basic credit card validation

### Text Extraction
- `regex_extract_emails(text)` - Extract all email addresses
- `regex_extract_urls(text)` - Extract all HTTP/HTTPS URLs
- `regex_extract_numbers(text)` - Extract all number sequences
- `regex_extract_words(text)` - Extract all word sequences

### Character Classification
- `is_alpha_only(text)` - Check if text contains only letters
- `is_numeric_only(text)` - Check if text contains only digits
- `is_alphanumeric_only(text)` - Check if text contains only letters/digits

## Supported Patterns

### Built-in Patterns
- `".*"` - Match everything
- `"^[a-zA-Z]+$"` - Only letters
- `"^[0-9]+$"` - Only digits
- `"^[a-zA-Z0-9]+$"` - Only alphanumeric
- `"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"` - Email format
- `"^https?://.*"` - HTTP/HTTPS URLs
- `"^[0-9]{3}-[0-9]{3}-[0-9]{4}$"` - Phone numbers
- `"^[0-9]{4}-[0-9]{2}-[0-9]{2}$"` - Date format

### String Patterns
For patterns not in the built-in list, the module falls back to substring matching.

## Usage Examples

### Email Validation
```cursed
yeet "regexz"

ready (is_email_format("user@example.com")) {
    vibez.spill("Valid email address")
}

sus emails []tea = regex_extract_emails("Contact us at info@company.com or support@help.org")
bestie i := 0; i < len(emails); i++ {
    vibez.spill("Email:", emails[i])
}
```

### URL Processing
```cursed
sus text tea = "Visit http://example.com or https://secure.site.com for more info"
sus urls []tea = regex_extract_urls(text)
bestie i := 0; i < len(urls); i++ {
    vibez.spill("URL:", urls[i])
}
```

### Phone Number Validation
```cursed
ready (is_phone_format("555-123-4567")) {
    vibez.spill("Valid phone number format")
}
```

### Text Cleaning
```cursed
sus dirty_text tea = "Remove all FOO instances from FOO this text FOO"
sus clean_text tea = regex_replace_all(dirty_text, "FOO", "")
vibez.spill(clean_text)  # "Remove all  instances from  this text "
```

### Data Extraction
```cursed
sus log_line tea = "ERROR 2024-08-10 user@company.com failed login from 192.168.1.100"

sus emails []tea = regex_extract_emails(log_line)
sus numbers []tea = regex_extract_numbers(log_line)

ready (len(emails) > 0) {
    vibez.spill("User:", emails[0])
}

ready (is_date_format("2024-08-10")) {
    vibez.spill("Valid date found in log")
}
```

### Form Validation
```cursed
slay validate_user_input(email tea, phone tea, ip tea) lit {
    ready (!is_email_format(email)) {
        vibez.spill("Invalid email format")
        damn cringe
    }
    
    ready (!is_phone_format(phone)) {
        vibez.spill("Invalid phone format")
        damn cringe
    }
    
    ready (!validate_ip_address(ip)) {
        vibez.spill("Invalid IP address")
        damn cringe
    }
    
    damn based
}
```

## Performance Notes

- This is a simplified regex implementation focusing on common server-side patterns
- For complex regex needs, consider using specialized text processing
- Pattern matching is optimized for the most common use cases
- Built-in patterns are faster than fallback substring matching

## Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed-zig stdlib/regexz/test_regexz.csd
```

The test suite covers:
- Pattern matching functionality
- Email/URL/phone validation
- Text extraction and processing
- Character classification
- IP/MAC address validation
- String replacement and splitting

## Integration

The regexz module integrates well with other CURSED stdlib modules:

- **stringz**: Basic string manipulation functions
- **httpz**: URL validation for HTTP requests
- **filez**: Log file processing and pattern extraction
- **dbz**: Data validation before database operations

## Limitations

- Simplified regex engine (not full PCRE compatibility)
- Limited capture group support
- Basic character class support
- Focused on common server-side use cases

For most web development and server-side text processing needs, this module provides sufficient functionality with zero external dependencies.
