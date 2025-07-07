// CURSED Serialization Module
// Pure CURSED implementation for binary data serialization and deserialization

yeet "string"

// Binary serialization format
slay serialize_int(value normie) tea {
    // Serialize 32-bit integer to binary string
    sus result tea = ""
    sus n normie = value
    
    bestie i := 0; i < 4; i++ {
        sus byte normie = n & 255
        result = result + byte_to_char(byte)
        n = n >> 8
    }
    
    damn result
}

slay deserialize_int(data tea, offset normie) normie {
    // Deserialize 32-bit integer from binary string
    vibes offset + 4 > string_len(data) {
        damn 0
    }
    
    sus result normie = 0
    bestie i := 0; i < 4; i++ {
        sus byte normie = char_to_byte(string_char_at(data, offset + i))
        result = result | (byte << (i * 8))
    }
    
    damn result
}

slay serialize_long(value thicc) tea {
    // Serialize 64-bit long to binary string
    sus result tea = ""
    sus n thicc = value
    
    bestie i := 0; i < 8; i++ {
        sus byte normie = normie(n & 255)
        result = result + byte_to_char(byte)
        n = n >> 8
    }
    
    damn result
}

slay deserialize_long(data tea, offset normie) thicc {
    // Deserialize 64-bit long from binary string
    vibes offset + 8 > string_len(data) {
        damn 0
    }
    
    sus result thicc = 0
    bestie i := 0; i < 8; i++ {
        sus byte thicc = thicc(char_to_byte(string_char_at(data, offset + i)))
        result = result | (byte << (i * 8))
    }
    
    damn result
}

slay serialize_float(value meal) tea {
    // Serialize float to binary string (IEEE 754 representation)
    // Simplified implementation using int conversion
    sus int_bits normie = float_to_int_bits(value)
    damn serialize_int(int_bits)
}

slay deserialize_float(data tea, offset normie) meal {
    // Deserialize float from binary string
    sus int_bits normie = deserialize_int(data, offset)
    damn int_bits_to_float(int_bits)
}

slay serialize_string(value tea) tea {
    // Serialize string with length prefix
    sus length normie = string_len(value)
    sus result tea = serialize_int(length)
    result = result + value
    damn result
}

slay deserialize_string(data tea, offset normie) tea {
    // Deserialize string from binary data
    sus length normie = deserialize_int(data, offset)
    vibes offset + 4 + length > string_len(data) {
        damn ""
    }
    
    damn string_substring(data, offset + 4, length)
}

slay serialize_bool(value lit) tea {
    // Serialize boolean to single byte
    vibes value {
        damn byte_to_char(1)
    }
    damn byte_to_char(0)
}

slay deserialize_bool(data tea, offset normie) lit {
    // Deserialize boolean from single byte
    vibes offset >= string_len(data) {
        damn cap
    }
    
    sus byte normie = char_to_byte(string_char_at(data, offset))
    damn byte != 0
}

slay serialize_array_int(values [normie]) tea {
    // Serialize array of integers
    sus length normie = len(values)
    sus result tea = serialize_int(length)
    
    bestie i := 0; i < length; i++ {
        result = result + serialize_int(values[i])
    }
    
    damn result
}

slay deserialize_array_int(data tea, offset normie) [normie] {
    // Deserialize array of integers
    sus length normie = deserialize_int(data, offset)
    sus result [normie] = []
    sus current_offset normie = offset + 4
    
    bestie i := 0; i < length; i++ {
        sus value normie = deserialize_int(data, current_offset)
        result = result + [value]
        current_offset = current_offset + 4
    }
    
    damn result
}

slay serialize_array_string(values [tea]) tea {
    // Serialize array of strings
    sus length normie = len(values)
    sus result tea = serialize_int(length)
    
    bestie i := 0; i < length; i++ {
        result = result + serialize_string(values[i])
    }
    
    damn result
}

