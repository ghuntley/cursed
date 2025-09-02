fr fr ===== ENCODINGZ EXAMPLES - Real-World Usage Patterns =====
fr fr Demonstrates practical applications of encoding utilities
fr fr Covers web APIs, data serialization, file processing, and security use cases

yeet "encodingz"
yeet "vibez"
yeet "stringz"
yeet "networz"
yeet "filez"
yeet "jsonz"

fr fr ===== WEB API EXAMPLES =====

slay example_jwt_token_encoding() {
    vibez.spill("🔐 JWT Token Encoding Example")
    vibez.spill("=" * 40)
    
    fr fr JWT Header
    sus header_json tea = '{"alg":"HS256","typ":"JWT"}'
    sus header_b64 tea = base64_encode_url_safe(header_json)
    vibez.spill("Header: " + header_b64)
    
    fr fr JWT Payload
    sus payload_json tea = '{"sub":"1234567890","name":"John Doe","iat":1516239022}'
    sus payload_b64 tea = base64_encode_url_safe(payload_json)
    vibez.spill("Payload: " + payload_b64)
    
    fr fr Simulate signature (in real app, use cryptz module)
    sus signature_data tea = header_b64 + "." + payload_b64
    sus signature tea = "simulated_signature_hash"
    sus signature_b64 tea = base64_encode_url_safe(signature)
    
    fr fr Complete JWT
    sus jwt_token tea = header_b64 + "." + payload_b64 + "." + signature_b64
    vibez.spill("Complete JWT: " + jwt_token)
    
    fr fr Decode JWT components
    sus decoded_header tea = base64_decode_url_safe(header_b64) fam {
        when err -> {
            vibez.spill("❌ Header decode error: " + err)
            damn
        }
    }
    
    sus decoded_payload tea = base64_decode_url_safe(payload_b64) fam {
        when err -> {
            vibez.spill("❌ Payload decode error: " + err)
            damn
        }
    }
    
    vibez.spill("Decoded Header: " + decoded_header)
    vibez.spill("Decoded Payload: " + decoded_payload)
    vibez.spill("✅ JWT encoding/decoding successful\n")
}

slay example_rest_api_response() {
    vibez.spill("🌐 REST API Response Encoding Example")
    vibez.spill("=" * 45)
    
    fr fr Simulate binary data in API response (like images, files)
    sus binary_data tea = "Binary file content: PNG\x89\x50\x4E\x47\x0D\x0A\x1A\x0A"
    sus encoded_data tea = base64_encode(binary_data)
    
    fr fr Create JSON response with embedded binary data
    sus api_response tea = '{' +
        '"status":"success",' +
        '"data":"' + encoded_data + '",' +
        '"content_type":"image/png",' +
        '"size":' + int_to_string(string_length(binary_data)) +
        '}'
    
    vibez.spill("API Response: " + api_response)
    
    fr fr Client-side: decode the binary data
    fr fr Extract base64 data from JSON (simplified parsing)
    sus data_start drip = string_index(api_response, '"data":"') + 8
    sus data_end drip = string_index_from(api_response, '"', data_start)
    sus extracted_b64 tea = substring(api_response, data_start, data_end)
    
    sus decoded_binary tea = base64_decode(extracted_b64) fam {
        when err -> {
            vibez.spill("❌ Binary data decode error: " + err)
            damn
        }
    }
    
    vibez.spill("Decoded binary data: " + decoded_binary)
    vibez.spill("✅ REST API binary data transfer successful\n")
}

slay example_url_parameter_encoding() {
    vibez.spill("🔗 URL Parameter Encoding Example")
    vibez.spill("=" * 40)
    
    fr fr User search query with special characters
    sus search_query tea = "Hello World! How are you? @user #hashtag"
    sus encoded_query tea = url_encode(search_query)
    
    fr fr Build complete URL
    sus base_url tea = "https://api.example.com/search"
    sus full_url tea = base_url + "?q=" + encoded_query + "&limit=10&sort=date"
    vibez.spill("Original query: " + search_query)
    vibez.spill("Encoded query: " + encoded_query)
    vibez.spill("Full URL: " + full_url)
    
    fr fr Server-side: decode the parameters
    sus query_start drip = string_index(full_url, "q=") + 2
    sus query_end drip = string_index_from(full_url, "&", query_start)
    sus url_encoded_param tea = substring(full_url, query_start, query_end)
    
    sus decoded_query tea = url_decode(url_encoded_param) fam {
        when err -> {
            vibez.spill("❌ URL decode error: " + err)
            damn
        }
    }
    
    vibez.spill("Decoded on server: " + decoded_query)
    vibez.spill("✅ URL parameter encoding successful\n")
}

