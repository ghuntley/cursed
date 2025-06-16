/// Emoji support and detection functions
use crate::stdlib::glyph_gang::error::{GlyphGangResult, emoji_error};
use crate::stdlib::glyph_gang::ranges::{EMOJI, EMOJI_MODIFIER, EMOJI_COMPONENT, EMOJI_MODIFIER_BASE};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Emoji name database (simplified for initial implementation)
static EMOJI_NAMES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut names = HashMap::new();
    
    // Basic smileys and emotion
    names.insert("😀", "GRINNING FACE");
    names.insert("😁", "GRINNING FACE WITH SMILING EYES");
    names.insert("😂", "FACE WITH TEARS OF JOY");
    names.insert("🤣", "ROLLING ON THE FLOOR LAUGHING");
    names.insert("😃", "SMILING FACE WITH OPEN MOUTH");
    names.insert("😄", "SMILING FACE WITH OPEN MOUTH AND SMILING EYES");
    names.insert("😅", "SMILING FACE WITH OPEN MOUTH AND COLD SWEAT");
    names.insert("😆", "SMILING FACE WITH OPEN MOUTH AND TIGHTLY-CLOSED EYES");
    names.insert("😉", "WINKING FACE");
    names.insert("😊", "SMILING FACE WITH SMILING EYES");
    names.insert("😋", "FACE SAVOURING DELICIOUS FOOD");
    names.insert("😎", "SMILING FACE WITH SUNGLASSES");
    names.insert("😍", "SMILING FACE WITH HEART-SHAPED EYES");
    names.insert("😘", "FACE THROWING A KISS");
    names.insert("🥰", "SMILING FACE WITH HEARTS");
    names.insert("😗", "KISSING FACE");
    names.insert("😙", "KISSING FACE WITH SMILING EYES");
    names.insert("😚", "KISSING FACE WITH CLOSED EYES");
    names.insert("🙂", "SLIGHTLY SMILING FACE");
    names.insert("🤗", "HUGGING FACE");
    names.insert("🤩", "STAR-STRUCK");
    names.insert("🤔", "THINKING FACE");
    names.insert("🤨", "FACE WITH RAISED EYEBROW");
    names.insert("😐", "NEUTRAL FACE");
    names.insert("😑", "EXPRESSIONLESS FACE");
    names.insert("😶", "FACE WITHOUT MOUTH");
    names.insert("🙄", "FACE WITH ROLLING EYES");
    names.insert("😏", "SMIRKING FACE");
    names.insert("😣", "PERSEVERING FACE");
    names.insert("😥", "DISAPPOINTED BUT RELIEVED FACE");
    names.insert("😮", "FACE WITH OPEN MOUTH");
    names.insert("🤐", "ZIPPER-MOUTH FACE");
    names.insert("😯", "HUSHED FACE");
    names.insert("😪", "SLEEPY FACE");
    names.insert("😫", "TIRED FACE");
    names.insert("🥱", "YAWNING FACE");
    names.insert("😴", "SLEEPING FACE");
    names.insert("😌", "RELIEVED FACE");
    names.insert("😛", "FACE WITH STUCK-OUT TONGUE");
    names.insert("😜", "FACE WITH STUCK-OUT TONGUE AND WINKING EYE");
    names.insert("😝", "FACE WITH STUCK-OUT TONGUE AND TIGHTLY-CLOSED EYES");
    names.insert("🤤", "DROOLING FACE");
    names.insert("😒", "UNAMUSED FACE");
    names.insert("😓", "FACE WITH COLD SWEAT");
    names.insert("😔", "PENSIVE FACE");
    names.insert("😕", "CONFUSED FACE");
    names.insert("🙃", "UPSIDE-DOWN FACE");
    names.insert("🤑", "MONEY-MOUTH FACE");
    names.insert("😲", "ASTONISHED FACE");
    names.insert("☹️", "FROWNING FACE");
    names.insert("🙁", "SLIGHTLY FROWNING FACE");
    names.insert("😖", "CONFOUNDED FACE");
    names.insert("😞", "DISAPPOINTED FACE");
    names.insert("😟", "WORRIED FACE");
    names.insert("😤", "FACE WITH STEAM FROM NOSE");
    names.insert("😢", "CRYING FACE");
    names.insert("😭", "LOUDLY CRYING FACE");
    names.insert("😦", "FROWNING FACE WITH OPEN MOUTH");
    names.insert("😧", "ANGUISHED FACE");
    names.insert("😨", "FEARFUL FACE");
    names.insert("😩", "WEARY FACE");
    names.insert("🤯", "EXPLODING HEAD");
    names.insert("😬", "GRIMACING FACE");
    names.insert("😰", "FACE WITH OPEN MOUTH AND COLD SWEAT");
    names.insert("😱", "FACE SCREAMING IN FEAR");
    names.insert("🥵", "HOT FACE");
    names.insert("🥶", "COLD FACE");
    names.insert("😳", "FLUSHED FACE");
    names.insert("🤪", "ZANY FACE");
    names.insert("😵", "DIZZY FACE");
    names.insert("🥴", "WOOZY FACE");
    names.insert("😷", "FACE WITH MEDICAL MASK");
    names.insert("🤒", "FACE WITH THERMOMETER");
    names.insert("🤕", "FACE WITH HEAD-BANDAGE");
    names.insert("🤢", "NAUSEATED FACE");
    names.insert("🤮", "FACE VOMITING");
    names.insert("🤧", "SNEEZING FACE");
    names.insert("😇", "SMILING FACE WITH HALO");
    names.insert("🥳", "PARTYING FACE");
    names.insert("🥺", "PLEADING FACE");
    names.insert("🤠", "COWBOY HAT FACE");
    names.insert("🤡", "CLOWN FACE");
    names.insert("🤥", "LYING FACE");
    names.insert("🤫", "SHUSHING FACE");
    names.insert("🤭", "FACE WITH HAND OVER MOUTH");
    names.insert("🧐", "FACE WITH MONOCLE");
    names.insert("🤓", "NERD FACE");
    
    // Hand gestures
    names.insert("👍", "THUMBS UP SIGN");
    names.insert("👎", "THUMBS DOWN SIGN");
    names.insert("👌", "OK HAND SIGN");
    names.insert("✌️", "VICTORY HAND");
    names.insert("🤞", "CROSSED FINGERS");
    names.insert("🤟", "LOVE-YOU GESTURE");
    names.insert("🤘", "SIGN OF THE HORNS");
    names.insert("🤙", "CALL ME HAND");
    names.insert("👈", "WHITE LEFT POINTING BACKHAND INDEX");
    names.insert("👉", "WHITE RIGHT POINTING BACKHAND INDEX");
    names.insert("👆", "WHITE UP POINTING BACKHAND INDEX");
    names.insert("🖕", "REVERSED HAND WITH MIDDLE FINGER EXTENDED");
    names.insert("👇", "WHITE DOWN POINTING BACKHAND INDEX");
    names.insert("☝️", "WHITE UP POINTING INDEX");
    names.insert("👋", "WAVING HAND SIGN");
    names.insert("🤚", "RAISED BACK OF HAND");
    names.insert("🖐️", "RAISED HAND WITH FINGERS SPLAYED");
    names.insert("✋", "RAISED HAND");
    names.insert("🖖", "RAISED HAND WITH PART BETWEEN MIDDLE AND RING FINGERS");
    names.insert("👏", "CLAPPING HANDS SIGN");
    names.insert("🙌", "PERSON RAISING BOTH HANDS IN CELEBRATION");
    names.insert("🤝", "HANDSHAKE");
    names.insert("🙏", "PERSON WITH FOLDED HANDS");
    names.insert("✍️", "WRITING HAND");
    names.insert("💅", "NAIL POLISH");
    names.insert("🤳", "SELFIE");
    names.insert("💪", "FLEXED BICEPS");
    
    // Hearts and symbols
    names.insert("❤️", "HEAVY BLACK HEART");
    names.insert("🧡", "ORANGE HEART");
    names.insert("💛", "YELLOW HEART");
    names.insert("💚", "GREEN HEART");
    names.insert("💙", "BLUE HEART");
    names.insert("💜", "PURPLE HEART");
    names.insert("🖤", "BLACK HEART");
    names.insert("🤍", "WHITE HEART");
    names.insert("🤎", "BROWN HEART");
    names.insert("💔", "BROKEN HEART");
    names.insert("❣️", "HEAVY HEART EXCLAMATION MARK SYMBOL");
    names.insert("💕", "TWO HEARTS");
    names.insert("💞", "REVOLVING HEARTS");
    names.insert("💓", "BEATING HEART");
    names.insert("💗", "GROWING HEART");
    names.insert("💖", "SPARKLING HEART");
    names.insert("💘", "HEART WITH ARROW");
    names.insert("💝", "HEART WITH RIBBON");
    names.insert("💟", "HEART DECORATION");
    
    // Common objects
    names.insert("🔥", "FIRE");
    names.insert("💧", "DROPLET");
    names.insert("⭐", "WHITE MEDIUM STAR");
    names.insert("🌟", "GLOWING STAR");
    names.insert("✨", "SPARKLES");
    names.insert("⚡", "HIGH VOLTAGE SIGN");
    names.insert("☀️", "BLACK SUN WITH RAYS");
    names.insert("🌈", "RAINBOW");
    names.insert("☁️", "CLOUD");
    names.insert("⛅", "PARTLY CLOUDY");
    names.insert("🌧️", "CLOUD WITH RAIN");
    names.insert("⛈️", "THUNDER CLOUD AND RAIN");
    names.insert("🌩️", "CLOUD WITH LIGHTNING");
    names.insert("🌨️", "CLOUD WITH SNOW");
    names.insert("❄️", "SNOWFLAKE");
    names.insert("☃️", "SNOWMAN");
    names.insert("⛄", "SNOWMAN WITHOUT SNOW");
    names.insert("🌊", "WATER WAVE");
    names.insert("💨", "DASH SYMBOL");
    
    // Food and drink
    names.insert("🍕", "SLICE OF PIZZA");
    names.insert("🍔", "HAMBURGER");
    names.insert("🍟", "FRENCH FRIES");
    names.insert("🌭", "HOT DOG");
    names.insert("🍗", "POULTRY LEG");
    names.insert("🍖", "MEAT ON BONE");
    names.insert("🥓", "BACON");
    names.insert("🍳", "COOKING");
    names.insert("🥚", "EGG");
    names.insert("🧀", "CHEESE WEDGE");
    names.insert("🥖", "BAGUETTE BREAD");
    names.insert("🍞", "BREAD");
    names.insert("🥯", "BAGEL");
    names.insert("🥨", "PRETZEL");
    names.insert("🍝", "SPAGHETTI");
    names.insert("🍜", "STEAMING BOWL");
    names.insert("🍲", "POT OF FOOD");
    names.insert("🍛", "CURRY AND RICE");
    names.insert("🍣", "SUSHI");
    names.insert("🍱", "BENTO BOX");
    names.insert("🥟", "DUMPLING");
    names.insert("🍤", "FRIED SHRIMP");
    names.insert("🍙", "RICE BALL");
    names.insert("🍚", "COOKED RICE");
    names.insert("🍘", "RICE CRACKER");
    names.insert("🍥", "FISH CAKE WITH SWIRL DESIGN");
    names.insert("🥠", "FORTUNE COOKIE");
    names.insert("🍢", "ODEN");
    names.insert("🍡", "DANGO");
    names.insert("🍧", "SHAVED ICE");
    names.insert("🍨", "ICE CREAM");
    names.insert("🍦", "SOFT ICE CREAM");
    names.insert("🥧", "PIE");
    names.insert("🍰", "SHORTCAKE");
    names.insert("🎂", "BIRTHDAY CAKE");
    names.insert("🧁", "CUPCAKE");
    names.insert("🍮", "CUSTARD");
    names.insert("🍭", "LOLLIPOP");
    names.insert("🍬", "CANDY");
    names.insert("🍫", "CHOCOLATE BAR");
    names.insert("🍿", "POPCORN");
    names.insert("🍩", "DOUGHNUT");
    names.insert("🍪", "COOKIE");
    
    // Drinks
    names.insert("☕", "HOT BEVERAGE");
    names.insert("🍵", "TEACUP WITHOUT HANDLE");
    names.insert("🧃", "BEVERAGE BOX");
    names.insert("🥤", "CUP WITH STRAW");
    names.insert("🍶", "SAKE BOTTLE AND CUP");
    names.insert("🍺", "BEER MUG");
    names.insert("🍻", "CLINKING BEER MUGS");
    names.insert("🥂", "CLINKING GLASSES");
    names.insert("🍷", "WINE GLASS");
    names.insert("🥃", "TUMBLER GLASS");
    names.insert("🍸", "COCKTAIL GLASS");
    names.insert("🍹", "TROPICAL DRINK");
    names.insert("🧊", "ICE CUBE");
    
    names
});

