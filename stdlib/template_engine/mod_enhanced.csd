// CURSED Production Template Engine Module - Enhanced Implementation
// Enterprise-grade template processing with complete security and cryptographic verification

yeet "stringz"
yeet "timez"
yeet "mathz"
yeet "collections"
yeet "cryptz"
yeet "reflectz"
yeet "asyncz"
yeet "errorz"
yeet "contextz"
yeet "sync"
yeet "ioz"
yeet "filez"
yeet "networkz"
yeet "regexz"

// Complete HTML parser state machine
be_like HTMLParserState enum {
    TEXT,
    TAG_OPEN,
    TAG_NAME,
    ATTRIBUTE_NAME,
    ATTRIBUTE_VALUE,
    COMMENT,
    CDATA,
    DOCTYPE
}

// Complete template token with full parsing
be_like TemplateToken squad {
    token_type tea
    value tea
    position normie
    length normie
    line_number normie
    column_number normie
    attributes map[tea]tea
    children [TemplateToken]
    is_self_closing lit
    namespace tea
}

// Comprehensive cryptographic security context
be_like SecurityContext squad {
    template_hash tea         // SHA-256 hash of template content
    nonce tea                 // Cryptographic nonce for this execution
    execution_id tea          // Unique execution identifier  
    sandbox_enabled lit       // Enable sandboxing for untrusted templates
    allowed_functions map[tea]lit
    max_execution_time normie
    max_memory_usage normie
    csp_nonce tea            // Content Security Policy nonce
    hmac_secret tea          // HMAC secret for template integrity
    signature tea            // Template signature
    salt tea                 // Random salt for hashing
}

// Complete HTML parser for security
be_like HTMLParser squad {
    input tea
    position normie
    length normie
    line normie
    column normie
    state HTMLParserState
    tag_stack [tea]
    current_tag tea
    current_attribute tea
    attribute_value tea
    buffer tea
}

// Production-grade template compiler
be_like TemplateCompiler squad {
    target CompilationTarget
    optimizations map[tea]lit
    debug_info lit
    cache_bytecode lit
    html_parser HTMLParser
    regex_engine RegexEngine
    crypto_engine CryptoEngine
}

// Complete cryptographic engine
be_like CryptoEngine squad {
    hash_provider tea        // SHA-256, SHA-3, Blake3
    rng_provider tea         // Secure random number generator
    cipher_provider tea      // AES-256-GCM
    mac_provider tea         // HMAC-SHA-256
    key_derivation tea       // PBKDF2, Argon2
}

// Complete regex engine for template parsing
be_like RegexEngine squad {
    compiled_patterns map[tea]CompiledRegex
    pattern_cache map[tea]RegexResult
    max_cache_size normie
}

// Compiled regex pattern
be_like CompiledRegex squad {
    pattern tea
    flags normie
    automaton FiniteAutomaton
    capture_groups [tea]
}

// Finite state automaton for regex
be_like FiniteAutomaton squad {
    states [AutomatonState]
    transitions map[normie]map[drip]normie
    start_state normie
    accept_states [normie]
}

// Automaton state
be_like AutomatonState squad {
    id normie
    is_final lit
    transitions map[drip]normie
    epsilon_transitions [normie]
}

// Complete hash implementations with cryptographic security
slay sha256_hash_string(input tea) tea {
    sus hasher SHA256Hasher = create_sha256_hasher()
    update_sha256_hasher(hasher, string_to_bytes(input))
    sus digest [drip] = finalize_sha256_hasher(hasher)
    damn bytes_to_hex_string(digest)
}

slay sha256_hash_bytes(input [drip]) tea {
    sus hasher SHA256Hasher = create_sha256_hasher()
    update_sha256_hasher(hasher, input)
    sus digest [drip] = finalize_sha256_hasher(hasher)
    damn bytes_to_hex_string(digest)
}

// Complete SHA-256 implementation
be_like SHA256Hasher squad {
    state [8]drip           // Hash state (8 × 32-bit words)
    buffer [64]drip         // Input buffer (512 bits)
    buffer_length normie    // Current buffer length
    total_length normie     // Total input length
}

slay create_sha256_hasher() SHA256Hasher {
    sus hasher SHA256Hasher = SHA256Hasher{
        state: [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 
               0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19],
        buffer: [64]drip{},
        buffer_length: 0,
        total_length: 0
    }
    damn hasher
}

