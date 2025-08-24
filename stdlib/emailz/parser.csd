// EmailZ Parser Module - RFC 5322 Email Parsing Implementation
// Comprehensive email parsing with MIME multipart support

yeet "emailz/core"
yeet "stringz"
yeet "arrayz"

// ============================================================================
// Email Parsing Functions
// ============================================================================

// Parses a complete raw email message into structured data
slay parse_email(raw_email tea) yikes<ParsedEmail> {
    ready (stringz.len(raw_email) == 0) {
        yikes email_format_error("Empty email message")
    }
    
    // Split headers from body (double CRLF separator)
    sus separator_pos drip = stringz.find_first(raw_email, "\r\n\r\n")
    ready (separator_pos == -1) {
        // Try single LF separator as fallback
        separator_pos = stringz.find_first(raw_email, "\n\n")
        ready (separator_pos == -1) {
            yikes email_format_error("No header/body separator found")
        }
    }
    
    sus raw_headers tea = stringz.substring(raw_email, 0, separator_pos)
    sus raw_body tea = stringz.substring(raw_email, separator_pos + 4, stringz.len(raw_email))  // +4 for \r\n\r\n
    
    // Parse headers
    sus headers []EmailHeader = parse_email_headers(raw_headers) fam {
        when err -> yikes err
    }
    
    // Extract standard header values
    sus from tea = get_header_value(headers, "From")
    sus to_header tea = get_header_value(headers, "To")
    sus cc_header tea = get_header_value(headers, "Cc")
    sus subject tea = get_header_value(headers, "Subject")
    sus date tea = get_header_value(headers, "Date")
    sus message_id tea = get_header_value(headers, "Message-ID")
    sus content_type tea = get_header_value(headers, "Content-Type")
    
    // Parse TO addresses
    sus to_addresses []tea = parse_address_list(to_header)
    sus cc_addresses []tea = parse_address_list(cc_header)
    
    // Initialize parsed email structure
    sus parsed ParsedEmail = ParsedEmail{
        headers: headers,
        from: parse_single_address(from),
        to: to_addresses,
        subject: decode_header_value(subject),
        date: date,
        message_id: message_id,
        body_text: "",
        body_html: "",
        attachments: [],
        raw_headers: raw_headers,
        raw_body: raw_body,
        is_multipart: stringz.contains(content_type, "multipart/"),
        content_type: content_type
    }
    
    // Parse body based on content type
    ready (parsed.is_multipart) {
        parsed = parse_multipart_body(parsed) fam {
            when err -> yikes err
        }
    } otherwise {
        // Simple single-part message
        ready (stringz.contains(content_type, "text/html")) {
            parsed.body_html = decode_body_content(raw_body, get_content_encoding(headers))
        } otherwise {
            parsed.body_text = decode_body_content(raw_body, get_content_encoding(headers))
        }
    }
    
    damn parsed
}

// Parses email headers from raw header section
slay parse_email_headers(header_section tea) yikes<[]EmailHeader> {
    ready (stringz.len(header_section) == 0) {
        damn []
    }
    
    sus headers []EmailHeader = []
    sus lines []tea = stringz.split(header_section, "\r\n")
    
    // Handle line-wrapped headers (RFC 5322 folding)
    sus unfolded_lines []tea = unfold_header_lines(lines)
    
    sus i drip = 0
    bestie (i < arrayz.len(unfolded_lines)) {
        sus line tea = stringz.trim(unfolded_lines[i])
        
        // Skip empty lines
        ready (stringz.len(line) == 0) {
            i = i + 1
            damn // Continue
        }
        
        // Find colon separator
        sus colon_pos drip = stringz.find_first(line, ":")
        ready (colon_pos == -1) {
            yikes email_format_error(stringz.concat(["Invalid header format: ", line]))
        }
        
        sus header_name tea = stringz.trim(stringz.substring(line, 0, colon_pos))
        sus header_value tea = stringz.trim(stringz.substring(line, colon_pos + 1, stringz.len(line)))
        
        // Validate header name
        ready (stringz.len(header_name) == 0) {
            yikes email_format_error("Empty header name found")
        }
        
        sus header EmailHeader = EmailHeader{
            name: header_name,
            value: header_value
        }
        
        headers = arrayz.push(headers, header)
        i = i + 1
    }
    
    damn headers
}