/// Emoji categories database
static EMOJI_CATEGORIES: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut categories = HashMap::new();
    
    categories.insert("smileys_emotion", vec![
        "😀", "😁", "😂", "🤣", "😃", "😄", "😅", "😆", "😉", "😊",
        "😋", "😎", "😍", "😘", "🥰", "😗", "😙", "😚", "🙂", "🤗",
        "🤩", "🤔", "🤨", "😐", "😑", "😶", "🙄", "😏", "😣", "😥",
        "😮", "🤐", "😯", "😪", "😫", "🥱", "😴", "😌", "😛", "😜",
        "😝", "🤤", "😒", "😓", "😔", "😕", "🙃", "🤑", "😲", "☹️",
        "🙁", "😖", "😞", "😟", "😤", "😢", "😭", "😦", "😧", "😨",
        "😩", "🤯", "😬", "😰", "😱", "🥵", "🥶", "😳", "🤪", "😵",
        "🥴", "😷", "🤒", "🤕", "🤢", "🤮", "🤧", "😇", "🥳", "🥺",
        "🤠", "🤡", "🤥", "🤫", "🤭", "🧐", "🤓"
    ]);
    
    categories.insert("people_body", vec![
        "👍", "👎", "👌", "✌️", "🤞", "🤟", "🤘", "🤙", "👈", "👉",
        "👆", "🖕", "👇", "☝️", "👋", "🤚", "🖐️", "✋", "🖖", "👏",
        "🙌", "🤝", "🙏", "✍️", "💅", "🤳", "💪"
    ]);
    
    categories.insert("hearts", vec![
        "❤️", "🧡", "💛", "💚", "💙", "💜", "🖤", "🤍", "🤎", "💔",
        "❣️", "💕", "💞", "💓", "💗", "💖", "💘", "💝", "💟"
    ]);
    
    categories.insert("nature", vec![
        "🔥", "💧", "⭐", "🌟", "✨", "⚡", "☀️", "🌈", "☁️", "⛅",
        "🌧️", "⛈️", "🌩️", "🌨️", "❄️", "☃️", "⛄", "🌊", "💨"
    ]);
    
    categories.insert("food_drink", vec![
        "🍕", "🍔", "🍟", "🌭", "🍗", "🍖", "🥓", "🍳", "🥚", "🧀",
        "🥖", "🍞", "🥯", "🥨", "🍝", "🍜", "🍲", "🍛", "🍣", "🍱",
        "🥟", "🍤", "🍙", "🍚", "🍘", "🍥", "🥠", "🍢", "🍡", "🍧",
        "🍨", "🍦", "🥧", "🍰", "🎂", "🧁", "🍮", "🍭", "🍬", "🍫",
        "🍿", "🍩", "🍪", "☕", "🍵", "🧃", "🥤", "🍶", "🍺", "🍻",
        "🥂", "🍷", "🥃", "🍸", "🍹", "🧊"
    ]);
    
    categories
});

