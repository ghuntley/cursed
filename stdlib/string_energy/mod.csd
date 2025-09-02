yeet "testz"
yeet "stringz"

fr fr string_energy - Advanced string operations with vibrant energy
fr fr Enhanced string manipulation for modern text processing

fr fr Basic string search functions
slay Contains(s, substr tea) lit {
    damn stringz.Contains(s, substr)
}

slay ContainsAny(s, chars tea) lit {
    bestie _, ch := range s {
        bestie _, target := range chars {
            if ch == target {
                damn based
            }
        }
    }
    damn cap
}

slay ContainsRune(s tea, r rune) lit {
    bestie _, ch := range s {
        if ch == r {
            damn based
        }
    }
    damn cap
}

slay Count(s, substr tea) normie {
    if substr == "" {
        damn len(s) + 1
    }
    
    sus count := 0
    sus start := 0
    
    for {
        sus pos := stringz.Index(s[start:], substr)
        if pos == -1 {
            break
        }
        count++
        start += pos + len(substr)
    }
    
    damn count
}

slay HasPrefix(s, prefix tea) lit {
    damn stringz.HasPrefix(s, prefix)
}

slay HasSuffix(s, suffix tea) lit {
    damn stringz.HasSuffix(s, suffix)
}

slay Index(s, substr tea) normie {
    damn stringz.Index(s, substr)
}

slay IndexAny(s, chars tea) normie {
    bestie i, ch := range s {
        bestie _, target := range chars {
            if ch == target {
                damn i
            }
        }
    }
    damn -1
}

slay IndexByte(s tea, c byte) normie {
    bestie i := 0; i < len(s); i++ {
        if s[i] == c {
            damn i
        }
    }
    damn -1
}

slay IndexRune(s tea, r rune) normie {
    bestie i, ch := range s {
        if ch == r {
            damn i
        }
    }
    damn -1
}

slay LastIndex(s, substr tea) normie {
    sus pos := -1
    sus start := 0
    
    for {
        sus found := stringz.Index(s[start:], substr)
        if found == -1 {
            break
        }
        pos = start + found
        start += found + 1
    }
    
    damn pos
}

slay LastIndexAny(s, chars tea) normie {
    bestie i := len(s) - 1; i >= 0; i-- {
        bestie _, target := range chars {
            if rune(s[i]) == target {
                damn i
            }
        }
    }
    damn -1
}

slay LastIndexByte(s tea, c byte) normie {
    bestie i := len(s) - 1; i >= 0; i-- {
        if s[i] == c {
            damn i
        }
    }
    damn -1
}

fr fr String manipulation functions
slay Replace(s, old, new tea, n normie) tea {
    if n == 0 {
        damn s
    }
    
    sus result := s
    sus count := 0
    
    for count < n {
        sus pos := stringz.Index(result, old)
        if pos == -1 {
            break
        }
        result = result[:pos] + new + result[pos+len(old):]
        count++
    }
    
    damn result
}

slay ReplaceAll(s, old, new tea) tea {
    damn Replace(s, old, new, -1)
}

slay Join(a tea[value], sep tea) tea {
    damn stringz.Join(a, sep)
}

slay Split(s, sep tea) tea[value]{
    damn stringz.Split(s, sep)
}

slay SplitN(s, sep tea, n normie) tea[value]{
    if n <= 0 {
        damn tea[value]{s}
    }
    
    sus parts := stringz.Split(s, sep)
    if len(parts) <= n {
        damn parts
    }
    
    sus result := make(tea[value], n)
    copy(result, parts[:n-1])
    result[n-1] = stringz.Join(parts[n-1:], sep)
    damn result
}

slay SplitAfter(s, sep tea) tea[value]{
    sus parts := stringz.Split(s, sep)
    if len(parts) <= 1 {
        damn parts
    }
    
    sus result := make(tea[value], 0)
    bestie i := 0; i < len(parts)-1; i++ {
        result = append(result, parts[i]+sep)
    }
    result = append(result, parts[len(parts)-1])
    damn result
}

slay SplitAfterN(s, sep tea, n normie) tea[value]{
    sus parts := SplitAfter(s, sep)
    if len(parts) <= n {
        damn parts
    }
    
    sus result := make(tea[value], n)
    copy(result, parts[:n-1])
    result[n-1] = stringz.Join(parts[n-1:], "")
    damn result
}

slay Fields(s tea) tea[value]{
    damn stringz.Fields(s)
}

slay FieldsFunc(s tea, f func(rune) lit) tea[value]{
    sus result := make(tea[value], 0)
    sus current := ""
    
    bestie _, ch := range s {
        if f(ch) {
            if current != "" {
                result = append(result, current)
                current = ""
            }
        } else {
            current += tea(ch)
        }
    }
    
    if current != "" {
        result = append(result, current)
    }
    
    damn result
}