// Gets the value of a specific header (case-insensitive)
slay get_header_value(headers []EmailHeader, header_name tea) tea {
    sus target_name tea = stringz.to_lower(header_name)
    
    sus i drip = 0
    bestie (i < arrayz.len(headers)) {
        sus header EmailHeader = headers[i]
        sus current_name tea = stringz.to_lower(header.name)
        
        ready (stringz.equals(current_name, target_name)) {
            damn header.value
        }
        i = i + 1
    }
    
    damn ""  // Header not found
}

// Gets all values for headers with the same name
slay get_header_values(headers []EmailHeader, header_name tea) []tea {
    sus target_name tea = stringz.to_lower(header_name)
    sus values []tea = []
    
    sus i drip = 0
    bestie (i < arrayz.len(headers)) {
        sus header EmailHeader = headers[i]
        sus current_name tea = stringz.to_lower(header.name)
        
        ready (stringz.equals(current_name, target_name)) {
            values = arrayz.push(values, header.value)
        }
        i = i + 1
    }
    
    damn values
}

// ============================================================================
// MIME Multipart Parsing
// ============================================================================

// Parses multipart message body and extracts parts
slay parse_multipart_body(parsed ParsedEmail) yikes<ParsedEmail> {
    // Extract boundary from Content-Type header
    sus boundary tea = extract_mime_boundary(parsed.content_type)
    ready (stringz.len(boundary) == 0) {
        yikes email_format_error("No MIME boundary found in multipart message")
    }
    
    // Split body into parts using boundary
    sus parts []tea = split_mime_parts(parsed.raw_body, boundary)
    
    sus i drip = 0
    bestie (i < arrayz.len(parts)) {
        sus part tea = parts[i]
        parsed = process_mime_part(parsed, part) fam {
            when err -> {
                // Log warning but continue processing other parts
                damn // Continue with next part
            }
        }
        i = i + 1
    }
    
    damn parsed
}

// Extracts MIME boundary from Content-Type header
slay extract_mime_boundary(content_type tea) tea {
    sus boundary_start drip = stringz.find_first(content_type, "boundary=")
    ready (boundary_start == -1) {
        damn ""
    }
    
    boundary_start = boundary_start + 9  // Length of "boundary="
    
    sus boundary_value tea = stringz.substring(content_type, boundary_start, stringz.len(content_type))
    boundary_value = stringz.trim(boundary_value)
    
    // Remove quotes if present
    ready (stringz.starts_with(boundary_value, "\"") && stringz.ends_with(boundary_value, "\"")) {
        boundary_value = stringz.substring(boundary_value, 1, stringz.len(boundary_value) - 1)
    }
    
    // Handle parameter continuation
    sus semicolon_pos drip = stringz.find_first(boundary_value, ";")
    ready (semicolon_pos != -1) {
        boundary_value = stringz.substring(boundary_value, 0, semicolon_pos)
    }
    
    damn stringz.trim(boundary_value)
}

// Splits multipart body into individual parts
slay split_mime_parts(body tea, boundary tea) []tea {
    sus parts []tea = []
    sus boundary_delimiter tea = stringz.concat(["--", boundary])
    sus end_boundary tea = stringz.concat([boundary_delimiter, "--"])
    
    // Find all boundary positions
    sus current_pos drip = 0
    sus start_pos drip = -1
    
    bestie (current_pos < stringz.len(body)) {
        sus next_boundary drip = stringz.find_from(body, boundary_delimiter, current_pos)
        ready (next_boundary == -1) {
            damn parts  // No more boundaries found
        }
        
        // Check if this is the end boundary
        sus potential_end tea = stringz.substring(body, next_boundary, next_boundary + stringz.len(end_boundary))
        ready (stringz.equals(potential_end, end_boundary)) {
            // Process final part if we have a start position
            ready (start_pos != -1) {
                sus part_content tea = stringz.substring(body, start_pos, next_boundary)
                parts = arrayz.push(parts, stringz.trim(part_content))
            }
            damn parts
        }
        
        // If we have a previous start position, extract that part
        ready (start_pos != -1) {
            sus part_content tea = stringz.substring(body, start_pos, next_boundary)
            parts = arrayz.push(parts, stringz.trim(part_content))
        }
        
        // Find start of next part (after the boundary line)
        sus line_end drip = stringz.find_from(body, "\r\n", next_boundary)
        ready (line_end == -1) {
            line_end = stringz.find_from(body, "\n", next_boundary)
        }
        
        ready (line_end != -1) {
            start_pos = line_end + 2  // Skip \r\n or \n
            current_pos = start_pos
        } otherwise {
            current_pos = next_boundary + stringz.len(boundary_delimiter)
        }
    }
    
    damn parts
}

