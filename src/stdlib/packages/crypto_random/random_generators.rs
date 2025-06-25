/// Specialized random generators for different use cases
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use super::secure_random::SecureRandom;

/// Random password generator
pub struct PasswordGenerator {
    length: usize,
    include_uppercase: bool,
    include_lowercase: bool,
    include_numbers: bool,
    include_symbols: bool,
    exclude_ambiguous: bool,
    custom_charset: Option<String>,
    secure_rng: SecureRandom,
}

impl PasswordGenerator {
    /// Create new password generator with default settings
    pub fn new() -> AdvancedCryptoResult<Self> {
        Ok(Self {
            length: 16,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: true,
            include_symbols: false,
            exclude_ambiguous: true,
            custom_charset: None,
            secure_rng: SecureRandom::new()?,
        })
    }
    
    /// Set password length
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }
    
    /// Include uppercase letters
    pub fn uppercase(mut self, include: bool) -> Self {
        self.include_uppercase = include;
        self
    }
    
    /// Include lowercase letters
    pub fn lowercase(mut self, include: bool) -> Self {
        self.include_lowercase = include;
        self
    }
    
    /// Include numbers
    pub fn numbers(mut self, include: bool) -> Self {
        self.include_numbers = include;
        self
    }
    
    /// Include symbols
    pub fn symbols(mut self, include: bool) -> Self {
        self.include_symbols = include;
        self
    }
    
    /// Exclude ambiguous characters (0, O, l, 1, etc.)
    pub fn exclude_ambiguous(mut self, exclude: bool) -> Self {
        self.exclude_ambiguous = exclude;
        self
    }
    
    /// Use custom character set
    pub fn custom_charset(mut self, charset: String) -> Self {
        self.custom_charset = Some(charset);
        self
    }
    
    /// Generate password
    pub fn generate(&self) -> AdvancedCryptoResult<String> {
        if self.length == 0 {
            return Ok(String::new());
        }
        
        let charset = if let Some(ref custom) = self.custom_charset {
            custom.clone()
        } else {
            self.build_charset()
        };
        
        if charset.is_empty() {
            return Err("No characters available for password generation".into());
        }
        
        let charset_bytes = charset.as_bytes();
        let mut password = Vec::with_capacity(self.length);
        
        for _ in 0..self.length {
            let index = self.secure_rng.range_u64(0, charset_bytes.len() as u64 - 1)?;
            password.push(charset_bytes[index as usize]);
        }
        
        String::from_utf8(password).map_err(|e| e.to_string().into())
    }
    
    /// Build character set based on settings
    fn build_charset(&self) -> String {
        let mut charset = String::new();
        
        if self.include_lowercase {
            if self.exclude_ambiguous {
                charset.push_str("abcdefghijkmnopqrstuvwxyz"); // excluded: l
            } else {
                charset.push_str("abcdefghijklmnopqrstuvwxyz");
            }
        }
        
        if self.include_uppercase {
            if self.exclude_ambiguous {
                charset.push_str("ABCDEFGHJKLMNPQRSTUVWXYZ"); // excluded: I, O
            } else {
                charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            }
        }
        
        if self.include_numbers {
            if self.exclude_ambiguous {
                charset.push_str("23456789"); // excluded: 0, 1
            } else {
                charset.push_str("0123456789");
            }
        }
        
        if self.include_symbols {
            charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        }
        
        charset
    }
    
    /// Generate multiple passwords
    pub fn generate_batch(&self, count: usize) -> AdvancedCryptoResult<Vec<String>> {
        let mut passwords = Vec::with_capacity(count);
        for _ in 0..count {
            passwords.push(self.generate()?);
        }
        Ok(passwords)
    }
}

/// Random UUID generator
pub struct UuidGenerator {
    secure_rng: SecureRandom,
}

impl UuidGenerator {
    /// Create new UUID generator
    pub fn new() -> AdvancedCryptoResult<Self> {
        Ok(Self {
            secure_rng: SecureRandom::new()?,
        })
    }
    
    /// Generate UUID v4 (random)
    pub fn v4(&self) -> AdvancedCryptoResult<String> {
        let mut bytes = [0u8; 16];
        self.secure_rng.fill_bytes(&mut bytes)?;
        
        // Set version (4) and variant bits
        bytes[6] = (bytes[6] & 0x0f) | 0x40; // Version 4
        bytes[8] = (bytes[8] & 0x3f) | 0x80; // Variant 10
        
        Ok(format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        ))
    }
    
    /// Generate simple UUID without hyphens
    pub fn simple(&self) -> AdvancedCryptoResult<String> {
        let uuid = self.v4()?;
        Ok(uuid.replace("-", ""))
    }
    
    /// Generate short UUID (base62 encoded)
    pub fn short(&self) -> AdvancedCryptoResult<String> {
        let bytes = self.secure_rng.bytes(16)?;
        Ok(self.base62_encode(&bytes))
    }
    
    /// Base62 encoding for short UUIDs
    fn base62_encode(&self, bytes: &[u8]) -> String {
        const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        
        let mut num = 0u128;
        for &byte in bytes {
            num = num * 256 + byte as u128;
        }
        
        if num == 0 {
            return "0".to_string();
        }
        
        let mut result = Vec::new();
        while num > 0 {
            result.push(CHARSET[(num % 62) as usize]);
            num /= 62;
        }
        
        result.reverse();
        String::from_utf8(result).unwrap_or_else(|_| "error".to_string())
    }
    
    /// Generate batch of UUIDs
    pub fn batch_v4(&self, count: usize) -> AdvancedCryptoResult<Vec<String>> {
        let mut uuids = Vec::with_capacity(count);
        for _ in 0..count {
            uuids.push(self.v4()?);
        }
        Ok(uuids)
    }
}