/// Check if a string contains an emoji sequence
pub fn is_emoji_sequence(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    // Check if the string contains ZWJ (Zero Width Joiner) which indicates a sequence
    if s.contains('\u{200D}') {
        return true;
    }
    
    // Check if it's a single emoji character
    if s.chars().count() == 1 {
        let ch = s.chars().next().unwrap();
        return EMOJI.contains(ch);
    }
    
    // Check for emoji modifier sequences (emoji + modifier)
    let chars: Vec<char> = s.chars().collect();
    if chars.len() == 2 {
        return EMOJI_MODIFIER_BASE.contains(chars[0]) && EMOJI_MODIFIER.contains(chars[1]);
    }
    
    // For more complex sequences, check if all characters are emoji-related
    chars.iter().all(|&ch| {
        EMOJI.contains(ch) || 
        EMOJI_COMPONENT.contains(ch) || 
        EMOJI_MODIFIER.contains(ch) ||
        ch == '\u{200D}' || // Zero Width Joiner
        ch == '\u{FE0F}' || // Variation Selector-16 (emoji presentation)
        ch == '\u{FE0E}'    // Variation Selector-15 (text presentation)
    })
}

/// Check if a string contains any emoji characters
pub fn contains_emoji(s: &str) -> bool {
    s.chars().any(|ch| EMOJI.contains(ch))
}