// Processes a single MIME part
slay process_mime_part(parsed ParsedEmail, part_content tea) yikes<ParsedEmail> {
    // Split part into headers and body
    sus header_end drip = stringz.find_first(part_content, "\r\n\r\n")
    ready (header_end == -1) {
        header_end = stringz.find_first(part_content, "\n\n")
        ready (header_end == -1) {
            // No headers, treat entire content as body
            yikes email_format_error("Invalid MIME part format - no header/body separator")
        }
    }
    
    sus part_headers_raw tea = stringz.substring(part_content, 0, header_end)
    sus part_body tea = stringz.substring(part_content, header_end + 4, stringz.len(part_content))
    
    // Parse part headers
    sus part_headers []EmailHeader = parse_email_headers(part_headers_raw) fam {
        when err -> yikes err
    }
    
    sus content_type tea = get_header_value(part_headers, "Content-Type")
    sus content_disposition tea = get_header_value(part_headers, "Content-Disposition")
    sus content_transfer_encoding tea = get_header_value(part_headers, "Content-Transfer-Encoding")
    sus content_id tea = get_header_value(part_headers, "Content-ID")
    
    // Decode body content based on transfer encoding
    sus decoded_body tea = decode_body_content(part_body, content_transfer_encoding)
    
    // Determine how to handle this part
    ready (stringz.contains(content_disposition, "attachment")) {
        // This is an attachment
        sus attachment EmailAttachment = create_attachment_from_mime_part(part_headers, decoded_body)
        parsed.attachments = arrayz.push(parsed.attachments, attachment)
        
    } otherwise ready (stringz.contains(content_type, "text/plain")) {
        // Plain text body part
        ready (stringz.len(parsed.body_text) == 0) {  // Use first text part
            parsed.body_text = decoded_body
        }
        
    } otherwise ready (stringz.contains(content_type, "text/html")) {
        // HTML body part
        ready (stringz.len(parsed.body_html) == 0) {  // Use first HTML part
            parsed.body_html = decoded_body
        }
        
    } otherwise ready (stringz.contains(content_type, "multipart/")) {
        // Nested multipart - recursively process
        sus nested_boundary tea = extract_mime_boundary(content_type)
        ready (stringz.len(nested_boundary) > 0) {
            sus nested_parts []tea = split_mime_parts(decoded_body, nested_boundary)
            sus j drip = 0
            bestie (j < arrayz.len(nested_parts)) {
                parsed = process_mime_part(parsed, nested_parts[j]) fam {
                    when err -> {
                        // Log error but continue
                    }
                }
                j = j + 1
            }
        }
        
    } otherwise {
        // Unknown or binary content - treat as attachment
        sus attachment EmailAttachment = create_attachment_from_mime_part(part_headers, decoded_body)
        parsed.attachments = arrayz.push(parsed.attachments, attachment)
    }
    
    damn parsed
}

// Creates an attachment from a MIME part
slay create_attachment_from_mime_part(headers []EmailHeader, content tea) EmailAttachment {
    sus content_type tea = get_header_value(headers, "Content-Type")
    sus content_disposition tea = get_header_value(headers, "Content-Disposition")
    sus content_id tea = get_header_value(headers, "Content-ID")
    sus content_transfer_encoding tea = get_header_value(headers, "Content-Transfer-Encoding")
    
    // Extract filename from Content-Disposition
    sus filename tea = extract_filename_from_disposition(content_disposition)
    ready (stringz.len(filename) == 0) {
        // Fallback: try to get filename from Content-Type
        filename = extract_filename_from_content_type(content_type)
    }
    ready (stringz.len(filename) == 0) {
        // Generate a default filename
        filename = "attachment.bin"
    }
    
    // Clean up Content-ID (remove angle brackets)
    ready (stringz.starts_with(content_id, "<") && stringz.ends_with(content_id, ">")) {
        content_id = stringz.substring(content_id, 1, stringz.len(content_id) - 1)
    }
    
    // Determine disposition
    sus disposition tea = "attachment"
    ready (stringz.contains(content_disposition, "inline")) {
        disposition = "inline"
    }
    
    damn EmailAttachment{
        filename: filename,
        content_type: content_type,
        content: content,  // Already decoded
        content_id: content_id,
        disposition: disposition,
        encoding: content_transfer_encoding,
        size: stringz.len(content)
    }
}

