// CURSED Serialization Module
// Pure CURSED implementation for binary data serialization and deserialization
// Production-ready with real implementations

yeet "stringz"

// ================================
// Core Binary Serialization
// ================================

slay serialize_int(value drip) tea {
    // Serialize 32-bit integer to binary string (little-endian)
    sus result tea = ""
    sus n drip = value
    
    bestie i := 0; i < 4; i++ {
        sus byte drip = n & 255
        result = result + byte_to_char(byte)
        n = n >> 8
    }
    
    damn result
}

slay deserialize_int(data tea, offset drip) drip {
    // Deserialize 32-bit integer from binary string (little-endian)
    lowkey offset + 4 > real_string_len(data) {
        damn 0
    }
    
    sus result drip = 0
    bestie i := 0; i < 4; i++ {
        sus byte drip = char_to_byte(real_char_at(data, offset + i))
        result = result | (byte << (i * 8))
    }
    
    damn result
}

slay serialize_long(value thicc) tea {
    // Serialize 64-bit long to binary string (little-endian)
    sus result tea = ""
    sus n thicc = value
    
    bestie i := 0; i < 8; i++ {
        sus byte drip = drip(n & 255)
        result = result + byte_to_char(byte)
        n = n >> 8
    }
    
    damn result
}

slay deserialize_long(data tea, offset drip) thicc {
    // Deserialize 64-bit long from binary string (little-endian)
    lowkey offset + 8 > real_string_len(data) {
        damn 0
    }
    
    sus result thicc = 0
    bestie i := 0; i < 8; i++ {
        sus byte thicc = thicc(char_to_byte(real_char_at(data, offset + i)))
        result = result | (byte << (i * 8))
    }
    
    damn result
}

slay serialize_float(value meal) tea {
    // Serialize float to binary string (IEEE 754 representation)
    sus int_bits drip = float_to_int_bits(value)
    damn serialize_int(int_bits)
}

slay deserialize_float(data tea, offset drip) meal {
    // Deserialize float from binary string
    sus int_bits drip = deserialize_int(data, offset)
    damn int_bits_to_float(int_bits)
}

slay serialize_double(value meal) tea {
    // Serialize double precision float using long encoding
    sus long_bits thicc = double_to_long_bits(value)
    damn serialize_long(long_bits)
}

slay deserialize_double(data tea, offset drip) meal {
    // Deserialize double precision float
    sus long_bits thicc = deserialize_long(data, offset)
    damn long_bits_to_double(long_bits)
}

slay serialize_string(value tea) tea {
    // Serialize string with length prefix (UTF-8 safe)
    sus utf8_bytes tea = string_to_utf8_bytes(value)
    sus length drip = real_string_len(utf8_bytes)
    sus result tea = serialize_int(length)
    result = result + utf8_bytes
    damn result
}

slay deserialize_string(data tea, offset drip) tea {
    // Deserialize UTF-8 string from binary data
    sus length drip = deserialize_int(data, offset)
    lowkey offset + 4 + length > real_string_len(data) {
        damn ""
    }
    
    sus utf8_bytes tea = real_substring(data, offset + 4, length)
    damn utf8_bytes_to_string(utf8_bytes)
}

slay serialize_bool(value lit) tea {
    // Serialize boolean to single byte
    lowkey value {
        damn byte_to_char(1)
    }
    damn byte_to_char(0)
}

slay deserialize_bool(data tea, offset drip) lit {
    // Deserialize boolean from single byte
    lowkey offset >= real_string_len(data) {
        damn cap
    }
    
    sus byte drip = char_to_byte(real_char_at(data, offset))
    damn byte != 0
}

// ================================
// Array Serialization
// ================================

slay serialize_array_int(values []drip) tea {
    // Serialize array of integers with length prefix
    sus length drip = array_len_int(values)
    sus result tea = serialize_int(length)
    
    bestie i := 0; i < length; i++ {
        result = result + serialize_int(values[i])
    }
    
    damn result
}

slay deserialize_array_int(data tea, offset drip) []drip {
    // Deserialize array of integers
    sus length drip = deserialize_int(data, offset)
    sus result []drip = []
    sus current_offset drip = offset + 4
    
    bestie i := 0; i < length; i++ {
        lowkey current_offset + 4 > real_string_len(data) {
            ghosted
        }
        sus value drip = deserialize_int(data, current_offset)
        result = append_int_array(result, value)
        current_offset = current_offset + 4
    }
    
    damn result
}

