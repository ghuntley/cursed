/// Common regex patterns library
/// Pre-compiled regex patterns for common use cases
use super::pattern::VibePattern;
use lazy_static::lazy_static;
use crate::error::Error;

lazy_static! {
    /// Email regex pattern - validates common email formats
    pub static ref EMAIL_PATTERN: VibePattern = VibePattern::compile(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).expect("Email pattern should compile");

    /// URL regex pattern - matches HTTP and HTTPS URLs
    pub static ref URL_PATTERN: VibePattern = VibePattern::compile(
        r"^https?://(?:[-\w.])+(?:\.[a-zA-Z]{2,})?(?:/[^\s]*)?$"
    ).expect("URL pattern should compile");

    /// Date pattern - matches YYYY-MM-DD format
    pub static ref DATE_PATTERN: VibePattern = VibePattern::compile(
        r"^\d{4}-\d{2}-\d{2}$"
    ).expect("Date pattern should compile");

    /// Time pattern - matches HH:MM:SS format
    pub static ref TIME_PATTERN: VibePattern = VibePattern::compile(
        r"^\d{2}:\d{2}:\d{2}$"
    ).expect("Time pattern should compile");

    /// Username pattern - alphanumeric with underscores and hyphens
    pub static ref USERNAME_PATTERN: VibePattern = VibePattern::compile(
        r"^[a-zA-Z0-9_-]{3,20}$"
    ).expect("Username pattern should compile");

    /// Password pattern - at least 8 chars with mixed case, digits, and symbols
    pub static ref PASSWORD_PATTERN: VibePattern = VibePattern::compile(
        r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$"
    ).expect("Password pattern should compile");

    /// Phone pattern - US phone number formats
    pub static ref PHONE_PATTERN: VibePattern = VibePattern::compile(
        r"^\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}$"
    ).expect("Phone pattern should compile");

    /// Zip code pattern - US zip code formats (5 digit and 5+4)
    pub static ref ZIP_CODE_PATTERN: VibePattern = VibePattern::compile(
        r"^\d{5}(?:-\d{4})?$"
    ).expect("Zip code pattern should compile");

    /// Hashtag pattern - social media hashtag format
    pub static ref HASHTAG_PATTERN: VibePattern = VibePattern::compile(
        r"#[a-zA-Z0-9_]+(?![a-zA-Z0-9_])"
    ).expect("Hashtag pattern should compile");

    /// Emoji pattern - basic emoji matching (simplified)
    pub static ref EMOJI_PATTERN: VibePattern = VibePattern::compile(
        r"[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]"
    ).expect("Emoji pattern should compile");

    /// IPv4 address pattern
    pub static ref IPV4_PATTERN: VibePattern = VibePattern::compile(
        r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$"
    ).expect("IPv4 pattern should compile");

    /// IPv6 address pattern (simplified)
    pub static ref IPV6_PATTERN: VibePattern = VibePattern::compile(
        r"^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$"
    ).expect("IPv6 pattern should compile");

    /// Credit card pattern (basic - any 4 groups of 4 digits)
    pub static ref CREDIT_CARD_PATTERN: VibePattern = VibePattern::compile(
        r"^\d{4}[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4}$"
    ).expect("Credit card pattern should compile");

    /// Hexadecimal color pattern
    pub static ref HEX_COLOR_PATTERN: VibePattern = VibePattern::compile(
        r"^#[0-9a-fA-F]{6}$"
    ).expect("Hex color pattern should compile");

    /// UUID pattern (version 4)
    pub static ref UUID_PATTERN: VibePattern = VibePattern::compile(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$"
    ).expect("UUID pattern should compile");

    /// HTML tag pattern (basic)
    pub static ref HTML_TAG_PATTERN: VibePattern = VibePattern::compile(
        r#"</?[a-zA-Z][a-zA-Z0-9]*(?:\s+[a-zA-Z-]+(?:=["'][^"']*["'])?)*\s*/?>"#
    ).expect("HTML tag pattern should compile");

    /// JSON string pattern (basic)
    pub static ref JSON_STRING_PATTERN: VibePattern = VibePattern::compile(
        r#""(?:[^"\\]|\\.)*""#
    ).expect("JSON string pattern should compile");

    /// Base64 pattern
    pub static ref BASE64_PATTERN: VibePattern = VibePattern::compile(
        r"^[A-Za-z0-9+/]*={0,2}$"
    ).expect("Base64 pattern should compile");

    /// MAC address pattern
    pub static ref MAC_ADDRESS_PATTERN: VibePattern = VibePattern::compile(
        r"^[0-9a-fA-F]{2}(?:[:-][0-9a-fA-F]{2}){5}$"
    ).expect("MAC address pattern should compile");

    /// Social Security Number pattern (US)
    pub static ref SSN_PATTERN: VibePattern = VibePattern::compile(
        r"^\d{3}-\d{2}-\d{4}$"
    ).expect("SSN pattern should compile");

    /// Currency pattern (USD)
    pub static ref CURRENCY_PATTERN: VibePattern = VibePattern::compile(
        r"^\$?(?:\d{1,3}(?:,\d{3})*|\d+)(?:\.\d{2})?$"
    ).expect("Currency pattern should compile");

    /// Version number pattern (semantic versioning)
    pub static ref VERSION_PATTERN: VibePattern = VibePattern::compile(
        r"^v?(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$"
    ).expect("Version pattern should compile");
}