// ============================================================================
// Content Decoding Functions
// ============================================================================

// Decodes body content based on transfer encoding
slay decode_body_content(content tea, encoding tea) tea {
    sus encoding_lower tea = stringz.to_lower(stringz.trim(encoding))
    
    sick(encoding_lower) {
        when "base64" -> {
            damn decode_base64(stringz.replace_all(content, "\r\n", "")) fam {
                when err -> damn content  // Return as-is if decoding fails
            }
        }
        when "quoted-printable" -> {
            damn decode_quoted_printable(content) fam {
                when err -> damn content
            }
        }
        when "7bit", "8bit", "binary", "" -> {
            damn content  // No decoding needed
        }
        _ -> {
            damn content  // Unknown encoding, return as-is
        }
    }
}

// Decodes quoted-printable content
slay decode_quoted_printable(content tea) yikes<tea> {
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < stringz.len(content)) {
        sus char tea = stringz.char_at(content, i)
        
        ready (stringz.equals(char, "=")) {
            // Check for soft line break (= at end of line)
            ready (i + 1 < stringz.len(content)) {
                sus next_char tea = stringz.char_at(content, i + 1)
                ready (stringz.equals(next_char, "\r") || stringz.equals(next_char, "\n")) {
                    // Soft line break - skip the = and following whitespace
                    i = i + 1
                    ready (stringz.equals(next_char, "\r") && i + 1 < stringz.len(content) && stringz.equals(stringz.char_at(content, i + 1), "\n")) {
                        i = i + 1  // Skip \n after \r
                    }
                    i = i + 1
                    damn // Continue
                }
            }
            
            // Hex encoded character
            ready (i + 2 < stringz.len(content)) {
                sus hex_digits tea = stringz.substring(content, i + 1, i + 3)
                sus decoded_char tea = hex_to_char(hex_digits) fam {
                    when err -> {
                        // Invalid hex, keep as-is
                        result = stringz.concat([result, char])
                        i = i + 1
                        damn // Continue
                    }
                }
                result = stringz.concat([result, decoded_char])
                i = i + 3
            } otherwise {
                result = stringz.concat([result, char])
                i = i + 1
            }
            
        } otherwise {
            result = stringz.concat([result, char])
            i = i + 1
        }
    }
    
    damn result
}

// Decodes header values (handles RFC 2047 encoded-words)
slay decode_header_value(header_value tea) tea {
    ready (!stringz.contains(header_value, "=?") || !stringz.contains(header_value, "?=")) {
        damn header_value  // No encoding
    }
    
    sus result tea = header_value
    sus start drip = 0
    
    bestie (start < stringz.len(result)) {
        sus encoded_start drip = stringz.find_from(result, "=?", start)
        ready (encoded_start == -1) {
            damn result  // No more encoded words
        }
        
        sus encoded_end drip = stringz.find_from(result, "?=", encoded_start)
        ready (encoded_end == -1) {
            damn result  // Malformed encoded word
        }
        
        sus encoded_word tea = stringz.substring(result, encoded_start, encoded_end + 2)
        sus decoded_word tea = decode_encoded_word(encoded_word) fam {
            when err -> damn result  // Keep original if decoding fails
        }
        
        // Replace encoded word with decoded version
        result = stringz.replace_first(result, encoded_word, decoded_word)
        start = encoded_start + stringz.len(decoded_word)
    }
    
    damn result
}

