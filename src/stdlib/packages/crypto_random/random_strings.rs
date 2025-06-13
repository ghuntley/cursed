/// Random string generation with various character sets and formats
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use super::secure_random::SecureRandom;

/// Random string generator with customizable character sets
pub struct RandomStrings {
    secure_rng: SecureRandom,
}

/// Character set presets
pub enum CharSet {
    Alphabetic,         // a-z, A-Z
    Alphanumeric,       // a-z, A-Z, 0-9
    AlphanumericLower,  // a-z, 0-9
    AlphanumericUpper,  // A-Z, 0-9
    Numeric,            // 0-9
    Hexadecimal,        // 0-9, a-f
    HexadecimalUpper,   // 0-9, A-F
    Base64,             // a-z, A-Z, 0-9, +, /
    Base64Url,          // a-z, A-Z, 0-9, -, _
    Printable,          // All printable ASCII characters
    PrintableNoQuotes,  // Printable ASCII without quotes
    Symbols,            // Common symbols
    SafeSymbols,        // Safe symbols (no quotes or slashes)
    Custom(String),     // Custom character set
}

impl CharSet {
    /// Get the character set as a string
    pub fn as_str(&self) -> &str {
        match self {
            CharSet::Alphabetic => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            CharSet::Alphanumeric => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            CharSet::AlphanumericLower => "abcdefghijklmnopqrstuvwxyz0123456789",
            CharSet::AlphanumericUpper => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            CharSet::Numeric => "0123456789",
            CharSet::Hexadecimal => "0123456789abcdef",
            CharSet::HexadecimalUpper => "0123456789ABCDEF",
            CharSet::Base64 => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
            CharSet::Base64Url => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
            CharSet::Printable => " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            CharSet::PrintableNoQuotes => " !#$%&()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
            CharSet::Symbols => "!@#$%^&*()_+-=[]{}|;:,.<>?",
            CharSet::SafeSymbols => "!@#$%^&*()_+-=[]{}|;:,.<>",
            CharSet::Custom(chars) => chars,
        }
    }
}

impl RandomStrings {
    /// Create new random string generator
    pub fn new() -> AdvancedCryptoResult<Self> {
        Ok(Self {
            secure_rng: SecureRandom::new()?,
        })
    }
    
    /// Generate random string with specified length and character set
    pub fn generate(&self, length: usize, charset: CharSet) -> AdvancedCryptoResult<String> {
        if length == 0 {
            return Ok(String::new());
        }
        
        let chars = charset.as_str();
        if chars.is_empty() {
            return Err("Character set cannot be empty".into());
        }
        
        let char_bytes = chars.as_bytes();
        let mut result = Vec::with_capacity(length);
        
        for _ in 0..length {
            let index = self.secure_rng.range_u64(0, char_bytes.len() as u64 - 1)?;
            result.push(char_bytes[index as usize]);
        }
        
        String::from_utf8(result).map_err(|e| e.to_string().into())
    }
    