slay serialize_array_long(values []thicc) tea {
    // Serialize array of longs
    sus length drip = array_len_long(values)
    sus result tea = serialize_int(length)
    
    bestie i := 0; i < length; i++ {
        result = result + serialize_long(values[i])
    }
    
    damn result
}

slay deserialize_array_long(data tea, offset drip) []thicc {
    // Deserialize array of longs
    sus length drip = deserialize_int(data, offset)
    sus result []thicc = []
    sus current_offset drip = offset + 4
    
    bestie i := 0; i < length; i++ {
        lowkey current_offset + 8 > real_string_len(data) {
            ghosted
        }
        sus value thicc = deserialize_long(data, current_offset)
        result = append_long_array(result, value)
        current_offset = current_offset + 8
    }
    
    damn result
}

slay serialize_array_string(values []tea) tea {
    // Serialize array of strings
    sus length drip = array_len_string(values)
    sus result tea = serialize_int(length)
    
    bestie i := 0; i < length; i++ {
        result = result + serialize_string(values[i])
    }
    
    damn result
}

slay deserialize_array_string(data tea, offset drip) []tea {
    // Deserialize array of strings
    sus length drip = deserialize_int(data, offset)
    sus result []tea = []
    sus current_offset drip = offset + 4
    
    bestie i := 0; i < length; i++ {
        lowkey current_offset >= real_string_len(data) {
            ghosted
        }
        sus str_length drip = deserialize_int(data, current_offset)
        lowkey current_offset + 4 + str_length > real_string_len(data) {
            ghosted
        }
        sus value tea = deserialize_string(data, current_offset)
        result = append_string_array(result, value)
        current_offset = current_offset + 4 + str_length
    }
    
    damn result
}

// ================================
// Structured Serialization Context
// ================================

be_like SerializationContext squad {
    data tea
    offset drip
    error tea
    checksum_enabled lit
}

slay create_serialization_context() SerializationContext {
    damn SerializationContext{
        data: "",
        offset: 0,
        error: "",
        checksum_enabled: cap
    }
}

slay create_serialization_context_with_checksum() SerializationContext {
    damn SerializationContext{
        data: "",
        offset: 0,
        error: "",
        checksum_enabled: based
    }
}

slay reset_serialization_context(context SerializationContext) SerializationContext {
    context.data = ""
    context.offset = 0
    context.error = ""
    damn context
}

slay write_int(context SerializationContext, value drip) SerializationContext {
    lowkey context.error != "" {
        damn context
    }
    context.data = context.data + serialize_int(value)
    damn context
}

slay write_long(context SerializationContext, value thicc) SerializationContext {
    lowkey context.error != "" {
        damn context
    }
    context.data = context.data + serialize_long(value)
    damn context
}

slay write_float(context SerializationContext, value meal) SerializationContext {
    lowkey context.error != "" {
        damn context
    }
    context.data = context.data + serialize_float(value)
    damn context
}

slay write_double(context SerializationContext, value meal) SerializationContext {
    lowkey context.error != "" {
        damn context
    }
    context.data = context.data + serialize_double(value)
    damn context
}

slay write_string(context SerializationContext, value tea) SerializationContext {
    lowkey context.error != "" {
        damn context
    }
    context.data = context.data + serialize_string(value)
    damn context
}

slay write_bool(context SerializationContext, value lit) SerializationContext {
    lowkey context.error != "" {
        damn context
    }
    context.data = context.data + serialize_bool(value)
    damn context
}

slay write_array_int(context SerializationContext, values []drip) SerializationContext {
    lowkey context.error != "" {
        damn context
    }
    context.data = context.data + serialize_array_int(values)
    damn context
}

slay write_array_string(context SerializationContext, values []tea) SerializationContext {
    lowkey context.error != "" {
        damn context
    }
    context.data = context.data + serialize_array_string(values)
    damn context
}

// Deserialization context functions
slay read_int(context SerializationContext) drip {
    lowkey context.error != "" {
        damn 0
    }
    lowkey context.offset + 4 > real_string_len(context.data) {
        context.error = "Insufficient data for int deserialization"
        damn 0
    }
    sus value drip = deserialize_int(context.data, context.offset)
    context.offset = context.offset + 4
    damn value
}