// Decodes RFC 2047 encoded word (=?charset?encoding?data?=)
slay decode_encoded_word(encoded_word tea) yikes<tea> {
    // Remove =? prefix and ?= suffix
    ready (!stringz.starts_with(encoded_word, "=?") || !stringz.ends_with(encoded_word, "?=")) {
        yikes email_format_error("Invalid encoded word format")
    }
    
    sus inner tea = stringz.substring(encoded_word, 2, stringz.len(encoded_word) - 2)
    
    // Split into parts: charset?encoding?data
    sus parts []tea = stringz.split(inner, "?")
    ready (arrayz.len(parts) != 3) {
        yikes email_format_error("Invalid encoded word format - expected 3 parts")
    }
    
    sus charset tea = parts[0]
    sus encoding tea = stringz.to_upper(parts[1])
    sus data tea = parts[2]
    
    // Decode based on encoding type
    sus decoded tea = ""
    ready (stringz.equals(encoding, "B")) {
        // Base64 encoding
        decoded = decode_base64(data) fam {
            when err -> yikes err
        }
    } otherwise ready (stringz.equals(encoding, "Q")) {
        // Quoted-printable encoding (with _ as space)
        sus qp_data tea = stringz.replace_all(data, "_", " ")
        decoded = decode_quoted_printable(qp_data) fam {
            when err -> yikes err
        }
    } otherwise {
        yikes email_format_error(stringz.concat(["Unsupported encoding: ", encoding]))
    }
    
    // Note: charset conversion would be handled here in a full implementation
    // For now, assume UTF-8 or ASCII compatibility
    
    damn decoded
}

// ============================================================================
// Address Parsing Functions
// ============================================================================

// Parses a comma-separated list of email addresses
slay parse_address_list(address_header tea) []tea {
    ready (stringz.len(address_header) == 0) {
        damn []
    }
    
    sus addresses []tea = []
    sus parts []tea = stringz.split(address_header, ",")
    
    sus i drip = 0
    bestie (i < arrayz.len(parts)) {
        sus part tea = stringz.trim(parts[i])
        ready (stringz.len(part) > 0) {
            sus clean_address tea = parse_single_address(part)
            ready (stringz.len(clean_address) > 0) {
                addresses = arrayz.push(addresses, clean_address)
            }
        }
        i = i + 1
    }
    
    damn addresses
}

// Parses a single email address (handles display names)
slay parse_single_address(address_string tea) tea {
    ready (stringz.len(address_string) == 0) {
        damn ""
    }
    
    sus trimmed tea = stringz.trim(address_string)
    
    // Handle format: "Display Name" <email@example.com>
    ready (stringz.contains(trimmed, "<") && stringz.ends_with(trimmed, ">")) {
        sus start_bracket drip = stringz.find_last(trimmed, "<")
        sus email_part tea = stringz.substring(trimmed, start_bracket + 1, stringz.len(trimmed) - 1)
        damn stringz.trim(email_part)
    }
    
    // Handle format: Display Name <email@example.com> (no quotes)
    ready (stringz.contains(trimmed, "<") && stringz.contains(trimmed, ">")) {
        sus start_bracket drip = stringz.find_first(trimmed, "<")
        sus end_bracket drip = stringz.find_first(trimmed, ">")
        sus email_part tea = stringz.substring(trimmed, start_bracket + 1, end_bracket)
        damn stringz.trim(email_part)
    }
    
    // Plain email address
    damn trimmed
}

// ============================================================================
// Utility Functions
// ============================================================================

// Unfolds header lines (handles RFC 5322 line continuation)
slay unfold_header_lines(lines []tea) []tea {
    ready (arrayz.len(lines) == 0) {
        damn []
    }
    
    sus unfolded []tea = []
    sus current_line tea = ""
    
    sus i drip = 0
    bestie (i < arrayz.len(lines)) {
        sus line tea = lines[i]
        
        // Check if this line is a continuation (starts with whitespace)
        ready (stringz.len(line) > 0 && (stringz.starts_with(line, " ") || stringz.starts_with(line, "\t"))) {
            // Continuation line - append to current line
            current_line = stringz.concat([current_line, " ", stringz.trim(line)])
        } otherwise {
            // New header line
            ready (stringz.len(current_line) > 0) {
                unfolded = arrayz.push(unfolded, current_line)
            }
            current_line = line
        }
        i = i + 1
    }
    
    // Add final line
    ready (stringz.len(current_line) > 0) {
        unfolded = arrayz.push(unfolded, current_line)
    }
    
    damn unfolded
}

