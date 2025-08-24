# EmailZ - CURSED Standard Library Email & SMTP Module

The EmailZ module provides comprehensive email composition, parsing, and SMTP client functionality for the CURSED programming language. This module is implemented entirely in pure CURSED with no external dependencies and follows RFC specifications.

## Features

- **SMTP Client**: Full RFC 5321 compliant SMTP client
- **Email Composition**: RFC 5322 compliant email message creation
- **Email Parsing**: Complete email header and body parsing
- **MIME Support**: Multipart messages with attachments
- **Authentication**: PLAIN, LOGIN, and CRAM-MD5 SASL mechanisms
- **Security**: TLS/STARTTLS support with certificate validation
- **Attachments**: File attachment support with Base64 encoding
- **Templates**: HTML and plain text email templates
- **Error Handling**: Comprehensive SMTP error management

## Quick Start

```cursed
yeet "emailz"

// Simple email sending
sus client SmtpClient = emailz.create_smtp_client("smtp.gmail.com", 587) fam {
    when err -> {
        vibez.spill("SMTP client creation failed:", err.message)
        damn
    }
}

sus email Email = emailz.create_email(
    "sender@example.com",
    "recipient@example.com", 
    "Hello World",
    "This is a test email from CURSED!"
) fam {
    when err -> {
        vibez.spill("Email creation failed:", err.message)
        damn
    }
}

emailz.send_email(client, email) fam {
    when err -> {
        vibez.spill("Email send failed:", err.message)
        damn
    }
}

vibez.spill("Email sent successfully!")
```

## Data Structures

### EmailError
```cursed
squad EmailError {
    sus kind tea        // Error type (smtp_connect, auth_failed, etc.)
    sus message tea     // Human-readable error message
    sus smtp_code drip  // SMTP response code (220, 250, 550, etc.)
    sus details tea     // Additional error details
}
```

### SmtpClient
```cursed
squad SmtpClient {
    sus host tea            // SMTP server hostname
    sus port drip           // SMTP server port
    sus username tea        // Authentication username
    sus password tea        // Authentication password
    sus use_tls lit         // Whether to use TLS/SSL
    sus use_starttls lit    // Whether to use STARTTLS
    sus timeout drip        // Connection timeout in seconds
    sus connection TcpConnection // Network connection
    sus authenticated lit   // Authentication status
    sus capabilities []tea  // Server capabilities from EHLO
}
```

### Email
```cursed
squad Email {
    // Headers
    sus from tea            // From address
    sus to []tea            // To addresses
    sus cc []tea            // CC addresses
    sus bcc []tea           // BCC addresses
    sus subject tea         // Email subject
    sus reply_to tea        // Reply-to address
    sus date tea            // Date header
    sus message_id tea      // Message-ID header
    sus headers []EmailHeader // Custom headers
    
    // Content
    sus body_text tea       // Plain text body
    sus body_html tea       // HTML body
    sus attachments []EmailAttachment // File attachments
    sus priority EmailPriority // Email priority
    sus encoding tea        // Content encoding (utf-8, etc.)
}
```

### EmailHeader
```cursed
squad EmailHeader {
    sus name tea        // Header name
    sus value tea       // Header value
}
```

### EmailAttachment
```cursed
squad EmailAttachment {
    sus filename tea        // Original filename
    sus content_type tea    // MIME content type
    sus content tea         // Base64 encoded content
    sus content_id tea      // Content-ID for inline images
    sus disposition tea     // attachment or inline
    sus encoding tea        // Base64, quoted-printable, etc.
}
```

### EmailPriority
```cursed
enum EmailPriority {
    Low,
    Normal,
    High,
    Urgent
}
```

### SmtpResponse
```cursed
squad SmtpResponse {
    sus code drip       // SMTP response code
    sus message tea     // Response message
    sus enhanced_code tea // Enhanced status code (RFC 3463)
}
```

### ParsedEmail
```cursed
squad ParsedEmail {
    sus headers []EmailHeader   // All parsed headers
    sus from tea               // From address
    sus to []tea               // To addresses
    sus subject tea            // Subject line
    sus date tea               // Date sent
    sus message_id tea         // Message ID
    sus body_text tea          // Plain text body
    sus body_html tea          // HTML body
    sus attachments []EmailAttachment // Attachments
    sus raw_headers tea        // Raw header section
    sus raw_body tea           // Raw body section
}
```