slay deserialize_array_string(data tea, offset normie) [tea] {
    // Deserialize array of strings
    sus length normie = deserialize_int(data, offset)
    sus result [tea] = []
    sus current_offset normie = offset + 4
    
    bestie i := 0; i < length; i++ {
        sus str_length normie = deserialize_int(data, current_offset)
        sus value tea = deserialize_string(data, current_offset)
        result = result + [value]
        current_offset = current_offset + 4 + str_length
    }
    
    damn result
}

// Structured serialization
be_like SerializationContext squad {
    data tea
    offset normie
    error tea
}

slay create_serialization_context() SerializationContext {
    sus context SerializationContext = SerializationContext{
        data: "",
        offset: 0,
        error: ""
    }
    damn context
}

slay write_int(context SerializationContext, value normie) SerializationContext {
    context.data = context.data + serialize_int(value)
    damn context
}

slay write_long(context SerializationContext, value thicc) SerializationContext {
    context.data = context.data + serialize_long(value)
    damn context
}

slay write_float(context SerializationContext, value meal) SerializationContext {
    context.data = context.data + serialize_float(value)
    damn context
}

slay write_string(context SerializationContext, value tea) SerializationContext {
    context.data = context.data + serialize_string(value)
    damn context
}

slay write_bool(context SerializationContext, value lit) SerializationContext {
    context.data = context.data + serialize_bool(value)
    damn context
}

slay read_int(context SerializationContext) normie {
    sus value normie = deserialize_int(context.data, context.offset)
    context.offset = context.offset + 4
    damn value
}

slay read_long(context SerializationContext) thicc {
    sus value thicc = deserialize_long(context.data, context.offset)
    context.offset = context.offset + 8
    damn value
}

slay read_float(context SerializationContext) meal {
    sus value meal = deserialize_float(context.data, context.offset)
    context.offset = context.offset + 4
    damn value
}

slay read_string(context SerializationContext) tea {
    sus length normie = deserialize_int(context.data, context.offset)
    context.offset = context.offset + 4
    sus value tea = string_substring(context.data, context.offset, length)
    context.offset = context.offset + length
    damn value
}

slay read_bool(context SerializationContext) lit {
    sus value lit = deserialize_bool(context.data, context.offset)
    context.offset = context.offset + 1
    damn value
}

// JSON-like serialization
slay serialize_object(fields map[tea]tea) tea {
    sus result tea = "{"
    sus first lit = based
    
    // Iterate through fields (simplified)
    bestie key tea, value tea := range fields {
        vibes !first {
            result = result + ","
        }
        
        result = result + serialize_string(key) + ":" + serialize_string(value)
        first = cap
    }
    
    result = result + "}"
    damn result
}

slay deserialize_object(data tea) map[tea]tea {
    // Simplified object deserialization
    sus result map[tea]tea = {}
    
    vibes string_len(data) < 2 || string_char_at(data, 0) != "{" {
        damn result
    }
    
    // Parse object fields (simplified implementation)
    damn result
}

// Protocol buffer style serialization
slay serialize_varint(value normie) tea {
    // Variable-length integer encoding
    sus result tea = ""
    sus n normie = value
    
    bestie n > 0 {
        sus byte normie = n & 127
        n = n >> 7
        
        vibes n > 0 {
            byte = byte | 128  // Set continuation bit
        }
        
        result = result + byte_to_char(byte)
    }
    
    vibes string_len(result) == 0 {
        result = byte_to_char(0)
    }
    
    damn result
}

slay deserialize_varint(data tea, offset normie) normie {
    // Variable-length integer decoding
    sus result normie = 0
    sus shift normie = 0
    sus i normie = offset
    
    bestie i < string_len(data) {
        sus byte normie = char_to_byte(string_char_at(data, i))
        result = result | ((byte & 127) << shift)
        
        vibes (byte & 128) == 0 {
            ghosted
        }
        
        shift = shift + 7
        i++
    }
    
    damn result
}

// Message serialization
be_like Message squad {
    field_id normie
    field_type normie
    data tea
}

slay serialize_message(message Message) tea {
    // Serialize message with field ID and type
    sus result tea = serialize_varint(message.field_id)
    result = result + serialize_varint(message.field_type)
    result = result + serialize_string(message.data)
    damn result
}