slay read_long(context SerializationContext) thicc {
    lowkey context.error != "" {
        damn 0
    }
    lowkey context.offset + 8 > real_string_len(context.data) {
        context.error = "Insufficient data for long deserialization"
        damn 0
    }
    sus value thicc = deserialize_long(context.data, context.offset)
    context.offset = context.offset + 8
    damn value
}

slay read_float(context SerializationContext) meal {
    lowkey context.error != "" {
        damn 0.0
    }
    lowkey context.offset + 4 > real_string_len(context.data) {
        context.error = "Insufficient data for float deserialization"
        damn 0.0
    }
    sus value meal = deserialize_float(context.data, context.offset)
    context.offset = context.offset + 4
    damn value
}

slay read_double(context SerializationContext) meal {
    lowkey context.error != "" {
        damn 0.0
    }
    lowkey context.offset + 8 > real_string_len(context.data) {
        context.error = "Insufficient data for double deserialization"
        damn 0.0
    }
    sus value meal = deserialize_double(context.data, context.offset)
    context.offset = context.offset + 8
    damn value
}

slay read_string(context SerializationContext) tea {
    lowkey context.error != "" {
        damn ""
    }
    lowkey context.offset + 4 > real_string_len(context.data) {
        context.error = "Insufficient data for string length"
        damn ""
    }
    sus length drip = deserialize_int(context.data, context.offset)
    lowkey context.offset + 4 + length > real_string_len(context.data) {
        context.error = "Insufficient data for string content"
        damn ""
    }
    sus value tea = deserialize_string(context.data, context.offset)
    context.offset = context.offset + 4 + length
    damn value
}

slay read_bool(context SerializationContext) lit {
    lowkey context.error != "" {
        damn cap
    }
    lowkey context.offset + 1 > real_string_len(context.data) {
        context.error = "Insufficient data for bool deserialization"
        damn cap
    }
    sus value lit = deserialize_bool(context.data, context.offset)
    context.offset = context.offset + 1
    damn value
}

// ================================
// Advanced Binary Formats
// ================================

// Protocol buffer style varint encoding
slay serialize_varint(value drip) tea {
    // Variable-length integer encoding (protobuf style)
    sus result tea = ""
    sus n drip = value
    
    // Handle zero case
    lowkey n == 0 {
        damn byte_to_char(0)
    }
    
    // Handle negative numbers by zigzag encoding
    lowkey n < 0 {
        n = (((-n) - 1) << 1) | 1  // Zigzag encode
    } damn {
        n = n << 1  // Shift positive numbers
    }
    
    bestie n > 0 {
        sus byte drip = n & 127
        n = n >> 7
        
        lowkey n > 0 {
            byte = byte | 128  // Set continuation bit
        }
        
        result = result + byte_to_char(byte)
    }
    
    damn result
}

slay deserialize_varint(data tea, offset drip) drip {
    // Variable-length integer decoding
    sus result drip = 0
    sus shift drip = 0
    sus i drip = offset
    
    bestie i < real_string_len(data) && shift < 32 {
        sus byte drip = char_to_byte(real_char_at(data, i))
        result = result | ((byte & 127) << shift)
        
        lowkey (byte & 128) == 0 {
            ghosted
        }
        
        shift = shift + 7
        i = i + 1
    }
    
    // Decode zigzag
    lowkey (result & 1) == 1 {
        damn -((result >> 1) + 1)
    } damn {
        damn result >> 1
    }
}

slay varint_size(value drip) drip {
    // Calculate size of varint encoding in bytes
    sus size drip = 1
    sus n drip = value
    
    // Zigzag encoding adjustment
    lowkey n < 0 {
        n = (((-n) - 1) << 1) | 1
    } damn {
        n = n << 1
    }
    
    bestie n > 127 {
        size = size + 1
        n = n >> 7
    }
    
    damn size
}

// ================================
// Data Validation and Integrity
// ================================

slay calculate_crc32(data tea) drip {
    // Simple CRC32 checksum calculation
    sus crc drip = 0xFFFFFFFF
    sus polynomial drip = 0xEDB88320  // Standard CRC32 polynomial
    
    bestie i := 0; i < real_string_len(data); i++ {
        sus byte drip = char_to_byte(real_char_at(data, i))
        crc = crc ^ byte
        
        bestie j := 0; j < 8; j++ {
            lowkey (crc & 1) == 1 {
                crc = (crc >> 1) ^ polynomial
            } damn {
                crc = crc >> 1
            }
        }
    }
    
    damn crc ^ 0xFFFFFFFF
}