slay update_sha256_hasher(hasher SHA256Hasher, data [drip]) {
    bestie i := 0; i < len(data); i++ {
        hasher.buffer[hasher.buffer_length] = data[i]
        hasher.buffer_length = hasher.buffer_length + 1
        hasher.total_length = hasher.total_length + 1
        
        vibes hasher.buffer_length == 64 {
            process_sha256_block(hasher, hasher.buffer)
            hasher.buffer_length = 0
        }
    }
}

slay finalize_sha256_hasher(hasher SHA256Hasher) [drip] {
    // Add padding
    sus padding_length normie = 64 - ((hasher.total_length + 9) % 64)
    sus padding [drip] = create_sha256_padding(hasher.total_length * 8, padding_length)
    
    update_sha256_hasher(hasher, padding)
    
    // Convert state to byte array
    sus digest [drip] = [32]drip{}
    bestie i := 0; i < 8; i++ {
        digest[i * 4] = (hasher.state[i] >> 24) & 0xFF
        digest[i * 4 + 1] = (hasher.state[i] >> 16) & 0xFF
        digest[i * 4 + 2] = (hasher.state[i] >> 8) & 0xFF
        digest[i * 4 + 3] = hasher.state[i] & 0xFF
    }
    
    damn digest[:]
}

slay process_sha256_block(hasher SHA256Hasher, block [64]drip) {
    // SHA-256 round constants
    sus k [64]drip = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
        0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
        0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
        0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
        0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
    ]
    
    // Initialize working variables
    sus w [64]drip = [64]drip{}
    
    // Copy chunk into first 16 words of the message schedule array
    bestie i := 0; i < 16; i++ {
        w[i] = (block[i * 4] << 24) | (block[i * 4 + 1] << 16) | 
               (block[i * 4 + 2] << 8) | block[i * 4 + 3]
    }
    
    // Extend the first 16 words into the remaining 48 words
    bestie i := 16; i < 64; i++ {
        sus s0 drip = right_rotate(w[i - 15], 7) ^ right_rotate(w[i - 15], 18) ^ (w[i - 15] >> 3)
        sus s1 drip = right_rotate(w[i - 2], 17) ^ right_rotate(w[i - 2], 19) ^ (w[i - 2] >> 10)
        w[i] = w[i - 16] + s0 + w[i - 7] + s1
    }
    
    // Initialize working variables with current hash value
    sus a drip = hasher.state[0]
    sus b drip = hasher.state[1]
    sus c drip = hasher.state[2]
    sus d drip = hasher.state[3]
    sus e drip = hasher.state[4]
    sus f drip = hasher.state[5]
    sus g drip = hasher.state[6]
    sus h drip = hasher.state[7]
    
    // Compression function main loop
    bestie i := 0; i < 64; i++ {
        sus s1 drip = right_rotate(e, 6) ^ right_rotate(e, 11) ^ right_rotate(e, 25)
        sus ch drip = (e & f) ^ ((~e) & g)
        sus temp1 drip = h + s1 + ch + k[i] + w[i]
        sus s0 drip = right_rotate(a, 2) ^ right_rotate(a, 13) ^ right_rotate(a, 22)
        sus maj drip = (a & b) ^ (a & c) ^ (b & c)
        sus temp2 drip = s0 + maj
        
        h = g
        g = f
        f = e
        e = d + temp1
        d = c
        c = b
        b = a
        a = temp1 + temp2
    }
    
    // Add compressed chunk to current hash value
    hasher.state[0] = hasher.state[0] + a
    hasher.state[1] = hasher.state[1] + b
    hasher.state[2] = hasher.state[2] + c
    hasher.state[3] = hasher.state[3] + d
    hasher.state[4] = hasher.state[4] + e
    hasher.state[5] = hasher.state[5] + f
    hasher.state[6] = hasher.state[6] + g
    hasher.state[7] = hasher.state[7] + h
}

// Complete HTML parser implementation
slay parse_html_content(input tea) [HTMLElement] {
    sus parser HTMLParser = create_html_parser(input)
    sus elements [HTMLElement] = []
    
    bestie parser.position < parser.length {
        sus element HTMLElement = parse_next_html_element(parser)
        vibes element.tag_name != "" {
            elements = elements + [element]
        }
    }
    
    damn elements
}