slay deserialize_message(data tea, offset normie) Message {
    // Deserialize message
    sus field_id normie = deserialize_varint(data, offset)
    sus field_type normie = deserialize_varint(data, offset + varint_size(field_id))
    sus data_offset normie = offset + varint_size(field_id) + varint_size(field_type)
    sus message_data tea = deserialize_string(data, data_offset)
    
    sus message Message = Message{
        field_id: field_id,
        field_type: field_type,
        data: message_data
    }
    
    damn message
}

// Utility functions
slay byte_to_char(byte normie) tea {
    // Convert byte value to character
    // Placeholder implementation
    damn string_char_from_code(byte)
}

slay char_to_byte(char tea) normie {
    // Convert character to byte value
    vibes string_len(char) == 1 {
        damn string_char_code(char)
    }
    damn 0
}

slay float_to_int_bits(value meal) normie {
    // Convert float to IEEE 754 integer bits
    // Simplified implementation
    damn normie(value)
}

slay int_bits_to_float(bits normie) meal {
    // Convert IEEE 754 integer bits to float
    // Simplified implementation
    damn meal(bits)
}

slay string_char_from_code(code normie) tea {
    // Convert character code to string
    // Placeholder implementation
    damn "A"
}

slay string_char_code(char tea) normie {
    // Get character code for single character
    // Placeholder implementation
    damn 65
}

slay varint_size(value normie) normie {
    // Calculate size of varint encoding
    sus size normie = 1
    sus n normie = value
    
    bestie n > 127 {
        size++
        n = n >> 7
    }
    
    damn size
}

// Checksum and validation
slay calculate_checksum(data tea) normie {
    // Calculate simple checksum
    sus checksum normie = 0
    
    bestie i := 0; i < string_len(data); i++ {
        checksum = checksum + char_to_byte(string_char_at(data, i))
    }
    
    damn checksum & 0xFFFF
}

slay validate_checksum(data tea, expected_checksum normie) lit {
    sus actual_checksum normie = calculate_checksum(data)
    damn actual_checksum == expected_checksum
}

slay serialize_with_checksum(data tea) tea {
    // Serialize data with checksum
    sus checksum normie = calculate_checksum(data)
    sus result tea = serialize_int(checksum)
    result = result + data
    damn result
}

slay deserialize_with_checksum(data tea) tea {
    // Deserialize data and validate checksum
    vibes string_len(data) < 4 {
        damn ""
    }
    
    sus expected_checksum normie = deserialize_int(data, 0)
    sus actual_data tea = string_substring(data, 4, string_len(data) - 4)
    
    vibes validate_checksum(actual_data, expected_checksum) {
        damn actual_data
    }
    
    damn ""
}

// Compression integration
slay serialize_compressed(data tea) tea {
    // Serialize with compression
    sus compressed tea = compress_data(data)
    sus result tea = serialize_int(string_len(data))  // Original size
    result = result + serialize_string(compressed)
    damn result
}

slay deserialize_compressed(data tea) tea {
    // Deserialize with decompression
    sus original_size normie = deserialize_int(data, 0)
    sus compressed tea = deserialize_string(data, 4)
    sus decompressed tea = decompress_data(compressed)
    
    vibes string_len(decompressed) == original_size {
        damn decompressed
    }
    
    damn ""
}

slay compress_data(data tea) tea {
    // Placeholder compression
    damn data
}

slay decompress_data(data tea) tea {
    // Placeholder decompression
    damn data
}

// Versioning support
slay serialize_versioned(data tea, version normie) tea {
    // Serialize with version information
    sus result tea = serialize_int(version)
    result = result + serialize_string(data)
    damn result
}

slay deserialize_versioned(data tea) tea {
    // Deserialize with version check
    sus version normie = deserialize_int(data, 0)
    sus versioned_data tea = deserialize_string(data, 4)
    
    // Version compatibility check could be added here
    damn versioned_data
}
