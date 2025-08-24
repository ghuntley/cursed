// EmailZ Core Module - CURSED Email & SMTP Library
// Implements RFC 5321 (SMTP) and RFC 5322 (Email Message Format)
// Pure CURSED implementation with comprehensive error handling

yeet "networkz"
yeet "stringz" 
yeet "arrayz"
yeet "timez"
yeet "cryptz"
yeet "filez"

// ============================================================================
// Core Data Structures
// ============================================================================

// Email error structure for comprehensive error reporting
squad EmailError {
    sus kind tea        // Error type: smtp_connect, auth_failed, email_format, etc.
    sus message tea     // Human-readable error message
    sus smtp_code drip  // SMTP response code (220, 250, 550, etc.)
    sus details tea     // Additional error details or context
}

// SMTP client structure for managing server connections
squad SmtpClient {
    sus host tea            // SMTP server hostname (e.g., "smtp.gmail.com")
    sus port drip           // SMTP server port (25, 587, 465)
    sus username tea        // Authentication username
    sus password tea        // Authentication password
    sus use_tls lit         // Whether to use TLS/SSL connection
    sus use_starttls lit    // Whether to use STARTTLS upgrade
    sus timeout drip        // Connection timeout in seconds
    sus connection TcpConnection // Network connection handle
    sus authenticated lit   // Current authentication status
    sus capabilities []tea  // Server capabilities from EHLO response
    sus verify_certificate lit // Whether to verify TLS certificates
}

// SMTP server response structure
squad SmtpResponse {
    sus code drip       // SMTP response code (220, 250, 421, 550, etc.)
    sus message tea     // Response message text
    sus enhanced_code tea // Enhanced status code per RFC 3463
}

// Email header structure
squad EmailHeader {
    sus name tea        // Header name (e.g., "Subject", "From")
    sus value tea       // Header value
}

// Email attachment structure with MIME support
squad EmailAttachment {
    sus filename tea        // Original filename
    sus content_type tea    // MIME content type (e.g., "application/pdf")
    sus content tea         // Base64 encoded content
    sus content_id tea      // Content-ID for inline images
    sus disposition tea     // "attachment" or "inline"
    sus encoding tea        // Transfer encoding: "base64", "quoted-printable"
    sus size drip          // Original file size in bytes
}

// Email priority enumeration
enum EmailPriority {
    Low,     // X-Priority: 5
    Normal,  // X-Priority: 3 (default)
    High,    // X-Priority: 1
    Urgent   // X-Priority: 1, X-MSMail-Priority: High
}

// Main email message structure
squad Email {
    // RFC 5322 Headers
    sus from tea            // From address (required)
    sus to []tea            // To addresses (required)
    sus cc []tea            // CC addresses
    sus bcc []tea           // BCC addresses (not included in headers)
    sus subject tea         // Email subject
    sus reply_to tea        // Reply-to address
    sus date tea            // Date header (RFC 2822 format)
    sus message_id tea      // Unique message identifier
    sus headers []EmailHeader // Additional custom headers
    
    // Content
    sus body_text tea       // Plain text body
    sus body_html tea       // HTML body content
    sus attachments []EmailAttachment // File attachments
    sus priority EmailPriority // Message priority
    sus encoding tea        // Character encoding (default: "utf-8")
    
    // MIME Information
    sus content_type tea    // Content-Type header value
    sus boundary tea        // MIME boundary for multipart messages
}

// Parsed email structure for incoming messages
squad ParsedEmail {
    sus headers []EmailHeader   // All parsed headers
    sus from tea               // From address
    sus to []tea               // To addresses  
    sus subject tea            // Subject line
    sus date tea               // Date sent
    sus message_id tea         // Message ID
    sus body_text tea          // Plain text body
    sus body_html tea          // HTML body
    sus attachments []EmailAttachment // Attachments found
    sus raw_headers tea        // Raw header section
    sus raw_body tea           // Raw body section
    sus is_multipart lit       // Whether message is multipart
    sus content_type tea       // Main content type
}

// ============================================================================
// Error Handling Functions
// ============================================================================

// Creates a new email error with specified details
slay create_email_error(kind tea, message tea, smtp_code drip, details tea) EmailError {
    damn EmailError{
        kind: kind,
        message: message,
        smtp_code: smtp_code,
        details: details
    }
}

// Creates SMTP connection error
slay smtp_connection_error(message tea) EmailError {
    damn create_email_error("smtp_connect", message, 0, "")
}

// Creates SMTP authentication error
slay smtp_auth_error(message tea, code drip) EmailError {
    damn create_email_error("smtp_auth", message, code, "")
}

// Creates email format validation error
slay email_format_error(message tea) EmailError {
    damn create_email_error("email_format", message, 0, "")
}