/// Collection of all common patterns for easy access
pub struct CommonPatterns;

impl CommonPatterns {
    /// Get the email validation pattern
    pub fn email() -> &'static VibePattern {
        &EMAIL_PATTERN
    }

    /// Get the URL validation pattern
    pub fn url() -> &'static VibePattern {
        &URL_PATTERN
    }

    /// Get the date validation pattern
    pub fn date() -> &'static VibePattern {
        &DATE_PATTERN
    }

    /// Get the time validation pattern
    pub fn time() -> &'static VibePattern {
        &TIME_PATTERN
    }

    /// Get the username validation pattern
    pub fn username() -> &'static VibePattern {
        &USERNAME_PATTERN
    }

    /// Get the password validation pattern
    pub fn password() -> &'static VibePattern {
        &PASSWORD_PATTERN
    }

    /// Get the phone number validation pattern
    pub fn phone() -> &'static VibePattern {
        &PHONE_PATTERN
    }

    /// Get the zip code validation pattern
    pub fn zip_code() -> &'static VibePattern {
        &ZIP_CODE_PATTERN
    }

    /// Get the hashtag pattern
    pub fn hashtag() -> &'static VibePattern {
        &HASHTAG_PATTERN
    }

    /// Get the emoji pattern
    pub fn emoji() -> &'static VibePattern {
        &EMOJI_PATTERN
    }

    /// Get the IPv4 address pattern
    pub fn ipv4() -> &'static VibePattern {
        &IPV4_PATTERN
    }

    /// Get the IPv6 address pattern
    pub fn ipv6() -> &'static VibePattern {
        &IPV6_PATTERN
    }

    /// Get the credit card pattern
    pub fn credit_card() -> &'static VibePattern {
        &CREDIT_CARD_PATTERN
    }

    /// Get the hex color pattern
    pub fn hex_color() -> &'static VibePattern {
        &HEX_COLOR_PATTERN
    }

    /// Get the UUID pattern
    pub fn uuid() -> &'static VibePattern {
        &UUID_PATTERN
    }

    /// Get the HTML tag pattern
    pub fn html_tag() -> &'static VibePattern {
        &HTML_TAG_PATTERN
    }

    /// Get the JSON string pattern
    pub fn json_string() -> &'static VibePattern {
        &JSON_STRING_PATTERN
    }

    /// Get the Base64 pattern
    pub fn base64() -> &'static VibePattern {
        &BASE64_PATTERN
    }

    /// Get the MAC address pattern
    pub fn mac_address() -> &'static VibePattern {
        &MAC_ADDRESS_PATTERN
    }

    /// Get the SSN pattern
    pub fn ssn() -> &'static VibePattern {
        &SSN_PATTERN
    }

    /// Get the currency pattern
    pub fn currency() -> &'static VibePattern {
        &CURRENCY_PATTERN
    }

    /// Get the version number pattern
    pub fn version() -> &'static VibePattern {
        &VERSION_PATTERN
    }

    /// Get all pattern names
    pub fn pattern_names() -> Vec<&'static str> {
        vec![
            "email", "url", "date", "time", "username", "password",
            "phone", "zip_code", "hashtag", "emoji", "ipv4", "ipv6",
            "credit_card", "hex_color", "uuid", "html_tag", "json_string",
            "base64", "mac_address", "ssn", "currency", "version"
        ]
    }

    /// Get pattern by name
    pub fn get_pattern(name: &str) -> Option<&'static VibePattern> {
        match name {
            "email" => Some(&EMAIL_PATTERN),
            "url" => Some(&URL_PATTERN),
            "date" => Some(&DATE_PATTERN),
            "time" => Some(&TIME_PATTERN),
            "username" => Some(&USERNAME_PATTERN),
            "password" => Some(&PASSWORD_PATTERN),
            "phone" => Some(&PHONE_PATTERN),
            "zip_code" => Some(&ZIP_CODE_PATTERN),
            "hashtag" => Some(&HASHTAG_PATTERN),
            "emoji" => Some(&EMOJI_PATTERN),
            "ipv4" => Some(&IPV4_PATTERN),
            "ipv6" => Some(&IPV6_PATTERN),
            "credit_card" => Some(&CREDIT_CARD_PATTERN),
            "hex_color" => Some(&HEX_COLOR_PATTERN),
            "uuid" => Some(&UUID_PATTERN),
            "html_tag" => Some(&HTML_TAG_PATTERN),
            "json_string" => Some(&JSON_STRING_PATTERN),
            "base64" => Some(&BASE64_PATTERN),
            "mac_address" => Some(&MAC_ADDRESS_PATTERN),
            "ssn" => Some(&SSN_PATTERN),
            "currency" => Some(&CURRENCY_PATTERN),
            "version" => Some(&VERSION_PATTERN),
            _ => None,
        }
    }

    /// Test a string against multiple common patterns
    pub fn test_multiple(s: &str, pattern_names: &[&str]) -> Vec<(String, bool)> {
        pattern_names.iter()
            .map(|name| {
                let matches = Self::get_pattern(name)
                    .map(|p| p.match_string(s))
                    .unwrap_or(false);
                (name.to_string(), matches)
            })
            .collect()
    }

    /// Find which common patterns match a string
    pub fn find_matching_patterns(s: &str) -> Vec<String> {
        Self::pattern_names()
            .into_iter()
            .filter(|name| {
                Self::get_pattern(name)
                    .map(|p| p.match_string(s))
                    .unwrap_or(false)
            })
            .map(|name| name.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_pattern() {
        assert!(EMAIL_PATTERN.match_string("user@example.com"));
        assert!(EMAIL_PATTERN.match_string("test.email+tag@domain.org"));
        assert!(!EMAIL_PATTERN.match_string("invalid-email"));
        assert!(!EMAIL_PATTERN.match_string("@domain.com"));
    }

    #[test]
    fn test_url_pattern() {
        assert!(URL_PATTERN.match_string("https://example.com"));
        assert!(URL_PATTERN.match_string("http://test.org/path"));
        assert!(!URL_PATTERN.match_string("ftp://example.com"));
        assert!(!URL_PATTERN.match_string("not-a-url"));
    }

    #[test]
    fn test_phone_pattern() {
        assert!(PHONE_PATTERN.match_string("(555) 123-4567"));
        assert!(PHONE_PATTERN.match_string("555-123-4567"));
        assert!(PHONE_PATTERN.match_string("555.123.4567"));
        assert!(PHONE_PATTERN.match_string("5551234567"));
        assert!(!PHONE_PATTERN.match_string("555-123-456"));
    }

    #[test]
    fn test_date_time_patterns() {
        assert!(DATE_PATTERN.match_string("2023-12-25"));
        assert!(!DATE_PATTERN.match_string("25-12-2023"));
        assert!(!DATE_PATTERN.match_string("2023/12/25"));

        assert!(TIME_PATTERN.match_string("14:30:45"));
        assert!(!TIME_PATTERN.match_string("2:30:45"));
        assert!(!TIME_PATTERN.match_string("14:30"));
    }

    #[test]
    fn test_password_pattern() {
        assert!(PASSWORD_PATTERN.match_string("Password123!"));
        assert!(PASSWORD_PATTERN.match_string("SecureP@ss1"));
        assert!(!PASSWORD_PATTERN.match_string("password"));
        assert!(!PASSWORD_PATTERN.match_string("PASSWORD"));
        assert!(!PASSWORD_PATTERN.match_string("Pass123"));
    }

    #[test]
    fn test_ip_patterns() {
        assert!(IPV4_PATTERN.match_string("192.168.1.1"));
        assert!(IPV4_PATTERN.match_string("0.0.0.0"));
        assert!(IPV4_PATTERN.match_string("255.255.255.255"));
        assert!(!IPV4_PATTERN.match_string("256.1.1.1"));
        assert!(!IPV4_PATTERN.match_string("192.168.1"));

        assert!(IPV6_PATTERN.match_string("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
        assert!(!IPV6_PATTERN.match_string("192.168.1.1"));
    }

    #[test]
    fn test_common_patterns_access() {
        assert!(CommonPatterns::email().match_string("test@example.com"));
        assert!(CommonPatterns::phone().match_string("555-123-4567"));
        
        let pattern = CommonPatterns::get_pattern("email");
        assert!(pattern.is_some());
        assert!(pattern.unwrap().match_string("test@example.com"));

        let pattern = CommonPatterns::get_pattern("nonexistent");
        assert!(pattern.is_none());
    }

    #[test]
    fn test_pattern_names() {
        let names = CommonPatterns::pattern_names();
        assert!(names.contains(&"email"));
        assert!(names.contains(&"phone"));
        assert!(names.contains(&"url"));
        assert!(names.len() > 10);
    }

    #[test]
    fn test_multiple_pattern_testing() {
        let results = CommonPatterns::test_multiple("test@example.com", &["email", "phone", "url"]);
        assert_eq!(results.len(), 3);
        
        // Should match email but not phone or url
        let email_result = results.iter().find(|(name, _)| name == "email");
        assert_eq!(email_result, Some(&("email".to_string(), true)));
        
        let phone_result = results.iter().find(|(name, _)| name == "phone");
        assert_eq!(phone_result, Some(&("phone".to_string(), false)));
    }

    #[test]
    fn test_find_matching_patterns() {
        let matches = CommonPatterns::find_matching_patterns("test@example.com");
        assert!(matches.contains(&"email".to_string()));
        
        let matches = CommonPatterns::find_matching_patterns("https://example.com");
        assert!(matches.contains(&"url".to_string()));

        let matches = CommonPatterns::find_matching_patterns("not-matching-anything-special");
        assert!(matches.is_empty() || matches.len() < 3); // Should match very few if any
    }

    #[test]
    fn test_special_patterns() {
        assert!(UUID_PATTERN.match_string("123e4567-e89b-42d3-a456-426614174000"));
        assert!(!UUID_PATTERN.match_string("not-a-uuid"));

        assert!(HEX_COLOR_PATTERN.match_string("#FF5733"));
        assert!(HEX_COLOR_PATTERN.match_string("#000000"));
        assert!(!HEX_COLOR_PATTERN.match_string("#GG5733"));

        assert!(MAC_ADDRESS_PATTERN.match_string("00:1B:63:84:45:E6"));
        assert!(MAC_ADDRESS_PATTERN.match_string("00-1B-63-84-45-E6"));
        assert!(!MAC_ADDRESS_PATTERN.match_string("00:1B:63:84:45"));
    }
}
