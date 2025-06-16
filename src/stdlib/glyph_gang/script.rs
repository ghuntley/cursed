/// Script detection and analysis for Unicode text
use crate::stdlib::glyph_gang::error::{GlyphGangResult, script_detection_error};
use crate::stdlib::glyph_gang::ranges::{
    LATIN, GREEK, CYRILLIC, HEBREW, ARABIC, DEVANAGARI, THAI,
    HAN, HIRAGANA, KATAKANA, HANGUL
};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Unicode script enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Script {
    Unknown,
    Latin,
    Greek,
    Cyrillic,
    Hebrew,
    Arabic,
    Devanagari,
    Thai,
    Han,
    Hiragana,
    Katakana,
    Hangul,
    // Add more scripts as needed
}

/// Constants for scripts
pub const SCRIPT_UNKNOWN: Script = Script::Unknown;
pub const SCRIPT_LATIN: Script = Script::Latin;
pub const SCRIPT_GREEK: Script = Script::Greek;
pub const SCRIPT_CYRILLIC: Script = Script::Cyrillic;
pub const SCRIPT_HEBREW: Script = Script::Hebrew;
pub const SCRIPT_ARABIC: Script = Script::Arabic;
pub const SCRIPT_DEVANAGARI: Script = Script::Devanagari;
pub const SCRIPT_THAI: Script = Script::Thai;
pub const SCRIPT_HAN: Script = Script::Han;
pub const SCRIPT_HIRAGANA: Script = Script::Hiragana;
pub const SCRIPT_KATAKANA: Script = Script::Katakana;
pub const SCRIPT_HANGUL: Script = Script::Hangul;

/// Script names database
static SCRIPT_NAMES: Lazy<HashMap<Script, &'static str>> = Lazy::new(|| {
    let mut names = HashMap::new();
    names.insert(Script::Unknown, "Unknown");
    names.insert(Script::Latin, "Latin");
    names.insert(Script::Greek, "Greek");
    names.insert(Script::Cyrillic, "Cyrillic");
    names.insert(Script::Hebrew, "Hebrew");
    names.insert(Script::Arabic, "Arabic");
    names.insert(Script::Devanagari, "Devanagari");
    names.insert(Script::Thai, "Thai");
    names.insert(Script::Han, "Han");
    names.insert(Script::Hiragana, "Hiragana");
    names.insert(Script::Katakana, "Katakana");
    names.insert(Script::Hangul, "Hangul");
    names
});