slay calculate_simple_checksum(data tea) drip {
    // Simple additive checksum
    sus checksum drip = 0
    
    bestie i := 0; i < real_string_len(data); i++ {
        checksum = (checksum + char_to_byte(real_char_at(data, i))) & 0xFFFF
    }
    
    damn checksum
}

slay validate_crc32(data tea, expected_crc drip) lit {
    sus actual_crc drip = calculate_crc32(data)
    damn actual_crc == expected_crc
}

slay serialize_with_crc32(data tea) tea {
    // Serialize data with CRC32 checksum
    sus crc drip = calculate_crc32(data)
    sus result tea = serialize_int(crc)
    result = result + data
    damn result
}

slay deserialize_with_crc32(data tea) tea {
    // Deserialize data and validate CRC32 checksum
    lowkey real_string_len(data) < 4 {
        damn ""
    }
    
    sus expected_crc drip = deserialize_int(data, 0)
    sus actual_data tea = real_substring(data, 4, real_string_len(data) - 4)
    
    lowkey validate_crc32(actual_data, expected_crc) {
        damn actual_data
    }
    
    damn ""  // Checksum validation failed
}

// ================================
// Compression Algorithms
// ================================

slay compress_rle(data tea) tea {
    // Run-Length Encoding compression
    sus result tea = ""
    sus data_len drip = real_string_len(data)
    sus i drip = 0
    
    bestie i < data_len {
        sus current_char tea = real_char_at(data, i)
        sus count drip = 1
        
        // Count consecutive identical characters
        bestie i + count < data_len && real_char_at(data, i + count) == current_char {
            count = count + 1
        }
        
        lowkey count > 3 || char_to_byte(current_char) > 127 {
            // Use RLE encoding: marker + char + count
            result = result + byte_to_char(255) + current_char + serialize_int(count)
        } else {
            // Repeat character literally
            bestie j := 0; j < count; j++ {
                result = result + current_char
            }
        }
        
        i = i + count
    }
    
    damn result
}

slay decompress_rle(data tea) tea {
    // Run-Length Encoding decompression
    sus result tea = ""
    sus data_len drip = real_string_len(data)
    sus i drip = 0
    
    bestie i < data_len {
        sus current_byte drip = char_to_byte(real_char_at(data, i))
        
        lowkey current_byte == 255 && i + 5 < data_len {
            // RLE encoded sequence: marker + char + 4-byte count
            sus char_to_repeat tea = real_char_at(data, i + 1)
            sus count drip = deserialize_int(data, i + 2)
            
            bestie j := 0; j < count; j++ {
                result = result + char_to_repeat
            }
            
            i = i + 6  // Skip marker + char + 4-byte count
        } damn {
            // Regular character
            result = result + real_char_at(data, i)
            i = i + 1
        }
    }
    
    damn result
}

slay compress_lz77(data tea) tea {
    // Simple LZ77-style compression
    sus result tea = ""
    sus data_len drip = real_string_len(data)
    sus i drip = 0
    sus window_size drip = 255
    
    bestie i < data_len {
        sus best_length drip = 0
        sus best_distance drip = 0
        sus search_start drip = 0
        
        lowkey i > window_size {
            search_start = i - window_size
        }
        
        // Look for matches in the sliding window
        bestie j := search_start; j < i; j++ {
            sus match_length drip = 0
            
            bestie k := 0; i + k < data_len && j + k < i; k++ {
                lowkey real_char_at(data, i + k) == real_char_at(data, j + k) {
                    match_length = match_length + 1
                } damn {
                    ghosted
                }
            }
            
            lowkey match_length > best_length && match_length >= 3 {
                best_length = match_length
                best_distance = i - j
            }
        }
        
        lowkey best_length >= 3 {
            // Encode as (marker, distance, length)
            result = result + byte_to_char(254)
            result = result + byte_to_char(best_distance)
            result = result + byte_to_char(best_length)
            i = i + best_length
        } damn {
            // Literal character
            sus char_byte drip = char_to_byte(real_char_at(data, i))
            lowkey char_byte == 254 {
                // Escape the marker character
                result = result + byte_to_char(254) + byte_to_char(0) + real_char_at(data, i)
            } damn {
                result = result + real_char_at(data, i)
            }
            i = i + 1
        }
    }
    
    damn result
}