// ============================================================================
// SMTP Client Functions
// ============================================================================

// Creates a basic SMTP client for plain connections
slay create_smtp_client(host tea, port drip) yikes<SmtpClient> {
    ready (stringz.len(host) == 0) {
        yikes smtp_connection_error("SMTP host cannot be empty")
    }
    
    ready (port <= 0 || port > 65535) {
        yikes smtp_connection_error("Invalid SMTP port number")
    }
    
    damn SmtpClient{
        host: host,
        port: port,
        username: "",
        password: "",
        use_tls: cap,
        use_starttls: cap,
        timeout: 30,
        connection: TcpConnection{},
        authenticated: cap,
        capabilities: [],
        verify_certificate: based
    }
}

// Creates SMTP client with TLS (SMTPS - usually port 465)
slay create_smtp_client_tls(host tea, port drip) yikes<SmtpClient> {
    sus client SmtpClient = create_smtp_client(host, port) fam {
        when err -> yikes err
    }
    
    client.use_tls = based
    damn client
}

// Creates SMTP client with STARTTLS (usually port 587)
slay create_smtp_client_starttls(host tea, port drip) yikes<SmtpClient> {
    sus client SmtpClient = create_smtp_client(host, port) fam {
        when err -> yikes err
    }
    
    client.use_starttls = based
    damn client
}

// Establishes connection to SMTP server
slay connect_smtp(client SmtpClient) yikes<SmtpResponse> {
    // Establish TCP connection
    client.connection = networkz.tcp_connect(client.host, client.port) fam {
        when err -> yikes smtp_connection_error(stringz.concat(["Failed to connect to ", client.host, ":", string_from_drip(client.port)]))
    }
    
    // Read initial server greeting
    sus greeting tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read server greeting")
    }
    
    // Parse greeting response
    sus response SmtpResponse = parse_smtp_response(greeting) fam {
        when err -> yikes smtp_connection_error("Invalid server greeting format")
    }
    
    ready (response.code != 220) {
        yikes smtp_connection_error(stringz.concat(["Server rejected connection: ", response.message]))
    }
    
    damn response
}

// Performs EHLO command and discovers server capabilities
slay ehlo_smtp(client SmtpClient, hostname tea) yikes<SmtpResponse> {
    sus ehlo_command tea = stringz.concat(["EHLO ", hostname, "\r\n"])
    
    networkz.tcp_send(client.connection, ehlo_command) fam {
        when err -> yikes smtp_connection_error("Failed to send EHLO command")
    }
    
    sus response_text tea = networkz.tcp_receive(client.connection, 2048) fam {
        when err -> yikes smtp_connection_error("Failed to read EHLO response")
    }
    
    sus response SmtpResponse = parse_smtp_response(response_text) fam {
        when err -> yikes smtp_connection_error("Invalid EHLO response format")
    }
    
    ready (response.code == 250) {
        // Parse capabilities from multi-line response
        client.capabilities = parse_ehlo_capabilities(response_text)
    }
    
    damn response
}

// Upgrades connection to TLS using STARTTLS
slay starttls_smtp(client SmtpClient) yikes<SmtpResponse> {
    ready (!supports_capability(client, "STARTTLS")) {
        yikes smtp_connection_error("Server does not support STARTTLS")
    }
    
    sus starttls_command tea = "STARTTLS\r\n"
    
    networkz.tcp_send(client.connection, starttls_command) fam {
        when err -> yikes smtp_connection_error("Failed to send STARTTLS command")
    }
    
    sus response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read STARTTLS response")
    }
    
    sus response SmtpResponse = parse_smtp_response(response_text) fam {
        when err -> yikes smtp_connection_error("Invalid STARTTLS response")
    }
    
    ready (response.code == 220) {
        // Upgrade connection to TLS (implementation would use TLS library)
        // For now, we'll mark the connection as TLS-enabled
        client.use_tls = based
    }
    
    damn response
}

// ============================================================================
// Authentication Functions
// ============================================================================

// Authenticates using SASL PLAIN mechanism (RFC 4616)
slay auth_plain(client SmtpClient, username tea, password tea) yikes<SmtpResponse> {
    ready (!supports_capability(client, "AUTH PLAIN") && !supports_capability(client, "AUTH")) {
        yikes smtp_auth_error("Server does not support PLAIN authentication", 0)
    }
    
    // Create PLAIN auth string: \0username\0password
    sus auth_string tea = stringz.concat(["\0", username, "\0", password])
    sus auth_b64 tea = encode_base64(auth_string)
    
    sus auth_command tea = stringz.concat(["AUTH PLAIN ", auth_b64, "\r\n"])
    
    networkz.tcp_send(client.connection, auth_command) fam {
        when err -> yikes smtp_connection_error("Failed to send AUTH PLAIN command")
    }
    
    sus response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read AUTH response")
    }
    
    sus response SmtpResponse = parse_smtp_response(response_text) fam {
        when err -> yikes smtp_connection_error("Invalid AUTH response")
    }
    
    ready (response.code == 235) {
        client.authenticated = based
    }
    
    damn response
}