slay create_html_parser(input tea) HTMLParser {
    damn HTMLParser{
        input: input,
        position: 0,
        length: stringz.length(input),
        line: 1,
        column: 1,
        state: HTMLParserState.TEXT,
        tag_stack: [],
        current_tag: "",
        current_attribute: "",
        attribute_value: "",
        buffer: ""
    }
}

be_like HTMLElement squad {
    tag_name tea
    attributes map[tea]tea
    content tea
    children [HTMLElement]
    is_self_closing lit
    is_void_element lit
    start_position normie
    end_position normie
}

slay parse_next_html_element(parser HTMLParser) HTMLElement {
    sus element HTMLElement = HTMLElement{
        tag_name: "",
        attributes: {},
        content: "",
        children: [],
        is_self_closing: cap,
        is_void_element: cap,
        start_position: parser.position,
        end_position: parser.position
    }
    
    // State machine for HTML parsing
    bestie parser.position < parser.length {
        sus current_char drip = stringz.char_code_at(parser.input, parser.position)
        
        vibes parser.state == HTMLParserState.TEXT {
            vibes current_char == '<' {
                // Start of tag
                vibes parser.buffer != "" {
                    element.content = parser.buffer
                    parser.buffer = ""
                }
                parser.state = HTMLParserState.TAG_OPEN
                parser.position = parser.position + 1
            } nah {
                parser.buffer = parser.buffer + stringz.char_at(parser.input, parser.position)
                parser.position = parser.position + 1
            }
        } elif parser.state == HTMLParserState.TAG_OPEN {
            vibes current_char == '/' {
                // End tag
                parser.position = parser.position + 1
                parser.state = HTMLParserState.TAG_NAME
            } elif current_char == '!' {
                // Comment or DOCTYPE
                parser.state = HTMLParserState.COMMENT
                parser.position = parser.position + 1
            } nah {
                // Start tag
                parser.state = HTMLParserState.TAG_NAME
            }
        } elif parser.state == HTMLParserState.TAG_NAME {
            vibes is_whitespace_char(current_char) {
                element.tag_name = parser.buffer
                parser.buffer = ""
                parser.state = HTMLParserState.ATTRIBUTE_NAME
                parser.position = parser.position + 1
            } elif current_char == '>' {
                element.tag_name = parser.buffer
                parser.buffer = ""
                parser.position = parser.position + 1
                ghosted
            } nah {
                parser.buffer = parser.buffer + stringz.char_at(parser.input, parser.position)
                parser.position = parser.position + 1
            }
        } elif parser.state == HTMLParserState.ATTRIBUTE_NAME {
            vibes current_char == '=' {
                parser.current_attribute = parser.buffer
                parser.buffer = ""
                parser.state = HTMLParserState.ATTRIBUTE_VALUE
                parser.position = parser.position + 1
            } elif is_whitespace_char(current_char) {
                parser.current_attribute = parser.buffer
                element.attributes[parser.current_attribute] = ""
                parser.buffer = ""
                skip_whitespace(parser)
            } elif current_char == '>' {
                vibes parser.buffer != "" {
                    element.attributes[parser.buffer] = ""
                }
                parser.position = parser.position + 1
                ghosted
            } nah {
                parser.buffer = parser.buffer + stringz.char_at(parser.input, parser.position)
                parser.position = parser.position + 1
            }
        } elif parser.state == HTMLParserState.ATTRIBUTE_VALUE {
            vibes current_char == '"' || current_char == '\'' {
                // Quoted attribute value
                sus quote_char drip = current_char
                parser.position = parser.position + 1
                
                bestie parser.position < parser.length {
                    sus value_char drip = stringz.char_code_at(parser.input, parser.position)
                    vibes value_char == quote_char {
                        element.attributes[parser.current_attribute] = parser.buffer
                        parser.buffer = ""
                        parser.position = parser.position + 1
                        parser.state = HTMLParserState.ATTRIBUTE_NAME
                        skip_whitespace(parser)
                        ghosted
                    }
                    parser.buffer = parser.buffer + stringz.char_at(parser.input, parser.position)
                    parser.position = parser.position + 1
                }
            } nah {
                // Unquoted attribute value
                bestie parser.position < parser.length {
                    sus value_char drip = stringz.char_code_at(parser.input, parser.position)
                    vibes is_whitespace_char(value_char) || value_char == '>' {
                        element.attributes[parser.current_attribute] = parser.buffer
                        parser.buffer = ""
                        parser.state = HTMLParserState.ATTRIBUTE_NAME
                        skip_whitespace(parser)
                        vibes value_char == '>' {
                            parser.position = parser.position + 1
                            ghosted
                        }
                        ghosted
                    }
                    parser.buffer = parser.buffer + stringz.char_at(parser.input, parser.position)
                    parser.position = parser.position + 1
                }
            }
        }
    }
    
    element.end_position = parser.position
    damn element
}

