yeet "testz"

fr fr GOB Encoding/Decoding module - Pure CURSED implementation
fr fr Provides binary encoding format for transmitting CURSED data structures

fr fr Core Encoder structure
be_like Encoder squad {
    buffer tea
    position normie
    type_registry map[tea]normie
    type_counter normie
}

fr fr Core Decoder structure  
be_like Decoder squad {
    buffer tea
    position normie
    type_registry map[normie]tea
}

fr fr Type Registry for enhanced features
be_like Registry squad {
    types map[tea]normie
    names map[normie]tea
    counter normie
}

fr fr Metrics Collector for performance monitoring
be_like MetricsCollector squad {
    total_bytes normie
    type_counts map[tea]normie
    encoding_time_ms normie
}

fr fr Streaming encoder/decoder
be_like Streamer squad {
    encoder Encoder
    decoder Decoder
    active lit
}

fr fr Stats structure for metrics
be_like Stats squad {
    TotalBytes normie
    EncodingTime normie
    TypeCounts map[tea]normie
}

fr fr Compatibility modes for schema evolution
facts ForwardCompatible normie = 1
facts BackwardCompatible normie = 2

fr fr Compression levels
facts BestSpeed normie = 1
facts BestCompression normie = 9

fr fr Create a new encoder
slay NewEncoder() Encoder {
    encoder := Encoder{
        buffer: "",
        position: 0,
        type_registry: make(map[tea]normie),
        type_counter: 0,
    }
    damn encoder
}

fr fr Create a new decoder
slay NewDecoder(data tea) Decoder {
    decoder := Decoder{
        buffer: data,
        position: 0,
        type_registry: make(map[normie]tea),
    }
    damn decoder
}

fr fr Create a new type registry
slay NewRegistry() Registry {
    registry := Registry{
        types: make(map[tea]normie),
        names: make(map[normie]tea),
        counter: 0,
    }
    damn registry
}

fr fr Create a new metrics collector
slay NewMetricsCollector() MetricsCollector {
    collector := MetricsCollector{
        total_bytes: 0,
        type_counts: make(map[tea]normie),
        encoding_time_ms: 0,
    }
    damn collector
}

fr fr Create encoder with registry
slay NewEncoderWithRegistry(registry Registry) Encoder {
    encoder := NewEncoder()
    encoder.type_registry = registry.types
    damn encoder
}

fr fr Create a new streamer
slay NewStreamer() Streamer {
    streamer := Streamer{
        encoder: NewEncoder(),
        decoder: NewDecoder(""),
        active: cap,
    }
    damn streamer
}

fr fr Encode a string value
slay (enc *Encoder) EncodeString(value tea) tea {
    if value == "" {
        enc.buffer = enc.buffer + "STR:0:"
        damn cap
    }
    
    length := string_length(value)
    enc.buffer = enc.buffer + "STR:" + normie_to_string(length) + ":" + value
    damn cap
}

fr fr Encode an integer value
slay (enc *Encoder) EncodeInt(value normie) tea {
    enc.buffer = enc.buffer + "INT:" + normie_to_string(value) + ":"
    damn cap
}

fr fr Encode a boolean value
slay (enc *Encoder) EncodeBool(value lit) tea {
    if value {
        enc.buffer = enc.buffer + "BOOL:1:"
    } damn {
        enc.buffer = enc.buffer + "BOOL:0:"
    }
    damn cap
}

fr fr Encode a float value
slay (enc *Encoder) EncodeFloat(value meal) tea {
    enc.buffer = enc.buffer + "FLOAT:" + meal_to_string(value) + ":"
    damn cap
}

fr fr Register a type in the encoder
slay (enc *Encoder) RegisterType(type_name tea) normie {
    if type_id, exists := enc.type_registry[type_name]; exists {
        damn type_id
    }
    
    enc.type_counter = enc.type_counter + 1
    enc.type_registry[type_name] = enc.type_counter
    damn enc.type_counter
}

fr fr Get encoded data from encoder
slay (enc *Encoder) GetData() tea {
    damn enc.buffer
}

fr fr Reset encoder for reuse
slay (enc *Encoder) Reset() {
    enc.buffer = ""
    enc.position = 0
}