fr fr ===== FILE PROCESSING EXAMPLES =====

slay example_config_file_processing() {
    vibez.spill("📁 Configuration File Processing Example")
    vibez.spill("=" * 48)
    
    fr fr Simulate reading binary config data
    sus config_binary tea = "Config data with special chars: \x00\x01\x02\xFF"
    
    fr fr Encode for storage in text-based config file
    sus config_hex tea = hex_encode(config_binary)
    sus config_b64 tea = base64_encode(config_binary)
    
    vibez.spill("Original config: (binary data)")
    vibez.spill("Hex encoded: " + config_hex)
    vibez.spill("Base64 encoded: " + config_b64)
    
    fr fr Create config file content
    sus config_file_content tea = "[binary_config]\n" +
        "hex_data = " + config_hex + "\n" +
        "base64_data = " + config_b64 + "\n" +
        "encoding = hex,base64\n"
    
    vibez.spill("Config file content:")
    vibez.spill(config_file_content)
    
    fr fr Decode configuration
    sus decoded_hex tea = hex_decode(config_hex) fam {
        when err -> {
            vibez.spill("❌ Config hex decode error: " + err)
            damn ""
        }
    }
    
    sus decoded_b64 tea = base64_decode(config_b64) fam {
        when err -> {
            vibez.spill("❌ Config base64 decode error: " + err)
            damn ""
        }
    }
    
    vibez.spill("Decoded from hex matches original: " + bool_to_string(decoded_hex == config_binary))
    vibez.spill("Decoded from base64 matches original: " + bool_to_string(decoded_b64 == config_binary))
    vibez.spill("✅ Config file processing successful\n")
}

slay example_streaming_file_encoding() {
    vibez.spill("🌊 Streaming File Encoding Example")
    vibez.spill("=" * 42)
    
    fr fr Simulate processing a large file in chunks
    sus file_chunks tea[value] = [
        "First chunk of large file data...",
        "Second chunk continues the data stream...",
        "Third chunk with more binary content...",
        "Fourth chunk near the end...",
        "Final chunk completes the file."
    ]
    
    fr fr Create streaming encoder
    sus encoder StreamEncoder = create_stream_encoder("base64")
    sus encoded_result tea = ""
    
    vibez.spill("Processing file in " + int_to_string(array_length(file_chunks)) + " chunks...")
    
    sus i drip = 0
    bestie i < array_length(file_chunks) {
        sus chunk tea = file_chunks[i]
        sus chunk_encoded tea = stream_encode_chunk(encoder, chunk)
        encoded_result = encoded_result + chunk_encoded
        
        vibez.spill("Chunk " + int_to_string(i + 1) + ": " + int_to_string(string_length(chunk)) + " bytes -> " + 
                   int_to_string(string_length(chunk_encoded)) + " encoded bytes")
        i = i + 1
    }
    
    fr fr Finalize the stream
    sus final_chunk tea = stream_finalize(encoder)
    encoded_result = encoded_result + final_chunk
    
    vibez.spill("Final chunk: " + int_to_string(string_length(final_chunk)) + " bytes")
    vibez.spill("Total encoded length: " + int_to_string(string_length(encoded_result)) + " bytes")
    
    fr fr Verify by decoding the complete result
    sus complete_original tea = ""
    sus j drip = 0
    bestie j < array_length(file_chunks) {
        complete_original = complete_original + file_chunks[j]
        j = j + 1
    }
    
    sus decoded_complete tea = base64_decode(encoded_result) fam {
        when err -> {
            vibez.spill("❌ Streaming decode error: " + err)
            damn ""
        }
    }
    
    vibez.spill("Streaming encoding preserves data: " + bool_to_string(decoded_complete == complete_original))
    vibez.spill("✅ Streaming file encoding successful\n")
}

fr fr ===== SECURITY EXAMPLES =====

