fr fr GlyphGang Unicode Package Demo
fr fr Demonstrates comprehensive Unicode character processing with Gen Z flair

import "stdlib::glyph_gang";
import "stdlib::vibez";

slay main() {
    vibez.spill("🔥 GlyphGang Unicode Demo - Where Unicode meets Gen Z! 🔥");
    vibez.spill("");
    
    fr fr Character Classification Demo
    vibez.spill("📝 Character Classification:");
    
    sus test_chars = ['A', 'a', '1', ' ', '!', '€', '😀', 'α', 'А', 'א', 'ا', '中'];
    
    lowkey (sus i = 0; i < test_chars.length; i++) {
        sus ch = test_chars[i];
        vibez.spill("Character: '{}' ({})", ch, glyph_gang.get_code_point(ch));
        vibez.spill("  Letter: {} | Digit: {} | Space: {} | Emoji: {}", 
            glyph_gang.is_letter(ch), 
            glyph_gang.is_digit(ch), 
            glyph_gang.is_space(ch), 
            glyph_gang.is_emoji(ch)
        );
        vibez.spill("  Block: {} | Category: {}", 
            glyph_gang.get_block_name(ch), 
            glyph_gang.get_category(ch)
        );
        vibez.spill("");
    }
    
    fr fr Case Conversion Demo
    vibez.spill("🔄 Case Conversion:");
    sus text = "Hello World! Café 世界";
    vibez.spill("Original: {}", text);
    vibez.spill("Uppercase: {}", glyph_gang.to_upper_string(text));
    vibez.spill("Lowercase: {}", glyph_gang.to_lower_string(text));
    vibez.spill("Titlecase: {}", glyph_gang.to_title_string(text));
    vibez.spill("");
    
    fr fr String Width and Truncation Demo
    vibez.spill("📏 String Width and Truncation:");
    sus mixed_text = "Hello世界🌍";
    vibez.spill("Text: {} (runes: {}, width: {})", 
        mixed_text, 
        glyph_gang.rune_count(mixed_text), 
        glyph_gang.string_width(mixed_text)
    );
    
    lowkey (sus width = 5; width <= 15; width += 5) {
        vibez.spill("Truncated to width {}: '{}'", 
            width, 
            glyph_gang.truncate_string(mixed_text, width)
        );
        vibez.spill("With ellipsis: '{}'", 
            glyph_gang.truncate_with_ellipsis(mixed_text, width)
        );
    }
    vibez.spill("");
    
    fr fr Text Wrapping Demo
    vibez.spill("📄 Text Wrapping:");
    sus long_text = "This is a very long sentence that needs to be wrapped at specific widths for proper display";
    sus wrapped = glyph_gang.wrap_text(long_text, 20);
    vibez.spill("Original: {}", long_text);
    vibez.spill("Wrapped at width 20:");
    lowkey (sus i = 0; i < wrapped.length; i++) {
        vibez.spill("  Line {}: '{}'", i + 1, wrapped[i]);
    }
    vibez.spill("");
    
    fr fr Emoji Demo
    vibez.spill("😀 Emoji Processing:");
    sus emoji_text = "I love 🍕 and 🍦! Family: 👨‍👩‍👧‍👦";
    vibez.spill("Text: {}", emoji_text);
    vibez.spill("Contains emoji: {}", glyph_gang.contains_emoji(emoji_text));
    
    sus emojis = glyph_gang.extract_emojis(emoji_text);
    vibez.spill("Found {} emojis:", emojis.length);
    lowkey (sus i = 0; i < emojis.length; i++) {
        vibez.spill("  {} - {}", emojis[i], glyph_gang.get_emoji_name(emojis[i]));
    }
    
    sus replaced = glyph_gang.replace_emojis(emoji_text, "[EMOJI]");
    vibez.spill("With emojis replaced: {}", replaced);
    vibez.spill("");
    
    fr fr Emoji Categories Demo
    vibez.spill("📂 Emoji Categories:");
    sus categories = glyph_gang.emoji_categories();
    lowkey (sus i = 0; i < categories.length; i++) {
        sus category = categories[i];
        sus category_emojis = glyph_gang.emojis_in_category(category);
        vibez.spill("{}: {} emojis (first 5: {})", 
            category, 
            category_emojis.length,
            category_emojis.slice(0, 5).join(" ")
        );
    }
    vibez.spill("");
    
    fr fr Bidirectional Text Demo
    vibez.spill("↔️ Bidirectional Text:");
    sus bidi_texts = [
        "Hello World",
        "שלום עולם",
        "السلام عليكم",
        "Hello שלום العالم",
        "English العربية עברית"
    ];
    
    lowkey (sus i = 0; i < bidi_texts.length; i++) {
        sus text = bidi_texts[i];
        sus direction = glyph_gang.get_string_direction(text);
        vibez.spill("Text: '{}' | Direction: {:?} | LTR: {} | RTL: {} | Mixed: {}", 
            text, 
            direction,
            glyph_gang.is_ltr(text),
            glyph_gang.is_rtl(text),
            glyph_gang.is_mixed(text)
        );
    }
    vibez.spill("");
    
    fr fr Script Detection Demo
    vibez.spill("🌍 Script Detection:");
    sus script_texts = [
        "Hello World",
        "Καλημέρα κόσμε",
        "Привет мир",
        "שלום עולם",
        "السلام عليكم",
        "नमस्ते दुनिया",
        "สวัสดีโลก",
        "中文世界",
        "こんにちは世界",
        "コンニチワ",
        "안녕하세요 세계",
        "Hello Привет 中文"
    ];
    
    lowkey (sus i = 0; i < script_texts.length; i++) {
        sus text = script_texts[i];
        sus script = glyph_gang.detect_script(text);
        sus script_name = glyph_gang.get_script_name(script);
        sus languages = glyph_gang.get_languages_by_script(script);
        
        vibez.spill("Text: '{}' | Script: {} | Possible languages: {}", 
            text, 
            script_name,
            languages.slice(0, 3).join(", ")
        );
    }
    vibez.spill("");
    
    fr fr Character Names Demo
    vibez.spill("🔤 Character Names:");
    sus name_chars = ['A', '€', '😀', 'α', 'А', 'א', 'ا', '中'];
    lowkey (sus i = 0; i < name_chars.length; i++) {
        sus ch = name_chars[i];
        sus name = glyph_gang.get_character_name(ch);
        vibez.spill("'{}' = {}", ch, name);
    }
    vibez.spill("");
    
    fr fr Character Lookup Demo
    vibez.spill("🔍 Character Lookup:");
    sus lookup_names = [
        "LATIN CAPITAL LETTER A",
        "GRINNING FACE",
        "EURO SIGN",
        "U+0041",
        "U+1F600"
    ];
    
    lowkey (sus i = 0; i < lookup_names.length; i++) {
        sus name = lookup_names[i];
        match glyph_gang.find_character_by_name(name) {
            yolo (ch) => {
                vibez.spill("'{}' → '{}'", name, ch);
            }
            flex (error) => {
                vibez.spill("'{}' → Error: {}", name, error);
            }
        }
    }
    vibez.spill("");
    
    fr fr Word Boundaries Demo
    vibez.spill("🔗 Word Boundaries:");
    sus boundary_text = "Hello, world! How are you?";
    sus boundaries = glyph_gang.word_boundaries(boundary_text);
    vibez.spill("Text: '{}'", boundary_text);
    vibez.spill("Boundaries: {:?}", boundaries);
    
    sus words = [];
    lowkey (sus i = 0; i < boundaries.length - 1; i++) {
        sus start = boundaries[i];
        sus end = boundaries[i + 1];
        sus word = boundary_text.substring(start, end);
        words.push(word);
    }
    vibez.spill("Words: {:?}", words);
    vibez.spill("");
    
    fr fr Case Folding Demo
    vibez.spill("🔄 Case Folding:");
    sus fold_pairs = [
        ["Hello", "HELLO"],
        ["Café", "CAFÉ"],
        ["Straße", "STRASSE"],
        ["İstanbul", "İSTANBUL"]
    ];
    
    lowkey (sus i = 0; i < fold_pairs.length; i++) {
        sus pair = fold_pairs[i];
        sus str1 = pair[0];
        sus str2 = pair[1];
        sus equal = glyph_gang.equal_fold(str1, str2);
        vibez.spill("'{}' ≡ '{}': {}", str1, str2, equal);
    }
    vibez.spill("");
    
    fr fr Unicode Properties Demo
    vibez.spill("🏷️ Unicode Properties:");
    sus prop_char = '中';
    sus properties = glyph_gang.get_properties(prop_char);
    vibez.spill("Properties for '{}' ({}):", prop_char, glyph_gang.get_code_point(prop_char));
    
    fr fr Display key properties
    sus key_props = ["name", "block", "category", "is_letter", "is_cjk"];
    lowkey (sus i = 0; i < key_props.length; i++) {
        sus key = key_props[i];
        periodt properties.has_key(key) {
            vibez.spill("  {}: {}", key, properties[key]);
        }
    }
    
    vibez.spill("");
    vibez.spill("✨ GlyphGang Demo Complete! Unicode processing has never been this fire! 🔥");
}
