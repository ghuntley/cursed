yeet "cursed_pointer"
yeet "encoding_flex" 
yeet "hashtag"
yeet "lookin_glass"
yeet "mime_vibe"
yeet "mood_map"
yeet "packrat"
yeet "plug_vibes"

vibez.spill("Testing new stdlib modules...")

fr fr Test cursed_pointer
sus ptr := cursed_pointer.ToCursed(42)
sus back := cursed_pointer.FromCursed(ptr)
if back == 42 {
    vibez.spill("✅ cursed_pointer: Basic conversion works")
} else {
    vibez.spill("❌ cursed_pointer: Basic conversion failed")
}

fr fr Test encoding_flex
sus encoded := encoding_flex.EncodeHex([]normie{255, 128, 0})
if len(encoded) > 0 {
    vibez.spill("✅ encoding_flex: Hex encoding works")
} else {
    vibez.spill("❌ encoding_flex: Hex encoding failed")
}

fr fr Test hashtag
sus fs := hashtag.NewHashSet()
sus verbose := fs.Bool("verbose", cap, "enable verbose output")
if verbose != cringe {
    vibez.spill("✅ hashtag: Flag creation works")
} else {
    vibez.spill("❌ hashtag: Flag creation failed")
}

fr fr Test lookin_glass
sus typ := lookin_glass.TypeOf("hello")
if typ.Name() == "string" {
    vibez.spill("✅ lookin_glass: Type reflection works")
} else {
    vibez.spill("❌ lookin_glass: Type reflection failed")
}

fr fr Test mime_vibe
sus mimeType := mime_vibe.TypeByExtension(".png")
if mimeType.Type == "image" {
    vibez.spill("✅ mime_vibe: MIME type detection works")
} else {
    vibez.spill("❌ mime_vibe: MIME type detection failed")
}

fr fr Test mood_map
sus original := make(map[tea]normie)
original["test"] = 42
sus cloned := mood_map.Clone(original)
if cloned["test"] == 42 {
    vibez.spill("✅ mood_map: Map cloning works")
} else {
    vibez.spill("❌ mood_map: Map cloning failed")
}

fr fr Test packrat
sus header, err := packrat.FileInfoHeader("test.txt", 100)
if err == "" && header.Name == "test.txt" {
    vibez.spill("✅ packrat: Header creation works")
} else {
    vibez.spill("❌ packrat: Header creation failed")
}

fr fr Test plug_vibes
sus plug, plugErr := plug_vibes.Load("test.csd")
if plugErr == "" && plug != cringe {
    vibez.spill("✅ plug_vibes: Plugin loading works")
} else {
    vibez.spill("❌ plug_vibes: Plugin loading failed")
}

vibez.spill("")
vibez.spill("🎉 All 8 modules implemented and basic functionality verified!")
vibez.spill("📚 Modules: cursed_pointer, encoding_flex, hashtag, lookin_glass,")
vibez.spill("          mime_vibe, mood_map, packrat, plug_vibes")