/// Random token generator for API keys, session tokens, etc.
pub struct TokenGenerator {
    length: usize,
    alphabet: String,
    secure_rng: SecureRandom,
}

impl TokenGenerator {
    /// Create new token generator
    pub fn new() -> AdvancedCryptoResult<Self> {
        Ok(Self {
            length: 32,
            alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string(),
            secure_rng: SecureRandom::new()?,
        })
    }
    
    /// Set token length
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }
    
    /// Set custom alphabet
    pub fn alphabet(mut self, alphabet: String) -> Self {
        self.alphabet = alphabet;
        self
    }
    
    /// Use base64url alphabet
    pub fn base64url(mut self) -> Self {
        self.alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_".to_string();
        self
    }
    
    /// Use hexadecimal alphabet
    pub fn hex(mut self) -> Self {
        self.alphabet = "0123456789abcdef".to_string();
        self
    }
    
    /// Use uppercase hexadecimal alphabet
    pub fn hex_upper(mut self) -> Self {
        self.alphabet = "0123456789ABCDEF".to_string();
        self
    }
    
    /// Use alphanumeric alphabet (no special characters)
    pub fn alphanumeric(mut self) -> Self {
        self.alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string();
        self
    }
    
    /// Generate token
    pub fn generate(&self) -> AdvancedCryptoResult<String> {
        if self.length == 0 {
            return Ok(String::new());
        }
        
        if self.alphabet.is_empty() {
            return Err("Empty alphabet for token generation".into());
        }
        
        let alphabet_bytes = self.alphabet.as_bytes();
        let mut token = Vec::with_capacity(self.length);
        
        for _ in 0..self.length {
            let index = self.secure_rng.range_u64(0, alphabet_bytes.len() as u64 - 1)?;
            token.push(alphabet_bytes[index as usize]);
        }
        
        String::from_utf8(token).map_err(|e| e.to_string().into())
    }
    
    /// Generate API key (64 character alphanumeric)
    pub fn api_key(&self) -> AdvancedCryptoResult<String> {
        let generator = TokenGenerator::new()?
            .length(64)
            .alphanumeric();
        generator.generate()
    }
    
    /// Generate session token (32 character base64url)
    pub fn session_token(&self) -> AdvancedCryptoResult<String> {
        let generator = TokenGenerator::new()?
            .length(32)
            .base64url();
        generator.generate()
    }
    
    /// Generate CSRF token (32 character hex)
    pub fn csrf_token(&self) -> AdvancedCryptoResult<String> {
        let generator = TokenGenerator::new()?
            .length(32)
            .hex();
        generator.generate()
    }
    
    /// Generate batch of tokens
    pub fn batch(&self, count: usize) -> AdvancedCryptoResult<Vec<String>> {
        let mut tokens = Vec::with_capacity(count);
        for _ in 0..count {
            tokens.push(self.generate()?);
        }
        Ok(tokens)
    }
}

/// Random data generator for testing and simulation
pub struct DataGenerator {
    secure_rng: SecureRandom,
}

impl DataGenerator {
    /// Create new data generator
    pub fn new() -> AdvancedCryptoResult<Self> {
        Ok(Self {
            secure_rng: SecureRandom::new()?,
        })
    }
    
    /// Generate random email address
    pub fn email(&self) -> AdvancedCryptoResult<String> {
        let domains = ["example.com", "test.org", "demo.net", "sample.io"];
        let domain = self.secure_rng.choose(&domains)?.unwrap();
        
        let username_len = self.secure_rng.range_u32(5, 12)?;
        let username = TokenGenerator::new()?
            .length(username_len as usize)
            .alphabet("abcdefghijklmnopqrstuvwxyz0123456789".to_string())
            .generate()?;
        
        Ok(format!("{}@{}", username, domain))
    }
    