slay example_password_storage() {
    vibez.spill("🔒 Secure Password Storage Example")
    vibez.spill("=" * 40)
    
    fr fr Simulate password hashing and storage
    sus user_password tea = "user_secure_password_123"
    sus password_hash tea = "simulated_hash_output_binary_data"  fr fr In real app, use cryptz
    sus salt tea = "random_salt_value_16_bytes"
    
    fr fr Encode hash and salt for database storage
    sus hash_b64 tea = base64_encode(password_hash)
    sus salt_b64 tea = base64_encode(salt)
    
    vibez.spill("Original password: " + user_password)
    vibez.spill("Password hash (base64): " + hash_b64)
    vibez.spill("Salt (base64): " + salt_b64)
    
    fr fr Create database record
    sus db_record tea = '{' +
        '"user_id":"12345",' +
        '"password_hash":"' + hash_b64 + '",' +
        '"salt":"' + salt_b64 + '",' +
        '"algorithm":"pbkdf2",' +
        '"iterations":100000' +
        '}'
    
    vibez.spill("Database record: " + db_record)
    
    fr fr Verification process
    sus stored_hash tea = base64_decode(hash_b64) fam {
        when err -> {
            vibez.spill("❌ Hash decode error: " + err)
            damn ""
        }
    }
    
    sus stored_salt tea = base64_decode(salt_b64) fam {
        when err -> {
            vibez.spill("❌ Salt decode error: " + err)
            damn ""
        }
    }
    
    vibez.spill("Hash verification: " + bool_to_string(stored_hash == password_hash))
    vibez.spill("Salt verification: " + bool_to_string(stored_salt == salt))
    vibez.spill("✅ Password storage encoding successful\n")
}

slay example_api_key_encoding() {
    vibez.spill("🔑 API Key Encoding Example")
    vibez.spill("=" * 35)
    
    fr fr Generate API key with metadata
    sus api_key_raw tea = "user123:project456:permissions789:timestamp1234567890"
    sus api_key_b64 tea = base64_encode_url_safe(api_key_raw)
    
    fr fr Add prefix for identification
    sus api_key_formatted tea = "ck_" + api_key_b64  fr fr 'ck' = cursed key
    
    vibez.spill("Raw API key: " + api_key_raw)
    vibez.spill("Encoded API key: " + api_key_formatted)
    
    fr fr Client uses the API key
    sus auth_header tea = "Authorization: Bearer " + api_key_formatted
    vibez.spill("Auth header: " + auth_header)
    
    fr fr Server validates the API key
    ready string_starts_with(api_key_formatted, "ck_") {
        sus key_data tea = substring(api_key_formatted, 3, string_length(api_key_formatted))
        sus decoded_key tea = base64_decode_url_safe(key_data) fam {
            when err -> {
                vibez.spill("❌ Invalid API key format: " + err)
                damn
            }
        }
        
        vibez.spill("Decoded API key: " + decoded_key)
        
        fr fr Parse key components (simplified)
        sus key_parts tea[value] = split(decoded_key, ":")
        ready array_length(key_parts) == 4 {
            vibez.spill("User ID: " + key_parts[0])
            vibez.spill("Project ID: " + key_parts[1])
            vibez.spill("Permissions: " + key_parts[2])
            vibez.spill("Timestamp: " + key_parts[3])
            vibez.spill("✅ API key validation successful")
        } otherwise {
            vibez.spill("❌ Invalid API key format")
        }
    } otherwise {
        vibez.spill("❌ Invalid API key prefix")
    }
    
    vibez.spill("")
}

fr fr ===== DATA SERIALIZATION EXAMPLES =====

slay example_binary_protocol() {
    vibez.spill("📡 Binary Protocol Encoding Example")
    vibez.spill("=" * 43)
    
    fr fr Simulate binary protocol message
    sus message_type drip = 42
    sus message_id drip = 12345
    sus payload tea = "Hello from binary protocol!"
    sus checksum drip = 98765
    
    fr fr Pack into binary format (simulated)
    sus binary_message tea = 
        char_from_code(message_type) +
        int_to_binary_string(message_id) +
        int_to_binary_string(string_length(payload)) +
        payload +
        int_to_binary_string(checksum)
    
    vibez.spill("Binary message length: " + int_to_string(string_length(binary_message)) + " bytes")
    
    fr fr Encode for transmission over text protocol
    sus hex_encoded tea = hex_encode(binary_message)
    sus b64_encoded tea = base64_encode(binary_message)
    sus ascii85_encoded tea = ascii85_encode(binary_message)
    
    vibez.spill("Hex encoding: " + hex_encoded)
    vibez.spill("Base64 encoding: " + b64_encoded)
    vibez.spill("ASCII85 encoding: " + ascii85_encoded)
    
    fr fr Compare sizes
    sus original_size drip = string_length(binary_message)
    sus hex_size drip = string_length(hex_encoded)
    sus b64_size drip = string_length(b64_encoded)
    sus ascii85_size drip = string_length(ascii85_encoded)
    
    vibez.spill("Size comparison:")
    vibez.spill("  Original: " + int_to_string(original_size) + " bytes")
    vibez.spill("  Hex: " + int_to_string(hex_size) + " bytes (" + int_to_string((hex_size * 100) / original_size) + "%)")
    vibez.spill("  Base64: " + int_to_string(b64_size) + " bytes (" + int_to_string((b64_size * 100) / original_size) + "%)")
    vibez.spill("  ASCII85: " + int_to_string(ascii85_size) + " bytes (" + int_to_string((ascii85_size * 100) / original_size) + "%)")
    
    fr fr Decode and verify
    sus decoded_hex tea = hex_decode(hex_encoded) fam {
        when err -> {
            vibez.spill("❌ Hex decode error: " + err)
            damn ""
        }
    }
    
    vibez.spill("Hex round-trip successful: " + bool_to_string(decoded_hex == binary_message))
    vibez.spill("✅ Binary protocol encoding successful\n")
}