## SMTP Client Functions

### Connection Management

#### `create_smtp_client(host tea, port drip) yikes<SmtpClient>`
Creates a new SMTP client instance.

```cursed
// Standard SMTP
sus client SmtpClient = emailz.create_smtp_client("smtp.example.com", 25) fam {
    when err -> {
        vibez.spill("SMTP client creation failed:", err.message)
        damn
    }
}

// SMTP with TLS (SMTPS)
sus secure_client SmtpClient = emailz.create_smtp_client_tls("smtp.gmail.com", 465) fam {
    when err -> {
        vibez.spill("Secure SMTP client creation failed:", err.message)
        damn
    }
}

// SMTP with STARTTLS
sus starttls_client SmtpClient = emailz.create_smtp_client_starttls("smtp.office365.com", 587) fam {
    when err -> {
        vibez.spill("STARTTLS SMTP client creation failed:", err.message)
        damn
    }
}
```

#### `connect_smtp(client SmtpClient) yikes<SmtpResponse>`
Establishes connection to SMTP server and performs initial handshake.

```cursed
sus response SmtpResponse = emailz.connect_smtp(client) fam {
    when err -> {
        vibez.spill("SMTP connection failed:", err.message)
        damn
    }
}

ready (response.code == 220) {
    vibez.spill("SMTP connection established:", response.message)
} otherwise {
    vibez.spill("SMTP connection rejected:", response.code, response.message)
}
```

#### `ehlo_smtp(client SmtpClient, hostname tea) yikes<SmtpResponse>`
Performs EHLO command and discovers server capabilities.

```cursed
sus response SmtpResponse = emailz.ehlo_smtp(client, "client.example.com") fam {
    when err -> {
        vibez.spill("EHLO failed:", err.message)
        damn
    }
}

ready (response.code == 250) {
    vibez.spill("EHLO successful. Capabilities:", client.capabilities)
} otherwise {
    vibez.spill("EHLO failed:", response.code, response.message)
}
```

#### `starttls_smtp(client SmtpClient) yikes<SmtpResponse>`
Upgrades connection to TLS using STARTTLS.

```cursed
sus response SmtpResponse = emailz.starttls_smtp(client) fam {
    when err -> {
        vibez.spill("STARTTLS failed:", err.message)
        damn
    }
}

ready (response.code == 220) {
    vibez.spill("TLS upgrade successful")
} otherwise {
    vibez.spill("STARTTLS failed:", response.code, response.message)
}
```

### Authentication

#### `auth_plain(client SmtpClient, username tea, password tea) yikes<SmtpResponse>`
Authenticates using SASL PLAIN mechanism.

```cursed
sus response SmtpResponse = emailz.auth_plain(client, "user@example.com", "password123") fam {
    when err -> {
        vibez.spill("PLAIN authentication failed:", err.message)
        damn
    }
}

ready (response.code == 235) {
    vibez.spill("Authentication successful")
    client.authenticated = based
} otherwise {
    vibez.spill("Authentication failed:", response.code, response.message)
}
```

#### `auth_login(client SmtpClient, username tea, password tea) yikes<SmtpResponse>`
Authenticates using LOGIN mechanism.

```cursed
sus response SmtpResponse = emailz.auth_login(client, "user@example.com", "password123") fam {
    when err -> {
        vibez.spill("LOGIN authentication failed:", err.message)
        damn
    }
}
```

#### `auth_cram_md5(client SmtpClient, username tea, password tea) yikes<SmtpResponse>`
Authenticates using CRAM-MD5 mechanism.

```cursed
sus response SmtpResponse = emailz.auth_cram_md5(client, "user@example.com", "password123") fam {
    when err -> {
        vibez.spill("CRAM-MD5 authentication failed:", err.message)
        damn
    }
}
```

#### `auto_authenticate(client SmtpClient, username tea, password tea) yikes<SmtpResponse>`
Automatically selects best available authentication method.