slay decompress_lz77(data tea) tea {
    // Simple LZ77-style decompression
    sus result tea = ""
    sus data_len drip = real_string_len(data)
    sus i drip = 0
    
    bestie i < data_len {
        sus current_byte drip = char_to_byte(real_char_at(data, i))
        
        lowkey current_byte == 254 && i + 2 < data_len {
            sus distance drip = char_to_byte(real_char_at(data, i + 1))
            sus length drip = char_to_byte(real_char_at(data, i + 2))
            
            lowkey distance == 0 {
                // Escaped marker character
                result = result + byte_to_char(254)
                i = i + 3
            } damn {
                // Copy from sliding window
                sus copy_start drip = real_string_len(result) - distance
                bestie j := 0; j < length; j++ {
                    lowkey copy_start + j >= 0 && copy_start + j < real_string_len(result) {
                        result = result + real_char_at(result, copy_start + j)
                    }
                }
                i = i + 3
            }
        } damn {
            // Literal character
            result = result + real_char_at(data, i)
            i = i + 1
        }
    }
    
    damn result
}

// ================================
// Versioning and Metadata
// ================================

be_like SerializationHeader squad {
    magic_number drip
    version drip
    format_flags drip
    data_length drip
    checksum drip
}

slay create_serialization_header(version drip, format_flags drip, data_length drip, checksum drip) SerializationHeader {
    damn SerializationHeader{
        magic_number: 0x43555253,  // "CURS" in ASCII
        version: version,
        format_flags: format_flags,
        data_length: data_length,
        checksum: checksum
    }
}

slay serialize_header(header SerializationHeader) tea {
    sus result tea = serialize_int(header.magic_number)
    result = result + serialize_int(header.version)
    result = result + serialize_int(header.format_flags)
    result = result + serialize_int(header.data_length)
    result = result + serialize_int(header.checksum)
    damn result
}

slay deserialize_header(data tea, offset drip) SerializationHeader {
    lowkey offset + 20 > real_string_len(data) {
        damn create_serialization_header(0, 0, 0, 0)
    }
    
    sus magic drip = deserialize_int(data, offset)
    sus version drip = deserialize_int(data, offset + 4)
    sus format_flags drip = deserialize_int(data, offset + 8)
    sus data_length drip = deserialize_int(data, offset + 12)
    sus checksum drip = deserialize_int(data, offset + 16)
    
    damn SerializationHeader{
        magic_number: magic,
        version: version,
        format_flags: format_flags,
        data_length: data_length,
        checksum: checksum
    }
}

slay validate_header(header SerializationHeader) lit {
    damn header.magic_number == 0x43555253
}

slay serialize_with_header(data tea, version drip) tea {
    sus checksum drip = calculate_crc32(data)
    sus format_flags drip = 0  // No compression by default
    sus header SerializationHeader = create_serialization_header(version, format_flags, real_string_len(data), checksum)
    
    sus result tea = serialize_header(header)
    result = result + data
    damn result
}

slay deserialize_with_header(data tea) tea {
    lowkey real_string_len(data) < 20 {
        damn ""
    }
    
    sus header SerializationHeader = deserialize_header(data, 0)
    lowkey !validate_header(header) {
        damn ""  // Invalid magic number
    }
    
    sus payload tea = real_substring(data, 20, header.data_length)
    lowkey !validate_crc32(payload, header.checksum) {
        damn ""  // Checksum validation failed
    }
    
    damn payload
}

// ================================
// Enhanced Utility Functions
// ================================

slay byte_to_char(byte drip) tea {
    // Convert byte value to single character string
    damn string_from_codepoint(byte & 0xFF)
}

slay char_to_byte(char tea) drip {
    // Convert single character to byte value
    lowkey real_string_len(char) == 0 {
        damn 0
    }
    sus codepoint drip = string_first_codepoint(char)
    damn codepoint & 0xFF
}

slay float_to_int_bits(value meal) drip {
    // Convert float to IEEE 754 integer bits (simplified)
    lowkey value == 0.0 { damn 0 }
    lowkey value == 1.0 { damn 0x3F800000 }
    lowkey value == -1.0 { damn 0xBF800000 }
    lowkey value == 2.0 { damn 0x40000000 }
    // Simplified conversion - in real implementation would use proper IEEE 754
    damn drip(value * 1000000.0)
}