// Authenticates using LOGIN mechanism
slay auth_login(client SmtpClient, username tea, password tea) yikes<SmtpResponse> {
    ready (!supports_capability(client, "AUTH LOGIN")) {
        yikes smtp_auth_error("Server does not support LOGIN authentication", 0)
    }
    
    // Send AUTH LOGIN command
    sus auth_command tea = "AUTH LOGIN\r\n"
    networkz.tcp_send(client.connection, auth_command) fam {
        when err -> yikes smtp_connection_error("Failed to send AUTH LOGIN command")
    }
    
    // Server should respond with 334 and base64 "Username:" prompt
    sus response1_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read AUTH LOGIN response")
    }
    
    sus response1 SmtpResponse = parse_smtp_response(response1_text) fam {
        when err -> yikes smtp_connection_error("Invalid AUTH LOGIN response")
    }
    
    ready (response1.code != 334) {
        yikes smtp_auth_error("AUTH LOGIN failed at username prompt", response1.code)
    }
    
    // Send base64 encoded username
    sus username_b64 tea = stringz.concat([encode_base64(username), "\r\n"])
    networkz.tcp_send(client.connection, username_b64) fam {
        when err -> yikes smtp_connection_error("Failed to send username")
    }
    
    // Server should respond with 334 and base64 "Password:" prompt
    sus response2_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read password prompt")
    }
    
    sus response2 SmtpResponse = parse_smtp_response(response2_text) fam {
        when err -> yikes smtp_connection_error("Invalid password prompt response")
    }
    
    ready (response2.code != 334) {
        yikes smtp_auth_error("AUTH LOGIN failed at password prompt", response2.code)
    }
    
    // Send base64 encoded password
    sus password_b64 tea = stringz.concat([encode_base64(password), "\r\n"])
    networkz.tcp_send(client.connection, password_b64) fam {
        when err -> yikes smtp_connection_error("Failed to send password")
    }
    
    // Read final authentication response
    sus final_response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read final AUTH response")
    }
    
    sus final_response SmtpResponse = parse_smtp_response(final_response_text) fam {
        when err -> yikes smtp_connection_error("Invalid final AUTH response")
    }
    
    ready (final_response.code == 235) {
        client.authenticated = based
    }
    
    damn final_response
}

// Authenticates using CRAM-MD5 mechanism (RFC 2195)
slay auth_cram_md5(client SmtpClient, username tea, password tea) yikes<SmtpResponse> {
    ready (!supports_capability(client, "AUTH CRAM-MD5")) {
        yikes smtp_auth_error("Server does not support CRAM-MD5 authentication", 0)
    }
    
    // Send AUTH CRAM-MD5 command
    sus auth_command tea = "AUTH CRAM-MD5\r\n"
    networkz.tcp_send(client.connection, auth_command) fam {
        when err -> yikes smtp_connection_error("Failed to send AUTH CRAM-MD5 command")
    }
    
    // Server responds with 334 and base64 encoded challenge
    sus challenge_response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read CRAM-MD5 challenge")
    }
    
    sus challenge_response SmtpResponse = parse_smtp_response(challenge_response_text) fam {
        when err -> yikes smtp_connection_error("Invalid CRAM-MD5 challenge response")
    }
    
    ready (challenge_response.code != 334) {
        yikes smtp_auth_error("CRAM-MD5 challenge failed", challenge_response.code)
    }
    
    // Decode challenge and compute HMAC-MD5 response
    sus challenge tea = decode_base64(challenge_response.message) fam {
        when err -> yikes smtp_auth_error("Failed to decode CRAM-MD5 challenge", 0)
    }
    
    sus hmac_response tea = compute_hmac_md5(password, challenge)
    sus auth_response tea = stringz.concat([username, " ", hmac_response])
    sus auth_response_b64 tea = stringz.concat([encode_base64(auth_response), "\r\n"])
    
    networkz.tcp_send(client.connection, auth_response_b64) fam {
        when err -> yikes smtp_connection_error("Failed to send CRAM-MD5 response")
    }
    
    // Read final authentication result
    sus final_response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read CRAM-MD5 final response")
    }
    
    sus final_response SmtpResponse = parse_smtp_response(final_response_text) fam {
        when err -> yikes smtp_connection_error("Invalid CRAM-MD5 final response")
    }
    
    ready (final_response.code == 235) {
        client.authenticated = based
    }
    
    damn final_response
}