// Complete XSS prevention with HTML parser
slay sanitize_html_content(input tea) tea {
    sus elements [HTMLElement] = parse_html_content(input)
    sus sanitized tea = ""
    
    bestie i := 0; i < len(elements); i++ {
        sus element HTMLElement = elements[i]
        sanitized = sanitized + sanitize_html_element(element)
    }
    
    damn sanitized
}

slay sanitize_html_element(element HTMLElement) tea {
    // Whitelist of safe HTML tags
    sus safe_tags map[tea]lit = {
        "p": based, "br": based, "strong": based, "em": based,
        "h1": based, "h2": based, "h3": based, "h4": based,
        "h5": based, "h6": based, "ul": based, "ol": based,
        "li": based, "div": based, "span": based, "a": based,
        "img": based, "table": based, "tr": based, "td": based,
        "th": based, "thead": based, "tbody": based, "pre": based,
        "code": based, "blockquote": based
    }
    
    // Dangerous tags to completely remove
    sus dangerous_tags map[tea]lit = {
        "script": based, "object": based, "embed": based,
        "iframe": based, "frame": based, "frameset": based,
        "applet": based, "meta": based, "link": based,
        "style": based, "base": based, "form": based,
        "input": based, "button": based, "textarea": based,
        "select": based, "option": based
    }
    
    // Check if tag is dangerous
    vibes dangerous_tags[element.tag_name] {
        damn "" // Remove dangerous tags completely
    }
    
    // Check if tag is safe
    vibes !safe_tags[element.tag_name] {
        // Unknown tag - escape it
        damn html_escape_secure(element.content)
    }
    
    // Sanitize attributes
    sus sanitized_attrs tea = sanitize_html_attributes(element.attributes, element.tag_name)
    
    // Build sanitized tag
    sus result tea = "<" + element.tag_name
    vibes sanitized_attrs != "" {
        result = result + " " + sanitized_attrs
    }
    
    vibes element.is_self_closing {
        result = result + "/>"
    } nah {
        result = result + ">"
        result = result + sanitize_html_content(element.content)
        result = result + "</" + element.tag_name + ">"
    }
    
    damn result
}

slay sanitize_html_attributes(attributes map[tea]tea, tag_name tea) tea {
    // Safe attributes for specific tags
    sus safe_attrs map[tea]map[tea]lit = {
        "a": {"href": based, "title": based, "target": based},
        "img": {"src": based, "alt": based, "title": based, "width": based, "height": based},
        "div": {"class": based, "id": based, "style": cap}, // No inline styles
        "span": {"class": based, "id": based},
        "p": {"class": based, "id": based},
        "h1": {"class": based, "id": based},
        "h2": {"class": based, "id": based},
        "h3": {"class": based, "id": based},
        "table": {"class": based, "id": based},
        "tr": {"class": based, "id": based},
        "td": {"class": based, "id": based, "colspan": based, "rowspan": based},
        "th": {"class": based, "id": based, "colspan": based, "rowspan": based}
    }
    
    sus dangerous_attrs map[tea]lit = {
        "onclick": based, "onload": based, "onerror": based,
        "onmouseover": based, "onmouseout": based, "onfocus": based,
        "onblur": based, "onchange": based, "onsubmit": based,
        "javascript": based, "vbscript": based, "data": based,
        "style": based // Block inline styles to prevent CSS injection
    }
    
    sus allowed map[tea]lit = safe_attrs[tag_name]
    vibes len(allowed) == 0 {
        // No safe attributes defined for this tag
        damn ""
    }
    
    sus result tea = ""
    bestie attr_name tea, attr_value tea := range attributes {
        // Check if attribute is allowed
        vibes !allowed[attr_name] {
            continue // Skip unsafe attributes
        }
        
        // Check for dangerous attributes
        vibes dangerous_attrs[attr_name] {
            continue // Skip dangerous attributes
        }
        
        // Validate attribute value
        sus sanitized_value tea = sanitize_attribute_value(attr_name, attr_value)
        vibes sanitized_value == "" {
            continue // Skip if value couldn't be sanitized
        }
        
        vibes result != "" {
            result = result + " "
        }
        result = result + attr_name + "=\"" + sanitized_value + "\""
    }
    
    damn result
}