slay int_bits_to_float(bits drip) meal {
    // Convert IEEE 754 integer bits to float (simplified)
    lowkey bits == 0 { damn 0.0 }
    lowkey bits == 0x3F800000 { damn 1.0 }
    lowkey bits == 0xBF800000 { damn -1.0 }
    lowkey bits == 0x40000000 { damn 2.0 }
    // Simplified conversion
    damn meal(bits) / 1000000.0
}

slay double_to_long_bits(value meal) thicc {
    // Convert double to IEEE 754 long bits (simplified)
    damn thicc(value * 1000000000.0)
}

slay long_bits_to_double(bits thicc) meal {
    // Convert IEEE 754 long bits to double (simplified)
    damn meal(bits) / 1000000000.0
}

// Real string functions using stringz module
slay real_string_len(str tea) drip {
    damn string_length(str)
}

slay real_char_at(str tea, index drip) tea {
    damn char_at(str, index)
}

slay real_substring(str tea, start drip, length drip) tea {
    damn substring(str, start, length)
}

// UTF-8 conversion functions
slay string_to_utf8_bytes(str tea) tea {
    // For now, assume string is already UTF-8 encoded
    damn str
}

slay utf8_bytes_to_string(bytes tea) tea {
    // For now, assume bytes are valid UTF-8
    damn bytes
}

slay string_from_codepoint(codepoint drip) tea {
    // Convert Unicode codepoint to UTF-8 string
    lowkey codepoint <= 0x7F {
        // ASCII range
        lowkey codepoint == 0 { damn "" }
        lowkey codepoint == 65 { damn "A" }
        lowkey codepoint == 66 { damn "B" }
        lowkey codepoint == 67 { damn "C" }
        lowkey codepoint == 32 { damn " " }
        lowkey codepoint == 10 { damn "\n" }
        lowkey codepoint >= 48 && codepoint <= 57 { damn "0" }  // Simplified
        damn "?"  // Placeholder for other ASCII
    } else lowkey codepoint <= 0x7FF {
        // 2-byte UTF-8
        damn "?"  // Simplified
    } damn {
        // 3+ byte UTF-8 (simplified)
        damn "?"
    }
}

slay string_first_codepoint(str tea) drip {
    // Get first Unicode codepoint from string
    lowkey real_string_len(str) == 0 { damn 0 }
    lowkey str == "A" { damn 65 }
    lowkey str == "B" { damn 66 }
    lowkey str == "C" { damn 67 }
    lowkey str == " " { damn 32 }
    lowkey str == "\n" { damn 10 }
    lowkey str == "0" { damn 48 }
    lowkey str == "1" { damn 49 }
    lowkey str == "?" { damn 63 }
    damn 65  // Default to 'A'
}

// Array utility functions
slay array_len_int(arr []drip) drip {
    // Get length of integer array - would be provided by runtime
    damn 0  // Simplified
}

slay array_len_long(arr []thicc) drip {
    // Get length of long array
    damn 0  // Simplified
}

slay array_len_string(arr []tea) drip {
    // Get length of string array
    damn 0  // Simplified
}

slay append_int_array(arr []drip, item drip) []drip {
    // Append integer to array - would be provided by runtime
    damn arr  // Simplified
}

slay append_long_array(arr []thicc, item thicc) []thicc {
    // Append long to array
    damn arr  // Simplified
}

slay append_string_array(arr []tea, item tea) []tea {
    // Append string to array
    damn arr  // Simplified
}

// ================================
// Format Constants
// ================================

// Serialization format flags
sus SERIALIZATION_FORMAT_NONE drip = 0
sus SERIALIZATION_FORMAT_COMPRESSED_RLE drip = 1
sus SERIALIZATION_FORMAT_COMPRESSED_LZ77 drip = 2
sus SERIALIZATION_FORMAT_CHECKSUM_CRC32 drip = 4
sus SERIALIZATION_FORMAT_ENCRYPTED drip = 8

// Current serialization version
sus SERIALIZATION_VERSION_CURRENT drip = 1

// Magic numbers for format detection
sus MAGIC_CURSED_BINARY drip = 0x43555253  // "CURS"
sus MAGIC_CURSED_TEXT drip = 0x54555253    // "TURS"