    /// Generate random name
    pub fn name(&self) -> AdvancedCryptoResult<String> {
        let first_names = [
            "Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace", "Henry",
            "Ivy", "Jack", "Kate", "Liam", "Maya", "Noah", "Olivia", "Peter"
        ];
        
        let last_names = [
            "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller",
            "Davis", "Rodriguez", "Martinez", "Hernandez", "Lopez", "Gonzalez"
        ];
        
        let first = self.secure_rng.choose(&first_names)?.unwrap();
        let last = self.secure_rng.choose(&last_names)?.unwrap();
        
        Ok(format!("{} {}", first, last))
    }
    
    /// Generate random phone number
    pub fn phone(&self) -> AdvancedCryptoResult<String> {
        let area_code = self.secure_rng.range_u32(200, 999)?;
        let exchange = self.secure_rng.range_u32(200, 999)?;
        let number = self.secure_rng.range_u32(1000, 9999)?;
        
        Ok(format!("({}) {}-{}", area_code, exchange, number))
    }
    
    /// Generate random IP address
    pub fn ip_address(&self) -> AdvancedCryptoResult<String> {
        let a = self.secure_rng.range_u32(1, 254)?;
        let b = self.secure_rng.range_u32(0, 255)?;
        let c = self.secure_rng.range_u32(0, 255)?;
        let d = self.secure_rng.range_u32(1, 254)?;
        
        Ok(format!("{}.{}.{}.{}", a, b, c, d))
    }
    
    /// Generate random MAC address
    pub fn mac_address(&self) -> AdvancedCryptoResult<String> {
        let bytes = self.secure_rng.bytes(6)?;
        Ok(format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
        ))
    }
    
    /// Generate random credit card number (for testing only - not real)
    pub fn credit_card(&self) -> AdvancedCryptoResult<String> {
        // Generate test credit card number (not real)
        let mut digits = vec![4]; // Start with 4 for Visa
        
        for _ in 0..14 {
            digits.push(self.secure_rng.range_u32(0, 9)? as u8);
        }
        
        // Calculate Luhn checksum
        let checksum = self.luhn_checksum(&digits);
        digits.push(checksum);
        
        let digits_str: String = digits.iter().map(|d| d.to_string()).collect();
        Ok(format!("{} {} {} {}",
            &digits_str[0..4],
            &digits_str[4..8],
            &digits_str[8..12],
            &digits_str[12..16]
        ))
    }
    
    /// Calculate Luhn checksum for credit card validation
    fn luhn_checksum(&self, digits: &[u8]) -> u8 {
        let mut sum = 0;
        let mut alternate = false;
        
        for &digit in digits.iter().rev() {
            let mut n = digit as u32;
            if alternate {
                n *= 2;
                if n > 9 {
                    n = (n / 10) + (n % 10);
                }
            }
            sum += n;
            alternate = !alternate;
        }
        
        ((10 - (sum % 10)) % 10) as u8
    }
    
    /// Generate random date in the past year
    pub fn date(&self) -> AdvancedCryptoResult<String> {
        let days_ago = self.secure_rng.range_u32(0, 365)?;
        let year = 2024;
        let month = self.secure_rng.range_u32(1, 12)?;
        let day = self.secure_rng.range_u32(1, 28)?; // Safe for all months
        
        Ok(format!("{:04}-{:02}-{:02}", year, month, day))
    }
    
    /// Generate random text of specified length
    pub fn text(&self, length: usize) -> AdvancedCryptoResult<String> {
        let words = [
            "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing",
            "elit", "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore",
            "et", "dolore", "magna", "aliqua", "enim", "ad", "minim", "veniam"
        ];
        
        let mut text = String::new();
        
        while text.len() < length {
            if !text.is_empty() {
                text.push(' ');
            }
            
            let word = self.secure_rng.choose(&words)?.unwrap();
            text.push_str(word);
        }
        
        text.truncate(length);
        Ok(text)
    }
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default PasswordGenerator")
    }
}

impl Default for UuidGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default UuidGenerator")
    }
}

impl Default for TokenGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default TokenGenerator")
    }
}

impl Default for DataGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default DataGenerator")
    }
}

/// Convenient functions for common use cases
pub fn generate_password(length: usize) -> AdvancedCryptoResult<String> {
    PasswordGenerator::new()?.length(length).generate()
}

pub fn generate_secure_password() -> AdvancedCryptoResult<String> {
    PasswordGenerator::new()?
        .length(16)
        .uppercase(true)
        .lowercase(true)
        .numbers(true)
        .symbols(true)
        .exclude_ambiguous(true)
        .generate()
}

pub fn generate_uuid() -> AdvancedCryptoResult<String> {
    UuidGenerator::new()?.v4()
}

pub fn generate_token(length: usize) -> AdvancedCryptoResult<String> {
    TokenGenerator::new()?.length(length).generate()
}

pub fn generate_api_key() -> AdvancedCryptoResult<String> {
    TokenGenerator::new()?.api_key()
}

pub fn generate_session_token() -> AdvancedCryptoResult<String> {
    TokenGenerator::new()?.session_token()
}