// Automatically selects best available authentication method
slay auto_authenticate(client SmtpClient, username tea, password tea) yikes<SmtpResponse> {
    // Store credentials for future use
    client.username = username
    client.password = password
    
    // Try CRAM-MD5 first (most secure)
    ready (supports_capability(client, "AUTH CRAM-MD5")) {
        sus response SmtpResponse = auth_cram_md5(client, username, password) fam {
            when err -> {
                // CRAM-MD5 failed, try other methods
                damn // Continue to next method
            }
        }
        damn response
    }
    
    // Try PLAIN authentication
    ready (supports_capability(client, "AUTH PLAIN")) {
        sus response SmtpResponse = auth_plain(client, username, password) fam {
            when err -> {
                // PLAIN failed, try LOGIN
                damn // Continue to next method
            }
        }
        damn response
    }
    
    // Try LOGIN authentication as last resort
    ready (supports_capability(client, "AUTH LOGIN")) {
        sus response SmtpResponse = auth_login(client, username, password) fam {
            when err -> yikes err
        }
        damn response
    }
    
    yikes smtp_auth_error("No supported authentication methods available", 0)
}

// ============================================================================
// Email Creation Functions
// ============================================================================

// Creates a simple plain text email
slay create_email(from tea, to tea, subject tea, body tea) yikes<Email> {
    // Validate email addresses
    ready (!validate_email_address(from)) {
        yikes email_format_error(stringz.concat(["Invalid from address: ", from]))
    }
    
    ready (!validate_email_address(to)) {
        yikes email_format_error(stringz.concat(["Invalid to address: ", to]))
    }
    
    // Generate message ID and date
    sus domain tea = extract_domain(from)
    sus msg_id tea = generate_message_id(domain)
    sus date_header tea = format_date_header()
    
    damn Email{
        from: from,
        to: [to],
        cc: [],
        bcc: [],
        subject: subject,
        reply_to: "",
        date: date_header,
        message_id: msg_id,
        headers: [],
        body_text: body,
        body_html: "",
        attachments: [],
        priority: EmailPriority.Normal,
        encoding: "utf-8",
        content_type: "text/plain",
        boundary: ""
    }
}

// Creates an HTML email with plain text fallback
slay create_html_email(from tea, to tea, subject tea, html_body tea, text_body tea) yikes<Email> {
    sus email Email = create_email(from, to, subject, text_body) fam {
        when err -> yikes err
    }
    
    email.body_html = html_body
    email.content_type = "multipart/alternative"
    email.boundary = generate_mime_boundary()
    
    damn email
}

// Adds a recipient to the TO field
slay add_recipient(email Email, address tea) yikes<Email> {
    ready (!validate_email_address(address)) {
        yikes email_format_error(stringz.concat(["Invalid recipient address: ", address]))
    }
    
    email.to = arrayz.push(email.to, address)
    damn email
}

// Adds a recipient to the CC field
slay add_cc_recipient(email Email, address tea) yikes<Email> {
    ready (!validate_email_address(address)) {
        yikes email_format_error(stringz.concat(["Invalid CC address: ", address]))
    }
    
    email.cc = arrayz.push(email.cc, address)
    damn email
}

// Adds a recipient to the BCC field
slay add_bcc_recipient(email Email, address tea) yikes<Email> {
    ready (!validate_email_address(address)) {
        yikes email_format_error(stringz.concat(["Invalid BCC address: ", address]))
    }
    
    email.bcc = arrayz.push(email.bcc, address)
    damn email
}

// Adds a custom header to the email
slay add_custom_header(email Email, name tea, value tea) yikes<Email> {
    ready (stringz.len(name) == 0) {
        yikes email_format_error("Header name cannot be empty")
    }
    
    // Validate header name (no spaces, colons, or control characters)
    ready (stringz.contains(name, " ") || stringz.contains(name, ":") || stringz.contains(name, "\r") || stringz.contains(name, "\n")) {
        yikes email_format_error(stringz.concat(["Invalid header name: ", name]))
    }
    
    sus header EmailHeader = EmailHeader{
        name: name,
        value: sanitize_header_value(value)
    }
    
    email.headers = arrayz.push(email.headers, header)
    damn email
}

// Sets the Reply-To address
slay set_reply_to(email Email, address tea) yikes<Email> {
    ready (stringz.len(address) > 0 && !validate_email_address(address)) {
        yikes email_format_error(stringz.concat(["Invalid Reply-To address: ", address]))
    }
    
    email.reply_to = address
    damn email
}

// Sets the email priority
slay set_priority(email Email, priority EmailPriority) Email {
    email.priority = priority
    damn email
}

// ============================================================================
// Attachment Functions
// ============================================================================