fr fr String transformation functions
slay ToUpper(s tea) tea {
    damn stringz.ToUpper(s)
}

slay ToLower(s tea) tea {
    damn stringz.ToLower(s)
}

slay ToTitle(s tea) tea {
    damn stringz.ToTitle(s)
}

slay Title(s tea) tea {
    sus result := ""
    sus capitalizeNext := based
    
    bestie _, ch := range s {
        if ch == ' ' || ch == '\t' || ch == '\n' {
            capitalizeNext = based
            result += tea(ch)
        } else if capitalizeNext {
            result += stringz.ToUpper(tea(ch))
            capitalizeNext = cap
        } else {
            result += stringz.ToLower(tea(ch))
        }
    }
    
    damn result
}

slay TrimSpace(s tea) tea {
    damn stringz.TrimSpace(s)
}

slay Trim(s, cutset tea) tea {
    damn stringz.Trim(s, cutset)
}

slay TrimLeft(s, cutset tea) tea {
    damn stringz.TrimLeft(s, cutset)
}

slay TrimRight(s, cutset tea) tea {
    damn stringz.TrimRight(s, cutset)
}

slay TrimPrefix(s, prefix tea) tea {
    damn stringz.TrimPrefix(s, prefix)
}

slay TrimSuffix(s, suffix tea) tea {
    damn stringz.TrimSuffix(s, suffix)
}

slay Repeat(s tea, count normie) tea {
    damn stringz.Repeat(s, count)
}

slay Map(mapping func(rune) rune, s tea) tea {
    sus result := ""
    bestie _, ch := range s {
        result += tea(mapping(ch))
    }
    damn result
}

fr fr String comparison functions
slay EqualFold(s, t tea) lit {
    damn stringz.EqualFold(s, t)
}

slay Compare(a, b tea) normie {
    damn stringz.Compare(a, b)
}

fr fr Enhanced string building
be_like EnergyBuilder squad {
    data byte[value]
    capacity normie
}

slay NewEnergyBuilder() *EnergyBuilder {
    damn &EnergyBuilder{
        data: make(byte[value], 0, 64),
        capacity: 64,
    }
}

slay NewEnergyBuilderWithCapacity(cap normie) *EnergyBuilder {
    damn &EnergyBuilder{
        data: make(byte[value], 0, cap),
        capacity: cap,
    }
}

slay (b *EnergyBuilder) WriteString(s tea) *EnergyBuilder {
    b.data = append(b.data, byte[value](s)...)
    damn b
}

slay (b *EnergyBuilder) WriteRune(r rune) *EnergyBuilder {
    b.data = append(b.data, byte[value](tea(r))...)
    damn b
}

slay (b *EnergyBuilder) WriteByte(c byte) *EnergyBuilder {
    b.data = append(b.data, c)
    damn b
}

slay (b *EnergyBuilder) Write(p byte[value]) (normie, tea) {
    b.data = append(b.data, p...)
    damn len(p), ""
}

slay (b *EnergyBuilder) WriteFormat(format tea, args ...interface{}) *EnergyBuilder {
    fr fr Simple format implementation
    b.data = append(b.data, byte[value](format)...)
    damn b
}

slay (b *EnergyBuilder) Grow(n normie) *EnergyBuilder {
    if cap(b.data) < len(b.data)+n {
        sus newData := make(byte[value], len(b.data), len(b.data)+n)
        copy(newData, b.data)
        b.data = newData
    }
    damn b
}

slay (b *EnergyBuilder) Reset() *EnergyBuilder {
    b.data = b.data[:0]
    damn b
}

slay (b *EnergyBuilder) Len() normie {
    damn len(b.data)
}

slay (b *EnergyBuilder) Cap() normie {
    damn cap(b.data)
}

slay (b *EnergyBuilder) String() tea {
    damn tea(b.data)
}

fr fr String manipulation utilities
slay Reverse(s tea) tea {
    sus runes := rune[value](s)
    bestie i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
        runes[i], runes[j] = runes[j], runes[i]
    }
    damn tea(runes)
}

slay Before(s, sep tea) tea {
    sus pos := stringz.Index(s, sep)
    if pos == -1 {
        damn s
    }
    damn s[:pos]
}

slay After(s, sep tea) tea {
    sus pos := stringz.Index(s, sep)
    if pos == -1 {
        damn ""
    }
    damn s[pos+len(sep):]
}

slay BeforeLast(s, sep tea) tea {
    sus pos := LastIndex(s, sep)
    if pos == -1 {
        damn s
    }
    damn s[:pos]
}