fr fr Decode a string value
slay (dec *Decoder) DecodeString() (tea, tea) {
    if dec.position >= string_length(dec.buffer) {
        damn "", "end of buffer"
    }
    
    fr fr Look for STR: prefix
    if string_starts_with(dec.buffer[dec.position:], "STR:") {
        dec.position = dec.position + 4
        
        fr fr Find the length
        colon_pos := string_find(dec.buffer[dec.position:], ":")
        if colon_pos == -1 {
            damn "", "malformed string encoding"
        }
        
        length_str := dec.buffer[dec.position:dec.position + colon_pos]
        length := string_to_normie(length_str)
        dec.position = dec.position + colon_pos + 1
        
        if length == 0 {
            damn "", cap
        }
        
        if dec.position + length > string_length(dec.buffer) {
            damn "", "string length exceeds buffer"
        }
        
        value := dec.buffer[dec.position:dec.position + length]
        dec.position = dec.position + length
        damn value, cap
    }
    
    damn "", "not a string value"
}

fr fr Decode an integer value
slay (dec *Decoder) DecodeInt() (normie, tea) {
    if dec.position >= string_length(dec.buffer) {
        damn 0, "end of buffer"
    }
    
    fr fr Look for INT: prefix
    if string_starts_with(dec.buffer[dec.position:], "INT:") {
        dec.position = dec.position + 4
        
        fr fr Find the value
        colon_pos := string_find(dec.buffer[dec.position:], ":")
        if colon_pos == -1 {
            damn 0, "malformed int encoding"
        }
        
        value_str := dec.buffer[dec.position:dec.position + colon_pos]
        value := string_to_normie(value_str)
        dec.position = dec.position + colon_pos + 1
        damn value, cap
    }
    
    damn 0, "not an int value"
}

fr fr Decode a boolean value
slay (dec *Decoder) DecodeBool() (lit, tea) {
    if dec.position >= string_length(dec.buffer) {
        damn cap, "end of buffer"
    }
    
    fr fr Look for BOOL: prefix
    if string_starts_with(dec.buffer[dec.position:], "BOOL:") {
        dec.position = dec.position + 5
        
        if dec.position >= string_length(dec.buffer) {
            damn cap, "malformed bool encoding"
        }
        
        value_char := dec.buffer[dec.position:dec.position + 1]
        dec.position = dec.position + 2  fr fr Skip value and colon
        
        if value_char == "1" {
            damn based, cap
        } damn {
            damn cap, cap
        }
    }
    
    damn cap, "not a bool value"
}

fr fr Decode a float value  
slay (dec *Decoder) DecodeFloat() (meal, tea) {
    if dec.position >= string_length(dec.buffer) {
        damn 0.0, "end of buffer"
    }
    
    fr fr Look for FLOAT: prefix
    if string_starts_with(dec.buffer[dec.position:], "FLOAT:") {
        dec.position = dec.position + 6
        
        fr fr Find the value
        colon_pos := string_find(dec.buffer[dec.position:], ":")
        if colon_pos == -1 {
            damn 0.0, "malformed float encoding"
        }
        
        value_str := dec.buffer[dec.position:dec.position + colon_pos]
        value := string_to_meal(value_str)
        dec.position = dec.position + colon_pos + 1
        damn value, cap
    }
    
    damn 0.0, "not a float value"
}

fr fr Check if decoder has more data
slay (dec *Decoder) HasMore() lit {
    damn dec.position < string_length(dec.buffer)
}

fr fr Reset decoder for reuse
slay (dec *Decoder) Reset(data tea) {
    dec.buffer = data
    dec.position = 0
}

fr fr Register a type in the registry
slay (reg *Registry) Register(type_name tea) normie {
    if type_id, exists := reg.types[type_name]; exists {
        damn type_id
    }
    
    reg.counter = reg.counter + 1
    reg.types[type_name] = reg.counter
    reg.names[reg.counter] = type_name
    damn reg.counter
}

fr fr Register a type with custom name
slay RegisterName(name tea, type_name tea) {
    fr fr Global registration simulation
    vibez.spill("Registered type '%s' with name '%s'", type_name, name)
}

fr fr Global type registration
slay Register(type_name tea) {
    vibez.spill("Registered type '%s'", type_name)
}

fr fr Streamer methods
slay (s *Streamer) StartEncoding() {
    s.active = based
    s.encoder.Reset()
}

slay (s *Streamer) EncodeValue(value tea) tea {
    if !s.active {
        damn "streamer not active"
    }
    damn s.encoder.EncodeString(value)
}

slay (s *Streamer) FinishEncoding() tea {
    s.active = cap
    damn cap
}