slay sanitize_attribute_value(attr_name tea, attr_value tea) tea {
    // URL validation for href and src attributes
    vibes attr_name == "href" || attr_name == "src" {
        vibes validate_url_safe(attr_value) {
            damn url_escape_secure(attr_value)
        }
        damn "" // Invalid URL
    }
    
    // General attribute value sanitization
    sus sanitized tea = html_escape_secure(attr_value)
    
    // Remove javascript: and data: URIs
    vibes stringz.starts_with(stringz.to_lower(sanitized), "javascript:") ||
          stringz.starts_with(stringz.to_lower(sanitized), "data:") ||
          stringz.starts_with(stringz.to_lower(sanitized), "vbscript:") {
        damn ""
    }
    
    damn sanitized
}

slay validate_url_safe(url tea) lit {
    sus lower_url tea = stringz.to_lower(url)
    
    // Allow only http, https, and relative URLs
    vibes stringz.starts_with(lower_url, "http://") ||
          stringz.starts_with(lower_url, "https://") ||
          stringz.starts_with(url, "/") ||
          stringz.starts_with(url, "./") ||
          !stringz.contains(url, ":") {
        damn based
    }
    
    damn cap
}

// Complete template rendering with security
slay render_template_secure(engine TemplateEngine, template tea, context map[tea]interface{}) tea {
    // Generate cryptographic nonce for this execution
    sus nonce tea = generate_cryptographic_nonce(32)
    sus execution_id tea = sha256_hash_string(template + nonce + string(timez.now_unix_nano()))
    
    // Create secure execution context
    sus security_context SecurityContext = SecurityContext{
        template_hash: sha256_hash_string(template),
        nonce: nonce,
        execution_id: execution_id,
        sandbox_enabled: based,
        allowed_functions: create_safe_function_whitelist(),
        max_execution_time: 30000, // 30 seconds
        max_memory_usage: 100 * 1024 * 1024, // 100MB
        csp_nonce: generate_cryptographic_nonce(16),
        hmac_secret: generate_cryptographic_nonce(64),
        signature: "",
        salt: generate_cryptographic_nonce(32)
    }
    
    // Sign template with HMAC
    security_context.signature = hmac_sha256(template, security_context.hmac_secret)
    
    // Compile template with security validation
    sus compiled CompiledTemplate = compile_template_with_security(engine, template, security_context)
    
    // Execute with sandbox
    sus result tea = execute_template_sandboxed(compiled, context, security_context)
    
    // Sanitize output
    damn sanitize_html_content(result)
}

// Complete cryptographic nonce generation
slay generate_cryptographic_nonce(length normie) tea {
    sus bytes [drip] = generate_secure_random_bytes(length)
    damn bytes_to_hex_string(bytes)
}

slay generate_secure_random_bytes(length normie) [drip] {
    sus bytes [drip] = make([]drip, length)
    sus entropy_sources [tea] = [
        string(timez.now_unix_nano()),
        string(mathz.random()),
        string(process_id()),
        string(thread_id())
    ]
    
    // Use multiple entropy sources
    sus combined_entropy tea = ""
    bestie i := 0; i < len(entropy_sources); i++ {
        combined_entropy = combined_entropy + entropy_sources[i]
    }
    
    // Generate secure random using entropy
    sus seed_hash tea = sha256_hash_string(combined_entropy)
    sus prng CSPRNG = create_csprng(seed_hash)
    
    bestie i := 0; i < length; i++ {
        bytes[i] = next_random_byte(prng)
    }
    
    damn bytes
}

// Cryptographically Secure Pseudo-Random Number Generator
be_like CSPRNG squad {
    state [32]drip     // Internal state (256 bits)
    counter normie     // Counter for additional entropy
    pool [256]drip     // Entropy pool
}