    /// Generate alphabetic string (a-z, A-Z)
    pub fn alphabetic(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::Alphabetic)
    }
    
    /// Generate alphanumeric string (a-z, A-Z, 0-9)
    pub fn alphanumeric(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::Alphanumeric)
    }
    
    /// Generate lowercase alphanumeric string (a-z, 0-9)
    pub fn alphanumeric_lower(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::AlphanumericLower)
    }
    
    /// Generate uppercase alphanumeric string (A-Z, 0-9)
    pub fn alphanumeric_upper(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::AlphanumericUpper)
    }
    
    /// Generate numeric string (0-9)
    pub fn numeric(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::Numeric)
    }
    
    /// Generate hexadecimal string (0-9, a-f)
    pub fn hexadecimal(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::Hexadecimal)
    }
    
    /// Generate uppercase hexadecimal string (0-9, A-F)
    pub fn hexadecimal_upper(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::HexadecimalUpper)
    }
    
    /// Generate base64 string
    pub fn base64(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::Base64)
    }
    
    /// Generate base64url string
    pub fn base64url(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::Base64Url)
    }
    
    /// Generate string with printable ASCII characters
    pub fn printable(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::Printable)
    }
    
    /// Generate string with symbols
    pub fn symbols(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::Symbols)
    }
    
    /// Generate password-style string with mixed character types
    pub fn password(&self, length: usize, include_symbols: bool) -> AdvancedCryptoResult<String> {
        if length == 0 {
            return Ok(String::new());
        }
        
        let mut charset = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        if include_symbols {
            charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        }
        
        // Ensure at least one character from each required category
        let mut result = Vec::new();
        
        // At least one uppercase
        let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let idx = self.secure_rng.range_u64(0, uppercase.len() as u64 - 1)?;
        result.push(uppercase.as_bytes()[idx as usize]);
        
        // At least one lowercase
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let idx = self.secure_rng.range_u64(0, lowercase.len() as u64 - 1)?;
        result.push(lowercase.as_bytes()[idx as usize]);
        
        // At least one digit
        let digits = "0123456789";
        let idx = self.secure_rng.range_u64(0, digits.len() as u64 - 1)?;
        result.push(digits.as_bytes()[idx as usize]);
        
        // At least one symbol if requested
        if include_symbols {
            let symbols = "!@#$%^&*()_+-=[]{}|;:,.<>?";
            let idx = self.secure_rng.range_u64(0, symbols.len() as u64 - 1)?;
            result.push(symbols.as_bytes()[idx as usize]);
        }
        
        // Fill remaining positions
        let charset_bytes = charset.as_bytes();
        while result.len() < length {
            let idx = self.secure_rng.range_u64(0, charset_bytes.len() as u64 - 1)?;
            result.push(charset_bytes[idx as usize]);
        }
        
        // Shuffle the result to avoid predictable patterns
        self.secure_rng.shuffle(&mut result)?;
        
        String::from_utf8(result).map_err(|e| e.to_string().into())
    }
    
    /// Generate pronounceable password using syllable patterns
    pub fn pronounceable(&self, syllables: usize) -> AdvancedCryptoResult<String> {
        let consonants = ["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z"];
        let vowels = ["a", "e", "i", "o", "u"];
        let endings = ["", "s", "t", "r", "n", "l"];
        
        let mut result = String::new();
        
        for i in 0..syllables {
            // Consonant
            let consonant = self.secure_rng.choose(&consonants)?.unwrap();
            result.push_str(consonant);
            
            // Vowel
            let vowel = self.secure_rng.choose(&vowels)?.unwrap();
            result.push_str(vowel);
            
            // Optional ending consonant
            if i == syllables - 1 || self.secure_rng.bool()? {
                let ending = self.secure_rng.choose(&endings)?.unwrap();
                result.push_str(ending);
            }
        }
        
        // Capitalize first letter
        if let Some(first_char) = result.chars().next() {
            result = first_char.to_uppercase().collect::<String>() + &result[1..];
        }
        
        // Add some numbers for security
        let num = self.secure_rng.range_u32(10, 99)?;
        result.push_str(&num.to_string());
        
        Ok(result)
    }
    
    /// Generate mnemonic-style phrase
    pub fn mnemonic(&self, word_count: usize) -> AdvancedCryptoResult<String> {
        let words = [
            "apple", "brave", "chair", "dance", "eagle", "field", "grape", "house",
            "image", "juice", "knife", "light", "mouse", "night", "ocean", "phone",
            "queen", "river", "stone", "table", "under", "voice", "water", "youth",
            "zebra", "above", "beach", "cloud", "dream", "extra", "flame", "green",
            "happy", "index", "joint", "known", "lucky", "magic", "noble", "orbit",
            "peace", "quick", "round", "space", "trust", "urban", "valid", "world"
        ];
        
        let mut selected_words = Vec::new();
        for _ in 0..word_count {
            let word = self.secure_rng.choose(&words)?.unwrap();
            selected_words.push(*word);
        }
        
        Ok(selected_words.join(" "))
    }
    
    /// Generate identifier (suitable for variable names, IDs)
    pub fn identifier(&self, length: usize) -> AdvancedCryptoResult<String> {
        if length == 0 {
            return Ok(String::new());
        }
        
        // First character must be a letter
        let first_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let remaining_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_";
        
        let mut result = Vec::with_capacity(length);
        
        // First character
        let idx = self.secure_rng.range_u64(0, first_chars.len() as u64 - 1)?;
        result.push(first_chars.as_bytes()[idx as usize]);
        
        // Remaining characters
        for _ in 1..length {
            let idx = self.secure_rng.range_u64(0, remaining_chars.len() as u64 - 1)?;
            result.push(remaining_chars.as_bytes()[idx as usize]);
        }
        
        String::from_utf8(result).map_err(|e| e.to_string().into())
    }
    
    /// Generate filename-safe string
    pub fn filename(&self, length: usize) -> AdvancedCryptoResult<String> {
        // Safe characters for filenames across different operating systems
        let safe_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
        self.generate(length, CharSet::Custom(safe_chars.to_string()))
    }
    
    /// Generate URL-safe string
    pub fn url_safe(&self, length: usize) -> AdvancedCryptoResult<String> {
        self.generate(length, CharSet::Base64Url)
    }
    
    /// Generate domain name
    pub fn domain_name(&self, length: usize) -> AdvancedCryptoResult<String> {
        if length < 4 {
            return Err("Domain name must be at least 4 characters long".into());
        }
        
        let tlds = ["com", "org", "net", "io", "dev", "app"];
        let tld = self.secure_rng.choose(&tlds)?.unwrap();
        
        let name_length = length - tld.len() - 1; // -1 for the dot
        let name = self.generate(name_length, CharSet::AlphanumericLower)?;
        
        Ok(format!("{}.{}", name, tld))
    }
    
    /// Generate email address
    pub fn email(&self, local_length: usize) -> AdvancedCryptoResult<String> {
        let domains = ["example.com", "test.org", "demo.net", "sample.io"];
        let domain = self.secure_rng.choose(&domains)?.unwrap();
        
        let local_part = self.generate(local_length, CharSet::AlphanumericLower)?;
        
        Ok(format!("{}@{}", local_part, domain))
    }
    
    /// Generate random words from a dictionary
    pub fn words(&self, count: usize) -> AdvancedCryptoResult<String> {
        let dictionary = [
            "ability", "absence", "academy", "account", "accused", "achieve", "address", "advance",
            "advisor", "airport", "alcohol", "analyst", "ancient", "animals", "anxiety", "anybody",
            "applied", "arrange", "article", "assault", "attempt", "attract", "auction", "average",
            "balance", "banking", "battery", "bedroom", "benefit", "bicycle", "brother", "builder",
            "cabinet", "caliber", "camera", "camping", "caption", "carbon", "catalog", "ceiling",
            "chamber", "chapter", "charity", "chicken", "circuit", "classic", "climate", "clothes",
            "coastal", "college", "combine", "comfort", "command", "comment", "company", "concept",
            "concern", "conduct", "confirm", "connect", "consist", "contact", "contain", "content",
            "contest", "context", "control", "convert", "correct", "council", "counter", "country",
            "courage", "creator", "crystal", "culture", "current", "custody", "cutting", "dancing"
        ];
        
        let mut words = Vec::new();
        for _ in 0..count {
            let word = self.secure_rng.choose(&dictionary)?.unwrap();
            words.push(*word);
        }
        
        Ok(words.join(" "))
    }
    
    /// Generate random sentence
    pub fn sentence(&self, word_count: usize) -> AdvancedCryptoResult<String> {
        let mut sentence = self.words(word_count)?;
        
        // Capitalize first letter
        if let Some(first_char) = sentence.chars().next() {
            sentence = first_char.to_uppercase().collect::<String>() + &sentence[1..];
        }
        
        // Add period
        sentence.push('.');
        
        Ok(sentence)
    }
    
    /// Generate multiple strings
    pub fn batch(&self, count: usize, length: usize, charset: CharSet) -> AdvancedCryptoResult<Vec<String>> {
        let mut strings = Vec::with_capacity(count);
        for _ in 0..count {
            strings.push(self.generate(length, charset.clone())?);
        }
        Ok(strings)
    }
    
    /// Analyze character distribution in a string
    pub fn analyze(&self, text: &str) -> StringAnalysis {
        let mut char_freq = std::collections::HashMap::new();
        let mut uppercase = 0;
        let mut lowercase = 0;
        let mut digits = 0;
        let mut symbols = 0;
        let mut spaces = 0;
        
        for ch in text.chars() {
            *char_freq.entry(ch).or_insert(0) += 1;
            
            if ch.is_uppercase() {
                uppercase += 1;
            } else if ch.is_lowercase() {
                lowercase += 1;
            } else if ch.is_numeric() {
                digits += 1;
            } else if ch.is_whitespace() {
                spaces += 1;
            } else {
                symbols += 1;
            }
        }
        
        let total_chars = text.len();
        let unique_chars = char_freq.len();
        let most_common = char_freq.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&ch, &count)| (ch, count))
            .unwrap_or((' ', 0));
        
        StringAnalysis {
            length: total_chars,
            unique_chars,
            uppercase_count: uppercase,
            lowercase_count: lowercase,
            digit_count: digits,
            symbol_count: symbols,
            space_count: spaces,
            most_common_char: most_common.0,
            most_common_count: most_common.1,
            char_frequencies: char_freq,
        }
    }
}