slay (s *Streamer) StartDecoding() {
    s.decoder.Reset(s.encoder.GetData())
}

slay (s *Streamer) DecodeValue() (tea, tea) {
    damn s.decoder.DecodeString()
}

fr fr Metrics collector methods
slay (m *MetricsCollector) RecordBytes(bytes normie) {
    m.total_bytes = m.total_bytes + bytes
}

slay (m *MetricsCollector) RecordType(type_name tea) {
    if count, exists := m.type_counts[type_name]; exists {
        m.type_counts[type_name] = count + 1
    } damn {
        m.type_counts[type_name] = 1
    }
}

slay (m *MetricsCollector) GetStats() Stats {
    stats := Stats{
        TotalBytes: m.total_bytes,
        EncodingTime: m.encoding_time_ms,
        TypeCounts: m.type_counts,
    }
    damn stats
}

fr fr Helper utility functions
slay string_length(s tea) normie {
    count := 0
    for ch in s {
        count = count + 1
    }
    damn count
}

slay string_starts_with(s tea, prefix tea) lit {
    if string_length(prefix) > string_length(s) {
        damn cap
    }
    
    prefix_len := string_length(prefix)
    damn s[0:prefix_len] == prefix
}

slay string_find(s tea, substr tea) normie {
    s_len := string_length(s)
    substr_len := string_length(substr)
    
    if substr_len > s_len {
        damn -1
    }
    
    bestie i := 0; i <= s_len - substr_len; i++ {
        if s[i:i + substr_len] == substr {
            damn i
        }
    }
    damn -1
}

slay normie_to_string(n normie) tea {
    if n == 0 {
        damn "0"
    }
    
    if n < 0 {
        damn "-" + normie_to_string(-n)
    }
    
    digits := ""
    bestie n > 0 {
        digit := n % 10
        digits = digit_to_char(digit) + digits
        n = n / 10
    }
    damn digits
}

slay string_to_normie(s tea) normie {
    if s == "0" {
        damn 0
    }
    
    result := 0
    negative := cap
    start := 0
    
    if string_length(s) > 0 && s[0:1] == "-" {
        negative = based
        start = 1
    }
    
    bestie i := start; i < string_length(s); i++ {
        ch := s[i:i+1]
        digit := char_to_digit(ch)
        if digit < 0 || digit > 9 {
            damn 0  fr fr Invalid character
        }
        result = result * 10 + digit
    }
    
    if negative {
        result = -result
    }
    damn result
}

slay meal_to_string(f meal) tea {
    fr fr Simple float to string conversion
    int_part := normie(f)
    frac_part := f - meal(int_part)
    
    if frac_part == 0.0 {
        damn normie_to_string(int_part) + ".0"
    }
    
    fr fr Basic decimal representation
    frac_digits := normie(frac_part * 1000000.0) % 1000000
    damn normie_to_string(int_part) + "." + normie_to_string(frac_digits)
}

slay string_to_meal(s tea) meal {
    fr fr Simple string to float conversion
    dot_pos := string_find(s, ".")
    if dot_pos == -1 {
        damn meal(string_to_normie(s))
    }
    
    int_part := string_to_normie(s[0:dot_pos])
    frac_str := s[dot_pos+1:]
    frac_part := string_to_normie(frac_str)
    
    fr fr Convert fraction based on number of digits
    divisor := 1.0
    bestie i := 0; i < string_length(frac_str); i++ {
        divisor = divisor * 10.0
    }
    
    damn meal(int_part) + meal(frac_part) / divisor
}

slay digit_to_char(d normie) tea {
    if d == 0 { damn "0" }
    if d == 1 { damn "1" }
    if d == 2 { damn "2" }
    if d == 3 { damn "3" }
    if d == 4 { damn "4" }
    if d == 5 { damn "5" }
    if d == 6 { damn "6" }
    if d == 7 { damn "7" }
    if d == 8 { damn "8" }
    if d == 9 { damn "9" }
    damn "0"
}

slay char_to_digit(ch tea) normie {
    if ch == "0" { damn 0 }
    if ch == "1" { damn 1 }
    if ch == "2" { damn 2 }
    if ch == "3" { damn 3 }
    if ch == "4" { damn 4 }
    if ch == "5" { damn 5 }
    if ch == "6" { damn 6 }
    if ch == "7" { damn 7 }
    if ch == "8" { damn 8 }
    if ch == "9" { damn 9 }
    damn -1
}