/// Languages associated with scripts
static SCRIPT_LANGUAGES: Lazy<HashMap<Script, Vec<&'static str>>> = Lazy::new(|| {
    let mut languages = HashMap::new();
    
    languages.insert(Script::Latin, vec![
        "English", "Spanish", "French", "German", "Italian", "Portuguese", 
        "Dutch", "Polish", "Czech", "Slovak", "Hungarian", "Romanian", 
        "Croatian", "Serbian", "Slovenian", "Estonian", "Latvian", 
        "Lithuanian", "Albanian", "Basque", "Catalan", "Galician", 
        "Irish", "Scottish Gaelic", "Welsh", "Breton", "Cornish", 
        "Manx", "Icelandic", "Faroese", "Norwegian", "Swedish", "Danish", 
        "Finnish", "Turkish", "Azerbaijani", "Uzbek", "Kazakh", 
        "Kyrgyz", "Turkmen", "Tatar", "Bashkir", "Chuvash", "Yakut", 
        "Vietnamese", "Indonesian", "Malay", "Tagalog", "Swahili", 
        "Yoruba", "Igbo", "Hausa", "Zulu", "Xhosa", "Afrikaans"
    ]);
    
    languages.insert(Script::Greek, vec![
        "Greek", "Coptic"
    ]);
    
    languages.insert(Script::Cyrillic, vec![
        "Russian", "Ukrainian", "Belarusian", "Bulgarian", "Macedonian", 
        "Serbian", "Montenegrin", "Bosnian", "Croatian", "Czech", 
        "Slovak", "Kazakh", "Kyrgyz", "Tajik", "Uzbek", "Mongolian", 
        "Azerbaijani", "Turkmen", "Tatar", "Bashkir", "Chuvash", 
        "Yakut", "Evenk", "Even", "Nanai", "Udmurt", "Mari", 
        "Komi", "Nenets", "Selkup", "Ossetian", "Abkhaz", "Georgian"
    ]);
    
    languages.insert(Script::Hebrew, vec![
        "Hebrew", "Yiddish", "Ladino"
    ]);
    
    languages.insert(Script::Arabic, vec![
        "Arabic", "Persian", "Urdu", "Pashto", "Kurdish", "Sindhi", 
        "Balochi", "Uyghur", "Kazakh", "Kyrgyz", "Uzbek", "Tajik", 
        "Turkmen", "Azerbaijani", "Turkish", "Malay", "Hausa", 
        "Swahili", "Somali", "Oromo", "Amharic", "Tigrinya"
    ]);
    
    languages.insert(Script::Devanagari, vec![
        "Hindi", "Marathi", "Nepali", "Sanskrit", "Konkani", "Sindhi", 
        "Bhojpuri", "Maithili", "Magahi", "Awadhi", "Chhattisgarhi", 
        "Rajasthani", "Haryanvi", "Bhili", "Garhwali", "Kumaoni"
    ]);
    
    languages.insert(Script::Thai, vec![
        "Thai", "Lao", "Khmer"
    ]);
    
    languages.insert(Script::Han, vec![
        "Chinese", "Japanese", "Korean", "Vietnamese"
    ]);
    
    languages.insert(Script::Hiragana, vec![
        "Japanese"
    ]);
    
    languages.insert(Script::Katakana, vec![
        "Japanese"
    ]);
    
    languages.insert(Script::Hangul, vec![
        "Korean"
    ]);
    
    languages.insert(Script::Unknown, vec![]);
    
    languages
});

/// Get the script of a character
fn get_character_script(ch: char) -> Script {
    // Check each script range table
    if LATIN.contains(ch) {
        Script::Latin
    } else if GREEK.contains(ch) {
        Script::Greek
    } else if CYRILLIC.contains(ch) {
        Script::Cyrillic
    } else if HEBREW.contains(ch) {
        Script::Hebrew
    } else if ARABIC.contains(ch) {
        Script::Arabic
    } else if DEVANAGARI.contains(ch) {
        Script::Devanagari
    } else if THAI.contains(ch) {
        Script::Thai
    } else if HAN.contains(ch) {
        Script::Han
    } else if HIRAGANA.contains(ch) {
        Script::Hiragana
    } else if KATAKANA.contains(ch) {
        Script::Katakana
    } else if HANGUL.contains(ch) {
        Script::Hangul
    } else {
        // Additional checks for common characters
        let code_point = ch as u32;
        
        if code_point <= 0x007F {
            // ASCII - typically Latin script context
            if ch.is_ascii_alphabetic() {
                Script::Latin
            } else {
                Script::Unknown
            }
        } else {
            Script::Unknown
        }
    }
}

/// Detect the primary script of a string
pub fn detect_script(s: &str) -> Script {
    if s.is_empty() {
        return Script::Unknown;
    }
    
    let mut script_counts: HashMap<Script, usize> = HashMap::new();
    
    for ch in s.chars() {
        let script = get_character_script(ch);
        if script != Script::Unknown {
            *script_counts.entry(script).or_insert(0) += 1;
        }
    }
    
    // Find the script with the highest count
    script_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(script, _)| script)
        .unwrap_or(Script::Unknown)
}

/// Get the name of a script
pub fn get_script_name(script: Script) -> String {
    SCRIPT_NAMES
        .get(&script)
        .unwrap_or(&"Unknown")
        .to_string()
}