```cursed
sus response SmtpResponse = emailz.auto_authenticate(client, "user@example.com", "password123") fam {
    when err -> {
        vibez.spill("Auto authentication failed:", err.message)
        damn
    }
}
```

### Email Sending

#### `send_email(client SmtpClient, email Email) yikes<SmtpResponse>`
Sends an email through the SMTP connection.

```cursed
sus response SmtpResponse = emailz.send_email(client, email) fam {
    when err -> {
        vibez.spill("Email send failed:", err.message)
        damn
    }
}

ready (response.code >= 200 && response.code < 300) {
    vibez.spill("Email sent successfully:", response.message)
} otherwise {
    vibez.spill("Email send failed:", response.code, response.message)
}
```

#### `send_multiple_emails(client SmtpClient, emails []Email) yikes<[]SmtpResponse>`
Sends multiple emails efficiently using a single connection.

```cursed
sus responses []SmtpResponse = emailz.send_multiple_emails(client, emails) fam {
    when err -> {
        vibez.spill("Bulk email send failed:", err.message)
        damn
    }
}

sus i drip = 0
bestie (i < arrayz.len(responses)) {
    sus response SmtpResponse = responses[i]
    ready (response.code >= 200 && response.code < 300) {
        vibez.spill("Email", i + 1, "sent successfully")
    } otherwise {
        vibez.spill("Email", i + 1, "failed:", response.code, response.message)
    }
    i = i + 1
}
```

#### `quit_smtp(client SmtpClient) yikes<SmtpResponse>`
Properly closes SMTP connection.

```cursed
sus response SmtpResponse = emailz.quit_smtp(client) fam {
    when err -> {
        vibez.spill("QUIT command failed:", err.message)
        damn
    }
}
```

## Email Composition Functions

### Basic Email Creation

#### `create_email(from tea, to tea, subject tea, body tea) yikes<Email>`
Creates a simple plain text email.

```cursed
sus email Email = emailz.create_email(
    "sender@example.com",
    "recipient@example.com",
    "Meeting Reminder",
    "Don't forget about our meeting tomorrow at 2 PM."
) fam {
    when err -> {
        vibez.spill("Email creation failed:", err.message)
        damn
    }
}
```

#### `create_html_email(from tea, to tea, subject tea, html_body tea, text_body tea) yikes<Email>`
Creates an HTML email with plain text fallback.

```cursed
sus html_content tea = "<h1>Welcome!</h1><p>Thanks for signing up for our service.</p>"
sus text_content tea = "Welcome! Thanks for signing up for our service."

sus email Email = emailz.create_html_email(
    "noreply@example.com",
    "newuser@example.com",
    "Welcome to Our Service",
    html_content,
    text_content
) fam {
    when err -> {
        vibez.spill("HTML email creation failed:", err.message)
        damn
    }
}
```

### Email Building Functions

#### `add_recipient(email Email, address tea) yikes<Email>`
Adds a TO recipient to an email.

```cursed
email = emailz.add_recipient(email, "additional@example.com") fam {
    when err -> {
        vibez.spill("Failed to add recipient:", err.message)
        damn
    }
}
```

#### `add_cc_recipient(email Email, address tea) yikes<Email>`
Adds a CC recipient to an email.

```cursed
email = emailz.add_cc_recipient(email, "manager@example.com") fam {
    when err -> {
        vibez.spill("Failed to add CC recipient:", err.message)
        damn
    }
}
```

#### `add_bcc_recipient(email Email, address tea) yikes<Email>`
Adds a BCC recipient to an email.

```cursed
email = emailz.add_bcc_recipient(email, "archive@example.com") fam {
    when err -> {
        vibez.spill("Failed to add BCC recipient:", err.message)
        damn
    }
}
```

#### `add_custom_header(email Email, name tea, value tea) yikes<Email>`
Adds a custom header to an email.

```cursed
email = emailz.add_custom_header(email, "X-Custom-App", "MyApp v1.0") fam {
    when err -> {
        vibez.spill("Failed to add custom header:", err.message)
        damn
    }
}

email = emailz.add_custom_header(email, "X-Priority", "1") fam {
    when err -> {
        vibez.spill("Failed to add priority header:", err.message)
        damn
    }
}
```