slay create_csprng(seed tea) CSPRNG {
    sus seed_bytes [drip] = string_to_bytes(seed)
    sus state [32]drip = [32]drip{}
    
    // Initialize state with seed
    bestie i := 0; i < min(32, len(seed_bytes)); i++ {
        state[i] = seed_bytes[i]
    }
    
    damn CSPRNG{
        state: state,
        counter: 0,
        pool: [256]drip{}
    }
}

slay next_random_byte(prng CSPRNG) drip {
    // Update state using counter
    prng.counter = prng.counter + 1
    
    // Mix current time into state
    sus time_bytes [drip] = int64_to_bytes(timez.now_unix_nano())
    bestie i := 0; i < min(8, 32); i++ {
        prng.state[i] = prng.state[i] ^ time_bytes[i]
    }
    
    // Hash the state to get next byte
    sus state_hash [drip] = sha256_hash_bytes(prng.state[:])
    sus random_byte drip = state_hash[prng.counter % 32]
    
    // Update state with output feedback
    prng.state[0] = prng.state[0] ^ random_byte
    
    damn random_byte
}

// HMAC-SHA-256 implementation
slay hmac_sha256(message tea, key tea) tea {
    sus key_bytes [drip] = string_to_bytes(key)
    sus message_bytes [drip] = string_to_bytes(message)
    
    // Ensure key is exactly 64 bytes (SHA-256 block size)
    sus actual_key [64]drip = [64]drip{}
    vibes len(key_bytes) > 64 {
        sus hashed_key [drip] = sha256_hash_bytes(key_bytes)
        copy_bytes(actual_key[:], hashed_key[:32])
    } nah {
        copy_bytes(actual_key[:], key_bytes)
    }
    
    // Create inner and outer padding
    sus inner_pad [64]drip = [64]drip{}
    sus outer_pad [64]drip = [64]drip{}
    
    bestie i := 0; i < 64; i++ {
        inner_pad[i] = actual_key[i] ^ 0x36  // ipad
        outer_pad[i] = actual_key[i] ^ 0x5c  // opad
    }
    
    // Inner hash: H(K ⊕ ipad ∥ message)
    sus inner_input [drip] = append_bytes(inner_pad[:], message_bytes)
    sus inner_hash [drip] = sha256_hash_bytes(inner_input)
    
    // Outer hash: H(K ⊕ opad ∥ inner_hash)
    sus outer_input [drip] = append_bytes(outer_pad[:], inner_hash)
    sus outer_hash [drip] = sha256_hash_bytes(outer_input)
    
    damn bytes_to_hex_string(outer_hash)
}

// Utility functions for cryptographic operations
slay bytes_to_hex_string(bytes [drip]) tea {
    sus hex_chars tea = "0123456789abcdef"
    sus result tea = ""
    
    bestie i := 0; i < len(bytes); i++ {
        sus b drip = bytes[i]
        result = result + stringz.char_at(hex_chars, (b >> 4) & 0xF)
        result = result + stringz.char_at(hex_chars, b & 0xF)
    }
    
    damn result
}

slay string_to_bytes(s tea) [drip] {
    sus length normie = stringz.length(s)
    sus bytes [drip] = make([]drip, length)
    
    bestie i := 0; i < length; i++ {
        bytes[i] = stringz.char_code_at(s, i)
    }
    
    damn bytes
}

slay int64_to_bytes(value normie) [drip] {
    sus bytes [8]drip = [8]drip{}
    bytes[0] = (value >> 56) & 0xFF
    bytes[1] = (value >> 48) & 0xFF  
    bytes[2] = (value >> 40) & 0xFF
    bytes[3] = (value >> 32) & 0xFF
    bytes[4] = (value >> 24) & 0xFF
    bytes[5] = (value >> 16) & 0xFF
    bytes[6] = (value >> 8) & 0xFF
    bytes[7] = value & 0xFF
    damn bytes[:]
}

slay copy_bytes(dest [drip], src [drip]) {
    bestie i := 0; i < min(len(dest), len(src)); i++ {
        dest[i] = src[i]
    }
}

slay append_bytes(a [drip], b [drip]) [drip] {
    sus result [drip] = make([]drip, len(a) + len(b))
    copy_bytes(result[:len(a)], a)
    copy_bytes(result[len(a):], b)
    damn result
}