// Gets content encoding from headers
slay get_content_encoding(headers []EmailHeader) tea {
    sus encoding tea = get_header_value(headers, "Content-Transfer-Encoding")
    ready (stringz.len(encoding) == 0) {
        damn "7bit"  // Default encoding
    }
    damn stringz.to_lower(stringz.trim(encoding))
}

// Extracts filename from Content-Disposition header
slay extract_filename_from_disposition(disposition tea) tea {
    ready (stringz.len(disposition) == 0) {
        damn ""
    }
    
    // Look for filename parameter
    sus filename_start drip = stringz.find_first(disposition, "filename=")
    ready (filename_start == -1) {
        // Try filename* (RFC 2231)
        filename_start = stringz.find_first(disposition, "filename*=")
        ready (filename_start == -1) {
            damn ""
        }
        filename_start = filename_start + 10  // Length of "filename*="
    } otherwise {
        filename_start = filename_start + 9  // Length of "filename="
    }
    
    sus filename_part tea = stringz.substring(disposition, filename_start, stringz.len(disposition))
    filename_part = stringz.trim(filename_part)
    
    // Remove quotes if present
    ready (stringz.starts_with(filename_part, "\"") && stringz.contains(stringz.substring(filename_part, 1, stringz.len(filename_part)), "\"")) {
        sus end_quote drip = stringz.find_from(filename_part, "\"", 1)
        filename_part = stringz.substring(filename_part, 1, end_quote)
    }
    
    // Handle parameter continuation (stop at semicolon)
    sus semicolon_pos drip = stringz.find_first(filename_part, ";")
    ready (semicolon_pos != -1) {
        filename_part = stringz.substring(filename_part, 0, semicolon_pos)
    }
    
    damn stringz.trim(filename_part)
}

// Extracts filename from Content-Type header (fallback)
slay extract_filename_from_content_type(content_type tea) tea {
    ready (stringz.len(content_type) == 0) {
        damn ""
    }
    
    sus name_start drip = stringz.find_first(content_type, "name=")
    ready (name_start == -1) {
        damn ""
    }
    
    name_start = name_start + 5  // Length of "name="
    sus name_part tea = stringz.substring(content_type, name_start, stringz.len(content_type))
    name_part = stringz.trim(name_part)
    
    // Remove quotes if present
    ready (stringz.starts_with(name_part, "\"") && stringz.contains(stringz.substring(name_part, 1, stringz.len(name_part)), "\"")) {
        sus end_quote drip = stringz.find_from(name_part, "\"", 1)
        name_part = stringz.substring(name_part, 1, end_quote)
    }
    
    // Stop at semicolon
    sus semicolon_pos drip = stringz.find_first(name_part, ";")
    ready (semicolon_pos != -1) {
        name_part = stringz.substring(name_part, 0, semicolon_pos)
    }
    
    damn stringz.trim(name_part)
}

// Converts hex digits to character
slay hex_to_char(hex_digits tea) yikes<tea> {
    ready (stringz.len(hex_digits) != 2) {
        yikes email_format_error("Invalid hex digits length")
    }
    
    // Simple hex to decimal conversion
    sus hex_upper tea = stringz.to_upper(hex_digits)
    sus first_digit tea = stringz.char_at(hex_upper, 0)
    sus second_digit tea = stringz.char_at(hex_upper, 1)
    
    sus first_val drip = hex_char_to_int(first_digit) fam {
        when err -> yikes err
    }
    
    sus second_val drip = hex_char_to_int(second_digit) fam {
        when err -> yikes err
    }
    
    sus char_code drip = (first_val * 16) + second_val
    
    // Convert character code to string (implementation dependent)
    damn char_from_code(char_code)
}

// Converts single hex character to integer
slay hex_char_to_int(hex_char tea) yikes<drip> {
    sick(hex_char) {
        when "0" -> damn 0
        when "1" -> damn 1
        when "2" -> damn 2
        when "3" -> damn 3
        when "4" -> damn 4
        when "5" -> damn 5
        when "6" -> damn 6
        when "7" -> damn 7
        when "8" -> damn 8
        when "9" -> damn 9
        when "A" -> damn 10
        when "B" -> damn 11
        when "C" -> damn 12
        when "D" -> damn 13
        when "E" -> damn 14
        when "F" -> damn 15
        _ -> yikes email_format_error(stringz.concat(["Invalid hex character: ", hex_char]))
    }
}