/// Get languages that use a specific script
pub fn get_languages_by_script(script: Script) -> Vec<String> {
    SCRIPT_LANGUAGES
        .get(&script)
        .unwrap_or(&vec![])
        .iter()
        .map(|&lang| lang.to_string())
        .collect()
}

/// Analyze script distribution in a string
pub fn analyze_script_distribution(s: &str) -> HashMap<String, usize> {
    let mut distribution = HashMap::new();
    
    for ch in s.chars() {
        let script = get_character_script(ch);
        let script_name = get_script_name(script);
        *distribution.entry(script_name).or_insert(0) += 1;
    }
    
    distribution
}

/// Check if a string contains mixed scripts
pub fn is_mixed_script(s: &str) -> bool {
    let mut scripts_found = std::collections::HashSet::new();
    
    for ch in s.chars() {
        let script = get_character_script(ch);
        if script != Script::Unknown {
            scripts_found.insert(script);
            if scripts_found.len() > 1 {
                return true;
            }
        }
    }
    
    false
}

/// Get all characters belonging to a specific script from a string
pub fn extract_script_characters(s: &str, target_script: Script) -> String {
    s.chars()
        .filter(|&ch| get_character_script(ch) == target_script)
        .collect()
}

/// Get script boundaries in a string (positions where script changes)
pub fn get_script_boundaries(s: &str) -> Vec<(usize, Script)> {
    let mut boundaries = Vec::new();
    let mut current_script = Script::Unknown;
    let mut byte_pos = 0;
    
    for ch in s.chars() {
        let script = get_character_script(ch);
        
        if script != Script::Unknown && script != current_script {
            boundaries.push((byte_pos, script));
            current_script = script;
        }
        
        byte_pos += ch.len_utf8();
    }
    
    boundaries
}

/// Segment text by script boundaries
pub fn segment_by_script(s: &str) -> Vec<(String, Script)> {
    let boundaries = get_script_boundaries(s);
    let mut segments = Vec::new();
    
    if boundaries.is_empty() {
        return vec![(s.to_string(), Script::Unknown)];
    }
    
    let mut last_pos = 0;
    
    for (i, &(pos, script)) in boundaries.iter().enumerate() {
        if i > 0 {
            let segment = s[last_pos..pos].to_string();
            let prev_script = boundaries[i - 1].1;
            segments.push((segment, prev_script));
        }
        last_pos = pos;
    }
    
    // Add the final segment
    if last_pos < s.len() {
        let segment = s[last_pos..].to_string();
        let last_script = boundaries.last().map(|(_, script)| *script).unwrap_or(Script::Unknown);
        segments.push((segment, last_script));
    }
    
    segments
}

/// Get detailed script analysis
pub fn get_script_info(s: &str) -> HashMap<String, String> {
    let mut info = HashMap::new();
    
    let primary_script = detect_script(s);
    info.insert("primary_script".to_string(), format!("{:?}", primary_script));
    info.insert("primary_script_name".to_string(), get_script_name(primary_script));
    info.insert("is_mixed_script".to_string(), is_mixed_script(s).to_string());
    info.insert("character_count".to_string(), s.chars().count().to_string());
    
    let distribution = analyze_script_distribution(s);
    info.insert("script_count".to_string(), distribution.len().to_string());
    
    // Add distribution details
    for (script_name, count) in distribution {
        info.insert(format!("{}_chars", script_name.to_lowercase()), count.to_string());
    }
    
    // Add languages for primary script
    let languages = get_languages_by_script(primary_script);
    if !languages.is_empty() {
        info.insert("possible_languages".to_string(), languages.join(", "));
    }
    
    info
}

/// Initialize script detection data
pub fn initialize_script_data() -> GlyphGangResult<()> {
    // Force initialization of lazy statics
    Lazy::force(&SCRIPT_NAMES);
    Lazy::force(&SCRIPT_LANGUAGES);
    
    // Validate that data loaded correctly
    if SCRIPT_NAMES.is_empty() {
        return Err(script_detection_error("Failed to initialize script names", ""));
    }
    
    if SCRIPT_LANGUAGES.is_empty() {
        return Err(script_detection_error("Failed to initialize script languages", ""));
    }
    
    Ok(())
}