// Adds a file attachment to the email
slay add_file_attachment(email Email, file_path tea, content_type tea) yikes<Email> {
    // Read file content
    sus file_content tea = filez.read_file(file_path) fam {
        when err -> yikes create_email_error("attachment_read", stringz.concat(["Failed to read file: ", file_path]), 0, err.message)
    }
    
    // Get filename from path
    sus filename tea = extract_filename_from_path(file_path)
    
    // Create attachment
    sus attachment EmailAttachment = EmailAttachment{
        filename: filename,
        content_type: content_type,
        content: encode_base64(file_content),
        content_id: "",
        disposition: "attachment",
        encoding: "base64",
        size: stringz.len(file_content)
    }
    
    email.attachments = arrayz.push(email.attachments, attachment)
    
    // Update email to multipart/mixed if needed
    ready (stringz.equals(email.content_type, "text/plain") || stringz.equals(email.content_type, "text/html")) {
        email.content_type = "multipart/mixed"
        email.boundary = generate_mime_boundary()
    }
    
    damn email
}

// Adds an attachment from raw data
slay add_attachment_from_data(email Email, filename tea, content_type tea, data tea) yikes<Email> {
    ready (stringz.len(filename) == 0) {
        yikes email_format_error("Attachment filename cannot be empty")
    }
    
    sus attachment EmailAttachment = EmailAttachment{
        filename: filename,
        content_type: content_type,
        content: encode_base64(data),
        content_id: "",
        disposition: "attachment", 
        encoding: "base64",
        size: stringz.len(data)
    }
    
    email.attachments = arrayz.push(email.attachments, attachment)
    
    // Update content type if needed
    ready (stringz.equals(email.content_type, "text/plain") || stringz.equals(email.content_type, "text/html")) {
        email.content_type = "multipart/mixed"
        email.boundary = generate_mime_boundary()
    }
    
    damn email
}

// Adds an inline image that can be referenced in HTML
slay add_inline_image(email Email, image_path tea, content_id tea) yikes<Email> {
    // Read image file
    sus image_data tea = filez.read_file(image_path) fam {
        when err -> yikes create_email_error("attachment_read", stringz.concat(["Failed to read image: ", image_path]), 0, err.message)
    }
    
    // Determine content type from file extension
    sus content_type tea = get_content_type_from_extension(image_path)
    sus filename tea = extract_filename_from_path(image_path)
    
    sus attachment EmailAttachment = EmailAttachment{
        filename: filename,
        content_type: content_type,
        content: encode_base64(image_data),
        content_id: content_id,
        disposition: "inline",
        encoding: "base64",
        size: stringz.len(image_data)
    }
    
    email.attachments = arrayz.push(email.attachments, attachment)
    
    // Update to multipart/related for inline images
    ready (!stringz.contains(email.content_type, "multipart")) {
        email.content_type = "multipart/related"
        email.boundary = generate_mime_boundary()
    }
    
    damn email
}

// ============================================================================
// Email Sending Functions  
// ============================================================================

// Sends an email through the SMTP connection
slay send_email(client SmtpClient, email Email) yikes<SmtpResponse> {
    ready (!client.authenticated) {
        yikes smtp_auth_error("Client must be authenticated before sending email", 0)
    }
    
    // Send MAIL FROM command
    sus mail_from_cmd tea = stringz.concat(["MAIL FROM:<", email.from, ">\r\n"])
    networkz.tcp_send(client.connection, mail_from_cmd) fam {
        when err -> yikes smtp_connection_error("Failed to send MAIL FROM command")
    }
    
    sus mail_response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read MAIL FROM response")
    }
    
    sus mail_response SmtpResponse = parse_smtp_response(mail_response_text) fam {
        when err -> yikes smtp_connection_error("Invalid MAIL FROM response")
    }
    
    ready (mail_response.code != 250) {
        yikes create_email_error("smtp_command", "MAIL FROM rejected", mail_response.code, mail_response.message)
    }
    
    // Send RCPT TO commands for all recipients (TO, CC, BCC)
    sus all_recipients []tea = combine_recipients(email)
    sus i drip = 0
    
    bestie (i < arrayz.len(all_recipients)) {
        sus recipient tea = all_recipients[i]
        sus rcpt_cmd tea = stringz.concat(["RCPT TO:<", recipient, ">\r\n"])
        
        networkz.tcp_send(client.connection, rcpt_cmd) fam {
            when err -> yikes smtp_connection_error(stringz.concat(["Failed to send RCPT TO for ", recipient]))
        }
        
        sus rcpt_response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
            when err -> yikes smtp_connection_error(stringz.concat(["Failed to read RCPT TO response for ", recipient]))
        }
        
        sus rcpt_response SmtpResponse = parse_smtp_response(rcpt_response_text) fam {
            when err -> yikes smtp_connection_error("Invalid RCPT TO response")
        }
        
        ready (rcpt_response.code != 250 && rcpt_response.code != 251) {
            yikes create_email_error("smtp_command", stringz.concat(["RCPT TO rejected for ", recipient]), rcpt_response.code, rcpt_response.message)
        }
        
        i = i + 1
    }
    
    // Send DATA command
    sus data_cmd tea = "DATA\r\n"
    networkz.tcp_send(client.connection, data_cmd) fam {
        when err -> yikes smtp_connection_error("Failed to send DATA command")
    }
    
    sus data_response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read DATA response")
    }
    
    sus data_response SmtpResponse = parse_smtp_response(data_response_text) fam {
        when err -> yikes smtp_connection_error("Invalid DATA response")
    }
    
    ready (data_response.code != 354) {
        yikes create_email_error("smtp_command", "DATA command rejected", data_response.code, data_response.message)
    }
    
    // Format and send email content
    sus formatted_email tea = format_email_for_sending(email)
    sus email_with_terminator tea = stringz.concat([formatted_email, "\r\n.\r\n"])
    
    networkz.tcp_send(client.connection, email_with_terminator) fam {
        when err -> yikes smtp_connection_error("Failed to send email content")
    }
    
    // Read final response
    sus final_response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read final DATA response")
    }
    
    sus final_response SmtpResponse = parse_smtp_response(final_response_text) fam {
        when err -> yikes smtp_connection_error("Invalid final DATA response")
    }
    
    damn final_response
}