// Helper function to convert character code to string
slay char_from_code(code drip) tea {
    // Convert ASCII code to character string
    ready (code == 0) { damn "\0" }
    ready (code == 9) { damn "\t" }
    ready (code == 10) { damn "\n" }
    ready (code == 13) { damn "\r" }
    ready (code == 32) { damn " " }
    ready (code == 33) { damn "!" }
    ready (code == 34) { damn "\"" }
    ready (code == 35) { damn "#" }
    ready (code == 36) { damn "$" }
    ready (code == 37) { damn "%" }
    ready (code == 38) { damn "&" }
    ready (code == 39) { damn "'" }
    ready (code == 40) { damn "(" }
    ready (code == 41) { damn ")" }
    ready (code == 42) { damn "*" }
    ready (code == 43) { damn "+" }
    ready (code == 44) { damn "," }
    ready (code == 45) { damn "-" }
    ready (code == 46) { damn "." }
    ready (code == 47) { damn "/" }
    ready (code >= 48 && code <= 57) { damn string_from_drip(code - 48) }  // 0-9
    ready (code == 58) { damn ":" }
    ready (code == 59) { damn ";" }
    ready (code == 60) { damn "<" }
    ready (code == 61) { damn "=" }
    ready (code == 62) { damn ">" }
    ready (code == 63) { damn "?" }
    ready (code == 64) { damn "@" }
    ready (code >= 65 && code <= 90) {  // A-Z
        sus chars tea = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        damn stringz.char_at(chars, code - 65)
    }
    ready (code >= 97 && code <= 122) {  // a-z
        sus chars tea = "abcdefghijklmnopqrstuvwxyz"
        damn stringz.char_at(chars, code - 97)
    }
    ready (code == 91) { damn "[" }
    ready (code == 92) { damn "\\" }
    ready (code == 93) { damn "]" }
    ready (code == 94) { damn "^" }
    ready (code == 95) { damn "_" }
    ready (code == 96) { damn "`" }
    ready (code == 123) { damn "{" }
    ready (code == 124) { damn "|" }
    ready (code == 125) { damn "}" }
    ready (code == 126) { damn "~" }
    
    // Default for unknown codes
    damn "?"
}

// Helper function to find string from position
slay stringz.find_from(text tea, search tea, start_pos drip) drip {
    ready (start_pos < 0 || start_pos >= stringz.len(text)) {
        damn -1
    }
    
    ready (stringz.len(search) == 0) {
        damn start_pos
    }
    
    sus text_len drip = stringz.len(text)
    sus search_len drip = stringz.len(search)
    
    sus i drip = start_pos
    bestie (i <= text_len - search_len) {
        sus match lit = based
        sus j drip = 0
        
        bestie (j < search_len) {
            ready (!stringz.equals(stringz.char_at(text, i + j), stringz.char_at(search, j))) {
                match = cap
                damn  // Break inner loop
            }
            j = j + 1
        }
        
        ready (match) {
            damn i
        }
        
        i = i + 1
    }
    
    damn -1
}

// Helper function for string_from_drip (used in char_from_code)
slay string_from_drip(value drip) tea {
    ready (value == 0) { damn "0" }
    ready (value == 1) { damn "1" }
    ready (value == 2) { damn "2" }
    ready (value == 3) { damn "3" }
    ready (value == 4) { damn "4" }
    ready (value == 5) { damn "5" }
    ready (value == 6) { damn "6" }
    ready (value == 7) { damn "7" }
    ready (value == 8) { damn "8" }
    ready (value == 9) { damn "9" }
    
    // For values > 9, use simple conversion
    sus result tea = ""
    sus val drip = value
    ready (val < 0) {
        result = "-"
        val = -val
    }
    
    bestie (val > 0) {
        sus digit drip = val % 10
        sus digit_char tea = stringz.char_at("0123456789", digit)
        result = stringz.concat([digit_char, result])
        val = val / 10
    }
    
    ready (stringz.len(result) == 0 || stringz.equals(result, "-")) {
        damn "0"
    }
    
    damn result
}