/// Extract all emoji sequences from a string
pub fn extract_emojis(s: &str) -> Vec<String> {
    let mut emojis = Vec::new();
    let mut current_emoji = String::new();
    let mut in_emoji_sequence = false;
    
    for ch in s.chars() {
        if EMOJI.contains(ch) || EMOJI_COMPONENT.contains(ch) {
            current_emoji.push(ch);
            in_emoji_sequence = true;
        } else if in_emoji_sequence && (ch == '\u{200D}' || ch == '\u{FE0F}' || ch == '\u{FE0E}' || EMOJI_MODIFIER.contains(ch)) {
            // Continue the emoji sequence
            current_emoji.push(ch);
        } else {
            // End of emoji sequence
            if !current_emoji.is_empty() {
                emojis.push(current_emoji.clone());
                current_emoji.clear();
            }
            in_emoji_sequence = false;
        }
    }
    
    // Add the last emoji if we ended with one
    if !current_emoji.is_empty() {
        emojis.push(current_emoji);
    }
    
    emojis
}

/// Replace all emojis in a string with a replacement string
pub fn replace_emojis(s: &str, replacement: &str) -> String {
    let mut result = String::new();
    let mut current_pos = 0;
    
    for emoji in extract_emojis(s) {
        // Find the position of this emoji in the original string
        if let Some(pos) = s[current_pos..].find(&emoji) {
            let actual_pos = current_pos + pos;
            // Add text before the emoji
            result.push_str(&s[current_pos..actual_pos]);
            // Add replacement
            result.push_str(replacement);
            // Update position
            current_pos = actual_pos + emoji.len();
        }
    }
    
    // Add remaining text
    result.push_str(&s[current_pos..]);
    
    result
}