#### `set_reply_to(email Email, address tea) yikes<Email>`
Sets the Reply-To address for an email.

```cursed
email = emailz.set_reply_to(email, "support@example.com") fam {
    when err -> {
        vibez.spill("Failed to set reply-to:", err.message)
        damn
    }
}
```

#### `set_priority(email Email, priority EmailPriority) Email`
Sets the priority level of an email.

```cursed
email = emailz.set_priority(email, EmailPriority.High)
```

### Attachment Functions

#### `add_file_attachment(email Email, file_path tea, content_type tea) yikes<Email>`
Adds a file as an attachment to an email.

```cursed
email = emailz.add_file_attachment(email, "/path/to/document.pdf", "application/pdf") fam {
    when err -> {
        vibez.spill("Failed to add file attachment:", err.message)
        damn
    }
}
```

#### `add_attachment_from_data(email Email, filename tea, content_type tea, data tea) yikes<Email>`
Adds an attachment from raw data.

```cursed
sus csv_data tea = "Name,Email\nJohn,john@example.com\nJane,jane@example.com"

email = emailz.add_attachment_from_data(
    email,
    "contacts.csv",
    "text/csv",
    csv_data
) fam {
    when err -> {
        vibez.spill("Failed to add data attachment:", err.message)
        damn
    }
}
```

#### `add_inline_image(email Email, image_path tea, content_id tea) yikes<Email>`
Adds an inline image that can be referenced in HTML content.

```cursed
// First add the inline image
email = emailz.add_inline_image(email, "/path/to/logo.png", "logo") fam {
    when err -> {
        vibez.spill("Failed to add inline image:", err.message)
        damn
    }
}

// Then reference it in HTML content
sus html_with_image tea = "<h1>Welcome!</h1><img src=\"cid:logo\" alt=\"Company Logo\">"
email.body_html = html_with_image
```

## Email Parsing Functions

### Message Parsing

#### `parse_email(raw_email tea) yikes<ParsedEmail>`
Parses a raw email message into structured data.

```cursed
sus raw_message tea = "From: sender@example.com\r\nTo: recipient@example.com\r\nSubject: Test\r\n\r\nHello World!"

sus parsed ParsedEmail = emailz.parse_email(raw_message) fam {
    when err -> {
        vibez.spill("Email parsing failed:", err.message)
        damn
    }
}

vibez.spill("From:", parsed.from)
vibez.spill("Subject:", parsed.subject)
vibez.spill("Body:", parsed.body_text)
```

#### `parse_email_headers(header_section tea) yikes<[]EmailHeader>`
Parses just the header section of an email.

```cursed
sus headers_raw tea = "From: sender@example.com\r\nTo: recipient@example.com\r\nSubject: Test\r\nDate: Mon, 1 Jan 2024 12:00:00 +0000\r\n"

sus headers []EmailHeader = emailz.parse_email_headers(headers_raw) fam {
    when err -> {
        vibez.spill("Header parsing failed:", err.message)
        damn
    }
}

sus i drip = 0
bestie (i < arrayz.len(headers)) {
    sus header EmailHeader = headers[i]
    vibez.spill("Header:", header.name, "=", header.value)
    i = i + 1
}
```

#### `get_header_value(headers []EmailHeader, header_name tea) tea`
Gets the value of a specific header.

```cursed
sus subject tea = emailz.get_header_value(headers, "Subject")
sus date tea = emailz.get_header_value(headers, "Date")
sus content_type tea = emailz.get_header_value(headers, "Content-Type")
```

### MIME Processing

#### `parse_mime_message(parsed_email ParsedEmail) yikes<ParsedEmail>`
Processes MIME multipart messages and extracts parts.

```cursed
sus mime_processed ParsedEmail = emailz.parse_mime_message(parsed) fam {
    when err -> {
        vibez.spill("MIME parsing failed:", err.message)
        damn
    }
}

vibez.spill("Text body:", mime_processed.body_text)
vibez.spill("HTML body:", mime_processed.body_html)
vibez.spill("Attachments:", arrayz.len(mime_processed.attachments))
```