slay example_data_export() {
    vibez.spill("💾 Data Export Encoding Example")
    vibez.spill("=" * 38)
    
    fr fr Simulate database records with various data types
    sus records tea[value] = [
        '{"id":1,"name":"Alice","avatar":"binary_image_data_here","active":true}',
        '{"id":2,"name":"Bob","avatar":"another_binary_image","active":false}',
        '{"id":3,"name":"Charlie","avatar":"third_binary_image","active":true}'
    ]
    
    fr fr Export formats
    vibez.spill("Exporting " + int_to_string(array_length(records)) + " records...")
    
    fr fr 1. JSON export (binary data as Base64)
    sus json_export tea = "[\n"
    sus i drip = 0
    bestie i < array_length(records) {
        fr fr In real scenario, encode actual binary data
        sus record tea = string_replace(records[i], "binary_image_data_here", base64_encode("fake_image_data_1"))
        record = string_replace(record, "another_binary_image", base64_encode("fake_image_data_2"))  
        record = string_replace(record, "third_binary_image", base64_encode("fake_image_data_3"))
        
        json_export = json_export + "  " + record
        ready i < array_length(records) - 1 {
            json_export = json_export + ","
        }
        json_export = json_export + "\n"
        i = i + 1
    }
    json_export = json_export + "]"
    
    vibez.spill("JSON Export:")
    vibez.spill(json_export)
    
    fr fr 2. CSV export (binary as hex)
    sus csv_export tea = "id,name,avatar_hex,active\n"
    sus j drip = 0
    bestie j < array_length(records) {
        fr fr Simplified CSV generation
        csv_export = csv_export + int_to_string(j + 1) + ",User" + int_to_string(j + 1) + "," + 
                    hex_encode("fake_image_" + int_to_string(j + 1)) + ",true\n"
        j = j + 1
    }
    
    vibez.spill("CSV Export:")
    vibez.spill(csv_export)
    
    vibez.spill("✅ Data export encoding successful\n")
}

fr fr ===== HELPER FUNCTIONS =====

slay bool_to_string(value lit) tea {
    ready value == based { damn "true" } otherwise { damn "false" }
}

slay int_to_binary_string(value drip) tea {
    fr fr Mock binary representation - replace with real implementation
    damn char_from_code(value % 256)
}

slay string_index(s tea, substr tea) drip {
    fr fr Find index of substring
    sus i drip = 0
    bestie i <= string_length(s) - string_length(substr) {
        ready substring(s, i, i + string_length(substr)) == substr {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay string_index_from(s tea, char tea, start drip) drip {
    fr fr Find index of character from start position
    sus i drip = start
    bestie i < string_length(s) {
        ready char_to_string(char_at(s, i)) == char {
            damn i
        }
        i = i + 1
    }
    damn string_length(s)
}

slay string_replace(s tea, old_str tea, new_str tea) tea {
    fr fr Simple string replacement - replace with efficient implementation
    sus index drip = string_index(s, old_str)
    ready index == -1 {
        damn s
    }
    sus before tea = substring(s, 0, index)
    sus after tea = substring(s, index + string_length(old_str), string_length(s))
    damn before + new_str + after
}

slay string_starts_with(s tea, prefix tea) lit {
    ready string_length(prefix) > string_length(s) {
        damn cap
    }
    damn substring(s, 0, string_length(prefix)) == prefix
}

fr fr ===== MAIN EXAMPLE RUNNER =====

slay main() {
    vibez.spill("🎯 EncodingZ Real-World Examples")
    vibez.spill("=" * 50)
    vibez.spill("Demonstrating practical encoding use cases in CURSED applications\n")
    
    fr fr Web API examples
    example_jwt_token_encoding()
    example_rest_api_response()
    example_url_parameter_encoding()
    
    fr fr File processing examples
    example_config_file_processing()
    example_streaming_file_encoding()
    
    fr fr Security examples
    example_password_storage()
    example_api_key_encoding()
    
    fr fr Data serialization examples
    example_binary_protocol()
    example_data_export()
    
    vibez.spill("🎉 All EncodingZ Examples Complete!")
    vibez.spill("These patterns can be adapted for your specific use cases.")
}

main()