/// Get the name of an emoji
pub fn get_emoji_name(emoji: &str) -> String {
    if let Some(&name) = EMOJI_NAMES.get(emoji) {
        name.to_string()
    } else {
        // Try to get the first character if it's a sequence
        if let Some(first_char) = emoji.chars().next() {
            let single_char_emoji = first_char.to_string();
            if let Some(&name) = EMOJI_NAMES.get(single_char_emoji.as_str()) {
                format!("{} (sequence)", name)
            } else {
                format!("UNKNOWN EMOJI: {}", emoji)
            }
        } else {
            "UNKNOWN EMOJI".to_string()
        }
    }
}

/// Find an emoji by its name
pub fn find_emoji_by_name(name: &str) -> GlyphGangResult<String> {
    let name_upper = name.to_uppercase();
    
    for (&emoji, &emoji_name) in EMOJI_NAMES.iter() {
        if emoji_name.to_uppercase() == name_upper {
            return Ok(emoji.to_string());
        }
    }
    
    Err(emoji_error(&format!("Emoji not found: {}", name)))
}

/// Get all available emoji categories
pub fn emoji_categories() -> Vec<String> {
    EMOJI_CATEGORIES.keys().map(|&k| k.to_string()).collect()
}

/// Get all emojis in a specific category
pub fn emojis_in_category(category: &str) -> Vec<String> {
    if let Some(emojis) = EMOJI_CATEGORIES.get(category) {
        emojis.iter().map(|&e| e.to_string()).collect()
    } else {
        Vec::new()
    }
}