#### `extract_attachments(parsed_email ParsedEmail) []EmailAttachment`
Extracts all attachments from a parsed email.

```cursed
sus attachments []EmailAttachment = emailz.extract_attachments(parsed)

sus i drip = 0
bestie (i < arrayz.len(attachments)) {
    sus attachment EmailAttachment = attachments[i]
    vibez.spill("Attachment:", attachment.filename)
    vibez.spill("Type:", attachment.content_type)
    vibez.spill("Size:", stringz.len(attachment.content), "bytes (base64)")
    i = i + 1
}
```

#### `save_attachment(attachment EmailAttachment, save_path tea) yikes<drip>`
Saves an attachment to disk.

```cursed
sus bytes_written drip = emailz.save_attachment(attachment, "/tmp/saved_file.pdf") fam {
    when err -> {
        vibez.spill("Failed to save attachment:", err.message)
        damn
    }
}

vibez.spill("Saved attachment:", bytes_written, "bytes")
```

## Email Validation Functions

#### `validate_email_address(address tea) lit`
Validates an email address format.

```cursed
sus valid_email lit = emailz.validate_email_address("user@example.com")
sus invalid_email lit = emailz.validate_email_address("invalid-email")

ready (valid_email) {
    vibez.spill("Email address is valid")
} otherwise {
    vibez.spill("Email address is invalid")
}
```

#### `normalize_email_address(address tea) tea`
Normalizes an email address (removes extra spaces, converts to lowercase).

```cursed
sus normalized tea = emailz.normalize_email_address("  USER@EXAMPLE.COM  ")
vibez.spill("Normalized:", normalized)  // "user@example.com"
```

#### `extract_domain(address tea) tea`
Extracts the domain part from an email address.

```cursed
sus domain tea = emailz.extract_domain("user@example.com")
vibez.spill("Domain:", domain)  // "example.com"
```

## Template Functions

### Email Templates

#### `create_template_email(template_name tea, variables []TemplateVariable) yikes<Email>`
Creates an email from a predefined template.

```cursed
squad TemplateVariable {
    sus name tea
    sus value tea
}

sus variables []TemplateVariable = [
    TemplateVariable{name: "user_name", value: "John Doe"},
    TemplateVariable{name: "activation_link", value: "https://example.com/activate/abc123"},
    TemplateVariable{name: "support_email", value: "support@example.com"}
]

sus email Email = emailz.create_template_email("welcome_email", variables) fam {
    when err -> {
        vibez.spill("Template email creation failed:", err.message)
        damn
    }
}
```

#### `register_email_template(name tea, subject_template tea, html_template tea, text_template tea) yikes<lit>`
Registers a new email template.

```cursed
sus subject_tmpl tea = "Welcome {{user_name}} - Activate Your Account"
sus html_tmpl tea = "<h1>Welcome {{user_name}}!</h1><p>Click <a href=\"{{activation_link}}\">here</a> to activate.</p>"
sus text_tmpl tea = "Welcome {{user_name}}! Visit {{activation_link}} to activate your account."

emailz.register_email_template("welcome_email", subject_tmpl, html_tmpl, text_tmpl) fam {
    when err -> {
        vibez.spill("Template registration failed:", err.message)
        damn
    }
}
```

## Utility Functions

### Email Formatting

#### `format_email_for_sending(email Email) tea`
Formats an email as RFC 5322 compliant text.

```cursed
sus formatted_email tea = emailz.format_email_for_sending(email)
vibez.spill("Formatted email:")
vibez.spill(formatted_email)
```

#### `generate_message_id(domain tea) tea`
Generates a unique Message-ID header.

```cursed
sus msg_id tea = emailz.generate_message_id("example.com")
vibez.spill("Message ID:", msg_id)  // <20240101120000.abc123@example.com>
```

#### `format_date_header() tea`
Generates a properly formatted Date header.

```cursed
sus date_header tea = emailz.format_date_header()
vibez.spill("Date:", date_header)  // Mon, 1 Jan 2024 12:00:00 +0000
```

### Content Encoding

#### `encode_base64(data tea) tea`
Encodes data to Base64 (for attachments).

