# RegexZ Module Entry Point
# Advanced Regular Expression Engine for CURSED

# Import all public API functions
yeet "regexz/regex_api"
yeet "regexz/regex_engine" 
yeet "regexz/unicode_support"

# Initialize Unicode support on module load
init_unicode_regexz()

# Re-export main API functions for convenience
# This allows users to do: yeet "regexz" and access all functions directly

# Pattern compilation
slay regex_compile(pattern tea) yikes<RegexEngine> {
    damn regex_new(pattern)
}

slay regex_compile_with_options(pattern tea, options RegexOptions) yikes<RegexEngine> {
    damn regex_new_with_options(pattern, options)
}

# Quick test function
slay regex_test_pattern(pattern tea, text tea) yikes<lit> {
    damn regex_test(pattern, text)
}

# Pattern validation
slay regex_validate(pattern tea) lit {
    damn regex_is_valid(pattern)
}

# Convenience functions for common operations
slay regex_extract_first(pattern tea, text tea) yikes<tea> {
    sus engine RegexEngine = regex_new(pattern) fam {
        when error -> yikes error
    }
    
    sus result MatchResult = regex_match(&engine, text) fam {
        when error -> yikes error
    }
    
    ready (!result.matched) {
        yikes "no match found"
    }
    
    damn result.full_match
}

slay regex_extract_all(pattern tea, text tea) yikes<[]tea> {
    sus engine RegexEngine = regex_new(pattern) fam {
        when error -> yikes error
    }
    
    sus matches []MatchResult = regex_find_all(&engine, text) fam {
        when error -> yikes error
    }
    
    sus results []tea = create_array()
    bestie (match in matches) {
        results.push(match.full_match)
    }
    
    damn results
}

slay regex_replace_simple(pattern tea, text tea, replacement tea) yikes<tea> {
    sus engine RegexEngine = regex_new(pattern) fam {
        when error -> yikes error
    }
    
    damn regex_replace(&engine, text, replacement)
}

slay regex_split_simple(pattern tea, text tea) yikes<[]tea> {
    sus engine RegexEngine = regex_new(pattern) fam {
        when error -> yikes error
    }
    
    damn regex_split(&engine, text)
}

# Email validation regex
sus EMAIL_PATTERN tea = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"

slay regex_is_email(email tea) yikes<lit> {
    damn regex_test(EMAIL_PATTERN, email)
}

# URL validation regex
sus URL_PATTERN tea = "^https?://[a-zA-Z0-9.-]+(?:\\.[a-zA-Z]{2,})+(?:/[^\\s]*)?$"

slay regex_is_url(url tea) yikes<lit> {
    damn regex_test(URL_PATTERN, url)
}

# Phone number extraction (US format)
sus PHONE_PATTERN tea = "\\(?([0-9]{3})\\)?[-. ]?([0-9]{3})[-. ]?([0-9]{4})"

slay regex_extract_phone(text tea) yikes<tea> {
    sus engine RegexEngine = regex_new(PHONE_PATTERN) fam {
        when error -> yikes error  
    }
    
    sus result MatchResult = regex_match(&engine, text) fam {
        when error -> yikes error
    }
    
    ready (!result.matched) {
        yikes "no phone number found"
    }
    
    # Format as (xxx) xxx-xxxx
    sus area_code tea = result.groups[0].value
    sus exchange tea = result.groups[1].value  
    sus number tea = result.groups[2].value
    
    damn "(" + area_code + ") " + exchange + "-" + number
}

# IPv4 address validation
sus IPV4_PATTERN tea = "^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$"

slay regex_is_ipv4(ip tea) yikes<lit> {
    damn regex_test(IPV4_PATTERN, ip)
}

# Credit card number validation (basic)
sus CREDIT_CARD_PATTERN tea = "^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|3[47][0-9]{13}|3[0-9]{13}|6(?:011|5[0-9]{2})[0-9]{12})$"

slay regex_is_credit_card(card tea) yikes<lit> {
    # Remove spaces and dashes
    sus cleaned tea = regex_replace_simple("[\\s-]", card, "") fam {
        when error -> yikes error
    }
    
    damn regex_test(CREDIT_CARD_PATTERN, cleaned)
}