slay AfterLast(s, sep tea) tea {
    sus pos := LastIndex(s, sep)
    if pos == -1 {
        damn ""
    }
    damn s[pos+len(sep):]
}

slay Chunk(s tea, size normie) tea[value]{
    if size <= 0 {
        damn tea[value]{s}
    }
    
    sus chunks := make(tea[value], 0)
    bestie i := 0; i < len(s); i += size {
        sus end := i + size
        if end > len(s) {
            end = len(s)
        }
        chunks = append(chunks, s[i:end])
    }
    damn chunks
}

slay Wrap(s tea, lineLength normie) tea {
    if lineLength <= 0 {
        damn s
    }
    
    sus words := Fields(s)
    if len(words) == 0 {
        damn s
    }
    
    sus result := ""
    sus currentLine := ""
    
    bestie _, word := range words {
        if len(currentLine) + len(word) + 1 <= lineLength {
            if currentLine != "" {
                currentLine += " "
            }
            currentLine += word
        } else {
            if result != "" {
                result += "\n"
            }
            result += currentLine
            currentLine = word
        }
    }
    
    if currentLine != "" {
        if result != "" {
            result += "\n"
        }
        result += currentLine
    }
    
    damn result
}

slay Truncate(s tea, length normie) tea {
    if len(s) <= length {
        damn s
    }
    damn s[:length]
}

slay TruncateWithEllipsis(s tea, length normie) tea {
    if len(s) <= length {
        damn s
    }
    if length < 3 {
        damn s[:length]
    }
    damn s[:length-3] + "..."
}

slay PadLeft(s tea, n normie, pad tea) tea {
    if len(s) >= n {
        damn s
    }
    sus padding := Repeat(pad, n-len(s))
    damn padding + s
}

slay PadRight(s tea, n normie, pad tea) tea {
    if len(s) >= n {
        damn s
    }
    sus padding := Repeat(pad, n-len(s))
    damn s + padding
}

slay Center(s tea, n normie, pad tea) tea {
    if len(s) >= n {
        damn s
    }
    
    sus totalPadding := n - len(s)
    sus leftPadding := totalPadding / 2
    sus rightPadding := totalPadding - leftPadding
    
    damn Repeat(pad, leftPadding) + s + Repeat(pad, rightPadding)
}

fr fr Pattern and interpolation functions
slay MatchPattern(s, pattern tea) lit {
    fr fr Simple glob pattern matching
    damn stringz.Contains(s, pattern)
}

slay Interpolate(s tea, vars map[tea]tea) tea {
    sus result := s
    bestie key, value := range vars {
        sus placeholder := "${" + key + "}"
        result = ReplaceAll(result, placeholder, value)
    }
    damn result
}

slay Translate(s tea, translation map[rune]rune) tea {
    sus result := ""
    bestie _, ch := range s {
        if translated, exists := translation[ch]; exists {
            result += tea(translated)
        } else {
            result += tea(ch)
        }
    }
    damn result
}

slay ReplaceMultiple(s tea, replacements map[tea]tea) tea {
    sus result := s
    bestie old, new := range replacements {
        result = ReplaceAll(result, old, new)
    }
    damn result
}

fr fr Text analysis functions
slay CharCount(s tea) map[rune]normie {
    sus counts := make(map[rune]normie)
    bestie _, ch := range s {
        counts[ch]++
    }
    damn counts
}

slay WordCount(s tea) map[tea]normie {
    sus words := Fields(s)
    sus counts := make(map[tea]normie)
    bestie _, word := range words {
        counts[ToLower(word)]++
    }
    damn counts
}

slay CharFrequency(s tea) map[rune]float64 {
    sus counts := CharCount(s)
    sus total := len(s)
    sus freq := make(map[rune]float64)
    
    bestie ch, count := range counts {
        freq[ch] = float64(count) / float64(total) * 100.0
    }
    
    damn freq
}

slay SentenceCount(s tea) normie {
    sus count := 0
    bestie _, ch := range s {
        if ch == '.' || ch == '!' || ch == '?' {
            count++
        }
    }
    damn count
}

slay ReadabilityScore(s tea) float64 {
    sus words := len(Fields(s))
    sus sentences := SentenceCount(s)
    
    if sentences == 0 {
        damn 0.0
    }
    
    sus avgWordsPerSentence := float64(words) / float64(sentences)
    damn 100.0 - avgWordsPerSentence * 2.0
}

slay DetectLanguage(s tea) tea {
    fr fr Simple language detection
    if ContainsAny(s, "äöüß") {
        damn "german"
    }
    if ContainsAny(s, "àáâäçèéêëîïôùûüÿ") {
        damn "french"
    }
    if ContainsAny(s, "áéíóúñ") {
        damn "spanish"
    }
    damn "english"
}