```cursed
sus file_content tea = "Hello, World!"
sus encoded tea = emailz.encode_base64(file_content)
vibez.spill("Base64:", encoded)  // SGVsbG8sIFdvcmxkIQ==
```

#### `decode_base64(encoded tea) yikes<tea>`
Decodes Base64 encoded data.

```cursed
sus decoded tea = emailz.decode_base64("SGVsbG8sIFdvcmxkIQ==") fam {
    when err -> {
        vibez.spill("Base64 decode failed:", err.message)
        damn
    }
}

vibez.spill("Decoded:", decoded)  // Hello, World!
```

#### `encode_quoted_printable(text tea) tea`
Encodes text using quoted-printable encoding.

```cursed
sus special_text tea = "Héllo Wörld! This has spëcial characters."
sus encoded tea = emailz.encode_quoted_printable(special_text)
vibez.spill("Quoted-printable:", encoded)
```

#### `decode_quoted_printable(encoded tea) yikes<tea>`
Decodes quoted-printable encoded text.

```cursed
sus decoded tea = emailz.decode_quoted_printable(encoded) fam {
    when err -> {
        vibez.spill("Quoted-printable decode failed:", err.message)
        damn
    }
}
```

## Advanced Features

### Bulk Email Management

#### `create_bulk_email_sender(client SmtpClient, batch_size drip) BulkEmailSender`
Creates a bulk email sender for efficient mass mailing.

```cursed
squad BulkEmailSender {
    sus client SmtpClient
    sus batch_size drip
    sus sent_count drip
    sus failed_count drip
    sus rate_limit drip  // emails per minute
}

sus bulk_sender BulkEmailSender = emailz.create_bulk_email_sender(client, 100)
bulk_sender.rate_limit = 60  // 60 emails per minute
```

#### `send_bulk_emails(sender BulkEmailSender, emails []Email) yikes<BulkEmailResult>`
Sends emails in batches with rate limiting.

```cursed
squad BulkEmailResult {
    sus total_sent drip
    sus total_failed drip
    sus failed_emails []Email
    sus send_duration drip  // milliseconds
}

sus result BulkEmailResult = emailz.send_bulk_emails(bulk_sender, email_list) fam {
    when err -> {
        vibez.spill("Bulk email send failed:", err.message)
        damn
    }
}

vibez.spill("Sent:", result.total_sent, "Failed:", result.total_failed)
vibez.spill("Duration:", result.send_duration, "ms")
```

### Email Bounce Handling

#### `parse_bounce_email(bounce_email ParsedEmail) yikes<BounceInfo>`
Parses bounce/delivery failure emails.

```cursed
squad BounceInfo {
    sus bounce_type tea         // hard, soft, transient
    sus original_recipient tea  // bounced email address
    sus smtp_code drip         // SMTP error code
    sus diagnostic tea         // Diagnostic message
    sus action tea            // failed, delayed, delivered
}

sus bounce_info BounceInfo = emailz.parse_bounce_email(parsed_bounce) fam {
    when err -> {
        vibez.spill("Bounce parsing failed:", err.message)
        damn
    }
}

vibez.spill("Bounce type:", bounce_info.bounce_type)
vibez.spill("Failed recipient:", bounce_info.original_recipient)
```

## Configuration and Connection Pooling

### SMTP Connection Pool

#### `create_smtp_pool(config SmtpPoolConfig) yikes<SmtpPool>`
Creates a pool of SMTP connections for high-throughput applications.

```cursed
squad SmtpPoolConfig {
    sus host tea
    sus port drip
    sus username tea
    sus password tea
    sus max_connections drip
    sus max_idle_time drip  // seconds
    sus use_tls lit
}

squad SmtpPool {
    sus config SmtpPoolConfig
    sus connections []SmtpClient
    sus active_count drip
    sus total_sent drip
}

sus pool_config SmtpPoolConfig = SmtpPoolConfig{
    host: "smtp.example.com",
    port: 587,
    username: "user@example.com", 
    password: "password123",
    max_connections: 10,
    max_idle_time: 300,
    use_tls: based
}

sus smtp_pool SmtpPool = emailz.create_smtp_pool(pool_config) fam {
    when err -> {
        vibez.spill("SMTP pool creation failed:", err.message)
        damn
    }
}
```