/// Initialize emoji data
pub fn initialize_emoji_data() -> GlyphGangResult<()> {
    // Force initialization of lazy statics
    Lazy::force(&EMOJI_NAMES);
    Lazy::force(&EMOJI_CATEGORIES);
    
    // Validate that data loaded correctly
    if EMOJI_NAMES.is_empty() {
        return Err(emoji_error("Failed to initialize emoji name database"));
    }
    
    if EMOJI_CATEGORIES.is_empty() {
        return Err(emoji_error("Failed to initialize emoji categories"));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_detection() {
        assert!(is_emoji_sequence("😀"));
        assert!(is_emoji_sequence("👨‍👩‍👧‍👦")); // Family emoji sequence
        assert!(!is_emoji_sequence("Hello"));
        assert!(!is_emoji_sequence(""));
        
        assert!(contains_emoji("Hello 😀 World"));
        assert!(contains_emoji("😀"));
        assert!(!contains_emoji("Hello World"));
        assert!(!contains_emoji(""));
    }

    #[test]
    fn test_emoji_extraction() {
        let emojis = extract_emojis("Hello 😀 World 🌍!");
        assert_eq!(emojis.len(), 2);
        assert!(emojis.contains(&"😀".to_string()));
        assert!(emojis.contains(&"🌍".to_string()));
        
        let emojis = extract_emojis("No emojis here");
        assert_eq!(emojis.len(), 0);
        
        let emojis = extract_emojis("😀😁😂");
        assert_eq!(emojis.len(), 3);
    }

    #[test]
    fn test_emoji_replacement() {
        let result = replace_emojis("Hello 😀 World 🌍!", "[EMOJI]");
        assert_eq!(result, "Hello [EMOJI] World [EMOJI]!");
        
        let result = replace_emojis("No emojis", "[EMOJI]");
        assert_eq!(result, "No emojis");
        
        let result = replace_emojis("😀😁😂", "X");
        assert_eq!(result, "XXX");
    }

    #[test]
    fn test_emoji_names() {
        assert_eq!(get_emoji_name("😀"), "GRINNING FACE");
        assert_eq!(get_emoji_name("❤️"), "HEAVY BLACK HEART");
        assert!(get_emoji_name("unknown_emoji").contains("UNKNOWN"));
        
        assert_eq!(find_emoji_by_name("GRINNING FACE").unwrap(), "😀");
        assert_eq!(find_emoji_by_name("HEAVY BLACK HEART").unwrap(), "❤️");
        assert!(find_emoji_by_name("NONEXISTENT EMOJI").is_err());
    }

    #[test]
    fn test_emoji_categories() {
        let categories = emoji_categories();
        assert!(categories.contains(&"smileys_emotion".to_string()));
        assert!(categories.contains(&"hearts".to_string()));
        assert!(categories.contains(&"food_drink".to_string()));
        
        let smileys = emojis_in_category("smileys_emotion");
        assert!(smileys.contains(&"😀".to_string()));
        assert!(smileys.contains(&"😂".to_string()));
        
        let hearts = emojis_in_category("hearts");
        assert!(hearts.contains(&"❤️".to_string()));
        assert!(hearts.contains(&"💙".to_string()));
        
        let unknown = emojis_in_category("unknown_category");
        assert_eq!(unknown.len(), 0);
    }

    #[test]
    fn test_initialization() {
        assert!(initialize_emoji_data().is_ok());
    }
}