/// Clone implementation for CharSet
impl Clone for CharSet {
    fn clone(&self) -> Self {
        match self {
            CharSet::Custom(s) => CharSet::Custom(s.clone()),
            other => {
                // For non-Custom variants, we can recreate them
                match other.as_str() {
                    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz" => CharSet::Alphabetic,
                    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789" => CharSet::Alphanumeric,
                    "abcdefghijklmnopqrstuvwxyz0123456789" => CharSet::AlphanumericLower,
                    "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789" => CharSet::AlphanumericUpper,
                    "0123456789" => CharSet::Numeric,
                    "0123456789abcdef" => CharSet::Hexadecimal,
                    "0123456789ABCDEF" => CharSet::HexadecimalUpper,
                    _ => CharSet::Custom(other.as_str().to_string()),
                }
            }
        }
    }
}

/// Analysis results for string data
#[derive(Debug, Clone)]
pub struct StringAnalysis {
    pub length: usize,
    pub unique_chars: usize,
    pub uppercase_count: usize,
    pub lowercase_count: usize,
    pub digit_count: usize,
    pub symbol_count: usize,
    pub space_count: usize,
    pub most_common_char: char,
    pub most_common_count: usize,
    pub char_frequencies: std::collections::HashMap<char, usize>,
}

impl Default for RandomStrings {
    fn default() -> Self {
        Self::new().expect("Failed to create default RandomStrings")
    }
}

/// Global functions for convenient access to random string generation
pub fn random_string(length: usize, charset: CharSet) -> AdvancedCryptoResult<String> {
    RandomStrings::new()?.generate(length, charset)
}

pub fn random_alphabetic(length: usize) -> AdvancedCryptoResult<String> {
    RandomStrings::new()?.alphabetic(length)
}

pub fn random_alphanumeric(length: usize) -> AdvancedCryptoResult<String> {
    RandomStrings::new()?.alphanumeric(length)
}

pub fn random_numeric(length: usize) -> AdvancedCryptoResult<String> {
    RandomStrings::new()?.numeric(length)
}

pub fn random_hexadecimal(length: usize) -> AdvancedCryptoResult<String> {
    RandomStrings::new()?.hexadecimal(length)
}

pub fn random_password(length: usize, include_symbols: bool) -> AdvancedCryptoResult<String> {
    RandomStrings::new()?.password(length, include_symbols)
}

pub fn random_identifier(length: usize) -> AdvancedCryptoResult<String> {
    RandomStrings::new()?.identifier(length)
}

pub fn random_filename(length: usize) -> AdvancedCryptoResult<String> {
    RandomStrings::new()?.filename(length)
}