#### `send_email_pooled(pool SmtpPool, email Email) yikes<SmtpResponse>`
Sends email using a connection from the pool.

```cursed
sus response SmtpResponse = emailz.send_email_pooled(smtp_pool, email) fam {
    when err -> {
        vibez.spill("Pooled email send failed:", err.message)
        damn
    }
}
```

## Error Handling

EmailZ provides comprehensive error handling through structured error types:

### Common Error Types

- **`smtp_connect`**: Connection establishment errors
- **`smtp_auth`**: Authentication failures
- **`smtp_command`**: SMTP command execution errors
- **`email_format`**: Email formatting/validation errors
- **`mime_parse`**: MIME parsing errors
- **`attachment_read`**: File attachment errors
- **`template_render`**: Template processing errors
- **`encoding_error`**: Content encoding/decoding errors

### Error Handling Patterns

```cursed
// Pattern 1: Basic error handling with retry
slay send_email_with_retry(client SmtpClient, email Email, max_retries drip) yikes<SmtpResponse> {
    sus retry_count drip = 0
    
    bestie (retry_count <= max_retries) {
        sus response SmtpResponse = emailz.send_email(client, email) fam {
            when err -> {
                ready (stringz.equals(err.kind, "smtp_connect")) {
                    retry_count = retry_count + 1
                    ready (retry_count <= max_retries) {
                        vibez.spill("Connection failed, retrying in 5 seconds...")
                        // Sleep 5 seconds (implementation dependent)
                        damn // Continue to next retry
                    }
                }
                yikes err // Re-throw for non-retryable errors
            }
        }
        damn response
    }
    
    yikes emailz.create_email_error("max_retries", "Maximum retries exceeded", 0, "")
}

// Pattern 2: Specific SMTP error code handling
emailz.send_email(client, email) fam {
    when err -> {
        ready (err.smtp_code == 550) {
            vibez.spill("Recipient rejected - invalid email address")
        } otherwise ready (err.smtp_code >= 400 && err.smtp_code < 500) {
            vibez.spill("Temporary failure - try again later")
        } otherwise ready (err.smtp_code >= 500) {
            vibez.spill("Permanent failure - do not retry")
        } otherwise {
            vibez.spill("Network or protocol error:", err.message)
        }
        damn
    }
}
```

## Security Considerations

### TLS/SSL Configuration
```cursed
// Always use TLS for production
sus secure_client SmtpClient = emailz.create_smtp_client_tls("smtp.gmail.com", 465) fam {
    when err -> {
        vibez.spill("Secure SMTP client creation failed:", err.message)
        damn
    }
}

// Verify certificate for security
secure_client.verify_certificate = based
```

### Input Validation
```cursed
// Always validate email addresses
ready (emailz.validate_email_address(recipient)) {
    // Proceed with email creation
} otherwise {
    vibez.spill("Invalid recipient email address:", recipient)
    damn
}

// Sanitize content to prevent injection
sus safe_subject tea = emailz.sanitize_header(subject)
sus safe_body tea = emailz.sanitize_content(body)
```

### Authentication Security
```cursed
// Use OAuth2 or app passwords when available
ready (emailz.supports_oauth2(client)) {
    emailz.auth_oauth2(client, oauth_token) fam {
        when err -> {
            vibez.spill("OAuth2 authentication failed:", err.message)
            damn
        }
    }
} otherwise {
    // Fallback to regular authentication
    emailz.auto_authenticate(client, username, password) fam {
        when err -> {
            vibez.spill("Authentication failed:", err.message)
            damn
        }
    }
}
```

## Complete Example: Newsletter System

