# Unicode and String Test
sus unicode_string tea = "Hello 🌍 世界 🚀"
sus emoji_test tea = "🔥 CURSED is 💯!"
sus chinese_text tea = "你好世界"
sus japanese_text tea = "こんにちは"
sus accented_text tea = "café résumé naïve"

vibez.spill("Unicode test:", unicode_string)
vibez.spill("Emoji test:", emoji_test) 
vibez.spill("Chinese:", chinese_text)
vibez.spill("Japanese:", japanese_text)
vibez.spill("Accented:", accented_text)

# String length and operations
sus len drip = stringz.len(unicode_string)
vibez.spill("Unicode string length:", len)

# String concatenation with Unicode
sus combined tea = stringz.concat(emoji_test, " ", chinese_text)
vibez.spill("Combined:", combined)