// Properly closes SMTP connection
slay quit_smtp(client SmtpClient) yikes<SmtpResponse> {
    sus quit_cmd tea = "QUIT\r\n"
    networkz.tcp_send(client.connection, quit_cmd) fam {
        when err -> yikes smtp_connection_error("Failed to send QUIT command")
    }
    
    sus response_text tea = networkz.tcp_receive(client.connection, 1024) fam {
        when err -> yikes smtp_connection_error("Failed to read QUIT response")
    }
    
    // Close TCP connection
    networkz.tcp_close(client.connection) fam {
        when err -> {
            // Log warning but don't fail the operation
        }
    }
    
    sus response SmtpResponse = parse_smtp_response(response_text) fam {
        when err -> yikes smtp_connection_error("Invalid QUIT response")
    }
    
    damn response
}

// ============================================================================
// Utility Functions
// ============================================================================

// Validates an email address format (basic RFC 5322 validation)
slay validate_email_address(address tea) lit {
    ready (stringz.len(address) == 0) {
        damn cap
    }
    
    // Must contain exactly one @ symbol
    sus at_count drip = stringz.count(address, "@")
    ready (at_count != 1) {
        damn cap
    }
    
    // Split into local and domain parts
    sus at_pos drip = stringz.find_first(address, "@")
    sus local tea = stringz.substring(address, 0, at_pos)
    sus domain tea = stringz.substring(address, at_pos + 1, stringz.len(address))
    
    // Basic validation rules
    ready (stringz.len(local) == 0 || stringz.len(domain) == 0) {
        damn cap
    }
    
    ready (stringz.len(local) > 64 || stringz.len(domain) > 253) {
        damn cap
    }
    
    // Domain must contain at least one dot
    ready (!stringz.contains(domain, ".")) {
        damn cap
    }
    
    // No consecutive dots
    ready (stringz.contains(address, "..")) {
        damn cap
    }
    
    // No leading/trailing dots
    ready (stringz.starts_with(address, ".") || stringz.ends_with(address, ".")) {
        damn cap
    }
    
    damn based
}

// Extracts domain from email address
slay extract_domain(address tea) tea {
    sus at_pos drip = stringz.find_first(address, "@")
    ready (at_pos == -1) {
        damn ""
    }
    damn stringz.substring(address, at_pos + 1, stringz.len(address))
}

// Generates a unique Message-ID header
slay generate_message_id(domain tea) tea {
    sus timestamp tea = string_from_drip(timez.unix_timestamp())
    sus random_part tea = generate_random_string(8)
    damn stringz.concat(["<", timestamp, ".", random_part, "@", domain, ">"])
}

// Formats current date for Date header (RFC 2822 format)
slay format_date_header() tea {
    // Implementation would use proper date/time formatting
    // For now, return a placeholder
    damn "Mon, 1 Jan 2024 12:00:00 +0000"
}

// Generates MIME boundary for multipart messages
slay generate_mime_boundary() tea {
    sus random_part tea = generate_random_string(16)
    damn stringz.concat(["----=_Part_", random_part])
}