/// Get all available scripts
pub fn get_available_scripts() -> Vec<Script> {
    SCRIPT_NAMES.keys().copied().collect()
}

/// Check if a script is supported
pub fn is_script_supported(script: Script) -> bool {
    SCRIPT_NAMES.contains_key(&script)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_script_detection() {
        assert_eq!(get_character_script('A'), Script::Latin);
        assert_eq!(get_character_script('a'), Script::Latin);
        assert_eq!(get_character_script('1'), Script::Unknown);
        assert_eq!(get_character_script(' '), Script::Unknown);
        
        // Greek
        assert_eq!(get_character_script('α'), Script::Greek);
        assert_eq!(get_character_script('Α'), Script::Greek);
        
        // Cyrillic
        assert_eq!(get_character_script('а'), Script::Cyrillic);
        assert_eq!(get_character_script('А'), Script::Cyrillic);
        
        // Hebrew
        assert_eq!(get_character_script('א'), Script::Hebrew);
        assert_eq!(get_character_script('ב'), Script::Hebrew);
        
        // Arabic
        assert_eq!(get_character_script('ا'), Script::Arabic);
        assert_eq!(get_character_script('ب'), Script::Arabic);
        
        // CJK
        assert_eq!(get_character_script('中'), Script::Han);
        assert_eq!(get_character_script('国'), Script::Han);
        
        // Japanese
        assert_eq!(get_character_script('あ'), Script::Hiragana);
        assert_eq!(get_character_script('ア'), Script::Katakana);
        
        // Korean
        assert_eq!(get_character_script('가'), Script::Hangul);
        assert_eq!(get_character_script('한'), Script::Hangul);
    }

    #[test]
    fn test_string_script_detection() {
        assert_eq!(detect_script("Hello World"), Script::Latin);
        assert_eq!(detect_script("Καλημέρα"), Script::Greek);
        assert_eq!(detect_script("Привет"), Script::Cyrillic);
        assert_eq!(detect_script("שלום"), Script::Hebrew);
        assert_eq!(detect_script("السلام"), Script::Arabic);
        assert_eq!(detect_script("中文"), Script::Han);
        assert_eq!(detect_script("こんにちは"), Script::Hiragana);
        assert_eq!(detect_script("コンニチワ"), Script::Katakana);
        assert_eq!(detect_script("안녕하세요"), Script::Hangul);
        assert_eq!(detect_script(""), Script::Unknown);
        assert_eq!(detect_script("123 !@#"), Script::Unknown);
    }

    #[test]
    fn test_script_names() {
        assert_eq!(get_script_name(Script::Latin), "Latin");
        assert_eq!(get_script_name(Script::Greek), "Greek");
        assert_eq!(get_script_name(Script::Cyrillic), "Cyrillic");
        assert_eq!(get_script_name(Script::Hebrew), "Hebrew");
        assert_eq!(get_script_name(Script::Arabic), "Arabic");
        assert_eq!(get_script_name(Script::Han), "Han");
        assert_eq!(get_script_name(Script::Hiragana), "Hiragana");
        assert_eq!(get_script_name(Script::Katakana), "Katakana");
        assert_eq!(get_script_name(Script::Hangul), "Hangul");
        assert_eq!(get_script_name(Script::Unknown), "Unknown");
    }

    #[test]
    fn test_script_languages() {
        let languages = get_languages_by_script(Script::Latin);
        assert!(languages.contains(&"English".to_string()));
        assert!(languages.contains(&"Spanish".to_string()));
        assert!(languages.contains(&"French".to_string()));
        
        let languages = get_languages_by_script(Script::Greek);
        assert!(languages.contains(&"Greek".to_string()));
        
        let languages = get_languages_by_script(Script::Cyrillic);
        assert!(languages.contains(&"Russian".to_string()));
        assert!(languages.contains(&"Ukrainian".to_string()));
        
        let languages = get_languages_by_script(Script::Hebrew);
        assert!(languages.contains(&"Hebrew".to_string()));
        
        let languages = get_languages_by_script(Script::Arabic);
        assert!(languages.contains(&"Arabic".to_string()));
        assert!(languages.contains(&"Persian".to_string()));
        
        let languages = get_languages_by_script(Script::Hiragana);
        assert!(languages.contains(&"Japanese".to_string()));
        
        let languages = get_languages_by_script(Script::Hangul);
        assert!(languages.contains(&"Korean".to_string()));
        
        let languages = get_languages_by_script(Script::Unknown);
        assert!(languages.is_empty());
    }

    #[test]
    fn test_script_distribution() {
        let distribution = analyze_script_distribution("Hello Привет");
        assert!(distribution.contains_key("Latin"));
        assert!(distribution.contains_key("Cyrillic"));
        assert_eq!(distribution.get("Latin"), Some(&5)); // "Hello"
        assert_eq!(distribution.get("Cyrillic"), Some(&6)); // "Привет"
        
        let distribution = analyze_script_distribution("English only");
        assert!(distribution.contains_key("Latin"));
        assert!(!distribution.contains_key("Cyrillic"));
    }

    #[test]
    fn test_mixed_script() {
        assert!(!is_mixed_script("Hello World"));
        assert!(!is_mixed_script("Привет"));
        assert!(is_mixed_script("Hello Привет"));
        assert!(is_mixed_script("English 中文"));
        assert!(!is_mixed_script(""));
        assert!(!is_mixed_script("123 !@#"));
    }

    #[test]
    fn test_script_extraction() {
        let text = "Hello Привет 中文";
        
        let latin = extract_script_characters(&text, Script::Latin);
        assert_eq!(latin, "Hello");
        
        let cyrillic = extract_script_characters(&text, Script::Cyrillic);
        assert_eq!(cyrillic, "Привет");
        
        let han = extract_script_characters(&text, Script::Han);
        assert_eq!(han, "中文");
        
        let unknown = extract_script_characters(&text, Script::Unknown);
        assert_eq!(unknown, "  ");
    }

    #[test]
    fn test_script_segmentation() {
        let segments = segment_by_script("Hello Привет");
        assert_eq!(segments.len(), 2);
        
        // Check that we get the expected scripts
        let scripts: Vec<Script> = segments.iter().map(|(_, script)| *script).collect();
        assert!(scripts.contains(&Script::Latin));
        assert!(scripts.contains(&Script::Cyrillic));
        
        let segments = segment_by_script("English only");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].1, Script::Latin);
    }

    #[test]
    fn test_script_info() {
        let info = get_script_info("Hello World");
        assert_eq!(info.get("primary_script").unwrap(), "Latin");
        assert_eq!(info.get("primary_script_name").unwrap(), "Latin");
        assert_eq!(info.get("is_mixed_script").unwrap(), "false");
        assert!(info.get("possible_languages").unwrap().contains("English"));
        
        let info = get_script_info("Hello Привет");
        assert_eq!(info.get("is_mixed_script").unwrap(), "true");
        assert!(info.contains_key("latin_chars"));
        assert!(info.contains_key("cyrillic_chars"));
    }

    #[test]
    fn test_script_support() {
        assert!(is_script_supported(Script::Latin));
        assert!(is_script_supported(Script::Greek));
        assert!(is_script_supported(Script::Unknown));
        
        let scripts = get_available_scripts();
        assert!(scripts.contains(&Script::Latin));
        assert!(scripts.contains(&Script::Greek));
        assert!(scripts.contains(&Script::Cyrillic));
    }

    #[test]
    fn test_initialization() {
        assert!(initialize_script_data().is_ok());
    }
}