```cursed
yeet "emailz"
yeet "filez"
yeet "jsonz"

squad NewsletterSubscriber {
    sus email tea
    sus name tea  
    sus preferences []tea
}

squad NewsletterConfig {
    sus smtp_host tea
    sus smtp_port drip
    sus smtp_username tea
    sus smtp_password tea
    sus from_email tea
    sus from_name tea
    sus template_dir tea
}

slay send_newsletter(config NewsletterConfig, subscribers []NewsletterSubscriber, subject tea, template_name tea) yikes<lit> {
    // Create SMTP client
    sus client SmtpClient = emailz.create_smtp_client_starttls(config.smtp_host, config.smtp_port) fam {
        when err -> yikes err
    }
    
    // Connect and authenticate
    emailz.connect_smtp(client) fam { when err -> yikes err }
    emailz.ehlo_smtp(client, "newsletter.example.com") fam { when err -> yikes err }
    emailz.starttls_smtp(client) fam { when err -> yikes err }
    emailz.auto_authenticate(client, config.smtp_username, config.smtp_password) fam { when err -> yikes err }
    
    // Load email template
    sus template_path tea = stringz.concat([config.template_dir, "/", template_name, ".html"])
    sus html_template tea = filez.read_file(template_path) fam {
        when err -> yikes err
    }
    
    sus text_template tea = filez.read_file(stringz.concat([config.template_dir, "/", template_name, ".txt"])) fam {
        when err -> {
            // Text template is optional
            damn ""
        }
    }
    
    // Send to each subscriber
    sus sent_count drip = 0
    sus i drip = 0
    
    bestie (i < arrayz.len(subscribers)) {
        sus subscriber NewsletterSubscriber = subscribers[i]
        
        // Personalize template
        sus personalized_html tea = stringz.replace_all(html_template, "{{name}}", subscriber.name)
        sus personalized_text tea = stringz.replace_all(text_template, "{{name}}", subscriber.name)
        
        // Create email
        sus email Email = emailz.create_html_email(
            stringz.concat([config.from_name, " <", config.from_email, ">"]),
            subscriber.email,
            subject,
            personalized_html,
            personalized_text
        ) fam {
            when err -> {
                vibez.spill("Failed to create email for", subscriber.email, ":", err.message)
                i = i + 1
                damn // Continue with next subscriber
            }
        }
        
        // Add unsubscribe header
        sus unsubscribe_link tea = stringz.concat(["https://newsletter.example.com/unsubscribe?email=", subscriber.email])
        email = emailz.add_custom_header(email, "List-Unsubscribe", stringz.concat(["<", unsubscribe_link, ">"])) fam {
            when err -> {
                vibez.spill("Failed to add unsubscribe header:", err.message)
            }
        }
        
        // Send email
        emailz.send_email(client, email) fam {
            when err -> {
                vibez.spill("Failed to send to", subscriber.email, ":", err.message)
                i = i + 1
                damn // Continue with next subscriber
            }
        }
        
        sent_count = sent_count + 1
        vibez.spill("Sent newsletter to", subscriber.email, "(", sent_count, "/", arrayz.len(subscribers), ")")
        
        // Rate limiting - wait between sends
        // In real implementation, would add proper delay
        
        i = i + 1
    }
    
    // Clean up
    emailz.quit_smtp(client) fam {
        when err -> {
            vibez.spill("Warning: Failed to properly close SMTP connection:", err.message)
        }
    }
    
    vibez.spill("Newsletter campaign completed. Sent to", sent_count, "subscribers.")
    damn based
}

// Usage
sus config NewsletterConfig = NewsletterConfig{
    smtp_host: "smtp.mailgun.org",
    smtp_port: 587, 
    smtp_username: "postmaster@mg.example.com",
    smtp_password: "your-mailgun-password",
    from_email: "newsletter@example.com",
    from_name: "Example Newsletter",
    template_dir: "/templates"
}

sus subscribers []NewsletterSubscriber = [
    NewsletterSubscriber{email: "user1@example.com", name: "John Doe", preferences: ["tech", "business"]},
    NewsletterSubscriber{email: "user2@example.com", name: "Jane Smith", preferences: ["tech", "design"]}
]

send_newsletter(config, subscribers, "Weekly Tech Update - January 2024", "weekly_tech") fam {
    when err -> {
        vibez.spill("Newsletter send failed:", err.message)
    }
}
```

This documentation provides comprehensive coverage of the EmailZ module's capabilities for building robust email applications in CURSED, including SMTP client functionality, email composition, MIME support, and advanced features like bulk sending and template processing.