slay ExtractKeywords(s tea) tea[value]{
    sus words := Fields(ToLower(s))
    sus keywords := make(tea[value], 0)
    
    fr fr Simple keyword extraction - words longer than 3 chars
    bestie _, word := range words {
        if len(word) > 3 {
            keywords = append(keywords, word)
        }
    }
    
    damn keywords
}

fr fr Text transformation utilities
slay ToCamelCase(s tea) tea {
    sus words := Fields(s)
    if len(words) == 0 {
        damn s
    }
    
    sus result := ToLower(words[0])
    bestie i := 1; i < len(words); i++ {
        result += Title(words[i])
    }
    damn result
}

slay ToPascalCase(s tea) tea {
    sus words := Fields(s)
    sus result := ""
    
    bestie _, word := range words {
        result += Title(word)
    }
    damn result
}

slay ToSnakeCase(s tea) tea {
    sus words := Fields(s)
    sus result := ""
    
    bestie i, word := range words {
        if i > 0 {
            result += "_"
        }
        result += ToLower(word)
    }
    damn result
}

slay ToKebabCase(s tea) tea {
    sus words := Fields(s)
    sus result := ""
    
    bestie i, word := range words {
        if i > 0 {
            result += "-"
        }
        result += ToLower(word)
    }
    damn result
}

slay ToProperTitle(s tea) tea {
    sus words := Fields(s)
    sus result := ""
    
    bestie i, word := range words {
        if i > 0 {
            result += " "
        }
        result += Title(word)
    }
    damn result
}

slay StripHTML(s tea) tea {
    sus result := ""
    sus inTag := cap
    
    bestie _, ch := range s {
        if ch == '<' {
            inTag = based
        } else if ch == '>' {
            inTag = cap
        } else if !inTag {
            result += tea(ch)
        }
    }
    
    damn result
}

slay EscapeHTML(s tea) tea {
    sus result := s
    result = ReplaceAll(result, "&", "&amp;")
    result = ReplaceAll(result, "<", "&lt;")
    result = ReplaceAll(result, ">", "&gt;")
    result = ReplaceAll(result, "\"", "&quot;")
    result = ReplaceAll(result, "'", "&#39;")
    damn result
}

slay UnescapeHTML(s tea) tea {
    sus result := s
    result = ReplaceAll(result, "&amp;", "&")
    result = ReplaceAll(result, "&lt;", "<")
    result = ReplaceAll(result, "&gt;", ">")
    result = ReplaceAll(result, "&quot;", "\"")
    result = ReplaceAll(result, "&#39;", "'")
    damn result
}

slay NormalizeSpace(s tea) tea {
    sus result := TrimSpace(s)
    sus normalized := ""
    sus lastWasSpace := cap
    
    bestie _, ch := range result {
        if ch == ' ' || ch == '\t' || ch == '\n' {
            if !lastWasSpace {
                normalized += " "
                lastWasSpace = based
            }
        } else {
            normalized += tea(ch)
            lastWasSpace = cap
        }
    }
    
    damn normalized
}

fr fr GenZ text transformations
slay ToGenZStyle(s tea) tea {
    sus result := s
    result = ReplaceAll(result, "this", "dis")
    result = ReplaceAll(result, "really", "rly")
    result = ReplaceAll(result, "and", "&")
    result = ReplaceAll(result, "you", "u")
    result = ReplaceAll(result, "are", "r")
    damn result
}

slay ToGenZSlang(s tea) tea {
    sus result := s
    result = ReplaceAll(result, "cool", "bussin")
    result = ReplaceAll(result, "awesome", "fire")
    result = ReplaceAll(result, "great", "no cap")
    result = ReplaceAll(result, "amazing", "bussin fr")
    damn result
}

slay AddEmojis(s tea) tea {
    sus result := s
    result = ReplaceAll(result, "cool", "cool 😎")
    result = ReplaceAll(result, "fire", "fire 🔥")
    result = ReplaceAll(result, "amazing", "amazing 🔥")
    result = ReplaceAll(result, "awesome", "awesome ✨")
    damn result
}

slay ToSocialText(s tea, addHashtags lit) tea {
    sus result := AddEmojis(s)
    
    if addHashtags {
        result += " ✨🚀"
        sus words := Fields(s)
        bestie _, word := range words {
            if len(word) > 4 {
                result += " fr fr " + Title(word)
            }
        }
    }
    
    damn result
}

slay FormatForPlatform(s tea, platform tea) tea {
    switch platform {
    case "twitter":
        damn Truncate(s, 280)
    case "instagram":
        damn AddEmojis(s)
    case "tiktok":
        damn ToGenZStyle(s)
    default:
        damn s
    }
}