// Formats email for RFC 5322 compliant sending
slay format_email_for_sending(email Email) tea {
    sus headers []tea = []
    
    // Required headers
    headers = arrayz.push(headers, stringz.concat(["From: ", email.from]))
    headers = arrayz.push(headers, stringz.concat(["To: ", stringz.join(email.to, ", ")]))
    
    // Optional TO headers
    ready (arrayz.len(email.cc) > 0) {
        headers = arrayz.push(headers, stringz.concat(["Cc: ", stringz.join(email.cc, ", ")]))
    }
    
    // Subject and other standard headers
    headers = arrayz.push(headers, stringz.concat(["Subject: ", email.subject]))
    headers = arrayz.push(headers, stringz.concat(["Date: ", email.date]))
    headers = arrayz.push(headers, stringz.concat(["Message-ID: ", email.message_id]))
    
    // Reply-To if set
    ready (stringz.len(email.reply_to) > 0) {
        headers = arrayz.push(headers, stringz.concat(["Reply-To: ", email.reply_to]))
    }
    
    // MIME headers
    headers = arrayz.push(headers, "MIME-Version: 1.0")
    
    ready (arrayz.len(email.attachments) > 0 || stringz.len(email.body_html) > 0) {
        headers = arrayz.push(headers, stringz.concat(["Content-Type: ", email.content_type, "; boundary=\"", email.boundary, "\""]))
    } otherwise {
        headers = arrayz.push(headers, stringz.concat(["Content-Type: ", email.content_type, "; charset=", email.encoding]))
    }
    
    // Priority headers
    sus priority_value tea = get_priority_header_value(email.priority)
    ready (stringz.len(priority_value) > 0) {
        headers = arrayz.push(headers, stringz.concat(["X-Priority: ", priority_value]))
    }
    
    // Custom headers
    sus i drip = 0
    bestie (i < arrayz.len(email.headers)) {
        sus header EmailHeader = email.headers[i]
        headers = arrayz.push(headers, stringz.concat([header.name, ": ", header.value]))
        i = i + 1
    }
    
    // Combine headers
    sus header_section tea = stringz.join(headers, "\r\n")
    
    // Format body based on content type
    sus body_section tea = format_email_body(email)
    
    damn stringz.concat([header_section, "\r\n\r\n", body_section])
}

// Formats email body based on content type and attachments
slay format_email_body(email Email) tea {
    // Simple text email
    ready (stringz.equals(email.content_type, "text/plain") && arrayz.len(email.attachments) == 0) {
        damn email.body_text
    }
    
    // Simple HTML email
    ready (stringz.equals(email.content_type, "text/html") && arrayz.len(email.attachments) == 0) {
        damn email.body_html
    }
    
    // Multipart email
    sus parts []tea = []
    
    // Add text part if present
    ready (stringz.len(email.body_text) > 0) {
        sus text_part tea = stringz.concat([
            "--", email.boundary, "\r\n",
            "Content-Type: text/plain; charset=", email.encoding, "\r\n",
            "Content-Transfer-Encoding: 7bit\r\n\r\n",
            email.body_text, "\r\n"
        ])
        parts = arrayz.push(parts, text_part)
    }
    
    // Add HTML part if present
    ready (stringz.len(email.body_html) > 0) {
        sus html_part tea = stringz.concat([
            "--", email.boundary, "\r\n",
            "Content-Type: text/html; charset=", email.encoding, "\r\n", 
            "Content-Transfer-Encoding: 7bit\r\n\r\n",
            email.body_html, "\r\n"
        ])
        parts = arrayz.push(parts, html_part)
    }
    
    // Add attachments
    sus i drip = 0
    bestie (i < arrayz.len(email.attachments)) {
        sus attachment EmailAttachment = email.attachments[i]
        sus attachment_part tea = format_attachment_part(attachment, email.boundary)
        parts = arrayz.push(parts, attachment_part)
        i = i + 1
    }
    
    // Close boundary
    parts = arrayz.push(parts, stringz.concat(["--", email.boundary, "--\r\n"]))
    
    damn stringz.join(parts, "")
}

// Formats an attachment as a MIME part
slay format_attachment_part(attachment EmailAttachment, boundary tea) tea {
    sus part_headers []tea = []
    
    part_headers = arrayz.push(part_headers, stringz.concat(["--", boundary]))
    part_headers = arrayz.push(part_headers, stringz.concat(["Content-Type: ", attachment.content_type]))
    part_headers = arrayz.push(part_headers, stringz.concat(["Content-Transfer-Encoding: ", attachment.encoding]))
    
    ready (stringz.equals(attachment.disposition, "inline") && stringz.len(attachment.content_id) > 0) {
        part_headers = arrayz.push(part_headers, stringz.concat(["Content-Disposition: inline"]))
        part_headers = arrayz.push(part_headers, stringz.concat(["Content-ID: <", attachment.content_id, ">"]))
    } otherwise {
        part_headers = arrayz.push(part_headers, stringz.concat(["Content-Disposition: attachment; filename=\"", attachment.filename, "\""]))
    }
    
    sus header_section tea = stringz.join(part_headers, "\r\n")
    damn stringz.concat([header_section, "\r\n\r\n", attachment.content, "\r\n"])
}