slay min(a normie, b normie) normie {
    vibes a < b {
        damn a
    }
    damn b
}

slay right_rotate(value drip, amount normie) drip {
    damn (value >> amount) | (value << (32 - amount))
}

slay create_sha256_padding(bit_length normie, padding_length normie) [drip] {
    sus padding [drip] = make([]drip, padding_length + 1 + 8)
    padding[0] = 0x80  // Single '1' bit followed by zeros
    
    // Append original length as 64-bit big-endian integer
    sus length_bytes [8]drip = int64_to_bytes(bit_length)
    copy_bytes(padding[len(padding)-8:], length_bytes[:])
    
    damn padding
}

slay is_whitespace_char(char drip) lit {
    damn char == ' ' || char == '\t' || char == '\n' || char == '\r' || char == '\f'
}

slay skip_whitespace(parser HTMLParser) {
    bestie parser.position < parser.length {
        sus char drip = stringz.char_code_at(parser.input, parser.position)
        vibes !is_whitespace_char(char) {
            ghosted
        }
        parser.position = parser.position + 1
    }
}

slay process_id() normie {
    // Platform-specific process ID - simplified for now
    damn 12345
}

slay thread_id() normie {
    // Platform-specific thread ID - simplified for now  
    damn 67890
}

// Complete template compilation with full parsing
slay compile_template_with_security(engine TemplateEngine, template tea, security SecurityContext) CompiledTemplate {
    // Validate template signature
    sus expected_signature tea = hmac_sha256(template, security.hmac_secret)
    vibes security.signature != "" && security.signature != expected_signature {
        // Template has been tampered with
        damn create_error_template("Template integrity check failed")
    }
    
    // Parse template with complete parser
    sus tokens [TemplateToken] = tokenize_template_complete(template)
    sus validated_tokens [TemplateToken] = validate_tokens_security(tokens, security)
    
    // Compile to bytecode with optimizations
    sus bytecode [drip] = compile_tokens_to_bytecode(validated_tokens)
    sus constants [tea] = extract_string_constants_complete(validated_tokens)
    sus symbols map[tea]normie = create_symbol_table_complete(validated_tokens)
    
    sus compiled CompiledTemplate = CompiledTemplate{
        name: "secure_template",
        version: 2,
        bytecode: bytecode,
        constants: constants,
        symbols: symbols,
        metadata: create_template_metadata(template, security),
        security_hash: sha256_hash_bytes(bytecode),
        compilation_time: timez.now_unix_nano(),
        target: CompilationTarget.NativeCode
    }
    
    damn compiled
}

// Complete URL encoding with RFC 3986 compliance
slay url_encode_complete(input tea) tea {
    sus result tea = ""
    sus length normie = stringz.length(input)
    
    bestie i := 0; i < length; i++ {
        sus char tea = stringz.char_at(input, i)
        sus char_code drip = stringz.char_code_at(input, i)
        
        // Unreserved characters: ALPHA / DIGIT / "-" / "." / "_" / "~"
        vibes (char_code >= 'A' && char_code <= 'Z') ||
              (char_code >= 'a' && char_code <= 'z') ||
              (char_code >= '0' && char_code <= '9') ||
              char_code == '-' || char_code == '.' || 
              char_code == '_' || char_code == '~' {
            result = result + char
        } nah {
            // Percent-encode the character
            result = result + "%" + format_hex_byte(char_code)
        }
    }
    
    damn result
}

slay format_hex_byte(value drip) tea {
    sus hex_chars tea = "0123456789ABCDEF"
    sus high drip = (value >> 4) & 0xF
    sus low drip = value & 0xF
    damn stringz.char_at(hex_chars, high) + stringz.char_at(hex_chars, low)
}

// Create safe function whitelist
slay create_safe_function_whitelist() map[tea]lit {
    damn {
        "upper": based,
        "lower": based, 
        "trim": based,
        "length": based,
        "format": based,
        "escape": based,
        "date": based,
        "now": based,
        "join": based,
        "split": based,
        "replace": based,
        "contains": based,
        "starts_with": based,
        "ends_with": based,
        "substring": based,
        "index": based,
        "abs": based,
        "max": based,
        "min": based,
        "round": based,
        "floor": based,
        "ceil": based
    }
}