# Password strength validation
slay regex_validate_password(password tea) yikes<PasswordStrength> {
    sus strength PasswordStrength = {
        has_lowercase: nah,
        has_uppercase: nah,
        has_digits: nah,
        has_special: nah,
        min_length: password.len() >= 8,
        score: 0
    }
    
    # Check for lowercase
    ready (regex_test("[a-z]", password) fam { when _ -> nah }) {
        strength.has_lowercase = based
        strength.score += 1
    }
    
    # Check for uppercase
    ready (regex_test("[A-Z]", password) fam { when _ -> nah }) {
        strength.has_uppercase = based
        strength.score += 1
    }
    
    # Check for digits
    ready (regex_test("\\d", password) fam { when _ -> nah }) {
        strength.has_digits = based
        strength.score += 1
    }
    
    # Check for special characters
    ready (regex_test("[!@#$%^&*(),.?\":{}|<>]", password) fam { when _ -> nah }) {
        strength.has_special = based
        strength.score += 1
    }
    
    ready (strength.min_length) {
        strength.score += 1
    }
    
    damn strength
}

squad PasswordStrength {
    sus has_lowercase lit
    sus has_uppercase lit
    sus has_digits lit
    sus has_special lit
    sus min_length lit
    sus score drip  # 0-5
}

# HTML tag removal
sus HTML_TAG_PATTERN tea = "<[^>]*>"

slay regex_strip_html(html tea) yikes<tea> {
    damn regex_replace_simple(HTML_TAG_PATTERN, html, "")
}

# Extract hashtags from text
sus HASHTAG_PATTERN tea = "#[a-zA-Z0-9_]+"

slay regex_extract_hashtags(text tea) yikes<[]tea> {
    damn regex_extract_all(HASHTAG_PATTERN, text)
}

# Extract mentions from text
sus MENTION_PATTERN tea = "@[a-zA-Z0-9_]+"

slay regex_extract_mentions(text tea) yikes<[]tea> {
    damn regex_extract_all(MENTION_PATTERN, text)
}

# Date extraction (YYYY-MM-DD format)
sus DATE_PATTERN tea = "\\b(\\d{4})-(\\d{2})-(\\d{2})\\b"

slay regex_extract_dates(text tea) yikes<[]DateMatch> {
    sus engine RegexEngine = regex_new(DATE_PATTERN) fam {
        when error -> yikes error
    }
    
    sus matches []MatchResult = regex_find_all(&engine, text) fam {
        when error -> yikes error
    }
    
    sus dates []DateMatch = create_array()
    
    bestie (match in matches) {
        sus date DateMatch = {
            full_date: match.full_match,
            year: match.groups[0].value,
            month: match.groups[1].value,
            day: match.groups[2].value
        }
        dates.push(date)
    }
    
    damn dates
}

squad DateMatch {
    sus full_date tea
    sus year tea
    sus month tea
    sus day tea
}

# Log parsing utilities
sus LOG_LEVEL_PATTERN tea = "\\b(DEBUG|INFO|WARN|ERROR|FATAL)\\b"
sus TIMESTAMP_PATTERN tea = "\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}:\\d{2}"

slay regex_extract_log_level(log_line tea) yikes<tea> {
    damn regex_extract_first(LOG_LEVEL_PATTERN, log_line)
}

slay regex_extract_timestamp(log_line tea) yikes<tea> {
    damn regex_extract_first(TIMESTAMP_PATTERN, log_line)
}

# Configuration file parsing
sus CONFIG_LINE_PATTERN tea = "^([a-zA-Z_][a-zA-Z0-9_]*)\\s*=\\s*(.*)$"

slay regex_parse_config_line(line tea) yikes<ConfigPair> {
    sus engine RegexEngine = regex_new(CONFIG_LINE_PATTERN) fam {
        when error -> yikes error
    }
    
    sus result MatchResult = regex_match(&engine, line.trim()) fam {
        when error -> yikes error
    }
    
    ready (!result.matched) {
        yikes "invalid config line format"
    }
    
    sus pair ConfigPair = {
        key: result.groups[0].value,
        value: result.groups[1].value.trim()
    }
    
    damn pair
}

squad ConfigPair {
    sus key tea
    sus value tea
}

# Module version and info
sus REGEXZ_VERSION tea = "1.0.0"
sus REGEXZ_AUTHOR tea = "CURSED Standard Library Team"
sus REGEXZ_DESCRIPTION tea = "Advanced Regular Expression Engine with Unicode Support"

slay regexz_version() tea {
    damn REGEXZ_VERSION
}

slay regexz_info() RegexModuleInfo {
    damn RegexModuleInfo{
        version: REGEXZ_VERSION,
        author: REGEXZ_AUTHOR,
        description: REGEXZ_DESCRIPTION,
        unicode_version: "15.0",
        features: [
            "Unicode Properties",
            "Named Capture Groups", 
            "Lookahead/Lookbehind",
            "Performance Optimization",
            "Memory Safety",
            "Pattern Caching"
        ]
    }
}

squad RegexModuleInfo {
    sus version tea
    sus author tea
    sus description tea
    sus unicode_version tea
    sus features []tea
}