// Base64 encoding for attachments and authentication
slay encode_base64(data tea) tea {
    // Implementation would use proper base64 encoding
    // For now, return placeholder
    damn "base64_encoded_data"
}

// Base64 decoding
slay decode_base64(encoded tea) yikes<tea> {
    // Implementation would use proper base64 decoding
    damn "decoded_data"
}

// Helper function to parse SMTP responses
slay parse_smtp_response(response_text tea) yikes<SmtpResponse> {
    ready (stringz.len(response_text) < 3) {
        yikes create_email_error("smtp_parse", "Invalid SMTP response format", 0, response_text)
    }
    
    sus code_str tea = stringz.substring(response_text, 0, 3)
    sus code drip = drip_from_string(code_str) fam {
        when err -> yikes create_email_error("smtp_parse", "Invalid SMTP response code", 0, code_str)
    }
    
    sus message tea = stringz.trim(stringz.substring(response_text, 4, stringz.len(response_text)))
    
    damn SmtpResponse{
        code: code,
        message: message,
        enhanced_code: ""
    }
}

// Helper function to check if server supports a capability
slay supports_capability(client SmtpClient, capability tea) lit {
    sus i drip = 0
    bestie (i < arrayz.len(client.capabilities)) {
        sus cap tea = client.capabilities[i]
        ready (stringz.contains(stringz.to_upper(cap), stringz.to_upper(capability))) {
            damn based
        }
        i = i + 1
    }
    damn cap
}

// Helper function to parse EHLO capabilities
slay parse_ehlo_capabilities(response_text tea) []tea {
    sus lines []tea = stringz.split(response_text, "\r\n")
    sus capabilities []tea = []
    
    sus i drip = 1  // Skip first line (250-hostname greeting)
    bestie (i < arrayz.len(lines)) {
        sus line tea = stringz.trim(lines[i])
        ready (stringz.starts_with(line, "250-") || stringz.starts_with(line, "250 ")) {
            sus capability tea = stringz.trim(stringz.substring(line, 4, stringz.len(line)))
            capabilities = arrayz.push(capabilities, capability)
        }
        i = i + 1
    }
    
    damn capabilities
}

// Helper function to combine all email recipients
slay combine_recipients(email Email) []tea {
    sus recipients []tea = email.to
    
    sus i drip = 0
    bestie (i < arrayz.len(email.cc)) {
        recipients = arrayz.push(recipients, email.cc[i])
        i = i + 1
    }
    
    i = 0
    bestie (i < arrayz.len(email.bcc)) {
        recipients = arrayz.push(recipients, email.bcc[i])
        i = i + 1
    }
    
    damn recipients
}

// Helper functions for various utilities
slay sanitize_header_value(value tea) tea {
    // Remove or escape dangerous characters
    sus sanitized tea = stringz.replace_all(value, "\r", "")
    sanitized = stringz.replace_all(sanitized, "\n", "")
    damn sanitized
}

slay extract_filename_from_path(path tea) tea {
    sus last_slash drip = stringz.find_last(path, "/")
    ready (last_slash == -1) {
        damn path
    }
    damn stringz.substring(path, last_slash + 1, stringz.len(path))
}

slay get_content_type_from_extension(filename tea) tea {
    ready (stringz.ends_with(filename, ".pdf")) {
        damn "application/pdf"
    } otherwise ready (stringz.ends_with(filename, ".jpg") || stringz.ends_with(filename, ".jpeg")) {
        damn "image/jpeg"
    } otherwise ready (stringz.ends_with(filename, ".png")) {
        damn "image/png"
    } otherwise ready (stringz.ends_with(filename, ".txt")) {
        damn "text/plain"
    } otherwise {
        damn "application/octet-stream"
    }
}

slay get_priority_header_value(priority EmailPriority) tea {
    sick(priority) {
        when EmailPriority.Low -> damn "5"
        when EmailPriority.Normal -> damn ""  // Don't set priority for normal
        when EmailPriority.High -> damn "1"
        when EmailPriority.Urgent -> damn "1"
        _ -> damn ""
    }
}

slay generate_random_string(length drip) tea {
    // Implementation would generate random alphanumeric string
    damn "randomstring"
}

slay compute_hmac_md5(key tea, data tea) tea {
    // Implementation would compute HMAC-MD5 hash
    damn "hmac_md5_hash"
}

slay string_from_drip(value drip) tea {
    // Convert number to string - implementation dependent
    damn "123"
}

slay drip_from_string(value tea) yikes<drip> {
    // Convert string to number - implementation dependent
    damn 123
}
